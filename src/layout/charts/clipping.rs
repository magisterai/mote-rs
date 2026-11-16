//! Marks beyond a stated value range are confined to the vertical extent of the plot.
type Point = [f64; 2];

fn crossing(first: Point, second: Point, level: f64) -> Point {
    let ratio = (level - first[1]) / (second[1] - first[1]);
    [first[0] + (second[0] - first[0]) * ratio, level]
}

/// The parts of an open line that lie between two levels, in their original order.
pub(super) fn clip_line(points: &[Point], top: f64, bottom: f64) -> Vec<Vec<Point>> {
    let mut parts: Vec<Vec<Point>> = Vec::new();
    let mut current: Vec<Point> = Vec::new();
    for pair in points.windows(2) {
        let (mut first, mut second) = (pair[0], pair[1]);
        let entirely_outside =
            (first[1] < top && second[1] < top) || (first[1] > bottom && second[1] > bottom);
        if entirely_outside {
            parts.push(std::mem::take(&mut current));
            continue;
        }
        for (level, above) in [(top, true), (bottom, false)] {
            let outside = |point: Point| {
                if above {
                    point[1] < level
                } else {
                    point[1] > level
                }
            };
            if outside(first) {
                first = crossing(pair[0], pair[1], level);
            }
            if outside(second) {
                second = crossing(pair[0], pair[1], level);
            }
        }
        if current.last() != Some(&first) {
            parts.push(std::mem::take(&mut current));
            current.push(first);
        }
        current.push(second);
    }
    parts.push(current);
    parts.retain(|part| part.len() > 1);
    parts
}

/// The part of a closed outline that lies between two levels.
pub(super) fn clip_outline(points: Vec<Point>, top: f64, bottom: f64) -> Vec<Point> {
    let mut outline = points;
    for (level, above) in [(top, true), (bottom, false)] {
        let inside = |point: Point| {
            if above {
                point[1] >= level
            } else {
                point[1] <= level
            }
        };
        let mut clipped = Vec::with_capacity(outline.len() + 2);
        for (index, &point) in outline.iter().enumerate() {
            let previous = outline[(index + outline.len() - 1) % outline.len()];
            if inside(previous) != inside(point) {
                clipped.push(crossing(previous, point, level));
            }
            if inside(point) {
                clipped.push(point);
            }
        }
        outline = clipped;
    }
    outline
}

#[cfg(test)]
mod tests {
    use super::{clip_line, clip_outline};

    #[test]
    fn a_line_leaving_and_re_entering_the_range_is_split_at_the_bounds() {
        let parts = clip_line(
            &[[0.0, 5.0], [10.0, -5.0], [20.0, -5.0], [30.0, 5.0]],
            0.0,
            10.0,
        );
        assert_eq!(
            parts,
            [vec![[0.0, 5.0], [5.0, 0.0]], vec![[25.0, 0.0], [30.0, 5.0]]]
        );
        let inside = [[0.0, 1.0], [10.0, 9.0], [20.0, 0.0]];
        assert_eq!(clip_line(&inside, 0.0, 10.0), [inside.to_vec()]);
        assert!(clip_line(&[[0.0, 20.0], [10.0, 30.0]], 0.0, 10.0).is_empty());
    }

    #[test]
    fn an_outline_is_cut_at_both_levels() {
        let outline = clip_outline(
            vec![[0.0, -10.0], [10.0, -10.0], [10.0, 20.0], [0.0, 20.0]],
            0.0,
            10.0,
        );
        assert_eq!(
            outline,
            [[0.0, 10.0], [0.0, 0.0], [10.0, 0.0], [10.0, 10.0]]
        );
    }
}
