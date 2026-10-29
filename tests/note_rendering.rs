#[path = "support/native_capture.rs"]
mod capture;
use capture::{compare, pixels, projection};
use mote::{
    layout::DocumentTextPlan,
    render::{self, PngOptions},
    source::FileAssetResolver,
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/notes")
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
const CASES: &[&str] = &[
    "basic",
    "markdown",
    "rich",
    "stories",
    "pressure",
    "multiple",
    "endnote-flow",
    "endnote-footer",
    "fallbacks",
    "fields",
    "overflow",
];
#[test]
fn notes_match_frozen_geometry() {
    for case in CASES {
        let expected: Value = serde_json::from_slice(
            &fs::read(fixtures().join("captured").join(case).join("geometry.json")).unwrap(),
        )
        .unwrap();
        compare(&projection(&planned(case).output), &expected, case);
    }
}
#[test]
fn note_rasters_match_frozen_pages() {
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
fn note_reservations_and_fallbacks_are_observable() {
    let pressure = planned("pressure");
    assert_eq!(pressure.output.pages.len(), 3);
    let mut reserved = 0;
    for page in &pressure.output.pages {
        if let Some(bottom) = page.note_body_bottom {
            reserved += 1;
            assert!(bottom < page.geometry.unwrap().content_bottom());
            assert!(
                page.lines
                    .iter()
                    .flat_map(|placed| &placed.line.fragments)
                    .any(|fragment| fragment.footnote_number.is_some())
            );
        }
    }
    assert_eq!(reserved, 2);
    assert!(
        planned("overflow")
            .warnings
            .iter()
            .any(|warning| warning.contains("oversized footnote"))
    );
    for case in ["fallbacks", "fields", "stories", "rich"] {
        assert!(
            planned(case)
                .warnings
                .iter()
                .any(|warning| warning.contains("flattened inline")),
            "{case}"
        );
    }
    let endnotes = planned("endnote-flow");
    assert_eq!(endnotes.output.pages.len(), 3);
    assert!(endnotes.output.pages.iter().all(|page| page.operations.iter().any(|operation| matches!(operation, mote::layout::PageOperation::Rule(rule) if rule.role == mote::layout::RuleRole::NoteSeparator))));
}
