use mote::{
    layout::{DocumentTextPlan, PageOperation},
    render::{self, PngOptions},
    source::FileAssetResolver,
};
use serde_json::{Value, json};
use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/references")
}
fn planned(case: &str) -> render::Rendered<DocumentTextPlan> {
    planned_with_fonts(case, false)
}
fn planned_with_fonts(case: &str, installed: bool) -> render::Rendered<DocumentTextPlan> {
    let path = fixtures().join(format!("{case}.mote"));
    let document = mote::compile(&fs::read_to_string(&path).unwrap()).unwrap();
    let original = document.clone();
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    let planned = if installed {
        render::plan(&document, &resolver, Some(path.as_os_str())).unwrap()
    } else {
        render::plan_with_fonts(&document, &resolver, Some(path.as_os_str()), None).unwrap()
    };
    assert_eq!(
        document, original,
        "{case}: planning changed the source model"
    );
    planned
}
fn projection(plan: &DocumentTextPlan) -> Value {
    let pages: Vec<_> = plan
        .pages
        .iter()
        .map(|page| {
            let lines: Vec<_> = page
                .lines
                .iter()
                .map(|placed| {
                    let fragments: Vec<_> = placed
                        .line
                        .fragments
                        .iter()
                        .map(|fragment| {
                            json!({
                                "text": fragment.text,
                                "x": fragment.x,
                                "baseline": fragment.baseline,
                                "advance": fragment.advance,
                                "size": fragment.style.measurement.size_points,
                            })
                        })
                        .collect();
                    json!({
                        "top": placed.top,
                        "height": placed.line.height,
                        "x": placed.line.x,
                        "width": if fragments.is_empty() { 0.0 } else { placed.line.width },
                        "fragments": fragments,
                    })
                })
                .collect();
            let rules: Vec<_> = page
                .operations
                .iter()
                .filter_map(|operation| {
                    let (rule, dash) = match operation {
                        PageOperation::Rule(rule) => (rule, &[][..]),
                        PageOperation::Stroke { rule, dash, .. } => (rule, dash.as_slice()),
                        _ => return None,
                    };
                    Some(json!({
                        "start": rule.start,
                        "end": rule.end,
                        "width": rule.width,
                        "color": rule.color,
                        "dash": dash,
                    }))
                })
                .collect();
            let images: Vec<_> = page
                .operations
                .iter()
                .filter_map(|operation| match operation {
                    PageOperation::Image(image) => Some(image.bounds),
                    _ => None,
                })
                .collect();
            json!({ "lines": lines, "rules": rules, "images": images })
        })
        .collect();
    json!({ "pages": pages })
}

fn compare(actual: &Value, expected: &Value, path: &str) {
    match (actual, expected) {
        (Value::Number(actual), Value::Number(expected)) => assert!(
            (actual.as_f64().unwrap() - expected.as_f64().unwrap()).abs() <= 1e-8,
            "{path}: {actual} != {expected}"
        ),
        (Value::Array(actual), Value::Array(expected)) => {
            assert_eq!(actual.len(), expected.len(), "{path}: array length");
            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                compare(actual, expected, &format!("{path}[{index}]"));
            }
        }
        (Value::Object(actual), Value::Object(expected)) => {
            assert_eq!(
                actual.keys().collect::<Vec<_>>(),
                expected.keys().collect::<Vec<_>>(),
                "{path}: fields"
            );
            for (key, actual) in actual {
                compare(actual, &expected[key], &format!("{path}.{key}"));
            }
        }
        _ => assert_eq!(actual, expected, "{path}"),
    }
}
fn pixels(bytes: &[u8]) -> (u32, u32, Vec<u8>) {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA);
    let mut reader = decoder.read_info().unwrap();
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut pixels).unwrap();
    pixels.truncate(info.buffer_size());
    assert_eq!(info.color_type, png::ColorType::Rgba);
    (info.width, info.height, pixels)
}

const CASES: &[&str] = &[
    "basic",
    "missing",
    "names",
    "page-flow",
    "placement",
    "story-labels",
    "wrapping",
];
#[test]
fn references_match_frozen_geometry() {
    for case in CASES {
        let expected: Value = serde_json::from_slice(
            &fs::read(fixtures().join("captured").join(case).join("geometry.json")).unwrap(),
        )
        .unwrap();
        compare(&projection(&planned(case).output), &expected, case);
    }
}
#[test]
fn reference_rasters_match_frozen_pages() {
    for case in CASES {
        let plan = planned_with_fonts(case, true);
        let actual = render::paint_png(&plan.output, &PngOptions::default()).unwrap();
        let folder = fixtures().join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(folder.join("native-capture.json")).unwrap()).unwrap();
        assert_eq!(
            actual.output.len(),
            manifest["pages"].as_u64().unwrap() as usize
        );
        for page in actual.output {
            let (width, height, expected) =
                pixels(&fs::read(folder.join(format!("reference-{}.png", page.number))).unwrap());
            let (actual_width, actual_height, actual) = pixels(&page.bytes);
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
                maximum <= 56 && mean <= 11.0,
                "{case} page {}: maximum {maximum}, mean {mean}",
                page.number
            );
        }
    }
}

#[test]
fn page_references_resolve_numbering_and_pdf_destinations() {
    let plan = planned("page-flow").output;
    assert_eq!(
        plan.pages
            .iter()
            .map(|page| page.page_number)
            .collect::<Vec<_>>(),
        [1, 8, 9, 10]
    );
    assert!(plan.pages[3].anchors.contains_key("end"));
    let first = &plan.pages[0].lines[0].line.fragments;
    let reference = first
        .iter()
        .find(|fragment| fragment.internal_target.as_deref() == Some("end"))
        .unwrap();
    assert_eq!(reference.text, "10");
    let pdf = render::paint_pdf(&plan).unwrap();
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("/GoTo"));
    assert!(text.contains("/XYZ"));
    assert!(!text.contains("/URI"));
    let wrapping = planned("wrapping").output;
    assert_eq!(
        wrapping
            .pages
            .iter()
            .filter(|page| page.anchors.contains_key("wrapped"))
            .count(),
        1
    );
}
#[test]
fn unplaced_destinations_warn_without_fabricated_links() {
    for case in ["missing", "story-labels"] {
        let rendered = planned(case);
        assert!(
            rendered
                .warnings
                .iter()
                .any(|warning| warning.contains("no placed native destination"))
        );
        assert!(
            rendered
                .output
                .pages
                .iter()
                .flat_map(|page| &page.lines)
                .flat_map(|placed| &placed.line.fragments)
                .all(|fragment| fragment.internal_target.is_none())
        );
    }
}
