//! DrawingML picture effects shared by document and presentation packages.
use crate::{
    model::Properties,
    properties::{boolean, number, object},
};
fn percent(properties: &Properties, name: &str) -> Option<i64> {
    number(properties, name).map(|value| (value * 1000.0).round() as i64)
}
/// Returns the share of its own alpha that a picture keeps: the product of its
/// `opacity` and the complement of its `effects.transparency`.
pub(crate) fn alpha(attributes: &Properties) -> f64 {
    let opacity = number(attributes, "opacity").map_or(1.0, |value| {
        if value <= 1.0 { value } else { value / 100.0 }.clamp(0.0, 1.0)
    });
    let transparency = object(attributes, "effects")
        .and_then(|effects| number(effects, "transparency"))
        .map_or(0.0, |value| value.clamp(0.0, 100.0) / 100.0);
    opacity * (1.0 - transparency)
}
pub(crate) fn effects(attributes: &Properties) -> String {
    let empty = Properties::new();
    let effects = object(attributes, "effects").unwrap_or(&empty);
    let mut output = String::new();
    let mut luminance = String::new();
    for (name, field) in [("brightness", "bright"), ("contrast", "contrast")] {
        if let Some(value) = percent(effects, name) {
            luminance.push_str(&format!(" {field}=\"{value}\""));
        }
    }
    if !luminance.is_empty() {
        output.push_str(&format!("<a:lum{luminance}/>"));
    }
    if boolean(effects, "grayscale") {
        output.push_str("<a:grayscl/>");
    }
    if attributes.contains_key("opacity") || effects.contains_key("transparency") {
        output.push_str(&format!(
            "<a:alphaModFix amt=\"{}\"/>",
            (alpha(attributes) * 100000.0).round() as i64
        ));
    }
    output
}
