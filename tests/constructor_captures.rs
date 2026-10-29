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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constructors")
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
fn document_constructors_preserve_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "configuration",
        "empty",
        "fields",
        "helpers",
        "media",
        "structures",
        "text",
    ] {
        let source = fs::read_to_string(fixtures().join(format!("{case}.mote"))).unwrap();
        let source_name = fixtures().join(format!("{case}.mote"));
        let document = mote::compile_with_options(
            &source,
            mote::evaluation::EvaluationOptions {
                enable_lua: true,
                asset_resolver: Some(&resolver),
                source_name: Some(source_name.as_os_str()),
                ..Default::default()
            },
        )
        .unwrap();
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

#[allow(dead_code)]
#[path = "support/native_capture.rs"]
mod capture;
#[test]
fn constructor_sources_and_native_pages_match_frozen_captures() {
    use mote::render::{self, PngOptions};
    use sha2::{Digest, Sha256};
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for entry in fs::read_dir(fixtures().join("captured")).unwrap() {
        let folder = entry.unwrap().path();
        let case = folder.file_name().unwrap().to_str().unwrap();
        let source_path = fixtures().join(format!("{case}.mote"));
        let source = fs::read_to_string(&source_path).unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
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
        let document = mote::compile_with_options(
            &source,
            mote::evaluation::EvaluationOptions {
                enable_lua: true,
                asset_resolver: Some(&resolver),
                source_name: Some(source_path.as_os_str()),
                ..Default::default()
            },
        )
        .unwrap();
        let plan = render::plan(&document, &resolver, Some(source_path.as_os_str())).unwrap();
        let pages = render::paint_png(&plan.output, &PngOptions::default()).unwrap();
        assert_eq!(
            pages.output.len(),
            manifest["native_pdf_pages"].as_u64().unwrap() as usize
        );
        for page in pages.output {
            let name = format!("reference-{}.png", page.number);
            let bytes = fs::read(folder.join(&name)).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&bytes)),
                manifest["native_png_files"][&name]
            );
            let (width, height, expected) = capture::pixels(&bytes);
            let (actual_width, actual_height, actual) = capture::pixels(&page.bytes);
            assert_eq!((actual_width, actual_height), (width, height));
            let mut maximum = 0u8;
            let mut sum = 0u64;
            let mut count = 0u64;
            for (actual, expected) in actual
                .as_chunks::<4>()
                .0
                .iter()
                .zip(expected.as_chunks::<4>().0)
            {
                if actual[..3] == [255; 3] && expected[..3] == [255; 3] {
                    continue;
                }
                for channel in 0..3 {
                    let difference = actual[channel].abs_diff(expected[channel]);
                    maximum = maximum.max(difference);
                    sum += u64::from(difference);
                    count += 1;
                }
            }
            let mean = sum as f64 / count.max(1) as f64;
            assert!(
                maximum <= 56 && mean <= 12.0,
                "{case}: maximum {maximum}, mean {mean}"
            );
        }
    }
}
