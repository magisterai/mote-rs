//! The shading and the borders of a paragraph in a table cell, which follow the
//! rule of body text relative to the text area of the cell.

use mote::{
    layout::{PageOperation, Rule, RuleRole},
    render,
    source::FileAssetResolver,
};

/// A table whose left border, half a point wide, begins on the left margin of
/// 72 points, and whose cell margins are 8 points on each side. The text area
/// of the first of two columns of 200 points therefore runs from 80.25 to
/// 264.25 points, as in the sample that Word measured.
const TABLE: &str = "#table(width = pt(400), cell = { margin = { vertical = pt(4), horizontal = pt(8) } }, borders = { all = { style = 'single', width = pt(0.5), color = '#94a3b8' } })";
const TEXT_LEFT: f64 = 80.25;
const TEXT_RIGHT: f64 = 264.25;
const BOX: &str =
    "border = { all = { style = 'single', width = pt(1), color = '#2563eb', space = pt(3) } }";

struct PaintedPage {
    line_tops: Vec<f64>,
    line_lefts: Vec<f64>,
    paragraph_rules: Vec<Rule>,
    /// Holds the colour, the left edge, the top, the width and the height of
    /// each shading.
    shading: Vec<(String, [f64; 4])>,
    clips: Vec<[f64; 4]>,
}

impl PaintedPage {
    fn shading_of(&self, color: &str) -> Vec<[f64; 4]> {
        self.shading
            .iter()
            .filter(|(painted, _)| painted == color)
            .map(|(_, bounds)| *bounds)
            .collect()
    }

    fn horizontal_rules(&self) -> Vec<&Rule> {
        self.paragraph_rules
            .iter()
            .filter(|rule| rule.start[1] == rule.end[1])
            .collect()
    }

    fn vertical_rules(&self) -> Vec<&Rule> {
        self.paragraph_rules
            .iter()
            .filter(|rule| rule.start[0] == rule.end[0])
            .collect()
    }
}

/// Returns the pages of a Letter document with one-inch margins, and requires
/// that the plan carry no warning.
fn pages(source: &str) -> Vec<PaintedPage> {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    planned
        .output
        .pages
        .iter()
        .map(|page| {
            let mut painted = PaintedPage {
                line_tops: page.lines.iter().map(|line| line.top).collect(),
                line_lefts: page.lines.iter().map(|line| line.line.x).collect(),
                paragraph_rules: Vec::new(),
                shading: Vec::new(),
                clips: Vec::new(),
            };
            for operation in &page.operations {
                match operation {
                    PageOperation::Rule(rule) if rule.role == RuleRole::Generic => {
                        painted.paragraph_rules.push(rule.clone())
                    }
                    PageOperation::Rectangle(rectangle) => painted.shading.push((
                        rectangle.color.clone(),
                        [
                            rectangle.x,
                            rectangle.top,
                            rectangle.width,
                            rectangle.height,
                        ],
                    )),
                    PageOperation::PushClip(bounds) => painted.clips.push(*bounds),
                    _ => {}
                }
            }
            painted
        })
        .collect()
}

fn two_cells(first: &str, second: &str) -> String {
    format!(
        "{TABLE}[\n#row[\n#cell(width = pt(200))[{first}]\n#cell(width = pt(200))[{second}]\n]\n]\n"
    )
}

fn assert_near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 0.001, "{actual} {expected}");
}

#[test]
fn the_shading_of_a_paragraph_passes_the_text_area_of_its_cell_as_in_body_text() {
    let page = &pages(&two_cells(
        "#paragraph(fill = '#10e0f0')[Shaded.]",
        "Plain.",
    ))[0];
    assert_near(page.line_lefts[0], TEXT_LEFT);
    let shading = page.shading_of("10E0F0");
    assert_eq!(shading.len(), 1, "{:?}", page.shading);
    // Word shades from 78.72 to 265.68 on its grid of 0.48 point.
    assert_near(shading[0][0], TEXT_LEFT - 1.5);
    assert_near(shading[0][0] + shading[0][2], TEXT_RIGHT + 1.5);
    assert!(page.paragraph_rules.is_empty());
}

#[test]
fn a_boxed_paragraph_in_a_cell_keeps_the_rules_and_the_space_of_body_text() {
    let boxed = format!("#paragraph(fill = '#10e0f0', {BOX})[Boxed.]");
    let page = &pages(&two_cells(&boxed, "Plain."))[0];
    let plain = &pages(&two_cells("Boxed.", "Plain."))[0];
    // The top rule and its space lower the first line by four points, and the
    // bottom rule and its space add as much below it.
    assert_near(page.line_tops[0], plain.line_tops[0]);
    let shading = page.shading_of("10E0F0");
    assert_near(shading[0][0], TEXT_LEFT - 1.5 - 3.0 - 0.5);
    assert_near(shading[0][0] + shading[0][2], TEXT_RIGHT + 1.5 + 3.0 + 0.5);
    let plain_height = plain.clips[0][3];
    assert_near(page.clips[0][3], plain_height + 8.0);
    let sides: Vec<_> = page
        .vertical_rules()
        .iter()
        .map(|rule| rule.start[0])
        .collect();
    assert_eq!(sides.len(), 2);
    assert_near(sides[0], TEXT_LEFT - 1.5 - 3.0 - 0.5);
    assert_near(sides[1], TEXT_RIGHT + 1.5 + 3.0 + 0.5);
    let across = page.horizontal_rules();
    assert_eq!(across.len(), 2);
    for rule in across {
        assert_near(rule.start[0], TEXT_LEFT - 1.5 - 3.0 - 1.0);
        assert_near(rule.end[0], TEXT_RIGHT + 1.5 + 3.0 + 1.0);
    }
    assert_near(
        shading[0][3],
        page.horizontal_rules()[1].start[1] + 0.5 - shading[0][1],
    );
}

#[test]
fn the_shading_and_the_rules_of_a_paragraph_are_clipped_to_its_cell() {
    let wide = "#paragraph(fill = '#10e0f0', border = { all = { style = 'single', width = pt(2), color = '#2563eb', space = pt(12) } })[Boxed.]";
    let page = &pages(&two_cells(wide, "Plain."))[0];
    let shading = page.shading_of("10E0F0");
    // The shading is planned beyond the cell, whose clip bounds it.
    assert!(shading[0][0] < 72.25);
    assert_near(page.clips[0][0], 72.25);
    assert_near(page.clips[0][2], 200.0);
}

#[test]
fn adjacent_paragraphs_with_identical_borders_share_one_box_in_a_cell() {
    let boxed = |text: &str| format!("#paragraph(fill = '#10e0f0', {BOX})[{text}]\n\n");
    let alone = &pages(&two_cells(&boxed("First."), "Plain."))[0];
    let joined = &pages(&two_cells(
        &format!("{}{}", boxed("First."), boxed("Second.")),
        "Plain.",
    ))[0];
    let plain = &pages(&two_cells("First.\n\nSecond.", "Plain."))[0];
    assert_eq!(joined.horizontal_rules().len(), 2);
    assert_near(joined.line_tops[0], alone.line_tops[0]);
    assert_near(
        joined.clips[0][3] - alone.clips[0][3],
        plain.line_tops[1] - plain.line_tops[0],
    );
    let mut spans: Vec<_> = joined
        .shading_of("10E0F0")
        .iter()
        .map(|bounds| [bounds[1], bounds[1] + bounds[3]])
        .collect();
    spans.sort_by(|first, second| first[0].total_cmp(&second[0]));
    assert!(
        spans
            .windows(2)
            .all(|pair| (pair[0][1] - pair[1][0]).abs() < 0.001),
        "{spans:?}"
    );
}

#[test]
fn shading_crosses_the_space_between_two_shaded_paragraphs_in_a_cell() {
    let shaded = |text: &str| {
        format!("#paragraph(fill = '#10e0f0', spacing = {{ after = pt(10) }})[{text}]\n\n")
    };
    let page = &pages(&two_cells(
        &format!("{}{}", shaded("First."), shaded("Second.")),
        "Plain.",
    ))[0];
    let mut spans: Vec<_> = page
        .shading_of("10E0F0")
        .iter()
        .map(|bounds| [bounds[1], bounds[1] + bounds[3]])
        .collect();
    spans.sort_by(|first, second| first[0].total_cmp(&second[0]));
    assert_eq!(spans.len(), 3, "{spans:?}");
    assert_near(spans[1][1] - spans[1][0], 10.0);
    assert_near(spans[0][1], spans[1][0]);
    assert_near(spans[1][1], spans[2][0]);
}

#[test]
fn a_cell_fill_lies_beneath_the_shading_of_its_paragraphs() {
    let source = format!(
        "{TABLE}[\n#row[\n#cell(width = pt(200), fill = '#fff7cc')[#paragraph(fill = '#10e0f0')[Shaded.]]\n#cell(width = pt(200))[Plain.]\n]\n]\n"
    );
    let page = &pages(&source)[0];
    let order: Vec<_> = page
        .shading
        .iter()
        .map(|(color, _)| color.as_str())
        .collect();
    assert_eq!(order, ["FFF7CC", "10E0F0"]);
}

#[test]
fn a_box_that_splits_with_its_row_is_left_open_at_the_break() {
    let filler = "Filler.\n\n".repeat(21);
    let long = "A boxed paragraph that runs over several lines in its cell. ".repeat(8);
    // A table is begun on the page that holds its first row entire, so that a
    // short row precedes the row that splits.
    let source = format!(
        "{filler}{TABLE}[\n#row[#cell(width = pt(200))[Above.]#cell(width = pt(200))[Above.]]\n#row[#cell[#paragraph(fill = '#10e0f0', {BOX})[{long}]]#cell[Plain.]]\n]\n"
    );
    let painted = pages(&source);
    assert_eq!(painted.len(), 2);
    let [first, second] = [&painted[0], &painted[1]];
    assert!(!first.shading_of("10E0F0").is_empty());
    assert!(!second.shading_of("10E0F0").is_empty());
    assert_eq!(first.horizontal_rules().len(), 1);
    assert_eq!(second.horizontal_rules().len(), 1);
    let top_rule = first.horizontal_rules()[0].start[1];
    let bottom_rule = second.horizontal_rules()[0].start[1];
    assert!(
        first
            .shading_of("10E0F0")
            .iter()
            .all(|bounds| bounds[1] + 0.001 > top_rule - 0.5)
    );
    assert!(
        second
            .shading_of("10E0F0")
            .iter()
            .all(|bounds| bounds[1] + bounds[3] < bottom_rule + 0.5 + 0.001)
    );
    assert!(!first.vertical_rules().is_empty() && !second.vertical_rules().is_empty());
}

#[test]
fn a_repeated_header_a_merged_cell_and_a_nested_table_shade_their_paragraphs() {
    let filler = "Filler.\n\n".repeat(26);
    let rows = "#row[#cell[Body.]#cell[Body.]]\n".repeat(6);
    let source = format!(
        "{filler}{TABLE}[\n#row(repeat_header = true)[#cell(colspan = 2)[#paragraph(fill = '#10e0f0')[Header.]]]\n{rows}#row[#cell(rowspan = 2, vertical_alignment = 'bottom')[#paragraph(fill = '#16e0f0')[Merged.]]#cell[Body.]]\n#row[#cell[#table[#row[#cell[#paragraph(fill = '#1ce0f0')[Nested.]]]]]]\n]\n"
    );
    let painted = pages(&source);
    assert_eq!(painted.len(), 2);
    for page in &painted {
        assert_eq!(page.shading_of("10E0F0").len(), 1);
    }
    let merged = painted[1].shading_of("16E0F0");
    let nested = painted[1].shading_of("1CE0F0");
    assert_eq!((merged.len(), nested.len()), (1, 1));
    // The merged cell sets its paragraph on the bottom of its two rows, beside
    // the nested table of the second.
    assert!(merged[0][1] + merged[0][3] > nested[0][1]);
}

#[test]
fn a_table_without_paragraph_shading_or_borders_is_planned_as_before() {
    let plain = &pages(&two_cells("First.\n\nSecond.", "Plain."))[0];
    let shaded = &pages(&two_cells(
        "#paragraph(fill = '#10e0f0')[First.]\n\n#paragraph(fill = '#10e0f0')[Second.]",
        "Plain.",
    ))[0];
    assert_eq!(plain.line_tops, shaded.line_tops);
    assert_eq!(plain.line_lefts, shaded.line_lefts);
    assert_eq!(plain.clips, shaded.clips);
    assert!(plain.shading.is_empty() && plain.paragraph_rules.is_empty());
}
