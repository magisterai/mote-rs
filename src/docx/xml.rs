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
