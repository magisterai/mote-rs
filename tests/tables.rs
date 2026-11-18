#[allow(dead_code)]
#[path = "docx/support.rs"]
mod xml_support;
use mote::{
    model::{Content, ElementKind},
    source::FileAssetResolver,
};
use std::{
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tables")
}
fn normalize_numbers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        serde_json::Value::Object(fields) => fields.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}
#[test]
fn tables_preserve_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::unconfined(fixtures()).unwrap();
    for case in [
        "basic",
        "attributes",
        "merges",
        "markdown",
        "content",
        "styles",
        "borders",
        "cell-blocks",
        "cell-list-order",
        "columns",
        "column-flow",
        "objects",
        "oversized-merge",
        "pagination",
        "rowspan-flow",
        "widths",
    ] {
        let source = fs::read_to_string(fixtures().join(format!("{case}.mote"))).unwrap();
        let document = mote::compile(&source).unwrap();
        let original = document.clone();
        let capture = fixtures().join("captured").join(case);
        let mut actual: serde_json::Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}: model");
        let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
        assert!(
            if case == "pagination" {
                generated.warnings.len() == 1 && generated.warnings[0].contains("paragraph.size")
            } else {
                generated.warnings.is_empty()
            },
            "{case}: {:?}",
            generated.warnings
        );
        let mut archive = zip::ZipArchive::new(Cursor::new(generated.bytes)).unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("capture.json")).unwrap()).unwrap();
        let names = manifest["parts"].as_array().unwrap();
        assert_eq!(archive.len(), names.len());
        for name in names {
            let name = name.as_str().unwrap();
            let mut actual = Vec::new();
            archive
                .by_name(name)
                .unwrap()
                .read_to_end(&mut actual)
                .unwrap();
            let expected = fs::read(capture.join("package").join(name)).unwrap();
            if name.ends_with(".xml") || name.ends_with(".rels") {
                xml_support::compare_xml(
                    std::str::from_utf8(&actual).unwrap(),
                    std::str::from_utf8(&expected).unwrap(),
                    &format!("{case}/{name}"),
                );
            } else {
                assert_eq!(actual, expected, "{case}/{name}");
            }
        }
        assert_eq!(document, original);
    }
}
#[test]
fn table_structure_and_attribute_errors_are_local() {
    for source in [
        "#table",
        "#table()",
        "#table(3)[]",
        "#table[text]",
        "#table[#cell[text]]",
        "#row[#row[]]",
        "#row[text]",
        "#cell()",
        "#table[][]",
        "#cell(colspan=0)[]",
        "#cell(rowspan=1.5)[]",
        "#cell(width=-1)[]",
        "#cell(vertical_alignment=\"left\")[]",
        "#cell(text_direction=\"up\")[]",
        "#row(repeat_header=4)[]",
        "#row(height_rule=\"minimum\")[]",
        "#table(layout=\"fluid\")[]",
        "#table(cell_padding=pt(2))[]",
        "#cell(padding={all=pt(2)})[]",
        "#paragraph[#table[]]",
        "#paragraph[#row[]]",
        "#paragraph[#cell[]]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let document = mote::compile("#table[#row[]]").unwrap();
    assert_eq!(document.warnings.len(), 1);
    let Content::Element(table) = &document.content[0] else {
        panic!()
    };
    let Content::Element(row) = &table.children[0] else {
        panic!()
    };
    assert_eq!(row.kind, ElementKind::Row);
    assert!(row.kind.is_table_part());
    assert!(!row.kind.is_block());
    let document = mote::compile("#table[]").unwrap();
    assert_eq!(document.warnings.len(), 1);
    assert!(mote::docx::emit(&document).is_ok());
}
#[test]
fn markdown_table_recovery_and_source_ranges_remain_precise() {
    for source in [
        "a | b\n-- | ---\n",
        "a | b | c\n--- | ---\n",
        "plain | text",
    ] {
        let document = mote::compile(source).unwrap();
        assert!(!document.content.iter().any(|child|matches!(child,Content::Element(element) if element.kind==ElementKind::Table)),"{source}");
    }
    let source = "| First | Second |\r\n| --- | --- |\r\n| good | #span(size = false)[bad] |";
    let diagnostic = mote::compile(source).unwrap_err();
    assert_eq!(&source[diagnostic.span.start..diagnostic.span.end], "false");
}
#[test]
fn excessive_table_grids_fail_before_package_allocation() {
    let document = mote::compile("#table[#row[#cell(colspan=20000)[Wide]]]").unwrap();
    assert!(
        mote::docx::emit(&document)
            .unwrap_err()
            .to_string()
            .contains("grid limit")
    );
}
#[test]
fn a_cell_that_ends_with_a_nested_table_is_closed_by_a_collapsed_paragraph() {
    let document = mote::compile(
        "#table[\n  #row[\n    #cell[\n      #paragraph[Before]\n      #table[\n        #row[\n          #cell[\n            #table[\n              #row[\n                #cell[Innermost]\n              ]\n            ]\n          ]\n        ]\n      ]\n    ]\n    #cell[Plain]\n  ]\n]\n",
    )
    .unwrap();
    let resolver = FileAssetResolver::unconfined(fixtures()).unwrap();
    let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    assert!(generated.warnings.is_empty(), "{:?}", generated.warnings);
    let parts = xml_support::unpack(generated.bytes);
    let package = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let cells: Vec<_> = package
        .descendants()
        .filter(|node| node.has_tag_name((namespace, "tc")))
        .collect();
    assert_eq!(cells.len(), 4);
    let mut closed_after_table = 0;
    for cell in cells {
        let last = cell.last_element_child().unwrap();
        assert!(last.has_tag_name((namespace, "p")));
        let before = last.prev_sibling_element().unwrap();
        if before.has_tag_name((namespace, "tbl")) {
            closed_after_table += 1;
            let spacing = last
                .descendants()
                .find(|node| node.has_tag_name((namespace, "spacing")))
                .unwrap();
            assert_eq!(spacing.attribute((namespace, "line")), Some("20"));
            assert_eq!(spacing.attribute((namespace, "lineRule")), Some("exact"));
        }
    }
    assert_eq!(closed_after_table, 2);
}
#[test]
fn a_shape_in_a_table_cell_is_drawn_at_its_own_place_on_the_page() {
    let shape = "#shape(position = { x = inch(3), y = inch(4), width = inch(1), height = inch(0.5) }, fill = \"#2563eb\")[Shape]";
    let paths = |source: &str| {
        let resolver = FileAssetResolver::unconfined(fixtures()).unwrap();
        let document = mote::compile(source).unwrap();
        let planned = mote::render::plan(&document, &resolver, None).unwrap();
        assert_eq!(planned.warnings, Vec::<String>::new());
        planned.output.pages[0]
            .operations
            .iter()
            .filter(|operation| matches!(operation, mote::layout::PageOperation::Path(_)))
            .map(|operation| format!("{operation:?}"))
            .collect::<Vec<_>>()
    };
    let in_body = paths(&format!("{shape}\n"));
    assert!(!in_body.is_empty());
    let in_cell = paths(&format!(
        "#table[\n#row[\n#cell[\n{shape}\n]\n#cell[Beside]\n]\n]\n"
    ));
    assert_eq!(in_cell, in_body);
    let in_nested_cell = paths(&format!(
        "#table[\n#row[\n#cell[\n#table[\n#row[\n#cell[\n{shape}\n]\n]\n]\n]\n]\n]\n"
    ));
    assert_eq!(in_nested_cell, in_body);
}
