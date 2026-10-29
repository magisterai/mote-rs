const TOLERANCE: f64 = 0.001;

pub(super) fn fitting_lines(heights: &[f64], available: f64) -> usize {
    let mut used = 0.0;
    for (index, height) in heights.iter().enumerate() {
        if used + height > available + TOLERANCE {
            return index;
        }
        used += height;
    }
    heights.len()
}
pub(super) fn line_count(heights: &[f64], available: f64, maximum: f64, at_start: bool) -> usize {
    if heights.is_empty() {
        return 0;
    }
    let natural = fitting_lines(heights, available);
    if natural == heights.len() {
        return natural;
    }
    if natural == 0 {
        return usize::from(at_start);
    }
    let partitions = fresh_partitions(heights, maximum);
    let maximum_current = natural.min(heights.len().saturating_sub(2));
    for count in (2..=maximum_current).rev() {
        if partitions[count] {
            return count;
        }
    }
    if !at_start && (partitions[0] || fitting_lines(heights, maximum) > natural) {
        return 0;
    }
    natural
}
fn fresh_partitions(heights: &[f64], maximum: f64) -> Vec<bool> {
    let mut possible = vec![false; heights.len() + 1];
    if heights.len() < 2 {
        return possible;
    }
    for start in (0..=heights.len() - 2).rev() {
        let mut height = 0.0;
        for end in start..heights.len() {
            height += heights[end];
            if height > maximum + TOLERANCE {
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
    if keep_together || (widow_control && heights.len() <= 3) {
        heights.iter().sum()
    } else if widow_control && heights.len() >= 4 {
        heights[..2].iter().sum()
    } else {
        heights.first().copied().unwrap_or(0.0)
    }
}
