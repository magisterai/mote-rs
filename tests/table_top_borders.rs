//! The top border of a table, which Word sets above the first row of a nested
//! table and above the row that begins a page, as the nested-table-cell-end
//! probe shows.

use mote::{render, source::FileAssetResolver};

/// Returns the tops of the lines of each page of a Letter document with
/// one-inch margins, and requires that the plan carry no warning.
fn line_tops(source: &str) -> Vec<Vec<f64>> {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    planned
        .output
        .pages
        .iter()
        .map(|page| page.lines.iter().map(|line| line.top).collect())
        .collect()
}

fn nested(border: &str) -> String {
    format!(
        "#table[\n#row[#cell[\n#paragraph[Before]\n#table(borders = {{ all = {border} }})[#row[#cell[Inner]]]\n]]\n#row[#cell[Next]]\n]\n"
    )
}

fn assert_near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 0.001, "{actual} {expected}");
}

#[test]
fn a_nested_table_lies_its_top_border_below_the_content_above_it() {
    let plain = line_tops(&nested("{ style = 'none' }"));
    for width in [1.0, 3.0, 6.0] {
        let bordered = line_tops(&nested(&format!(
            "{{ style = 'single', width = {width}, color = '#000000' }}"
        )));
        assert_near(bordered[0][0], plain[0][0]);
        assert_near(bordered[0][1] - plain[0][1], width);
        // The row after the nested table lies below its bottom border as well.
        assert_near(bordered[0][2] - plain[0][2], 2.0 * width);
    }
}

#[test]
fn the_row_that_begins_a_page_lies_its_top_border_below_the_top_of_the_text_area() {
    let rows = "#row[#cell[Body.]]\n".repeat(60);
    let first_on_second_page = |border: &str| {
        let pages = line_tops(&format!(
            "#table(borders = {{ all = {border} }})[\n{rows}]\n"
        ));
        assert!(pages.len() > 1);
        pages[1][0]
    };
    let plain = first_on_second_page("{ style = 'none' }");
    for width in [0.5, 3.0] {
        let bordered = first_on_second_page(&format!(
            "{{ style = 'single', width = {width}, color = '#000000' }}"
        ));
        assert_near(bordered - plain, width);
    }
}
