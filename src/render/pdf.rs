mod fonts;
use super::{RenderError, color, validate_plan};
use crate::{
    layout::{DocumentTextPlan, TextLine},
    measurement::{glyphs, win_ansi},
};
use flate2::{Compression, write::ZlibEncoder};
use pdf_writer::{Content, Filter, Name, Pdf, Rect, Ref, Str, TextStr};
use std::io::Write;

pub fn paint_pdf(plan: &DocumentTextPlan) -> Result<Vec<u8>, RenderError> {
    validate_plan(plan)?;
    let mut document = Pdf::new();
    document.set_version(1, 7);
    let mut objects = Objects::default();
    let catalog = objects.allocate();
    let page_tree = objects.allocate();
    document.catalog(catalog).pages(page_tree);
    let fonts = fonts::Fonts::write(plan, &mut document, &mut objects)?;
    let mut page_ids = Vec::new();
    // An empty source still produces a useful, valid one-page PDF.
    for page_index in 0..plan.pages.len().max(1) {
        let page_id = objects.allocate();
        let content_id = objects.allocate();
        page_ids.push(page_id);
        let mut content = Content::new();
        if let Some(page) = plan.pages.get(page_index) {
            for placed in &page.lines {
                paint_line(&mut content, &placed.line, plan.geometry.height, &fonts)?;
            }
        }
        let content_bytes = compress(&content.finish())?;
        document
            .stream(content_id, &content_bytes)
            .filter(Filter::FlateDecode);
        let mut page = document.page(page_id);
        page.parent(page_tree)
            .media_box(Rect::new(
                0.0,
                0.0,
                plan.geometry.width as f32,
                plan.geometry.height as f32,
            ))
            .contents(content_id);
        let mut resources = page.resources();
        let mut page_fonts = resources.fonts();
        for font in &fonts.entries {
            page_fonts.pair(Name(font.resource.as_bytes()), font.reference);
        }
    }
    document
        .pages(page_tree)
        .count(page_ids.len() as i32)
        .kids(page_ids);
    Ok(document.finish())
}
fn paint_line(
    content: &mut Content,
    line: &TextLine,
    page_height: f64,
    fonts: &fonts::Fonts,
) -> Result<(), RenderError> {
    for rectangle in &line.highlights {
        let [red, green, blue] = color(&rectangle.color)?
            .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content
            .set_fill_rgb(red, green, blue)
            .rect(
                rounded(rectangle.x),
                rounded(page_height - rectangle.top),
                rounded(rectangle.width),
                -rounded(rectangle.height),
            )
            .fill_nonzero();
    }
    let mut actual_text = String::new();
    let mut previous: Option<&crate::layout::TextFragment> = None;
    for fragment in &line.fragments {
        if let Some(previous) = previous
            && !previous.text.ends_with(char::is_whitespace)
            && !fragment.text.starts_with(char::is_whitespace)
            && fragment.x - previous.x - previous.width > 0.5
        {
            actual_text.push(' ');
        }
        actual_text.push_str(&fragment.text);
        previous = Some(fragment);
    }
    let preserve_actual_text = !actual_text.is_empty()
        && line
            .fragments
            .iter()
            .any(|fragment| fragment.font.bytes.is_some());
    if preserve_actual_text {
        content
            .begin_marked_content_with_properties(Name(b"Span"))
            .properties()
            .actual_text(TextStr(&actual_text));
    }
    for fragment in &line.fragments {
        if fragment.text.is_empty() {
            continue;
        }
        let font = fonts.find(&fragment.font)?;
        let [red, green, blue] = color(&fragment.style.color)?
            .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content
            .set_fill_rgb(red, green, blue)
            .begin_text()
            .set_font(
                Name(font.resource.as_bytes()),
                fragment.style.measurement.size_points as f32,
            );
        let compact = font
            .selection
            .bytes
            .as_ref()
            .is_some_and(|bytes| bytes.starts_with(b"OTTO"));
        let baseline = if compact {
            (page_height - fragment.baseline) as f32
        } else {
            rounded(page_height - fragment.baseline)
        };
        if let Some(bytes) = &font.selection.bytes {
            let face = ttf_parser::Face::parse(bytes, 0)
                .map_err(|_| RenderError::Invalid("invalid PDF font".into()))?;
            if face.tables().cff.is_some() {
                for placement in glyphs::placements(
                    &face,
                    &fragment.text,
                    fragment.style.measurement.size_points,
                )? {
                    let code = font.code(placement.character)?;
                    content
                        .set_text_matrix([
                            1.0,
                            0.0,
                            0.0,
                            1.0,
                            (fragment.x + placement.offset) as f32,
                            baseline,
                        ])
                        .show(Str(&code.to_be_bytes()));
                }
            } else {
                let mut encoded = Vec::new();
                for character in fragment.text.chars() {
                    encoded.extend(font.code(character)?.to_be_bytes());
                }
                content
                    .set_text_matrix([1.0, 0.0, 0.0, 1.0, rounded(fragment.x), baseline])
                    .show(Str(&encoded));
            }
        } else {
            let encoded = fragment
                .text
                .chars()
                .map(|character| {
                    win_ansi(character)
                        .ok_or_else(|| RenderError::Invalid("unprepared core-font text".into()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            content
                .set_text_matrix([1.0, 0.0, 0.0, 1.0, rounded(fragment.x), baseline])
                .show(Str(&encoded));
        }
        content.end_text();
    }
    if preserve_actual_text {
        content.end_marked_content();
    }
    for stroke in &line.decorations {
        let [red, green, blue] = color(&stroke.color)?
            .map(|channel| ((f64::from(channel) / 255.0 * 1000.0).round() / 1000.0) as f32);
        content
            .set_stroke_rgb(red, green, blue)
            .set_line_width(rounded(stroke.width))
            .move_to(rounded(stroke.start), rounded(page_height - stroke.y))
            .line_to(rounded(stroke.end), rounded(page_height - stroke.y))
            .stroke();
    }
    Ok(())
}
#[derive(Default)]
struct Objects {
    next: i32,
}
impl Objects {
    fn allocate(&mut self) -> Ref {
        self.next += 1;
        Ref::new(self.next)
    }
}
fn compress(bytes: &[u8]) -> Result<Vec<u8>, RenderError> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    Ok(encoder.finish()?)
}

fn rounded(value: f64) -> f32 {
    ((value * 100.0).round() / 100.0) as f32
}
