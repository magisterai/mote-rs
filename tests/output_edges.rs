use mote::{
    evaluation::EvaluationOptions, layout::PageOperation, render, source::FileAssetResolver,
};
use std::io::{Cursor, Read};

fn zip_part(bytes: &[u8], name: &str) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut content = Vec::new();
    archive
        .by_name(name)
        .unwrap()
        .read_to_end(&mut content)
        .unwrap();
    content
}

#[test]
fn chart_workbooks_retain_unequal_arrays_without_inventing_numeric_cells() {
    for (categories, values, expected_cells, absent_cells) in [
        (
            "{'A'}",
            "{1,2,3}",
            vec!["A2", "B2", "B3", "B4"],
            vec!["A3", "A4"],
        ),
        (
            "{'A','B','C'}",
            "{1}",
            vec!["A2", "A3", "A4", "B2"],
            vec!["B3", "B4"],
        ),
    ] {
        let document = mote::compile(&format!(
            "#chart(data={{categories={categories},values={values}}})"
        ))
        .unwrap();
        let package = mote::docx::emit(&document).unwrap();
        let workbook = zip_part(&package, "word/embeddings/chart1.xlsx");
        let sheet = String::from_utf8(zip_part(&workbook, "xl/worksheets/sheet1.xml")).unwrap();
        let xml = roxmltree::Document::parse(&sheet).unwrap();
        let cells: Vec<_> = xml
            .descendants()
            .filter(|node| {
                node.has_tag_name((
                    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                    "c",
                ))
            })
            .filter_map(|node| node.attribute("r"))
            .collect();
        for reference in expected_cells {
            assert!(cells.contains(&reference), "missing {reference}: {sheet}");
        }
        for reference in absent_cells {
            assert!(!cells.contains(&reference), "invented {reference}: {sheet}");
        }
    }
    let document = mote::compile(
        "#chart(data={categories={'Repeated','Repeated'},name='Repeated',values={1,2}})",
    )
    .unwrap();
    let workbook = zip_part(
        &mote::docx::emit(&document).unwrap(),
        "word/embeddings/chart1.xlsx",
    );
    let strings = String::from_utf8(zip_part(&workbook, "xl/sharedStrings.xml")).unwrap();
    let xml = roxmltree::Document::parse(&strings).unwrap();
    assert_eq!(xml.root_element().attribute("count"), Some("3"));
    assert_eq!(xml.root_element().attribute("uniqueCount"), Some("1"));
}

#[test]
fn nonfinite_chart_values_fail_before_any_output_package_is_returned() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for value in ["NaN", "Inf", "-Inf", "1e400"] {
        for wrapper in ["{}", "#header[{}]\n\nBody", "#footer[{}]\n\nBody"] {
            let chart = format!("#chart(data={{values={{'{value}'}}}})");
            let source = wrapper.replace("{}", &chart);
            let document = mote::compile(&source).unwrap();
            assert!(
                mote::docx::emit(&document)
                    .unwrap_err()
                    .to_string()
                    .contains("finite")
            );
            assert!(
                mote::pptx::emit(&document)
                    .unwrap_err()
                    .to_string()
                    .contains("finite")
            );
            assert!(
                render::plan(&document, &resolver, None)
                    .err()
                    .unwrap()
                    .to_string()
                    .contains("finite")
            );
        }
    }
}

#[test]
fn empty_presentations_keep_declared_geometry_in_both_native_formats() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let document =
        mote::compile("#presentation(width=pt(320),height=pt(200),background='#123456')").unwrap();
    let plan = render::plan(&document, &resolver, None).unwrap().output;
    assert_eq!(plan.pages.len(), 1);
    assert_eq!((plan.geometry.width, plan.geometry.height), (320.0, 200.0));
    let pages = render::paint_png(&plan, &Default::default())
        .unwrap()
        .output;
    assert_eq!((pages[0].width, pages[0].height), (320, 200));
    assert!(!render::paint_pdf(&plan).unwrap().is_empty());
}

#[test]
fn zero_hanging_list_markers_fit_before_the_text_and_checkboxes_begin_the_line() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for directive in ["list", "ordered_list"] {
        for item in ["#item[Content]", "#item(checked=true)[Content]"] {
            let source = format!("#{directive}(indent={{left=pt(36),hanging=pt(0)}})[{item}]");
            let document = mote::compile(&source).unwrap();
            let plan = render::plan(&document, &resolver, None).unwrap().output;
            let marker = plan.pages[0]
                .operations
                .iter()
                .find_map(|operation| match operation {
                    PageOperation::ListMarker(marker) => Some(marker),
                    _ => None,
                })
                .unwrap();
            assert!(marker.slot[2] > 0.0);
            let text = plan.pages[0]
                .lines
                .iter()
                .find(|line| {
                    line.line
                        .fragments
                        .iter()
                        .any(|fragment| fragment.text == "Content")
                })
                .unwrap();
            if item.contains("checked") {
                let content = text
                    .line
                    .fragments
                    .iter()
                    .find(|fragment| fragment.text == "Content")
                    .unwrap();
                assert!((marker.slot[0] - text.line.x).abs() < 1e-8);
                assert!(content.x >= marker.slot[0] + marker.slot[2]);
            } else {
                assert!((marker.slot[0] + marker.slot[2] - text.line.x).abs() < 1e-8);
            }
            assert!(!render::paint_pdf(&plan).unwrap().is_empty());
            assert_eq!(
                render::paint_png(&plan, &Default::default())
                    .unwrap()
                    .output
                    .len(),
                1
            );
        }
    }
}

#[test]
fn underline_patterns_preserve_text_metrics_and_suppress_none() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let mut baseline = None;
    for pattern in ["single", "none", "wave"] {
        let document = mote::compile(&format!("#underline(style='{pattern}')[Underline]")).unwrap();
        let plan = render::plan(&document, &resolver, None).unwrap();
        assert!(
            !plan
                .warnings
                .iter()
                .any(|warning| warning.contains("solid underline"))
        );
        let line = &plan.output.pages[0].lines[0].line;
        let geometry = (line.x, line.baseline, line.width, line.height);
        assert_eq!(*baseline.get_or_insert(geometry), geometry);
        assert_eq!(line.decorations.len(), usize::from(pattern != "none"));
        if pattern == "wave" {
            let decoration = &line.decorations[0];
            let points = decoration.wave_points();
            assert!(points.iter().any(|point| point[1] < decoration.y));
            assert!(points.iter().any(|point| point[1] > decoration.y));
        }
        assert!(!render::paint_pdf(&plan.output).unwrap().is_empty());
        render::paint_png(&plan.output, &Default::default()).unwrap();
    }
}

#[test]
fn tagged_empty_lua_content_stays_empty_while_plain_maps_remain_maps() {
    let document = mote::compile_with_options("#function identity(body) return body end\n#paragraph[Before#identity[]after]\n#function empty() return document.paragraph(document.tab(),document.span('')) end\n#empty()", EvaluationOptions { enable_lua:true,..Default::default() }).unwrap();
    assert!(
        !document
            .content
            .iter()
            .map(|node| node.to_text())
            .collect::<String>()
            .contains("{}")
    );
    let plain_map = mote::compile_with_options(
        "#function empty() return {} end\n#empty()",
        EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        plain_map
            .content
            .iter()
            .map(|node| node.to_text())
            .collect::<String>(),
        "{}"
    );
}

#[test]
fn connector_markers_use_distinct_shapes_and_independent_dimensions() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for kind in ["triangle", "stealth", "oval", "diamond"] {
        for (start, end) in [
            ("x=pt(20),y=pt(30)", "x=pt(100),y=pt(30)"),
            ("x=pt(30),y=pt(20)", "x=pt(30),y=pt(100)"),
        ] {
            let source = format!(
                "#presentation()#slide[#line(from={{{start}}},to={{{end}}},border={{width=pt(2)}},cap='flat',arrow={{end='{kind}',end_size='small',end_length='large'}})]"
            );
            let plan = render::plan_with_layout(
                &mote::compile(&source).unwrap(),
                &resolver,
                None,
                render::LayoutMode::Presentation,
            )
            .unwrap()
            .output;
            let marker = plan.pages[0]
                .operations
                .iter()
                .find_map(|operation| match operation {
                    PageOperation::Path(path) if path.fill.is_some() => Some(path),
                    _ => None,
                })
                .unwrap();
            assert!(marker.closed);
            if kind == "diamond" {
                assert_eq!(marker.points.len(), 4);
            }
            let axis = usize::from(start.contains("x=pt(30)"));
            let minimum = marker
                .points
                .iter()
                .map(|point| point[axis])
                .fold(f64::INFINITY, f64::min);
            let maximum = marker
                .points
                .iter()
                .map(|point| point[axis])
                .fold(f64::NEG_INFINITY, f64::max);
            assert!(
                (maximum - minimum - 10.0).abs() < 1e-8,
                "{kind}: {minimum}..{maximum}"
            );
        }
    }
}

#[path = "support/language_capture.rs"]
mod language_capture;

#[test]
fn combined_output_edges_match_captured_models_packages_geometry_and_pixels() {
    language_capture::compare("output-edges", &["document", "slides", "empty"]);
}
