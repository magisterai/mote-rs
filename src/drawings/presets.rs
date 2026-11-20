//! Preset shape geometry. The outline, the text rectangle and the connection
//! sites of each shape kind follow from its definition in ECMA-376
//! presetShapeDefinitions.xml. The file `presets.xml` holds the definitions of
//! the kinds that Mote supports, without their adjust handles.
use crate::{
    model::Properties,
    properties::{number, object, text},
};
use std::{collections::HashMap, f64::consts::PI, sync::OnceLock};

/// The shape kinds, each with the DrawingML preset that defines it.
pub(crate) const KINDS: &[(&str, &str)] = &[
    ("rectangle", "rect"),
    ("round_rect", "roundRect"),
    ("ellipse", "ellipse"),
    ("triangle", "triangle"),
    ("diamond", "diamond"),
    ("hexagon", "hexagon"),
    ("chevron", "chevron"),
    ("cloud", "cloud"),
    ("parallelogram", "parallelogram"),
    ("trapezoid", "trapezoid"),
    ("terminator", "flowChartTerminator"),
    ("document", "flowChartDocument"),
    ("predefined_process", "flowChartPredefinedProcess"),
    ("manual_input", "flowChartManualInput"),
    ("preparation", "flowChartPreparation"),
    ("delay", "flowChartDelay"),
    ("off_page_connector", "flowChartOffpageConnector"),
    ("stored_data", "flowChartOnlineStorage"),
    ("cylinder", "can"),
    ("cube", "cube"),
    ("pentagon", "pentagon"),
    ("octagon", "octagon"),
    ("right_triangle", "rtTriangle"),
    ("plus", "plus"),
    ("right_arrow", "rightArrow"),
    ("left_arrow", "leftArrow"),
    ("up_arrow", "upArrow"),
    ("down_arrow", "downArrow"),
    ("left_right_arrow", "leftRightArrow"),
    ("notched_right_arrow", "notchedRightArrow"),
    ("home_plate", "homePlate"),
    ("circular_arrow", "circularArrow"),
    ("rect_callout", "wedgeRectCallout"),
    ("round_rect_callout", "wedgeRoundRectCallout"),
    ("ellipse_callout", "wedgeEllipseCallout"),
    ("snip_rect", "snip1Rect"),
    ("round_top_rect", "round2SameRect"),
    ("folded_corner", "foldedCorner"),
    ("frame", "frame"),
    ("pie", "pie"),
    ("donut", "donut"),
    ("block_arc", "blockArc"),
    ("left_brace", "leftBrace"),
    ("right_brace", "rightBrace"),
    ("star4", "star4"),
    ("star5", "star5"),
    ("heart", "heart"),
    ("lightning_bolt", "lightningBolt"),
];

/// The arguments that adjust a preset: the kind, the argument, and the guides
/// of the definition that the argument sets.
const ADJUSTMENTS: &[(&str, &str, &[&str])] = &[
    ("round_rect", "corner_radius", &["adj"]),
    ("rect_callout", "tail", &["adj1", "adj2"]),
    ("round_rect_callout", "tail", &["adj1", "adj2"]),
    ("ellipse_callout", "tail", &["adj1", "adj2"]),
    ("pie", "angles", &["adj1", "adj2"]),
    ("block_arc", "angles", &["adj1", "adj2"]),
    ("block_arc", "thickness", &["adj3"]),
    ("circular_arrow", "angles", &["adj4", "adj3"]),
    ("donut", "thickness", &["adj"]),
    ("frame", "thickness", &["adj1"]),
];

/// The arguments of a shape that adjust its preset.
pub(crate) const ADJUSTING: [&str; 4] = ["corner_radius", "tail", "angles", "thickness"];

/// The fields of the adjusting arguments that are tables.
pub(crate) const ADJUSTING_FIELDS: [(&str, &[&str]); 2] =
    [("tail", &["x", "y"]), ("angles", &["start", "end"])];

/// Returns a warning for each adjusting argument that a shape states and that
/// has no effect on its kind.
pub(crate) fn idle_arguments(attributes: &Properties) -> Vec<String> {
    let kind = text(attributes, "kind");
    ADJUSTING
        .into_iter()
        .filter(|argument| attributes.contains_key(*argument))
        .filter_map(|argument| {
            let kinds: Vec<&str> = ADJUSTMENTS
                .iter()
                .filter(|(_, name, _)| *name == argument)
                .map(|(kind, ..)| *kind)
                .collect();
            let (last, rest) = kinds.split_last()?;
            let list = if rest.is_empty() {
                (*last).to_owned()
            } else {
                format!("{} or {last}", rest.join(", "))
            };
            (!kinds.contains(&kind.as_str()))
                .then(|| format!("{argument} has an effect only on a {list} shape"))
        })
        .collect()
}

/// The geometry of a shape within its frame, whose corner is the origin.
pub(crate) struct Geometry {
    /// The paths in the order in which they are drawn.
    pub paths: Vec<Path>,
    pub sites: Vec<Site>,
    /// The left, top, right and bottom edges of the text rectangle.
    pub text: [f64; 4],
}

pub(crate) struct Path {
    pub contours: Vec<Contour>,
    /// The share by which the path moves the fill of the shape toward white,
    /// or toward black where it is negative; a path without a fill has none.
    pub brightness: Option<f64>,
    pub stroked: bool,
}

pub(crate) struct Contour {
    pub points: Vec<[f64; 2]>,
    pub closed: bool,
}

/// A connection site: the direction in which it faces, in degrees clockwise
/// from the right, and its place.
pub(crate) struct Site {
    pub facing: f64,
    pub point: [f64; 2],
}

/// Returns the name of the DrawingML preset of a shape kind.
pub(crate) fn preset(kind: &str) -> &'static str {
    KINDS
        .iter()
        .find(|(name, _)| *name == kind)
        .map_or("rect", |(_, preset)| preset)
}

/// Returns the definition of the preset of a shape kind.
fn definition(kind: &str) -> roxmltree::Node<'static, 'static> {
    static DEFINITIONS: OnceLock<roxmltree::Document<'static>> = OnceLock::new();
    DEFINITIONS
        .get_or_init(|| {
            roxmltree::Document::parse(include_str!("presets.xml"))
                .expect("the preset definitions are well formed")
        })
        .root_element()
        .children()
        .find(|node| node.has_tag_name(preset(kind)))
        .expect("every kind has a definition")
}

/// Returns the elements of the list `name` of a definition.
fn list(
    definition: roxmltree::Node<'static, 'static>,
    name: &'static str,
) -> impl Iterator<Item = roxmltree::Node<'static, 'static>> {
    definition
        .children()
        .find(|node| node.has_tag_name(name))
        .into_iter()
        .flat_map(|node| node.children().filter(roxmltree::Node::is_element))
}

/// Returns the adjustments of the preset of a shape, each with the name of its
/// guide, where the attributes of the shape state any; the defaults of the
/// definition then accompany those stated, as Office writes them together.
/// The size `size` of the frame is in drawing units.
pub(crate) fn adjustments(attributes: &Properties, size: [i64; 2]) -> Vec<(&'static str, i64)> {
    let kind = text(attributes, "kind");
    let length = super::length_in_drawing_units;
    let position = object(attributes, "position");
    let mut stated: Vec<(&str, i64)> = Vec::new();
    for (_, argument, guides) in ADJUSTMENTS.iter().filter(|(owner, ..)| *owner == kind) {
        let fields = object(attributes, argument);
        let values = match *argument {
            // The tail of a callout is stated from the origin of the position
            // of the shape, and held from the centre of the frame in hundred
            // thousandths of its width and its height.
            "tail" => [0, 1].map(|axis| {
                let name = ["x", "y"][axis];
                let coordinate = length(fields?.get(name))? as f64;
                let corner = length(position.and_then(|position| position.get(name)));
                let extent = size[axis].max(1) as f64;
                let centre = corner.unwrap_or(0) as f64 + extent / 2.0;
                Some(((coordinate - centre) * 100000.0 / extent).round() as i64)
            }),
            // An angle is stated in degrees and held in sixty thousandths of one.
            "angles" => ["start", "end"].map(|name| {
                let degrees = number(fields?, name)?.rem_euclid(360.0);
                Some(((degrees * 60000.0).round() as i64).min(21599999))
            }),
            // A corner radius or a thickness is a share of the shorter side,
            // in hundred thousandths and at most a half.
            _ => [
                length(attributes.get(*argument)).map(|length| {
                    let shorter = size[0].min(size[1]).max(1);
                    (length as f64 * 100000.0 / shorter as f64)
                        .round()
                        .clamp(0.0, 50000.0) as i64
                }),
                None,
            ],
        };
        stated.extend(
            guides
                .iter()
                .zip(values)
                .filter_map(|(guide, value)| Some((*guide, value?))),
        );
    }
    if stated.is_empty() {
        return Vec::new();
    }
    list(definition(&kind), "avLst")
        .filter_map(|guide| {
            let name = guide.attribute("name")?;
            let default = guide
                .attribute("fmla")?
                .strip_prefix("val ")?
                .parse()
                .ok()?;
            let value = stated
                .iter()
                .find(|(guide, _)| *guide == name)
                .map_or(default, |(_, value)| *value);
            Some((name, value))
        })
        .collect()
}

/// Returns the geometry of a shape of the kind `kind` and of the size `width`
/// by `height`, with the adjustments `adjustments`.
pub(crate) fn geometry(
    kind: &str,
    width: f64,
    height: f64,
    adjustments: &[(&'static str, i64)],
) -> Geometry {
    let definition = definition(kind);
    let list = |name| list(definition, name);
    let shorter = width.min(height);
    let mut values: HashMap<String, f64> = [
        ("w", width),
        ("h", height),
        ("l", 0.0),
        ("t", 0.0),
        ("r", width),
        ("b", height),
        ("hc", width / 2.0),
        ("vc", height / 2.0),
        ("ss", shorter),
        ("ls", width.max(height)),
        ("cd8", 2700000.0),
        ("cd4", 5400000.0),
        ("3cd8", 8100000.0),
        ("cd2", 10800000.0),
        ("5cd8", 13500000.0),
        ("3cd4", 16200000.0),
        ("7cd8", 18900000.0),
    ]
    .into_iter()
    .map(|(name, value)| (name.to_owned(), value))
    .collect();
    for divisor in [2, 3, 4, 5, 6, 8, 10, 12, 32] {
        values.insert(format!("wd{divisor}"), width / f64::from(divisor));
        values.insert(format!("hd{divisor}"), height / f64::from(divisor));
    }
    for divisor in [2, 4, 6, 8, 16, 32] {
        values.insert(format!("ssd{divisor}"), shorter / f64::from(divisor));
    }
    let define = |values: &mut HashMap<String, f64>, guide: roxmltree::Node<'_, '_>| {
        let value = formula(guide.attribute("fmla").unwrap_or_default(), values);
        values.insert(
            guide.attribute("name").unwrap_or_default().to_owned(),
            value,
        );
    };
    for guide in list("avLst") {
        define(&mut values, guide);
    }
    for (name, value) in adjustments {
        values.insert((*name).to_owned(), *value as f64);
    }
    for guide in list("gdLst") {
        define(&mut values, guide);
    }
    let value = |node: roxmltree::Node<'_, '_>, name: &str| {
        operand(node.attribute(name).unwrap_or_default(), &values)
    };
    let paths = list("pathLst")
        .map(|path| {
            let point = |node: roxmltree::Node<'_, '_>| [value(node, "x"), value(node, "y")];
            let mut contours: Vec<Contour> = Vec::new();
            for command in path.children().filter(roxmltree::Node::is_element) {
                let mut targets = command
                    .children()
                    .filter(roxmltree::Node::is_element)
                    .map(point);
                if command.has_tag_name("moveTo") {
                    contours.push(Contour {
                        points: targets.collect(),
                        closed: false,
                    });
                    continue;
                }
                let Some(contour) = contours.last_mut() else {
                    continue;
                };
                match command.tag_name().name() {
                    "lnTo" => contour.points.extend(targets),
                    "arcTo" => arc(
                        &mut contour.points,
                        [value(command, "wR"), value(command, "hR")],
                        radians(value(command, "stAng")),
                        radians(value(command, "swAng")),
                    ),
                    "cubicBezTo" => {
                        if let (Some(first), Some(second), Some(end)) =
                            (targets.next(), targets.next(), targets.next())
                        {
                            cubic(&mut contour.points, [first, second, end]);
                        }
                    }
                    _ => contour.closed = true,
                }
            }
            // A path may state the extent of its own coordinates, which then
            // span the frame; its arcs are circular in those coordinates.
            let scale = [("w", width), ("h", height)].map(|(name, extent)| {
                path.attribute(name)
                    .and_then(|unit| unit.parse::<f64>().ok())
                    .map_or(1.0, |unit| extent / unit)
            });
            for point in contours.iter_mut().flat_map(|contour| &mut contour.points) {
                *point = [point[0] * scale[0], point[1] * scale[1]];
            }
            Path {
                contours,
                brightness: match path.attribute("fill") {
                    Some("none") => None,
                    Some("lighten") => Some(0.4),
                    Some("lightenLess") => Some(0.2),
                    Some("darken") => Some(-0.4),
                    Some("darkenLess") => Some(-0.2),
                    _ => Some(0.0),
                },
                stroked: path.attribute("stroke") != Some("false"),
            }
        })
        .collect();
    let sites = list("cxnLst")
        .filter_map(|site| {
            let position = site.first_element_child()?;
            Some(Site {
                facing: (value(site, "ang") / 60000.0).rem_euclid(360.0),
                point: [value(position, "x"), value(position, "y")],
            })
        })
        .collect();
    let text = definition
        .children()
        .find(|node| node.has_tag_name("rect"))
        .map_or([0.0, 0.0, width, height], |rectangle| {
            ["l", "t", "r", "b"].map(|edge| value(rectangle, edge))
        });
    Geometry { paths, sites, text }
}

/// Converts an angle of a definition, in sixty thousandths of a degree.
fn radians(angle: f64) -> f64 {
    angle / 60000.0 * PI / 180.0
}

fn operand(token: &str, values: &HashMap<String, f64>) -> f64 {
    token
        .parse()
        .unwrap_or_else(|_| values.get(token).copied().unwrap_or_default())
}

/// Evaluates a guide formula: an operator and at most three operands, each a
/// number or the name of a guide.
fn formula(text: &str, values: &HashMap<String, f64>) -> f64 {
    let mut tokens = text.split_whitespace();
    let operator = tokens.next().unwrap_or_default();
    let [x, y, z] =
        std::array::from_fn(|_| tokens.next().map_or(0.0, |token| operand(token, values)));
    match operator {
        "*/" => x * y / z,
        "+-" => x + y - z,
        "+/" => (x + y) / z,
        "?:" => {
            if x > 0.0 {
                y
            } else {
                z
            }
        }
        "abs" => x.abs(),
        "at2" => y.atan2(x) * 180.0 / PI * 60000.0,
        "cat2" => x * z.atan2(y).cos(),
        "sat2" => x * z.atan2(y).sin(),
        "cos" => x * radians(y).cos(),
        "sin" => x * radians(y).sin(),
        "tan" => x * radians(y).tan(),
        "max" => x.max(y),
        "min" => x.min(y),
        "mod" => (x * x + y * y + z * z).sqrt(),
        "pin" => y.max(x).min(z),
        "sqrt" => x.sqrt(),
        _ => x,
    }
}

/// Appends an arc of an ellipse with the radii `radii` that begins at the last
/// point. An angle of a definition is that of the ray from the centre to the
/// point of the ellipse, which the parameter of the ellipse must render. The
/// arc is divided evenly by that parameter, so that a flat ellipse keeps its
/// curve.
fn arc(points: &mut Vec<[f64; 2]>, radii: [f64; 2], start: f64, sweep: f64) {
    let parameter = |angle: f64| (radii[0] * angle.sin()).atan2(radii[1] * angle.cos());
    let Some(&current) = points.last() else {
        return;
    };
    let first = parameter(start);
    // The parameter lies in the quadrant of its angle, so its sweep is the
    // turn nearest the sweep of the angle.
    let turn = sweep + (parameter(start + sweep) - first - sweep + PI).rem_euclid(2.0 * PI) - PI;
    let center = [
        current[0] - radii[0] * first.cos(),
        current[1] - radii[1] * first.sin(),
    ];
    let segments = (turn.abs() / (PI / 32.0)).ceil().max(2.0) as usize;
    for index in 1..=segments {
        let parameter = first + turn * index as f64 / segments as f64;
        points.push([
            center[0] + radii[0] * parameter.cos(),
            center[1] + radii[1] * parameter.sin(),
        ]);
    }
}

/// Appends a cubic Bézier curve that begins at the last point.
fn cubic(points: &mut Vec<[f64; 2]>, [first, second, end]: [[f64; 2]; 3]) {
    const SEGMENTS: usize = 24;
    let Some(&start) = points.last() else {
        return;
    };
    for index in 1..=SEGMENTS {
        let along = index as f64 / SEGMENTS as f64;
        let before = 1.0 - along;
        points.push(std::array::from_fn(|axis| {
            before.powi(3) * start[axis]
                + 3.0 * before.powi(2) * along * first[axis]
                + 3.0 * before * along.powi(2) * second[axis]
                + along.powi(3) * end[axis]
        }));
    }
}
