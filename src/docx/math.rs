use super::{Error, resources::Resources, run_properties::RunFormatting};
use crate::{model::Element, properties};
pub(super) fn write(
    element: &Element,
    formatting: &RunFormatting,
    resources: &mut Resources,
) -> Result<String, Error> {
    let color = formatting
        .properties
        .get("color")
        .map(super::values::color)
        .transpose()?
        .unwrap_or_default();
    let options = crate::math::Options {
        size_half_points: 0,
        color,
        presentation: false,
    };
    let equation = crate::math::convert(element, options)
        .map_err(|error| Error::InvalidValue(error.to_string()))?;
    resources.warnings.extend(equation.warnings);
    Ok(if equation.xml.is_empty() {
        format!(
            r#"<w:r><w:t xml:space="preserve">{}</w:t></w:r>"#,
            super::xml::escape(&properties::text(&element.attributes, "source"))?
        )
    } else {
        equation.xml
    })
}
