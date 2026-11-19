//! The rule that parts the notes from the text. Word draws the rule in the line
//! of a paragraph of the default style, about half the size of its text above
//! the foot of the line, as its exports of the notes cases show: the rule lies
//! lower before the notes of a document whose Normal style has exact lines and
//! little space after them than before those of a document without one.
use mote::{
    layout::{PageOperation, RuleRole},
    render,
    source::FileAssetResolver,
};

/// Returns the height of each note separator of the first page and the top of
/// the line that begins with `note`.
fn separators(source: &str, note: &str) -> (Vec<f64>, f64) {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    let page = &planned.output.pages[0];
    let rules = page
        .operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::Rule(rule) if rule.role == RuleRole::NoteSeparator => {
                Some(rule.start[1])
            }
            _ => None,
        })
        .collect();
    let top = page
        .lines
        .iter()
        .find(|placed| {
            placed
                .line
                .fragments
                .iter()
                .map(|fragment| fragment.text.as_str())
                .collect::<String>()
                .contains(note)
        })
        .unwrap()
        .top;
    (rules, top)
}

const STYLED: &str = "#style(name = 'Normal', type = 'paragraph', run = { size = pt(10) }, paragraph = { spacing = { before = pt(0), after = pt(2), line = { rule = 'exact', value = pt(15) } } })\n";

#[test]
fn the_footnote_separator_lies_in_a_line_of_the_default_style() {
    let body = "Text#footnote[The note at the foot.] of the body.\n";
    let (rules, note) = separators(body, "The note at the foot");
    // The default style sets 8 points after an 11 point line whose multiple
    // adds 1.98 points beneath it.
    assert!(
        (note - rules[0] - (8.0 + 1.98 + 0.48 * 11.0)).abs() < 0.01,
        "{rules:?} {note}"
    );
    let (rules, note) = separators(&format!("{STYLED}{body}"), "The note at the foot");
    assert!(
        (note - rules[0] - (2.0 + 0.48 * 10.0)).abs() < 0.01,
        "{rules:?} {note}"
    );
}

#[test]
fn the_endnote_separator_lies_in_its_line_and_not_at_the_top_of_it() {
    let body = "Text#endnote[The note at the end.] of the body.\n";
    let (rules, note) = separators(&format!("{STYLED}{body}"), "The note at the end");
    // The separator paragraph is a line of 15 points with 2 points after it.
    assert!(
        (note - rules[0] - (2.0 + 0.48 * 10.0)).abs() < 0.01,
        "{rules:?} {note}"
    );
    assert!(note - rules[0] < 15.0);
}
