#[allow(dead_code)]
#[path = "support/native_capture.rs"]
mod capture;
use mote::{layout::PageOperation, render, source::FileAssetResolver};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

#[test]
fn preset_paths_transforms_shadows_and_anchor_flow_match_captures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/shape-geometry");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in ["presets", "effects", "flow"] {
        let source = fs::read_to_string(root.join(format!("{case}.mote"))).unwrap();
        let folder = root.join("captured").join(case);
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        let document = mote::compile(&source).unwrap();
        let plan = render::plan(&document, &resolver, None).unwrap();
        let paths:Vec<_>=plan.output.pages.iter().map(|page| page.operations.iter().filter_map(|operation| {
            let PageOperation::Path(path)=operation else {return None};
            Some(json!({"points":path.points,"closed":path.closed,"fill":path.fill.as_ref().map(|paint|json!({"color":paint.color,"alpha":paint.alpha})),"stroke":path.stroke.as_ref().map(|stroke|json!({"color":stroke.paint.color,"alpha":stroke.paint.alpha,"width":stroke.width,"dash":stroke.dash,"cap":format!("{:?}",stroke.cap)}))}))
        }).collect::<Vec<_>>()).collect();
        let expected: serde_json::Value =
            serde_json::from_slice(&fs::read(folder.join("paths.json")).unwrap()).unwrap();
        capture::compare(&json!(paths), &expected, case);
        let pages = render::paint_png(&plan.output, &Default::default())
            .unwrap()
            .output;
        assert_eq!(
            pages.len(),
            manifest["native_png_files"].as_object().unwrap().len()
        );
        for page in pages {
            let name = format!("reference-{}.png", page.number);
            let bytes = fs::read(folder.join(&name)).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&bytes)),
                manifest["native_png_files"][&name]
            );
            let (width, height, expected) = capture::pixels(&bytes);
            let (actual_width, actual_height, actual) = capture::pixels(&page.bytes);
            assert_eq!((width, height), (actual_width, actual_height));
            let mut maximum = 0u8;
            let mut sum = 0u64;
            let mut count = 0u64;
            let mut high_difference = 0usize;
            for (actual, expected) in actual
                .as_chunks::<4>()
                .0
                .iter()
                .zip(expected.as_chunks::<4>().0)
            {
                if actual[..3] == [255; 3] && expected[..3] == [255; 3] {
                    continue;
                }
                let differences: Vec<_> = (0..3)
                    .map(|channel| actual[channel].abs_diff(expected[channel]))
                    .collect();
                high_difference += usize::from(differences.iter().any(|value| *value > 64));
                for difference in differences {
                    maximum = maximum.max(difference);
                    sum += u64::from(difference);
                    count += 1;
                }
            }
            let mean = sum as f64 / count.max(1) as f64;
            assert!(
                maximum <= if case == "presets" { 112 } else { 56 },
                "{case}: maximum {maximum}"
            );
            assert!(
                mean <= if case == "presets" { 1.0 } else { 12.0 },
                "{case}: mean {mean}"
            );
            assert!(
                high_difference <= if case == "presets" { 90 } else { 0 },
                "{case}: {high_difference} edge pixels"
            );
        }
    }
}

#[test]
fn path_painters_reject_invalid_geometry_before_writing_output() {
    use mote::layout::{LineCap, PaintedPath, PathPaint, PathStroke};
    let resolver = FileAssetResolver::new(".").unwrap();
    let mut plan = render::plan(&mote::compile("Text").unwrap(), &resolver, None)
        .unwrap()
        .output;
    for path in [
        PaintedPath {
            points: vec![[0.0, 0.0]],
            closed: true,
            fill: None,
            stroke: None,
        },
        PaintedPath {
            points: vec![[f64::NAN, 0.0], [1.0, 1.0]],
            closed: false,
            fill: None,
            stroke: None,
        },
        PaintedPath {
            points: vec![[0.0, 0.0], [1.0, 1.0]],
            closed: false,
            fill: None,
            stroke: Some(PathStroke {
                paint: PathPaint {
                    color: "000000".into(),
                    alpha: 255,
                },
                width: 1.0,
                dash: vec![0.0],
                cap: LineCap::Flat,
            }),
        },
    ] {
        plan.pages[0].operations.push(PageOperation::Path(path));
        assert!(render::paint_pdf(&plan).is_err());
        assert!(render::paint_png(&plan, &Default::default()).is_err());
        plan.pages[0].operations.pop();
    }
}
