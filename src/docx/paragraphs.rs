use super::{Error, numbering, paragraph_properties, runs, styles::Styles, xml};
use crate::model::{Content, Properties};

pub(super) fn write(
    attributes: &Properties,
    children: &[Content],
    styles: &Styles<'_>,
    numbering_reference: Option<(usize, u8)>,
    leading_column_break: bool,
) -> Result<String, Error> {
    let mut attributes = attributes.clone();
    let mut properties = String::new();
    if let Some(name) = attributes
        .remove("style")
        .map(|value| value.to_text())
        .filter(|name| !name.is_empty())
    {
        properties.push_str(&format!(
            "<w:pStyle w:val=\"{}\"/>",
            xml::escape(styles.identifier(&name)?)?
        ));
    }
    if let Some((identifier, level)) = numbering_reference {
        properties.push_str(&numbering::paragraph_reference(identifier, level));
    }
    properties.push_str(&paragraph_properties::write(&attributes)?);
    let mut paragraph = String::from("<w:p>");
    if !properties.is_empty() {
        paragraph.push_str(&format!("<w:pPr>{properties}</w:pPr>"));
    }
    if leading_column_break {
        paragraph.push_str("<w:r><w:br w:type=\"column\"/></w:r>");
    }
    paragraph.push_str(&runs::write_inlines(children, styles)?);
    paragraph.push_str("</w:p>");
    Ok(paragraph)
}
