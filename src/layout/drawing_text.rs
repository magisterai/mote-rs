//! Shared paragraph measurement, fitting and clipping for drawing text.
use crate::drawings::paragraphs::DrawingParagraph;
use crate::{
    layout::{
        LayoutError, LineSpacing, ListMarker, ListMarkerKind, PageOperation, ParagraphText,
        PlacedTextLine, TextPage, TextRun,
    },
    measurement::TextMeasurer,
    model::{Properties, Value},
    properties::{object, text},
};
#[derive(Clone, Copy)]
pub(crate) enum TextFrame {
    Drawing,
    TableCell,
}
pub(crate) struct FittedText {
    pub page: TextPage,
    pub font_scale: f64,
    pub height_delta: f64,
}
fn points(value: Option<&Value>, fallback: f64) -> f64 {
    crate::drawings::length_in_drawing_units(value)
        .map(|value| value as f64 / 12700.0)
        .unwrap_or(fallback)
}
fn plan_paragraphs(
    paragraphs: &[DrawingParagraph],
    width: f64,
    alignment: &str,
    scale: f64,
    frame: TextFrame,
    measurer: &mut TextMeasurer<'_>,
) -> Result<(Vec<ParagraphText>, f64), LayoutError> {
    let mut output = Vec::new();
    let mut total = 0.0;
    for paragraph in paragraphs {
        let mut style = paragraph.style.clone();
        let mut runs = paragraph.runs.clone();
        if matches!(frame, TextFrame::TableCell) {
            style.indent_left = 0.0;
            style.indent_right = 0.0;
            style.first_line = 0.0;
        }
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
        let mut result = super::paragraphs::plan_resolved_paragraph(
            style,
            None,
            &runs,
            width
                + match frame {
                    TextFrame::Drawing => 0.75,
                    TextFrame::TableCell => 0.0,
                },
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
pub(crate) fn fit(
    paragraphs: &[DrawingParagraph],
    bounds: [f64; 4],
    attributes: &Properties,
    measurer: &mut TextMeasurer<'_>,
) -> Result<FittedText, LayoutError> {
    fit_with_frame(paragraphs, bounds, attributes, TextFrame::Drawing, measurer)
}
pub(crate) fn fit_with_frame(
    paragraphs: &[DrawingParagraph],
    bounds: [f64; 4],
    attributes: &Properties,
    frame: TextFrame,
    measurer: &mut TextMeasurer<'_>,
) -> Result<FittedText, LayoutError> {
    let mut paragraphs = paragraphs.to_vec();
    for paragraph in &mut paragraphs {
        if paragraph.marker.is_some() {
            add_native_marker(paragraph, measurer)?;
        }
    }
    let empty = Properties::new();
    let margin = object(attributes, "margin").unwrap_or(&empty);
    for name in ["left", "right", "top", "bottom"] {
        if let Some(value) = margin.get(name)
            && crate::drawings::length_in_drawing_units(Some(value)).is_none()
        {
            return Err(LayoutError::InvalidGeometry("invalid drawing text margin"));
        }
    }
    let left = points(margin.get("left"), 0.0);
    let top = points(margin.get("top"), 0.0);
    let mut content = [
        bounds[0] + left,
        bounds[1] + top,
        (bounds[2] - left - points(margin.get("right"), 0.0)).max(1.0),
        (bounds[3] - top - points(margin.get("bottom"), 0.0)).max(1.0),
    ];
    let initial_height = content[3];
    let alignment = text(attributes, "alignment");
    let (mut planned, mut total) =
        plan_paragraphs(&paragraphs, content[2], &alignment, 1.0, frame, measurer)?;
    let mut scale = 1.0;
    if text(attributes, "fit") == "shrink" && total > content[3] {
        let mut low = 0.1;
        let mut high = 1.0;
        for _ in 0..14 {
            let candidate = (low + high) / 2.0;
            let (_, height) = plan_paragraphs(
                &paragraphs,
                content[2],
                &alignment,
                candidate,
                frame,
                measurer,
            )?;
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
                    && plan_paragraphs(
                        &paragraphs,
                        content[2],
                        &alignment,
                        candidate,
                        frame,
                        measurer,
                    )?
                    .1 > content[3] + 0.25
                {
                    quantized = effective.floor();
                }
                if quantized > 0.0 {
                    scale = quantized / anchor;
                }
            }
        }
    } else if text(attributes, "fit") == "resize_shape" && total > content[3] {
        content[3] = total;
    }
    if scale != 1.0 {
        (planned, total) =
            plan_paragraphs(&paragraphs, content[2], &alignment, scale, frame, measurer)?;
    }
    let mut top = content[1]
        + match text(attributes, "vertical_alignment").as_str() {
            "center" | "middle" => ((content[3] - total) / 2.0).max(0.0),
            "bottom" => (content[3] - total).max(0.0),
            _ => 0.0,
        };
    let mut page = TextPage::default();
    page.operations.push(PageOperation::PushClip(content));
    for (paragraph, original) in planned.into_iter().zip(paragraphs) {
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
    Ok(FittedText {
        page,
        font_scale: scale,
        height_delta: (content[3] - initial_height).max(0.0),
    })
}

fn add_native_marker(
    paragraph: &mut DrawingParagraph,
    measurer: &mut TextMeasurer<'_>,
) -> Result<(), LayoutError> {
    let style = paragraph.style.text.clone();
    let mut prefix = Vec::new();
    let space = measurer
        .measure(&style.measurement, " ", false)?
        .measurement;
    if paragraph.task.is_some() {
        let mut spacer = TextRun::new(" ", style.clone());
        spacer.fixed_advance = Some(style.measurement.size_points + space.layout_width);
        spacer.preserve_space = true;
        prefix.push(spacer);
    } else if let Some(marker) = &paragraph.marker {
        let advance = measurer
            .measure(&style.measurement, &marker.text, false)?
            .measurement
            .layout_width;
        let hanging = if paragraph.style.first_line < 0.0 {
            -paragraph.style.first_line
        } else {
            advance + space.paint_width
        };
        prefix.push(TextRun::new(marker.text.clone(), style.clone()));
        let mut spacer = TextRun::new(" ", style);
        spacer.preserve_space = true;
        spacer.fixed_advance = Some((hanging - advance).max(space.paint_width));
        prefix.push(spacer);
    }
    prefix.append(&mut paragraph.runs);
    paragraph.runs = prefix;
    Ok(())
}
