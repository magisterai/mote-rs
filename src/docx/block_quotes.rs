use super::{Error, check_attributes, paragraphs, styles::Styles};
use crate::model::{Element, Value};

pub(super) fn write(
    quote: &Element,
    styles: &Styles<'_>,
    leading_column_break: bool,
    state: &mut super::document::DocumentState,
) -> Result<String, Error> {
    check_attributes(
        &quote.attributes,
        &[
            "style",
            "border",
            "fill",
            "alignment",
            "spacing",
            "indent",
            "spacing_before",
            "spacing_after",
            "line_spacing",
        ],
        "blockquote",
    )?;
    let mut attributes = quote.attributes.clone();
    attributes
        .entry("style".into())
        .or_insert_with(|| Value::String("BlockQuote".into()));
    paragraphs::write_flattened(
        &attributes,
        &quote.children,
        styles,
        None,
        leading_column_break,
        &mut state.resources,
    )
}
