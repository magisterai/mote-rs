//! Stable note selection, numbering, and page reservations for native document flow.
use super::{LayoutError, LayoutStyleResolver, LineOptions, PageGeometry, TextLine, TextRun};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Element, ElementKind, Properties, Section, Value},
};
use std::collections::{BTreeMap, BTreeSet};
mod flow;
pub(super) use flow::NoteFlowContext;
mod placement;

pub(super) const NUMBER: &str = "__layout_note_number";
const PARAGRAPH_GAP: f64 = 1.8;
const SEPARATOR_BAND: f64 = 14.84;
const BODY_GAP: f64 = 6.0;
const BOTTOM_PADDING: f64 = 1.2;
const ENDNOTE_BAND: f64 = 20.878;

struct Note {
    lines: Vec<TextLine>,
    height: f64,
}
#[derive(Default)]
pub(super) struct NoteLayout {
    footnotes: Vec<Note>,
    endnotes: Vec<Note>,
    endnote_footer_height: f64,
    pages: BTreeMap<usize, Vec<usize>>,
    assigned: BTreeSet<usize>,
    pub warnings: Vec<String>,
}
impl NoteLayout {
    pub fn prepare(
        sections: &mut [Section],
        styles: &LayoutStyleResolver<'_>,
        measurer: &mut TextMeasurer<'_>,
    ) -> Result<Self, LayoutError> {
        let mut layout = Self::default();
        let eligible = sections.len() == 1 && sections[0].columns.count.unwrap_or(1) <= 1;
        for section in sections {
            let geometry = PageGeometry::from_section(section)?;
            layout.endnote_footer_height = section
                .footer
                .properties
                .get("height")
                .and_then(super::style_values::points)
                .unwrap_or(0.0);
            layout.select(
                &mut section.content,
                eligible,
                false,
                styles,
                measurer,
                geometry,
            )?;
            layout.select(
                &mut section.header.content,
                false,
                false,
                styles,
                measurer,
                geometry,
            )?;
            layout.select(
                &mut section.footer.content,
                false,
                false,
                styles,
                measurer,
                geometry,
            )?;
        }
        Ok(layout)
    }
    fn select(
        &mut self,
        content: &mut [Content],
        eligible: bool,
        inline: bool,
        styles: &LayoutStyleResolver<'_>,
        measurer: &mut TextMeasurer<'_>,
        geometry: PageGeometry,
    ) -> Result<(), LayoutError> {
        for node in content {
            let Content::Element(element) = node else {
                continue;
            };
            if matches!(element.kind, ElementKind::Footnote | ElementKind::Endnote) {
                element.attributes.remove(NUMBER);
                // Nested notes are terminal for selection, but their fallback still warns.
                self.select(
                    &mut element.children,
                    false,
                    true,
                    styles,
                    measurer,
                    geometry,
                )?;
                if eligible && inline && stable_content(&element.children) {
                    let sequence = if element.kind == ElementKind::Footnote {
                        &mut self.footnotes
                    } else {
                        &mut self.endnotes
                    };
                    let number = sequence.len() + 1;
                    let marker = if element.kind == ElementKind::Footnote {
                        number.to_string()
                    } else {
                        crate::numbering::format_counter(number as u64, "lower_roman")
                            .unwrap_or_else(|_| number.to_string())
                    };
                    let note = measure(element, &marker, styles, measurer, geometry)?;
                    sequence.push(note);
                    element
                        .attributes
                        .insert(NUMBER.into(), Value::Number(number as f64));
                    if has_bookmarks(&element.children) {
                        self.warnings.push(
                            "native note text retains bookmarks without a placed destination"
                                .into(),
                        );
                    }
                    if element.kind == ElementKind::Footnote {
                        let alignment = styles.paragraph(&empty_paragraph()).alignment;
                        if !alignment.is_empty() && alignment != "left" {
                            self.warnings.push("native footnotes use left alignment instead of document paragraph alignment".into());
                        }
                    }
                } else {
                    self.warnings.push(format!("native {} content is flattened inline because a note story cannot be placed in this context", element.kind.as_text()));
                }
                continue;
            }
            let (children_inline, visible) = child_flow(element.kind, inline);
            self.select(
                &mut element.children,
                eligible && visible && element.kind != ElementKind::Cell,
                children_inline,
                styles,
                measurer,
                geometry,
            )?;
        }
        Ok(())
    }
    fn stack_height(&self, numbers: &[usize]) -> f64 {
        numbers
            .iter()
            .map(|number| self.footnotes[number - 1].height)
            .sum::<f64>()
            + numbers.len().saturating_sub(1) as f64 * PARAGRAPH_GAP
    }
    fn body_bottom(&self, numbers: &[usize], geometry: PageGeometry) -> f64 {
        let height = self.stack_height(numbers);
        if height <= 0.0 {
            geometry.content_bottom()
        } else {
            geometry.content_bottom() - BOTTOM_PADDING - height - SEPARATOR_BAND - BODY_GAP
        }
    }
}
fn child_flow(kind: ElementKind, inline: bool) -> (bool, bool) {
    use ElementKind::*;
    match kind {
        Paragraph | Heading | Blockquote | Caption | ListItem => (true, true),
        List | OrderedList | Table | Row | Cell => (false, true),
        Strong | Emphasis | Span | Code | Link | Bookmark => (true, inline),
        _ => (false, false),
    }
}
fn stable_content(content: &[Content]) -> bool {
    let content = match content {
        [Content::Element(element)] if element.kind == ElementKind::Paragraph => &element.children,
        content => content,
    };
    stable_content_inline(content)
}

fn stable_content_inline(content: &[Content]) -> bool {
    !content.iter().any(|node| match node {
        Content::Element(element)
            if matches!(element.kind, ElementKind::Footnote | ElementKind::Endnote) =>
        {
            contains_dynamic(&element.children)
        }
        Content::Element(element) => {
            element.kind.is_block()
                || element.kind.is_table_part()
                || matches!(
                    element.kind,
                    ElementKind::PageNumber | ElementKind::Reference
                )
                || !stable_content_inline(&element.children)
        }
        _ => false,
    })
}
fn contains_dynamic(content: &[Content]) -> bool {
    content.iter().any(|node| matches!(node, Content::Element(element) if matches!(element.kind, ElementKind::PageNumber | ElementKind::Reference) || contains_dynamic(&element.children)))
}
fn has_bookmarks(content: &[Content]) -> bool {
    content.iter().any(|node| matches!(node, Content::Element(element) if element.kind == ElementKind::Bookmark || has_bookmarks(&element.children)))
}
fn empty_paragraph() -> Element {
    Element {
        source_name: None,
        kind: ElementKind::Paragraph,
        attributes: Properties::new(),
        children: Vec::new(),
    }
}
fn measure(
    element: &Element,
    marker: &str,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
    geometry: PageGeometry,
) -> Result<Note, LayoutError> {
    let style = styles.paragraph(&empty_paragraph());
    let mut content = element.children.clone();
    let prefix = Content::Text(format!("{marker} "));
    match content.first_mut() {
        Some(Content::Element(first)) if first.kind == ElementKind::Paragraph => {
            first.children.insert(0, prefix)
        }
        _ => content.insert(
            0,
            Content::Element(Element {
                children: vec![prefix],
                ..empty_paragraph()
            }),
        ),
    }
    let mut runs = Vec::new();
    for (index, node) in content.iter().enumerate() {
        if crate::references::text(std::slice::from_ref(node)).is_empty() {
            continue;
        }
        if index > 0 {
            let mut newline = TextRun::new("\n", style.text.clone());
            newline.preserve_space = true;
            runs.push(newline);
        }
        let children = match node {
            Content::Element(element) => element.children.as_slice(),
            _ => std::slice::from_ref(node),
        };
        runs.extend(super::resolve_inline(children, &style.text, styles));
    }
    let options = LineOptions {
        width: geometry.width - geometry.margin_left - geometry.margin_right,
        base_style: style.text,
        base_line_height: Some(style.line_height),
        spacing: style.line_spacing,
        fallback_multiplier: style.line_multiplier,
        ..Default::default()
    };
    let mut lines = super::plan_text(&runs, &options, measurer)?;
    for line in &mut lines {
        for fragment in &mut line.fragments {
            fragment.anchors.clear();
        }
    }
    let height = lines.iter().map(|line| line.height).sum();
    Ok(Note { lines, height })
}
