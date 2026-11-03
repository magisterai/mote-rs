use super::{Error, check_attributes, paragraphs, styles::Styles};
use crate::model::{Content, Element, ElementKind, Value};

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
    if state.table_cell_depth > 0
        && quote.children.iter().any(|child| {
            matches!(child, Content::Element(element) if element.kind == ElementKind::Paragraph)
        })
    {
        let mut output = String::new();
        for (index, (explicit, children)) in crate::tables::paragraph_units(&quote.children)
            .into_iter()
            .enumerate()
        {
            let properties = crate::tables::cell_defaults(&explicit, &attributes, false);
            output.push_str(&paragraphs::write(
                &properties, &children, styles, None,
                leading_column_break && index == 0, &mut state.resources,
            )?);
        }
        return Ok(output);
    }
    paragraphs::write_flattened(
        &attributes,
        &quote.children,
        styles,
        None,
        leading_column_break,
        &mut state.resources,
    )
}
