use crate::layout::{LineCap, PaintedPath, PathPaint, PathStroke};
use crate::{
    model::{Properties, Unit, Value},
    properties::{self, object, text},
};
use std::f64::consts::PI;

pub(super) fn points(value: Option<&Value>, fallback: f64) -> f64 {
    let Some(value) = value else { return fallback };
    let units = match value {
        Value::Length { value, unit } => {
            value
                * match unit {
                    Unit::Point => 12700.0,
                    Unit::Inch => 914400.0,
                    Unit::Centimetre => 914400.0 / 2.54,
                    Unit::Millimetre => 914400.0 / 25.4,
                    Unit::Twip => 635.0,
                }
        }
        _ => match value.to_text().trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => return fallback,
        },
    };
    if units.is_finite() {
        units.round() / 12700.0
    } else {
        fallback
    }
}
pub(super) fn rectangle(attributes: &Properties) -> [f64; 4] {
    let empty = Properties::new();
    let position = object(attributes, "position").unwrap_or(&empty);
    [
        points(position.get("x"), 0.0),
        points(position.get("y"), 0.0),
        points(position.get("width"), 288.0),
        points(position.get("height"), 144.0),
    ]
}
fn rotation(attributes: &Properties, name: &str, fallback: f64) -> f64 {
    properties::number(attributes, name)
        .map(|value| (value * 60000.0).round() / 60000.0)
        .unwrap_or(fallback)
}
fn alpha(attributes: &Properties, name: &str, fallback: u8) -> u8 {
    properties::number(attributes, name)
        .map(|value| {
            let percentage = if value <= 1.0 { value * 100.0 } else { value };
            ((percentage.clamp(0.0, 100.0) * 1000.0).round() * 255.0 / 100000.0).round() as u8
        })
        .unwrap_or(fallback)
}
fn paint(attributes: &Properties, name: &str, fallback: Option<&str>) -> Option<PathPaint> {
    let color = super::super::style_values::color(attributes, name)
        .or_else(|| fallback.map(str::to_owned))?;
    Some(PathPaint {
        color,
        alpha: alpha(attributes, "opacity", 255),
    })
}
fn stroke(attributes: &Properties) -> Option<PathStroke> {
    if text(attributes, "style") == "none" {
        return None;
    }
    let width = points(attributes.get("width"), 1.0);
    if width <= 0.0 {
        return None;
    }
    let mut cap = match text(attributes, "cap").as_str() {
        "round" => LineCap::Round,
        "square" => LineCap::Square,
        _ => LineCap::Flat,
    };
    let dash = match text(attributes, "style").as_str() {
        "dashed" => vec![3.0 * width, 3.0 * width],
        "dotted" => {
            cap = LineCap::Round;
            vec![(width / 100.0).max(0.01), 4.0 * width]
        }
        _ => Vec::new(),
    };
    Some(PathStroke {
        paint: paint(attributes, "color", Some("000000"))?,
        width,
        dash,
        cap,
    })
}
pub(super) fn shape(attributes: &Properties) -> Vec<PaintedPath> {
    let bounds = rectangle(attributes);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return Vec::new();
    }
    let kind = text(attributes, "kind");
    let points = transform(preset(&kind, bounds), bounds, attributes);
    let mut paths = shadows(attributes, &points);
    let fill = paint(attributes, "fill", None);
    let stroke = object(attributes, "border").and_then(stroke);
    if fill.is_some() || stroke.is_some() {
        paths.push(PaintedPath {
            points,
            closed: true,
            fill,
            stroke: stroke.clone(),
        });
    }
    if kind == "cloud" && stroke.is_some() {
        for path in cloud_decorations(bounds) {
            paths.push(PaintedPath {
                points: transform(path, bounds, attributes),
                closed: false,
                fill: None,
                stroke: stroke.clone(),
            });
        }
    }
    paths
}
fn transform(points: Vec<[f64; 2]>, bounds: [f64; 4], attributes: &Properties) -> Vec<[f64; 2]> {
    let center = [bounds[0] + bounds[2] / 2.0, bounds[1] + bounds[3] / 2.0];
    let (sine, cosine) = (rotation(attributes, "rotation", 0.0) * PI / 180.0).sin_cos();
    points
        .into_iter()
        .map(|point| {
            let horizontal = (point[0] - center[0])
                * if properties::boolean(attributes, "flip_horizontal") {
                    -1.0
                } else {
                    1.0
                };
            let vertical = (point[1] - center[1])
                * if properties::boolean(attributes, "flip_vertical") {
                    -1.0
                } else {
                    1.0
                };
            [
                center[0] + horizontal * cosine - vertical * sine,
                center[1] + horizontal * sine + vertical * cosine,
            ]
        })
        .collect()
}
fn preset(kind: &str, bounds: [f64; 4]) -> Vec<[f64; 2]> {
    let width = bounds[2];
    let height = bounds[3];
    let inset = width.min(height) * 0.25 / width;
    let normalized = match kind {
        "round_rect" => {
            let corner = width.min(height) * 16667.0 / 100000.0;
            let horizontal_radius = corner / width;
            let vertical_radius = corner / height;
            let mut points = Vec::new();
            for (center_horizontal, center_vertical, start) in [
                (1.0 - horizontal_radius, vertical_radius, -PI / 2.0),
                (1.0 - horizontal_radius, 1.0 - vertical_radius, 0.0),
                (horizontal_radius, 1.0 - vertical_radius, PI / 2.0),
                (horizontal_radius, vertical_radius, PI),
            ] {
                for index in 0..=8 {
                    let angle = start + f64::from(index) * PI / 16.0;
                    points.push([
                        center_horizontal + horizontal_radius * angle.cos(),
                        center_vertical + vertical_radius * angle.sin(),
                    ]);
                }
            }
            points
        }
        "ellipse" => (0..64)
            .map(|index| {
                let angle = 2.0 * PI * f64::from(index) / 64.0;
                [0.5 + 0.5 * angle.cos(), 0.5 + 0.5 * angle.sin()]
            })
            .collect(),
        "triangle" => vec![[0.5, 0.0], [1.0, 1.0], [0.0, 1.0]],
        "diamond" => vec![[0.5, 0.0], [1.0, 0.5], [0.5, 1.0], [0.0, 0.5]],
        "hexagon" => vec![
            [inset, 0.0],
            [1.0 - inset, 0.0],
            [1.0, 0.5],
            [1.0 - inset, 1.0],
            [inset, 1.0],
            [0.0, 0.5],
        ],
        "chevron" => vec![
            [0.0, 0.0],
            [1.0 - 2.0 * inset, 0.0],
            [1.0, 0.5],
            [1.0 - 2.0 * inset, 1.0],
            [0.0, 1.0],
            [2.0 * inset, 0.5],
        ],
        "parallelogram" => vec![[inset, 0.0], [1.0, 0.0], [1.0 - inset, 1.0], [0.0, 1.0]],
        "trapezoid" => vec![[inset, 0.0], [1.0 - inset, 0.0], [1.0, 1.0], [0.0, 1.0]],
        "cloud" => arc_path(
            [3900.0, 14370.0],
            &[
                [6753.0, 9190.0, -11429249.0, 7426832.0],
                [5333.0, 7267.0, -8646143.0, 5396714.0],
                [4365.0, 5945.0, -8748475.0, 5983381.0],
                [4857.0, 6595.0, -7859164.0, 7034504.0],
                [5333.0, 7273.0, -4722533.0, 6541615.0],
                [6775.0, 9220.0, -2776035.0, 7816140.0],
                [5785.0, 7867.0, 37501.0, 6842000.0],
                [6752.0, 9215.0, 1347096.0, 6910353.0],
                [7720.0, 10543.0, 3974558.0, 4542661.0],
                [4360.0, 5918.0, -16496525.0, 8804134.0],
                [4345.0, 5945.0, -14809710.0, 9151131.0],
            ],
        ),
        _ => vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    };
    normalized
        .into_iter()
        .map(|point| [bounds[0] + point[0] * width, bounds[1] + point[1] * height])
        .collect()
}
fn cloud_decorations(bounds: [f64; 4]) -> Vec<Vec<[f64; 2]>> {
    [
        ([4693.0, 26177.0], [4345.0, 5945.0, 5204520.0, 1585770.0]),
        ([6928.0, 34899.0], [4360.0, 5918.0, 4416628.0, 686848.0]),
        ([16478.0, 39090.0], [6752.0, 9215.0, 8257449.0, 844866.0]),
        ([28827.0, 34751.0], [6752.0, 9215.0, 387196.0, 959901.0]),
        ([34129.0, 22954.0], [5785.0, 7867.0, -4217541.0, 4255042.0]),
        ([41798.0, 15354.0], [5333.0, 7273.0, 1819082.0, 1665090.0]),
        ([38324.0, 5426.0], [4857.0, 6595.0, -824660.0, 891534.0]),
        ([29078.0, 3952.0], [4857.0, 6595.0, -8950887.0, 1091722.0]),
        ([22141.0, 4720.0], [4365.0, 5945.0, -9809656.0, 1061181.0]),
        ([14000.0, 5192.0], [6753.0, 9190.0, -4002417.0, 739161.0]),
        ([4127.0, 15789.0], [6753.0, 9190.0, 9459261.0, 711490.0]),
    ]
    .into_iter()
    .map(|(start, arc)| {
        arc_path(start, &[arc])
            .into_iter()
            .map(|point| {
                [
                    bounds[0] + point[0] * bounds[2],
                    bounds[1] + point[1] * bounds[3],
                ]
            })
            .collect()
    })
    .collect()
}
fn arc_path(start: [f64; 2], arcs: &[[f64; 4]]) -> Vec<[f64; 2]> {
    let mut current = start;
    let mut points = vec![start.map(|value| value / 43200.0)];
    for &[horizontal_radius, vertical_radius, start, sweep] in arcs {
        let start = start / 60000.0 * PI / 180.0;
        let sweep = sweep / 60000.0 * PI / 180.0;
        let polar = |angle: f64| {
            let cosine = angle.cos();
            let sine = angle.sin();
            let radius = horizontal_radius * vertical_radius
                / (vertical_radius * cosine).hypot(horizontal_radius * sine);
            [radius * cosine, radius * sine]
        };
        let offset = polar(start);
        let center = [current[0] - offset[0], current[1] - offset[1]];
        let segments = (sweep.abs() / (PI / 24.0)).ceil().max(2.0) as usize;
        for index in 1..=segments {
            let offset = polar(start + sweep * index as f64 / segments as f64);
            current = [center[0] + offset[0], center[1] + offset[1]];
            points.push(current.map(|value| value / 43200.0));
        }
    }
    points
}
fn shadows(attributes: &Properties, points: &[[f64; 2]]) -> Vec<PaintedPath> {
    let Some(shadow) = object(attributes, "shadow") else {
        return Vec::new();
    };
    let mut fill = paint(shadow, "color", Some("000000")).unwrap();
    fill.alpha = alpha(shadow, "opacity", 89);
    if fill.alpha == 0 {
        return Vec::new();
    }
    let distance = self::points(shadow.get("distance"), 2.0);
    let angle = rotation(shadow, "angle", 270.0) * PI / 180.0;
    let offset = [distance * angle.cos(), distance * angle.sin()];
    let blur = self::points(shadow.get("blur"), 4.0).max(0.0);
    let layers = if blur == 0.0 {
        1
    } else {
        (blur * 2.0).ceil().clamp(2.0, 12.0) as usize
    };
    if blur > 0.0 {
        fill.alpha = ((1.0 - (1.0 - f64::from(fill.alpha) / 255.0).powf(1.0 / layers as f64))
            * 255.0)
            .round()
            .max(1.0) as u8;
    }
    let minimum = [
        points
            .iter()
            .map(|point| point[0])
            .fold(f64::INFINITY, f64::min),
        points
            .iter()
            .map(|point| point[1])
            .fold(f64::INFINITY, f64::min),
    ];
    let maximum = [
        points
            .iter()
            .map(|point| point[0])
            .fold(f64::NEG_INFINITY, f64::max),
        points
            .iter()
            .map(|point| point[1])
            .fold(f64::NEG_INFINITY, f64::max),
    ];
    (1..=layers)
        .rev()
        .map(|layer| {
            let expansion = blur * layer as f64 / layers as f64;
            PaintedPath {
                points: points
                    .iter()
                    .map(|point| {
                        std::array::from_fn(|axis| {
                            let center = (minimum[axis] + maximum[axis]) / 2.0;
                            let extent = maximum[axis] - minimum[axis];
                            let scale = if extent > 0.0 {
                                1.0 + 2.0 * expansion / extent
                            } else {
                                1.0
                            };
                            center + (point[axis] - center) * scale + offset[axis]
                        })
                    })
                    .collect(),
                closed: true,
                fill: Some(fill.clone()),
                stroke: None,
            }
        })
        .collect()
}
