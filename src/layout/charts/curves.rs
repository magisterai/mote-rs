//! Line and area marks use the same axes, category centers and signed scales.
use super::{Chart, LayoutError, Planner, geometry::ChartLayout, scale::Scale};

impl Planner<'_, '_> {
    pub(super) fn line_chart(
        &mut self,
        chart: &Chart,
        layout: &ChartLayout,
    ) -> Result<(), LayoutError> {
        let scale = Scale::for_chart(chart, layout.plot.height, false).validate()?;
        let foreground = self.axes(chart, layout, scale, false)?;
        let count = chart.categories.len();
        let mut series_points = Vec::new();
        let mut labels = Vec::new();
        if count > 0 {
            for (series_index, series) in chart.series.iter().enumerate() {
                let points: Vec<_> = (0..count)
                    .map(|index| {
                        [
                            layout.plot.category_center(count, index),
                            layout.plot.top
                                + layout.plot.height
                                    * (1.0 - scale.ratio(super::series_value(series, index))),
                        ]
                    })
                    .collect();
                let color = chart.color(series_index);
                self.path(points.clone(), false, None, Some((color, 2.0)));
                for (index, point) in points.iter().enumerate() {
                    self.ellipse([point[0], point[1]], [5.0, 5.0], 24, color);
                    if chart.data_labels.enabled {
                        labels.push(super::line_labels::LineLabel {
                            point: *point,
                            text: super::bars::label(chart, series_index, index),
                        });
                    }
                }
                series_points.push(points);
            }
        }
        self.place_line_labels(&labels, &series_points, layout.plot)?;
        self.page.operations.extend(foreground);
        Ok(())
    }
    pub(super) fn area_chart(
        &mut self,
        chart: &Chart,
        layout: &ChartLayout,
    ) -> Result<(), LayoutError> {
        let scale = Scale::for_chart(chart, layout.plot.height, false).validate()?;
        let foreground = self.axes(chart, layout, scale, false)?;
        let count = chart.categories.len();
        if count > 0 {
            let stacked = chart.kind.contains("stacked");
            let percent = chart.kind == "percent_stacked_area";
            let mut positive = vec![0.0; count];
            let mut negative = vec![0.0; count];
            for (series_index, series) in chart.series.iter().enumerate() {
                let mut upper = Vec::with_capacity(count);
                let mut lower = Vec::with_capacity(count);
                for index in 0..count {
                    let mut value = super::series_value(series, index);
                    if percent {
                        let total = super::category_total(chart, index);
                        if total != 0.0 {
                            value = super::percentage(chart, index, value);
                        }
                    }
                    let mut base = 0.0;
                    if stacked {
                        if value < 0.0 {
                            base = negative[index];
                            negative[index] += value;
                        } else {
                            base = positive[index];
                            positive[index] += value;
                        }
                    }
                    let left = layout.plot.category_center(count, index);
                    upper.push([
                        left,
                        layout.plot.top + layout.plot.height * (1.0 - scale.ratio(base + value)),
                    ]);
                    lower.push([
                        left,
                        layout.plot.top + layout.plot.height * (1.0 - scale.ratio(base)),
                    ]);
                }
                upper.extend(lower.into_iter().rev());
                self.path(upper, true, Some(chart.color(series_index)), None);
            }
        }
        self.page.operations.extend(foreground);
        Ok(())
    }
}
