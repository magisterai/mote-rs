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
fn the_shading_of_a_paragraph_with_one_side_rule_stays_at_the_text() {
    let page = first_page(&format!(
        "#paragraph(fill='#eef4ff',{LEFT_RULE})[First.]\n\n#paragraph(fill='#eef4ff',{LEFT_RULE})[Second.]\n\n#paragraph(fill='#eef4ff')[Third.]\n"
    ));
    assert!(
        page.shading
            .iter()
            .all(|band| band[0] == 72.0 && band[2] == 468.0)
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
