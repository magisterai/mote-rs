//! Shared text semantics for editable and native presentation boxes.
use super::Theme;
use crate::{
    drawings::paragraphs::{self, DrawingParagraph},
    layout::{LayoutError, LayoutStyleResolver, TextStyle},
    model::{Content, Document, Element, ElementKind, Properties},
    properties::text,
};
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextDefaults {
    /// Leave theme defaults implicit in editable DrawingML.
    Inherited,
    /// Supply concrete theme fonts, sizes and colors for native measurement.
    Resolved,
}
pub(crate) fn resolve(
    document: &Document,
    content: &[Content],
    attributes: &Properties,
    theme: &Theme,
    defaults: TextDefaults,
) -> Result<Vec<DrawingParagraph>, LayoutError> {
    validate_content(content).map_err(LayoutError::Unsupported)?;
    let styles = LayoutStyleResolver::for_drawing_text(document);
    let native = defaults == TextDefaults::Resolved;
    let mut base = TextStyle::default();
    base.measurement.font = if native {
        theme.body.clone()
    } else {
        String::new()
    };
    base.measurement.size_points = if native { 18.0 } else { 0.0 };
    base.color = if native {
        theme.colors[0].clone()
    } else {
        String::new()
    };
    base = styles.inline(
        &Element {
            kind: ElementKind::Span,
            attributes: attributes.clone(),
            children: Vec::new(),
        },
        &base,
    );
    let heading = (text(attributes, "font").is_empty()
        && (native || text(attributes, "style").is_empty()))
    .then_some(if native {
        theme.heading.as_str()
    } else {
        "+mj-lt"
    });
    let mut paragraphs = paragraphs::resolve(
        content,
        &base,
        &paragraphs::paragraph_defaults(attributes),
        heading,
        &styles,
    );
    for paragraph in &mut paragraphs {
        for run in &mut paragraph.runs {
            if run.link.is_some() {
                run.style.color = theme.colors[10].clone();
            }
        }
    }
    Ok(paragraphs)
}
pub(super) fn validate_content(content: &[Content]) -> Result<(), String> {
    for child in content {
        if let Content::Element(element) = child {
            if !matches!(
                element.kind,
                ElementKind::Paragraph
                    | ElementKind::Heading
                    | ElementKind::Span
                    | ElementKind::Strong
                    | ElementKind::Emphasis
                    | ElementKind::Underline
                    | ElementKind::Strike
                    | ElementKind::Highlight
                    | ElementKind::SmallCaps
                    | ElementKind::Subscript
                    | ElementKind::Superscript
                    | ElementKind::Code
                    | ElementKind::Raw
                    | ElementKind::DefinitionList
                    | ElementKind::Term
                    | ElementKind::Definition
                    | ElementKind::Divider
                    | ElementKind::Link
                    | ElementKind::LineBreak
                    | ElementKind::Tab
                    | ElementKind::List
                    | ElementKind::OrderedList
                    | ElementKind::ListItem
                    | ElementKind::Blockquote
            ) {
                return Err(format!(
                    "presentation text boxes do not yet support {}",
                    element.kind.as_text()
                ));
            }
            validate_content(&element.children)?;
        }
    }
    Ok(())
}
