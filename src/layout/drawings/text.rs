//! Resolves shape-specific text bounds before shared drawing-text fitting.
use super::geometry;
use crate::{
    drawings::paragraphs,
    layout::{LayoutError, LayoutStyleResolver, TextPage, TextStyle},
    measurement::TextMeasurer,
    model::{Element, ElementKind},
    properties::text,
};
pub(super) fn prepare(
    element: &Element,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
) -> Result<TextPage, LayoutError> {
    let attributes = crate::drawings::text_attributes(&element.attributes);
    let mut base = TextStyle::default();
    base.measurement.font = "Aptos".into();
    base.measurement.size_points = 18.0;
    base = styles.inline(
        &Element {
            kind: ElementKind::Span,
            attributes: attributes.clone(),
            children: Vec::new(),
        },
        &base,
    );
    let paragraphs = paragraphs::resolve(
        &element.children,
        &base,
        &paragraphs::paragraph_defaults(&attributes),
        (text(&attributes, "font").is_empty() && text(&attributes, "style").is_empty())
            .then_some("Aptos"),
        styles,
        Some(measurer),
    )?;
    let mut bounds = geometry::text_rectangle(
        &text(&element.attributes, "kind"),
        geometry::rectangle(&element.attributes),
    );
    bounds[1] -= geometry::points(attributes.get("size"), 18.0).max(0.0) * 0.055;
    Ok(crate::layout::drawing_text::fit(&paragraphs, bounds, &attributes, measurer)?.page)
}
