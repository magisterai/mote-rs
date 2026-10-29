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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/captions")
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
    "flow",
    "formatting",
    "grouping",
    "image",
    "metadata",
    "nested",
    "position",
    "sequences",
    "stories",
];
#[test]
fn captions_match_frozen_geometry() {
    for case in CASES {
        let expected: Value = serde_json::from_slice(
            &fs::read(fixtures().join("captured").join(case).join("geometry.json")).unwrap(),
        )
        .unwrap();
        compare(&projection(&planned(case).output), &expected, case);
    }
}
#[test]
fn caption_rasters_match_frozen_pages() {
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
fn caption_targets_follow_the_placed_caption_after_natural_pagination() {
    let rendered = planned("flow");
    assert_eq!(rendered.output.pages.len(), 2);
    assert!(!rendered.output.pages[0].anchors.contains_key("long"));
    assert!(rendered.output.pages[1].anchors.contains_key("long"));
    let fragments: Vec<_> = rendered.output.pages[1]
        .lines
        .iter()
        .flat_map(|placed| &placed.line.fragments)
        .collect();
    assert!(fragments.iter().any(
        |fragment| fragment.internal_target.as_deref() == Some("long") && fragment.text == "2"
    ));
    let pdf = render::paint_pdf(&rendered.output).unwrap();
    assert!(String::from_utf8_lossy(&pdf).contains("/GoTo"));
}
#[test]
fn caption_numbering_limits_remain_explicit() {
    for case in ["nested", "stories"] {
        let rendered = planned(case);
        assert!(
            rendered
                .warnings
                .iter()
                .any(|warning| warning.contains("reference numbering can differ")),
            "{case}"
        );
    }
    let rendered = planned("image");
    assert!(
        rendered
            .warnings
            .iter()
            .any(|warning| warning.contains("placement"))
    );
    assert!(
        rendered
            .warnings
            .iter()
            .any(|warning| warning.contains("style"))
    );
}
