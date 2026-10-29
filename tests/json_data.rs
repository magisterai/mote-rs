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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/json-data")
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
fn json_data_preserves_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in ["values", "empty", "reports/invoice"] {
        let source = fs::read_to_string(fixtures().join(format!("{case}.mote"))).unwrap();
        let name = fixtures().join(format!("{case}.mote"));
        let document =
            mote::compile_with_assets(&source, &resolver, Some(name.as_os_str())).unwrap();
        let original = document.clone();
        let capture = fixtures().join("captured").join(case);
        let mut actual: serde_json::Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}: model");
        let generated =
            mote::docx::emit_with_assets(&document, &resolver, Some(name.as_os_str())).unwrap();
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
fn json_resources_preserve_typed_values_and_snapshot_repeated_requests() {
    use mote::source::{AssetRequest, ResolvedAsset};
    use std::{cell::Cell, ffi::OsStr};
    let requests = Cell::new(0);
    let resolver = |request: AssetRequest<'_>| {
        assert_eq!(
            request.source_name,
            Some(OsStr::new("asset://bundle/main.mote"))
        );
        assert_eq!(request.reference, "data.json");
        requests.set(requests.get() + 1);
        Ok(ResolvedAsset {
            name: "asset://bundle/data.json".into(),
            bytes: br#"{"title":"First","format":{"bold":true,"size":"12pt"},"empty":null}"#
                .to_vec(),
        })
    };
    let source = "#let(first=json(\"data.json\"), second=json(\"data.json\"))#paragraph[#span(properties=first.format)[#second.title]#first.empty]";
    let document = mote::compile_with_assets(
        source,
        &resolver,
        Some(OsStr::new("asset://bundle/main.mote")),
    )
    .unwrap();
    assert_eq!(requests.get(), 1);
    let expected = mote::compile("#paragraph[#span(bold=true,size=pt(12))[First]]").unwrap();
    assert_eq!(document, expected);
}

#[test]
fn json_reports_resource_errors_without_fallback_or_partial_content() {
    use mote::source::{AssetRequest, ResolvedAsset};
    use std::io;
    let failure = |_: AssetRequest<'_>| {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "denied by resolver",
        ))
    };
    let source = "Visible\n#json(\"data.json\")";
    let error = mote::compile_with_assets(source, &failure, None).unwrap_err();
    assert!(error.message.contains("denied by resolver"));
    assert_eq!(error.location(source).0, 2);
    for bytes in [
        b"{".as_slice(),
        b"true false",
        b"1e999",
        b"NaN",
        b"[1,]",
        b"\xff",
    ] {
        let resolver = |_: AssetRequest<'_>| {
            Ok(ResolvedAsset {
                name: "logical/data.json".into(),
                bytes: bytes.to_vec(),
            })
        };
        let error = mote::compile_with_assets(source, &resolver, None).unwrap_err();
        assert!(error.message.contains("logical/data.json"), "{error}");
        assert_eq!(error.location(source).0, 2);
    }
    for source in [
        "#json()",
        "#json(1)",
        "#json(path=\"data.json\")",
        "#json(\"data.json\", 1)",
        "#json(\"data.json\")[]",
    ] {
        let resolver = |_: AssetRequest<'_>| -> io::Result<ResolvedAsset> {
            panic!("invalid request reached resolver")
        };
        assert!(mote::compile_with_assets(source, &resolver, None).is_err());
    }
    assert!(
        mote::compile("#json(\"data.json\")")
            .unwrap_err()
            .message
            .contains("asset resolver")
    );
}

#[test]
fn json_rejects_conflicting_bytes_for_one_logical_name() {
    use mote::source::{AssetRequest, ResolvedAsset};
    let resolver = |request: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "shared.json".into(),
            bytes: if request.reference == "first.json" {
                b"1".to_vec()
            } else {
                b"2".to_vec()
            },
        })
    };
    let error = mote::compile_with_assets(
        "#let(first=json(\"first.json\"), second=json(\"second.json\"))",
        &resolver,
        None,
    )
    .unwrap_err();
    assert!(error.message.contains("conflicting bytes"));
}

#[test]
fn json_cli_resolves_file_and_standard_input_resources() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let output = Command::new(env!("CARGO_BIN_EXE_mote"))
        .current_dir(fixtures())
        .arg("reports/invoice.mote")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_mote"))
        .current_dir(fixtures().join("reports"))
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&fs::read(fixtures().join("reports/invoice.mote")).unwrap())
        .unwrap();
    let standard_input = child.wait_with_output().unwrap();
    assert!(standard_input.status.success());
    assert_eq!(output.stdout, standard_input.stdout);
}

#[test]
fn json_capture_inputs_remain_frozen() {
    use sha2::{Digest, Sha256};
    for case in ["values", "empty", "reports/invoice"] {
        let manifest: serde_json::Value = serde_json::from_slice(
            &fs::read(fixtures().join("captured").join(case).join("capture.json")).unwrap(),
        )
        .unwrap();
        let source = fs::read(fixtures().join(format!("{case}.mote"))).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source)),
            manifest["source_sha256"]
        );
        for (path, expected) in manifest["assets"].as_object().unwrap() {
            assert_eq!(
                format!(
                    "{:x}",
                    Sha256::digest(fs::read(fixtures().join(path)).unwrap())
                ),
                expected.as_str().unwrap()
            );
        }
    }
}
