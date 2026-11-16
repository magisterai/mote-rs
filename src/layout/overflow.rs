//! Warnings for text that its frame cuts off.
use crate::{model::Element, presentation::fields::FieldKind, properties::text as property_text};

/// Describes a slide object by its path, followed by its name where it has
/// one.
pub(crate) fn slide_object(element: &Element, path: &str) -> String {
    let name = property_text(&element.attributes, "name");
    if name.is_empty() {
        path.to_owned()
    } else {
        format!("{path} {name:?}")
    }
}

/// Describes a shape of a document by its name, where it has one.
pub(crate) fn document_shape(element: &Element) -> String {
    let name = property_text(&element.attributes, "name");
    if name.is_empty() {
        "a shape".to_owned()
    } else {
        format!("shape {name:?}")
    }
}

/// Returns the `text.fit` of a box or a shape.
pub(crate) fn text_fit(element: &Element) -> String {
    property_text(
        &crate::drawings::text_attributes(&element.attributes),
        "fit",
    )
}

/// The warning for a box or a shape of a slide whose text falls below its
/// frame.
pub(crate) fn slide_frame_text(element: &Element, path: &str) -> String {
    frame_text(&slide_object(element, path), &text_fit(element))
}

/// The warning for text that falls below its frame under the given
/// `text.fit`.
pub(crate) fn frame_text(described: &str, fit: &str) -> String {
    if fit == "shrink" {
        format!(
            "the text of {described} exceeds its frame at the least size of text.fit = \"shrink\"; enlarge the frame or shorten the text"
        )
    } else {
        format!(
            "the text of {described} exceeds its frame; set text.fit = \"shrink\" or enlarge the frame"
        )
    }
}

/// The warning for a footer, a date or a slide number that the native frame
/// of the field cuts off.
pub(crate) fn field_text(kind: FieldKind, slide_path: &str) -> String {
    let name = match kind {
        FieldKind::Footer => "footer",
        FieldKind::Date { .. } => "date",
        FieldKind::SlideNumber => "slide number",
    };
    format!(
        "the {name} of {slide_path} exceeds its frame; shorten the text or reduce field_text.size"
    )
}

/// The warning for a slide table whose rows, grown to hold their text, pass
/// the lower edge of the frame.
pub(crate) fn table_rows(described: &str) -> String {
    format!(
        "the rows of the table in {described} grow beyond its frame to hold their text; enlarge the frame or shorten the text"
    )
}

/// The warning for a cell of a slide table whose text is cut off. The row and
/// the column are counted from zero.
pub(crate) fn cell_text(described: &str, row: usize, column: usize) -> String {
    format!(
        "the text of row {} column {} of the table in {described} exceeds its cell; enlarge the cell or shorten the text",
        row + 1,
        column + 1
    )
}

/// The warning for a row of a document table whose exact height cuts off its
/// content. The row is counted from zero, and the page is the one on which
/// the outermost table begins.
pub(crate) fn exact_row(row: usize, page_number: usize, nested: bool) -> String {
    format!(
        "row {} of {} on page {page_number} is cut off at its exact height; set height_rule = \"at_least\" or a greater height",
        row + 1,
        if nested {
            "a table nested in the table"
        } else {
            "the table"
        }
    )
}
