//! The order in which a cell takes its formatting from a table style, the
//! regions of the style, the table and the cell itself. Word ranks a region
//! above the formatting that the style gives every cell, except for the
//! vertical alignment, and ranks the margin of a region above the margin of the
//! table, as its exports of the review case of table style precedence show.

#[allow(dead_code)]
#[path = "docx/support.rs"]
mod xml_support;
use mote::{layout::PageOperation, render, source::FileAssetResolver};

struct Painted {
    /// Holds the text, the left edge and the top of each line.
    lines: Vec<(String, f64, f64)>,
    /// Holds the colour of the first fragment of each line.
    colors: Vec<String>,
    /// Holds the colour and the top of each filled rectangle.
    fills: Vec<(String, f64)>,
}

fn painted(source: &str) -> Painted {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    let page = &planned.output.pages[0];
    Painted {
        lines: page
            .lines
            .iter()
            .map(|placed| {
                let text = placed
                    .line
                    .fragments
                    .iter()
                    .map(|fragment| fragment.text.as_str())
                    .collect();
                (text, placed.line.x, placed.top)
            })
            .collect(),
        colors: page
            .lines
            .iter()
            .map(|placed| placed.line.fragments[0].style.color.to_uppercase())
            .collect(),
        fills: page
            .operations
            .iter()
            .filter_map(|operation| match operation {
                PageOperation::Rectangle(rectangle) => {
                    Some((rectangle.color.to_uppercase(), rectangle.top))
                }
                _ => None,
            })
            .collect(),
    }
}

fn assert_near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 0.001, "{actual} {expected}");
}

fn table(attributes: &str, first_cell: &str) -> String {
    format!(
        "#table({attributes})[\n#row[#cell{first_cell}[Head]]\n#row[#cell[Body]]\n#row[#cell[Last]]\n]\n"
    )
}

#[test]
fn a_style_that_states_no_margin_leaves_a_cell_the_margins_of_a_plain_table() {
    let style = "#style(name = 'Bare', type = 'table', table = { alignment = 'left' })\n";
    let plain = painted(&table("", ""));
    let styled = painted(&format!("{style}{}", table("style = 'Bare'", "")));
    assert_eq!(styled.lines, plain.lines);

    let package = xml_support::package(&format!("{style}{}", table("style = 'Bare'", "")));
    assert!(package["word/document.xml"].contains(
        "<w:tblCellMar><w:top w:w=\"80\" w:type=\"dxa\"/><w:left w:w=\"120\" w:type=\"dxa\"/>\
         <w:bottom w:w=\"80\" w:type=\"dxa\"/><w:right w:w=\"120\" w:type=\"dxa\"/></w:tblCellMar>"
    ));
}

#[test]
fn a_margin_is_taken_side_by_side_from_the_cell_the_region_the_table_and_the_style() {
    let style = "#style(name = 'Margins', type = 'table', table = {\n\
        cell = { margin = { top = pt(10) } },\n\
        first_row = { cell = { margin = { left = pt(20) } } }\n})\n";
    let plain = painted(&table("", ""));
    let styled = painted(&format!(
        "{style}{}",
        table(
            "style = 'Margins', cell = { margin = { left = pt(8), bottom = pt(1) } }",
            ""
        )
    ));
    // The plain table has margins of 4 points above and below and of 6 points
    // at the sides.
    let (left, top) = (plain.lines[0].1 - 6.0, plain.lines[0].2 - 4.0);
    assert_near(styled.lines[0].1, left + 20.0);
    assert_near(styled.lines[0].2, top + 10.0);
    assert_near(styled.lines[1].1, left + 8.0);
    let line = plain.lines[1].2 - plain.lines[0].2 - 8.0;
    assert_near(styled.lines[1].2 - styled.lines[0].2, line + 1.0 + 10.0);

    let own = painted(&format!(
        "{style}{}",
        table("style = 'Margins'", "(margin = { right = pt(30) })")
    ));
    assert_near(own.lines[0].1, left + 20.0);
    assert_near(own.lines[0].2, top + 10.0);

    // The margins of the table are written whole, since Word gives a cell no
    // margin on a side that neither the table nor its style states.
    let package = xml_support::package(&format!("{style}{}", table("style = 'Margins'", "")));
    assert!(package["word/document.xml"].contains(
        "<w:tblCellMar><w:top w:w=\"200\" w:type=\"dxa\"/><w:left w:w=\"120\" w:type=\"dxa\"/>\
         <w:bottom w:w=\"80\" w:type=\"dxa\"/><w:right w:w=\"120\" w:type=\"dxa\"/></w:tblCellMar>"
    ));
}

const FILLED: &str = "#style(name = 'Filled', type = 'table', table = {\n\
    cell = { fill = '#FFE4E1' },\n\
    text = { color = '#FF0000' },\n\
    first_row = { fill = '#000080', text = { color = '#FFFFFF' } },\n\
    last_row = { cell = { fill = '#90EE90' } },\n\
    banding = { rows = true, odd = { fill = '#DDDDDD' }, even = { fill = '#FFFF00' } }\n})\n";

#[test]
fn a_region_outranks_the_formatting_that_the_style_gives_every_cell() {
    let source = format!(
        "{FILLED}#table(style = 'Filled')[\n#row[#cell[Head]]\n#row[#cell[Odd]]\n#row[#cell[Even]]\n#row[#cell[Last]]\n]\n"
    );
    let page = painted(&source);
    let mut fills = page.fills.clone();
    fills.sort_by(|first, second| first.1.total_cmp(&second.1));
    let fills: Vec<&str> = fills.iter().map(|(color, _)| color.as_str()).collect();
    // The last row outranks the band on which it lies.
    assert_eq!(fills, ["000080", "DDDDDD", "FFFF00", "90EE90"]);
    assert_eq!(page.colors, ["FFFFFF", "FF0000", "FF0000", "FF0000"]);

    let package = xml_support::package(&source);
    let document = &package["word/document.xml"];
    let head = document.find("Head").unwrap();
    assert!(document[..head].contains("<w:color w:val=\"FFFFFF\"/>"));
    assert!(!document[..head].contains("<w:color w:val=\"FF0000\"/>"));
}

#[test]
fn the_cell_attribute_of_a_table_outranks_a_region_and_a_cell_outranks_both() {
    let page = painted(&format!(
        "{FILLED}#table(style = 'Filled', cell = {{ fill = '#FFA500' }})[\n#row[#cell[Head]]\n#row[#cell(fill = '#00FFFF')[Body]]\n#row[#cell[Last]]\n]\n"
    ));
    let mut fills = page.fills.clone();
    fills.sort_by(|first, second| first.1.total_cmp(&second.1));
    let fills: Vec<&str> = fills.iter().map(|(color, _)| color.as_str()).collect();
    assert_eq!(fills, ["FFA500", "00FFFF", "FFA500"]);
}

#[test]
fn the_vertical_alignment_of_a_style_outranks_that_of_a_region() {
    let rows = "#row(height = pt(40), height_rule = 'exact')[#cell[Head]]\n\
        #row(height = pt(40), height_rule = 'exact')[#cell[Last]]\n";
    let tops = |table: &str| {
        let page = painted(&format!(
            "#style(name = 'Aligned', type = 'table', table = {{ {table} }})\n#table(style = 'Aligned')[\n{rows}]\n"
        ));
        (page.lines[0].2, page.lines[1].2 - 40.0)
    };
    let top = tops("cell = { vertical_alignment = 'top' }");
    let bottom = tops("cell = { vertical_alignment = 'bottom' }");
    assert!(bottom.0 > top.0 + 10.0);
    assert_eq!(
        tops(
            "cell = { vertical_alignment = 'bottom' }, first_row = { vertical_alignment = 'top' }"
        ),
        bottom
    );
    // A region aligns its cells where the style gives every cell no alignment.
    let region = tops("first_row = { vertical_alignment = 'bottom' }");
    assert_near(region.0, bottom.0);
    assert_near(region.1, top.1);
}

#[test]
fn a_table_takes_its_width_and_its_alignment_from_its_style_unless_it_states_them() {
    let style = "#style(name = 'Wide', type = 'table', table = { width = pt(300), alignment = 'center' })\n";
    let cells = "[\n#row[#cell[First]#cell[Second]]\n]\n";
    let lefts = |table: &str| -> Vec<f64> {
        painted(&format!("{style}#table({table}){cells}"))
            .lines
            .iter()
            .map(|line| line.1)
            .collect()
    };
    // The table of 300 points lies at the centre of a text area of 468 points
    // that begins 72 points from the edge of the page, and its text is not
    // centred in its cells.
    let stated = lefts("width = pt(300), alignment = 'center'");
    assert_near(stated[1] - stated[0], 150.0);
    assert!(stated[0] > 72.0 + 84.0 && stated[0] < 72.0 + 84.0 + 10.0);
    assert_eq!(lefts("style = 'Wide'"), stated);
    let own = lefts("style = 'Wide', width = pt(200), alignment = 'left'");
    assert_near(own[1] - own[0], 100.0);
    assert!(own[0] < 72.0 + 10.0);

    // Word takes neither from the style, so DOCX writes both on the table.
    let package = xml_support::package(&format!("{style}#table(style = 'Wide'){cells}"));
    assert!(
        package["word/document.xml"]
            .contains("<w:tblW w:w=\"6000\" w:type=\"dxa\"/><w:jc w:val=\"center\"/>")
    );
}
