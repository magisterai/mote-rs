use super::{Objects, RenderError, color, rounded};
use crate::layout::{LineCap, PaintedPath};
use pdf_writer::{Content, Name, Pdf, Ref};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct Opacities {
    pub entries: BTreeMap<(u8, u8), (String, Ref)>,
}
impl Opacities {
    fn name(&mut self, fill: u8, stroke: u8, document: &mut Pdf, objects: &mut Objects) -> String {
        self.entries
            .entry((fill, stroke))
            .or_insert_with(|| {
                let reference = objects.allocate();
                document
                    .ext_graphics(reference)
                    .non_stroking_alpha((f64::from(fill) / 255.0 * 1000.0).round() as f32 / 1000.0)
                    .stroking_alpha((f64::from(stroke) / 255.0 * 1000.0).round() as f32 / 1000.0);
                (format!("Opacity{fill}_{stroke}"), reference)
            })
            .0
            .clone()
    }
}
pub(super) fn draw(
    content: &mut Content,
    path: &PaintedPath,
    height: f64,
    document: &mut Pdf,
    objects: &mut Objects,
    opacities: &mut Opacities,
) -> Result<(), RenderError> {
    content.save_state();
    let name = opacities.name(
        path.fill.as_ref().map_or(255, |paint| paint.alpha),
        path.stroke
            .as_ref()
            .map_or(255, |stroke| stroke.paint.alpha),
        document,
        objects,
    );
    content.set_parameters(Name(name.as_bytes()));
    let construct = |content: &mut Content, filling: bool| {
        content.move_to(
            rounded(path.points[0][0]),
            rounded(height - path.points[0][1]),
        );
        for point in path.points.iter().skip(1) {
            if filling {
                content.line_to(
                    (point[0] * 100000.0).round() as f32 / 100000.0,
                    ((height - point[1]) * 100000.0).round() as f32 / 100000.0,
                );
            } else {
                content.line_to(rounded(point[0]), rounded(height - point[1]));
            }
        }
        if filling {
            content.line_to(
                (path.points[0][0] * 100000.0).round() as f32 / 100000.0,
                ((height - path.points[0][1]) * 100000.0).round() as f32 / 100000.0,
            );
        } else if path.closed {
            content.close_path();
        }
    };
    if path.closed
        && let Some(fill) = &path.fill
    {
        let [red, green, blue] = color(&fill.color)?
            .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content.set_fill_rgb(red, green, blue);
        construct(content, true);
        content.fill_nonzero();
    }
    if let Some(stroke) = &path.stroke {
        let [red, green, blue] = color(&stroke.paint.color)?
            .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content
            .set_stroke_rgb(red, green, blue)
            .set_line_width(rounded(stroke.width))
            .set_dash_pattern(stroke.dash.iter().map(|value| rounded(*value)), 0.0)
            .set_line_cap(match stroke.cap {
                LineCap::Flat => pdf_writer::types::LineCapStyle::ButtCap,
                LineCap::Round => pdf_writer::types::LineCapStyle::RoundCap,
                LineCap::Square => pdf_writer::types::LineCapStyle::ProjectingSquareCap,
            })
            .set_line_join(if path.closed {
                pdf_writer::types::LineJoinStyle::RoundJoin
            } else {
                pdf_writer::types::LineJoinStyle::MiterJoin
            });
        construct(content, false);
        content.stroke();
    }
    content.restore_state();
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_rectangle(
    content: &mut Content,
    bounds: [f64; 4],
    fill: Option<&crate::layout::PathPaint>,
    stroke: Option<&crate::layout::PathStroke>,
    height: f64,
    document: &mut Pdf,
    objects: &mut Objects,
    opacities: &mut Opacities,
) -> Result<(), RenderError> {
    content.save_state();
    let name = opacities.name(
        fill.map_or(255, |paint| paint.alpha),
        stroke.map_or(255, |stroke| stroke.paint.alpha),
        document,
        objects,
    );
    content.set_parameters(Name(name.as_bytes()));
    let rectangle = |content: &mut Content| {
        content.rect(
            rounded(bounds[0]),
            rounded(height - bounds[1]),
            rounded(bounds[2]),
            -rounded(bounds[3]),
        );
    };
    if let Some(fill) = fill {
        let [red, green, blue] = color(&fill.color)?
            .map(|value| ((f64::from(value) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content.set_fill_rgb(red, green, blue);
        rectangle(content);
        content.fill_nonzero();
    }
    if let Some(stroke) = stroke {
        let [red, green, blue] = color(&stroke.paint.color)?
            .map(|value| ((f64::from(value) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content
            .set_stroke_rgb(red, green, blue)
            .set_line_width(rounded(stroke.width));
        content.set_dash_pattern(stroke.dash.iter().map(|value| rounded(*value)), 0.0);
        rectangle(content);
        content.stroke();
    }
    content.restore_state();
    Ok(())
}
