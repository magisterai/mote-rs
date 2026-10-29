use super::{RenderError, Rendered, color, fonts::RasterFonts, validate_plan};
use crate::{layout::DocumentTextPlan, measurement::glyphs};
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
    let width = (plan.geometry.width * scale).round();
    let height = (plan.geometry.height * scale).round();
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
    let mut fonts = RasterFonts::default();
    let mut warnings = BTreeSet::new();
    let mut pages = Vec::new();
    for page_index in 0..plan.pages.len().max(1) {
        let mut pixels = Pixmap::new(width as u32, height as u32)
            .ok_or_else(|| RenderError::Invalid("cannot allocate PNG page".into()))?;
        pixels.fill(tiny_skia::Color::WHITE);
        if let Some(page) = plan.pages.get(page_index) {
            for placed in &page.lines {
                let line = &placed.line;
                for rectangle in &line.highlights {
                    let left = (rectangle.x * scale).round() as f32;
                    let top = (rectangle.top * scale).round() as f32;
                    let right = ((rectangle.x + rectangle.width) * scale).round() as f32;
                    let bottom = ((rectangle.top + rectangle.height) * scale).round() as f32;
                    if let Some(bounds) = tiny_skia::Rect::from_ltrb(left, top, right, bottom) {
                        pixels.fill_rect(
                            bounds,
                            &paint(&rectangle.color, false)?,
                            Transform::identity(),
                            None,
                        );
                    }
                }
                for fragment in &line.fragments {
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
                    let outline_scale =
                        (size * 64.0).round() / 64.0 / f64::from(face.units_per_em());
                    let baseline = (fragment.baseline * scale).round();
                    let origin = (fragment.x * scale).round();
                    let brush = paint(&fragment.style.color, true)?;
                    for placement in glyphs::placements(&face, &fragment.text, size)? {
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
                                None,
                            );
                        }
                    }
                }
                for decoration in &line.decorations {
                    crisp_horizontal(
                        &mut pixels,
                        decoration.start * scale,
                        decoration.end * scale,
                        decoration.y * scale,
                        decoration.width * scale,
                        color(&decoration.color)?,
                    );
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

fn crisp_horizontal(
    pixels: &mut Pixmap,
    start: f64,
    end: f64,
    vertical: f64,
    width: f64,
    color: [u8; 3],
) {
    let left = start.min(end);
    let right = start.max(end);
    let steps = (right - left).ceil();
    if steps <= 0.0 {
        return;
    }
    let step_width = (right - left) / steps;
    let radius = (width / 2.0).max(0.5);
    let first_row = (vertical - radius).floor().max(0.0) as u32;
    let last_row = (vertical + radius).ceil().min(f64::from(pixels.height())) as u32;
    let first_column = (left - radius).floor().max(0.0) as u32;
    let last_column = (right + radius).ceil().min(f64::from(pixels.width())) as u32;
    let pixel_width = pixels.width() as usize;
    for row in first_row..last_row {
        let distance_y = f64::from(row) + 0.5 - vertical;
        for column in first_column..last_column {
            let center = f64::from(column) + 0.5;
            let closest = ((center - left) / step_width).round().clamp(0.0, steps);
            let distance_x = center - (left + closest * step_width);
            if distance_x * distance_x + distance_y * distance_y <= radius * radius {
                let offset = (row as usize * pixel_width + column as usize) * 4;
                pixels.data_mut()[offset..offset + 4]
                    .copy_from_slice(&[color[0], color[1], color[2], 255]);
            }
        }
    }
}
