//! Pie sectors, radial labels and the doughnut's composited center.
use super::radial_labels::{self, RadialLabel};
use super::{
    Chart, ChartPlanner, LayoutError,
    geometry::{ChartLayout, Rectangle},
};
use std::f64::consts::{PI, TAU};

impl ChartPlanner<'_, '_> {
    pub(super) fn pie_chart(
        &mut self,
        chart: &Chart,
        layout: &ChartLayout,
    ) -> Result<(), LayoutError> {
        let Some(series) = chart.series.first() else {
            return Ok(());
        };
        let total: f64 = series.values.iter().map(|value| value.max(0.0)).sum();
        if !total.is_finite() {
            return Err(LayoutError::Unsupported(
                "native pie total must be finite".into(),
            ));
        }
        if total <= 0.0 {
            return Ok(());
        }
        let radius = ((layout.plot.width.min(layout.plot.height) / 2.0 - 1.0) * 0.87).max(1.0);
        let center = layout.plot.center();
        let mut angle = -PI / 2.0;
        let mut labels = Vec::new();
        // Round accumulated boundaries once in both passes. An intermediate
        // rounding can change the number of five-degree sampling intervals.
        for (index, &value) in series.values.iter().enumerate() {
            let fraction = value.max(0.0) / total;
            let sweep = TAU * fraction;
            if chart.data_labels.enabled {
                let middle = angle + sweep / 2.0;
                let text = label(chart, index, value, total);
                let lines: Vec<_> = text.split('\n').collect();
                let mut width: f64 = 0.0;
                for line in &lines {
                    width = width.max(self.measurement.text_width(line, 10.0)? + 4.0);
                }
                width = width.min((radius * 1.15).max(28.0));
                let height = lines.len() as f64 * 14.0;
                let label_radius = label_radius(chart, radius, middle, width, height);
                labels.push(RadialLabel {
                    text,
                    middle,
                    sweep,
                    bounds: Rectangle::new(
                        center[0] + middle.cos() * label_radius - width / 2.0,
                        center[1] + middle.sin() * label_radius - height / 2.0,
                        width,
                        height,
                    ),
                    outside: false,
                });
            }
            angle = TAU.mul_add(fraction, angle);
        }
        let radius = radial_labels::arrange(
            &mut labels,
            center,
            layout.plot,
            radius,
            chart.kind == "doughnut",
        );
        angle = -PI / 2.0;
        for (index, &value) in series.values.iter().enumerate() {
            let fraction = value.max(0.0) / total;
            let next_angle = TAU.mul_add(fraction, angle);
            if fraction > 0.0 {
                self.path(
                    sector_points(center, radius, angle, next_angle),
                    true,
                    Some(chart.color(index)),
                    None,
                );
            }
            angle = next_angle;
        }
        if chart.kind == "doughnut" {
            let default_background = self.background.clone();
            let mut background = default_background.as_str();
            for color in [
                Some(chart.chart_area.fill.as_str()),
                chart.plot_area.as_ref().map(|area| area.fill.as_str()),
            ]
            .into_iter()
            .flatten()
            {
                if let Some(color) = super::valid_color(color) {
                    background = color;
                }
            }
            self.ellipse(center, [radius, radius], 72, background);
        }
        let leader_color = self.leader_color.clone();
        for label in &labels {
            if label.outside {
                let right = label.middle.cos() >= -1e-9;
                let edge = if right {
                    label.bounds.left - 2.0
                } else {
                    label.bounds.left + label.bounds.width + 2.0
                };
                let start = [
                    center[0] + label.middle.cos() * radius,
                    center[1] + label.middle.sin() * radius,
                ];
                let end = [edge, label.bounds.top + label.bounds.height / 2.0];
                // Route past the circle before changing vertical position, so
                // a displaced label never draws its leader through a sector.
                let bend = [
                    center[0] + if right { radius + 4.0 } else { -radius - 4.0 },
                    start[1],
                ];
                self.rule(
                    start,
                    bend,
                    0.5,
                    &leader_color,
                    crate::layout::RuleRole::Generic,
                );
                self.rule(
                    bend,
                    end,
                    0.5,
                    &leader_color,
                    crate::layout::RuleRole::Generic,
                );
            }
        }
        // Labels follow both sector fills and the composited doughnut centre.
        for label in labels {
            self.text(label.bounds, &label.text, 10.0, "center", 0.0)?;
        }
        Ok(())
    }
}

fn sector_points(center: [f64; 2], radius: f64, start: f64, end: f64) -> Vec<[f64; 2]> {
    let steps = ((end - start).abs() / (PI / 36.0)).ceil().max(2.0) as usize;
    let mut points = Vec::with_capacity(steps + 2);
    points.push(center);
    for index in 0..=steps {
        let angle = start + (end - start) * index as f64 / steps as f64;
        points.push([
            center[0] + angle.cos() * radius,
            center[1] + angle.sin() * radius,
        ]);
    }
    points
}

fn label(chart: &Chart, index: usize, value: f64, total: f64) -> String {
    let mut parts = Vec::new();
    if chart.data_labels.category
        && let Some(category) = chart.categories.get(index)
    {
        parts.push(category.clone());
    }
    if chart.data_labels.series
        && let Some(series) = chart.series.first()
    {
        parts.push(series.name.clone());
    }
    if chart.data_labels.value {
        parts.push(value.to_string());
    }
    if chart.data_labels.percentage && total != 0.0 {
        parts.push(format!("{:.0}%", (value / total * 100.0).round()));
    }
    parts.join("\n")
}

// Office sets a label just inside the circle, whatever the legend's position.
fn label_radius(chart: &Chart, radius: f64, angle: f64, width: f64, height: f64) -> f64 {
    let inside = maximum_label_radius(radius, angle, width, height);
    if chart.kind == "pie" && inside > 0.0 {
        inside
    } else {
        radius * 0.78
    }
}

fn maximum_label_radius(radius: f64, angle: f64, width: f64, height: f64) -> f64 {
    let inner_radius = (radius - 3.0).max(0.0);
    let half_width = width.max(0.0) / 2.0;
    let half_height = height.max(0.0) / 2.0;
    let mut maximum = inner_radius;
    for horizontal in [-half_width, half_width] {
        for vertical in [-half_height, half_height] {
            let corner_length_squared = horizontal * horizontal + vertical * vertical;
            if corner_length_squared > inner_radius * inner_radius {
                return 0.0;
            }
            let projection = angle.cos() * horizontal + angle.sin() * vertical;
            let discriminant =
                projection * projection + inner_radius * inner_radius - corner_length_squared;
            maximum = maximum.min(-projection + discriminant.max(0.0).sqrt());
        }
    }
    maximum.max(0.0)
}
