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
    let styles = LayoutStyleResolver::for_drawing_text(styles.document);
    let attributes = crate::drawings::text_attributes(&element.attributes);
    let mut base = TextStyle::default();
    base.measurement.font = "Aptos".into();
    base.measurement.size_points = 18.0;
    base = styles.inline(
        &Element {
            source_name: None,
            source_location: None,
            kind: ElementKind::Span,
            attributes: attributes.clone(),
            children: Vec::new(),
        },
        &base,
    );
    // Word writes the shape's text formatting on every run of the text box, so it
    // overrides the paragraph style rather than being overridden by it.
    let mut defaults = paragraphs::paragraph_defaults(&attributes);
    for name in [
        "font",
        "size",
        "color",
        "highlight",
        "bold",
        "italic",
        "underline",
        "small_caps",
        "strike",
        "double_strike",
    ] {
        if let Some(value) = attributes.get(name) {
            defaults.entry(name.into()).or_insert_with(|| value.clone());
        }
    }
    let mut paragraphs = paragraphs::resolve(
        &element.children,
        &base,
        &defaults,
        (text(&attributes, "font").is_empty() && text(&attributes, "style").is_empty())
            .then_some("Aptos"),
        &styles,
    );
    for paragraph in &mut paragraphs {
        paragraph.style.word_exact_baseline = true;
    }
    let mut bounds = geometry::text_rectangle(
        &text(&element.attributes, "kind"),
        geometry::rectangle(&element.attributes),
    );
    bounds[1] -= geometry::points(attributes.get("size"), 18.0).max(0.0) * 0.055;
    Ok(crate::layout::drawing_text::fit(&paragraphs, bounds, &attributes, measurer)?.page)
}
