use super::Error;
pub(super) use crate::xml::HEADER;

pub(super) fn escape(text: &str) -> Result<String, Error> {
    crate::xml::escape(text).map_err(|error| Error::InvalidValue(error.to_string()))
}

pub(super) fn word_part(name: &str, content: &str) -> String {
    format!(
        "{HEADER}<w:{name} xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">{content}</w:{name}>"
    )
}

pub(super) const ROOT_ATTRIBUTES: &str = r#" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="w14 wps""#;
