use super::{Error, check_attributes, paragraphs, styles::Styles};
use crate::model::{Content, Element, ElementKind, Value};

pub(super) fn write(
    quote: &Element,
    styles: &Styles<'_>,
    leading_column_break: bool,
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
    let children = flatten_content(&quote.children)?;
    paragraphs::write(&attributes, &children, styles, None, leading_column_break)
}

fn flatten_content(children: &[Content]) -> Result<Vec<Content>, Error> {
    let mut result = Vec::new();
    for child in children {
        if let Content::Element(element) = child
            && element.kind.is_block()
        {
            // Outside table cells, a quote is one paragraph. Its nested block
            // structure supplies text, without independent layout or numbering.
            let supported: &[&str] = match element.kind {
                ElementKind::Paragraph | ElementKind::Blockquote => &[],
                ElementKind::List | ElementKind::OrderedList => {
                    &["id", "start", "restart", "format", "level"]
                }
                ElementKind::ListItem => &["marker", "level"],
                _ => {
                    return Err(Error::Unsupported(format!(
                        "{} inside a block quote",
                        element.kind.as_text()
                    )));
                }
            };
            check_attributes(&element.attributes, supported, "nested quote content")?;
            result.extend(flatten_content(&element.children)?);
        } else {
            result.push(child.clone());
        }
    }
    Ok(result)
}
