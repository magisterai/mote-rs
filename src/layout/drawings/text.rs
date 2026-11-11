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
    let flow_styles = styles;
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
    // Word lays out the text of a shape as it lays out body text.
    for paragraph in &mut paragraphs {
        paragraph.style.word_exact_baseline = true;
        flow_styles.apply_word_line_metrics(&mut paragraph.style.text);
        paragraph.style.line_height = paragraph.style.line_spacing.height(&paragraph.style.text);
        for run in &mut paragraph.runs {
            flow_styles.apply_word_line_metrics(&mut run.style);
        }
    }
    let mut bounds = geometry::text_rectangle(
        &text(&element.attributes, "kind"),
        geometry::rectangle(&element.attributes),
    );
    // Word keeps the text clear of the inner half of the shape's outline.
    let inset = crate::properties::object(&element.attributes, "border")
        .and_then(geometry::stroke)
        .map_or(0.0, |stroke| stroke.width / 2.0);
    bounds = [
        bounds[0] + inset,
        bounds[1] + inset,
        (bounds[2] - 2.0 * inset).max(0.0),
        (bounds[3] - 2.0 * inset).max(0.0),
    ];
    Ok(crate::layout::drawing_text::fit(&paragraphs, bounds, &attributes, measurer)?.page)
}
