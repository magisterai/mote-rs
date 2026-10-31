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
    compare_captures(
        "chart-native-bars",
        &["kinds", "appearance", "labels", "flow"],
    );
}

#[test]
fn native_line_and_area_charts_preserve_paths_markers_and_labels() {
    compare_captures("chart-native-curves", &["kinds", "legends", "data"]);
}

#[test]
fn native_pie_and_doughnut_charts_preserve_sectors_labels_and_backgrounds() {
    compare_captures(
        "chart-native-pie",
        &["defaults", "legends", "labels", "data"],
    );
}

fn compare_captures(collection: &str, cases: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(collection);
    let resolver = FileAssetResolver::new(&root).unwrap();
    for &case in cases {
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
            let (maximum_limit, mean_limit, edge_limit) = match (collection, case) {
                ("chart-native-bars", "appearance") => (88, 0.5, 4),
                ("chart-native-curves", "data") => (92, 2.5, 1),
                ("chart-native-curves", "legends") => (88, 0.6, 4),
                ("chart-native-pie", "data") if page.number == 1 => (48, 8.5, 0),
                ("chart-native-pie", "data") => (92, 0.3, 1),
                ("chart-native-pie", "legends") => (56, 0.2, 0),
                ("chart-native-pie", _) => (48, 1.0, 0),
                _ => (64, 2.0, 0),
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
        "#table[#row[#cell[#chart(data={values={1}})]]]",
        "#chart(data={values={'Inf'}})",
        "#chart(kind='pie',data={values={'1e308','1e308'}})",
        "#chart(kind='pie',data={values={'1e308'}})",
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

#[test]
fn single_category_line_and_area_limits_are_explicit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/chart-native-curves/unsupported");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in ["single-line", "single-area"] {
        let source = fs::read_to_string(root.join(format!("{case}.mote"))).unwrap();
        let document = mote::compile(&source).unwrap();
        let error = render::plan(&document, &resolver, None).err().unwrap();
        assert!(
            error.to_string().contains("at least two categories"),
            "{error}"
        );
        assert!(mote::docx::emit(&document).is_ok());
    }
}

#[test]
fn native_curve_chart_warnings_describe_retained_properties() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for (kind, labels, expected) in [
        ("line", "{position='bottom'}", "automatic placement around"),
        ("area", "true", "omitted from native area"),
        ("stacked_area", "true", "omitted from native area"),
        ("percent_stacked_area", "true", "omitted from native area"),
    ] {
        let source = format!(
            "#chart(kind='{kind}',data={{values={{1,2,3}}}},data_labels={labels},gap_width=100,overlap=20)"
        );
        let document = mote::compile(&source).unwrap();
        let plan = render::plan(&document, &resolver, None).unwrap();
        for fragment in [expected, ".gap_width", ".overlap"] {
            assert!(
                plan.warnings
                    .iter()
                    .any(|warning| warning.contains(fragment)),
                "{kind}: {fragment}: {:?}",
                plan.warnings
            );
        }
    }
}

#[test]
fn native_radial_chart_warnings_describe_automatic_placement() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for kind in ["pie", "doughnut"] {
        let source = format!(
            "#chart(kind='{kind}',data={{values={{1,2,3}}}},data_labels={{position='outside_end'}},axis={{value={{title='Unused'}}}},gridlines=true,gap_width=100,overlap=20)"
        );
        let plan = render::plan(&mote::compile(&source).unwrap(), &resolver, None).unwrap();
        for fragment in [
            "automatic radial placement",
            ".axis",
            ".gridlines",
            ".gap_width",
            ".overlap",
        ] {
            assert!(
                plan.warnings
                    .iter()
                    .any(|warning| warning.contains(fragment)),
                "{kind}: {fragment}: {:?}",
                plan.warnings
            );
        }
    }
}

#[test]
fn bar_labels_paint_after_all_series_marks() {
    let source = "#chart(data={categories={'Category'},series={{values={2}},{values={4}}}},legend=false,data_labels=true,colors={'#123456','#654321'})";
    let resolver = FileAssetResolver::new(".").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    let page = &planned.output.pages[0];
    let last_bar = page.operations.iter().rposition(|operation| matches!(operation, mote::layout::PageOperation::Rectangle(rectangle) if matches!(rectangle.color.as_str(), "123456" | "654321"))).unwrap();
    let first_label = page
        .operations
        .iter()
        .position(|operation| {
            if let mote::layout::PageOperation::Text(index) = operation {
                let text: String = page.lines[*index]
                    .line
                    .fragments
                    .iter()
                    .map(|fragment| fragment.text.as_str())
                    .collect();
                text == "2" || text == "4"
            } else {
                false
            }
        })
        .unwrap();
    assert!(first_label > last_bar);
}

#[test]
fn percentage_charts_normalize_finite_values_with_overflowing_total_magnitude() {
    let source = "#chart(kind='percent_stacked_column',data={series={{values={'-1e308'}},{values={'1e308'}}}},legend=false,data_labels={percent=true})";
    let resolver = FileAssetResolver::new(".").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    let labels: Vec<_> = planned.output.pages[0]
        .lines
        .iter()
        .map(|line| {
            line.line
                .fragments
                .iter()
                .map(|fragment| fragment.text.as_str())
                .collect::<String>()
        })
        .collect();
    assert!(labels.iter().any(|label| label == "-50%"));
    assert!(labels.iter().any(|label| label == "50%"));
    assert!(
        render::paint_pdf(&planned.output)
            .unwrap()
            .starts_with(b"%PDF-")
    );
    assert_eq!(
        render::paint_png(&planned.output, &Default::default())
            .unwrap()
            .output
            .len(),
        1
    );
}

#[test]
fn nearby_line_labels_separate_in_both_layouts_and_series_orders() {
    use mote::layout::PageOperation;
    let resolver = FileAssetResolver::new(".").unwrap();
    for presentation in [false, true] {
        for reversed in [false, true] {
            let series = [
                "{name='Web',values={12,18,16,24,30}}",
                "{name='Mobile',values={8,10,15,18,24}}",
            ];
            let ordered = if reversed {
                [series[1], series[0]]
            } else {
                series
            };
            let chart = format!(
                "#chart(kind='line',width=pt(340),height=pt(320),data={{categories={{'W1','W2','W3','W4','W5'}},series={{{},{}}}}},data_labels=true,legend='bottom')",
                ordered[0], ordered[1]
            );
            let source = if presentation {
                format!(
                    "#presentation(width=pt(360),height=pt(340),theme={{fonts={{body='Helvetica'}}}})\n#slide[#box(position={{x=pt(10),y=pt(10),width=pt(340),height=pt(320)}})[{chart}]]"
                )
            } else {
                chart
            };
            let planned = render::plan(&mote::compile(&source).unwrap(), &resolver, None).unwrap();
            let page = &planned.output.pages[0];
            let markers: Vec<_> = page
                .operations
                .iter()
                .filter_map(|operation| {
                    let PageOperation::Path(path) = operation else {
                        return None;
                    };
                    if path.fill.is_none() || path.stroke.is_some() {
                        return None;
                    }
                    let minimum = path
                        .points
                        .iter()
                        .fold([f64::INFINITY; 2], |result, point| {
                            [result[0].min(point[0]), result[1].min(point[1])]
                        });
                    let maximum = path
                        .points
                        .iter()
                        .fold([f64::NEG_INFINITY; 2], |result, point| {
                            [result[0].max(point[0]), result[1].max(point[1])]
                        });
                    Some([
                        (minimum[0] + maximum[0]) / 2.0,
                        (minimum[1] + maximum[1]) / 2.0,
                    ])
                })
                .collect();
            // Ten data markers precede the two legend keys.
            let upper = markers[if reversed { 7 } else { 2 }];
            let lower = markers[if reversed { 2 } else { 7 }];
            let label = |text: &str| {
                page.lines
                    .iter()
                    .find(|line| {
                        line.line
                            .fragments
                            .iter()
                            .map(|fragment| fragment.text.as_str())
                            .collect::<String>()
                            == text
                            && line
                                .line
                                .fragments
                                .first()
                                .is_some_and(|fragment| (fragment.x - upper[0]).abs() < 20.0)
                    })
                    .unwrap()
            };
            let above = label("16");
            let below = label("15");
            assert!(above.top + above.line.height < upper[1] - 2.5);
            assert!(below.top > lower[1] + 2.5);
            assert!((above.line.fragments[0].x - below.line.fragments[0].x).abs() < 1e-9);
            assert!(
                render::paint_pdf(&planned.output)
                    .unwrap()
                    .starts_with(b"%PDF-")
            );
            assert_eq!(
                render::paint_png(&planned.output, &Default::default())
                    .unwrap()
                    .output
                    .len(),
                1
            );
        }
    }
}
