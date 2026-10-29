use super::{Error, check_attributes, run_properties::RunFormatting, styles::Styles, xml};
use crate::model::{Content, ElementKind};

pub(super) fn write_inlines(children: &[Content], styles: &Styles<'_>) -> Result<String, Error> {
    write_formatted(children, &RunFormatting::default(), styles)
}

fn write_formatted(
    children: &[Content],
    formatting: &RunFormatting,
    styles: &Styles<'_>,
) -> Result<String, Error> {
    let mut result = String::new();
    for child in children {
        match child {
            Content::Text(text) => result.push_str(&write_text(text, formatting, styles)?),
            Content::Element(element) if element.kind == ElementKind::LineBreak => {
                check_attributes(&element.attributes, &[], "line_break")?;
                if !element.children.is_empty() {
                    return Err(Error::InvalidValue(
                        "line_break cannot contain children".into(),
                    ));
                }
                result.push_str("<w:r><w:br/></w:r>");
            }
            Content::Element(element) => {
                let formatting = formatting.for_element(element, styles)?;
                result.push_str(&write_formatted(&element.children, &formatting, styles)?);
            }
        }
    }
    Ok(result)
}

fn write_text(
    text: &str,
    formatting: &RunFormatting,
    styles: &Styles<'_>,
) -> Result<String, Error> {
    if text.is_empty() {
        return Ok(String::new());
    }
    let mut result = String::from("<w:r>");
    let properties = formatting.write(styles)?;
    if !properties.is_empty() {
        result.push_str(&format!("<w:rPr>{properties}</w:rPr>"));
    }
    for (line_index, line) in text.split('\n').enumerate() {
        if line_index > 0 {
            result.push_str("<w:br/>");
        }
        for (part_index, part) in line.split('\t').enumerate() {
            if part_index > 0 {
                result.push_str(if formatting.fixed_tabs {
                    "<w:t xml:space=\"preserve\">    </w:t>"
                } else {
                    "<w:tab/>"
                });
            }
            if !part.is_empty() {
                result.push_str(&format!(
                    "<w:t xml:space=\"preserve\">{}</w:t>",
                    xml::escape(part)?
                ));
            }
        }
    }
    result.push_str("</w:r>");
    Ok(result)
}
