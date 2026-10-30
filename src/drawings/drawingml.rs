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
    let kind = text(attributes, "kind");
    let preset = match kind.as_str() {
        "round_rect" => "roundRect",
        "ellipse" | "triangle" | "diamond" | "hexagon" | "chevron" | "cloud" | "parallelogram"
        | "trapezoid" => &kind,
        _ => "rect",
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
        "<a:xfrm{transform}><a:off x=\"{horizontal}\" y=\"{vertical}\"/><a:ext cx=\"{width}\" cy=\"{height}\"/></a:xfrm><a:prstGeom prst=\"{preset}\"><a:avLst/></a:prstGeom>{fill}{border}{shadow}"
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
fn shadow(attributes: &Properties) -> Result<String, String> {
    let Some(shadow) = object(attributes, "shadow") else {
        return Ok(String::new());
    };
    let color = text(shadow, "color");
    let color = color_value(if color.is_empty() { "000000" } else { &color })?;
    let opacity = number(shadow, "opacity").map(alpha).unwrap_or(35000);
    let blur = length(shadow, "blur", 50800)?;
    let distance = length(shadow, "distance", 25400)?;
    let direction = angle(number(shadow, "angle").unwrap_or(270.0))?;
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
    let digits = value.trim().trim_start_matches('#');
    if !matches!(digits.len(), 3 | 6) || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("invalid drawing color {value:?}"));
    }
    Ok(if digits.len() == 3 {
        digits
            .chars()
            .flat_map(|character| [character, character])
            .collect::<String>()
    } else {
        digits.into()
    }
    .to_ascii_uppercase())
}
