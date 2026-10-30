use super::Planner;

impl Planner<'_, '_> {
    pub(super) fn path(
        &mut self,
        points: Vec<[f64; 2]>,
        closed: bool,
        fill: Option<&str>,
        stroke: Option<(&str, f64)>,
    ) {
        use crate::layout::{LineCap, PageOperation, PaintedPath, PathPaint, PathStroke};
        if points.len() < if closed { 3 } else { 2 } {
            return;
        }
        let paint = |color: &str| PathPaint {
            color: super::valid_color(color).unwrap_or("2563EB").into(),
            alpha: 255,
        };
        self.page.operations.push(PageOperation::Path(PaintedPath {
            points,
            closed,
            fill: fill.map(paint),
            stroke: stroke.map(|(color, width)| PathStroke {
                paint: paint(color),
                width,
                dash: Vec::new(),
                cap: LineCap::Flat,
            }),
        }));
    }
    pub(super) fn ellipse(&mut self, center: [f64; 2], size: [f64; 2], steps: usize, color: &str) {
        let steps = steps.max(8);
        let points = (0..steps)
            .map(|index| {
                let angle = 2.0 * std::f64::consts::PI * index as f64 / steps as f64;
                [
                    center[0] + angle.cos() * size[0] / 2.0,
                    center[1] + angle.sin() * size[1] / 2.0,
                ]
            })
            .collect();
        self.path(points, true, Some(color), None);
    }
}
