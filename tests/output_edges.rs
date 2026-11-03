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
fn zero_hanging_list_markers_fit_before_the_unchanged_text_origin() {
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
            assert!((marker.slot[0] + marker.slot[2] - text.line.x).abs() < 1e-8);
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
