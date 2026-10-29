use super::{RenderError, color};
use crate::layout::Rule;
use tiny_skia::{Mask, Pixmap};
pub(super) fn paint(
    pixels: &mut Pixmap,
    rule: &Rule,
    dash: &[f64],
    round_cap: bool,
    scale: f64,
    clip: Option<&Mask>,
) -> Result<(), RenderError> {
    if rule.start[0] != rule.end[0] && rule.start[1] != rule.end[1] {
        let mut path = tiny_skia::PathBuilder::new();
        path.move_to(
            (rule.start[0] * scale) as f32,
            (rule.start[1] * scale) as f32,
        );
        path.line_to((rule.end[0] * scale) as f32, (rule.end[1] * scale) as f32);
        if let Some(path) = path.finish() {
            pixels.stroke_path(
                &path,
                &super::paint(&rule.color, true)?,
                &tiny_skia::Stroke {
                    width: (rule.width * scale).max(1.0) as f32,
                    line_cap: if round_cap {
                        tiny_skia::LineCap::Round
                    } else {
                        tiny_skia::LineCap::Butt
                    },
                    dash: tiny_skia::StrokeDash::new(
                        dash.iter().map(|value| (value * scale) as f32).collect(),
                        0.0,
                    ),
                    ..Default::default()
                },
                tiny_skia::Transform::identity(),
                clip,
            );
        }
        return Ok(());
    }
    if dash.is_empty() && (rule.start[0] == rule.end[0] || rule.start[1] == rule.end[1]) {
        let horizontal = rule.start[1] == rule.end[1];
        let thickness = (rule.width * scale).round().max(1.0) as i64;
        let first = rule.start.map(|value| (value * scale).round() as i64);
        let last = rule.end.map(|value| (value * scale).round() as i64);
        let bounds = if horizontal {
            [
                first[0].min(last[0]),
                first[1] - thickness / 2,
                first[0].max(last[0]).max(first[0] + 1),
                first[1] - thickness / 2 + thickness,
            ]
        } else {
            [
                first[0] - thickness / 2,
                first[1].min(last[1]),
                first[0] - thickness / 2 + thickness,
                first[1].max(last[1]).max(first[1] + 1),
            ]
        };
        let [red, green, blue] = color(&rule.color)?;
        let width = pixels.width() as usize;
        for row in bounds[1].max(0)..bounds[3].min(pixels.height() as i64) {
            for column in bounds[0].max(0)..bounds[2].min(pixels.width() as i64) {
                let offset = row as usize * width + column as usize;
                if clip.is_none_or(|mask| mask.data()[offset] > 0) {
                    pixels.data_mut()[offset * 4..offset * 4 + 4]
                        .copy_from_slice(&[red, green, blue, 255]);
                }
            }
        }
        return Ok(());
    }
    let length = (rule.end[0] - rule.start[0]).hypot(rule.end[1] - rule.start[1]);
    let mut position = 0.0;
    let mut index = 0;
    while position < length {
        let segment = dash
            .get(index % dash.len().max(1))
            .copied()
            .unwrap_or(length);
        let next = (position + segment).min(length);
        if index.is_multiple_of(2) {
            let start = std::array::from_fn(|axis| {
                (rule.start[axis] + (rule.end[axis] - rule.start[axis]) * position / length) * scale
            });
            let end = std::array::from_fn(|axis| {
                (rule.start[axis] + (rule.end[axis] - rule.start[axis]) * next / length) * scale
            });
            super::crisp_axis_aligned(
                pixels,
                start,
                end,
                rule.width * scale,
                color(&rule.color)?,
                clip,
            );
        }
        position = next;
        index += 1;
    }
    Ok(())
}
