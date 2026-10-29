//! Header/footer text uses full-width story bands independently of body columns.
use super::{DocumentTextPlan, LayoutError, LayoutStyleResolver, PlacedTextLine, TextLine};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Document, Element, ElementKind, Section, Value},
};
struct StoryLine {
    line: TextLine,
    leading: f64,
    trailing: f64,
}
impl StoryLine {
    fn height(&self) -> f64 {
        self.line.height + self.leading + self.trailing
    }
}

pub(super) fn place(
    plan: &mut DocumentTextPlan,
    sections: &[Section],
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
    reference_pages: &std::collections::BTreeMap<String, usize>,
) -> Result<(), LayoutError> {
    let mut styles = LayoutStyleResolver::new(document);
    styles.reference_pages = reference_pages.clone();
    let total = plan.pages.len().max(1);
    for page in &mut plan.pages {
        let geometry = page.geometry.unwrap_or(plan.geometry);
        let section = &sections[page.section_index];
        for (header, story) in [(true, &section.header), (false, &section.footer)] {
            let name = if header { "header" } else { "footer" };
            let style_name = if header { "Header" } else { "Footer" };
            let mut content = story.content.clone();
            super::fields::resolve(
                &mut content,
                page.page_number,
                total,
                None,
                &Default::default(),
            );
            let mut lines: Vec<StoryLine> = Vec::new();
            for child in &content {
                let mut paragraph = match child {
                    Content::Element(element) if element.kind == ElementKind::Paragraph => {
                        element.clone()
                    }
                    _ => Element {
                        kind: ElementKind::Paragraph,
                        attributes: Default::default(),
                        children: vec![child.clone()],
                    },
                };
                if crate::properties::text(&paragraph.attributes, "style").is_empty() {
                    paragraph
                        .attributes
                        .insert("style".into(), Value::String(style_name.into()));
                }
                let mut style = styles.paragraph(&paragraph);
                style.indent_left = 0.0;
                style.indent_right = 0.0;
                style.first_line = 0.0;
                let runs = super::resolve_inline(&paragraph.children, &style.text, &styles);
                let before = style.spacing_before;
                let after = style.spacing_after;
                let paragraph = super::paragraphs::plan_resolved_paragraph(
                    style,
                    None,
                    &runs,
                    geometry.content_width(),
                    measurer,
                    true,
                )?;
                let count = paragraph.lines.len();
                for (index, line) in paragraph.lines.into_iter().enumerate() {
                    let leading = if index == 0 { before } else { 0.0 };
                    let trailing = if index + 1 == count { after } else { 0.0 };
                    let leading = if let Some(previous) = lines.last() {
                        (leading - previous.trailing).max(0.0)
                    } else {
                        leading
                    };
                    lines.push(StoryLine {
                        line,
                        leading,
                        trailing,
                    });
                }
            }
            let available = if header {
                geometry.margin_top - geometry.header_space
            } else {
                geometry.margin_bottom - geometry.footer_space
            };
            let mut height = 0.0;
            let count = lines
                .iter()
                .take_while(|line| {
                    height += line.height();
                    height <= available + 0.001
                })
                .count();
            if count < lines.len() {
                plan.warnings.push(format!(
                    "{name} content exceeds its reserved area; trailing lines are omitted"
                ));
            }
            lines.truncate(count);
            let height: f64 = lines.iter().map(StoryLine::height).sum();
            let mut top = if header {
                geometry.header_space
            } else {
                geometry.height - geometry.footer_space - height
            };
            for mut line in lines {
                let height = line.height();
                line.line
                    .translate(geometry.margin_left, top + line.leading);
                for fragment in &mut line.line.fragments {
                    fragment.anchors.clear();
                }
                page.push_line(PlacedTextLine {
                    top: top + line.leading,
                    line: line.line,
                });
                top += height;
            }
        }
    }
    Ok(())
}
