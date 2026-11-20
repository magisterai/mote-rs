//! The geometry of every shape kind is evaluated from its DrawingML preset
//! definition, of which the library holds a copy.
use std::{fs, path::Path};

/// Returns the lists of a definition that give its geometry, each element by
/// its name, its attributes in order of name, and its children.
fn geometry(definition: roxmltree::Node<'_, '_>) -> Vec<String> {
    fn written(node: roxmltree::Node<'_, '_>) -> String {
        let mut attributes: Vec<_> = node
            .attributes()
            .map(|attribute| format!(" {}={:?}", attribute.name(), attribute.value()))
            .collect();
        attributes.sort();
        let children: String = node
            .children()
            .filter(roxmltree::Node::is_element)
            .map(written)
            .collect();
        format!(
            "<{}{}>{children}</>",
            node.tag_name().name(),
            attributes.concat()
        )
    }
    ["avLst", "gdLst", "cxnLst", "rect", "pathLst"]
        .into_iter()
        // The standard exchanges the top and the right edge of the text
        // rectangle of the pie, which Word's export of the shape-kinds probe
        // shows Office not to do; the library holds the edges in their places.
        .filter(|name| !(*name == "rect" && definition.has_tag_name("pie")))
        .filter_map(|name| {
            definition
                .children()
                .find(|node| node.has_tag_name(name))
                .filter(|node| name == "rect" || node.has_children())
                .map(written)
        })
        .collect()
}

#[test]
fn the_library_holds_the_definitions_of_the_standard_unchanged() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let held = fs::read_to_string(root.join("src/drawings/presets.xml")).unwrap();
    let standard = fs::read_to_string(root.join(
        "docs/reference/ooxml/ecma-376/part-1/drawingml-geometries/presetShapeDefinitions.xml",
    ))
    .unwrap();
    let held = roxmltree::Document::parse(&held).unwrap();
    let standard = roxmltree::Document::parse(standard.trim_start_matches('\u{feff}')).unwrap();
    let mut count = 0;
    for definition in held
        .root_element()
        .children()
        .filter(roxmltree::Node::is_element)
    {
        let name = definition.tag_name().name();
        // The copy of the standard lacks the up arrow, which the library
        // defines as the down arrow turned over.
        if name == "upArrow" {
            continue;
        }
        let original = standard
            .root_element()
            .children()
            .find(|node| node.tag_name().name() == name)
            .unwrap_or_else(|| panic!("the standard defines no {name}"));
        assert_eq!(geometry(definition), geometry(original), "{name}");
        count += 1;
    }
    assert!(count >= 47);
}

fn deck(objects: &str) -> mote::model::Document {
    mote::compile(&format!(
        "#presentation(width = pt(400), height = pt(300))\n#slide[\n{objects}\n]\n"
    ))
    .unwrap()
}

fn planned(
    document: &mote::model::Document,
) -> mote::render::Rendered<mote::layout::DocumentTextPlan> {
    let resolver =
        mote::source::FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    mote::render::plan_with_layout(
        document,
        &resolver,
        None,
        mote::render::LayoutMode::Presentation,
    )
    .unwrap()
}

fn paths(document: &mote::model::Document) -> Vec<mote::layout::PaintedPath> {
    planned(document).output.pages[0]
        .operations
        .iter()
        .filter_map(|operation| match operation {
            mote::layout::PageOperation::Path(path) => Some(path.clone()),
            _ => None,
        })
        .collect()
}

/// Returns the adjustment lists of the shapes of the first slide of a deck.
fn adjustment_lists(document: &mote::model::Document) -> Vec<String> {
    use std::io::Read;
    let package = mote::pptx::emit(document).unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(package)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("ppt/slides/slide1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml.split("<a:prstGeom ")
        .skip(1)
        .map(|rest| rest.split("</a:prstGeom>").next().unwrap().to_owned())
        .collect()
}

#[test]
fn the_arguments_of_a_kind_adjust_its_preset() {
    let document = deck(
        r##"#shape(kind = "rect_callout", fill = "#336699", position = { x = pt(100), y = pt(50), width = pt(200), height = pt(100) }, tail = { x = pt(150), y = pt(250) })
#shape(kind = "pie", position = { x = pt(10), y = pt(10), width = pt(60), height = pt(60) }, angles = { start = 30, end = "300deg" })
#shape(kind = "block_arc", position = { x = pt(10), y = pt(100), width = pt(80), height = pt(60) }, angles = { end = 90 }, thickness = pt(6))
#shape(kind = "circular_arrow", position = { x = pt(10), y = pt(200), width = pt(60), height = pt(60) }, angles = { start = 90, end = 300 })
#shape(kind = "donut", position = { x = pt(320), y = pt(10), width = pt(60), height = pt(40) }, thickness = pt(10))
#shape(kind = "frame", position = { x = pt(320), y = pt(100), width = pt(60), height = pt(40) }, thickness = pt(100))
#shape(kind = "cube", position = { x = pt(320), y = pt(200), width = pt(60), height = pt(60) })"##,
    );
    let guide = |name: &str, value: i64| format!("<a:gd name=\"{name}\" fmla=\"val {value}\"/>");
    assert_eq!(
        adjustment_lists(&document),
        [
            // The tail lies fifty points left of and two hundred below the
            // centre of a frame two hundred points wide and one hundred high.
            format!(
                "prst=\"wedgeRectCallout\"><a:avLst>{}{}</a:avLst>",
                guide("adj1", -25000),
                guide("adj2", 150000)
            ),
            format!(
                "prst=\"pie\"><a:avLst>{}{}</a:avLst>",
                guide("adj1", 1800000),
                guide("adj2", 18000000)
            ),
            // The angle that is not stated keeps the default of the preset.
            format!(
                "prst=\"blockArc\"><a:avLst>{}{}{}</a:avLst>",
                guide("adj1", 10800000),
                guide("adj2", 5400000),
                guide("adj3", 10000)
            ),
            format!(
                "prst=\"circularArrow\"><a:avLst>{}{}{}{}{}</a:avLst>",
                guide("adj1", 12500),
                guide("adj2", 1142319),
                guide("adj3", 18000000),
                guide("adj4", 5400000),
                guide("adj5", 12500)
            ),
            format!("prst=\"donut\"><a:avLst>{}</a:avLst>", guide("adj", 25000)),
            // A thickness is at most half the shorter side.
            format!("prst=\"frame\"><a:avLst>{}</a:avLst>", guide("adj1", 50000)),
            "prst=\"cube\"><a:avLst/>".to_owned(),
        ]
    );
    assert_eq!(planned(&document).warnings, Vec::<String>::new());
    let paths = paths(&document);
    // The outline of the callout reaches its tail.
    assert!(
        paths[0]
            .points
            .iter()
            .any(|point| (point[0] - 150.0).abs() < 1e-6 && (point[1] - 250.0).abs() < 1e-6)
    );
}

#[test]
fn a_definition_fills_its_paths_lighter_and_darker_and_joins_their_contours() {
    let fills = |kind: &str| -> Vec<String> {
        paths(&deck(&format!(
            "#shape(kind = \"{kind}\", fill = \"#336699\", border = {{ color = \"#000000\" }}, position = {{ x = pt(100), y = pt(100), width = pt(120), height = pt(90) }})"
        )))
        .iter()
        .filter_map(|path| path.fill.as_ref().map(|fill| fill.color.clone()))
        .collect()
    };
    // The face of a cube keeps the fill, its side is a fifth nearer black and
    // its top a fifth nearer white; the end of a cylinder is two fifths nearer
    // white.
    assert_eq!(fills("cube"), ["336699", "29527A", "5C85AD"]);
    assert_eq!(fills("cylinder"), ["336699", "85A3C2"]);
    // The ring of a donut is one filled path, which holds both its contours,
    // and each contour is stroked by itself.
    let donut = paths(&deck(
        "#shape(kind = \"donut\", fill = \"#336699\", border = { color = \"#000000\" }, position = { x = pt(100), y = pt(100), width = pt(120), height = pt(90) })",
    ));
    let filled: Vec<_> = donut.iter().filter(|path| path.fill.is_some()).collect();
    let stroked: Vec<_> = donut.iter().filter(|path| path.stroke.is_some()).collect();
    assert_eq!((filled.len(), stroked.len()), (1, 2));
    assert!(filled[0].stroke.is_none());
    assert!(filled[0].points.len() > stroked[0].points.len() + stroked[1].points.len());
}

#[test]
fn an_adjusting_argument_warns_on_a_kind_that_it_does_not_adjust() {
    let document = deck(
        r#"#shape(kind = "rectangle", position = { x = pt(10), y = pt(10), width = pt(60), height = pt(60) }, tail = { x = pt(0), y = pt(0) }, angles = { start = 10, middle = 20 }, thickness = pt(3), corner_radius = pt(3))"#,
    );
    assert_eq!(
        planned(&document).warnings,
        [
            "content[0].slide[0].shape.angles has an effect only on a pie, block_arc or circular_arrow shape",
            "content[0].slide[0].shape.angles.middle is retained without a presentation output effect",
            "content[0].slide[0].shape.corner_radius has an effect only on a round_rect shape",
            "content[0].slide[0].shape.tail has an effect only on a rect_callout, round_rect_callout or ellipse_callout shape",
            "content[0].slide[0].shape.thickness has an effect only on a block_arc, donut or frame shape",
        ]
    );
    let error = mote::compile("#shape(kind = \"pie\", position = { width = pt(60), height = pt(60) }, angles = { start = \"north\" })").unwrap_err();
    assert_eq!(error.message, "angles.start must be a number of degrees");
}
