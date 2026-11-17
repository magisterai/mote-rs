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
const SEPARATOR_BAND: f64 = 14.84;
const BODY_GAP: f64 = 6.0;

struct Note {
    lines: Vec<TextLine>,
    height: f64,
    /// The space after the note's paragraph, which Word keeps below each note.
    after: f64,
    /// The left edge of the text area of the section that holds the reference.
    left: f64,
    /// The width of the text area of the section that holds the reference.
    width: f64,
}
/// A context in which a note story cannot be placed, so that the note keeps
/// its content inline.
#[derive(Clone, Copy)]
enum Obstacle {
    Header,
    Footer,
    TableCell,
    SectionColumns,
    LastSectionColumns,
    EnclosingNote,
    Container(ElementKind),
    OutsideParagraph,
    UnstableContent,
}
impl Obstacle {
    fn reason(self) -> String {
        match self {
            Self::Header => "a note story cannot be placed in a header".into(),
            Self::Footer => "a note story cannot be placed in a footer".into(),
            Self::TableCell => "a note story cannot be placed in a table cell".into(),
            Self::SectionColumns => {
                "a note story cannot be placed in a section with more than one column".into()
            }
            Self::LastSectionColumns => {
                "the endnotes cannot be placed after a last section with more than one column"
                    .into()
            }
            Self::EnclosingNote => "a note story cannot be placed inside another note".into(),
            Self::Container(kind) => {
                let name = kind.as_text().replace('_', " ");
                let article = if name.starts_with(['a', 'e', 'i', 'o', 'u']) {
                    "an"
                } else {
                    "a"
                };
                format!("a note story cannot be placed inside {article} {name}")
            }
            Self::OutsideParagraph => {
                "a note story cannot be placed outside the text of a paragraph".into()
            }
            Self::UnstableContent => {
                "a note story cannot hold a block, a page number or a reference".into()
            }
        }
    }
}
#[derive(Default)]
pub(super) struct NoteLayout {
    footnotes: Vec<Note>,
    endnotes: Vec<Note>,
    /// The height of the separator before the endnotes, which Word sets as a
    /// paragraph of the default style with its space after.
    endnote_band: f64,
    endnote_footer_heights: Vec<f64>,
    /// The geometry of the last section, in which the endnotes are set.
    endnote_geometry: PageGeometry,
    endnote_obstacle: Option<Obstacle>,
    pages: BTreeMap<usize, Vec<usize>>,
    assigned: BTreeSet<usize>,
    pub warnings: Vec<String>,
}
impl NoteLayout {
    pub fn prepare(
        sections: &mut [Section],
        total: usize,
        styles: &LayoutStyleResolver<'_>,
        measurer: &mut TextMeasurer<'_>,
    ) -> Result<Self, LayoutError> {
        let mut layout = Self::default();
        let separator = styles.paragraph(&empty_paragraph());
        layout.endnote_band = separator.line_height + separator.spacing_after.max(0.0);
        if let Some(last) = sections.last() {
            layout.endnote_geometry = PageGeometry::from_section(last)?;
            if layout.endnote_geometry.column_count > 1 {
                layout.endnote_obstacle = Some(Obstacle::LastSectionColumns);
            }
        }
        for section in sections.iter_mut() {
            let geometry = PageGeometry::from_section(section)?;
            let section_obstacle = (geometry.column_count > 1).then_some(Obstacle::SectionColumns);
            for (content, obstacle) in [
                (&mut section.content, section_obstacle),
                (&mut section.header.content, Some(Obstacle::Header)),
                (&mut section.footer.content, Some(Obstacle::Footer)),
            ] {
                layout.select(content, obstacle, false, styles, measurer, geometry)?;
            }
        }
        if let Some(last) = sections.last()
            && !layout.endnotes.is_empty()
        {
            layout.endnote_footer_heights = super::stories::footer_heights(
                last,
                layout.endnote_geometry,
                total,
                styles,
                measurer,
            )?;
        }
        Ok(layout)
    }
    fn select(
        &mut self,
        content: &mut [Content],
        obstacle: Option<Obstacle>,
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
                    Some(Obstacle::EnclosingNote),
                    true,
                    styles,
                    measurer,
                    geometry,
                )?;
                let (geometry, kind_obstacle) = if element.kind == ElementKind::Footnote {
                    (geometry, None)
                } else {
                    (self.endnote_geometry, self.endnote_obstacle)
                };
                let obstacle = obstacle
                    .or((!inline).then_some(Obstacle::OutsideParagraph))
                    .or(kind_obstacle)
                    .or((!stable_content(&element.children)).then_some(Obstacle::UnstableContent));
                if let Some(obstacle) = obstacle {
                    self.warnings.push(format!(
                        "native {} content is flattened inline because {}",
                        element.kind.as_text(),
                        obstacle.reason()
                    ));
                } else {
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
                }
                continue;
            }
            let (children_inline, visible) = child_flow(element.kind, inline);
            let inner_obstacle = if element.kind == ElementKind::Cell {
                Some(Obstacle::TableCell)
            } else if visible {
                None
            } else if children_inline {
                Some(Obstacle::OutsideParagraph)
            } else {
                Some(Obstacle::Container(element.kind))
            };
            self.select(
                &mut element.children,
                obstacle.or(inner_obstacle),
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
            .map(|number| self.footnotes[number - 1].height + self.footnotes[number - 1].after)
            .sum::<f64>()
    }
    /// Returns the lowest position of body text above the given footnotes on
    /// a page whose text area ends at `content_bottom`.
    fn body_bottom(&self, numbers: &[usize], content_bottom: f64) -> f64 {
        let height = self.stack_height(numbers);
        if height <= 0.0 {
            content_bottom
        } else {
            content_bottom - height - SEPARATOR_BAND - BODY_GAP
        }
    }
}
fn child_flow(kind: ElementKind, inline: bool) -> (bool, bool) {
    use ElementKind::*;
    match kind {
        Paragraph | Heading | Blockquote | Raw | Caption | ListItem | Term | Definition => {
            (true, true)
        }
        List | OrderedList | DefinitionList | Table | Row | Cell => (false, true),
        Strong | Emphasis | Underline | Strike | Highlight | SmallCaps | Subscript
        | Superscript | Span | Code | Link | Bookmark | Comment => (true, inline),
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
            element.is_block()
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
        source_location: None,
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
    let width = geometry.width - geometry.margin_left - geometry.margin_right;
    let options = LineOptions {
        width,
        base_style: style.text,
        base_line_height: Some(style.line_height),
        spacing: style.line_spacing,
        layout_kerning: false,
        word_exact_baseline: style.word_exact_baseline,
        whole_point_baseline: style.whole_point_baseline,
        ..Default::default()
    };
    let mut lines = super::plan_text(&runs, &options, measurer)?;
    for line in &mut lines {
        for fragment in &mut line.fragments {
            fragment.anchors.clear();
        }
    }
    let height = lines.iter().map(|line| line.height).sum();
    Ok(Note {
        lines,
        height,
        after: style.spacing_after.max(0.0),
        left: geometry.margin_left,
        width,
    })
}
