use super::{
    Chart, LayoutError, Planner,
    geometry::{ChartLayout, Rectangle},
    scale::Scale,
};
impl Planner<'_, '_> {
    pub(super) fn bars(&mut self, chart: &Chart, layout: &ChartLayout) -> Result<(), LayoutError> {
        let horizontal = super::horizontal(chart);
        let scale = Scale::for_chart(chart, layout.plot.width.min(layout.plot.height), horizontal)
            .validate()?;
        let foreground = self.axes(chart, layout, scale, horizontal)?;
        if !chart.categories.is_empty() && !chart.series.is_empty() {
            self.bar_marks(chart, layout.plot, scale, horizontal)?;
        }
        self.page.operations.extend(foreground);
        Ok(())
    }
    fn bar_marks(
        &mut self,
        chart: &Chart,
        plot: Rectangle,
        scale: Scale,
        horizontal: bool,
    ) -> Result<(), LayoutError> {
        let stacked = chart.kind.contains("stacked");
        let percent = chart.kind.contains("percent_stacked");
        let count = chart.categories.len();
        let series_count = chart.series.len();
        let gap = chart.gap_width.unwrap_or(150).max(0) as f64;
        let overlap = chart.overlap.unwrap_or(if stacked { 100 } else { 0 }) as f64;
        let slot = if horizontal { plot.height } else { plot.width } / count as f64;
        let visible = if stacked { 1.0 } else { series_count as f64 };
        let mut thickness = slot / (visible + gap / 100.0).max(1.0);
        if !horizontal && !stacked && overlap != 0.0 && series_count > 1 {
            thickness = slot
                / (1.0 + (series_count as f64 - 1.0) * (1.0 - overlap / 100.0) + gap / 100.0)
                    .max(1.0);
        }
        for display_index in 0..count {
            let category_index = if horizontal {
                count - 1 - display_index
            } else {
                display_index
            };
            let group_size = thickness * visible;
            let group_start = if horizontal { plot.top } else { plot.left }
                + display_index as f64 * slot
                + (slot - group_size) / 2.0;
            let mut positive = 0.0;
            let mut negative = 0.0;
            let total = super::category_total(chart, category_index);
            for (series_index, series) in chart.series.iter().enumerate() {
                let mut value = super::series_value(series, category_index);
                if percent && total != 0.0 {
                    value = value / total * 100.0;
                }
                let mut start = 0.0;
                let mut display_series = series_index;
                if horizontal && !stacked && chart.kind == "bar" {
                    display_series = series_count - 1 - series_index;
                }
                let mut cross = group_start + display_series as f64 * thickness;
                if stacked {
                    if value < 0.0 {
                        start = negative;
                        negative += value;
                    } else {
                        start = positive;
                        positive += value;
                    }
                    cross = group_start;
                }
                let bar = if horizontal {
                    let first = plot.left + plot.width * scale.ratio(start);
                    let last = plot.left + plot.width * scale.ratio(start + value);
                    Rectangle::new(first.min(last), cross, (last - first).abs(), thickness)
                } else {
                    let first = plot.top + plot.height * (1.0 - scale.ratio(start));
                    let last = plot.top + plot.height * (1.0 - scale.ratio(start + value));
                    Rectangle::new(cross, first.min(last), thickness, (last - first).abs())
                };
                self.fill(bar, chart.color(series_index));
                if chart.data_labels.enabled {
                    let label = label(chart, series_index, category_index);
                    let (bounds, alignment) = if horizontal {
                        horizontal_label(bar, &chart.data_labels.position, value < 0.0)
                    } else {
                        vertical_label(bar, &chart.data_labels.position, value < 0.0)
                    };
                    self.text(bounds, &label, 10.0, alignment, 0.0)?;
                }
            }
        }
        Ok(())
    }
}
pub(super) fn label(chart: &Chart, series_index: usize, category_index: usize) -> String {
    let series = &chart.series[series_index];
    let value = super::series_value(series, category_index);
    let mut parts = Vec::new();
    if chart.data_labels.category
        && let Some(category) = chart.categories.get(category_index)
    {
        parts.push(category.clone());
    }
    if chart.data_labels.series {
        parts.push(series.name.clone());
    }
    if chart.data_labels.value {
        parts.push(value.to_string());
    }
    if chart.data_labels.percentage {
        let total = super::category_total(chart, category_index);
        if total != 0.0 {
            parts.push(format!("{:.0}%", (value / total * 100.0).round()));
        }
    }
    parts.join(" ")
}
fn vertical_label(bar: Rectangle, position: &str, negative: bool) -> (Rectangle, &'static str) {
    let width = (bar.width + 20.0).max(42.0);
    let top = match position {
        "ctr" => bar.top + bar.height / 2.0 - 8.0,
        "inEnd" if negative => bar.top + bar.height - 18.0,
        "inEnd" => bar.top + 2.0,
        "inBase" if negative => bar.top + 2.0,
        "inBase" => bar.top + bar.height - 18.0,
        _ if negative => bar.top + bar.height + 1.0,
        _ => bar.top - 17.0,
    };
    (
        Rectangle::new(bar.left + bar.width / 2.0 - width / 2.0, top, width, 16.0),
        "center",
    )
}
fn horizontal_label(bar: Rectangle, position: &str, negative: bool) -> (Rectangle, &'static str) {
    let (left, alignment) = match position {
        "ctr" => (bar.left + bar.width / 2.0 - 21.0, "center"),
        "inEnd" if negative => (bar.left + 3.0, "left"),
        "inEnd" => ((bar.left + 2.0).max(bar.left + bar.width - 45.0), "right"),
        "inBase" if negative => ((bar.left + 2.0).max(bar.left + bar.width - 45.0), "right"),
        "inBase" => (bar.left + 3.0, "left"),
        _ if negative => (bar.left - 46.0, "right"),
        _ => (bar.left + bar.width + 4.0, "left"),
    };
    (
        Rectangle::new(left, bar.top + bar.height / 2.0 - 8.0, 42.0, 16.0),
        alignment,
    )
}
