#[allow(dead_code)]
#[path = "native_capture.rs"]
mod capture;
#[allow(dead_code)]
#[path = "../docx/support.rs"]
mod xml_support;
use mote::{evaluation::EvaluationOptions, source::FileAssetResolver};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Read},
    path::Path,
};

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

fn compare_part(actual: &[u8], expected: &[u8], name: &str) {
    if name.ends_with(".xlsx") {
        let mut actual = zip::ZipArchive::new(Cursor::new(actual)).unwrap();
        let mut expected = zip::ZipArchive::new(Cursor::new(expected)).unwrap();
        let actual_names: std::collections::BTreeSet<_> = actual
            .file_names()
            .map(|name| name.unwrap().into_owned())
            .collect();
        let expected_names: std::collections::BTreeSet<_> = expected
            .file_names()
            .map(|name| name.unwrap().into_owned())
            .collect();
        assert_eq!(actual_names.len(), actual.len(), "{name}: duplicate parts");
        assert_eq!(
            expected_names.len(),
            expected.len(),
            "{name}: duplicate parts"
        );
        assert_eq!(actual_names, expected_names, "{name}: workbook parts");
        for part in actual_names {
            let mut actual_bytes = Vec::new();
            let mut expected_bytes = Vec::new();
            actual
                .by_name(&part)
                .unwrap()
                .read_to_end(&mut actual_bytes)
                .unwrap();
            expected
                .by_name(&part)
                .unwrap()
                .read_to_end(&mut expected_bytes)
                .unwrap();
            compare_part(&actual_bytes, &expected_bytes, &format!("{name}/{part}"));
        }
    } else if name.ends_with(".xml") || name.ends_with(".rels") {
        xml_support::compare_xml(
            std::str::from_utf8(actual).unwrap(),
            std::str::from_utf8(expected).unwrap(),
            name,
        );
    } else {
        assert!(actual == expected, "{name}: binary contents");
    }
}

pub fn compare(group: &str, cases: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(group);
    let resolver = FileAssetResolver::new(&root).unwrap();
    for &case in cases {
        let path = root.join(format!("{case}.mote"));
        let source = fs::read_to_string(&path).unwrap();
        let folder = root.join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        for (asset, digest) in manifest["assets"].as_object().unwrap() {
            assert_eq!(
                format!("{:x}", Sha256::digest(fs::read(root.join(asset)).unwrap())),
                digest.as_str().unwrap()
            );
        }
        let document = mote::compile_with_options(
            &source,
            EvaluationOptions {
                asset_resolver: Some(&resolver),
                source_name: Some(path.as_os_str()),
                enable_lua: true,
                ..Default::default()
            },
        )
        .unwrap();
        let original = document.clone();
        let mut actual: Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: Value = serde_json::from_str(
            &fs::read_to_string(folder.join("model.json"))
                .unwrap()
                .replace("${fixture}", &root.to_string_lossy()),
        )
        .unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}: model");
        let package = if manifest["format"] == "docx" {
            mote::docx::emit_with_assets(&document, &resolver, Some(path.as_os_str()))
                .unwrap()
                .bytes
        } else {
            mote::pptx::emit_with_assets(&document, &resolver, Some(path.as_os_str()))
                .unwrap()
                .bytes
        };
        let mut archive = zip::ZipArchive::new(Cursor::new(package)).unwrap();
        let parts = manifest["parts"].as_object().unwrap();
        assert_eq!(archive.len(), parts.len());
        for (name, part) in parts {
            let mut actual = Vec::new();
            archive
                .by_name(name)
                .unwrap()
                .read_to_end(&mut actual)
                .unwrap();
            let expected = fs::read(root.join(part["path"].as_str().unwrap())).unwrap();
            assert_eq!(format!("{:x}", Sha256::digest(&expected)), part["sha256"]);
            compare_part(&actual, &expected, &format!("{case}/{name}"));
        }
        let planned = mote::render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
        assert!(
            mote::render::paint_pdf(&planned.output)
                .unwrap()
                .starts_with(b"%PDF-"),
            "{case}: PDF output"
        );
        for warning in manifest["warnings"].as_array().into_iter().flatten() {
            let expected = warning.as_str().unwrap();
            assert!(
                planned
                    .warnings
                    .iter()
                    .any(|warning| warning.contains(expected)),
                "{case}: missing warning {expected}"
            );
        }
        let pages = mote::render::paint_png(&planned.output, &Default::default())
            .unwrap()
            .output;
        assert_eq!(pages.len(), manifest["pages"].as_array().unwrap().len());
        for (page, expected) in pages.iter().zip(manifest["pages"].as_array().unwrap()) {
            let bytes = fs::read(folder.join(expected["path"].as_str().unwrap())).unwrap();
            assert_eq!(format!("{:x}", Sha256::digest(&bytes)), expected["sha256"]);
            let (width, height, reference) = capture::pixels(&bytes);
            let (actual_width, actual_height, actual) = capture::pixels(&page.bytes);
            assert_eq!((actual_width, actual_height), (width, height));
            let mut maximum = 0u8;
            let mut sum = 0u64;
            let mut count = 0u64;
            let mut outliers = 0usize;
            for (actual, reference) in actual
                .as_chunks::<4>()
                .0
                .iter()
                .zip(reference.as_chunks::<4>().0)
            {
                if actual[..3] == [255; 3] && reference[..3] == [255; 3] {
                    continue;
                }
                let differences: Vec<_> = (0..3)
                    .map(|channel| actual[channel].abs_diff(reference[channel]))
                    .collect();
                outliers += usize::from(differences.iter().any(|difference| *difference > 64));
                for difference in differences {
                    maximum = maximum.max(difference);
                    sum += u64::from(difference);
                    count += 1;
                }
            }
            assert!(
                u64::from(maximum) <= expected["maximum"].as_u64().unwrap(),
                "{case}: {maximum}"
            );
            assert!(
                sum as f64 / count.max(1) as f64 <= expected["mean"].as_f64().unwrap(),
                "{case}: mean"
            );
            assert!(
                outliers <= expected["outliers"].as_u64().unwrap() as usize,
                "{case}: {outliers}"
            );
        }
        assert_eq!(document, original);
    }
}
