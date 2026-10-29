use super::{Error, check_attributes, xml};
use crate::model::{Content, ElementKind};

pub(super) fn write_inlines(children: &[Content]) -> Result<String, Error> {
    let mut result = String::new();
    for child in children {
        match child {
            Content::Text(text) => result.push_str(&write_text(text)?),
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
                return Err(Error::Unsupported(format!(
                    "{} inline content",
                    element.kind.as_text()
                )));
            }
        }
    }
    Ok(result)
}

fn write_text(text: &str) -> Result<String, Error> {
    if text.is_empty() {
        return Ok(String::new());
    }
    let mut result = String::from("<w:r>");
    for (line_index, line) in text.split('\n').enumerate() {
        if line_index > 0 {
            result.push_str("<w:br/>");
        }
        for (part_index, part) in line.split('\t').enumerate() {
            if part_index > 0 {
                result.push_str("<w:tab/>");
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
