#[allow(dead_code)]
#[path = "support/native_capture.rs"]
mod capture;
#[path = "support/chart_capture.rs"]
mod charts;
use mote::{render, source::FileAssetResolver};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

#[test]
fn native_bar_charts_preserve_geometry_paint_order_and_independent_pngs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/chart-native-bars");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in ["kinds", "appearance", "labels", "flow"] {
        let source = fs::read_to_string(root.join(format!("{case}.mote"))).unwrap();
        let folder = root.join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        let document = mote::compile(&source).unwrap();
        let original = document.clone();
        let plan = render::plan(&document, &resolver, None).unwrap();
        assert_eq!(document, original);
        assert!(plan.warnings.is_empty(), "{case}: {:?}", plan.warnings);
        let geometry = fs::read(folder.join("geometry.json")).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&geometry)),
            manifest["geometry_sha256"]
        );
        capture::compare(
            &charts::projection(&plan.output),
            &serde_json::from_slice(&geometry).unwrap(),
            case,
        );
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
            let mut edges = 0usize;
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
                edges += usize::from(differences.iter().any(|difference| *difference > 64));
                for difference in differences {
                    maximum = maximum.max(difference);
                    sum += u64::from(difference);
                    count += 1;
                }
            }
            let mean = sum as f64 / count.max(1) as f64;
            let (maximum_limit, mean_limit, edge_limit) = if case == "appearance" {
                (88, 0.5, 4)
            } else {
                (64, 2.0, 0)
            };
            assert!(
                maximum <= maximum_limit && mean <= mean_limit && edges <= edge_limit,
                "{case}/{name}: maximum {maximum}, mean {mean}, edges {edges}"
            );
        }
    }
}

#[test]
fn native_chart_support_boundaries_and_numeric_failures_are_explicit() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for source in [
        "#chart(kind='pie',data={values={1,2}})",
        "#table[#row[#cell[#chart(data={values={1}})]]]",
        "#chart(data={values={'Inf'}})",
        "#chart(kind='stacked_column',data={series={{values={'1e308'}},{values={'1e308'}}}})",
    ] {
        let document = mote::compile(source).unwrap();
        assert!(
            render::plan(&document, &resolver, None).is_err(),
            "{source}"
        );
    }
    let document =
        mote::compile("#chart(data={values={1}},axis={value={maximum=10}},style='Retained')")
            .unwrap();
    let plan = render::plan(&document, &resolver, None).unwrap();
    assert!(
        plan.warnings
            .iter()
            .any(|warning| warning.contains("axis.value.maximum"))
    );
    assert!(
        plan.warnings
            .iter()
            .any(|warning| warning.contains("style"))
    );
}

#[test]
fn zero_size_charts_end_fill_continuity_without_consuming_flow_space() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let first = "#paragraph(fill='#eee')[First]";
    let second = "#paragraph(fill='#eee')[Second]";
    let without_chart = mote::compile(&format!("{first}\n{second}")).unwrap();
    let with_chart = mote::compile(&format!(
        "{first}\n#chart(width=0,data={{values={{1}}}})\n{second}"
    ))
    .unwrap();
    let plain = render::plan(&without_chart, &resolver, None)
        .unwrap()
        .output;
    let divided = render::plan(&with_chart, &resolver, None).unwrap().output;
    assert_eq!(plain.pages[0].lines, divided.pages[0].lines);
    let rectangles = |plan: &mote::layout::DocumentTextPlan| {
        plan.pages[0]
            .operations
            .iter()
            .filter(|operation| matches!(operation, mote::layout::PageOperation::Rectangle(_)))
            .count()
    };
    assert_eq!(rectangles(&plain), 3);
    assert_eq!(rectangles(&divided), 2);
}
