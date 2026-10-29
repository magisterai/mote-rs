#[allow(dead_code)]
#[path = "docx/support.rs"]
mod xml_support;
use mote::source::FileAssetResolver;
use std::{
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/references")
}
fn normalize_numbers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        serde_json::Value::Object(fields) => fields.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}
#[test]
fn references_preserve_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "basic",
        "missing",
        "names",
        "page-flow",
        "placement",
        "story-labels",
        "wrapping",
    ] {
        let source = fs::read_to_string(fixtures().join(format!("{case}.mote"))).unwrap();
        let document = mote::compile(&source).unwrap();
        let original = document.clone();
        let capture = fixtures().join("captured").join(case);
        let mut actual: serde_json::Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}: model");
        let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
        let mut archive = zip::ZipArchive::new(Cursor::new(generated.bytes)).unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("capture.json")).unwrap()).unwrap();
        let names = manifest["parts"].as_array().unwrap();
        assert_eq!(archive.len(), names.len());
        for name in names {
            let name = name.as_str().unwrap();
            let mut actual = Vec::new();
            archive
                .by_name(name)
                .unwrap()
                .read_to_end(&mut actual)
                .unwrap();
            let expected = fs::read(capture.join("package").join(name)).unwrap();
            if name.ends_with(".xml") || name.ends_with(".rels") {
                xml_support::compare_xml(
                    std::str::from_utf8(&actual).unwrap(),
                    std::str::from_utf8(&expected).unwrap(),
                    &format!("{case}/{name}"),
                );
            } else {
                assert_eq!(actual, expected, "{case}/{name}");
            }
        }
        assert_eq!(document, original);
    }
}

#[test]
fn declarations_validate_before_resolving_forward_references() {
    for source in [
        "#bookmark()",
        "#bookmark(\"\")",
        "#bookmark(2)",
        "#label(\" \" )",
        "#label(\"target\")[body]",
        "#ref(\"missing\")",
        "#ref(\"missing\",fallback=\"fallback\")",
        "#bookmark(\"target\")[Text] #ref(\"target\")",
        "#label(\"target\") #label(\"target\")",
        "#label(\"target\") #ref(\"target\")[body]",
        "#label(\"target\") #ref(target={label=\"\",name=\"target\"})",
        "#label(\"target\") #ref(\"target\",kind=\"unknown\")",
        "#label(\"target\") #ref(\"target\",display=\"unknown\")",
        "#label(\"target\",name=\"duplicate\")",
        "#section[#header[#label(\"same\")]First] #section[#header[#label(\"same\")]Second]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let source = "#label(\"same\")\n#paragraph[Second #label(\"same\")]";
    let diagnostic = mote::compile(source).unwrap_err();
    assert_eq!(diagnostic.location(source), (2, 19));
    assert!(diagnostic.message.contains("declared more than once"));
    assert!(mote::compile("#ref(\"later\") #label(\"later\")").is_ok());
    assert!(
        mote::compile("#header[#label(\"running\")] #section[First] #section[#ref(\"running\")]")
            .is_ok()
    );
}

#[test]
fn replaced_running_stories_do_not_register_discarded_names() {
    let source = "#header[#label(\"same\")Discarded #ref(\"unused\")]\n#header[#label(\"same\")Current]\n#paragraph[See #ref(\"same\").]";
    assert!(mote::compile(source).is_ok());
    let source = "#header[#label(\"discarded\")Unused]\n#header[Current]\n#paragraph[See #ref(\"discarded\").]";
    let diagnostic = mote::compile(source).unwrap_err();
    assert!(diagnostic.message.contains("not labeled"));
    let source = "#header[#label(\"earlier\")Earlier]\nFirst body.\n#header[Current]\n#paragraph[See #ref(\"earlier\").]";
    assert!(
        mote::compile(source).is_ok(),
        "a story used by an earlier section remains registered"
    );
}
