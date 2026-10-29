//! Each fenced Mote example must produce its model or a documented validation error.

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
        if path.extension().is_none_or(|extension| extension != "md") {
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
        let example_identifier = case["example"].as_str().unwrap();
        let source = examples.remove(example_identifier).unwrap_or_else(|| {
            panic!("duplicate or missing specification example: {example_identifier}")
        });
        assert!(
            case.get("deferred").is_none(),
            "{example_identifier}: deferred implementation is not an example outcome"
        );
        assert_ne!(
            case.get("model").is_some(),
            case.get("diagnostic").is_some(),
            "{example_identifier}: declare exactly one model or intentional diagnostic"
        );
        parse(&source).unwrap_or_else(|error| panic!("{example_identifier}: {error}"));
        if let Some(model_name) = case["model"].as_str() {
            let expected = models
                .get(model_name)
                .unwrap_or_else(|| panic!("missing expected model: {model_name}"));
            let compiled = if let Some(assets) = case.get("assets") {
                let resolver = |request: mote::source::AssetRequest<'_>| {
                    let name = request.reference.to_str().unwrap();
                    let value = assets
                        .get(name)
                        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, name))?;
                    Ok(mote::source::ResolvedAsset {
                        name: request.reference.to_owned(),
                        bytes: serde_json::to_vec(value).unwrap(),
                    })
                };
                mote::compile_with_assets(&source, &resolver, None)
            } else {
                compile(&source)
            };
            let document = compiled.unwrap_or_else(|error| panic!("{example_identifier}: {error}"));
            let actual: Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
            assert_eq!(&actual, expected, "{example_identifier}");
        } else {
            let reason = case["invalid"]
                .as_str()
                .expect("an invalid example must explain the rule it violates");
            assert!(
                !reason.is_empty(),
                "{example_identifier}: an invalid example must explain the rule it violates"
            );
            let message = case["diagnostic"]
                .as_str()
                .expect("an invalid example must name its expected diagnostic");
            let error = compile(&source).expect_err("this documented invalid use must be rejected");
            assert!(
                error.message.contains(message),
                "{example_identifier}: {error}"
            );
        }
    }
    assert!(
        examples.is_empty(),
        "specification examples need declared outcomes: {:?}",
        examples.keys()
    );
}
