use super::{Error, paragraph_properties, runs, styles::Styles, whitespace, xml};
use crate::model::{Content, Properties};

pub(super) fn write(
    attributes: &Properties,
    children: &[Content],
    styles: &Styles<'_>,
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
    properties.push_str(&paragraph_properties::write(&attributes)?);
    let mut paragraph = String::from("<w:p>");
    if !properties.is_empty() {
        paragraph.push_str(&format!("<w:pPr>{properties}</w:pPr>"));
    }
    paragraph.push_str(&runs::write_inlines(
        &whitespace::trim_line_edges(children),
        styles,
    )?);
    paragraph.push_str("</w:p>");
    Ok(paragraph)
}
