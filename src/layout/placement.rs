use super::{Alignment, LayoutError, LineBreak, TextRun, wrapping};
use crate::measurement::TextMeasurer;
use unicode_script::{Script, UnicodeScript};

pub(super) struct PlacedRun {
    pub run: TextRun,
    pub x: f64,
    pub width: f64,
    pub advance: f64,
}

pub(super) fn place(
    runs: &[TextRun],
    width: f64,
    origin: f64,
    alignment: Alignment,
    line_break: LineBreak,
    measurer: &mut TextMeasurer<'_>,
    kerning: bool,
) -> Result<(f64, f64, Vec<PlacedRun>), LayoutError> {
    let text = wrapping::join(runs);
    let mut gaps = Vec::new();
    if alignment == Alignment::Justify && line_break == LineBreak::Soft {
        let start = text.len() - text.trim_start_matches(' ').len();
        let end = text.trim_end_matches(' ').len();
        for (offset, character) in text.char_indices() {
            if character == ' ' && offset >= start && offset < end {
                gaps.push(offset + 1);
            }
        }
        gaps.retain(|offset| gap_allowed(runs, *offset));
        if gaps.is_empty() {
            for offset in wrapping::break_offsets(&text) {
                if offset == 0 || offset == text.len() {
                    continue;
                }
                let left = text[..offset].chars().next_back().unwrap();
                let right = text[offset..].chars().next().unwrap();
                if !left.is_whitespace()
                    && !right.is_whitespace()
                    && !left.is_control()
                    && !right.is_control()
                    && (east_asian(left) || east_asian(right))
                    && gap_allowed(runs, offset)
                {
                    gaps.push(offset);
                }
            }
        }
    }
    let mut position = 0;
    let mut fragments = Vec::new();
    for run in runs {
        let mut start = 0;
        for end in gaps
            .iter()
            .copied()
            .filter(|offset| *offset > position && *offset <= position + run.text.len())
            .map(|offset| offset - position)
        {
            fragments.push((run.with_text(&run.text[start..end]), true));
            start = end;
        }
        if start < run.text.len() {
            fragments.push((run.with_text(&run.text[start..]), false));
        }
        position += run.text.len();
    }
    let mut widths = Vec::new();
    for (run, _) in &fragments {
        widths.push(if let Some(advance) = run.fixed_advance {
            advance
        } else {
            measurer
                .measure(&run.style.measurement, &run.text, kerning)?
                .measurement
                .layout_width
        });
    }
    let natural: f64 = widths.iter().sum();
    let extra = if gaps.is_empty() {
        0.0
    } else {
        (width - natural).max(0.0) / gaps.len() as f64
    };
    let offset = match alignment {
        Alignment::Center => ((width - natural) / 2.0).max(0.0),
        Alignment::Right => (width - natural).max(0.0),
        _ => 0.0,
    };
    let mut cursor = origin + offset;
    let mut total = 0.0;
    let mut placed = Vec::new();
    for ((run, stretch), width) in fragments.into_iter().zip(widths) {
        let advance = width + if stretch { extra } else { 0.0 };
        placed.push(PlacedRun {
            run,
            x: cursor,
            width,
            advance,
        });
        cursor += advance;
        total += advance;
    }
    Ok((origin + offset, total, placed))
}

fn gap_allowed(runs: &[TextRun], offset: usize) -> bool {
    let mut position = 0;
    for run in runs {
        position += run.text.len();
        if offset <= position {
            return !run.preserve_space && !run.synthetic_tab;
        }
    }
    false
}

fn east_asian(character: char) -> bool {
    matches!(
        character.script(),
        Script::Han | Script::Hiragana | Script::Katakana | Script::Hangul | Script::Bopomofo
    )
}
