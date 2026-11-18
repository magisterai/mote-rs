//! The marks of a PDF page that are not text: its fills, its strokes and the
//! frames of its images, painted without the glyphs. The text lines of a page
//! say where the text lies; the picture says what is drawn about it.
use hayro::hayro_interpret::{
    BlendMode, ClipPath, Context, Device, DrawMode, DrawProps, FillRule, Image, ImageDrawProps,
    InterpreterCache, InterpreterSettings, Paint, SoftMask, StrokeProps, font::GlyphRun,
    interpret_page, util::TransformExt,
};
use hayro::hayro_syntax::Pdf;
use hayro::kurbo::{Affine, BezPath, Rect, Stroke};
use hayro::vello_cpu::{
    Pixmap, RasterizerSettings, RenderContext, Resources, TargetInit,
    color::{AlphaColor, Srgb, palette::css::WHITE},
    peniko::Fill,
};
use std::io::Cursor;

/// A page is painted at two pixels to the point and kept at one, each kept
/// pixel the mean of four, so that a rule half a point wide leaves a trace.
const SCALE: f64 = 2.0;
/// The size in points below which an image is taken for a glyph.
const GLYPH: f64 = 24.0;
/// The grey of the frame of an image, whose content is not compared.
const IMAGE: [u8; 3] = [128, 128, 128];
/// The difference in a channel by which a point may lie outside the range of
/// its neighbours in the other picture. A hairline of the lightest default
/// border, half covered, differs from the page by more.
const THRESHOLD: u8 = 12;

/// The marks of one page, one RGB pixel to the point.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Picture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

struct Marks {
    context: RenderContext,
}

impl Marks {
    fn paint(&mut self, paint: &Paint<'_>) {
        let color = match paint {
            Paint::Color(color) => color.to_rgba().to_rgba8(),
            // A pattern is drawn as the frame of an image is.
            Paint::Pattern(_) => [IMAGE[0], IMAGE[1], IMAGE[2], 255],
        };
        self.context.set_paint(AlphaColor::<Srgb>::from_rgba8(
            color[0], color[1], color[2], color[3],
        ));
    }

    fn fill(&mut self, path: &BezPath, rule: FillRule) {
        self.context.set_fill_rule(fill_rule(rule));
        self.context.fill_path(path);
    }

    fn stroke(&mut self, path: &BezPath, stroke: &StrokeProps) {
        self.context.set_stroke(Stroke {
            width: f64::from(stroke.line_width),
            join: stroke.line_join,
            miter_limit: f64::from(stroke.miter_limit),
            start_cap: stroke.line_cap,
            end_cap: stroke.line_cap,
            dash_pattern: stroke
                .dash_array
                .iter()
                .map(|dash| f64::from(*dash))
                .collect(),
            dash_offset: f64::from(stroke.dash_offset),
        });
        self.context.stroke_path(path);
    }
}

fn fill_rule(rule: FillRule) -> Fill {
    match rule {
        FillRule::NonZero => Fill::NonZero,
        FillRule::EvenOdd => Fill::EvenOdd,
    }
}

impl<'a> Device<'a> for Marks {
    fn draw_path(&mut self, path: &BezPath, props: DrawProps<'a>, mode: &DrawMode) {
        self.context.set_transform(props.transform);
        self.paint(&props.paint);
        match mode {
            DrawMode::Fill(rule) => self.fill(path, *rule),
            DrawMode::Stroke(stroke) => self.stroke(path, stroke),
            DrawMode::FillAndStroke(rule, stroke) => {
                self.fill(path, *rule);
                self.stroke(path, stroke);
            }
            DrawMode::Invisible => {}
        }
    }
    fn push_clip_path(&mut self, clip: &ClipPath) {
        self.context.set_fill_rule(fill_rule(clip.fill));
        self.context.set_transform(Affine::IDENTITY);
        self.context.push_clip_path(&clip.path);
    }
    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}
    fn draw_glyph_run(&mut self, _: &GlyphRun<'_, 'a>, _: DrawProps<'a>, _: &DrawMode) {}
    fn draw_image(&mut self, image: Image<'a, '_>, props: ImageDrawProps<'a>) {
        let (Image::Raster(raster), None) = (&image, &props.soft_mask) else {
            return;
        };
        let (width, height) = (f64::from(image.width()), f64::from(image.height()));
        // An image no larger than a line of text is a glyph, such as an emoji,
        // which one renderer draws as an image and another as text.
        let frame = props
            .transform
            .transform_rect_bbox(Rect::new(0.0, 0.0, width, height));
        if frame.width().max(frame.height()) < GLYPH * SCALE {
            return;
        }
        // The frame of an image with transparency is what it covers of each of
        // its rows. Word draws a turned image so, and draws a shadow, and text
        // that is not opaque, as an image that covers little of its frame;
        // such an image is no frame and is left out.
        let mut rows = vec![Rect::new(0.0, 0.0, width, height)];
        raster.with_rgba(
            |_, alpha| {
                let Some(alpha) = alpha else { return };
                let (columns, lines) = (alpha.width as usize, alpha.height as usize);
                let scale = (width / columns as f64, height / lines as f64);
                let covered = alpha.data.iter().filter(|value| **value > 127).count();
                rows.clear();
                if covered * 2 < alpha.data.len() {
                    return;
                }
                for line in 0..lines {
                    let row = &alpha.data[line * columns..(line + 1) * columns];
                    let (Some(first), Some(last)) = (
                        row.iter().position(|value| *value > 127),
                        row.iter().rposition(|value| *value > 127),
                    ) else {
                        continue;
                    };
                    rows.push(Rect::new(
                        first as f64 * scale.0,
                        line as f64 * scale.1,
                        (last + 1) as f64 * scale.0,
                        (line + 1) as f64 * scale.1,
                    ));
                }
            },
            None,
        );
        if rows.is_empty() {
            return;
        }
        self.context.set_transform(props.transform);
        self.context.set_paint(AlphaColor::<Srgb>::from_rgba8(
            IMAGE[0], IMAGE[1], IMAGE[2], 255,
        ));
        // The rows are filled as one outline, down their left ends and up
        // their right ends, so that no seam shows between them.
        let mut outline = BezPath::new();
        for (index, row) in rows.iter().enumerate() {
            if index == 0 {
                outline.move_to((row.x0, row.y0));
            } else {
                outline.line_to((row.x0, row.y0));
            }
            outline.line_to((row.x0, row.y1));
        }
        for row in rows.iter().rev() {
            outline.line_to((row.x1, row.y1));
            outline.line_to((row.x1, row.y0));
        }
        outline.close_path();
        self.fill(&outline, FillRule::NonZero);
    }
    fn pop_clip(&mut self) {
        self.context.pop_clip();
    }
    fn pop_transparency_group(&mut self) {}
}

/// Paints the marks of every page of a PDF.
pub(crate) fn read(bytes: Vec<u8>) -> Result<Vec<Picture>, String> {
    let pdf = Pdf::new(bytes).map_err(|error| format!("unreadable PDF: {error:?}"))?;
    let cache = InterpreterCache::new();
    let mut pictures = Vec::new();
    for page in pdf.pages().iter() {
        let (width, height) = page.render_dimensions();
        let (width, height) = (f64::from(width), f64::from(height));
        let mut marks = Marks {
            context: RenderContext::new((width * SCALE) as u16, (height * SCALE) as u16),
        };
        let mut context = Context::new(
            Affine::scale(SCALE) * page.initial_transform(true).to_kurbo(),
            Rect::new(0.0, 0.0, width * SCALE, height * SCALE),
            &cache,
            page.xref(),
            InterpreterSettings::default(),
        );
        interpret_page(page, &mut context, &mut marks);
        marks.context.flush();
        let mut pixmap = Pixmap::new(marks.context.width(), marks.context.height());
        marks.context.render_with(
            &mut pixmap,
            &mut Resources::default(),
            RasterizerSettings {
                target_init: TargetInit::Clear(WHITE),
                ..Default::default()
            },
        );
        pictures.push(halved(
            usize::from(pixmap.width()),
            usize::from(pixmap.height()),
            pixmap.data_as_u8_slice(),
        ));
    }
    Ok(pictures)
}

/// Returns the picture whose every pixel is the mean of four RGBA pixels.
fn halved(width: usize, height: usize, pixels: &[u8]) -> Picture {
    let (columns, rows) = (width / 2, height / 2);
    let mut output = Vec::with_capacity(columns * rows * 3);
    for row in 0..rows {
        for column in 0..columns {
            for channel in 0..3 {
                let sum: u32 = [(0, 0), (1, 0), (0, 1), (1, 1)]
                    .iter()
                    .map(|(right, down)| {
                        u32::from(
                            pixels[((row * 2 + down) * width + column * 2 + right) * 4 + channel],
                        )
                    })
                    .sum();
                output.push((sum / 4) as u8);
            }
        }
    }
    Picture {
        width: columns,
        height: rows,
        pixels: output,
    }
}

impl Picture {
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        let mut encoder = png::Encoder::new(&mut bytes, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_compression(png::Compression::High);
        encoder
            .write_header()
            .and_then(|mut writer| writer.write_image_data(&self.pixels))
            .map_err(|error| error.to_string())?;
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let mut decoder = png::Decoder::new(Cursor::new(bytes));
        decoder.set_transformations(png::Transformations::EXPAND);
        let mut reader = decoder.read_info().map_err(|error| error.to_string())?;
        let mut pixels = vec![0; reader.output_buffer_size().ok_or("oversized PNG")?];
        let info = reader
            .next_frame(&mut pixels)
            .map_err(|error| error.to_string())?;
        pixels.truncate(info.buffer_size());
        if info.color_type != png::ColorType::Rgb {
            return Err("a picture of marks is an RGB image".into());
        }
        Ok(Self {
            width: info.width as usize,
            height: info.height as usize,
            pixels,
        })
    }

    fn pixel(&self, column: usize, row: usize) -> [u8; 3] {
        let start = (row * self.width + column) * 3;
        [
            self.pixels[start],
            self.pixels[start + 1],
            self.pixels[start + 2],
        ]
    }

    /// Returns whether a pixel lies outside the range of the pixels within one
    /// point of the same place in `other`, in any channel.
    fn outside(&self, other: &Self, column: usize, row: usize) -> bool {
        let pixel = self.pixel(column, row);
        (0..3).any(|channel| {
            let (mut lowest, mut highest) = (u8::MAX, u8::MIN);
            for near_row in row.saturating_sub(1)..=(row + 1).min(other.height - 1) {
                for near_column in column.saturating_sub(1)..=(column + 1).min(other.width - 1) {
                    let value = other.pixel(near_column, near_row)[channel];
                    lowest = lowest.min(value);
                    highest = highest.max(value);
                }
            }
            pixel[channel] < lowest.saturating_sub(THRESHOLD)
                || pixel[channel] > highest.saturating_add(THRESHOLD)
        })
    }
}

/// The comparison of the marks of two pages.
pub(crate) struct Difference {
    /// The number of points at which the pages differ.
    pub points: usize,
    /// The differing points in red over a faint copy of the first page.
    pub map: Picture,
}

/// Compares the marks of two pages point by point. A mark shifted by less than
/// a point, or an edge rounded otherwise, stays within the range of its
/// neighbours; a mark that is moved, absent or of another colour does not.
pub(crate) fn compare(office: &Picture, native: &Picture) -> Difference {
    let (width, height) = (
        office.width.min(native.width),
        office.height.min(native.height),
    );
    // A hairline covers one row of points in one picture and parts of two in
    // another, so the pictures are compared by the ink about each point.
    let (shown, office, native) = (office, &office.spread(), &native.spread());
    let mut points = 0;
    let mut map = Vec::with_capacity(width * height * 3);
    for row in 0..height {
        for column in 0..width {
            let (first, second) = (office.pixel(column, row), native.pixel(column, row));
            // A point that is the same in both lies within the range of its
            // neighbours.
            if first != second
                && (office.outside(native, column, row) || native.outside(office, column, row))
            {
                points += 1;
                map.extend([220, 0, 0]);
            } else {
                map.extend(
                    shown
                        .pixel(column, row)
                        .map(|value| 255 - (255 - value) / 4),
                );
            }
        }
    }
    Difference {
        points,
        map: Picture {
            width,
            height,
            pixels: map,
        },
    }
}

impl Picture {
    /// Returns the picture whose every point is the mean of the points within
    /// one point of it.
    fn spread(&self) -> Self {
        let mut pixels = Vec::with_capacity(self.pixels.len());
        for row in 0..self.height {
            for column in 0..self.width {
                let mut sums = [0_u32; 3];
                let mut count = 0;
                for near_row in row.saturating_sub(1)..=(row + 1).min(self.height - 1) {
                    for near_column in column.saturating_sub(1)..=(column + 1).min(self.width - 1) {
                        let pixel = self.pixel(near_column, near_row);
                        for channel in 0..3 {
                            sums[channel] += u32::from(pixel[channel]);
                        }
                        count += 1;
                    }
                }
                pixels.extend(sums.map(|sum| (sum / count) as u8));
            }
        }
        Self {
            width: self.width,
            height: self.height,
            pixels,
        }
    }

    /// Returns the number of points of the page that bear a mark.
    pub fn marked(&self) -> usize {
        self.pixels
            .as_chunks::<3>()
            .0
            .iter()
            .filter(|pixel| **pixel != [255; 3])
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns a white page of 40 by 40 points with the given rows of the
    /// columns 5 to 34 painted in a colour of the given coverage.
    fn page(rows: &[(usize, f64)], color: [u8; 3]) -> Picture {
        let mut pixels = vec![255; 40 * 40 * 3];
        for (row, coverage) in rows {
            for column in 5..35 {
                for channel in 0..3 {
                    let value = 255.0 - (255.0 - f64::from(color[channel])) * coverage;
                    pixels[(row * 40 + column) * 3 + channel] = value as u8;
                }
            }
        }
        Picture {
            width: 40,
            height: 40,
            pixels,
        }
    }

    #[test]
    fn a_hairline_agrees_wherever_it_falls_among_the_pixels_and_a_moved_mark_does_not() {
        let black = [0, 0, 0];
        let whole = page(&[(10, 0.5)], black);
        // The same hairline lies across two rows of points, or one point lower.
        assert_eq!(
            compare(&whole, &page(&[(10, 0.25), (11, 0.25)], black)).points,
            0
        );
        assert_eq!(compare(&whole, &page(&[(11, 0.5)], black)).points, 0);
        assert!(compare(&whole, &page(&[(14, 0.5)], black)).points > 30);
        // A rule of another width, a rule of another colour and an absent rule differ.
        assert!(compare(&whole, &page(&[(10, 1.0), (11, 0.5)], black)).points > 30);
        assert!(compare(&whole, &page(&[(10, 0.5)], [0, 160, 0])).points > 30);
        assert!(compare(&whole, &page(&[], black)).points > 30);
        assert_eq!(page(&[(10, 0.5)], black).marked(), 30);
    }

    #[test]
    fn a_picture_is_read_back_as_it_was_written() {
        let picture = page(&[(3, 1.0), (4, 0.5)], [10, 120, 250]);
        assert_eq!(
            Picture::decode(&picture.encode().unwrap()).unwrap(),
            picture
        );
    }
}
