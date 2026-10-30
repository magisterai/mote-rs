//! DrawingML picture effects shared by document and presentation packages.
use crate::{
    model::Properties,
    properties::{boolean, number, object},
};
fn percent(properties: &Properties, name: &str) -> Option<i64> {
    number(properties, name).map(|value| (value * 1000.0).round() as i64)
}
pub(crate) fn effects(attributes: &Properties) -> String {
    let Some(effects) = object(attributes, "effects") else {
        return String::new();
    };
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
    if let Some(transparency) = percent(effects, "transparency") {
        output.push_str(&format!(
            "<a:alphaModFix amt=\"{}\"/>",
            (100000 - transparency).max(0)
        ));
    }
    output
}
