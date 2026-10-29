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
        children: node
            .children()
            .filter_map(|child| {
                if child.is_element() {
                    Some(XmlContent::Element(canonical_element(child)))
                } else if child.is_text() {
                    let text = child.text().unwrap();
                    if text.trim().is_empty() && name.name() != "t" {
                        None
                    } else {
                        Some(XmlContent::Text(if timestamp {
                            "<generation time>".into()
                        } else {
                            text.into()
                        }))
                    }
                } else {
                    None
                }
            })
            .collect(),
    }
}

pub fn compare_xml(actual: &str, expected: &str, name: &str) {
    let actual = roxmltree::Document::parse(actual).unwrap();
    let expected = roxmltree::Document::parse(expected).unwrap();
    assert_eq!(
        canonical_element(actual.root_element()),
        canonical_element(expected.root_element()),
        "{name}"
    );
}

pub fn compare_fixture(name: &str) -> BTreeMap<String, String> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/docx")
        .join(name);
    let source = fs::read_to_string(fixture.join("source.mote")).unwrap();
    let model: serde_json::Value =
        serde_json::from_str(&mote::json::to_json(&mote::compile(&source).unwrap()).unwrap())
            .unwrap();
    let expected_model: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(fixture.join("model.json")).unwrap()).unwrap();
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
        compare_xml(&actual[name], &expected, name);
    }
    actual
}
