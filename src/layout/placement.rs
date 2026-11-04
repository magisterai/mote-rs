use super::{Alignment, LayoutError, LineBreak, TabAlignment, TabStop, TextRun, wrapping};
use crate::measurement::TextMeasurer;
use unicode_script::{Script, UnicodeScript};

pub(super) struct PlacedRun {
    pub run: TextRun,
    pub x: f64,
    pub width: f64,
    pub advance: f64,
}

/// The horizontal frame of one wrapped line: its measure, its start and the
/// rules that distribute the runs within it.
pub(super) struct LineFrame<'a> {
    pub width: f64,
    pub origin: f64,
    pub alignment: Alignment,
    pub line_break: LineBreak,
    pub kerning: bool,
    pub tab_stops: Option<&'a [TabStop]>,
    pub tab_origin: f64,
}

/// Fragments of runs, each marked when justification may stretch the gap after it.
type Fragments = Vec<(TextRun, bool)>;

pub(super) fn place(
    runs: &[TextRun],
    frame: &LineFrame<'_>,
    measurer: &mut TextMeasurer<'_>,
) -> Result<(f64, f64, Vec<PlacedRun>), LayoutError> {
    let &LineFrame {
        width,
        origin,
        alignment,
        line_break,
        kerning,
        tab_stops,
        ..
    } = frame;
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
    let mut fragments: Fragments = Vec::new();
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
    let (fragments, widths) = match tab_stops {
        Some(tab_stops) => resolve_tabs(fragments, widths, frame, tab_stops, measurer)?,
        None => (fragments, widths),
    };
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

/// Word's default stop interval.
const DEFAULT_TAB_INTERVAL: f64 = 36.0;

/// Right, centre and decimal stops align the text that follows, up to the next tab.
fn resolve_tabs(
    fragments: Fragments,
    widths: Vec<f64>,
    frame: &LineFrame<'_>,
    tab_stops: &[TabStop],
    measurer: &mut TextMeasurer<'_>,
) -> Result<(Fragments, Vec<f64>), LayoutError> {
    let &LineFrame {
        origin,
        kerning,
        tab_origin,
        ..
    } = frame;
    let is_tab = |run: &TextRun| {
        run.synthetic_tab && run.fixed_advance.is_none() && run.text.trim().is_empty()
    };
    if !fragments.iter().any(|(run, _)| is_tab(run)) {
        return Ok((fragments, widths));
    }
    let mut resolved_fragments = Vec::with_capacity(fragments.len());
    let mut resolved_widths = Vec::with_capacity(widths.len());
    let mut cursor = origin;
    for index in 0..fragments.len() {
        let (run, stretch) = &fragments[index];
        if !is_tab(run) {
            cursor += widths[index];
            resolved_fragments.push((run.clone(), *stretch));
            resolved_widths.push(widths[index]);
            continue;
        }
        let column = cursor + tab_origin;
        let stop = tab_stops
            .iter()
            .find(|stop| stop.position > column + 0.01)
            .cloned()
            .unwrap_or(TabStop {
                position: ((column / DEFAULT_TAB_INTERVAL).floor() + 1.0) * DEFAULT_TAB_INTERVAL,
                alignment: TabAlignment::Left,
                leader: None,
            });
        let mut aligned = 0.0;
        for (following, width) in fragments[index + 1..].iter().zip(&widths[index + 1..]) {
            if is_tab(&following.0) {
                break;
            }
            if stop.alignment == TabAlignment::Decimal
                && let Some(point) = following.0.text.find('.')
            {
                aligned += measurer
                    .measure(
                        &following.0.style.measurement,
                        &following.0.text[..point],
                        kerning,
                    )?
                    .measurement
                    .layout_width;
                break;
            }
            aligned += width;
        }
        let target = stop.position - tab_origin;
        let advance = match stop.alignment {
            TabAlignment::Left => target - cursor,
            TabAlignment::Right | TabAlignment::Decimal => target - aligned - cursor,
            TabAlignment::Center => target - aligned / 2.0 - cursor,
        }
        .max(0.0);
        cursor += advance;
        let remaining = advance;
        if let Some(leader) = stop.leader {
            let mark = leader.to_string();
            let mark_width = measurer
                .measure(&run.style.measurement, &mark, kerning)?
                .measurement
                .layout_width;
            let count = if mark_width > 0.0 {
                ((advance - mark_width) / mark_width).floor().max(0.0) as usize
            } else {
                0
            };
            if count > 0 {
                let mut spacer = run.with_text(" ");
                spacer.fixed_advance = Some(advance - count as f64 * mark_width);
                resolved_widths.push(spacer.fixed_advance.unwrap());
                resolved_fragments.push((spacer, false));
                let mut marks = run.with_text(&mark.repeat(count));
                marks.synthetic_tab = false;
                resolved_widths.push(count as f64 * mark_width);
                resolved_fragments.push((marks, false));
                continue;
            }
        }
        let mut spacer = run.with_text(" ");
        spacer.fixed_advance = Some(remaining);
        resolved_widths.push(remaining);
        resolved_fragments.push((spacer, false));
    }
    Ok((resolved_fragments, resolved_widths))
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
