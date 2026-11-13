mod clipping;
mod dashes;
mod images;
mod paths;
mod rotation;
mod strokes;
use super::{RenderError, Rendered, color, fonts::RasterFonts, validate_plan};
use crate::{
    layout::{
        DocumentTextPlan, HighlightRectangle, ListMarker, ListMarkerKind, PageOperation, Rule,
        TextLine,
    },
    measurement::glyphs,
};
use std::collections::BTreeSet;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use ttf_parser::{Face, OutlineBuilder};

#[derive(Debug, Clone)]
pub struct PngOptions {
    pub dots_per_inch: u32,
    /// Bound the pixel allocation for each page before allocating memory.
    pub maximum_page_pixels: u64,
}
impl Default for PngOptions {
    fn default() -> Self {
        Self {
            dots_per_inch: 72,
            maximum_page_pixels: 64_000_000,
        }
    }
}
#[derive(Debug)]
pub struct RasterPage {
    pub number: usize,
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

pub fn paint_png(
    plan: &DocumentTextPlan,
    options: &PngOptions,
) -> Result<Rendered<Vec<RasterPage>>, RenderError> {
    validate_plan(plan)?;
    if options.dots_per_inch == 0 || options.dots_per_inch > 9600 {
        return Err(RenderError::Invalid(
            "PNG resolution must be between 1 and 9,600 DPI".into(),
        ));
    }
    let scale = f64::from(options.dots_per_inch) / 72.0;
    let mut fonts = RasterFonts::default();
    let mut warnings = BTreeSet::new();
    let mut pages = Vec::new();
    for page_index in 0..plan.pages.len().max(1) {
        let geometry = plan
            .pages
            .get(page_index)
            .and_then(|page| page.geometry)
            .unwrap_or(plan.geometry);
        let width = (geometry.width * scale).ceil();
        let height = (geometry.height * scale).ceil();
        if width < 1.0
            || height < 1.0
            || width > f64::from(i32::MAX)
            || height > f64::from(i32::MAX)
            || width * height > options.maximum_page_pixels as f64
        {
            return Err(RenderError::Invalid(
                "PNG page dimensions exceed the pixel allocation limit".into(),
            ));
        }
        let mut pixels = Pixmap::new(width as u32, height as u32)
            .ok_or_else(|| RenderError::Invalid("cannot allocate PNG page".into()))?;
        pixels.fill(tiny_skia::Color::WHITE);
        let mut clips = clipping::Clips::new(width as u32, height as u32)?;
        let mut rotations = Vec::new();
        if let Some(page) = plan.pages.get(page_index) {
            for operation in &page.operations {
                let clip = clips.mask();
                match operation {
                    PageOperation::PushRotation { degrees, center } => {
                        let layer =
                            Pixmap::new(pixels.width(), pixels.height()).ok_or_else(|| {
                                RenderError::Invalid("cannot allocate drawing text layer".into())
                            })?;
                        let layer_clips = clipping::Clips::new(pixels.width(), pixels.height())?;
                        rotations.push((
                            std::mem::replace(&mut pixels, layer),
                            std::mem::replace(&mut clips, layer_clips),
                            *degrees,
                            *center,
                        ));
                    }
                    PageOperation::PopRotation => {
                        let (parent, parent_clips, degrees, center) =
                            rotations.pop().expect("validated rotation nesting");
                        let layer = std::mem::replace(&mut pixels, parent);
                        clips = parent_clips;
                        rotation::paint(&mut pixels, layer, degrees, center, scale, clips.mask());
                    }

                    PageOperation::PaintedRectangle {
                        bounds,
                        fill,
                        stroke,
                    } => paths::draw_rectangle(
                        &mut pixels,
                        *bounds,
                        fill.as_ref(),
                        stroke.as_ref(),
                        scale,
                        clip,
                    )?,
                    PageOperation::Path(path) => paths::draw(&mut pixels, path, scale, clip)?,
                    PageOperation::PushClip(bounds) => clips.push(*bounds, scale),
                    PageOperation::PopClip => clips.pop(),
                    PageOperation::Stroke {
                        rule,
                        dash,
                        round_cap,
                    } => strokes::paint(&mut pixels, rule, dash, *round_cap, scale, clip)?,
                    PageOperation::OutlineRectangle {
                        bounds,
                        width,
                        color,
                    } => {
                        let [left, top, horizontal, vertical] = *bounds;
                        let line = width.max(1.0 / scale);
                        for (left, top, width, height) in [
                            (left, top, horizontal, line),
                            (left, top + vertical - line, horizontal, line),
                            (left, top, line, vertical),
                            (left + horizontal - line, top, line, vertical),
                        ] {
                            fill_rectangle(
                                &mut pixels,
                                &crate::layout::HighlightRectangle {
                                    x: left,
                                    top,
                                    width,
                                    height,
                                    color: color.clone(),
                                },
                                scale,
                                clip,
                            )?;
                        }
                    }
                    PageOperation::Image(image) => images::paint(&mut pixels, image, scale, clip),
                    PageOperation::Text(index) => paint_line(
                        &mut pixels,
                        &page.lines[*index].line,
                        scale,
                        &mut fonts,
                        &mut warnings,
                        clip,
                    )?,
                    PageOperation::Rectangle(rectangle) => {
                        fill_rectangle(&mut pixels, rectangle, scale, clip)?
                    }
                    PageOperation::Rule(rule) => paint_rule(&mut pixels, rule, scale, clip)?,
                    PageOperation::ListMarker(marker) => {
                        paint_marker(&mut pixels, marker, scale, &mut fonts, &mut warnings, clip)?
                    }
                }
            }
        }
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, width as u32, height as u32);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let pixels_per_meter = (f64::from(options.dots_per_inch) / 0.0254).round() as u32;
            encoder.set_pixel_dims(Some(png::PixelDimensions {
                xppu: pixels_per_meter,
                yppu: pixels_per_meter,
                unit: png::Unit::Meter,
            }));
            let mut writer = encoder
                .write_header()
                .map_err(|error| RenderError::Invalid(format!("PNG header: {error}")))?;
            writer
                .write_image_data(pixels.data())
                .map_err(|error| RenderError::Invalid(format!("PNG pixels: {error}")))?;
            writer
                .finish()
                .map_err(|error| RenderError::Invalid(format!("PNG finish: {error}")))?;
        }
        pages.push(RasterPage {
            number: page_index + 1,
            width: width as u32,
            height: height as u32,
            bytes,
        });
    }
    Ok(Rendered {
        output: pages,
        warnings: warnings.into_iter().collect(),
    })
}
fn paint(color_value: &str, antialias: bool) -> Result<Paint<'static>, RenderError> {
    let [red, green, blue] = color(color_value)?;
    let mut paint = Paint {
        anti_alias: antialias,
        ..Default::default()
    };
    paint.set_color_rgba8(red, green, blue, 255);
    Ok(paint)
}
struct Outline {
    path: PathBuilder,
    scale: f64,
    origin: f64,
    baseline: f64,
}
impl Outline {
    fn point(&self, horizontal: f32, vertical: f32) -> (f32, f32) {
        (
            (self.origin + (f64::from(horizontal) * self.scale * 64.0).round() / 64.0) as f32,
            (self.baseline - (f64::from(vertical) * self.scale * 64.0).round() / 64.0) as f32,
        )
    }
}
impl OutlineBuilder for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        let (horizontal, vertical) = self.point(x, y);
        self.path.move_to(horizontal, vertical);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let (horizontal, vertical) = self.point(x, y);
        self.path.line_to(horizontal, vertical);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (control_x, control_y) = self.point(x1, y1);
        let (horizontal, vertical) = self.point(x, y);
        self.path
            .quad_to(control_x, control_y, horizontal, vertical);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (first_x, first_y) = self.point(x1, y1);
        let (second_x, second_y) = self.point(x2, y2);
        let (horizontal, vertical) = self.point(x, y);
        self.path
            .cubic_to(first_x, first_y, second_x, second_y, horizontal, vertical);
    }
    fn close(&mut self) {
        self.path.close();
    }
}

fn crisp_axis_aligned(
    pixels: &mut Pixmap,
    start: [f64; 2],
    end: [f64; 2],
    width: f64,
    color: [u8; 3],
    clip: Option<&tiny_skia::Mask>,
) {
    let horizontal = start[1] == end[1];
    let along = usize::from(!horizontal);
    let across = 1 - along;
    let first = start[along].min(end[along]);
    let last = start[along].max(end[along]);
    if last <= first {
        return;
    }
    let radius = (width / 2.0).max(0.5);
    let dimensions = [f64::from(pixels.width()), f64::from(pixels.height())];
    let first_along = (first - radius).floor().max(0.0) as u32;
    let last_along = (last + radius).ceil().min(dimensions[along]) as u32;
    let first_across = (start[across] - radius).floor().max(0.0) as u32;
    let last_across = (start[across] + radius).ceil().min(dimensions[across]) as u32;
    let pixel_width = pixels.width() as usize;
    for cross_position in first_across..last_across {
        let cross_distance = f64::from(cross_position) + 0.5 - start[across];
        for position in first_along..last_along {
            let center = f64::from(position) + 0.5;
            // The distance to the segment itself; sampling points along it would
            // drift against the pixel grid and leave gaps in a one-pixel line.
            let distance = (first - center).max(center - last).max(0.0);
            if distance * distance + cross_distance * cross_distance <= radius * radius {
                let (column, row) = if horizontal {
                    (position, cross_position)
                } else {
                    (cross_position, position)
                };
                let offset = (row as usize * pixel_width + column as usize) * 4;
                if clip.is_some_and(|mask| mask.data()[offset / 4] == 0) {
                    continue;
                }
                pixels.data_mut()[offset..offset + 4]
                    .copy_from_slice(&[color[0], color[1], color[2], 255]);
            }
        }
    }
}

fn paint_line(
    pixels: &mut Pixmap,
    line: &TextLine,
    scale: f64,
    fonts: &mut RasterFonts,
    warnings: &mut BTreeSet<String>,
    clip: Option<&tiny_skia::Mask>,
) -> Result<(), RenderError> {
    for rectangle in &line.highlights {
        fill_rectangle(pixels, rectangle, scale, clip)?;
    }
    for fragment in &line.fragments {
        if let Some(formula) = &fragment.formula {
            paint_formula(pixels, fragment, formula, scale, fonts, clip)?;
            continue;
        }
        let selected = fonts.resolve(&fragment.font)?;
        if selected.fallback {
            warnings.insert(format!(
                "PNG uses a bundled outline fallback for {} {}",
                fragment.font.family,
                fragment.font.variant.as_text()
            ));
        }
        let face = Face::parse(&selected.bytes, selected.face_index)
            .map_err(|_| RenderError::Invalid("invalid PNG font".into()))?;
        let size = fragment.style.measurement.size_points * scale;
        let outline_scale = (size * 64.0).round() / 64.0 / f64::from(face.units_per_em());
        let baseline = (fragment.baseline * scale).round();
        let origin = (fragment.x * scale).round();
        let brush = paint(&fragment.style.color, true)?;
        let synthetic_bold = fragment.synthetic_bold();
        for placement in glyphs::placements(&face, &fragment.text, size, fragment.kerned)? {
            let mut outline = Outline {
                path: PathBuilder::new(),
                scale: outline_scale,
                origin: origin + placement.offset,
                baseline,
            };
            face.outline_glyph(placement.glyph, &mut outline);
            if let Some(path) = outline.path.finish() {
                pixels.fill_path(
                    &path,
                    &brush,
                    FillRule::Winding,
                    Transform::identity(),
                    clip,
                );
                if let Some(width) = synthetic_bold {
                    pixels.stroke_path(
                        &path,
                        &brush,
                        &tiny_skia::Stroke {
                            width: (width * scale) as f32,
                            ..Default::default()
                        },
                        Transform::identity(),
                        clip,
                    );
                }
            }
        }
    }
    for decoration in &line.decorations {
        if decoration.wave {
            paths::draw(
                pixels,
                &decoration.wave_outline(decoration.width.max(1.0 / scale)),
                scale,
                clip,
            )?;
            continue;
        }
        if !decoration.dash.is_empty() {
            strokes::paint(
                pixels,
                &crate::layout::Rule {
                    role: crate::layout::RuleRole::Generic,
                    start: [decoration.start, decoration.y],
                    end: [decoration.end, decoration.y],
                    width: decoration.width,
                    color: decoration.color.clone(),
                },
                &decoration.dash,
                false,
                scale,
                clip,
            )?;
            continue;
        }
        crisp_axis_aligned(
            pixels,
            [decoration.start * scale, decoration.y * scale],
            [decoration.end * scale, decoration.y * scale],
            decoration.width * scale,
            color(&decoration.color)?,
            clip,
        );
    }
    Ok(())
}

fn paint_formula(
    pixels: &mut Pixmap,
    fragment: &crate::layout::TextFragment,
    formula: &crate::layout::Formula,
    scale: f64,
    fonts: &mut RasterFonts,
    clip: Option<&tiny_skia::Mask>,
) -> Result<(), RenderError> {
    let math = fonts.resolve(&formula.font)?;
    let math = (math.bytes.clone(), math.face_index);
    let text = match &formula.text_font {
        Some(text_font) => {
            let text = fonts.resolve(text_font)?;
            Some((text.bytes.clone(), text.face_index))
        }
        None => None,
    };
    let invalid = |_| RenderError::Invalid("invalid PNG font".into());
    let math_face = Face::parse(&math.0, math.1).map_err(invalid)?;
    let text_face = text
        .as_ref()
        .map(|(bytes, index)| Face::parse(bytes, *index))
        .transpose()
        .map_err(invalid)?;
    let brush = paint(&fragment.style.color, true)?;
    for glyph in &formula.glyphs {
        let face = match &text_face {
            Some(text_face) if glyph.in_text_font => text_face,
            _ => &math_face,
        };
        let mut outline = Outline {
            path: PathBuilder::new(),
            scale: glyph.size * scale / f64::from(face.units_per_em()),
            origin: (fragment.x + glyph.x) * scale,
            baseline: (fragment.baseline - glyph.rise) * scale,
        };
        face.outline_glyph(ttf_parser::GlyphId(glyph.glyph), &mut outline);
        if let Some(path) = outline.path.finish() {
            pixels.fill_path(
                &path,
                &brush,
                FillRule::Winding,
                Transform::identity(),
                clip,
            );
        }
    }
    for rule in &formula.rules {
        // A rule takes the exact coverage of its rectangle, as the glyphs beside
        // it do; a rule snapped to whole pixels would vary in weight with its
        // place on the pixel grid.
        let bounds = tiny_skia::Rect::from_xywh(
            ((fragment.x + rule.x) * scale) as f32,
            ((fragment.baseline - rule.rise - rule.height) * scale) as f32,
            (rule.width * scale) as f32,
            (rule.height * scale) as f32,
        );
        if let Some(bounds) = bounds {
            pixels.fill_path(
                &PathBuilder::from_rect(bounds),
                &brush,
                FillRule::Winding,
                Transform::identity(),
                clip,
            );
        }
    }
    Ok(())
}

fn fill_rectangle(
    pixels: &mut Pixmap,
    rectangle: &HighlightRectangle,
    scale: f64,
    clip: Option<&tiny_skia::Mask>,
) -> Result<(), RenderError> {
    let left = (rectangle.x * scale).round() as f32;
    let top = (rectangle.top * scale).round() as f32;
    let right = ((rectangle.x + rectangle.width) * scale).round() as f32;
    let bottom = ((rectangle.top + rectangle.height) * scale).round() as f32;
    if right <= left || bottom <= top {
        return Ok(());
    }
    if let Some(bounds) = tiny_skia::Rect::from_ltrb(left, top, right, bottom) {
        pixels.fill_rect(
            bounds,
            &paint(&rectangle.color, false)?,
            Transform::identity(),
            clip,
        );
    }
    Ok(())
}
fn paint_rule(
    pixels: &mut Pixmap,
    rule: &Rule,
    scale: f64,
    clip: Option<&tiny_skia::Mask>,
) -> Result<(), RenderError> {
    if matches!(
        rule.role,
        crate::layout::RuleRole::ChartAxis | crate::layout::RuleRole::ChartGridline
    ) && (rule.start[0] == rule.end[0] || rule.start[1] == rule.end[1])
    {
        let pixel_width = (rule.width * scale).round().max(1.0) as i64;
        let half_width = pixel_width / 2;
        let start = rule.start.map(|value| (value * scale).round());
        let end = rule.end.map(|value| (value * scale).round());
        let (left, top, right, bottom) = if start[1] == end[1] {
            let left = start[0].min(end[0]);
            let top = start[1] - half_width as f64;
            (
                left,
                top,
                start[0].max(end[0]).max(left + 1.0),
                top + pixel_width as f64,
            )
        } else {
            let top = start[1].min(end[1]);
            let left = start[0] - half_width as f64;
            (
                left,
                top,
                left + pixel_width as f64,
                start[1].max(end[1]).max(top + 1.0),
            )
        };
        if let Some(bounds) =
            tiny_skia::Rect::from_ltrb(left as f32, top as f32, right as f32, bottom as f32)
        {
            pixels.fill_rect(
                bounds,
                &paint(&rule.color, false)?,
                Transform::identity(),
                clip,
            );
        }
        return Ok(());
    }
    if rule.role == crate::layout::RuleRole::Divider {
        return fill_rectangle(
            pixels,
            &HighlightRectangle {
                x: rule.start[0].min(rule.end[0]),
                top: rule.start[1] - rule.width / 2.0,
                width: (rule.end[0] - rule.start[0]).abs(),
                height: rule.width.max(0.5),
                color: rule.color.clone(),
            },
            scale,
            clip,
        );
    }
    if rule.role == crate::layout::RuleRole::NoteSeparator {
        return fill_rectangle(
            pixels,
            &HighlightRectangle {
                x: rule.start[0].min(rule.end[0]),
                top: rule.start[1] - rule.width.max(1.0 / scale) / 2.0,
                width: (rule.end[0] - rule.start[0]).abs(),
                height: rule.width.max(1.0 / scale),
                color: rule.color.clone(),
            },
            scale,
            clip,
        );
    }
    if rule.role == crate::layout::RuleRole::ColumnSeparator {
        return fill_rectangle(
            pixels,
            &HighlightRectangle {
                x: rule.start[0] - rule.width / 2.0,
                top: rule.start[1].min(rule.end[1]),
                width: rule.width,
                height: (rule.end[1] - rule.start[1]).abs(),
                color: rule.color.clone(),
            },
            scale,
            clip,
        );
    }
    if rule.start[1] == rule.end[1] || rule.start[0] == rule.end[0] {
        crisp_axis_aligned(
            pixels,
            rule.start.map(|value| value * scale),
            rule.end.map(|value| value * scale),
            rule.width * scale,
            color(&rule.color)?,
            clip,
        );
    } else {
        let mut path = PathBuilder::new();
        path.move_to(
            (rule.start[0] * scale) as f32,
            (rule.start[1] * scale) as f32,
        );
        path.line_to((rule.end[0] * scale) as f32, (rule.end[1] * scale) as f32);
        if let Some(path) = path.finish() {
            pixels.stroke_path(
                &path,
                &paint(&rule.color, true)?,
                &tiny_skia::Stroke {
                    width: (rule.width * scale).max(1.0) as f32,
                    line_cap: tiny_skia::LineCap::Round,
                    ..Default::default()
                },
                Transform::identity(),
                clip,
            );
        }
    }
    Ok(())
}
fn paint_marker(
    pixels: &mut Pixmap,
    marker: &ListMarker,
    scale: f64,
    fonts: &mut RasterFonts,
    warnings: &mut BTreeSet<String>,
    clip: Option<&tiny_skia::Mask>,
) -> Result<(), RenderError> {
    match marker.kind {
        ListMarkerKind::Ordered => {
            if let Some(line) = &marker.text {
                paint_line(pixels, line, scale, fonts, warnings, clip)?;
            }
            return Ok(());
        }
        ListMarkerKind::Bullet if marker.glyph_bullet() => {
            if let Some(line) = &marker.text {
                paint_line(pixels, line, scale, fonts, warnings, clip)?;
            }
            return Ok(());
        }
        ListMarkerKind::Bullet => {
            let ([center_x, center_y], radius) = marker.bullet_disc();
            let color = marker
                .text
                .as_ref()
                .and_then(|line| line.fragments.first())
                .map_or("111827", |fragment| fragment.style.color.as_str());
            if let Some(path) = PathBuilder::from_circle(
                (center_x * scale) as f32,
                (center_y * scale) as f32,
                (radius * scale) as f32,
            ) {
                pixels.fill_path(
                    &path,
                    &paint(color, true)?,
                    FillRule::Winding,
                    Transform::identity(),
                    clip,
                );
            }
            return Ok(());
        }
        ListMarkerKind::Task => {}
    }
    let mut path = PathBuilder::new();
    for contour in marker.task_contours() {
        let [x, y] = contour[0];
        path.move_to((x * scale) as f32, (y * scale) as f32);
        for [x, y] in &contour[1..] {
            path.line_to((x * scale) as f32, (y * scale) as f32);
        }
        path.close();
    }
    if let Some(path) = path.finish() {
        pixels.fill_path(
            &path,
            &paint("111827", true)?,
            FillRule::EvenOdd,
            Transform::identity(),
            clip,
        );
    }
    Ok(())
}
