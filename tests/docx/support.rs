use std::{
    collections::BTreeMap,
    fs,
    io::{Cursor, Read},
    path::Path,
};

pub fn package(source: &str) -> BTreeMap<String, String> {
    let document = mote::compile(source).unwrap();
    let original = document.clone();
    let bytes = mote::docx::emit(&document).unwrap();
    assert_eq!(document, original, "emission changed the document model");
    unpack(bytes)
}

pub fn unpack(bytes: Vec<u8>) -> BTreeMap<String, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut parts = BTreeMap::new();
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).unwrap();
        let mut contents = String::new();
        entry.read_to_string(&mut contents).unwrap();
        assert!(
            parts
                .insert(entry.name().unwrap().into_owned(), contents)
                .is_none()
        );
    }
    parts
}

#[derive(Debug, PartialEq, Eq)]
struct XmlElement {
    name: (String, String),
    attributes: BTreeMap<(String, String), String>,
    children: Vec<XmlContent>,
}
#[derive(Debug, PartialEq, Eq)]
enum XmlContent {
    Element(XmlElement),
    Text(String),
}

fn canonical_element(node: roxmltree::Node<'_, '_>) -> XmlElement {
    let name = node.tag_name();
    let timestamp = name.namespace() == Some("http://purl.org/dc/terms/")
        && matches!(name.name(), "created" | "modified");
    XmlElement {
        name: (
            name.namespace().unwrap_or_default().into(),
            name.name().into(),
        ),
        attributes: node
            .attributes()
            .map(|attribute| {
                (
                    (
                        attribute.namespace().unwrap_or_default().into(),
                        attribute.name().into(),
                    ),
                    attribute.value().into(),
                )
            })
            .collect(),
        children: canonical_children(node, timestamp),
    }
}

fn canonical_children(node: roxmltree::Node<'_, '_>, timestamp: bool) -> Vec<XmlContent> {
    const WORD: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let mut output: Vec<XmlContent> = Vec::new();
    let mut advisory = false;
    for child in node.children() {
        if child.has_tag_name((WORD, "lastRenderedPageBreak")) {
            advisory = true;
            continue;
        }
        if child.is_element() {
            let current = canonical_element(child);
            if advisory
                && node.has_tag_name((WORD, "r"))
                && child.has_tag_name((WORD, "t"))
                && let Some(XmlContent::Element(previous)) = output.last_mut()
                && previous.name == current.name
                && previous.attributes == current.attributes
                && let ([XmlContent::Text(previous_text)], [XmlContent::Text(current_text)]) = (
                    previous.children.as_mut_slice(),
                    current.children.as_slice(),
                )
            {
                previous_text.push_str(current_text);
                advisory = false;
                continue;
            }
            output.push(XmlContent::Element(current));
            advisory = false;
        } else if child.is_text() {
            let text = child.text().unwrap();
            if !text.trim().is_empty() || node.tag_name().name() == "t" {
                output.push(XmlContent::Text(if timestamp {
                    "<generation time>".into()
                } else {
                    text.into()
                }));
            }
        }
    }
    output
}

pub fn compare_xml(actual: &str, expected: &str, name: &str) {
    let actual = roxmltree::Document::parse(actual).unwrap();
    let expected = roxmltree::Document::parse(expected).unwrap();
    compare_elements(
        &canonical_element(actual.root_element()),
        &canonical_element(expected.root_element()),
        name,
    );
}

pub fn compare_fixture(name: &str) -> BTreeMap<String, String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/docx")
        .join(name);
    let source = fs::read_to_string(fixture.join("source.mote")).unwrap();
    let mut model: serde_json::Value =
        serde_json::from_str(&mote::json::to_json(&mote::compile(&source).unwrap()).unwrap())
            .unwrap();
    let mut expected_model: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(fixture.join("model.json")).unwrap()).unwrap();
    normalize_numbers(&mut model);
    normalize_numbers(&mut expected_model);
    assert_eq!(model, expected_model, "{name}: evaluated model");
    let actual = package(&source);
    let capture: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(fixture.join("capture.json")).unwrap()).unwrap();
    let names: Vec<_> = capture["parts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap())
        .collect();
    assert_eq!(actual.len(), names.len());
    for name in names {
        let expected = fs::read_to_string(fixture.join("package").join(name)).unwrap();
        let count_advisory = |text: &str| {
            roxmltree::Document::parse(text)
                .unwrap()
                .descendants()
                .filter(|node| {
                    node.has_tag_name((
                        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                        "lastRenderedPageBreak",
                    ))
                })
                .count()
        };
        let expected_omissions = capture["omitted_advisory_page_breaks"][name]
            .as_u64()
            .unwrap_or(0) as usize;
        assert_eq!(
            count_advisory(&expected),
            expected_omissions,
            "{name}: advisory marker inventory"
        );
        assert_eq!(
            count_advisory(&actual[name]),
            0,
            "{name}: output has no pagination estimate"
        );
        compare_xml(&actual[name], &expected, name);
    }
    actual
}

fn normalize_numbers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        serde_json::Value::Object(properties) => {
            properties.values_mut().for_each(normalize_numbers)
        }
        _ => {}
    }
}

fn compare_elements(actual: &XmlElement, expected: &XmlElement, path: &str) {
    assert_eq!(actual.name, expected.name, "{path}: element name");
    assert_eq!(actual.attributes, expected.attributes, "{path}: attributes");
    assert_eq!(
        actual.children.len(),
        expected.children.len(),
        "{path}: child count"
    );
    for (index, (actual, expected)) in actual.children.iter().zip(&expected.children).enumerate() {
        let path = format!("{path}/{index}");
        match (actual, expected) {
            (XmlContent::Element(actual), XmlContent::Element(expected)) => {
                compare_elements(actual, expected, &path)
            }
            _ => assert_eq!(actual, expected, "{path}: content"),
        }
    }
}
