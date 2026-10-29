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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tables")
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
#[test]
fn table_text_edges_images_and_pagination_match_frozen_geometry() {
    for case in [
        "basic",
        "attributes",
        "borders",
        "cell-blocks",
        "cell-list-order",
        "column-flow",
        "content",
        "markdown",
        "merges",
        "objects",
        "oversized-merge",
        "pagination",
        "rowspan-flow",
        "styles",
        "widths",
    ] {
        let expected: Value = serde_json::from_slice(
            &fs::read(fixtures().join("captured").join(case).join("geometry.json")).unwrap(),
        )
        .unwrap();
        compare(&projection(&planned(case).output), &expected, case);
    }
}
#[test]
fn native_table_pages_match_frozen_rasters_with_bounded_font_edge_tolerance() {
    for case in [
        "basic",
        "attributes",
        "borders",
        "cell-blocks",
        "cell-list-order",
        "column-flow",
        "content",
        "markdown",
        "merges",
        "objects",
        "oversized-merge",
        "pagination",
        "rowspan-flow",
        "styles",
        "widths",
    ] {
        let planned = planned_with_fonts(case, true);
        if case == "styles" {
            assert!(
                planned
                    .output
                    .pages
                    .iter()
                    .flat_map(|page| &page.lines)
                    .flat_map(|line| &line.line.fragments)
                    .filter(|fragment| fragment.style.measurement.font == "Arial")
                    .all(
                        |fragment| fragment.font.source == mote::measurement::FontSource::Installed
                    ),
                "the styles raster capture requires installed Arial; portable geometry is tested separately"
            );
        }
        let capture = fixtures().join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(capture.join("native-capture.json")).unwrap())
                .unwrap();
        let painted = render::paint_png(&planned.output, &PngOptions::default()).unwrap();
        assert!(
            painted
                .warnings
                .iter()
                .all(|warning| !warning.contains("outline fallback")),
            "{case}: raster capture fonts are unavailable: {:?}",
            painted.warnings
        );
        let painted = painted.output;
        assert_eq!(painted.len(), manifest["pages"].as_u64().unwrap() as usize);
        for page in painted {
            let actual = pixels(&page.bytes);
            let expected =
                pixels(&fs::read(capture.join(format!("reference-{}.png", page.number))).unwrap());
            assert_eq!((actual.0, actual.1), (expected.0, expected.1));
            let mut total = 0_u64;
            let mut channels = 0_u64;
            let mut maximum = 0;
            for (actual, expected) in actual
                .2
                .as_chunks::<4>()
                .0
                .iter()
                .zip(expected.2.as_chunks::<4>().0)
            {
                assert_eq!(actual[3], 255);
                if actual[..3] != [255; 3] || expected[..3] != [255; 3] {
                    channels += 3;
                    for channel in 0..3 {
                        let difference = actual[channel].abs_diff(expected[channel]);
                        maximum = maximum.max(difference);
                        total += u64::from(difference);
                    }
                }
            }
            let mean = total as f64 / channels.max(1) as f64;
            assert!(
                maximum <= manifest["png_maximum_channel_difference"].as_u64().unwrap() as u8
                    && mean <= manifest["png_mean_foreground_difference"].as_f64().unwrap(),
                "{case}/{}: maximum {maximum}, foreground mean {mean}",
                page.number
            );
        }
        let pdf = render::paint_pdf(&planned.output).unwrap();
        assert_eq!(pdf, render::paint_pdf(&planned.output).unwrap());
        assert!(pdf.starts_with(b"%PDF-1.7"));
    }
}
#[test]
fn repeated_table_headers_in_columns_have_distinct_page_operations() {
    let plan = planned("columns").output;
    assert!(plan.pages.len() >= 2);
    let headings: Vec<_> = plan.pages[0]
        .lines
        .iter()
        .filter(|line| {
            line.line
                .fragments
                .iter()
                .map(|fragment| fragment.text.as_str())
                .collect::<String>()
                == "Code"
        })
        .collect();
    assert_eq!(headings.len(), 2);
    assert!(headings[1].line.x > headings[0].line.x);
    render::paint_pdf(&plan).unwrap();
    render::paint_png(&plan, &PngOptions::default()).unwrap();
    let metadata: Value = serde_json::from_slice(
        &fs::read(fixtures().join("captured/columns/native-capture.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(metadata["native_reference_status"], "blocked");
}
#[test]
fn output_limits_are_reported_without_warning_for_implemented_table_fields() {
    let warnings = planned("attributes").warnings;
    for field in [
        ".height",
        ".height_rule",
        ".text_direction",
        ".no_wrap",
        ".indent",
        ".cell_spacing",
        ".layout",
    ] {
        assert!(
            warnings.iter().any(|warning| warning.contains(field)),
            "{field}: {warnings:?}"
        );
    }
    for field in [
        ".width is retained",
        ".margin is retained",
        ".repeat_header is retained",
        ".cant_split is retained",
        ".fill is retained",
    ] {
        assert!(
            !warnings.iter().any(|warning| warning.contains(field)),
            "{field}: {warnings:?}"
        );
    }
    assert!(
        planned("objects")
            .warnings
            .iter()
            .all(|warning| !warning.contains("placeholder"))
    );
}
#[test]
fn cell_clipping_is_balanced_and_validated_before_both_painters() {
    let plan = planned("objects").output;
    let mut missing = plan.clone();
    missing.pages[0].operations.push(PageOperation::PopClip);
    let mut unclosed = plan.clone();
    unclosed.pages[0]
        .operations
        .push(PageOperation::PushClip([0.0, 0.0, 10.0, 10.0]));
    let mut invalid = plan.clone();
    invalid.pages[0]
        .operations
        .push(PageOperation::PushClip([0.0, 0.0, f64::NAN, 10.0]));
    for invalid in [missing, unclosed, invalid] {
        assert!(render::paint_pdf(&invalid).is_err());
        assert!(render::paint_png(&invalid, &PngOptions::default()).is_err());
    }
}

#[test]
fn oversized_unsplittable_merges_warn_and_preserve_following_content() {
    let planned = planned("oversized-merge");
    assert_eq!(planned.output.pages.len(), 3);
    assert!(
        planned
            .warnings
            .iter()
            .any(|warning| warning.contains("rowspans are reduced to individual rows"))
    );
    let final_text: String = planned.output.pages[2]
        .lines
        .iter()
        .flat_map(|line| &line.line.fragments)
        .map(|fragment| fragment.text.as_str())
        .collect();
    assert_eq!(final_text, "After the table.");
}
#[test]
fn diagonal_patterned_rules_respect_dash_gaps_and_nested_clipping() {
    let mut plan = planned("basic").output;
    plan.pages.truncate(1);
    plan.pages[0].lines.clear();
    plan.pages[0].operations = vec![
        PageOperation::PushClip([20.0, 20.0, 60.0, 60.0]),
        PageOperation::PushClip([30.0, 30.0, 30.0, 30.0]),
        PageOperation::Stroke {
            rule: mote::layout::Rule {
                role: mote::layout::RuleRole::Generic,
                start: [40.0, 40.0],
                end: [80.0, 80.0],
                width: 4.0,
                color: "224466".into(),
            },
            dash: vec![10.0, 10.0],
            round_cap: false,
        },
        PageOperation::PopClip,
        PageOperation::PopClip,
    ];
    let painted = render::paint_png(&plan, &PngOptions::default()).unwrap();
    let (width, _, bytes) = pixels(&painted.output[0].bytes);
    let sample = |position: usize| &bytes[(position * width as usize + position) * 4..][..3];
    assert_eq!(sample(44), [34, 68, 102]);
    assert_eq!(sample(51), [255; 3]);
    assert_eq!(sample(58), [34, 68, 102]);
    assert_eq!(sample(65), [255; 3]);
    render::paint_pdf(&plan).unwrap();
}
