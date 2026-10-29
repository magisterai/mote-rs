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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sections-stories")
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
    "balanced",
    "basic",
    "continuous",
    "default-changes",
    "empty-story-lines",
    "field-flow",
    "field-formats",
    "geometry",
    "local-stories",
    "overflow",
    "page-sizes",
    "rich-stories",
    "starts",
    "story-media",
];
#[test]
fn section_stories_and_fields_match_frozen_geometry() {
    for case in CASES {
        let expected: Value = serde_json::from_slice(
            &fs::read(fixtures().join("captured").join(case).join("geometry.json")).unwrap(),
        )
        .unwrap();
        compare(&projection(&planned(case).output), &expected, case);
    }
}
#[test]
fn section_rasters_match_frozen_pages() {
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
fn native_limits_are_visible_and_page_geometry_is_local() {
    let formats = planned("field-formats");
    assert!(
        formats
            .warnings
            .iter()
            .any(|warning| warning.contains("page_numbering.format"))
    );
    assert!(
        formats
            .warnings
            .iter()
            .any(|warning| warning.contains("page_number.format"))
    );
    assert!(
        planned("overflow")
            .warnings
            .iter()
            .any(|warning| warning.contains("trailing lines are omitted"))
    );
    assert!(
        planned("story-media")
            .warnings
            .iter()
            .any(|warning| warning.contains("placeholder"))
    );
    let plan = planned("geometry").output;
    assert_eq!(
        plan.pages
            .iter()
            .map(|page| page.geometry.unwrap().width)
            .collect::<Vec<_>>(),
        vec![360.0, 360.0, 360.0, 360.0]
    );
    assert_eq!(
        plan.pages
            .iter()
            .map(|page| page.geometry.unwrap().height)
            .collect::<Vec<_>>(),
        vec![300.0, 300.0, 300.0, 300.0]
    );
    assert_eq!(
        plan.pages
            .iter()
            .map(|page| page.page_number)
            .collect::<Vec<_>>(),
        vec![1, 7, 8, 9]
    );
}

#[test]
fn mixed_page_sizes_reach_both_painters() {
    let plan = planned("page-sizes").output;
    let dimensions: Vec<_> = plan
        .pages
        .iter()
        .map(|page| {
            let geometry = page.geometry.unwrap();
            (geometry.width, geometry.height)
        })
        .collect();
    assert_eq!(
        dimensions,
        vec![(300.0, 360.0), (360.0, 300.0), (300.0, 360.0)]
    );
    assert!(!render::paint_pdf(&plan).unwrap().is_empty());
    let png = render::paint_png(&plan, &PngOptions::default()).unwrap();
    assert_eq!(
        png.output
            .iter()
            .map(|page| (page.width, page.height))
            .collect::<Vec<_>>(),
        vec![(300, 360), (360, 300), (300, 360)]
    );
}
