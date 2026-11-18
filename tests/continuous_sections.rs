//! The space about a continuous section and the height of its balanced
//! columns, as Word's export of the review case
//! [continuous sections](cases/word-73-continuous-sections.mote) shows.
use mote::{render, source::FileAssetResolver};

/// Returns the text, the left edge, the top and the height of each line of the
/// only page of a document.
fn lines(source: &str) -> Vec<(String, f64, f64, f64)> {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    assert_eq!(planned.output.pages.len(), 1);
    planned.output.pages[0]
        .lines
        .iter()
        .map(|placed| {
            let text = placed
                .line
                .fragments
                .iter()
                .map(|fragment| fragment.text.as_str())
                .collect();
            (text, placed.line.x, placed.top, placed.line.height)
        })
        .collect()
}

fn line<'lines>(
    lines: &'lines [(String, f64, f64, f64)],
    text: &str,
) -> &'lines (String, f64, f64, f64) {
    lines
        .iter()
        .find(|line| line.0 == text)
        .unwrap_or_else(|| panic!("no line {text:?}"))
}

fn assert_near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 0.001, "{actual} {expected}");
}

const COLUMNS: &str = "#section(columns = { count = 2 }, properties = { start = 'continuous' })";
const ONE_COLUMN: &str = "#section(columns = { count = 1 }, properties = { start = 'continuous' })";

#[test]
fn the_space_before_the_first_block_of_a_continuous_section_collapses_with_the_space_above_it() {
    let lines = lines(&format!(
        "#paragraph(spacing = {{ after = pt(8) }})[Above]\n\n{COLUMNS}[\n#paragraph(spacing = {{ before = pt(30) }})[First]\n\n#break(type = 'column')\n\nSecond\n]\n"
    ));
    let above = line(&lines, "Above");
    let foot = above.2 + above.3;
    // The first paragraph lies 30 points below the paragraph above the
    // section, and not 38; the second column begins below the 8 points.
    assert_near(line(&lines, "First").2, foot + 30.0);
    assert_near(line(&lines, "Second").2, foot + 8.0);
}

#[test]
fn the_section_after_balanced_columns_begins_below_the_lowest_column_with_its_trailing_space() {
    let source = |after: &str| {
        format!(
            "{COLUMNS}[\n#paragraph(spacing = {{ after = pt(0) }})[One#line_break()Two#line_break()Three]\n\n#paragraph(spacing = {{ after = pt(20) }})[Four]\n\n#paragraph(spacing = {{ after = pt(8) }})[Five#line_break()Six]\n]\n\n{ONE_COLUMN}[\n#paragraph(spacing = {{ before = pt({after}) }})[After]\n]\n"
        )
    };
    let placed = lines(&source("0"));
    let (four, five) = (line(&placed, "Four"), line(&placed, "Five"));
    // The first column ends with the paragraph that has 20 points after it,
    // and the second begins with the last paragraph of the section.
    assert!(five.1 > four.1);
    let lowest = four.2 + four.3 + 20.0;
    assert_near(line(&placed, "After").2, lowest);
    // The space before the next paragraph collapses with the 8 points after
    // the last paragraph of the columns.
    assert_near(line(&lines(&source("30")), "After").2, lowest + 22.0);
}

#[test]
fn balanced_columns_grow_and_do_not_divide_a_paragraph_against_widow_control() {
    let lines = lines(&format!(
        "{COLUMNS}[\nOne#line_break()Two#line_break()Three\n\nFour#line_break()Five\n]\n\n{ONE_COLUMN}[\nAfter\n]\n"
    ));
    let column = |text: &str| line(&lines, text).1;
    // Two columns of two and a half lines would part the third line from its
    // paragraph.
    assert_eq!(column("Three"), column("One"));
    assert!(column("Four") > column("One"));
}
