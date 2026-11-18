use super::tolerances::FIT_TOLERANCE;

/// Counts the leading lines that `available` holds. The trailing space of the
/// last of them need not fit, since Word requires of a line at the foot of a
/// page only the part above that space.
pub(super) fn fitting_lines(heights: &[f64], trailing_spaces: &[f64], available: f64) -> usize {
    let mut used = 0.0;
    for (index, height) in heights.iter().enumerate() {
        if used + height - trailing_spaces[index] > available + FIT_TOLERANCE {
            return index;
        }
        used += height;
    }
    heights.len()
}
pub(super) fn line_count(
    heights: &[f64],
    trailing_spaces: &[f64],
    available: f64,
    maximum: f64,
    at_start: bool,
) -> usize {
    if heights.is_empty() {
        return 0;
    }
    let natural = fitting_lines(heights, trailing_spaces, available);
    if natural == heights.len() {
        return natural;
    }
    if natural == 0 {
        return usize::from(at_start);
    }
    let partitions = fresh_partitions(heights, trailing_spaces, maximum);
    let maximum_current = natural.min(heights.len().saturating_sub(2));
    for count in (2..=maximum_current).rev() {
        if partitions[count] {
            return count;
        }
    }
    if !at_start && (partitions[0] || fitting_lines(heights, trailing_spaces, maximum) > natural) {
        return 0;
    }
    natural
}
fn fresh_partitions(heights: &[f64], trailing_spaces: &[f64], maximum: f64) -> Vec<bool> {
    let mut possible = vec![false; heights.len() + 1];
    if heights.len() < 2 {
        return possible;
    }
    for start in (0..=heights.len() - 2).rev() {
        let mut height = 0.0;
        for end in start..heights.len() {
            height += heights[end];
            if height - trailing_spaces[end] > maximum + FIT_TOLERANCE {
                break;
            }
            if end - start + 1 >= 2 && (end == heights.len() - 1 || possible[end + 1]) {
                possible[start] = true;
                break;
            }
        }
    }
    possible
}
pub fn minimum_start_height(heights: &[f64], widow_control: bool, keep_together: bool) -> f64 {
    heights[..minimum_start_line_count(heights.len(), widow_control, keep_together)]
        .iter()
        .sum()
}
/// Returns the number of lines of a paragraph of `line_count` lines that must
/// share the page on which the paragraph starts.
pub(super) fn minimum_start_line_count(
    line_count: usize,
    widow_control: bool,
    keep_together: bool,
) -> usize {
    if keep_together || (widow_control && line_count <= 3) {
        line_count
    } else if widow_control {
        2
    } else {
        line_count.min(1)
    }
}
