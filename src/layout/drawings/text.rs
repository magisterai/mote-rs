//! Fits resolved drawing paragraphs into preset-specific text bounds.
use super::{
    geometry,
    paragraphs::{self, DrawingParagraph},
};
use crate::{
    layout::{
        LayoutError, LayoutStyleResolver, LineSpacing, ListMarker, ListMarkerKind, PageOperation,
        ParagraphText, PlacedTextLine, TextPage, TextStyle,
    },
    measurement::TextMeasurer,
    model::{Element, ElementKind, Properties},
    properties::{object, text},
};

fn planned(
    paragraphs: &[DrawingParagraph],
    width: f64,
    alignment: &str,
    scale: f64,
    measurer: &mut TextMeasurer<'_>,
) -> Result<(Vec<ParagraphText>, f64), LayoutError> {
    let mut output = Vec::new();
    let mut total = 0.0;
    for paragraph in paragraphs {
        let mut style = paragraph.style.clone();
        let mut runs = paragraph.runs.clone();
        if !alignment.is_empty() {
            style.alignment = alignment.into();
        }
        if scale != 1.0 {
            style.text.measurement.size_points *= scale;
            style.line_height *= scale;
            style.spacing_before *= scale;
            style.spacing_after *= scale;
            match &mut style.line_spacing {
                LineSpacing::Exact(points) | LineSpacing::AtLeast(points) => *points *= scale,
                _ => {}
            }
            for run in &mut runs {
                run.style.measurement.size_points *= scale;
                if let Some(advance) = &mut run.fixed_advance {
                    *advance *= scale;
                }
            }
        }
        let mut result = super::super::paragraphs::plan_resolved_paragraph(
            style,
            None,
            &runs,
            width + 0.75,
            measurer,
            false,
        )?;
        for line in &mut result.lines {
            for highlight in &mut line.highlights {
                highlight.top = 0.0;
                highlight.height = line.height;
            }
        }
        total += result.style.spacing_before
            + result.style.spacing_after
            + result.lines.iter().map(|line| line.height).sum::<f64>();
        output.push(result);
    }
    Ok((output, total))
}
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
        text(&attributes, "font").is_empty() && text(&attributes, "style").is_empty(),
        styles,
        measurer,
    )?;
    let mut bounds = geometry::text_rectangle(
        &text(&element.attributes, "kind"),
        geometry::rectangle(&element.attributes),
    );
    bounds[1] -= geometry::points(attributes.get("size"), 18.0).max(0.0) * 0.055;
    let empty = Properties::new();
    let margin = object(&attributes, "margin").unwrap_or(&empty);
    let left = geometry::points(margin.get("left"), 0.0);
    let top = geometry::points(margin.get("top"), 0.0);
    let mut content = [
        bounds[0] + left,
        bounds[1] + top,
        (bounds[2] - left - geometry::points(margin.get("right"), 0.0)).max(1.0),
        (bounds[3] - top - geometry::points(margin.get("bottom"), 0.0)).max(1.0),
    ];
    let alignment = text(&attributes, "alignment");
    let (_, total) = planned(&paragraphs, content[2], &alignment, 1.0, measurer)?;
    let mut scale = 1.0;
    if text(&attributes, "fit") == "shrink" && total > content[3] {
        let mut low = 0.1;
        let mut high = 1.0;
        for _ in 0..14 {
            let candidate = (low + high) / 2.0;
            let (_, height) = planned(&paragraphs, content[2], &alignment, candidate, measurer)?;
            if height <= content[3] {
                low = candidate
            } else {
                high = candidate
            }
        }
        scale = low;
        let anchor = paragraphs
            .iter()
            .flat_map(|paragraph| {
                std::iter::once(paragraph.style.text.measurement.size_points).chain(
                    paragraph
                        .runs
                        .iter()
                        .map(|run| run.style.measurement.size_points),
                )
            })
            .fold(0.0, f64::max);
        if anchor > 0.0 {
            let effective = anchor * scale;
            let mut quantized = effective.round();
            if quantized > 0.0 {
                let candidate = quantized / anchor;
                if candidate > scale
                    && planned(&paragraphs, content[2], &alignment, candidate, measurer)?.1
                        > content[3] + 0.25
                {
                    quantized = effective.floor();
                }
                if quantized > 0.0 {
                    scale = quantized / anchor;
                }
            }
        }
    } else if text(&attributes, "fit") == "resize_shape" && total > content[3] {
        content[3] = total;
    }
    let (planned, total) = planned(&paragraphs, content[2], &alignment, scale, measurer)?;
    let mut top = content[1]
        + match text(&attributes, "vertical_alignment").as_str() {
            "center" | "middle" => ((content[3] - total) / 2.0).max(0.0),
            "bottom" => (content[3] - total).max(0.0),
            _ => 0.0,
        };
    let mut page = TextPage::default();
    page.operations.push(PageOperation::PushClip(content));
    for (paragraph, original) in planned.into_iter().zip(&paragraphs) {
        top += paragraph.style.spacing_before;
        if let Some(checked) = original.task
            && let Some(line) = paragraph.lines.first()
        {
            page.operations
                .push(PageOperation::ListMarker(Box::new(ListMarker {
                    kind: ListMarkerKind::Task,
                    task_mark: crate::layout::TaskMark::Cross,
                    checked,
                    slot: [
                        content[0] + paragraph.first_frame.x,
                        top,
                        paragraph.style.text.measurement.size_points,
                        line.height,
                    ],
                    baseline: top + line.baseline,
                    font_size: paragraph.style.text.measurement.size_points,
                    text: None,
                })));
        }
        for mut line in paragraph.lines {
            let height = line.height;
            line.translate(content[0], top);
            page.push_line(PlacedTextLine { top, line });
            top += height;
        }
        top += paragraph.style.spacing_after;
    }
    page.operations.push(PageOperation::PopClip);
    Ok(page)
}
