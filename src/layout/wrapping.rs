use super::{LayoutError, LineBreak, LineOptions, TextRun};
use crate::measurement::TextMeasurer;
use std::collections::BTreeSet;
use unicode_segmentation::UnicodeSegmentation;

pub(super) struct WrappedLine {
    pub runs: Vec<TextRun>,
    pub line_break: LineBreak,
}

pub(super) fn measure(
    runs: &[TextRun],
    measurer: &mut TextMeasurer<'_>,
    kerning: bool,
) -> Result<f64, LayoutError> {
    let mut width = 0.0;
    for run in runs {
        width += if let Some(advance) = run.fixed_advance {
            advance
        } else {
            measurer
                .measure(&run.style.measurement, &run.text, kerning)?
                .measurement
                .layout_width
        };
    }
    Ok(width)
}

pub(super) fn join(runs: &[TextRun]) -> String {
    runs.iter().map(|run| run.text.as_str()).collect()
}

pub(super) fn slice(runs: &[TextRun], start: usize, end: usize) -> Vec<TextRun> {
    let mut position = 0;
    let mut output = Vec::new();
    for run in runs {
        let run_end = position + run.text.len();
        let start = start.max(position);
        let end = end.min(run_end);
        if start < end {
            let mut piece = run.with_text(&run.text[start - position..end - position]);
            if start > position {
                piece.anchors.clear();
                piece.footnote_number = None;
            }
            output.push(piece);
        }
        position = run_end;
    }
    output
}

fn trim(runs: &[TextRun]) -> Vec<TextRun> {
    let mut output = runs.to_vec();
    while let Some(run) = output.first_mut() {
        if run.preserve_space {
            break;
        }
        run.text = run.text.trim_start_matches([' ', '\t', '\r']).into();
        if !run.text.is_empty() {
            break;
        }
        output.remove(0);
    }
    while let Some(run) = output.last_mut() {
        if run.preserve_space {
            break;
        }
        run.text = run.text.trim_end_matches([' ', '\t', '\r']).into();
        if !run.text.is_empty() {
            break;
        }
        output.pop();
    }
    output
}

pub(super) fn break_offsets(text: &str) -> BTreeSet<usize> {
    let graphemes: BTreeSet<_> = text
        .grapheme_indices(true)
        .map(|(offset, cluster)| offset + cluster.len())
        .collect();
    unicode_linebreak::linebreaks(text)
        .map(|(offset, _)| offset)
        .filter(|offset| graphemes.contains(offset))
        .collect()
}

pub(super) fn groups(runs: &[TextRun]) -> Vec<Vec<TextRun>> {
    let mut normalized = Vec::new();
    let mut pending_carriage_return = false;
    for run in runs {
        let mut text = run.text.as_str();
        if pending_carriage_return && text.starts_with('\n') {
            text = &text[1..];
        }
        if !run.text.is_empty() {
            pending_carriage_return = run.text.ends_with('\r');
        }
        let text = text.replace("\r\n", "\n").replace('\r', "\n");
        if !text.is_empty() {
            normalized.push(run.with_text(&text));
        }
    }
    let text = join(&normalized);
    let mut offsets = break_offsets(&text);
    for (offset, character) in text.char_indices() {
        if matches!(character, '\n' | '\t') && offset > 0 {
            offsets.insert(offset);
        }
        if matches!(character, ' ' | '\n' | '\t') {
            offsets.insert(offset + character.len_utf8());
        }
    }
    let mut position = 0;
    for run in &normalized {
        if run.style.code {
            for (offset, cluster) in run.text.grapheme_indices(true) {
                offsets.insert(position + offset + cluster.len());
            }
        }
        position += run.text.len();
    }
    offsets.insert(text.len());
    let mut start = 0;
    offsets
        .into_iter()
        .filter_map(|end| {
            if end <= start {
                return None;
            }
            let group = slice(&normalized, start, end);
            start = end;
            Some(group)
        })
        .collect()
}

pub(super) fn wrap(
    runs: &[TextRun],
    options: &LineOptions,
    measurer: &mut TextMeasurer<'_>,
) -> Result<Vec<WrappedLine>, LayoutError> {
    let mut lines = Vec::new();
    let mut current = Vec::new();
    let mut preserved_hard_break = false;
    for mut group in groups(runs) {
        let text = join(&group);
        if text == "\n" {
            preserved_hard_break = group.iter().any(|run| run.preserve_space);
            flush(&mut current, &mut lines, LineBreak::Hard, true);
            continue;
        }
        preserved_hard_break = false;
        if text == "\t" {
            let mut tab = group[0].with_text("    ");
            tab.synthetic_tab = true;
            group = vec![tab];
        }
        if options.no_wrap {
            current.extend(group);
            continue;
        }
        while !group.is_empty() {
            let width = (options.width
                - if lines.is_empty() {
                    options.first_indent
                } else {
                    0.0
                })
            .max(1.0);
            let text = join(&group);
            let whitespace = text.trim().is_empty();
            let preserved = group.iter().any(|run| run.preserve_space);
            let candidate: Vec<_> = current.iter().chain(&group).cloned().collect();
            if (!whitespace || preserved)
                && measure(&trim(&candidate), measurer, options.layout_kerning)? > width
                && !trim(&current).is_empty()
            {
                flush(&mut current, &mut lines, LineBreak::Soft, false);
                continue;
            }
            if (!whitespace || preserved)
                && measure(&group, measurer, options.layout_kerning)? > width
            {
                let mut end = 0;
                for (offset, cluster) in text.grapheme_indices(true) {
                    let next = offset + cluster.len();
                    if end > 0
                        && measure(&slice(&group, 0, next), measurer, options.layout_kerning)?
                            > width
                    {
                        break;
                    }
                    end = next;
                }
                current.extend(slice(&group, 0, end));
                group = slice(&group, end, text.len());
                if !group.is_empty() {
                    flush(&mut current, &mut lines, LineBreak::Soft, false);
                }
                continue;
            }
            if current.is_empty() && whitespace && !preserved {
                break;
            }
            current.append(&mut group);
        }
    }
    flush(
        &mut current,
        &mut lines,
        LineBreak::Final,
        preserved_hard_break,
    );
    if lines.is_empty() {
        lines.push(WrappedLine {
            runs: Vec::new(),
            line_break: LineBreak::Final,
        });
    }
    Ok(lines)
}

fn flush(
    current: &mut Vec<TextRun>,
    lines: &mut Vec<WrappedLine>,
    line_break: LineBreak,
    force: bool,
) {
    let runs = trim(current);
    if !runs.is_empty() || force {
        lines.push(WrappedLine { runs, line_break });
    }
    current.clear();
}
