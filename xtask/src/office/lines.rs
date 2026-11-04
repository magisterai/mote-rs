//! Text lines of a PDF, read as glyph baselines through the hayro interpreter.
use hayro::hayro_interpret::{
    BlendMode, ClipPath, Context, Device, DrawMode, DrawProps, Image, ImageDrawProps,
    InterpreterCache, InterpreterSettings, SoftMask, font::GlyphRun, hayro_cmap::BfString,
    interpret_page, util::TransformExt,
};
use hayro::hayro_syntax::Pdf;
use hayro::kurbo::{BezPath, Point, Rect};

/// Glyph outlines are expressed in thousandths of an em.
const UNITS_PER_EM: f64 = 1000.0;
/// Glyphs whose baselines differ by less than this many points share a row.
const ROW_TOLERANCE: f64 = 1.5;
/// A horizontal jump of more than this many ems between glyph origins in a row
/// separates two lines, such as two columns or two table cells.
const LINE_GAP: f64 = 3.0;
/// A jump of more than this many ems between glyph origins with no space glyph
/// between them is read as a space, since a painter may position words or a list
/// marker without one.
const WORD_GAP: f64 = 0.9;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Line {
    pub text: String,
    /// The position of the first glyph after any list marker, since one renderer
    /// may paint a marker as text and another as a shape.
    pub x: f64,
    pub baseline: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Page {
    pub width: f64,
    pub height: f64,
    pub lines: Vec<Line>,
}

struct Glyph {
    x: f64,
    baseline: f64,
    size: f64,
    text: String,
}

#[derive(Default)]
struct Glyphs(Vec<Glyph>);

impl<'a> Device<'a> for Glyphs {
    fn draw_path(&mut self, _: &BezPath, _: DrawProps<'a>, _: &DrawMode) {}
    fn push_clip_path(&mut self, _: &ClipPath) {}
    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}
    fn draw_glyph_run(&mut self, run: &GlyphRun<'_, 'a>, props: DrawProps<'a>, _: &DrawMode) {
        for glyph in run.glyphs() {
            let transform = props.transform * glyph.transform();
            let origin = transform * Point::ORIGIN;
            let size = (transform * Point::new(0.0, UNITS_PER_EM) - origin).hypot();
            let text = match glyph.as_unicode() {
                Some(BfString::Char(character)) => character.to_string(),
                Some(BfString::String(text)) => text,
                None => String::new(),
            };
            self.0.push(Glyph {
                x: origin.x,
                baseline: origin.y,
                size,
                text,
            });
        }
    }
    fn draw_image(&mut self, _: Image<'a, '_>, _: ImageDrawProps<'a>) {}
    fn pop_clip(&mut self) {}
    fn pop_transparency_group(&mut self) {}
}

/// Reads every page's text lines, ordered by baseline and then by position, in
/// points from the top-left corner of the page.
pub(crate) fn read(bytes: Vec<u8>) -> Result<Vec<Page>, String> {
    let pdf = Pdf::new(bytes).map_err(|error| format!("unreadable PDF: {error:?}"))?;
    let cache = InterpreterCache::new();
    let mut pages = Vec::new();
    for page in pdf.pages().iter() {
        let (width, height) = page.render_dimensions();
        let mut glyphs = Glyphs::default();
        let mut context = Context::new(
            page.initial_transform(true).to_kurbo(),
            Rect::new(0.0, 0.0, f64::from(width), f64::from(height)),
            &cache,
            page.xref(),
            InterpreterSettings::default(),
        );
        interpret_page(page, &mut context, &mut glyphs);
        pages.push(Page {
            width: f64::from(width),
            height: f64::from(height),
            lines: group(glyphs.0),
        });
    }
    Ok(pages)
}

fn group(mut glyphs: Vec<Glyph>) -> Vec<Line> {
    glyphs.retain(|glyph| glyph.x.is_finite() && glyph.baseline.is_finite());
    glyphs.sort_by(|first, second| first.baseline.total_cmp(&second.baseline));
    let mut rows: Vec<Vec<Glyph>> = Vec::new();
    for glyph in glyphs {
        match rows.last_mut() {
            Some(row) if (glyph.baseline - row[0].baseline).abs() < ROW_TOLERANCE => {
                row.push(glyph)
            }
            _ => rows.push(vec![glyph]),
        }
    }
    let mut lines = Vec::new();
    for mut row in rows {
        row.sort_by(|first, second| first.x.total_cmp(&second.x));
        // A renderer may draw a glyph twice in place for an effect.
        row.dedup_by(|second, first| {
            first.text == second.text && (first.x - second.x).abs() < 0.05
        });
        let mut current: Vec<Glyph> = Vec::new();
        for glyph in row {
            // The gap is measured from the last visible glyph, since one renderer
            // fills a tab with space glyphs and another leaves it empty.
            let separated = !glyph.text.trim().is_empty()
                && current
                    .iter()
                    .rev()
                    .find(|previous| !previous.text.trim().is_empty())
                    .is_some_and(|previous| {
                        glyph.x - previous.x > LINE_GAP * previous.size.max(glyph.size).max(1.0)
                    });
            if separated {
                lines.extend(line(std::mem::take(&mut current)));
            }
            current.push(glyph);
        }
        lines.extend(line(current));
    }
    lines
}

fn line(glyphs: Vec<Glyph>) -> Option<Line> {
    let mut text = String::new();
    let mut starts = Vec::new();
    for (index, glyph) in glyphs.iter().enumerate() {
        if let Some(previous) = index.checked_sub(1).map(|previous| &glyphs[previous])
            && glyph.x - previous.x > WORD_GAP * previous.size.max(1.0)
            && !previous.text.ends_with(char::is_whitespace)
            && !glyph.text.starts_with(char::is_whitespace)
        {
            text.push(' ');
        }
        starts.push((text.len(), glyph.x));
        text.push_str(&glyph.text);
    }
    let leading = text.len() - text.trim_start().len();
    let content = match text.trim_start().split_once(char::is_whitespace) {
        Some((token, rest)) if is_marker(token) => text.len() - rest.trim_start().len(),
        _ => leading,
    };
    let x = starts
        .iter()
        .find(|(offset, _)| *offset >= content)
        .map_or(glyphs[0].x, |(_, x)| *x);
    let text = text.trim().to_owned();
    if text.is_empty() {
        return None;
    }
    let baselines: Vec<f64> = glyphs.iter().map(|glyph| glyph.baseline).collect();
    let baseline = baselines.iter().sum::<f64>() / baselines.len() as f64;
    Some(Line { text, x, baseline })
}

/// A leading token is a list marker if it holds no letters or digits, such as a
/// bullet or a checkbox, or if it is a short number or letter closed by a period
/// or parenthesis.
pub(crate) fn is_marker(token: &str) -> bool {
    if token.chars().all(|character| !character.is_alphanumeric()) {
        return true;
    }
    let opened = token.strip_prefix('(').unwrap_or(token);
    opened.strip_suffix(['.', ')']).is_some_and(|core| {
        !core.is_empty() && core.chars().count() <= 4 && core.chars().all(char::is_alphanumeric)
    })
}
