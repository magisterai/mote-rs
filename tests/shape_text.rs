#[allow(dead_code)]
#[path = "support/native_capture.rs"]
mod capture;
use mote::{
    layout::{DocumentTextPlan, PageOperation, TaskMark},
    render,
    source::FileAssetResolver,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn effects(plan: &DocumentTextPlan) -> Value {
    Value::Array(plan.pages.iter().map(|page| {
        let mut clips = Vec::new();
        let mut rotations = Vec::new();
        let mut markers = Vec::new();
        for operation in &page.operations {
            match operation {
                PageOperation::PushClip(bounds) => clips.push(json!(bounds)),
                PageOperation::PushRotation { degrees, center } => {
                    rotations.push(json!({"degrees": degrees, "center": center}));
                }
                PageOperation::ListMarker(marker) => markers.push(json!({
                    "checked": marker.checked,
                    "slot": marker.slot,
                    "baseline": marker.baseline,
                    "font_size": marker.font_size,
                    "task_mark": match marker.task_mark { TaskMark::Check => "check", TaskMark::Cross => "cross" },
                })),
                _ => {}
            }
        }
        json!({"clips": clips, "rotations": rotations, "markers": markers})
    }).collect())
}

#[test]
fn drawing_text_geometry_and_independent_rasters_match_frozen_captures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/shape-text");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for (case, maximum_limit, mean_limit, edge_limit) in [
        ("fitting", 80, 3.0, 20),
        ("formatting", 108, 9.0, 2),
        ("presets", 56, 1.0, 0),
        ("rotation", 64, 2.0, 0),
    ] {
        let source = fs::read_to_string(root.join(format!("{case}.mote"))).unwrap();
        let folder = root.join("captured").join(case);
        let read =
            |name| serde_json::from_slice::<Value>(&fs::read(folder.join(name)).unwrap()).unwrap();
        let manifest = read("capture.json");
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        let document = mote::compile(&source).unwrap();
        let original = document.clone();
        let plan = render::plan(&document, &resolver, None).unwrap();
        assert_eq!(document, original);
        capture::compare(
            &capture::projection(&plan.output),
            &read("geometry.json"),
            case,
        );
        capture::compare(&effects(&plan.output), &read("effects.json"), case);
        let pages = render::paint_png(&plan.output, &Default::default())
            .unwrap()
            .output;
        assert_eq!(
            pages.len(),
            manifest["native_png_files"].as_object().unwrap().len()
        );
        for page in pages {
            let name = format!("reference-{}.png", page.number);
            let expected = fs::read(folder.join(&name)).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&expected)),
                manifest["native_png_files"][&name]
            );
            let (width, height, expected) = capture::pixels(&expected);
            let (actual_width, actual_height, actual) = capture::pixels(&page.bytes);
            assert_eq!((width, height), (actual_width, actual_height));
            let mut maximum = 0u8;
            let mut sum = 0u64;
            let mut count = 0u64;
            let mut edge_pixels = 0usize;
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
                edge_pixels += usize::from(differences.iter().any(|value| *value > 64));
                for difference in differences {
                    maximum = maximum.max(difference);
                    sum += u64::from(difference);
                    count += 1;
                }
            }
            let mean = sum as f64 / count.max(1) as f64;
            assert!(maximum <= maximum_limit, "{case}: maximum {maximum}");
            assert!(mean <= mean_limit, "{case}: mean {mean}");
            assert!(
                edge_pixels <= edge_limit,
                "{case}: {edge_pixels} edge pixels"
            );
        }
    }
}

#[test]
fn drawing_text_reports_omitted_content_and_unrotated_link_hit_areas() {
    let document = mote::compile(
        "#shape(position={width=inch(3),height=inch(2)},rotation=30)[\
        #paragraph[#link('https://example.com')[Linked]]\
        #table[#row[#cell[Omitted]]]#paragraph[Field #page_number()]\
        ]",
    )
    .unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let plan = render::plan(&document, &resolver, None).unwrap();
    assert!(
        plan.warnings
            .iter()
            .any(|warning| warning.contains("PDF link hit areas remain at unrotated positions"))
    );
    assert!(
        plan.warnings
            .iter()
            .any(|warning| warning.contains("table is omitted from native drawing text"))
    );
    assert!(
        plan.warnings
            .iter()
            .any(|warning| warning.contains("field, anchor and note semantics are omitted"))
    );
}

#[test]
fn painters_reject_unbalanced_and_crossing_rotation_scopes() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let original = render::plan(&mote::compile("Text").unwrap(), &resolver, None)
        .unwrap()
        .output;
    let rotation = PageOperation::PushRotation {
        degrees: 30.0,
        center: [72.0, 72.0],
    };
    let clip = PageOperation::PushClip([0.0, 0.0, 200.0, 200.0]);
    for operations in [
        vec![rotation.clone()],
        vec![PageOperation::PopRotation],
        vec![
            PageOperation::PushRotation {
                degrees: f64::NAN,
                center: [0.0; 2],
            },
            PageOperation::PopRotation,
        ],
        vec![
            clip.clone(),
            rotation.clone(),
            PageOperation::PopClip,
            clip,
            PageOperation::PopRotation,
            PageOperation::PopClip,
        ],
        vec![rotation; 17],
    ] {
        let mut plan = original.clone();
        plan.pages[0].operations.extend(operations);
        assert!(render::paint_pdf(&plan).is_err());
        assert!(render::paint_png(&plan, &Default::default()).is_err());
    }
}
