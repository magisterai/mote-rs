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

pub(super) use crate::xml::WORD_ROOT_ATTRIBUTES as ROOT_ATTRIBUTES;
