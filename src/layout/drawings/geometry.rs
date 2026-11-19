use crate::layout::{LineCap, PaintedPath, PathPaint, PathStroke};
use crate::{
    drawings::presets,
    model::{Properties, Unit, Value},
    properties::{self, object, text},
};
use std::f64::consts::PI;

pub(crate) fn points(value: Option<&Value>, fallback: f64) -> f64 {
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
pub(crate) fn rectangle(attributes: &Properties) -> [f64; 4] {
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
pub(crate) fn stroke(attributes: &Properties) -> Option<PathStroke> {
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
        "dashed" => vec![4.0 * width, 3.0 * width],
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
/// Returns the geometry of the preset of a shape within its frame.
pub(crate) fn preset(attributes: &Properties) -> presets::Geometry {
    let bounds = rectangle(attributes);
    presets::geometry(
        &text(attributes, "kind"),
        bounds[2],
        bounds[3],
        &presets::adjustments(
            attributes,
            bounds.map(|length| (length * 12700.0).round() as i64),
        ),
    )
}
pub(crate) fn shape(attributes: &Properties) -> Vec<PaintedPath> {
    let bounds = rectangle(attributes);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return Vec::new();
    }
    let place = |points: &[[f64; 2]]| {
        transform(
            points
                .iter()
                .map(|point| [bounds[0] + point[0], bounds[1] + point[1]])
                .collect(),
            bounds,
            attributes,
        )
    };
    let geometry = preset(attributes);
    let mut paths = geometry
        .paths
        .first()
        .and_then(|path| path.contours.first())
        .map_or_else(Vec::new, |body| shadows(attributes, &place(&body.points)));
    let fill = paint(attributes, "fill", None);
    let stroke = object(attributes, "border").and_then(stroke);
    for path in &geometry.paths {
        let fill = fill
            .as_ref()
            .zip(path.brightness)
            .map(|(fill, brightness)| brightened(fill, brightness));
        let stroke = stroke.clone().filter(|_| path.stroked);
        if let [contour] = &path.contours[..]
            && (contour.closed || fill.is_none())
        {
            if fill.is_some() || stroke.is_some() {
                paths.push(PaintedPath {
                    points: place(&contour.points),
                    closed: contour.closed,
                    fill,
                    stroke,
                });
            }
            continue;
        }
        // The painters fill one polygon, so the contours of a path are joined
        // into one: each returns to the start of the first along the way by
        // which it left, and the two passes enclose nothing.
        if let (Some(fill), Some(first)) = (fill, path.contours.first()) {
            let mut points = Vec::new();
            for contour in &path.contours {
                points.extend(&contour.points);
                points.extend([contour.points[0], first.points[0]]);
            }
            paths.push(PaintedPath {
                points: place(&points),
                closed: true,
                fill: Some(fill),
                stroke: None,
            });
        }
        if stroke.is_some() {
            paths.extend(path.contours.iter().map(|contour| PaintedPath {
                points: place(&contour.points),
                closed: contour.closed,
                fill: None,
                stroke: stroke.clone(),
            }));
        }
    }
    paths
}
/// Returns a paint moved toward white by the share `brightness`, or toward
/// black where the share is negative.
fn brightened(paint: &PathPaint, brightness: f64) -> PathPaint {
    let channel = |index: usize| {
        let value =
            u8::from_str_radix(&paint.color[2 * index..2 * index + 2], 16).map_or(0.0, f64::from);
        let target = if brightness > 0.0 { 255.0 } else { 0.0 };
        (value + (target - value) * brightness.abs()).round() as u8
    };
    PathPaint {
        color: if brightness == 0.0 {
            paint.color.clone()
        } else {
            format!("{:02X}{:02X}{:02X}", channel(0), channel(1), channel(2))
        },
        alpha: paint.alpha,
    }
}
pub(crate) fn transform(
    points: Vec<[f64; 2]>,
    bounds: [f64; 4],
    attributes: &Properties,
) -> Vec<[f64; 2]> {
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

pub(crate) fn text_rectangle(attributes: &Properties) -> [f64; 4] {
    let bounds = rectangle(attributes);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return bounds;
    }
    let [left, top, right, bottom] = preset(attributes).text;
    [
        bounds[0] + left,
        bounds[1] + top,
        (right - left).max(1.0),
        (bottom - top).max(1.0),
    ]
}
