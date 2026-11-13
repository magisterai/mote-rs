//! Holds Mote's two native painters to the same picture: each page of the native
//! PDF, rasterized by an independent interpreter, must agree with the page the PNG
//! painter draws from the same plan.
use crate::{Outcome, arguments::Arguments, office};
use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_interpret::font::{FontData, FontQuery, StandardFont};
use hayro::hayro_syntax::Pdf;
use hayro::vello_cpu::color::palette::css::WHITE;
use hayro::{PixmapSettings, RenderCache, RenderSettings, render};
use mote::render::{self as painters, PngOptions};
use std::{collections::BTreeMap, io::Cursor, process::ExitCode, sync::Arc};

const HELP: &str =
    "Usage: cargo xtask painter-parity [--case NAME]... [--images DIRECTORY] [--allow]

Rasterize the native PDF of every review case and probe with an independent PDF
interpreter and compare each page with the native PNG painter's page.
--case NAME selects a review case by identifier or a probe as probe:NAME.
--images DIRECTORY writes each page's rasterized PDF, PNG painter page and a map
of the differing blocks there.
--allow replaces the selected subjects' allowances in
tests/fixtures/painter-parity/allowances.json with their present results.
";

const DOTS_PER_INCH: u32 = 144;
/// The luminance by which a block of two by two pixels may lie outside the range
/// of its neighbours in the other image.
const PIXEL_THRESHOLD: u8 = 64;
/// A small margin above an allowance absorbs floating-point noise.
const SLACK: f64 = 0.0005;

/// Supplies the system faces that the PNG painter uses for PDF standard fonts,
/// so that both painters draw the same outlines.
fn standard_font(font: &StandardFont) -> Option<(FontData, u32)> {
    use StandardFont::*;
    let (family, index) = match font {
        Helvetica => ("Helvetica", 0),
        HelveticaBold => ("Helvetica", 1),
        HelveticaOblique => ("Helvetica", 2),
        HelveticaBoldOblique => ("Helvetica", 3),
        TimesRoman => ("Times", 0),
        TimesBold => ("Times", 1),
        TimesItalic => ("Times", 2),
        TimesBoldItalic => ("Times", 3),
        Courier => ("Courier", 0),
        CourierBold => ("Courier", 1),
        CourierOblique => ("Courier", 2),
        CourierBoldOblique => ("Courier", 3),
        _ => return None,
    };
    let bytes = std::fs::read(format!("/System/Library/Fonts/{family}.ttc")).ok()?;
    Some((Arc::new(bytes), index))
}

/// Returns the luminance of the top-left `width` by `height` region of an RGBA
/// image with the given row stride, averaged over two-by-two blocks.
fn blocks(width: usize, height: usize, stride: usize, rgba: &[u8]) -> Vec<u8> {
    let (columns, rows) = (width / 2, height / 2);
    let mut values = Vec::with_capacity(columns * rows);
    for row in 0..rows {
        for column in 0..columns {
            let mut sum = 0u32;
            for (dy, dx) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                let offset = ((row * 2 + dy) * stride + column * 2 + dx) * 4;
                let [red, green, blue] = [rgba[offset], rgba[offset + 1], rgba[offset + 2]];
                sum +=
                    (u32::from(red) * 299 + u32::from(green) * 587 + u32::from(blue) * 114) / 1000;
            }
            values.push((sum / 4) as u8);
        }
    }
    values
}

/// Marks each block whose luminance lies outside the range of the blocks within
/// one block of the same place in the other image, in either direction. A mark
/// shifted by less than a block, or an edge rounded to another pixel, stays
/// within that range; a moved or reshaped mark does not.
fn differing_blocks(columns: usize, expected: &[u8], actual: &[u8]) -> Vec<bool> {
    let rows = expected.len() / columns.max(1);
    let outside = |source: &[u8], target: &[u8], index: usize| {
        let (row, column) = (index / columns, index % columns);
        let (mut lowest, mut highest) = (u8::MAX, u8::MIN);
        for neighbour_row in row.saturating_sub(1)..=(row + 1).min(rows - 1) {
            for neighbour_column in column.saturating_sub(1)..=(column + 1).min(columns - 1) {
                let value = target[neighbour_row * columns + neighbour_column];
                lowest = lowest.min(value);
                highest = highest.max(value);
            }
        }
        source[index] < lowest.saturating_sub(PIXEL_THRESHOLD)
            || source[index] > highest.saturating_add(PIXEL_THRESHOLD)
    };
    (0..expected.len())
        .map(|index| outside(expected, actual, index) || outside(actual, expected, index))
        .collect()
}

/// Draws the differing blocks in red over a faint copy of the PNG painter's page.
fn difference_map(
    columns: usize,
    rows: usize,
    expected: &[u8],
    differing: &[bool],
) -> Result<Vec<u8>, String> {
    let mut pixels = Vec::with_capacity(columns * rows * 3);
    for (expected, differs) in expected.iter().zip(differing) {
        if *differs {
            pixels.extend([220, 0, 0]);
        } else {
            let faint = 255 - (255 - expected) / 4;
            pixels.extend([faint, faint, faint]);
        }
    }
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, columns as u32, rows as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder
        .write_header()
        .and_then(|mut writer| writer.write_image_data(&pixels))
        .map_err(|error| error.to_string())?;
    Ok(bytes)
}

fn decode(bytes: &[u8]) -> Result<(usize, usize, Vec<u8>), String> {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA);
    let mut reader = decoder.read_info().map_err(|error| error.to_string())?;
    let mut pixels = vec![0; reader.output_buffer_size().ok_or("oversized PNG")?];
    let info = reader
        .next_frame(&mut pixels)
        .map_err(|error| error.to_string())?;
    pixels.truncate(info.buffer_size());
    Ok((info.width as usize, info.height as usize, pixels))
}

/// Returns, for each page, the share of inked blocks that differ between the
/// rasterized PDF and the PNG painter.
fn compare(
    subject: &office::Subject,
    images: Option<&std::path::Path>,
) -> Result<Vec<f64>, String> {
    let native = office::native(subject)?;
    let failure = |error: &dyn std::fmt::Display| format!("{}: {error}", subject.key);
    let pdf = painters::paint_pdf(&native.plan).map_err(|error| failure(&error))?;
    let pngs = painters::paint_png(
        &native.plan,
        &PngOptions {
            dots_per_inch: DOTS_PER_INCH,
            ..Default::default()
        },
    )
    .map_err(|error| failure(&error))?
    .output;
    let document = Pdf::new(pdf).map_err(|error| format!("{}: {error:?}", subject.key))?;
    let cache = RenderCache::new();
    let settings = InterpreterSettings {
        font_resolver: Arc::new(|query| match query {
            FontQuery::Standard(font) => standard_font(font).or_else(|| Some(font.get_font_data())),
            FontQuery::Fallback(fallback) => Some(fallback.pick_standard_font().get_font_data()),
        }),
        ..Default::default()
    };
    let scale = DOTS_PER_INCH as f32 / 72.0;
    let pages = document.pages();
    if pages.len() != pngs.len() {
        return Err(format!(
            "{}: {} PDF pages, {} PNG pages",
            subject.key,
            pages.len(),
            pngs.len()
        ));
    }
    let mut shares = Vec::new();
    for (page, png) in pages.iter().zip(&pngs) {
        let pixmap = render(
            page,
            &cache,
            &settings,
            &RenderSettings {
                hinting: false,
                ..Default::default()
            },
            &PixmapSettings {
                x_scale: scale,
                y_scale: scale,
                bg_color: WHITE,
            },
        );
        let (png_width, png_height, painted) = decode(&png.bytes)?;
        let pdf_width = usize::from(pixmap.width());
        let width = png_width.min(pdf_width);
        let height = png_height.min(usize::from(pixmap.height()));
        let expected = blocks(width, height, png_width, &painted);
        let actual = blocks(width, height, pdf_width, pixmap.data_as_u8_slice());
        let columns = width / 2;
        let differing = differing_blocks(columns, &expected, &actual);
        if let Some(directory) = images {
            let stem = format!("{}-{}", subject.key.replace('/', "-"), png.number);
            let write = |name: String, bytes: &[u8]| {
                std::fs::create_dir_all(directory)
                    .and_then(|_| std::fs::write(directory.join(name), bytes))
                    .map_err(|error| error.to_string())
            };
            write(format!("{stem}-png.png"), &png.bytes)?;
            write(
                format!("{stem}-difference.png"),
                &difference_map(columns, height / 2, &expected, &differing)?,
            )?;
            write(
                format!("{stem}-pdf.png"),
                &pixmap.into_png().map_err(|error| error.to_string())?,
            )?;
        }
        let inked = expected
            .iter()
            .zip(&actual)
            .filter(|(expected, actual)| **expected < 224 || **actual < 224)
            .count();
        let different = differing.iter().filter(|differs| **differs).count();
        shares.push(different as f64 / inked.max(1) as f64);
    }
    Ok(shares)
}

fn allowances_path() -> std::path::PathBuf {
    crate::repository().join("tests/fixtures/painter-parity/allowances.json")
}

/// Returns, for each subject, the largest share of a page's inked blocks that may
/// differ between the painters.
fn read_allowances() -> Result<BTreeMap<String, f64>, String> {
    let Ok(text) = std::fs::read_to_string(allowances_path()) else {
        return Ok(BTreeMap::new());
    };
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

/// Rounds a share up to the next tenth of a percent.
fn allowance(share: f64) -> f64 {
    (share * 1000.0).ceil() / 1000.0
}

pub(crate) fn run(mut arguments: Arguments) -> Outcome {
    let all = office::subjects()?;
    let mut selected = Vec::new();
    let mut images = None;
    let mut allowing = false;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(ExitCode::SUCCESS);
            }
            "--case" => {
                let identifier = arguments.value("--case")?;
                if !all.iter().any(|(candidate, _)| *candidate == identifier) {
                    return Err(format!("unknown subject: {identifier}"));
                }
                selected.push(identifier);
            }
            "--images" => images = Some(arguments.path("--images")?),
            "--allow" => allowing = true,
            other => return Err(format!("unknown option: {other}; use --help")),
        }
    }
    let subjects: Vec<&office::Subject> = all
        .iter()
        .filter(|(identifier, _)| selected.is_empty() || selected.contains(identifier))
        .map(|(_, subject)| subject)
        .collect();
    let mut allowances = read_allowances()?;
    let mut failed = false;
    let results = office::in_parallel(&subjects, |subject| compare(subject, images.as_deref()));
    for (subject, shares) in subjects.iter().zip(results) {
        let shares = shares?;
        let worst = shares.iter().copied().fold(0.0, f64::max);
        let allowed = allowances.get(&subject.key).copied();
        let exceeded = allowed.is_none_or(|allowed| worst > allowed + SLACK);
        println!(
            "{} | {}{}",
            subject.key,
            shares
                .iter()
                .map(|share| format!("{:.2}%", share * 100.0))
                .collect::<Vec<_>>()
                .join(" "),
            match allowed {
                _ if allowing => String::new(),
                None => "  no allowance".into(),
                Some(allowed) if exceeded => format!("  exceeds {:.1}%", allowed * 100.0),
                Some(_) => String::new(),
            }
        );
        if allowing {
            allowances.insert(subject.key.clone(), allowance(worst));
        } else {
            failed |= exceeded;
        }
    }
    if allowing {
        let path = allowances_path();
        std::fs::create_dir_all(path.parent().unwrap()).map_err(|error| error.to_string())?;
        std::fs::write(
            path,
            serde_json::to_string_pretty(&allowances).unwrap() + "\n",
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_native_pdf_and_png_painters_draw_every_page_alike() {
        let allowances = read_allowances().unwrap();
        let all = office::subjects().unwrap();
        let subjects: Vec<&office::Subject> = all.iter().map(|(_, subject)| subject).collect();
        let mut failures = Vec::new();
        for (subject, shares) in subjects
            .iter()
            .zip(office::in_parallel(&subjects, |subject| {
                compare(subject, None)
            }))
        {
            let worst = shares.unwrap().into_iter().fold(0.0, f64::max);
            match allowances.get(&subject.key) {
                None => failures.push(format!("{}: no allowance", subject.key)),
                Some(allowed) if worst > allowed + SLACK => failures.push(format!(
                    "{}: {:.2}% of inked blocks differ, above {:.1}%",
                    subject.key,
                    worst * 100.0,
                    allowed * 100.0
                )),
                Some(_) => {}
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
