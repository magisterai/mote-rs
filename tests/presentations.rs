use mote::{evaluation::EvaluationOptions, source::FileAssetResolver};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn normalize_numbers(value: &mut Value) {
    match value {
        Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        Value::Object(fields) => fields.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}

#[test]
fn presentation_declarations_slides_and_boxes_match_captured_models() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentations");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in ["basics", "properties", "merging", "content", "lua"] {
        let path = root.join(format!("{case}.mote"));
        let source = fs::read_to_string(&path).unwrap();
        let folder = root.join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        let expected = fs::read(folder.join("model.json")).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&expected)),
            manifest["model_sha256"]
        );
        let document = mote::compile_with_options(
            &source,
            EvaluationOptions {
                enable_lua: true,
                asset_resolver: Some(&resolver),
                source_name: Some(path.as_os_str()),
                ..Default::default()
            },
        )
        .unwrap();
        let mut actual: Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: Value = serde_json::from_slice(&expected).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}");
        assert!(
            mote::docx::emit(&document).is_err(),
            "{case} has no document output mapping"
        );
        assert!(
            mote::render::plan_with_layout(
                &document,
                &resolver,
                None,
                mote::render::LayoutMode::Document
            )
            .is_err(),
            "{case} needs presentation layout"
        );
    }
}

#[test]
fn presentation_signatures_and_content_boundaries_are_explicit() {
    for source in [
        "#presentation('wide')",
        "#presentation()[Body]",
        "#presentation(size='a4')",
        "#presentation(size={width=-1})",
        "#presentation(width=-1)",
        "#presentation(theme=true)",
        "#presentation(background={fit='tile'})",
        "#presentation(footer=1)",
        "#presentation(width=true,properties={width=1})",
        "#slide",
        "#slide(notes=3)[Text]",
        "#box(position={width=1,height=1})",
        "#box[Text]",
        "#box(position={width=1})[Text]",
        "#box(position={width=1,height=1},alignment='middle')[Text]",
        "#box(position={width=1,height=1},text={fit='expand'})[Text]",
        "#box(position={width=1,height=1})[Text #image(src='image.png')]",
        "#slide[Text #shape(position={width=1,height=1})]",
        "#slide[Text #line(from={x=1,y=1},to={x=2,y=2})]",
        "#slide(layout='title')[Text]",
        "#box(slot='title')[Text]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let document = mote::compile("#slide[] #slide[#box(position={width=1,height=1})[]]").unwrap();
    assert_eq!(document.warnings.len(), 2);
    let plain = mote::compile("#presentation()Plain text.").unwrap();
    assert!(plain.presentation.properties.is_empty());
    assert!(mote::docx::emit(&plain).is_ok());
}
