use mote::{layout::PageOperation, render, source::FileAssetResolver};
use std::io::{Read, Write};

const EDGES: [&str; 4] = ["left", "right", "top", "bottom"];
const BLUE: [u8; 3] = [0, 0, 255];
const WHITE: [u8; 3] = [255, 255, 255];

fn deck(objects: &str, time: Option<f64>) -> mote::model::Document {
    mote::compile_with_options(
        &format!("#presentation(width=pt(200),height=pt(150))#slide[{objects}]"),
        mote::evaluation::EvaluationOptions {
            time,
            ..Default::default()
        },
    )
    .unwrap()
}

/// A bar whose extent has its left at 40, its top at 30, its right at 120 and
/// its bottom at 70, in points.
fn bar(arguments: &str) -> String {
    format!(
        "#shape(name='Bar',position={{x=pt(40),y=pt(30),width=pt(80),height=pt(40)}},fill='#0000ff'{arguments})"
    )
}

fn operations(document: &mote::model::Document) -> Vec<PageOperation> {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let planned =
        render::plan_with_layout(document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    planned.output.pages[0].operations.clone()
}

/// Returns the left, top, right and bottom of the part of an extent that the
/// clip of a wipe leaves in view. The first clip of a page is that of the
/// slide, and the clip of a wipe is the next, since it precedes every clip
/// within its object.
fn part_in_view(operations: &[PageOperation], extent: [f64; 4]) -> [f64; 4] {
    operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::PushClip([left, top, width, height]) => Some([
                left.max(extent[0]),
                top.max(extent[1]),
                (left + width).min(extent[2]),
                (top + height).min(extent[3]),
            ]),
            _ => None,
        })
        .nth(1)
        .expect("the clip of a wipe is present")
}

fn pixel(document: &mote::model::Document, [x, y]: [usize; 2]) -> [u8; 3] {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let planned =
        render::plan_with_layout(document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    render::paint_pdf(&planned.output).unwrap();
    let page = &render::paint_png(&planned.output, &render::PngOptions::default())
        .unwrap()
        .output[0];
    let mut reader = png::Decoder::new(std::io::Cursor::new(&page.bytes))
        .read_info()
        .unwrap();
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let frame = reader.next_frame(&mut pixels).unwrap();
    let offset = (y * frame.width as usize + x) * frame.color_type.samples();
    [pixels[offset], pixels[offset + 1], pixels[offset + 2]]
}

#[test]
fn a_wipe_takes_an_edge_and_no_other_effect_does() {
    let document = mote::compile(&format!(
        "#presentation(width=pt(200),height=pt(150))#slide[{}{}]",
        bar(",animate={effect='wipe_in'}"),
        bar(",animate={{effect='wipe_in',from='top',duration=2},\
             {effect='wipe_out',from='bottom',start=3,easing='ease_out'}}")
    ))
    .unwrap();
    let model: serde_json::Value =
        serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
    let shapes = &model["content"][0]["children"];
    assert_eq!(
        shapes[0]["attributes"]["animate"],
        serde_json::json!([{"effect": "wipe_in"}])
    );
    let effects = &shapes[1]["attributes"]["animate"];
    assert_eq!(effects[0]["effect"], "wipe_in");
    assert_eq!(effects[0]["from"], "top");
    assert_eq!(effects[0]["duration"].as_f64(), Some(2.0));
    assert_eq!(effects[1]["effect"], "wipe_out");
    assert_eq!(effects[1]["from"], "bottom");
    assert_eq!(effects[1]["start"].as_f64(), Some(3.0));
    assert_eq!(effects[1]["easing"], "ease_out");
    for edge in EDGES {
        for effect in ["wipe_in", "wipe_out"] {
            let source = bar(&format!(",animate={{effect='{effect}',from='{edge}'}}"));
            assert!(
                mote::compile(&format!("#slide[{source}]")).is_ok(),
                "{source}"
            );
        }
    }

    for (animate, message) in [
        (
            "{effect='fade_in',from='left'}",
            "animate fade_in does not accept from",
        ),
        (
            "{effect='move',by={x=pt(1)},from='left'}",
            "animate move does not accept from",
        ),
        (
            "{effect='appear',from='top'}",
            "animate appear does not accept from",
        ),
        (
            "{effect='wipe_in',from='center'}",
            "animate from must be one of left, right, top, bottom",
        ),
        (
            "{effect='wipe_out',from=1}",
            "animate from must be one of left, right, top, bottom",
        ),
        (
            "{effect='wipe_in',by={x=pt(1)}}",
            "animate wipe_in does not accept by",
        ),
        (
            "{effect='wipe_out',duration=-1}",
            "animate duration must be a number of seconds, not negative",
        ),
    ] {
        let source = format!("#slide[{}]", bar(&format!(",animate={animate}")));
        let error = mote::compile(&source).unwrap_err().to_string();
        assert!(error.contains(message), "{source}: {error}");
    }
}

#[test]
fn a_wipe_clips_an_object_to_the_part_its_progress_leaves_in_view() {
    let extent = [40.0, 30.0, 120.0, 70.0];
    let authored = operations(&deck(&bar(""), None));
    let absent = operations(&deck("", Some(0.0)));

    // The first point of each pair lies in the half next to the named edge.
    let halves = [
        ("left", [40.0, 30.0, 80.0, 70.0], [[50, 50], [110, 50]]),
        ("right", [80.0, 30.0, 120.0, 70.0], [[110, 50], [50, 50]]),
        ("top", [40.0, 30.0, 120.0, 50.0], [[80, 35], [80, 65]]),
        ("bottom", [40.0, 50.0, 120.0, 70.0], [[80, 65], [80, 35]]),
    ];
    for (index, (edge, near_half, [near, far])) in halves.into_iter().enumerate() {
        let entering = bar(&format!(
            ",animate={{effect='wipe_in',from='{edge}',start=1,duration=2}}"
        ));
        assert_eq!(operations(&deck(&entering, None)), authored, "{edge}");
        assert_eq!(operations(&deck(&entering, Some(0.5))), absent, "{edge}");
        let half = deck(&entering, Some(2.0));
        assert_eq!(
            part_in_view(&operations(&half), extent),
            near_half,
            "{edge}"
        );
        assert_eq!(pixel(&half, near), BLUE, "{edge}");
        assert_eq!(pixel(&half, far), WHITE, "{edge}");
        assert_eq!(operations(&deck(&entering, Some(3.0))), authored, "{edge}");

        // The hiding advances from the named edge, so that the half in view
        // is the one that a `wipe_in` from the opposite edge reveals first.
        let leaving = bar(&format!(
            ",animate={{effect='wipe_out',from='{edge}',start=1,duration=2}}"
        ));
        assert_eq!(operations(&deck(&leaving, None)), authored, "{edge}");
        assert_eq!(operations(&deck(&leaving, Some(0.5))), authored, "{edge}");
        let half = deck(&leaving, Some(2.0));
        assert_eq!(
            part_in_view(&operations(&half), extent),
            halves[index ^ 1].1,
            "{edge}"
        );
        assert_eq!(pixel(&half, near), WHITE, "{edge}");
        assert_eq!(pixel(&half, far), BLUE, "{edge}");
        assert_eq!(operations(&deck(&leaving, Some(3.0))), absent, "{edge}");
    }

    // An effect that loses speed through its second half has covered two
    // thirds of its way at its middle.
    let eased = bar(",animate={effect='wipe_in',duration=3,easing='ease_out'}");
    let [left, _, right, _] = part_in_view(&operations(&deck(&eased, Some(1.5))), extent);
    assert_eq!(left, 40.0);
    assert!((right - (40.0 + 80.0 * 2.0 / 3.0)).abs() < 1e-9, "{right}");

    // A later effect upon the visibility supersedes a wipe.
    let superseded = bar(",animate={{effect='wipe_out',duration=2},{effect='appear',start=1}}");
    assert_eq!(operations(&deck(&superseded, Some(1.5))), authored);
}

#[test]
fn a_wipe_cuts_a_line_and_a_moved_and_enlarged_object_in_its_own_coordinates() {
    // The extent of a straight line has no breadth, and its stroke is drawn
    // on both sides of the extent.
    let rule = "#line(from={x=pt(20),y=pt(120)},to={x=pt(180),y=pt(120)},\
                border={color='#0000ff',width=pt(6)},\
                animate={effect='wipe_in',from='right',duration=2})";
    let half = deck(rule, Some(1.0));
    assert_eq!(
        part_in_view(&operations(&half), [20.0, 120.0, 180.0, 120.0]),
        [100.0, 120.0, 180.0, 120.0]
    );
    for (point, color) in [
        ([150, 118], BLUE),
        ([150, 121], BLUE),
        ([50, 118], WHITE),
        ([50, 121], WHITE),
    ] {
        assert_eq!(pixel(&half, point), color, "{point:?}");
    }

    let moving = bar(",animate={{effect='wipe_in',duration=2},\
         {effect='move',by={x=pt(40),y=pt(20)},duration=2},\
         {effect='scale',to=1.5,duration=2}}");
    let half = deck(&moving, Some(1.0));
    let operations = operations(&half);
    let layer = operations
        .iter()
        .position(|operation| matches!(operation, PageOperation::PushLayer { .. }))
        .unwrap();
    assert!(matches!(operations[layer + 1], PageOperation::PushClip(_)));
    let layer_end = operations
        .iter()
        .position(|operation| matches!(operation, PageOperation::PopLayer))
        .unwrap();
    assert!(matches!(operations[layer_end - 1], PageOperation::PopClip));
    let PageOperation::PushLayer { matrix, opacity } = &operations[layer] else {
        unreachable!()
    };
    // The object is displaced by 20 and 10 points and enlarged by a quarter
    // about its displaced centre at 100 and 60 points.
    assert_eq!(
        (*matrix, *opacity),
        ([1.25, 0.0, 0.0, 1.25, -25.0, -15.0], 1.0)
    );
    assert_eq!(
        part_in_view(&operations, [60.0, 40.0, 140.0, 80.0]),
        [60.0, 40.0, 100.0, 80.0]
    );
    // The enlarged object lies between 50 and 150 points across, and the wipe
    // cuts it at its centre.
    for (point, color) in [([55, 60], BLUE), ([95, 60], BLUE), ([105, 60], WHITE)] {
        assert_eq!(pixel(&half, point), color, "{point:?}");
    }

    // A group is wiped as one object over the extent of its members.
    let group = "#group(animate={effect='wipe_in',from='bottom',duration=2})[\
                 #shape(position={x=pt(20),y=pt(20),width=pt(40),height=pt(40)},fill='#0000ff')\
                 #shape(position={x=pt(100),y=pt(60),width=pt(40),height=pt(60)},fill='#0000ff')]";
    let half = deck(group, Some(1.0));
    assert_eq!(pixel(&half, [40, 40]), WHITE);
    assert_eq!(pixel(&half, [120, 65]), WHITE);
    assert_eq!(pixel(&half, [120, 75]), BLUE);
}

#[test]
fn a_wipe_is_written_as_the_wipe_of_powerpoint() {
    let objects = "#shape(name='Left',position={x=pt(10),y=pt(10),width=pt(40),height=pt(20)},fill='#0000ff',\
         animate={effect='wipe_in',duration=1})[Go]\
         #shape(name='Right',position={x=pt(10),y=pt(40),width=pt(40),height=pt(20)},fill='#0000ff',\
         animate={effect='wipe_in',from='right',start=0.5,duration=2,easing='ease_in'})\
         #line(name='Top',from={x=pt(100),y=pt(10)},to={x=pt(100),y=pt(140)},\
         animate={effect='wipe_in',from='top',start=1})\
         #box(name='Bottom',position={x=pt(120),y=pt(10),width=pt(60),height=pt(20)},\
         animate={{effect='wipe_in',from='bottom',start=1.5},\
         {effect='wipe_out',from='top',start=3,duration=1}})[Late]\
         #shape(name='Leaving',position={x=pt(10),y=pt(70),width=pt(40),height=pt(20)},fill='#0000ff',\
         animate={{effect='wipe_out',start=4,duration=0.25},{effect='wipe_out',from='right',start=5},\
         {effect='wipe_out',from='bottom',start=6}})";
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let package = mote::pptx::emit_with_fonts(&deck(objects, None), &resolver, None, None).unwrap();
    assert_eq!(package.warnings, Vec::<String>::new());
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(package.bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("ppt/slides/slide1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    let tree = roxmltree::Document::parse(&xml).unwrap();
    let identifier = |name: &str| {
        tree.descendants()
            .find(|node| node.has_tag_name("cNvPr") && node.attribute("name") == Some(name))
            .unwrap()
            .attribute("id")
            .unwrap()
            .to_owned()
    };
    let [left, right, top, bottom, leaving] =
        ["Left", "Right", "Top", "Bottom", "Leaving"].map(identifier);
    let effects: Vec<_> = tree
        .descendants()
        .filter(|node| node.has_tag_name("cTn") && node.attribute("presetID").is_some())
        .map(|node| {
            let find = |name: &str| {
                node.descendants()
                    .find(|child| child.has_tag_name(name))
                    .unwrap()
            };
            let filter = find("animEffect");
            let visibility = find("set");
            assert_eq!(find("attrName").text(), Some("style.visibility"));
            assert!(
                node.descendants()
                    .filter(|child| child.has_tag_name("spTgt"))
                    .all(|target| target.attribute("spid") == find("spTgt").attribute("spid"))
            );
            (
                find("spTgt").attribute("spid").unwrap().to_owned(),
                [
                    node.attribute("presetID").unwrap(),
                    node.attribute("presetClass").unwrap(),
                    node.attribute("presetSubtype").unwrap(),
                    find("cond").attribute("delay").unwrap(),
                    filter.attribute("transition").unwrap(),
                    filter.attribute("filter").unwrap(),
                    find_duration(filter),
                    find("strVal").attribute("val").unwrap(),
                    visibility
                        .descendants()
                        .find(|child| child.has_tag_name("cond"))
                        .unwrap()
                        .attribute("delay")
                        .unwrap(),
                ],
                node.attribute("accel"),
            )
        })
        .collect();
    let entrance = |subtype, delay, filter, duration| {
        [
            "22", "entr", subtype, delay, "in", filter, duration, "visible", "0",
        ]
    };
    let exit = |subtype, delay, filter, duration, hidden| {
        [
            "22", "exit", subtype, delay, "out", filter, duration, "hidden", hidden,
        ]
    };
    assert_eq!(
        effects,
        [
            (left.clone(), entrance("8", "0", "wipe(left)", "1000"), None),
            (
                right.clone(),
                entrance("2", "500", "wipe(right)", "2000"),
                Some("50000")
            ),
            (top, entrance("1", "1000", "wipe(up)", "500"), None),
            (
                bottom.clone(),
                entrance("4", "1500", "wipe(down)", "500"),
                None
            ),
            (
                bottom.clone(),
                exit("1", "3000", "wipe(up)", "1000", "999"),
                None
            ),
            (
                leaving.clone(),
                exit("8", "4000", "wipe(left)", "250", "249"),
                None
            ),
            (
                leaving.clone(),
                exit("2", "5000", "wipe(right)", "500", "499"),
                None
            ),
            (
                leaving.clone(),
                exit("4", "6000", "wipe(down)", "500", "499"),
                None
            ),
        ]
    );
    // An entrance makes its object visible before the filter plays, and an
    // exit hides its object after.
    for (class, order) in [
        ("entr", ["set", "animEffect"]),
        ("exit", ["animEffect", "set"]),
    ] {
        let effect = tree
            .descendants()
            .find(|node| node.attribute("presetClass") == Some(class))
            .unwrap();
        let behaviors: Vec<_> = effect
            .children()
            .find(|child| child.has_tag_name("childTnLst"))
            .unwrap()
            .children()
            .map(|child| child.tag_name().name())
            .collect();
        assert_eq!(behaviors, order);
    }
    // A shape with a text body has one build for each of its effects, and a
    // line has none.
    let builds: Vec<_> = tree
        .descendants()
        .filter(|node| node.has_tag_name("bldP"))
        .map(|node| {
            (
                node.attribute("spid").unwrap().to_owned(),
                node.attribute("grpId").unwrap(),
            )
        })
        .collect();
    assert_eq!(
        builds,
        [
            (left, "0"),
            (right, "0"),
            (bottom.clone(), "0"),
            (bottom, "1"),
            (leaving.clone(), "0"),
            (leaving.clone(), "1"),
            (leaving, "2"),
        ]
    );
}

fn find_duration<'input>(filter: roxmltree::Node<'input, 'input>) -> &'input str {
    filter
        .descendants()
        .find(|child| child.has_tag_name("cTn"))
        .unwrap()
        .attribute("dur")
        .unwrap()
}

#[test]
fn frames_end_with_the_latest_wipe() {
    let prefix = std::env::temp_dir().join(format!("mote-wipe-{}.frames", std::process::id()));
    let frame =
        |number: usize| std::path::PathBuf::from(format!("{}-{number:04}.png", prefix.display()));
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_mote"))
        .args(["--format", "png", "--frames", "2", "-o"])
        .arg(&prefix)
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let source = format!(
        "#presentation(width=pt(200),height=pt(150))#slide[{}]",
        bar(",animate={effect='wipe_in',start=1,duration=1}")
    );
    child
        .stdin
        .take()
        .unwrap()
        .write_all(source.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    // The frames at 0, 0.5, 1, 1.5 and 2 seconds are written.
    for number in 1..=5 {
        std::fs::remove_file(frame(number)).unwrap();
    }
    assert!(!frame(6).exists());
}
