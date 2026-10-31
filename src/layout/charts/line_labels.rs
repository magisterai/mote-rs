//! Keep line labels near their points without covering other labels or marks.
use super::{LayoutError, Planner, geometry::Rectangle, text::text_height};

pub(super) struct LineLabel {
    pub point: [f64; 2],
    pub text: String,
}

impl Planner<'_, '_> {
    pub(super) fn place_line_labels(
        &mut self,
        labels: &[LineLabel],
        series_points: &[Vec<[f64; 2]>],
        plot: Rectangle,
    ) -> Result<(), LayoutError> {
        // Place the higher point first, regardless of series order, so a nearby
        // lower point can put its label below instead of between the two marks.
        let mut order: Vec<_> = (0..labels.len()).collect();
        order.sort_by(|&first, &second| {
            labels[first].point[1]
                .total_cmp(&labels[second].point[1])
                .then(labels[first].point[0].total_cmp(&labels[second].point[0]))
        });
        let mut occupied = Vec::new();
        let mut placements = vec![Rectangle::default(); labels.len()];
        for index in order {
            let label = &labels[index];
            if label.text.trim().is_empty() {
                continue;
            }
            let plan = self.fit_text(
                &label.text,
                10.0,
                "center",
                Rectangle::new(0.0, 0.0, 56.0, 16.0),
            )?;
            let width = plan.lines.iter().map(|line| line.width).fold(0.0, f64::max);
            let height = text_height(&plan);
            let (frame, bounds) =
                place_line_label(label.point, [width, height], plot, &occupied, series_points);
            placements[index] = frame;
            occupied.push(bounds);
        }
        for (label, frame) in labels.iter().zip(placements) {
            self.text(frame, &label.text, 10.0, "center", 0.0)?;
        }
        Ok(())
    }
}

fn place_line_label(
    point: [f64; 2],
    size: [f64; 2],
    plot: Rectangle,
    occupied: &[Rectangle],
    series_points: &[Vec<[f64; 2]>],
) -> (Rectangle, Rectangle) {
    let side_gap = (size[0] / 2.0 + 2.0).max(5.0);
    let frames = [
        Rectangle::new(point[0] - 28.0, point[1] - 19.0, 56.0, 16.0),
        Rectangle::new(point[0] - 28.0, point[1] + 3.0, 56.0, 16.0),
        Rectangle::new(
            point[0] + side_gap + size[0] / 2.0 - 28.0,
            point[1] - 8.0,
            56.0,
            16.0,
        ),
        Rectangle::new(
            point[0] - side_gap - size[0] / 2.0 - 28.0,
            point[1] - 8.0,
            56.0,
            16.0,
        ),
    ];
    let mut best = (frames[0], Rectangle::default());
    let mut best_score = f64::INFINITY;
    for frame in frames {
        // Collision checks use measured text, not the much wider clipping frame.
        // One point on each side keeps adjacent labels visibly separated.
        let bounds = Rectangle::new(
            frame.left + (frame.width - size[0]) / 2.0 - 1.0,
            frame.top + (frame.height - size[1]) / 2.0 - 1.0,
            size[0] + 2.0,
            size[1] + 2.0,
        );
        let outside = (bounds.width * bounds.height - overlap_area(bounds, plot)).max(0.0);
        let mut score = outside * 10.0;
        for &earlier in occupied {
            score += overlap_area(bounds, earlier) * 10.0;
        }
        for points in series_points {
            for point in points {
                score += overlap_area(
                    bounds,
                    Rectangle::new(point[0] - 3.5, point[1] - 3.5, 7.0, 7.0),
                ) * 10.0;
            }
            for segment in points.windows(2) {
                let stroke_bounds = Rectangle::new(
                    bounds.left - 1.0,
                    bounds.top - 1.0,
                    bounds.width + 2.0,
                    bounds.height + 2.0,
                );
                if segment_intersects(stroke_bounds, segment[0], segment[1]) {
                    score += bounds.width * bounds.height;
                }
            }
        }
        if score < best_score {
            best = (frame, bounds);
            best_score = score;
        }
        if score <= 1e-9 {
            break;
        }
    }
    // Dense charts can exhaust all four local positions. Preserve every label
    // and choose the least obstructed one rather than moving it far from its point.
    best
}

fn overlap_area(first: Rectangle, second: Rectangle) -> f64 {
    let width =
        (first.left + first.width).min(second.left + second.width) - first.left.max(second.left);
    let height =
        (first.top + first.height).min(second.top + second.height) - first.top.max(second.top);
    width.max(0.0) * height.max(0.0)
}

fn segment_intersects(bounds: Rectangle, start: [f64; 2], end: [f64; 2]) -> bool {
    let mut entry: f64 = 0.0;
    let mut exit: f64 = 1.0;
    for (axis, minimum, maximum) in [
        (0, bounds.left, bounds.left + bounds.width),
        (1, bounds.top, bounds.top + bounds.height),
    ] {
        let direction = end[axis] - start[axis];
        if direction == 0.0 {
            if start[axis] < minimum || start[axis] > maximum {
                return false;
            }
        } else {
            let first = (minimum - start[axis]) / direction;
            let second = (maximum - start[axis]) / direction;
            entry = entry.max(first.min(second));
            exit = exit.min(first.max(second));
            if entry > exit {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_keep_clear_defaults_and_use_below_when_above_is_occupied() {
        let plot = Rectangle::new(0.0, 0.0, 200.0, 200.0);
        let (first, occupied) = place_line_label([100.0, 90.0], [12.0, 12.0], plot, &[], &[]);
        assert_eq!(first.top, 71.0);
        let (second, bounds) =
            place_line_label([100.0, 94.0], [12.0, 12.0], plot, &[occupied], &[]);
        assert_eq!(second.top, 97.0);
        assert_eq!(overlap_area(bounds, occupied), 0.0);
    }

    #[test]
    fn coincident_points_can_use_four_distinct_local_positions() {
        let plot = Rectangle::new(0.0, 0.0, 200.0, 200.0);
        let points = vec![vec![[100.0, 100.0]]];
        let mut occupied = Vec::new();
        for _ in 0..4 {
            let (_, bounds) =
                place_line_label([100.0, 100.0], [12.0, 12.0], plot, &occupied, &points);
            for &previous in &occupied {
                assert_eq!(overlap_area(bounds, previous), 0.0);
            }
            occupied.push(bounds);
        }
    }

    #[test]
    fn labels_avoid_plot_edges_and_nearby_series_segments() {
        let plot = Rectangle::new(0.0, 0.0, 200.0, 200.0);
        for (point, expected_top) in [([100.0, 0.0], 3.0), ([100.0, 200.0], 181.0)] {
            let (frame, bounds) = place_line_label(point, [12.0, 12.0], plot, &[], &[]);
            assert_eq!(frame.top, expected_top);
            assert!((overlap_area(bounds, plot) - bounds.width * bounds.height).abs() < 1e-9);
        }
        let crossing = vec![vec![[80.0, 89.0], [120.0, 89.0]]];
        let (frame, bounds) = place_line_label([100.0, 100.0], [12.0, 12.0], plot, &[], &crossing);
        assert_eq!(frame.top, 103.0);
        assert!(!segment_intersects(bounds, crossing[0][0], crossing[0][1]));
    }

    #[test]
    fn crowded_fallback_stays_local_and_preserves_finite_geometry() {
        let plot = Rectangle::new(0.0, 0.0, 4.0, 4.0);
        let (frame, bounds) = place_line_label([2.0, 2.0], [40.0, 12.0], plot, &[plot], &[]);
        assert!(frame.bounds().iter().all(|value| value.is_finite()));
        assert_eq!(bounds.width, 42.0);
        assert!((frame.center()[0] - 2.0).abs() <= 42.0);
        assert!((frame.center()[1] - 2.0).abs() <= 11.0);
    }
}
