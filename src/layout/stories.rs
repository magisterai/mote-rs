//! Header/footer text uses full-width story bands independently of body columns.
use super::{
    DocumentTextPlan, LayoutError, LayoutStyleResolver, PlacedTextLine, TextLine,
    spacing::collapsed_spacing, tolerances::FIT_TOLERANCE,
};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Document, Element, ElementKind, Section, Unit, Value},
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
    styles.measure_font_line_heights(measurer);
    let total = plan.pages.len().max(1);
    for page in &mut plan.pages {
        let geometry = page.geometry.unwrap_or(plan.geometry);
        let section = &sections[page.section_index];
        for (header, story) in [(true, &section.header), (false, &section.footer)] {
            let name = if header { "header" } else { "footer" };
            let style_name = if header { "Header" } else { "Footer" };
            let mut lines = measure(
                &story.content,
                style_name,
                geometry,
                page.page_number,
                total,
                &styles,
                measurer,
            )?;
            let available = if header {
                geometry.margin_top - geometry.header_space
            } else {
                geometry.margin_bottom - geometry.footer_space
            };
            let count = fitting_line_count(&lines, available);
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

/// Word begins the body below a header taller than its margin and ends it above
/// such a footer, as its export of the headers-and-footers probe shows. The
/// function raises the height of each story of each section to the height of
/// its lines on the first page of the section, so that the page geometry
/// reserves the story.
pub(super) fn reserve_heights(
    sections: &mut [Section],
    total: usize,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
) -> Result<(), LayoutError> {
    for section in sections {
        let geometry = super::PageGeometry::from_section(section)?;
        let start = crate::properties::number(&section.page_numbering, "start")
            .unwrap_or(1.0)
            .max(1.0) as usize;
        for (story, style_name) in [
            (&mut section.header, "Header"),
            (&mut section.footer, "Footer"),
        ] {
            let measured: f64 = measure(
                &story.content,
                style_name,
                geometry,
                start,
                total,
                styles,
                measurer,
            )?
            .iter()
            .map(StoryLine::height)
            .sum();
            let declared = story
                .properties
                .get("height")
                .and_then(super::style_values::points)
                .unwrap_or(0.0);
            if measured > declared {
                // The page geometry holds whole twips, so the height rises to
                // the next whole twip lest the last line lose its room.
                story.properties.insert(
                    "height".into(),
                    Value::Length {
                        value: (measured * 20.0).ceil() / 20.0,
                        unit: Unit::Point,
                    },
                );
            }
        }
    }
    Ok(())
}

fn fitting_line_count(lines: &[StoryLine], available: f64) -> usize {
    let mut height = 0.0;
    lines
        .iter()
        .take_while(|line| {
            height += line.height();
            height <= available + FIT_TOLERANCE
        })
        .count()
}

pub(super) fn footer_heights(
    section: &Section,
    geometry: super::PageGeometry,
    total: usize,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
) -> Result<Vec<f64>, LayoutError> {
    let declared = section
        .footer
        .properties
        .get("height")
        .and_then(super::style_values::points)
        .unwrap_or(0.0);
    let start = crate::properties::number(&section.page_numbering, "start")
        .unwrap_or(1.0)
        .max(1.0) as usize;
    (0..total.max(1))
        .map(|index| {
            let lines = measure(
                &section.footer.content,
                "Footer",
                geometry,
                start + index,
                total.max(1),
                styles,
                measurer,
            )?;
            let count = fitting_line_count(&lines, geometry.margin_bottom - geometry.footer_space);
            Ok(declared.max(lines[..count].iter().map(StoryLine::height).sum()))
        })
        .collect()
}

fn measure(
    content: &[Content],
    style_name: &str,
    geometry: super::PageGeometry,
    page_number: usize,
    total: usize,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
) -> Result<Vec<StoryLine>, LayoutError> {
    let mut content = content.to_vec();
    super::fields::resolve(&mut content, page_number, total, None, &Default::default());
    let mut lines: Vec<StoryLine> = Vec::new();
    for child in &content {
        let mut paragraph = match child {
            Content::Element(element) if element.kind == ElementKind::Paragraph => element.clone(),
            _ => Element {
                source_name: None,
                source_location: None,
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
        let runs = super::resolve_inline(&paragraph.children, &style.text, styles);
        let before = style.spacing_before;
        let after = style.spacing_after;
        let borders = styles.borders(&paragraph);
        let paragraph = super::paragraphs::plan_resolved_paragraph(
            style,
            borders,
            &runs,
            geometry.content_width(),
            measurer,
            false,
            true,
        )?;
        let count = paragraph.lines.len();
        for (index, line) in paragraph.lines.into_iter().enumerate() {
            let leading = if index == 0 { before } else { 0.0 };
            let trailing = if index + 1 == count { after } else { 0.0 };
            // The space after the paragraph above already lies below its last
            // line, so the line takes only the excess of the collapsed space.
            let leading = if let Some(previous) = lines.last() {
                collapsed_spacing(previous.trailing, leading) - previous.trailing
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
    Ok(lines)
}
