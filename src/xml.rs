//! XML 1.0 escaping shared by editable document and chart parts.
use std::{error::Error, fmt};

#[derive(Debug)]
pub(crate) struct InvalidCharacter(char);
impl fmt::Display for InvalidCharacter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character U+{:04X} is not allowed in XML 1.0",
            self.0 as u32
        )
    }
}
impl Error for InvalidCharacter {}

pub(crate) const HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>";

pub(crate) fn escape(text: &str) -> Result<String, InvalidCharacter> {
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
                return Err(InvalidCharacter(character));
            }
        }
    }
    Ok(result)
}

pub(crate) const WORD_ROOT_ATTRIBUTES: &str = r#" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="w14 wps""#;

pub(crate) const PRESENTATION_ROOT_ATTRIBUTES: &str = r#"xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:a14="http://schemas.microsoft.com/office/drawing/2010/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="a14" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main""#;

/// The child elements of one property container. The elements are supplied in
/// any order and written in the sequence that the schema of the container
/// declares, so that their order rests on one table and not on the order of the
/// statements that supply them.
pub(crate) struct OrderedChildren {
    sequence: &'static [&'static str],
    children: Vec<(usize, String)>,
}

impl OrderedChildren {
    /// `sequence` lists the qualified names of the children in schema order.
    pub(crate) fn new(sequence: &'static [&'static str]) -> Self {
        Self {
            sequence,
            children: Vec::new(),
        }
    }

    /// Adds one complete element. Elements of one name keep the order in which
    /// they were added.
    pub(crate) fn push(&mut self, element: impl Into<String>) {
        let element = element.into();
        if element.is_empty() {
            return;
        }
        let name = element[1..]
            .split([' ', '/', '>'])
            .next()
            .unwrap_or_default();
        let position = self.sequence.iter().position(|member| *member == name);
        debug_assert!(position.is_some(), "{name} is not in the sequence");
        self.children
            .push((position.unwrap_or(self.sequence.len()), element));
    }

    pub(crate) fn write(mut self) -> String {
        self.children.sort_by_key(|(position, _)| *position);
        self.children
            .into_iter()
            .map(|(_, element)| element)
            .collect()
    }
}

/// The children of `a:rPr` in the sequence of CT_TextCharacterProperties.
pub(crate) const DRAWING_RUN_PROPERTIES: &[&str] = &[
    "a:ln",
    "a:noFill",
    "a:solidFill",
    "a:gradFill",
    "a:blipFill",
    "a:pattFill",
    "a:grpFill",
    "a:effectLst",
    "a:effectDag",
    "a:highlight",
    "a:uLnTx",
    "a:uLn",
    "a:uFillTx",
    "a:uFill",
    "a:latin",
    "a:ea",
    "a:cs",
    "a:sym",
    "a:hlinkClick",
    "a:hlinkMouseOver",
    "a:rtl",
    "a:extLst",
];

/// The size of text in hundredths of a point, confined to the range of
/// ST_TextFontSize, which runs from one point to four thousand.
pub(crate) fn drawing_font_size(points: f64) -> i64 {
    ((points * 100.0).round() as i64).clamp(100, 400_000)
}

#[cfg(test)]
mod tests {
    use super::{OrderedChildren, drawing_font_size};

    #[test]
    fn children_are_written_in_the_declared_sequence_whatever_their_order_of_supply() {
        let mut children = OrderedChildren::new(&["w:first", "w:second", "w:third"]);
        children.push("<w:third w:val=\"1\"/>");
        children.push("<w:second><w:inner/></w:second>");
        children.push("");
        children.push("<w:first/>");
        children.push("<w:second/>");
        assert_eq!(
            children.write(),
            "<w:first/><w:second><w:inner/></w:second><w:second/><w:third w:val=\"1\"/>"
        );
    }

    #[test]
    fn a_drawing_font_size_stays_within_the_range_of_the_schema() {
        assert_eq!(drawing_font_size(0.6), 100);
        assert_eq!(drawing_font_size(12.345), 1235);
        assert_eq!(drawing_font_size(5000.0), 400_000);
    }
}
