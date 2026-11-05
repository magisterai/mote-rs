use super::{
    Chart, ChartPlanner, LayoutError, PageOperation,
    geometry::{ChartLayout, Rectangle},
    scale::{Scale, axis_label},
};
use crate::{
    charts::{RULE_COLOR, RULE_WIDTH},
    layout::RuleRole,
};

impl ChartPlanner<'_, '_> {
    pub(super) fn axes(
        &mut self,
        chart: &Chart,
        layout: &ChartLayout,
        scale: Scale,
        horizontal: bool,
    ) -> Result<Vec<PageOperation>, LayoutError> {
        let plot = layout.plot;
        let grid_color = super::valid_color(&chart.gridlines.color).unwrap_or(RULE_COLOR);
        let grid_width = if chart.gridlines.width > 0 {
            chart.gridlines.width as f64 / 12700.0
        } else {
            RULE_WIDTH as f64 / 12700.0
        };
        let gridline = |planner: &mut Self, value: f64| {
            let ratio = scale.ratio(value);
            let (start, end) = if horizontal {
                let left = plot.left + plot.width * ratio;
                ([left, plot.top], [left, plot.top + plot.height])
            } else {
                let top = plot.top + plot.height * (1.0 - ratio);
                ([plot.left, top], [plot.left + plot.width, top])
            };
            planner.rule(start, end, grid_width, grid_color, RuleRole::ChartGridline);
        };
        if chart.gridlines.minor {
            let step = scale.step / 5.0;
            let mut value = scale.minimum + step;
            for _ in 0..1000 {
                if value >= scale.maximum - scale.step / 10.0 {
                    break;
                }
                let multiple = value / scale.step;
                if (multiple - multiple.round()).abs() >= 0.0001 {
                    gridline(self, value);
                }
                value += step;
            }
        }
        let percent = chart.kind.contains("percent_stacked");
        let values: Vec<_> = (0..1000)
            .map(|index| scale.minimum + index as f64 * scale.step)
            .take_while(|value| *value <= scale.maximum + scale.step / 2.0)
            .map(|value| {
                if value.abs() < scale.step / 1000000.0 {
                    0.0
                } else {
                    value
                }
            })
            .collect();
        // Office labels every major tick of a vertical value axis, having chosen
        // the step from the plot height; only wide horizontal labels may collide.
        let visible = if horizontal {
            let mut intervals = Vec::new();
            for value in &values {
                let width = self
                    .measurement
                    .text_width(&axis_label(*value, scale.step, percent), 10.0)?
                    .min(40.0);
                intervals.push((scale.ratio(*value) * plot.width, width));
            }
            visible_labels(&intervals, values.iter().position(|value| *value == 0.0))
        } else {
            vec![true; values.len()]
        };
        for (index, &value) in values.iter().enumerate() {
            if chart.gridlines.major {
                gridline(self, value);
            }
            let ratio = scale.ratio(value);
            let (bounds, alignment) = if horizontal {
                (
                    Rectangle::new(
                        plot.left + plot.width * ratio - 20.0,
                        plot.top + plot.height + 4.0,
                        40.0,
                        15.0,
                    ),
                    "center",
                )
            } else {
                (
                    Rectangle::new(
                        plot.left - 37.0,
                        plot.top + plot.height * (1.0 - ratio) - 8.0,
                        28.0,
                        16.0,
                    ),
                    "right",
                )
            };
            if visible[index] {
                self.axis_label(bounds, &axis_label(value, scale.step, percent), alignment)?;
            }
        }
        let category_axis = if horizontal {
            let zero = plot.left + plot.width * scale.ratio(0.0);
            [[zero, plot.top], [zero, plot.top + plot.height]]
        } else {
            let zero = plot.top + plot.height * (1.0 - scale.ratio(0.0));
            [[plot.left, zero], [plot.left + plot.width, zero]]
        };
        self.rule(
            category_axis[0],
            category_axis[1],
            RULE_WIDTH as f64 / 12700.0,
            RULE_COLOR,
            RuleRole::ChartAxis,
        );
        let count = chart.categories.len();
        let category_top = plot.top
            + if chart.category_label_position() == "low" {
                plot.height
            } else {
                plot.height * (1.0 - scale.ratio(0.0))
            };
        for (index, category) in chart.categories.iter().enumerate() {
            let (bounds, alignment) = if horizontal {
                let slot = plot.height / count.max(1) as f64;
                (
                    Rectangle::new(
                        plot.left - layout.category_label_width - 9.0,
                        plot.top + (count - 1 - index) as f64 * slot + slot / 2.0 - 8.0,
                        layout.category_label_width,
                        16.0,
                    ),
                    "right",
                )
            } else {
                let slot = plot.width / count.max(1) as f64;
                (
                    Rectangle::new(
                        plot.category_center(count, index) - slot / 2.0,
                        category_top + 5.0,
                        slot,
                        16.0,
                    ),
                    "center",
                )
            };
            self.axis_label(bounds, category, alignment)?;
        }
        if horizontal {
            self.text(
                Rectangle::new(plot.left, plot.top + plot.height + 21.0, plot.width, 24.0),
                &chart.value_title,
                18.0,
                "center",
                0.0,
            )?;
            let offset = if chart.legend.enabled
                && matches!(super::legend::position(chart), "left" | "right")
            {
                21.5
            } else {
                29.0
            };
            let center_left = plot.left - layout.category_label_width - offset;
            self.text(
                Rectangle::new(
                    center_left - plot.height / 2.0,
                    plot.top + plot.height / 2.0 - 14.0,
                    plot.height,
                    28.0,
                ),
                &chart.category_title,
                18.0,
                "center",
                -90.0,
            )?;
        } else {
            self.text(
                Rectangle::new(plot.left, plot.top + plot.height + 20.5, plot.width, 25.0),
                &chart.category_title,
                18.0,
                "center",
                0.0,
            )?;
            self.text(
                Rectangle::new(
                    layout.chart.left
                        + 23.25
                        + if chart.legend.enabled && super::legend::position(chart) == "left" {
                            layout.legend.width + 6.0
                        } else {
                            0.0
                        }
                        - plot.height / 2.0,
                    plot.top + plot.height / 2.0 - 14.0,
                    plot.height,
                    28.0,
                ),
                &chart.value_title,
                18.0,
                "center",
                -90.0,
            )?;
        }
        let (background, foreground) = std::mem::take(&mut self.page.operations)
            .into_iter()
            .partition(|operation| {
                matches!(
                    operation,
                    PageOperation::Rule(_)
                        | PageOperation::Rectangle(_)
                        | PageOperation::OutlineRectangle { .. }
                        | PageOperation::Path(_)
                )
            });
        self.page.operations = background;
        Ok(foreground)
    }
}

// Prefer zero and the endpoints, then retain every label with enough clear space.
fn visible_labels(intervals: &[(f64, f64)], zero: Option<usize>) -> Vec<bool> {
    let mut visible = vec![false; intervals.len()];
    if intervals.is_empty() {
        return visible;
    }
    let order = zero
        .into_iter()
        .chain([0, intervals.len() - 1])
        .chain(0..intervals.len());
    for index in order {
        let (center, extent) = intervals[index];
        if visible.iter().enumerate().any(|(other, selected)| {
            *selected
                && (center - intervals[other].0).abs() < (extent + intervals[other].1) / 2.0 + 4.0
        }) {
            continue;
        }
        visible[index] = true;
    }
    visible
}
