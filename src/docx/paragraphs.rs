use super::{Error, numbering, paragraph_properties, runs, styles::Styles, xml};
use crate::model::{Content, Properties};

pub(super) fn write(
    attributes: &Properties,
    children: &[Content],
    styles: &Styles<'_>,
    numbering_reference: Option<(usize, u8)>,
    leading_column_break: bool,
    resources: &mut super::resources::Resources,
) -> Result<String, Error> {
    let mut paragraph = opening(attributes, styles, numbering_reference, resources)?;
    let (bookmarks, bookmark_identifiers) = super::references::start_pending(resources)?;
    paragraph.push_str(&bookmarks);
    if leading_column_break {
        paragraph.push_str("<w:r><w:br w:type=\"column\"/></w:r>");
    }
    paragraph.push_str(&runs::write_inlines(children, styles, resources)?);
    paragraph.push_str(&super::references::end_pending(&bookmark_identifiers));
    paragraph.push_str("</w:p>");
    Ok(paragraph)
}

pub(super) fn opening(
    attributes: &Properties,
    styles: &Styles<'_>,
    numbering_reference: Option<(usize, u8)>,
    resources: &mut super::resources::Resources,
) -> Result<String, Error> {
    let mut attributes = attributes.clone();
    for field in remove_run_attributes(&mut attributes) {
        resources.warnings.push(format!("paragraph.{field} is retained but has no editable paragraph effect; use an inline span or a named style"));
    }
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
    Ok(paragraph)
}

pub(super) fn remove_run_attributes(attributes: &mut Properties) -> Vec<&'static str> {
    [
        "font",
        "size",
        "color",
        "bold",
        "italic",
        "underline",
        "strike",
        "double_strike",
        "small_caps",
        "highlight",
    ]
    .into_iter()
    .filter(|field| attributes.remove(*field).is_some())
    .collect()
}
