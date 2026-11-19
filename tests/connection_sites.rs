//! A line whose end names a shape meets that shape at a connection site, the
//! point that the `cxnLst` of its preset definition gives and to which
//! PowerPoint attaches a connector.
use mote::{layout::PageOperation, render, source::FileAssetResolver};
use std::path::Path;

const KINDS: [&str; 10] = [
    "rectangle",
    "round_rect",
    "ellipse",
    "triangle",
    "diamond",
    "hexagon",
    "chevron",
    "cloud",
    "parallelogram",
    "trapezoid",
];

fn deck(objects: &str) -> String {
    format!("#presentation(size = \"wide\")\n#slide[\n{objects}\n]\n")
}

/// Returns the outline that native output draws for the one filled shape of
/// the slide, and the start of each line.
fn outline_and_starts(source: &str) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let plan =
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    let mut outline = Vec::new();
    let mut starts = Vec::new();
    for operation in &plan.output.pages[0].operations {
        let PageOperation::Path(path) = operation else {
            continue;
        };
        if path.closed {
            outline = path.points.clone();
        } else {
            starts.push(path.points[0]);
        }
    }
    (outline, starts)
}

/// Returns the distance from `point` to the nearest edge of the closed
/// polygon `outline`.
fn distance(point: [f64; 2], outline: &[[f64; 2]]) -> f64 {
    (0..outline.len())
        .map(|index| {
            let start = outline[index];
            let end = outline[(index + 1) % outline.len()];
            let edge = [end[0] - start[0], end[1] - start[1]];
            let length = edge[0] * edge[0] + edge[1] * edge[1];
            let along = if length == 0.0 {
                0.0
            } else {
                (((point[0] - start[0]) * edge[0] + (point[1] - start[1]) * edge[1]) / length)
                    .clamp(0.0, 1.0)
            };
            (point[0] - start[0] - along * edge[0]).hypot(point[1] - start[1] - along * edge[1])
        })
        .fold(f64::INFINITY, f64::min)
}

#[test]
fn every_site_of_every_kind_lies_on_the_outline_that_native_output_draws() {
    for transform in ["", "rotation = 30, flip_horizontal = true,"] {
        for kind in KINDS {
            let mut objects = format!(
                "#shape(name = \"Target\", kind = \"{kind}\", {transform} fill = \"#dce6f1\", position = {{ x = inch(3), y = inch(2), width = inch(1.8), height = inch(1.1) }})\n"
            );
            let ends: Vec<String> = ["top", "bottom", "left", "right"]
                .map(|side| format!("side = \"{side}\""))
                .into_iter()
                .chain((0..4).map(|site| format!("site = {site}")))
                .collect();
            for end in &ends {
                objects.push_str(&format!(
                    "#line(from = {{ shape = \"Target\", {end} }}, to = {{ x = inch(0.5), y = inch(0.5) }})\n"
                ));
            }
            let (outline, starts) = outline_and_starts(&deck(&objects));
            assert_eq!(starts.len(), ends.len(), "{kind}");
            for (end, start) in ends.iter().zip(starts) {
                let distance = distance(start, &outline);
                assert!(distance < 0.5, "{kind} {transform} {end}: {distance}");
            }
        }
    }
}

#[test]
fn a_side_takes_the_site_that_faces_it_nearest_its_middle() {
    let source = deck(
        r#"#shape(name = "Input", kind = "parallelogram", position = { x = inch(1), y = inch(2), width = inch(1.8), height = inch(0.8) })
#shape(name = "Ship", kind = "chevron", position = { x = inch(4), y = inch(2), width = inch(1.8), height = inch(0.8) })
#line(from = { shape = "Input", side = "right" }, to = { shape = "Ship", side = "left" }, arrow = "triangle")"#,
    );
    let document = mote::compile(&source).unwrap();
    let json: serde_json::Value =
        serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
    let line = &json["content"][0]["children"][2]["attributes"];
    // The right site of the parallelogram lies an eighth of its height within
    // the frame, and the left site of the chevron at its notch.
    assert_eq!(line["from"]["x"]["text"], "194.4pt");
    assert_eq!(line["from"]["y"]["text"], "172.8pt");
    assert_eq!(line["from"]["site"], 2.0);
    assert_eq!(line["to"]["x"]["text"], "316.8pt");
    assert_eq!(line["to"]["site"], 1.0);
}

#[test]
fn a_pptx_connector_records_the_shapes_and_sites_that_it_joins() {
    let source = deck(
        r#"#line(from = { shape = "First", side = "bottom" }, to = { shape = "Third", site = 3 })
#shape(name = "First", kind = "hexagon", position = { x = inch(1), y = inch(1), width = inch(2), height = inch(1) })
#shape(name = "Third", kind = "ellipse", position = { x = inch(7), y = inch(1), width = inch(2), height = inch(1) })
#group[
  #shape(name = "Second", kind = "ellipse", position = { x = inch(4), y = inch(3), width = inch(2), height = inch(1) })
  #line(from = { shape = "Second", side = "top" }, to = { x = inch(6), y = inch(1) })
]"#,
    );
    let document = mote::compile(&source).unwrap();
    let resolver = FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let package = mote::pptx::emit_with_fonts(&document, &resolver, None, None).unwrap();
    assert_eq!(package.warnings, Vec::<String>::new());
    let xml = slide_xml(package.bytes);
    let tree = roxmltree::Document::parse(&xml).unwrap();
    let identifier = |name: &str| {
        tree.descendants()
            .find(|node| node.has_tag_name("cNvPr") && node.attribute("name") == Some(name))
            .unwrap()
            .attribute("id")
            .unwrap()
            .to_owned()
    };
    let connections: Vec<_> = tree
        .descendants()
        .filter(|node| node.has_tag_name("stCxn") || node.has_tag_name("endCxn"))
        .map(|node| {
            (
                node.tag_name().name().to_owned(),
                node.attribute("id").unwrap().to_owned(),
                node.attribute("idx").unwrap().to_owned(),
            )
        })
        .collect();
    let connection =
        |kind: &str, name: &str, site: &str| (kind.to_owned(), identifier(name), site.to_owned());
    // The bottom of a hexagon has two sites, the right of which comes first.
    assert_eq!(
        connections,
        [
            connection("stCxn", "First", "1"),
            connection("endCxn", "Third", "3"),
            connection("stCxn", "Second", "0"),
        ]
    );
}

#[test]
fn a_line_names_one_shape_of_its_own_slide_or_group() {
    let shape = r#"#shape(name = "Box", position = { x = inch(1), y = inch(1), width = inch(1), height = inch(1) })"#;
    for (objects, message) in [
        (
            format!(
                r#"{shape} #line(from = {{ shape = "Other", side = "top" }}, to = {{ x = inch(3), y = inch(3) }})"#
            ),
            "line.from names shape \"Other\", which its slide or group does not hold",
        ),
        (
            format!(
                r#"{shape} {shape} #line(from = {{ x = inch(3), y = inch(3) }}, to = {{ shape = "Box", side = "top" }})"#
            ),
            "line.to names shape \"Box\", which more than one shape bears",
        ),
        (
            format!(
                r#"#group[{shape}] #line(from = {{ shape = "Box", side = "top" }}, to = {{ x = inch(3), y = inch(3) }})"#
            ),
            "line.from names shape \"Box\", which its slide or group does not hold",
        ),
        (
            format!(
                r#"{shape} #line(from = {{ shape = "Box", site = 4 }}, to = {{ x = inch(3), y = inch(3) }})"#
            ),
            "line.from: site 4 does not exist; the shape has sites 0 to 3",
        ),
        (
            format!(
                r#"{shape} #line(from = {{ shape = "Box" }}, to = {{ x = inch(3), y = inch(3) }})"#
            ),
            "line.from requires a side or a site of its shape",
        ),
        (
            format!(
                r#"{shape} #line(from = {{ shape = "Box", side = "top", x = inch(1) }}, to = {{ x = inch(3), y = inch(3) }})"#
            ),
            "line.from takes a shape or x and y, not both",
        ),
    ] {
        let error = mote::compile(&deck(&objects)).unwrap_err();
        assert_eq!(error.message, message);
    }
}

fn slide_xml(bytes: Vec<u8>) -> String {
    use std::io::{Cursor, Read};
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("ppt/slides/slide1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}
