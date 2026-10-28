//! Each fenced Mote example must have an expected model or an explicit deferral.

use mote::{compile, json::to_json, syntax::parse};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};

#[test]
fn specification_examples_match_their_declared_outcomes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture: Value = serde_json::from_str(include_str!("fixtures/specification.json")).unwrap();
    let mut examples = BTreeMap::new();
    for entry in fs::read_dir(root.join("docs/specification")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let chapter = path.file_stem().unwrap().to_str().unwrap();
        let markdown = fs::read_to_string(&path).unwrap();
        for (index, block) in markdown.split("```mote\n").skip(1).enumerate() {
            let (source, _) = block
                .split_once("\n```")
                .expect("Mote example must have a closing fence");
            examples.insert(format!("{chapter}/{}", index + 1), source.to_owned());
        }
    }
    let cases = fixture["cases"].as_array().unwrap();
    let models = fixture["models"].as_object().unwrap();
    assert!(!cases.is_empty());
    for case in cases {
        let id = case["example"].as_str().unwrap();
        let source = examples
            .remove(id)
            .unwrap_or_else(|| panic!("duplicate or missing specification example: {id}"));
        parse(&source).unwrap_or_else(|error| panic!("{id}: {error}"));
        if let Some(model_name) = case["model"].as_str() {
            let expected = models
                .get(model_name)
                .unwrap_or_else(|| panic!("missing expected model: {model_name}"));
            let document = compile(&source).unwrap_or_else(|error| panic!("{id}: {error}"));
            let actual: Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
            assert_eq!(&actual, expected, "{id}");
        } else {
            let reason = case["deferred"]
                .as_str()
                .expect("a case must declare a model or a deferral");
            assert!(
                !reason.is_empty(),
                "{id}: a deferral must explain the missing feature"
            );
            let message = case["diagnostic"]
                .as_str()
                .expect("a deferral must name its expected diagnostic");
            let error = compile(&source)
                .expect_err("implemented examples must be promoted from deferred to model cases");
            assert!(error.message.contains(message), "{id}: {error}");
        }
    }
    assert!(
        examples.is_empty(),
        "specification examples need declared outcomes: {:?}",
        examples.keys()
    );
}
