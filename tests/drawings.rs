use mote::evaluation::EvaluationOptions;
use std::{fs, path::Path};

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
fn drawing_models_match_captured_attributes_and_lua_constructors() {
    use sha2::{Digest, Sha256};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/drawings");
    for case in ["shapes", "lines", "kinds", "lua"] {
        let source = fs::read_to_string(root.join(format!("{case}.mote"))).unwrap();
        let capture = root.join("captured").join(case);
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        let document = mote::compile_with_options(
            &source,
            EvaluationOptions {
                enable_lua: true,
                ..Default::default()
            },
        )
        .unwrap();
        let mut actual: serde_json::Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}");
    }
}

#[test]
fn geometry_and_marker_errors_are_rejected_before_output() {
    for source in [
        "#shape()",
        "#shape(position={width=10})",
        "#shape(position={height=10})",
        "#shape(position={width=-1,height=10})",
        "#shape(position={width=10,height=10},kind='star')",
        "#shape(position={width=10,height=10},text={fit='expand'})",
        "#shape(position={width=10,height=10},shadow={blur=-1})",
        "#line()",
        "#line(from={x=0,y=0},to={x=1})",
        "#line(from={x=0,y=0},to={x=1,y=1})[body]",
        "#line(from={x=0,y=0},to={x=1,y=1},arrow={size='huge'})",
        "#line(from={x=0,y=0},to={x=1,y=1},arrow='circle')",
        "#line(from={x=0,y=0},to={x=1,y=1},cap='triangle')",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
}

#[test]
fn overrides_and_unknown_metadata_do_not_invent_drawing_semantics() {
    let source = "#shape(position={width=10,height=20},kind='oval',properties={kind='rect'},border={metadata='retained'},highlight='#abc')";
    let document = mote::compile(source).unwrap();
    let mote::model::Content::Element(shape) = &document.content[0] else {
        panic!("missing shape")
    };
    assert_eq!(shape.attributes["kind"].to_text(), "rectangle");
    assert_eq!(shape.attributes["highlight"].to_text(), "#abc");
    let mote::model::Value::Object(border) = &shape.attributes["border"] else {
        panic!("missing border")
    };
    assert_eq!(border["metadata"].to_text(), "retained");
}
