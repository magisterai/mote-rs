use super::{
    FlowArea, FlowCursor, FlowOptions, FlowStep, LayoutError, LayoutStyleResolver, LineBreak,
    LineFrame, PageGeometry, ParagraphText, TextLine, minimum_start_height, plan_paragraph_flow,
    plan_paragraph_text, style_values,
};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Document, Element, ElementKind, Properties, Value},
};

/// Absolute text positions. Painting and non-text block layout are separate stages.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedTextLine {
    pub top: f64,
    pub line: TextLine,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextPage {
    pub lines: Vec<PlacedTextLine>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentTextPlan {
    pub geometry: PageGeometry,
    pub pages: Vec<TextPage>,
}

enum Block {
    Paragraph(Box<ParagraphText>),
    Break(String),
}

/// Assemble single-column paragraph, heading and explicit-break text geometry.
/// Returns an error for block kinds or column balancing outside this boundary.
/// Colors and inline decorations remain attached to text styles; no paint output is produced.
pub fn plan_document_text(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
) -> Result<DocumentTextPlan, LayoutError> {
    let geometry = PageGeometry::from_document(document)?;
    if geometry.column_count != 1 {
        return Err(LayoutError::Unsupported(
            "document text planning does not yet balance multiple columns".into(),
        ));
    }
    let styles = LayoutStyleResolver::new(document);
    let mut blocks = Vec::new();
    let mut flow_start = true;
    for content in &document.content {
        let Content::Element(element) = content else {
            return Err(LayoutError::Unsupported(
                "document text planning requires evaluated block content".into(),
            ));
        };
        match element.kind {
            ElementKind::Paragraph | ElementKind::Heading => {
                if element.attributes.contains_key("numbering")
                    || element.attributes.contains_key("numbered")
                {
                    return Err(LayoutError::Unsupported(
                        "native heading numbering is not yet planned".into(),
                    ));
                }
                let mut element = element.clone();
                if flow_start
                    && element.kind == ElementKind::Heading
                    && !explicit_before(&element.attributes)
                {
                    element
                        .attributes
                        .insert("spacing_before".into(), Value::Number(0.0));
                }
                let mut paragraph = plan_paragraph_text(
                    &element,
                    geometry.column_width(),
                    &styles,
                    measurer,
                    true,
                )?;
                if paragraph.is_empty {
                    if paragraph.style.line_height <= 0.0 {
                        return Err(LayoutError::InvalidGeometry(
                            "blank paragraph line height must be positive",
                        ));
                    }
                    for line in &mut paragraph.lines {
                        line.width = paragraph.first_frame.width;
                        line.height = paragraph.style.line_height;
                        line.baseline = paragraph.style.line_height * 0.8;
                    }
                }
                blocks.push(Block::Paragraph(Box::new(paragraph)));
                flow_start = false;
            }
            ElementKind::Break => {
                let kind = style_values::text(&element.attributes, "type");
                flow_start = kind != "line";
                blocks.push(Block::Break(kind));
            }
            _ => {
                return Err(LayoutError::Unsupported(format!(
                    "native document text planning does not yet support {}",
                    element.kind.as_text()
                )));
            }
        }
    }
    if blocks.is_empty() {
        return Ok(DocumentTextPlan {
            geometry,
            pages: Vec::new(),
        });
    }
    let mut pages = vec![TextPage::default()];
    let mut cursor = FlowCursor {
        page_number: 1,
        current_top: geometry.margin_top,
        y: geometry.margin_top,
        ..Default::default()
    };
    let mut pending_spacing: f64 = 0.0;
    for (index, block) in blocks.iter().enumerate() {
        match block {
            Block::Break(kind) => {
                if kind == "line" {
                    let style = styles.paragraph(&Element {
                        kind: ElementKind::Paragraph,
                        attributes: Properties::new(),
                        children: Vec::new(),
                    });
                    if !style.line_height.is_finite() || style.line_height <= 0.0 {
                        return Err(LayoutError::InvalidGeometry(
                            "hard-break line height must be positive and finite",
                        ));
                    }
                    if pending_spacing > 0.0 {
                        ensure_space(&mut cursor, &mut pages, geometry, pending_spacing);
                        cursor.y += pending_spacing;
                    }
                    ensure_space(&mut cursor, &mut pages, geometry, style.line_height);
                    let frame = frame_for(&style, geometry.column_width());
                    pages[cursor.page_number - 1].lines.push(PlacedTextLine {
                        top: cursor.y,
                        line: TextLine {
                            x: geometry.margin_left + frame.x,
                            baseline: cursor.y + style.line_height * 0.8,
                            height: style.line_height,
                            width: frame.width,
                            line_break: LineBreak::Final,
                            fragments: Vec::new(),
                            highlights: Vec::new(),
                            decorations: Vec::new(),
                        },
                    });
                    cursor.y += style.line_height;
                    cursor.body_started = true;
                } else {
                    new_page(&mut cursor, &mut pages, geometry);
                }
                pending_spacing = 0.0;
            }
            Block::Paragraph(paragraph) => {
                if paragraph.style.page_break_before
                    && (cursor.body_started || cursor.y > geometry.margin_top)
                {
                    new_page(&mut cursor, &mut pages, geometry);
                    pending_spacing = 0.0;
                }
                let leading = pending_spacing.max(paragraph.style.spacing_before);
                let minimum = keep_height(
                    &blocks[index..],
                    geometry.content_bottom() - geometry.margin_top,
                );
                let heights: Vec<_> = paragraph.lines.iter().map(|line| line.height).collect();
                let frames: Vec<_> = (0..heights.len())
                    .map(|index| {
                        if index == 0 {
                            paragraph.first_frame
                        } else {
                            paragraph.continuation_frame
                        }
                    })
                    .collect();
                let flow = plan_paragraph_flow(
                    cursor,
                    &heights,
                    &frames,
                    &FlowOptions {
                        leading_space: leading,
                        keep_fit_leading_space: leading,
                        minimum_start_space: minimum,
                        keep_together: paragraph.style.keep_together,
                        widow_control: paragraph.style.widow_control,
                        ..Default::default()
                    },
                    |_| FlowArea::from(geometry),
                )?;
                for step in flow.steps {
                    if let FlowStep::Line(placement) = step {
                        while pages.len() < placement.page_number {
                            pages.push(TextPage::default());
                        }
                        let mut line = paragraph.lines[placement.line_index].clone();
                        let relative_frame = if placement.line_index == 0 {
                            paragraph.first_frame
                        } else {
                            paragraph.continuation_frame
                        };
                        let horizontal = placement.frame.x - relative_frame.x;
                        line.translate(horizontal, placement.top);
                        pages[placement.page_number - 1].lines.push(PlacedTextLine {
                            top: placement.top,
                            line,
                        });
                    }
                }
                cursor = flow.final_cursor;
                pending_spacing = paragraph.style.spacing_after.max(0.0);
            }
        }
    }
    Ok(DocumentTextPlan { geometry, pages })
}
fn explicit_before(properties: &Properties) -> bool {
    properties.contains_key("spacing_before")
        || matches!(properties.get("spacing"),Some(Value::Object(spacing))if spacing.contains_key("before")||spacing.contains_key("spacing_before"))
}
fn frame_for(style: &super::ParagraphStyle, width: f64) -> LineFrame {
    let available = (width - style.indent_left - style.indent_right).max(width.min(12.0));
    let origin = style.indent_left.min(width - available);
    LineFrame {
        x: origin + style.first_line,
        width: available - style.first_line,
    }
}
fn new_page(cursor: &mut FlowCursor, pages: &mut Vec<TextPage>, geometry: PageGeometry) {
    pages.push(TextPage::default());
    *cursor = FlowCursor {
        page_number: pages.len(),
        current_top: geometry.margin_top,
        y: geometry.margin_top,
        ..Default::default()
    };
}
fn ensure_space(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    geometry: PageGeometry,
    height: f64,
) {
    if cursor.y + height > geometry.content_bottom() {
        new_page(cursor, pages, geometry);
    }
}
fn keep_height(blocks: &[Block], maximum: f64) -> f64 {
    let Some(Block::Paragraph(first)) = blocks.first() else {
        return 0.0;
    };
    if !first.style.keep_with_next {
        return 0.0;
    }
    let mut count = 1;
    let mut required: f64 = first.lines.iter().map(|line| line.height).sum();
    let mut previous_after = first.style.spacing_after;
    for block in &blocks[1..] {
        let Block::Paragraph(paragraph) = block else {
            break;
        };
        if paragraph.style.page_break_before {
            break;
        }
        count += 1;
        required += previous_after.max(paragraph.style.spacing_before);
        let heights: Vec<_> = paragraph.lines.iter().map(|line| line.height).collect();
        if paragraph.style.keep_with_next {
            required += heights.iter().sum::<f64>();
            previous_after = paragraph.style.spacing_after;
        } else {
            required += minimum_start_height(
                &heights,
                paragraph.style.widow_control,
                paragraph.style.keep_together,
            );
            break;
        }
    }
    if count < 2 || first.style.spacing_before + required > maximum {
        0.0
    } else {
        required
    }
}
