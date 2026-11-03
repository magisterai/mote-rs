use mote::evaluation::EvaluationOptions;

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
