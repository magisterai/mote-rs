//! Shared editable shape geometry, fills, shadows and connector strokes.
use crate::{
    model::Properties,
    properties::{boolean, number, object, text},
};
pub(crate) fn shape(attributes: &Properties, bounds: [i64; 4]) -> Result<String, String> {
    let mut transform = String::new();
    if let Some(rotation) = number(attributes, "rotation") {
        transform.push_str(&format!(" rot=\"{}\"", angle(rotation)?));
    }
    for (name, field) in [("flip_horizontal", "flipH"), ("flip_vertical", "flipV")] {
        if boolean(attributes, name) {
            transform.push_str(&format!(" {field}=\"1\""));
        }
    }
    let [horizontal, vertical, width, height] = bounds;
    let preset = super::presets::preset(&text(attributes, "kind"));
    let adjustments = super::presets::adjustments(attributes, bounds);
    let adjustments = if adjustments.is_empty() {
        "<a:avLst/>".to_owned()
    } else {
        let guides: String = adjustments
            .iter()
            .map(|(name, value)| format!("<a:gd name=\"{name}\" fmla=\"val {value}\"/>"))
            .collect();
        format!("<a:avLst>{guides}</a:avLst>")
    };
    let fill = if let Some(value) = attributes.get("fill") {
        solid_fill(&value.to_text(), number(attributes, "opacity"))?
    } else {
        "<a:noFill/>".into()
    };
    let border = object(attributes, "border")
        .map(|border| line(border, "", None))
        .transpose()?
        .unwrap_or_else(|| "<a:ln><a:noFill/></a:ln>".into());
    let shadow = shadow(attributes)?;
    Ok(format!(
        "<a:xfrm{transform}><a:off x=\"{horizontal}\" y=\"{vertical}\"/><a:ext cx=\"{width}\" cy=\"{height}\"/></a:xfrm><a:prstGeom prst=\"{preset}\">{adjustments}</a:prstGeom>{fill}{border}{shadow}"
    ))
}
/// Writes the shape properties of a straight line between the `from` and `to`
/// points of a connector: its frame, turned over where the line runs leftward
/// or upward, the line geometry and the stroke.
pub(crate) fn connector(attributes: &Properties) -> Result<String, String> {
    let endpoint = |name| {
        let fields = object(attributes, name);
        ["x", "y"].map(|field| {
            crate::presentation::units(fields.and_then(|fields| fields.get(field))).unwrap_or(0)
        })
    };
    let start = endpoint("from");
    let end = endpoint("to");
    let flip = if end[0] < start[0] {
        " flipH=\"1\""
    } else {
        ""
    };
    let vertical_flip = if end[1] < start[1] {
        " flipV=\"1\""
    } else {
        ""
    };
    let horizontal = start[0].min(end[0]);
    let vertical = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    let mut border = object(attributes, "border").cloned().unwrap_or_default();
    if let Some(opacity) = attributes.get("opacity") {
        border
            .entry("opacity".into())
            .or_insert_with(|| opacity.clone());
    }
    let cap = text(attributes, "cap");
    let cap = if cap.is_empty() {
        text(&border, "cap")
    } else {
        cap
    };
    let line = line(&border, &cap, object(attributes, "arrow"))?;
    Ok(format!(
        "<a:xfrm{flip}{vertical_flip}><a:off x=\"{horizontal}\" y=\"{vertical}\"/><a:ext cx=\"{width}\" cy=\"{height}\"/></a:xfrm><a:prstGeom prst=\"line\"><a:avLst/></a:prstGeom>{line}"
    ))
}
pub(crate) fn solid_fill(color: &str, opacity: Option<f64>) -> Result<String, String> {
    let color = color_value(color)?;
    let alpha = opacity
        .map(alpha)
        .filter(|value| *value < 100000)
        .map(|value| format!("<a:alpha val=\"{value}\"/>"));
    let color = if let Some(alpha) = alpha {
        format!("<a:srgbClr val=\"{color}\">{alpha}</a:srgbClr>")
    } else {
        format!("<a:srgbClr val=\"{color}\"/>")
    };
    Ok(format!("<a:solidFill>{color}</a:solidFill>"))
}
pub(crate) fn line(
    border: &Properties,
    cap: &str,
    markers: Option<&Properties>,
) -> Result<String, String> {
    if text(border, "style") == "none" {
        return Ok("<a:ln><a:noFill/></a:ln>".into());
    }
    let width = length(border, "width", 12700)?;
    let cap = match cap {
        "round" => " cap=\"rnd\"",
        "square" => " cap=\"sq\"",
        "flat" => " cap=\"flat\"",
        _ => "",
    };
    let color = text(border, "color");
    let fill = solid_fill(
        if color.is_empty() { "000000" } else { &color },
        number(border, "opacity"),
    )?;
    let dash = match text(border, "style").as_str() {
        "dashed" => "<a:prstDash val=\"dash\"/>",
        "dotted" => "<a:prstDash val=\"dot\"/>",
        _ => "",
    };
    let mut output = format!("<a:ln w=\"{width}\"{cap}>{fill}{dash}");
    if let Some(markers) = markers {
        for (prefix, tag) in [("start", "headEnd"), ("end", "tailEnd")] {
            let kind = text(markers, prefix);
            if kind.is_empty() || kind == "none" {
                continue;
            }
            let kind = crate::xml::escape(&kind).map_err(|error| error.to_string())?;
            output.push_str(&format!("<a:{tag} type=\"{kind}\""));
            for (field, keys) in [
                ("w", vec![format!("{prefix}_size"), "size".into()]),
                (
                    "len",
                    vec![
                        format!("{prefix}_length"),
                        "length".into(),
                        format!("{prefix}_size"),
                        "size".into(),
                    ],
                ),
            ] {
                if let Some(value) = keys
                    .iter()
                    .map(|key| text(markers, key))
                    .find(|value| !value.is_empty())
                {
                    output.push_str(&format!(
                        " {field}=\"{}\"",
                        crate::xml::escape(&value).map_err(|error| error.to_string())?
                    ));
                }
            }
            output.push_str("/>");
        }
    }
    output.push_str("</a:ln>");
    Ok(output)
}
pub(crate) fn shadow(attributes: &Properties) -> Result<String, String> {
    let Some(shadow) = object(attributes, "shadow") else {
        return Ok(String::new());
    };
    let color = text(shadow, "color");
    let color = color_value(if color.is_empty() { "000000" } else { &color })?;
    let opacity = number(shadow, "opacity").map(alpha).unwrap_or(35000);
    let blur = length(shadow, "blur", 50800)?;
    let distance = length(shadow, "distance", 25400)?;
    // ST_PositiveFixedAngle runs from 0 up to, but not including, 360 degrees.
    let direction = angle(number(shadow, "angle").unwrap_or(270.0))?.rem_euclid(21_600_000);
    Ok(format!(
        "<a:effectLst><a:outerShdw blurRad=\"{blur}\" dist=\"{distance}\" dir=\"{direction}\" algn=\"tl\" rotWithShape=\"0\"><a:srgbClr val=\"{color}\"><a:alpha val=\"{opacity}\"/></a:srgbClr></a:outerShdw></a:effectLst>"
    ))
}
fn length(attributes: &Properties, name: &str, fallback: i64) -> Result<i64, String> {
    attributes
        .get(name)
        .map(|value| {
            super::length_in_drawing_units(Some(value))
                .ok_or_else(|| format!("invalid drawing {name}"))
        })
        .transpose()
        .map(|value| value.unwrap_or(fallback))
}
fn alpha(value: f64) -> i64 {
    (value * if value <= 1.0 { 100000.0 } else { 1000.0 })
        .round()
        .clamp(0.0, 100000.0) as i64
}
fn angle(value: f64) -> Result<i64, String> {
    let units = (value * 60000.0).round();
    if !units.is_finite() || units < i32::MIN as f64 || units > i32::MAX as f64 {
        return Err("drawing angle exceeds the signed integer range".into());
    }
    Ok(units as i64)
}

fn color_value(value: &str) -> Result<String, String> {
    crate::properties::hex_color(value).ok_or_else(|| format!("invalid drawing color {value:?}"))
}
