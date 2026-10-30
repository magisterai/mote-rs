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
fn chart_models_match_captured_data_properties_and_lua_constructors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/charts");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in [
        "data",
        "kinds",
        "labels",
        "legends",
        "lua",
        "overrides",
        "properties",
    ] {
        let path = root.join(format!("{case}.mote"));
        let source = fs::read_to_string(&path).unwrap();
        let folder = root.join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        for (path, expected) in manifest["assets"].as_object().unwrap() {
            assert_eq!(
                format!("{:x}", Sha256::digest(fs::read(root.join(path)).unwrap())),
                expected.as_str().unwrap()
            );
        }
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
        let mut expected: Value =
            serde_json::from_slice(&fs::read(folder.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}");
    }
}

#[test]
fn invalid_chart_signatures_and_formatting_are_rejected_before_output() {
    for source in [
        "#chart()",
        "#chart(properties={data={values={1}}})",
        "#chart(data={1,2})",
        "#chart({values={1}})",
        "#chart(data={values={1}})[body]",
        "#chart(data={values={1}},kind='scatter')",
        "#chart(data={values={1}},title=true)",
        "#chart(data={values={1}},width=-1)",
        "#chart(data={values={1}},legend='r')",
        "#chart(data={values={1}},legend={position='b'})",
        "#chart(data={values={1}},legend={enabled=1})",
        "#chart(data={values={1}},colors={'red'})",
        "#chart(data={values={1}},colors={metadata=true})",
        "#chart(data={values={1}},border={width=-1})",
        "#chart(data={values={1}},plot_area='false')",
        "#chart(data={values={1}},gridlines={major=1})",
        "#chart(data={values={1}},data_labels={position='automatic'})",
        "#chart(data={values={1}},gap_width=1.5)",
        "#chart(data={values={1}},overlap='-2.5')",
        "#chart(data={values={1}},style=17)",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
}
