//! Pairs the words of an Office rendering with the same words of the native
//! rendering, and measures how far the lines that carry them lie apart.
//!
//! Words rather than whole lines are matched, because two renderers may divide a
//! row of text into lines differently where a wide gap, such as a tab, separates
//! its parts.
use super::lines::{Page, is_marker};
use std::collections::BTreeSet;

/// Reduces a word to its lowercase letters and digits, so that punctuation and
/// glyph substitutions do not prevent a match.
fn key(word: &str) -> String {
    word.chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Comparison {
    pub office_pages: usize,
    pub native_pages: usize,
    /// Lines of either rendering none of whose words found a partner.
    pub unmatched: usize,
    /// Lines whose matched words lie on another page.
    pub page_moves: usize,
    pub maximum_vertical: f64,
    pub median_vertical: f64,
    pub maximum_horizontal: f64,
    /// The first line that lies more than half a point from Office.
    pub first_difference: Option<String>,
    /// Unmatched lines, and matched lines more than half a point apart, for
    /// diagnosis.
    pub details: Vec<String>,
}

struct Word {
    key: String,
    /// The index of the word's line among all lines of the rendering.
    line: usize,
    /// Whether the word begins its line, after any list marker.
    first: bool,
}

struct LineEntry<'page> {
    page: usize,
    x: f64,
    baseline: f64,
    text: &'page str,
}

fn words(pages: &[Page]) -> (Vec<LineEntry<'_>>, Vec<Word>) {
    let mut lines = Vec::new();
    let mut words = Vec::new();
    for (page, content) in pages.iter().enumerate() {
        for line in &content.lines {
            let index = lines.len();
            lines.push(LineEntry {
                page,
                x: line.x,
                baseline: line.baseline,
                text: &line.text,
            });
            let mut tokens: Vec<&str> = line.text.split_whitespace().collect();
            if tokens.len() > 1 && is_marker(tokens[0]) {
                tokens.remove(0);
            }
            let mut first = true;
            for token in tokens {
                let key = key(token);
                if key.is_empty() {
                    continue;
                }
                words.push(Word {
                    key,
                    line: index,
                    first,
                });
                first = false;
            }
        }
    }
    (lines, words)
}

/// Returns the index pairs of a longest common subsequence of the two key lists.
fn common_subsequence(first: &[Word], second: &[Word]) -> Vec<(usize, usize)> {
    let width = second.len() + 1;
    let mut lengths = vec![0u32; (first.len() + 1) * width];
    for row in (0..first.len()).rev() {
        for column in (0..second.len()).rev() {
            lengths[row * width + column] = if first[row].key == second[column].key {
                lengths[(row + 1) * width + column + 1] + 1
            } else {
                lengths[(row + 1) * width + column].max(lengths[row * width + column + 1])
            };
        }
    }
    let (mut row, mut column) = (0, 0);
    let mut pairs = Vec::new();
    while row < first.len() && column < second.len() {
        if first[row].key == second[column].key {
            pairs.push((row, column));
            row += 1;
            column += 1;
        } else if lengths[(row + 1) * width + column] >= lengths[row * width + column + 1] {
            row += 1;
        } else {
            column += 1;
        }
    }
    pairs
}

pub(crate) fn compare(office: &[Page], native: &[Page]) -> Comparison {
    let (office_lines, office_words) = words(office);
    let (native_lines, native_words) = words(native);
    let pairs = common_subsequence(&office_words, &native_words);
    let mut comparison = Comparison {
        office_pages: office.len(),
        native_pages: native.len(),
        ..Default::default()
    };
    // Each Office line takes the vertical offset of its first matched word, and
    // its horizontal offset where that word begins both lines.
    let mut line_pairs: Vec<(usize, usize, Option<f64>)> = Vec::new();
    let mut matched_office = BTreeSet::new();
    for &(office_index, native_index) in &pairs {
        let (office_word, native_word) = (&office_words[office_index], &native_words[native_index]);
        if matched_office.insert(office_word.line) {
            let (office_line, native_line) = (
                &office_lines[office_word.line],
                &native_lines[native_word.line],
            );
            // Two lines of as many words that differ in a word hold generated
            // text, such as the date of the export, whose width moves the
            // start of a centred or right-aligned line.
            let keys = |text: &str| text.split_whitespace().map(key).collect::<Vec<_>>();
            let (office_keys, native_keys) = (keys(office_line.text), keys(native_line.text));
            let generated = office_keys.len() == native_keys.len() && office_keys != native_keys;
            let horizontal = (office_word.first && native_word.first && !generated)
                .then_some(office_line.x - native_line.x);
            line_pairs.push((office_word.line, native_word.line, horizontal));
        }
    }
    let matched_native: BTreeSet<usize> = pairs
        .iter()
        .map(|&(_, index)| native_words[index].line)
        .collect();
    let with_words = |words: &[Word]| words.iter().map(|word| word.line).collect::<BTreeSet<_>>();
    for (label, lines, candidates, matched) in [
        (
            "Office only",
            &office_lines,
            with_words(&office_words),
            &matched_office,
        ),
        (
            "native only",
            &native_lines,
            with_words(&native_words),
            &matched_native,
        ),
    ] {
        for index in candidates.difference(matched) {
            let line = &lines[*index];
            comparison.unmatched += 1;
            comparison.details.push(format!(
                "{label}: page {} x {:.1} baseline {:.1} “{}”",
                line.page + 1,
                line.x,
                line.baseline,
                line.text
            ));
        }
    }
    let mut vertical = Vec::new();
    for (office_index, native_index, horizontal) in line_pairs {
        let (office_line, native_line) = (&office_lines[office_index], &native_lines[native_index]);
        if office_line.page != native_line.page {
            comparison.page_moves += 1;
            continue;
        }
        let difference = office_line.baseline - native_line.baseline;
        vertical.push(difference.abs());
        let horizontal = horizontal.unwrap_or(0.0);
        comparison.maximum_horizontal = comparison.maximum_horizontal.max(horizontal.abs());
        if difference.abs() > 0.5 || horizontal.abs() > 0.5 {
            comparison.details.push(format!(
                "offset: page {} {difference:+.2}pt down {horizontal:+.2}pt across “{}”",
                native_line.page + 1,
                office_line.text
            ));
        }
        if comparison.first_difference.is_none() && difference.abs() > 0.5 {
            comparison.first_difference = Some(format!(
                "page {} {:+.1}pt “{}”",
                native_line.page + 1,
                difference,
                office_line.text.chars().take(50).collect::<String>()
            ));
        }
    }
    if !vertical.is_empty() {
        vertical.sort_by(f64::total_cmp);
        comparison.maximum_vertical = vertical[vertical.len() - 1];
        comparison.median_vertical = vertical[vertical.len() / 2];
    }
    comparison
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::office::lines::Line;

    fn page(lines: &[(&str, f64, f64)]) -> Page {
        Page {
            width: 612.0,
            height: 792.0,
            lines: lines
                .iter()
                .map(|(text, x, baseline)| Line {
                    text: (*text).into(),
                    x: *x,
                    baseline: *baseline,
                })
                .collect(),
        }
    }

    #[test]
    fn punctuation_and_list_markers_do_not_prevent_a_match() {
        let office = [page(&[
            ("• Alpha, beta.", 90.0, 80.0),
            ("3. Third item", 90.0, 100.0),
        ])];
        let native = [page(&[
            ("Alpha beta", 90.0, 80.0),
            ("3. Third item", 90.0, 100.0),
        ])];
        let comparison = compare(&office, &native);
        assert_eq!(comparison.unmatched, 0);
        assert_eq!(comparison.maximum_vertical, 0.0);
    }

    #[test]
    fn rows_divided_differently_still_match_word_by_word() {
        let office = [page(&[("Label", 72.0, 80.0), ("value one", 200.0, 80.0)])];
        let native = [page(&[("Label value one", 72.0, 80.5)])];
        let comparison = compare(&office, &native);
        assert_eq!(comparison.unmatched, 0);
        assert!((comparison.maximum_vertical - 0.5).abs() < 1e-9);
    }

    #[test]
    fn matched_lines_report_their_offsets_and_page_moves() {
        let office = [
            page(&[("One", 72.0, 80.0), ("Two", 72.0, 100.0)]),
            page(&[("Three", 72.0, 80.0)]),
        ];
        let native = [page(&[
            ("One", 72.0, 80.4),
            ("Two", 73.0, 99.0),
            ("Three", 72.0, 120.0),
        ])];
        let comparison = compare(&office, &native);
        assert_eq!(comparison.page_moves, 1);
        assert!((comparison.maximum_vertical - 1.0).abs() < 1e-9);
        assert!((comparison.maximum_horizontal - 1.0).abs() < 1e-9);
        assert_eq!(
            comparison.first_difference.as_deref(),
            Some("page 1 +1.0pt “Two”")
        );
    }
}
