//! A line lies at the foot of a page when the page holds the line less its
//! trailing space: the space that a line multiple above one adds beneath the
//! descender. Word applies the rule, as the
//! [page-foot probe](office/probes/page-foot.mote) records.
use mote::{
    layout::{DocumentTextPlan, PlacedTextLine},
    render,
    source::FileAssetResolver,
};
use std::path::Path;

/// The height of the text area of a US Letter page with one-inch margins.
const TEXT_AREA: f64 = 648.0;

const LONG_TEXT: &str = "Sample begins a paragraph of several lines, whose words continue far \
    enough that the text must wrap at the right margin and so occupy three lines of the page, \
    which is what the measurement of the foot of a page requires.";

/// Returns a document whose first page holds a filler line of exact height
/// `filler` above a sample paragraph with the attributes `sample`.
fn document(filler: f64, sample: &str, text: &str) -> String {
    format!(
        r#"#document(
  page = {{ size = "letter", orientation = "portrait" }},
  margin = {{ top = pt(72), right = pt(72), bottom = pt(72), left = pt(72) }}
)
#style(name = "Normal", type = "paragraph", paragraph = {{ spacing = {{ before = pt(0), after = pt(0) }} }})

#paragraph(spacing = {{ line = {{ rule = "exact", value = pt({filler}) }} }})[Filler.]

#paragraph({sample})[{text}]

#paragraph(spacing = {{ line = {{ rule = "multiple", value = 1.5 }} }})[Closing.]
"#
    )
}

fn planned(source: &str) -> DocumentTextPlan {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    render::plan_with_fonts(&document, &resolver, None, None)
        .unwrap()
        .output
}

fn line_text(placed: &PlacedTextLine) -> String {
    placed
        .line
        .fragments
        .iter()
        .map(|fragment| fragment.text.as_str())
        .collect()
}

/// Returns the lines of the paragraph that begins with `start`, each with the
/// number of its page: the line that begins so and the lines that follow it up
/// to the line that begins with "Closing".
fn paragraph_lines(plan: &DocumentTextPlan, start: &str) -> Vec<(usize, PlacedTextLine)> {
    plan.pages
        .iter()
        .enumerate()
        .flat_map(|(index, page)| {
            page.lines
                .iter()
                .map(move |placed| (index + 1, placed.clone()))
        })
        .skip_while(|(_, placed)| !line_text(placed).starts_with(start))
        .take_while(|(_, placed)| !line_text(placed).starts_with("Closing"))
        .collect()
}

/// Returns the pages of the lines of the sample paragraph below a filler of
/// height `filler`.
fn sample_pages(filler: f64, sample: &str, text: &str) -> Vec<usize> {
    paragraph_lines(&planned(&document(filler, sample, text)), "Sample")
        .iter()
        .map(|(page, _)| *page)
        .collect()
}

/// Returns the height and the trailing space of the first line of the sample.
fn sample_line(sample: &str) -> (f64, f64) {
    let lines = paragraph_lines(&planned(&document(100.0, sample, "Sample.")), "Sample");
    (lines[0].1.line.height, lines[0].1.line.trailing_space)
}

/// Returns the tallest filler, in the twentieths of a point that Word stores,
/// that leaves `required` points of the text area below it.
fn fitting_filler(required: f64) -> f64 {
    ((TEXT_AREA - required - 0.001) * 20.0).floor() / 20.0
}

/// Returns the shortest filler that leaves less than `required` points of the
/// text area below it by more than the tolerance of the flow.
fn moving_filler(required: f64) -> f64 {
    ((TEXT_AREA - required + 0.002) * 20.0).ceil() / 20.0
}

fn multiple(value: f64) -> String {
    format!("spacing = {{ line = {{ rule = \"multiple\", value = {value} }} }}")
}

#[test]
fn a_line_multiple_above_one_adds_trailing_space_that_the_page_need_not_hold() {
    let (single_height, single_trailing) = sample_line(&multiple(1.0));
    assert_eq!(single_trailing, 0.0);
    for value in [1.15, 1.5, 2.0] {
        let sample = multiple(value);
        let (height, trailing) = sample_line(&sample);
        assert!((height - single_height * value).abs() < 0.001, "{value}");
        assert!((height - trailing - single_height).abs() < 0.001, "{value}");
        assert_eq!(
            sample_pages(fitting_filler(single_height), &sample, "Sample."),
            [1],
            "multiple {value}"
        );
        assert_eq!(
            sample_pages(moving_filler(single_height), &sample, "Sample."),
            [2],
            "multiple {value}"
        );
    }
}

#[test]
fn a_line_without_trailing_space_must_fit_whole() {
    let samples = [
        multiple(1.0),
        multiple(0.8),
        "spacing = { line = { rule = \"exact\", value = pt(24) } }".to_owned(),
        "spacing = { line = { rule = \"at_least\", value = pt(24) } }".to_owned(),
    ];
    for sample in samples {
        let (height, trailing) = sample_line(&sample);
        assert_eq!(trailing, 0.0, "{sample}");
        assert_eq!(
            sample_pages(fitting_filler(height), &sample, "Sample."),
            [1],
            "{sample}"
        );
        assert_eq!(
            sample_pages(moving_filler(height), &sample, "Sample."),
            [2],
            "{sample}"
        );
    }
}

#[test]
fn the_space_before_a_paragraph_must_fit_and_the_space_after_need_not() {
    let (height, trailing) = sample_line(&multiple(1.5));
    let required = height - trailing;
    let after = "spacing = { after = pt(12), line = { rule = \"multiple\", value = 1.5 } }";
    assert_eq!(
        sample_pages(fitting_filler(required), after, "Sample."),
        [1]
    );
    assert_eq!(sample_pages(moving_filler(required), after, "Sample."), [2]);
    let before = "spacing = { before = pt(8), line = { rule = \"multiple\", value = 1.5 } }";
    assert_eq!(
        sample_pages(fitting_filler(required + 8.0), before, "Sample."),
        [1]
    );
    assert_eq!(
        sample_pages(moving_filler(required + 8.0), before, "Sample."),
        [2]
    );
}

#[test]
fn the_last_line_on_a_page_of_a_divided_paragraph_omits_its_trailing_space() {
    let sample = format!("widow_control = false, {}", multiple(1.5));
    let (height, trailing) = sample_line(&sample);
    assert_eq!(sample_pages(100.0, &sample, LONG_TEXT), [1, 1, 1]);
    for (lines_on_first_page, expected) in [(3, [1, 1, 1]), (2, [1, 1, 2]), (1, [1, 2, 2])] {
        let required = lines_on_first_page as f64 * height - trailing;
        assert_eq!(
            sample_pages(fitting_filler(required), &sample, LONG_TEXT),
            expected
        );
        let mut moved = expected;
        moved[lines_on_first_page - 1] = 2;
        assert_eq!(
            sample_pages(moving_filler(required), &sample, LONG_TEXT),
            moved
        );
    }
}

#[test]
fn widow_control_keeps_three_lines_whose_last_fits_without_its_trailing_space() {
    let sample = multiple(1.5);
    let (height, trailing) = sample_line(&sample);
    let required = 3.0 * height - trailing;
    assert_eq!(
        sample_pages(fitting_filler(required), &sample, LONG_TEXT),
        [1, 1, 1]
    );
    assert_eq!(
        sample_pages(moving_filler(required), &sample, LONG_TEXT),
        [2, 2, 2]
    );
}

#[test]
fn a_kept_paragraph_stays_when_the_next_line_fits_without_its_trailing_space() {
    let sample = format!("keep_with_next = true, {}", multiple(1.5));
    let (height, trailing) = sample_line(&sample);
    // The closing paragraph has the line height of the sample.
    let required = 2.0 * height - trailing;
    let pages = |filler: f64| {
        let plan = planned(&document(filler, &sample, "Sample."));
        let page_of = |start: &str| {
            plan.pages
                .iter()
                .position(|page| {
                    page.lines
                        .iter()
                        .any(|placed| line_text(placed).starts_with(start))
                })
                .unwrap()
                + 1
        };
        (page_of("Sample"), page_of("Closing"))
    };
    assert_eq!(pages(fitting_filler(required)), (1, 1));
    assert_eq!(pages(moving_filler(required)), (2, 2));
}

#[test]
fn a_column_foot_holds_a_line_less_its_trailing_space() {
    let (height, trailing) = sample_line(&multiple(1.5));
    let column_of = |filler: f64| {
        let source = document(filler, &multiple(1.5), "Sample.").replace(
            "page = {",
            "columns = { count = 2, gap = pt(18) },\n  page = {",
        );
        let plan = planned(&source);
        assert_eq!(plan.pages.len(), 1);
        paragraph_lines(&plan, "Sample")[0].1.line.x > 300.0
    };
    assert!(!column_of(fitting_filler(height - trailing)));
    assert!(column_of(moving_filler(height - trailing)));
}
