//! The borders of a table at the head and the foot of a page. Word sets the top
//! border above the first row of a nested table and above the row that begins a
//! page, and draws the bottom border of the table beneath the last row on a
//! page, as the nested-table-cell-end and table-page-borders probes show. Word
//! keeps the header rows of a table on the page of the row that follows them,
//! as the table-header-rows probe shows.

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

#[test]
fn a_repeated_header_row_lies_its_own_top_border_below_the_top_of_the_text_area() {
    let rows = "#row[#cell[Body.]]\n".repeat(60);
    let header_on_second_page = |border: &str| {
        let pages = line_tops(&format!(
            "#table[\n#row(repeat_header = true)[#cell(borders = {{ top = {border} }})[Header.]]\n{rows}]\n"
        ));
        assert!(pages.len() > 1);
        pages[1][0]
    };
    let plain = header_on_second_page("{ style = 'none' }");
    let bordered = header_on_second_page("{ style = 'single', width = 3, color = '#000000' }");
    assert_near(bordered - plain, 3.0);
}

fn line_lefts(source: &str) -> Vec<f64> {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    planned.output.pages[0]
        .lines
        .iter()
        .map(|line| line.line.x)
        .collect()
}

#[test]
fn a_column_holds_the_outer_halves_of_the_side_borders_of_a_nested_table() {
    let beside = |border: &str| {
        let lefts = line_lefts(&format!(
            "#table[\n#row[#cell[\n#table(borders = {{ all = {border} }})[#row[#cell[Inner]]]\n]#cell[Beside]]\n]\n"
        ));
        lefts[1]
    };
    let plain = beside("{ style = 'none' }");
    for width in [1.0, 3.0, 6.0] {
        let bordered = beside(&format!(
            "{{ style = 'single', width = {width}, color = '#000000' }}"
        ));
        assert_near(bordered - plain, width);
    }
}

#[test]
fn a_row_leaves_room_at_the_foot_of_a_page_for_the_bottom_border_of_the_table() {
    let rows = "#row[#cell[Body.]]\n".repeat(60);
    let lines_on_first_page = |bottom: &str| {
        let pages = line_tops(&format!(
            "#table(borders = {{ top = {{ width = pt(0.5), color = '#000000' }}, inside_h = {{ width = pt(0.5), color = '#000000' }}, bottom = {{ width = pt({bottom}), color = '#000000' }} }})[\n{rows}]\n"
        ));
        assert!(pages.len() > 1);
        pages[0].len()
    };
    // A bottom border no wider than the border of a row asks for no further room.
    assert_eq!(lines_on_first_page("0.5"), lines_on_first_page("0.25"));
    // A bottom border taller than a row cannot follow the last row that fits.
    assert!(lines_on_first_page("30") < lines_on_first_page("0.5"));
}

/// Returns the texts of the lines of each page.
fn line_texts(source: &str) -> Vec<Vec<String>> {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    planned
        .output
        .pages
        .iter()
        .map(|page| {
            page.lines
                .iter()
                .map(|placed| {
                    placed
                        .line
                        .fragments
                        .iter()
                        .map(|fragment| fragment.text.as_str())
                        .collect()
                })
                .collect()
        })
        .collect()
}

/// Returns a table that follows a filler line of exact height `filler` on a
/// Letter page with a text area of 648 points. Each row is 28 points high.
fn table_below_filler(filler: f64, rows: &str) -> String {
    format!(
        "#paragraph(spacing = {{ before = pt(0), after = pt(0), line = {{ rule = 'exact', value = pt({filler}) }} }})[Filler.]\n\n#table[\n{rows}]\n"
    )
}

#[test]
fn the_header_rows_lie_on_the_page_of_the_row_that_follows_them() {
    let row = "height = pt(24), height_rule = 'exact'";
    let body = format!("#row({row})[#cell[Body one]]\n#row({row})[#cell[Body two]]\n");
    let header = |name: &str| format!("#row(repeat_header = true, {row})[#cell[{name}]]\n");
    let whole = ["Head one", "Head two", "Body one", "Body two"];
    // The page holds one row, and then two rows, below the filler: the header
    // rows fit there and the first row of the body does not.
    for filler in [610.0, 590.0] {
        let pages = line_texts(&table_below_filler(
            filler,
            &format!("{}{}{body}", header("Head one"), header("Head two")),
        ));
        assert_eq!(pages[0], ["Filler."]);
        assert_eq!(pages[1], whole);
    }
    // The page holds the header rows and the first row of the body.
    let pages = line_texts(&table_below_filler(
        560.0,
        &format!("{}{}{body}", header("Head one"), header("Head two")),
    ));
    assert_eq!(pages[0], ["Filler.", "Head one", "Head two", "Body one"]);
    assert_eq!(pages[1], ["Head one", "Head two", "Body two"]);
    // A table without header rows begins with the row that fits.
    let pages = line_texts(&table_below_filler(610.0, &body));
    assert_eq!(pages[0], ["Filler.", "Body one"]);
}

#[test]
fn a_header_row_that_begins_a_page_is_not_preceded_by_the_header_rows() {
    let row = "height = pt(400), height_rule = 'exact'";
    let pages = line_texts(&table_below_filler(
        100.0,
        &format!(
            "#row(repeat_header = true, {row})[#cell[Head one]]\n#row(repeat_header = true, {row})[#cell[Head two]]\n#row[#cell[Body]]\n"
        ),
    ));
    let count = |text: &str| pages.iter().flatten().filter(|line| *line == text).count();
    // The header rows exceed a page, so the second header row begins a page,
    // where the first is not repeated above it.
    assert_eq!(pages[0], ["Filler."]);
    assert_eq!(pages[1], ["Head one"]);
    assert_eq!(pages[2][0], "Head two");
    assert_eq!(count("Head one"), 1);
    assert_eq!(count("Head two"), 1);
}

/// Returns the height, the width and the colour of each horizontal border of
/// each page, from the top of the page down.
fn horizontal_borders(source: &str) -> Vec<Vec<(f64, f64, String)>> {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    planned
        .output
        .pages
        .iter()
        .map(|page| {
            let mut borders: Vec<(f64, f64, String)> = page
                .operations
                .iter()
                .filter_map(|operation| match operation {
                    mote::layout::PageOperation::Stroke { rule, .. }
                        if rule.role == mote::layout::RuleRole::TableBorder
                            && rule.start[1] == rule.end[1] =>
                    {
                        Some((rule.start[1], rule.width, rule.color.to_uppercase()))
                    }
                    _ => None,
                })
                .collect();
            borders.sort_by(|first, second| first.0.total_cmp(&second.0));
            borders
        })
        .collect()
}

#[test]
fn a_horizontal_border_lies_wholly_above_the_edge_it_belongs_to() {
    // Word's export of the table-row-height probe shows each border of three
    // points a point and a half above the place of a border centred on its
    // edge, and the side borders centred on theirs.
    let source = "#table(borders = { all = { style = 'single', width = pt(6), color = '#000000' } })[\n#row[#cell[First]]\n#row[#cell[Second]]\n]\n";
    let tops = line_tops(source);
    let borders = &horizontal_borders(source)[0];
    // The table begins at the top of the text area with a band of six points
    // for its top border, and each row ends with the band of the border below.
    assert_near(borders[0].0, 72.0 + 3.0);
    assert_near(tops[0][0], 72.0 + 6.0 + 4.0);
    assert_near(borders[1].0, tops[0][1] - 4.0 - 3.0);
    assert_eq!(borders.len(), 3);
}

#[test]
fn the_bottom_border_of_a_table_is_drawn_beneath_the_last_row_on_each_page() {
    let rows = "#row[#cell[Body.]]\n".repeat(60);
    let pages = horizontal_borders(&format!(
        "#table(borders = {{ outside = {{ style = 'single', width = pt(6), color = '#000000' }}, inside = {{ style = 'single', width = pt(0.5), color = '#94A3B8' }} }})[\n{rows}]\n"
    ));
    assert!(pages.len() > 1);
    for page in &pages {
        let (foot, width, color) = page.last().unwrap();
        assert_eq!((*width, color.as_str()), (6.0, "000000"));
        // The border begins where the border of the row would have begun, half
        // a point below the content of the row.
        let above = page[page.len() - 2].0;
        assert!(
            foot - 3.0 > above && foot + 3.0 <= 720.0 + 0.001,
            "{foot} {above}"
        );
    }
    // The first row of a page keeps its own top border.
    assert_eq!(pages[1][0].1, 0.5);
}
