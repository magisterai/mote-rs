use super::{RenderError, color};
use crate::layout::{LineCap, PaintedPath, PathPaint, PathStroke};
use tiny_skia::{FillRule, Mask, Paint, PathBuilder, Pixmap, Transform};

fn paint(value: &PathPaint) -> Result<Paint<'static>, RenderError> {
    let [red, green, blue] = color(&value.color)?;
    let mut paint = Paint {
        anti_alias: true,
        ..Default::default()
    };
    paint.set_color_rgba8(red, green, blue, value.alpha);
    Ok(paint)
}
pub(super) fn draw(
    pixels: &mut Pixmap,
    path: &PaintedPath,
    scale: f64,
    clip: Option<&Mask>,
) -> Result<(), RenderError> {
    let mut builder = PathBuilder::new();
    for (index, point) in path.points.iter().enumerate() {
        let horizontal = (point[0] * scale) as f32;
        let vertical = (point[1] * scale) as f32;
        if index == 0 {
            builder.move_to(horizontal, vertical)
        } else {
            builder.line_to(horizontal, vertical)
        }
    }
    if path.closed {
        builder.close();
    }
    let Some(geometry) = builder.finish() else {
        return Ok(());
    };
    if path.closed
        && let Some(fill) = &path.fill
    {
        pixels.fill_path(
            &geometry,
            &paint(fill)?,
            FillRule::Winding,
            Transform::identity(),
            clip,
        );
    }
    if let Some(stroke) = &path.stroke {
        if !path.closed
            && path.points.len() == 2
            && stroke.dash.is_empty()
            && (path.points[0][0] == path.points[1][0] || path.points[0][1] == path.points[1][1])
        {
            return crisp(pixels, path.points[0], path.points[1], stroke, scale, clip);
        }
        let segments = path.points.windows(2).map(|pair| [pair[0], pair[1]]).chain(
            path.closed
                .then(|| [*path.points.last().unwrap(), path.points[0]]),
        );
        if stroke.dash.is_empty() {
            // The segments meet in round joins; the free ends of an open path
            // take the cap of the stroke.
            let count = path.points.len() - 1 + usize::from(path.closed);
            let mut builder = PathBuilder::new();
            for (index, ends) in segments.enumerate() {
                let cap = |free: bool| {
                    if free && !path.closed {
                        stroke.cap
                    } else {
                        LineCap::Round
                    }
                };
                segment(
                    &mut builder,
                    ends,
                    stroke.width,
                    scale,
                    [cap(index == 0), cap(index + 1 == count)],
                );
            }
            fill_stroke(pixels, builder, &stroke.paint, clip)?;
        } else {
            for [start, end] in segments {
                dashed(pixels, start, end, stroke, scale, clip)?;
            }
        }
    }
    Ok(())
}

fn fill_stroke(
    pixels: &mut Pixmap,
    builder: PathBuilder,
    value: &PathPaint,
    clip: Option<&Mask>,
) -> Result<(), RenderError> {
    if let Some(path) = builder.finish() {
        pixels.fill_path(
            &path,
            &paint(value)?,
            FillRule::Winding,
            Transform::identity(),
            clip,
        );
    }
    Ok(())
}
/// Adds the outline of a stroke of `width` from `start` to `end`. An end with
/// a round cap is closed by a half circle, an end with a square cap reaches half
/// the width beyond its point, and an end with a flat cap stops at its point.
fn segment(
    builder: &mut PathBuilder,
    [start, end]: [[f64; 2]; 2],
    width: f64,
    scale: f64,
    [start_cap, end_cap]: [LineCap; 2],
) {
    let start = start.map(|value| value * scale);
    let end = end.map(|value| value * scale);
    let length = (end[0] - start[0]).hypot(end[1] - start[1]);
    if length == 0.0 {
        return;
    }
    let direction = [(end[0] - start[0]) / length, (end[1] - start[1]) / length];
    let radius = (width * scale / 2.0).max(0.5);
    let control = radius * 0.5522847498307936;
    let point = |along: f64, across: f64| {
        [
            (start[0] + direction[0] * along - direction[1] * across) as f32,
            (start[1] + direction[1] * along + direction[0] * across) as f32,
        ]
    };
    let reach = |cap: LineCap| if cap == LineCap::Square { radius } else { 0.0 };
    let (back, ahead) = (-reach(start_cap), length + reach(end_cap));
    let first = point(back, radius);
    builder.move_to(first[0], first[1]);
    let last = point(ahead, radius);
    builder.line_to(last[0], last[1]);
    if end_cap == LineCap::Round {
        for controls in [
            [
                [length + control, radius],
                [length + radius, control],
                [length + radius, 0.0],
            ],
            [
                [length + radius, -control],
                [length + control, -radius],
                [length, -radius],
            ],
        ] {
            let [first, second, last] = controls.map(|value| point(value[0], value[1]));
            builder.cubic_to(first[0], first[1], second[0], second[1], last[0], last[1]);
        }
    } else {
        let corner = point(ahead, -radius);
        builder.line_to(corner[0], corner[1]);
    }
    let corner = point(back, -radius);
    builder.line_to(corner[0], corner[1]);
    if start_cap == LineCap::Round {
        for controls in [
            [[-control, -radius], [-radius, -control], [-radius, 0.0]],
            [[-radius, control], [-control, radius], [0.0, radius]],
        ] {
            let [first, second, last] = controls.map(|value| point(value[0], value[1]));
            builder.cubic_to(first[0], first[1], second[0], second[1], last[0], last[1]);
        }
    }
    builder.close();
}
fn dashed(
    pixels: &mut Pixmap,
    start: [f64; 2],
    end: [f64; 2],
    stroke: &PathStroke,
    scale: f64,
    clip: Option<&Mask>,
) -> Result<(), RenderError> {
    let length = (end[0] - start[0]).hypot(end[1] - start[1]);
    for (position, next) in super::dashes::segments(length, &stroke.dash)? {
        let first =
            std::array::from_fn(|axis| start[axis] + (end[axis] - start[axis]) * position / length);
        let last =
            std::array::from_fn(|axis| start[axis] + (end[axis] - start[axis]) * next / length);
        if first[0] == last[0] || first[1] == last[1] {
            crisp(pixels, first, last, stroke, scale, clip)?;
        } else {
            let mut builder = PathBuilder::new();
            segment(
                &mut builder,
                [first, last],
                stroke.width,
                scale,
                [stroke.cap; 2],
            );
            fill_stroke(pixels, builder, &stroke.paint, clip)?;
        }
    }
    Ok(())
}
fn crisp(
    pixels: &mut Pixmap,
    start: [f64; 2],
    end: [f64; 2],
    stroke: &PathStroke,
    scale: f64,
    clip: Option<&Mask>,
) -> Result<(), RenderError> {
    let source = color(&stroke.paint.color)?;
    let steps =
        (((end[0] - start[0]).abs().max((end[1] - start[1]).abs())) * scale).ceil() as usize;
    if steps == 0 {
        return Ok(());
    }
    let radius = (stroke.width * scale / 2.0).max(0.5);
    if stroke.cap != LineCap::Round {
        // A flat or square end is the side of a rectangle.
        let reach = if stroke.cap == LineCap::Square {
            radius
        } else {
            0.0
        };
        let (first, last) = (
            start.map(|value| value * scale),
            end.map(|value| value * scale),
        );
        let horizontal = first[1] == last[1];
        let along = (
            first[0].min(last[0]),
            first[0].max(last[0]),
            first[1].min(last[1]),
            first[1].max(last[1]),
        );
        let [left, right, top, bottom] = if horizontal {
            [
                along.0 - reach,
                along.1 + reach,
                first[1] - radius,
                first[1] + radius,
            ]
        } else {
            [
                first[0] - radius,
                first[0] + radius,
                along.2 - reach,
                along.3 + reach,
            ]
        };
        for row in
            (top.round() as i64).max(0)..(bottom.round() as i64).min(i64::from(pixels.height()))
        {
            for column in
                (left.round() as i64).max(0)..(right.round() as i64).min(i64::from(pixels.width()))
            {
                blend(
                    pixels,
                    row as usize * pixels.width() as usize + column as usize,
                    source,
                    stroke.paint.alpha,
                    clip,
                );
            }
        }
        return Ok(());
    }
    for step in 0..=steps {
        let center: [f64; 2] = std::array::from_fn(|axis| {
            (start[axis] + (end[axis] - start[axis]) * step as f64 / steps as f64) * scale
        });
        for row in ((center[1] - radius).round() as i64).max(0)
            ..=((center[1] + radius).round() as i64).min(i64::from(pixels.height()) - 1)
        {
            for column in ((center[0] - radius).round() as i64).max(0)
                ..=((center[0] + radius).round() as i64).min(i64::from(pixels.width()) - 1)
            {
                if (column as f64 + 0.5 - center[0]).powi(2)
                    + (row as f64 + 0.5 - center[1]).powi(2)
                    > radius * radius
                {
                    continue;
                }
                let offset = row as usize * pixels.width() as usize + column as usize;
                blend(pixels, offset, source, stroke.paint.alpha, clip);
            }
        }
    }
    Ok(())
}
/// Lays a colour of the given alpha over one pixel, unless the clip hides it.
fn blend(pixels: &mut Pixmap, offset: usize, source: [u8; 3], alpha: u8, clip: Option<&Mask>) {
    if clip.is_some_and(|mask| mask.data()[offset] == 0) {
        return;
    }
    let alpha = u32::from(alpha);
    let inverse = 255 - alpha;
    let target = &mut pixels.data_mut()[offset * 4..offset * 4 + 4];
    for channel in 0..3 {
        target[channel] =
            ((u32::from(source[channel]) * alpha + u32::from(target[channel]) * inverse + 127)
                / 255) as u8;
    }
    target[3] = 255;
}

pub(super) fn draw_rectangle(
    pixels: &mut Pixmap,
    bounds: [f64; 4],
    fill: Option<&PathPaint>,
    stroke: Option<&PathStroke>,
    scale: f64,
    clip: Option<&Mask>,
) -> Result<(), RenderError> {
    let fill_rectangle =
        |pixels: &mut Pixmap, bounds: [f64; 4], value: &PathPaint| -> Result<(), RenderError> {
            let [left, top, width, height] = bounds;
            let [left, top, right, bottom] =
                [left, top, left + width, top + height].map(|value| (value * scale).round() as f32);
            if right <= left || bottom <= top {
                return Ok(());
            }
            if let Some(rectangle) = tiny_skia::Rect::from_ltrb(left, top, right, bottom) {
                pixels.fill_rect(rectangle, &paint(value)?, Transform::identity(), clip);
            }
            Ok(())
        };
    if let Some(fill) = fill {
        fill_rectangle(pixels, bounds, fill)?;
    }
    if let Some(stroke) = stroke {
        let [left, top, width, height] = bounds;
        if stroke.dash.is_empty() {
            let line = stroke.width.max(1.0 / scale);
            for rectangle in [
                [left, top, width, line],
                [left, top + height - line, width, line],
                [left, top, line, height],
                [left + width - line, top, line, height],
            ] {
                fill_rectangle(pixels, rectangle, &stroke.paint)?;
            }
        } else {
            for [start, end] in [
                [[left, top], [left + width, top]],
                [[left + width, top], [left + width, top + height]],
                [[left + width, top + height], [left, top + height]],
                [[left, top + height], [left, top]],
            ] {
                dashed(pixels, start, end, stroke, scale, clip)?;
            }
        }
    }
    Ok(())
}
