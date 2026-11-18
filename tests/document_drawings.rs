//! A shape or a group of a document whose position states no place is set in
//! the text and takes the room of its frame; one that states a place is fixed
//! on its page. A group is one drawing in either case.
use mote::{
    layout::{DocumentTextPlan, PageOperation},
    render,
    source::FileAssetResolver,
};
use std::io::{Cursor, Read};

const BAR: &str = "#shape(position={x=pt(0),y=pt(0),width=pt(144),height=pt(18)},fill='#2563eb')";

fn planned(source: &str) -> (DocumentTextPlan, Vec<String>) {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let rendered = render::plan(&document, &resolver, None).unwrap();
    let warnings = rendered
        .warnings
        .into_iter()
        .chain(rendered.output.warnings.clone())
        .collect();
    (rendered.output, warnings)
}

/// Returns the top of the line of the first page that begins with `text`.
fn top_of_line(plan: &DocumentTextPlan, text: &str) -> f64 {
    plan.pages[0]
        .lines
        .iter()
        .find(|placed| {
            placed
                .line
                .fragments
                .iter()
                .map(|fragment| fragment.text.as_str())
                .collect::<String>()
                .starts_with(text)
        })
        .unwrap_or_else(|| panic!("no line begins with {text:?}"))
        .top
}

/// Returns the left, top, right and bottom of every filled path of the first
/// page, in order.
fn painted_bounds(plan: &DocumentTextPlan) -> Vec<[f64; 4]> {
    plan.pages[0]
        .operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::Path(path) if path.fill.is_some() => Some(path.points.iter().fold(
                [f64::MAX, f64::MAX, f64::MIN, f64::MIN],
                |bounds, point| {
                    [
                        bounds[0].min(point[0]),
                        bounds[1].min(point[1]),
                        bounds[2].max(point[0]),
                        bounds[3].max(point[1]),
                    ]
                },
            )),
            _ => None,
        })
        .collect()
}

fn document_xml(source: &str) -> (String, Vec<String>) {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let package = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    let mut archive = zip::ZipArchive::new(Cursor::new(package.bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("word/document.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    (xml, package.warnings)
}

fn close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 0.01
}

#[test]
fn a_shape_without_a_place_takes_the_room_of_its_frame_in_the_text() {
    let lines = "#paragraph[Before]\n#paragraph[After]";
    let (plain, _) = planned(lines);
    let (flowing, warnings) = planned(
        "#paragraph[Before]\n#shape(position={width=pt(144),height=pt(60)},fill='#dcfce7')\n#paragraph[After]",
    );
    assert!(warnings.is_empty(), "{warnings:?}");
    let before = top_of_line(&flowing, "Before");
    assert!(close(before, top_of_line(&plain, "Before")));
    // The frame and the eight points beneath it lie between the two lines.
    assert!(close(
        top_of_line(&flowing, "After") - top_of_line(&plain, "After"),
        68.0
    ));
    let [left, top, right, bottom] = painted_bounds(&flowing)[0];
    assert!(close(right - left, 144.0) && close(bottom - top, 60.0));
    assert!(top > before && bottom < top_of_line(&flowing, "After"));
    // The frame begins at the left of the text.
    let margin = flowing.pages[0].geometry.unwrap().margin_left;
    assert!(close(left, margin), "{left} {margin}");
}

#[test]
fn a_shape_with_a_place_is_fixed_and_takes_no_room() {
    let (plain, _) = planned("#paragraph[Before]\n#paragraph[After]");
    for position in [
        "x=pt(300),y=pt(400),width=pt(144),height=pt(60)",
        "y=pt(400),width=pt(144),height=pt(60)",
        "x=pt(0),y=pt(0),width=pt(144),height=pt(60)",
    ] {
        let (fixed, warnings) = planned(&format!(
            "#paragraph[Before]\n#shape(position={{{position}}},fill='#dcfce7')\n#paragraph[After]"
        ));
        assert!(warnings.is_empty(), "{warnings:?}");
        assert!(
            top_of_line(&fixed, "After") - top_of_line(&plain, "After") < 2.0,
            "{position}"
        );
        let [left, top, ..] = painted_bounds(&fixed)[0];
        let expected_left = if position.starts_with("x=pt(300)") {
            300.0
        } else {
            0.0
        };
        let expected_top = if position.contains("y=pt(400)") {
            400.0
        } else {
            0.0
        };
        assert!(
            close(left, expected_left) && close(top, expected_top),
            "{position}"
        );
    }
}

#[test]
fn a_group_without_a_place_takes_the_reach_of_its_objects_or_its_stated_frame() {
    let (plain, _) = planned("#paragraph[Before]\n#paragraph[After]");
    let second = "#shape(position={x=pt(100),y=pt(40),width=pt(100),height=pt(20)},fill='#059669')";
    let cases = [
        // The objects reach 200 points across and 60 points down.
        (String::new(), 60.0, 200.0),
        (
            "position={width=pt(300),height=pt(90)}".to_owned(),
            90.0,
            300.0,
        ),
    ];
    for (arguments, height, width) in cases {
        for (alignment, offset) in [("left", 0.0), ("center", 0.5), ("right", 1.0)] {
            let separator = if arguments.is_empty() { "" } else { "," };
            let (flowing, warnings) = planned(&format!(
                "#paragraph[Before]\n#group({arguments}{separator}alignment='{alignment}')[{BAR}{second}]\n#paragraph[After]"
            ));
            assert!(warnings.is_empty(), "{warnings:?}");
            assert!(close(
                top_of_line(&flowing, "After") - top_of_line(&plain, "After"),
                height + 8.0
            ));
            let bounds = painted_bounds(&flowing);
            let page = &flowing.pages[0];
            let geometry = page.geometry.unwrap();
            let margin = geometry.margin_left;
            let column = geometry.width - margin - geometry.margin_right;
            let corner = margin + (column - width) * offset;
            assert!(close(bounds[0][0], corner), "{alignment} {arguments}");
            assert!(close(bounds[1][0], corner + 100.0));
            assert!(close(bounds[1][1] - bounds[0][1], 40.0));
        }
    }
}

#[test]
fn a_group_with_a_place_is_fixed_about_its_corner_and_nests() {
    let (plain, _) = planned("#paragraph[Before]\n#paragraph[After]");
    let (fixed, warnings) = planned(&format!(
        "#paragraph[Before]\n#group(position={{x=pt(200),y=pt(300)}})[{BAR}#group(position={{x=pt(50),y=pt(-20)}})[{BAR}]]\n#paragraph[After]"
    ));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(top_of_line(&fixed, "After") - top_of_line(&plain, "After") < 2.0);
    let bounds = painted_bounds(&fixed);
    assert!(close(bounds[0][0], 200.0) && close(bounds[0][1], 300.0));
    assert!(close(bounds[1][0], 250.0) && close(bounds[1][1], 280.0));
}

#[test]
fn a_line_is_drawn_within_a_group_and_not_alone() {
    let line = "#line(from={x=pt(0),y=pt(30)},to={x=pt(144),y=pt(30)},border={width=pt(2),color='#dc2626'},arrow='triangle')";
    let (grouped, warnings) = planned(&format!("#group[{BAR}{line}]"));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(
        grouped.pages[0]
            .operations
            .iter()
            .any(|operation| matches!(operation, PageOperation::Path(path) if !path.closed && path.stroke.is_some()))
    );
    let (alone, warnings) = planned(line);
    assert!(
        warnings
            .iter()
            .any(|warning| warning.contains("visible [line] fallback")),
        "{warnings:?}"
    );
    assert!(
        !alone.pages[0]
            .operations
            .iter()
            .any(|operation| matches!(operation, PageOperation::Path(path) if !path.closed && path.stroke.is_some()))
    );
}

#[test]
fn docx_writes_an_inline_drawing_in_the_text_and_an_anchored_drawing_at_a_place() {
    let (flowing, warnings) =
        document_xml("#shape(position={width=pt(144),height=pt(60)},fill='#dcfce7')[Callout]");
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(flowing.contains("<wp:inline ") && !flowing.contains("<wp:anchor "));
    assert!(flowing.contains(r#"<wp:extent cx="1828800" cy="762000"/>"#));
    assert!(flowing.contains("<wps:wsp><wps:cNvSpPr/>"));

    let (fixed, warnings) = document_xml(
        "#shape(position={x=pt(72),y=pt(144),width=pt(144),height=pt(60)},fill='#dcfce7')",
    );
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(fixed.contains("<wp:anchor ") && !fixed.contains("<wp:inline "));
    assert!(fixed.contains("<wp:posOffset>914400</wp:posOffset>"));
    assert!(fixed.contains("<wp:posOffset>1828800</wp:posOffset>"));
}

#[test]
fn docx_writes_a_group_as_one_group_shape_with_its_objects_about_its_corner() {
    let line = "#line(from={x=pt(0),y=pt(30)},to={x=pt(144),y=pt(30)},arrow='triangle')";
    let nested = format!("#group(position={{x=pt(50),y=pt(40)}},rotation=30)[{BAR}]");
    let (flowing, warnings) = document_xml(&format!(
        "#group(name='Plan',alignment='center')[{BAR}{line}{nested}]"
    ));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(flowing.matches("<w:drawing>").count(), 1);
    assert!(flowing.contains("<wp:inline ") && flowing.contains(r#"<w:jc w:val="center"/>"#));
    // The objects reach 194 points across and 58 points down.
    assert!(flowing.contains(r#"<wp:extent cx="2463800" cy="736600"/>"#));
    assert!(flowing.contains(r#"<wp:docPr id="1" name="Plan"/>"#));
    assert!(flowing.contains(
        r#"<wpg:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="2463800" cy="736600"/><a:chOff x="0" y="0"/><a:chExt cx="2463800" cy="736600"/></a:xfrm></wpg:grpSpPr>"#
    ));
    assert_eq!(flowing.matches("<wps:wsp>").count(), 3);
    assert!(flowing.contains("<wps:cNvCnPr/>") && flowing.contains(r#"<a:prstGeom prst="line">"#));
    assert!(flowing.contains(
        r#"<wpg:grpSp><wpg:cNvPr id="4" name="Group 4"/><wpg:cNvGrpSpPr/><wpg:grpSpPr><a:xfrm rot="1800000"><a:off x="635000" y="508000"/>"#
    ));
    assert!(flowing.contains(r#"mc:Ignorable="wpg""#));

    let (fixed, warnings) =
        document_xml(&format!("#group(position={{x=pt(72),y=pt(144)}})[{BAR}]"));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(fixed.contains("<wp:anchor ") && fixed.contains("<wpg:wgp>"));
    assert!(fixed.contains("<wp:posOffset>914400</wp:posOffset>"));
    assert!(fixed.contains("<wp:posOffset>1828800</wp:posOffset>"));
}

#[test]
fn a_group_position_is_normalized_and_an_offset_is_an_unknown_argument() {
    let model = |source: &str| -> serde_json::Value {
        serde_json::from_str(&mote::json::to_json(&mote::compile(source).unwrap()).unwrap())
            .unwrap()
    };
    let position = &model(&format!("#group(position={{y=pt(-5),width='2in'}})[{BAR}]"))["content"]
        [0]["attributes"]["position"]
        .clone();
    assert_eq!(position["x"]["value"], 0.0);
    assert_eq!(position["y"]["value"], -5.0);
    assert_eq!(position["width"]["text"], "2in");
    assert!(position.get("height").is_none());
    let flowing = &model(&format!("#group(position={{width=pt(10)}})[{BAR}]"))["content"][0]
        ["attributes"]["position"]
        .clone();
    assert!(flowing.get("x").is_none() && flowing.get("y").is_none());
    for source in [
        format!("#group(position={{width=pt(-1)}})[{BAR}]"),
        format!("#group(position={{x='wide'}})[{BAR}]"),
        format!("#group(alignment='justify')[{BAR}]"),
    ] {
        assert!(mote::compile(&source).is_err(), "{source}");
    }
    let (_, warnings) = planned(&format!("#group(offset={{x=pt(10)}})[{BAR}]"));
    assert!(
        warnings
            .iter()
            .any(|warning| warning.contains("group.offset")),
        "{warnings:?}"
    );
}

const BOX: &str = "#shape(position={width=pt(144),height=pt(60)},fill='#dcfce7')";

#[test]
fn the_paragraph_of_a_drawing_in_the_text_has_the_spacing_of_the_default_style() {
    // Word sets the drawing in a paragraph of the Normal style, as the review
    // case of drawings under a style shows.
    let style = "#style(name='Normal',type='paragraph',paragraph={spacing={before=pt(6),after=pt(12),line={rule='exact',value=pt(16)}}})\n";
    let (plan, warnings) = planned(&format!(
        "{style}#paragraph(spacing={{after=pt(0)}})[Above]\n\n{BOX}\n\n#paragraph(spacing={{before=pt(0)}})[Below]\n"
    ));
    assert_eq!(warnings, Vec::<String>::new());
    let above = top_of_line(&plan, "Above");
    let frame = painted_bounds(&plan)[0];
    assert!(close(frame[1], above + 16.0 + 6.0), "{frame:?}");
    assert!(close(top_of_line(&plan, "Below"), frame[3] + 12.0));
    // The default style of a document without a Normal style sets 8 points
    // after a paragraph and none before it.
    let (plan, _) = planned(&format!("Above\n\n{BOX}\n\n{BOX}\n\nBelow\n"));
    let frames = painted_bounds(&plan);
    assert!(close(frames[1][1], frames[0][3] + 8.0), "{frames:?}");
}

#[test]
fn a_drawing_in_the_text_of_a_table_cell_takes_the_room_of_its_frame() {
    let (plan, warnings) = planned(&format!(
        "#table(width=pt(400))[#row[#cell(width=pt(200))[\nAbove\n\n{BOX}\n\nBelow\n]#cell(width=pt(200))[Beside]]#row[#cell[Next]#cell[Row]]]\n"
    ));
    assert_eq!(warnings, Vec::<String>::new());
    let frame = painted_bounds(&plan)[0];
    let (above, below) = (top_of_line(&plan, "Above"), top_of_line(&plan, "Below"));
    // The frame lies in the cell below the first paragraph, and the paragraph
    // after it lies below the frame and the space after its paragraph.
    assert!(
        frame[1] > above && frame[0] > 72.0 && frame[2] < 72.0 + 200.0,
        "{frame:?}"
    );
    assert!(close(frame[3] - frame[1], 60.0));
    assert!(close(below, frame[3] + 8.0), "{below} {frame:?}");
    assert!(top_of_line(&plan, "Next") > below);
}

#[test]
fn a_drawing_taller_than_a_page_stays_at_the_head_of_the_next_page() {
    let (plan, _) = planned(
        "Above\n\n#shape(position={width=pt(144),height=pt(700)},fill='#dcfce7')\n\nBelow\n",
    );
    // No page is left empty between the text above the drawing and the drawing.
    assert_eq!(plan.pages.len(), 3);
    assert!(
        plan.pages[1]
            .operations
            .iter()
            .any(|operation| matches!(operation, PageOperation::Path(path) if path.fill.is_some()))
    );
    assert_eq!(plan.pages[2].lines.len(), 1);
}

#[test]
fn docx_begins_the_paragraph_of_a_drawing_with_a_column_break_before_it() {
    // A paragraph that holds a column break alone leaves an empty line at the
    // head of the next column in Word.
    let columns = "#document(columns={count=2})\n\nAbove\n\n#break(type='column')\n\n";
    for block in [BOX, "---", "#raw[code]"] {
        let (xml, _) = document_xml(&format!("{columns}{block}\n\nBelow\n"));
        assert_eq!(
            xml.matches("<w:br w:type=\"column\"/>").count(),
            1,
            "{block}"
        );
        assert!(
            !xml.contains("<w:p><w:r><w:br w:type=\"column\"/></w:r></w:p>"),
            "{block}"
        );
    }
}
