use super::Error;

pub(super) const HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>";

pub(super) fn escape(text: &str) -> Result<String, Error> {
    let mut result = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '\"' => result.push_str("&quot;"),
            '\'' => result.push_str("&apos;"),
            '\r' => result.push_str("&#13;"),
            '\n' => result.push_str("&#10;"),
            '\t' => result.push_str("&#9;"),
            '\u{20}'..='\u{d7ff}' | '\u{e000}'..='\u{fffd}' | '\u{10000}'..='\u{10ffff}' => {
                result.push(character)
            }
            _ => {
                return Err(Error::InvalidValue(format!(
                    "character U+{:04X} is not allowed in XML 1.0",
                    character as u32
                )));
            }
        }
    }
    Ok(result)
}

pub(super) fn word_part(name: &str, content: &str) -> String {
    format!(
        "{HEADER}<w:{name} xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">{content}</w:{name}>"
    )
}

pub(super) const ROOT_ATTRIBUTES: &str = r#" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="w14 wps""#;
