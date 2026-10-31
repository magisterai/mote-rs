use super::{
    Chart, LayoutError, Planner,
    geometry::{ChartLayout, Rectangle},
};
pub(super) struct LegendEntry {
    pub index: usize,
    pub key: [f64; 2],
    pub text: Rectangle,
}
#[derive(Default)]
struct LegendRow {
    indices: Vec<usize>,
    width: f64,
    height: f64,
}
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
    pub(super) fn arrange_legend(
        &mut self,
        chart: &Chart,
        layout: &mut ChartLayout,
    ) -> Result<(), LayoutError> {
        let labels = labels(chart);
        let mut bounds = layout.legend;
        if !chart.legend.enabled || labels.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0
        {
            return Ok(());
        }
        let position = position(chart);
        let side = matches!(position, "left" | "right");
        let key_width = key_width(chart);
        let mut widths = labels
            .iter()
            .map(|label| self.text_width(label, 10.0).map(|width| width + 3.0))
            .collect::<Result<Vec<_>, _>>()?;
        let total_width = 12.0 * (labels.len() - 1) as f64
            + widths
                .iter()
                .map(|width| key_width + 4.0 + width)
                .sum::<f64>();
        if !side && total_width <= bounds.width {
            // Trailing text padding does not participate in centering.
            let mut left =
                bounds.left + (bounds.width - (total_width - 2.0 * labels.len() as f64)) / 2.0;
            for (index, width) in widths.iter().enumerate() {
                layout.legend_entries.push(LegendEntry {
                    index,
                    key: [left, bounds.top],
                    text: Rectangle::new(left + key_width + 4.0, bounds.top, *width, 16.0),
                });
                left += key_width + 4.0 + width + 12.0;
            }
            return Ok(());
        }
        let mut rows: Vec<LegendRow> = Vec::new();
        for (index, label) in labels.iter().enumerate() {
            widths[index] = if side {
                (bounds.width - key_width - 8.0).max(1.0)
            } else {
                widths[index].min((bounds.width - key_width - 4.0).max(1.0))
            };
            let entry_width = key_width + 4.0 + widths[index];
            let text_height =
                super::text::text_height(&self.text_plan(label, 10.0, "left", widths[index])?);
            if side
                || rows
                    .last()
                    .is_none_or(|row| row.width + 12.0 + entry_width > bounds.width)
            {
                rows.push(LegendRow::default());
            }
            let row = rows.last_mut().expect("legend row was allocated");
            if !row.indices.is_empty() {
                row.width += 12.0;
            }
            row.indices.push(index);
            row.width += entry_width;
            row.height = row.height.max(18.0_f64.max(text_height + 2.0));
        }
        let total_height: f64 = rows.iter().map(|row| row.height).sum();
        if !side {
            let extra = (total_height - 18.0).max(0.0).min(
                (layout.chart.height * 0.4 - 18.0)
                    .max(0.0)
                    .min((layout.plot.height - 1.0).max(0.0)),
            );
            layout.legend.height += extra;
            layout.plot.height -= extra;
            if position == "top" {
                layout.plot.top += extra;
            } else {
                layout.legend.top -= extra;
            }
            bounds = layout.legend;
        }
        let row_scale = (bounds.height / total_height).min(1.0);
        let mut top = bounds.top + ((bounds.height - total_height * row_scale) / 2.0).max(0.0);
        for row in rows {
            let row_height = row.height * row_scale;
            let mut left = if side {
                bounds.left + 2.0
            } else {
                bounds.left + (bounds.width - row.width) / 2.0
            };
            for index in row.indices {
                layout.legend_entries.push(LegendEntry {
                    index,
                    key: [left, top + (row_height - 18.0) / 2.0],
                    text: Rectangle::new(
                        left + key_width + 4.0,
                        top,
                        widths[index],
                        (row_height - 2.0).max(row_height / 2.0),
                    ),
                });
                left += key_width + 4.0 + widths[index] + 12.0;
            }
            top += row_height;
        }
        Ok(())
    }
    fn legend_key(&mut self, chart: &Chart, left: f64, top: f64, color: &str) {
        if chart.kind == "line" {
            self.rule(
                [left, top + 8.0],
                [left + 18.0, top + 8.0],
                2.0,
                color,
                crate::layout::RuleRole::Generic,
            );
            self.ellipse([left + 9.0, top + 8.0], [5.0, 5.0], 16, color);
        } else {
            self.fill(Rectangle::new(left, top + 5.0, 6.0, 6.0), color);
        }
    }
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
        for entry in &layout.legend_entries {
            self.legend_key(
                chart,
                entry.key[0],
                entry.key[1],
                chart.color(color_index(chart, entry.index)),
            );
            self.text(entry.text, labels[entry.index], 10.0, "left", 0.0)?;
        }
        Ok(())
    }
}
