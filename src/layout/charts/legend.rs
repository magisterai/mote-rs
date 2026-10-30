use super::{
    Chart, LayoutError, Planner,
    geometry::{ChartLayout, Rectangle},
};
pub(super) fn position(chart: &Chart) -> &'static str {
    match chart.legend.position.as_str() {
        "b" | "bottom" | "" => "bottom",
        "t" | "top" => "top",
        "l" | "left" => "left",
        _ => "right",
    }
}
pub(super) fn key_width(chart: &Chart) -> f64 {
    if chart.kind == "line" { 18.0 } else { 7.0 }
}
fn color_index(chart: &Chart, index: usize) -> usize {
    if chart.kind == "bar" {
        chart.series.len() - 1 - index
    } else {
        index
    }
}
pub(super) fn labels(chart: &Chart) -> Vec<&str> {
    if matches!(chart.kind.as_str(), "pie" | "doughnut") {
        chart.categories.iter().map(String::as_str).collect()
    } else {
        (0..chart.series.len())
            .map(|index| chart.series[color_index(chart, index)].name.as_str())
            .collect()
    }
}
impl Planner<'_, '_> {
    pub(super) fn legend(
        &mut self,
        chart: &Chart,
        layout: &ChartLayout,
    ) -> Result<(), LayoutError> {
        let bounds = layout.legend;
        if !chart.legend.enabled || bounds.width <= 0.0 || bounds.height <= 0.0 {
            return Ok(());
        }
        let labels = labels(chart);
        if labels.is_empty() {
            return Ok(());
        }
        let widths = labels
            .iter()
            .map(|label| self.text_width(label, 10.0).map(|width| width + 1.0))
            .collect::<Result<Vec<_>, _>>()?;
        let key = key_width(chart);
        let total: f64 = widths.iter().map(|width| key + 4.0 + width).sum::<f64>()
            + 12.0 * (labels.len() - 1) as f64;
        if matches!(position(chart), "left" | "right") {
            let mut top =
                bounds.top + ((bounds.height - 18.0 * labels.len() as f64) / 2.0).max(0.0);
            for (index, label) in labels.iter().enumerate() {
                self.fill(
                    Rectangle::new(bounds.left + 2.0, top + 5.0, 6.0, 6.0),
                    chart.color(color_index(chart, index)),
                );
                self.text(
                    Rectangle::new(
                        bounds.left + key + 6.0,
                        top,
                        (bounds.width - key - 8.0).max(1.0),
                        16.0,
                    ),
                    label,
                    10.0,
                    "left",
                    0.0,
                )?;
                top += 18.0;
            }
        } else {
            let mut left = bounds.left + (bounds.width - total) / 2.0;
            for (index, label) in labels.iter().enumerate() {
                self.fill(
                    Rectangle::new(left, bounds.top + 5.0, 6.0, 6.0),
                    chart.color(color_index(chart, index)),
                );
                left += key + 4.0;
                self.text(
                    Rectangle::new(left, bounds.top, widths[index] + 2.0, 16.0),
                    label,
                    10.0,
                    "left",
                    0.0,
                )?;
                left += widths[index] + 2.0;
                if index != labels.len() - 1 {
                    left += 12.0;
                }
            }
        }
        Ok(())
    }
}
