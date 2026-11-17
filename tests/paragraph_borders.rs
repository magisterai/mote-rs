//! The distance of a paragraph border from its text, the shading within it, and
//! the box that paragraphs with identical borders share.

use mote::{
    layout::{PageOperation, Rule},
    render,
    source::FileAssetResolver,
};

const SOURCE: &str = "#paragraph(border={all={style='single',width=pt(2),color='#2563eb',space=SPACE}},fill='#eef4ff')[A boxed paragraph.]";

#[test]
fn a_border_space_is_a_length_of_at_most_thirty_one_points() {
    for space in ["8", "pt(8)", "inch(0.111)"] {
        let document = mote::compile(&SOURCE.replace("SPACE", space)).unwrap();
        let package = mote::docx::emit(&document).unwrap();
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(package)).unwrap();
        let mut xml = String::new();
        std::io::Read::read_to_string(&mut archive.by_name("word/document.xml").unwrap(), &mut xml)
            .unwrap();
        assert_eq!(xml.matches(r#"w:space="8""#).count(), 4, "{space}");
    }
    for space in ["pt(32)", "-1", "'wide'", "true"] {
        let error = mote::compile(&SOURCE.replace("SPACE", space))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("border space must be a length from 0 to 31 points"),
            "{space}: {error}"
        );
    }
}

#[test]
fn the_shading_of_a_paragraph_reaches_its_side_rules() {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let shading = |space: &str| {
        let document = mote::compile(&SOURCE.replace("SPACE", space)).unwrap();
        let planned = render::plan(&document, &resolver, None).unwrap();
        assert_eq!(planned.warnings, Vec::<String>::new());
        planned.output.pages[0]
            .operations
            .iter()
            .find_map(|operation| match operation {
                PageOperation::Rectangle(rectangle) if rectangle.color == "EEF4FF" => {
                    Some([rectangle.x, rectangle.width])
                }
                _ => None,
            })
            .unwrap()
    };
    let [near, narrow] = shading("0");
    let [far, wide] = shading("pt(8)");
    // Each side rule is two points wide, and the shading meets its middle.
    assert!((near - far - 8.0).abs() < 0.001, "{near} {far}");
    assert!((wide - narrow - 16.0).abs() < 0.001, "{wide} {narrow}");
}

const BOX: &str = "border={all={style='single',width=pt(1.5),color='#2563eb',space=pt(6)}}";
const LEFT_RULE: &str = "border={left={style='single',width=pt(3),color='#2563eb',space=pt(4)}}";

struct PaintedPage {
    line_tops: Vec<f64>,
    rules: Vec<Rule>,
    /// Holds the left edge, the top, the width and the height of each shading.
    shading: Vec<[f64; 4]>,
}

impl PaintedPage {
    fn horizontal_rules(&self) -> Vec<f64> {
        self.rules
            .iter()
            .filter(|rule| rule.start[1] == rule.end[1])
            .map(|rule| rule.start[1])
            .collect()
    }

    fn vertical_rules(&self) -> Vec<&Rule> {
        self.rules
            .iter()
            .filter(|rule| rule.start[0] == rule.end[0])
            .collect()
    }
}

/// Returns the first page of a Letter document with one-inch margins, whose
/// text therefore runs from 72 to 540 points across the page.
fn first_page(source: &str) -> PaintedPage {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = mote::compile(source).unwrap();
    let planned = render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    let page = &planned.output.pages[0];
    let mut painted = PaintedPage {
        line_tops: page.lines.iter().map(|line| line.top).collect(),
        rules: Vec::new(),
        shading: Vec::new(),
    };
    for operation in &page.operations {
        match operation {
            PageOperation::Rule(rule) => painted.rules.push(rule.clone()),
            PageOperation::Rectangle(rectangle) => painted.shading.push([
                rectangle.x,
                rectangle.top,
                rectangle.width,
                rectangle.height,
            ]),
            _ => {}
        }
    }
    painted
}

fn assert_contiguous(mut spans: Vec<[f64; 2]>) {
    spans.sort_by(|first, second| first[0].total_cmp(&second[0]));
    assert!(
        spans
            .windows(2)
            .all(|pair| (pair[0][1] - pair[1][0]).abs() < 0.001),
        "{spans:?}"
    );
}

#[test]
fn paragraphs_with_identical_borders_share_one_box() {
    let boxed = |text: &str| format!("#paragraph(fill='#eef4ff',{BOX})[{text}]\n\n");
    let alone = first_page(&boxed("First."));
    let plain = first_page("First.\n\nSecond.\n");
    let pitch = plain.line_tops[1] - plain.line_tops[0];
    let page = first_page(&format!(
        "{}{}{}After.\n",
        boxed("First."),
        boxed("Second."),
        boxed("Third.")
    ));
    let tops = &page.line_tops;
    // One rule above the first paragraph and one below the third remain.
    let across = page.horizontal_rules();
    assert_eq!(across.len(), 2, "{:?}", page.rules);
    assert!(across[0] < tops[0] + 7.5 && across[1] > tops[2]);
    // The paragraphs follow one another at the pitch of unbordered ones. The
    // first line alone holds the space and the width of the top rule.
    assert_eq!(tops[0], alone.line_tops[0]);
    assert!((tops[1] - tops[0] - 7.5 - pitch).abs() < 0.001, "{tops:?}");
    assert!((tops[2] - tops[1] - pitch).abs() < 0.001, "{tops:?}");
    for x in [72.0 - 1.5 - 6.0 - 0.75, 540.0 + 1.5 + 6.0 + 0.75] {
        let pieces: Vec<_> = page
            .vertical_rules()
            .into_iter()
            .filter(|rule| (rule.start[0] - x).abs() < 0.001)
            .map(|rule| [rule.start[1], rule.end[1]])
            .collect();
        assert_eq!(pieces.len(), 3, "{x}: {:?}", page.rules);
        assert_contiguous(pieces);
    }
    assert_eq!(page.shading.len(), 5);
    assert_contiguous(
        page.shading
            .iter()
            .map(|band| [band[1], band[1] + band[3]])
            .collect(),
    );
}

#[test]
fn paragraphs_whose_borders_or_side_indents_differ_keep_their_own_boxes() {
    for second in [
        "border={all={style='single',width=pt(1.5),color='#b45309',space=pt(6)}}".to_owned(),
        "border={all={style='single',width=pt(2),color='#2563eb',space=pt(6)}}".to_owned(),
        "border={all={style='single',width=pt(1.5),color='#2563eb',space=pt(4)}}".to_owned(),
        format!("indent={{left=pt(20)}},{BOX}"),
        format!("indent={{right=pt(20)}},{BOX}"),
    ] {
        let page = first_page(&format!(
            "#paragraph(fill='#eef4ff',{BOX})[First.]\n\n#paragraph(fill='#eef4ff',{second})[Second.]\n"
        ));
        assert_eq!(page.horizontal_rules().len(), 4, "{second}");
        // The space between the two boxes is not shaded.
        assert_eq!(page.shading.len(), 2, "{second}");
    }
    let page = first_page(&format!(
        "#paragraph(fill='#eef4ff',{BOX})[First.]\n\n#paragraph(alignment='center',indent={{first_line=pt(20)}},spacing={{before=pt(12),after=pt(3)}},{BOX})[Second.]\n"
    ));
    assert_eq!(page.horizontal_rules().len(), 2);
}

#[test]
fn a_side_rule_lies_its_space_and_a_clearance_outside_the_text() {
    for (width, space) in [(0.5, 0.0), (1.0, 4.0), (3.0, 12.0), (6.0, 20.0)] {
        let page = first_page(&format!(
            "#paragraph(fill='#eef4ff',border={{all={{style='single',width=pt({width}),color='#2563eb',space=pt({space})}}}})[Boxed.]\n"
        ));
        let sides: Vec<_> = page
            .vertical_rules()
            .into_iter()
            .map(|rule| rule.start[0])
            .collect();
        // The inner edge of each rule is 1.5 points and the space from the text.
        let inner = 1.5 + space;
        assert_eq!(sides.len(), 2);
        assert!(
            (sides[0] + width / 2.0 - (72.0 - inner)).abs() < 0.001,
            "{sides:?}"
        );
        assert!(
            (sides[1] - width / 2.0 - (540.0 + inner)).abs() < 0.001,
            "{sides:?}"
        );
        assert!((page.shading[0][0] - sides[0]).abs() < 0.001);
    }
}

#[test]
fn the_shading_of_a_paragraph_with_a_left_rule_alone_keeps_its_clearance() {
    let page = first_page(&format!(
        "#paragraph(fill='#eef4ff',{LEFT_RULE})[First.]\n\n#paragraph(fill='#eef4ff',{LEFT_RULE})[Second.]\n\n#paragraph(fill='#eef4ff')[Third.]\n"
    ));
    assert!(
        page.shading
            .iter()
            .all(|band| band[0] == 70.5 && band[2] == 471.0),
        "{:?}",
        page.shading
    );
    // The rule of the two joined paragraphs crosses the space between them. The
    // shading crosses that space, and not the space below the second paragraph,
    // whose borders the third does not share.
    assert_eq!(page.rules.len(), 2);
    assert_contiguous(
        page.rules
            .iter()
            .map(|rule| [rule.start[1], rule.end[1]])
            .collect(),
    );
    assert_eq!(page.shading.len(), 4);
}

/// Holds the name of a side, and the width and the space of its rule in points.
type SideRule = (&'static str, f64, f64);

/// Returns a further argument that gives the named sides their rules, or
/// nothing where no side is named.
fn rules(sides: &[SideRule]) -> String {
    if sides.is_empty() {
        return String::new();
    }
    let sides: Vec<_> = sides
        .iter()
        .map(|(name, width, space)| {
            format!("{name}={{style='single',width=pt({width}),color='#2563eb',space=pt({space})}}")
        })
        .collect();
    format!(",border={{{}}}", sides.join(","))
}

fn assert_span(actual: [f64; 2], expected: [f64; 2], context: &str) {
    assert!(
        (actual[0] - expected[0]).abs() < 0.001 && (actual[1] - expected[1]).abs() < 0.001,
        "{context}: {actual:?} is not {expected:?}"
    );
}

fn horizontal_extent(band: &[f64; 4]) -> [f64; 2] {
    [band[0], band[0] + band[2]]
}

/// The figures of the tests below are those that Word gave the
/// paragraph-shading probe, each within the quarter point of the 0.48 point
/// grid of Word's PDF export. The native shading passes half the width of a
/// rule beneath a rule whose inner edge it meets, where Word's ends.
#[test]
fn shading_passes_the_text_by_a_clearance_and_follows_the_right_rule() {
    let cases: [(&[SideRule], [f64; 2]); 10] = [
        (&[], [70.5, 541.5]),
        (&[("top", 3.0, 4.0), ("bottom", 3.0, 4.0)], [70.5, 541.5]),
        (&[("left", 3.0, 4.0)], [70.5, 541.5]),
        (&[("right", 3.0, 4.0)], [70.5, 545.5 + 1.5]),
        (
            &[("left", 3.0, 4.0), ("right", 3.0, 4.0)],
            [66.5 - 1.5, 545.5 + 1.5],
        ),
        (
            &[
                ("left", 3.0, 4.0),
                ("right", 3.0, 4.0),
                ("top", 3.0, 4.0),
                ("bottom", 3.0, 4.0),
            ],
            [66.5 - 1.5, 545.5 + 1.5],
        ),
        (
            &[("left", 3.0, 4.0), ("top", 3.0, 4.0), ("bottom", 3.0, 4.0)],
            [70.5, 541.5],
        ),
        (
            &[("right", 3.0, 4.0), ("top", 3.0, 4.0), ("bottom", 3.0, 4.0)],
            [70.5, 545.5 + 1.5],
        ),
        // Word measures the shading on both sides from the right rule, so that
        // the shading passes a nearer left rule and stops short of a farther one.
        (
            &[("left", 1.0, 4.0), ("right", 2.0, 12.0)],
            [58.5, 553.5 + 1.0],
        ),
        (
            &[("left", 1.0, 12.0), ("right", 2.0, 4.0)],
            [66.5, 545.5 + 1.0],
        ),
    ];
    for (sides, expected) in cases {
        let border = rules(sides);
        let page = first_page(&format!("#paragraph(fill='#eef4ff'{border})[Shaded.]\n"));
        assert_eq!(page.shading.len(), 1, "{border}");
        assert_span(horizontal_extent(&page.shading[0]), expected, &border);
    }
}

#[test]
fn a_top_or_bottom_rule_passes_the_text_by_a_clearance_on_a_side_without_a_rule() {
    let cases: [(&[&str], [f64; 2]); 4] = [
        (&["top", "bottom"], [70.5, 541.5]),
        (&["left", "top", "bottom"], [63.5, 541.5]),
        (&["right", "top", "bottom"], [70.5, 548.5]),
        (&["left", "right", "top", "bottom"], [63.5, 548.5]),
    ];
    for (names, expected) in cases {
        let sides: Vec<_> = names.iter().map(|name| (*name, 3.0, 4.0)).collect();
        let border = rules(&sides);
        let page = first_page(&format!("#paragraph(fill='#eef4ff'{border})[Ruled.]\n"));
        let across: Vec<_> = page
            .rules
            .iter()
            .filter(|rule| rule.start[1] == rule.end[1])
            .collect();
        assert_eq!(across.len(), 2, "{border}");
        for rule in across {
            assert_span([rule.start[0], rule.end[0]], expected, &border);
        }
    }
}

const TWO_LINES: &str = "The text runs on to a second line, so that the first line and the line after it are both measured against the box of the paragraph, whatever the indent of either.";

#[test]
fn the_box_of_a_paragraph_begins_at_its_leftmost_line() {
    for (indent, expected) in [
        ("left=pt(30)", [100.5, 541.5]),
        ("right=pt(30)", [70.5, 511.5]),
        ("left=pt(30),first_line=pt(20)", [100.5, 541.5]),
        ("left=pt(30),hanging=pt(20)", [80.5, 541.5]),
        ("first_line=pt(20)", [70.5, 541.5]),
        ("hanging=pt(20)", [50.5, 541.5]),
        ("left=pt(10),hanging=pt(20)", [60.5, 541.5]),
        ("left=pt(30),right=pt(40),first_line=pt(20)", [100.5, 501.5]),
        ("left=pt(30),right=pt(40),hanging=pt(20)", [80.5, 501.5]),
    ] {
        let page = first_page(&format!(
            "#paragraph(fill='#eef4ff',indent={{{indent}}})[{TWO_LINES}]\n"
        ));
        assert!(page.shading.len() > 1, "{indent}");
        for band in &page.shading {
            assert_span(horizontal_extent(band), expected, indent);
        }
    }
    for alignment in ["center", "right", "justify"] {
        let page = first_page(&format!(
            "#paragraph(fill='#eef4ff',alignment='{alignment}')[Shaded.]\n"
        ));
        assert_span(
            horizontal_extent(&page.shading[0]),
            [70.5, 541.5],
            alignment,
        );
    }
}

#[test]
fn the_rules_of_a_paragraph_follow_its_box_on_every_line() {
    let border = rules(&[
        ("left", 3.0, 4.0),
        ("right", 3.0, 4.0),
        ("top", 3.0, 4.0),
        ("bottom", 3.0, 4.0),
    ]);
    for (indent, inner_edge) in [
        ("left=pt(30),first_line=pt(20)", 96.5),
        ("left=pt(30),hanging=pt(20)", 76.5),
    ] {
        let page = first_page(&format!(
            "#paragraph(fill='#eef4ff',indent={{{indent}}}{border})[{TWO_LINES}]\n"
        ));
        let sides: Vec<_> = page
            .vertical_rules()
            .into_iter()
            .map(|rule| rule.start[0])
            .collect();
        assert!(sides.len() >= 4, "{indent}");
        assert!(
            sides
                .iter()
                .all(|x| *x == inner_edge - 1.5 || *x == 545.5 + 1.5),
            "{indent}: {sides:?}"
        );
        for band in &page.shading {
            assert_span(
                horizontal_extent(band),
                [inner_edge - 1.5, 545.5 + 1.5],
                indent,
            );
        }
        for rule in page
            .rules
            .iter()
            .filter(|rule| rule.start[1] == rule.end[1])
        {
            assert_span(
                [rule.start[0], rule.end[0]],
                [inner_edge - 3.0, 548.5],
                indent,
            );
        }
    }
}

#[test]
fn a_shaded_heading_and_a_shaded_quotation_are_shaded_as_paragraphs_are() {
    for (source, expected) in [
        ("#heading(level=2,fill='#eef4ff')[Shaded.]\n", [70.5, 541.5]),
        (
            "#blockquote(fill='#eef4ff',indent={left=pt(24)})[Shaded.]\n",
            [94.5, 541.5],
        ),
    ] {
        let page = first_page(source);
        assert_eq!(page.shading.len(), 1, "{source}");
        assert_span(horizontal_extent(&page.shading[0]), expected, source);
    }
}

#[test]
fn shading_crosses_the_space_between_unbordered_paragraphs_with_one_box() {
    let bands = |first: &str, second: &str| {
        first_page(&format!(
            "#paragraph(fill='#eef4ff',spacing={{after=pt(12)}}{first})[First.]\n\n#paragraph(fill='#fff4e5',spacing={{after=pt(12)}}{second})[Second.]\n"
        ))
        .shading
    };
    for (first, second) in [
        ("", ""),
        (
            ",indent={left=pt(20),right=pt(10)}",
            ",indent={left=pt(20),right=pt(10)}",
        ),
        (
            ",indent={left=pt(30)}",
            ",indent={left=pt(30),first_line=pt(20)}",
        ),
        (
            ",indent={left=pt(30),hanging=pt(20)}",
            ",indent={left=pt(10)}",
        ),
    ] {
        let shading = bands(first, second);
        assert_eq!(shading.len(), 3, "{first} {second}");
        assert!((shading[1][3] - 12.0).abs() < 0.001, "{shading:?}");
        assert_contiguous(
            shading
                .iter()
                .map(|band| [band[1], band[1] + band[3]])
                .collect(),
        );
    }
    for (first, second) in [
        ("", ",indent={left=pt(20)}"),
        (",indent={right=pt(20)}", ""),
        (
            ",indent={left=pt(30)}",
            ",indent={left=pt(30),hanging=pt(20)}",
        ),
    ] {
        assert_eq!(bands(first, second).len(), 2, "{first} {second}");
    }
}

#[test]
fn the_space_before_an_unbordered_paragraph_takes_its_shading_beyond_the_space_above() {
    let page = first_page(
        "#paragraph(fill='#eef4ff',spacing={after=pt(4)})[First.]\n\n#paragraph(fill='#fff4e5',spacing={before=pt(14)})[Second.]\n\n#paragraph(spacing={after=pt(4)})[Unshaded.]\n\n#paragraph(fill='#fff4e5',spacing={before=pt(14)})[Fourth.]\n",
    );
    let heights: Vec<_> = page.shading.iter().map(|band| band[3]).collect();
    // The fourth paragraph follows an unshaded one, and the space before it is
    // unshaded.
    assert_eq!(heights.len(), 5, "{heights:?}");
    assert!((heights[1] - 4.0).abs() < 0.001, "{heights:?}");
    assert!((heights[2] - 10.0).abs() < 0.001, "{heights:?}");
    assert_contiguous(
        page.shading[..4]
            .iter()
            .map(|band| [band[1], band[1] + band[3]])
            .collect(),
    );
}

#[test]
fn paragraphs_with_identical_borders_share_a_box_only_if_their_boxes_agree() {
    let horizontal_rules = |first: &str, second: &str| {
        first_page(&format!(
            "#paragraph(fill='#eef4ff',indent={{{first}}},{BOX})[First.]\n\n#paragraph(fill='#eef4ff',indent={{{second}}},{BOX})[Second.]\n"
        ))
        .horizontal_rules()
        .len()
    };
    assert_eq!(
        horizontal_rules("left=pt(30),hanging=pt(20)", "left=pt(10)"),
        2
    );
    assert_eq!(
        horizontal_rules("left=pt(30),first_line=pt(10)", "left=pt(30)"),
        2
    );
    assert_eq!(
        horizontal_rules("left=pt(30)", "left=pt(30),hanging=pt(20)"),
        4
    );
}

#[test]
fn shading_crosses_the_space_between_unbordered_paragraphs_alone() {
    let gap_is_shaded = |first: &str, second: &str| {
        first_page(&format!(
            "#paragraph(fill='#eef4ff'{first})[First.]\n\n#paragraph(fill='#fff4e5'{second})[Second.]\n"
        ))
        .shading
        .len()
            == 3
    };
    assert!(gap_is_shaded("", ""));
    assert!(!gap_is_shaded(
        ",border={bottom={style='single',width=pt(1),color='#2563eb'}}",
        ""
    ));
    assert!(!gap_is_shaded(
        "",
        ",border={top={style='single',width=pt(1),color='#2563eb'}}"
    ));
}
