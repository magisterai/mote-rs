use super::{
    Chart, ChartPlanner, LayoutError, PageOperation,
    geometry::{ChartLayout, Rectangle},
    scale::{Scale, axis_label},
};
use crate::{
    charts::{RULE_COLOR, RULE_WIDTH, title_distance},
    layout::RuleRole,
};

/// The points by which the middle of an axis title nears the edge it is measured
/// from for each point of size below 18.
const AXIS_TITLE_MIDDLE: f64 = 0.55;
/// Office sets a turned axis title of 10 points 0.6 point below the middle of
/// the plot, and one of 18 points at the middle.
fn rotated_title_drop(size: f64) -> f64 {
    (0.075 * (crate::charts::MEASURED_TITLE_SIZE - size)).max(0.0)
}

impl ChartPlanner<'_, '_> {
    pub(super) fn axes(
        &mut self,
        chart: &Chart,
        layout: &ChartLayout,
        scale: Scale,
        horizontal: bool,
    ) -> Result<Vec<PageOperation>, LayoutError> {
        let plot = layout.plot;
        let label_scale = chart.label_scale();
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
        let label_unit = scale.label_unit();
        // The last tick of a stated range may fall short of the maximum.
        let values: Vec<_> = (0..1000)
            .map(|index| scale.minimum + index as f64 * scale.step)
            .take_while(|value| *value <= scale.maximum + scale.step / 1000.0)
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
                    .text_width(&axis_label(*value, label_unit, percent), chart.label_size)?
                    .min(40.0 * label_scale);
                intervals.push((scale.ratio(*value) * plot.width, width));
            }
            visible_labels(
                &intervals,
                values.iter().position(|value| *value == 0.0),
                4.0 * label_scale,
            )
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
                        plot.left + plot.width * ratio - 20.0 * label_scale,
                        plot.top + plot.height + 4.0 * label_scale,
                        40.0 * label_scale,
                        15.0 * label_scale,
                    ),
                    "center",
                )
            } else {
                (
                    Rectangle::new(
                        plot.left - 37.0 * label_scale - chart.small_label_shift(1.9),
                        plot.top + plot.height * (1.0 - ratio) - 8.0 * label_scale,
                        28.0 * label_scale,
                        16.0 * label_scale,
                    ),
                    "right",
                )
            };
            if visible[index] {
                self.axis_label(
                    super::text::LabelKind::ValueAxis,
                    bounds,
                    &axis_label(value, label_unit, percent),
                    chart.label_size,
                    alignment,
                )?;
            }
        }
        let category_axis = if horizontal {
            let zero = plot.left + plot.width * scale.ratio(scale.crossing());
            [[zero, plot.top], [zero, plot.top + plot.height]]
        } else {
            let zero = plot.top + plot.height * (1.0 - scale.ratio(scale.crossing()));
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
                plot.height * (1.0 - scale.ratio(scale.crossing()))
            };
        for (index, category) in chart.categories.iter().enumerate() {
            let (bounds, alignment) = if horizontal {
                let slot = plot.height / count.max(1) as f64;
                (
                    Rectangle::new(
                        plot.left
                            - layout.category_label_width
                            - 9.0 * label_scale
                            - chart.small_label_shift(2.1),
                        plot.top + (count - 1 - index) as f64 * slot + slot / 2.0
                            - 8.0 * label_scale,
                        layout.category_label_width,
                        16.0 * label_scale,
                    ),
                    "right",
                )
            } else {
                let slot = plot.width / count.max(1) as f64;
                (
                    Rectangle::new(
                        plot.category_center(count, index) - slot / 2.0,
                        category_top + 5.0 * label_scale,
                        slot,
                        16.0 * label_scale,
                    ),
                    "center",
                )
            };
            self.axis_label(
                super::text::LabelKind::Category,
                bounds,
                category,
                chart.label_size,
                alignment,
            )?;
        }
        // The frames and offsets of the axis titles were measured for titles of
        // 18 points. A frame follows the size of its title in proportion, and its
        // middle moves as Office moves the title.
        let category_style = &chart.category_title_style;
        let value_style = &chart.value_title_style;
        let (category_scale, value_scale) = (
            category_style.axis_title_scale(),
            value_style.axis_title_scale(),
        );
        if horizontal {
            self.title(
                Rectangle::new(
                    plot.left,
                    plot.top
                        + plot.height
                        + 21.0 * label_scale
                        + title_distance(12.0, AXIS_TITLE_MIDDLE, value_style.axis_title_size())
                        - 12.0 * value_scale,
                    plot.width,
                    24.0 * value_scale,
                ),
                &chart.value_title,
                value_style,
                value_style.axis_title_size(),
                0.0,
            )?;
            let offset =
                if chart.legend.enabled
                    && matches!(super::legend::position(chart), "left" | "right")
                {
                    7.5
                } else {
                    15.0
                } + title_distance(14.0, AXIS_TITLE_MIDDLE, category_style.axis_title_size())
                    + 9.0 * (label_scale - 1.0);
            let center_left = plot.left - layout.category_label_width - offset;
            self.title(
                Rectangle::new(
                    center_left - plot.height / 2.0,
                    plot.top + plot.height / 2.0 - 14.0 * category_scale
                        + rotated_title_drop(category_style.axis_title_size()),
                    plot.height,
                    28.0 * category_scale,
                ),
                &chart.category_title,
                category_style,
                category_style.axis_title_size(),
                -90.0,
            )?;
        } else {
            self.title(
                Rectangle::new(
                    plot.left,
                    plot.top
                        + plot.height
                        + 20.5 * label_scale
                        + title_distance(12.5, AXIS_TITLE_MIDDLE, category_style.axis_title_size())
                        - 12.5 * category_scale,
                    plot.width,
                    25.0 * category_scale,
                ),
                &chart.category_title,
                category_style,
                category_style.axis_title_size(),
                0.0,
            )?;
            self.title(
                Rectangle::new(
                    layout.chart.left
                        + title_distance(23.25, AXIS_TITLE_MIDDLE, value_style.axis_title_size())
                        + if chart.legend.enabled && super::legend::position(chart) == "left" {
                            layout.legend.width + 6.0
                        } else {
                            0.0
                        }
                        - plot.height / 2.0,
                    plot.top + plot.height / 2.0 - 14.0 * value_scale
                        + rotated_title_drop(value_style.axis_title_size()),
                    plot.height,
                    28.0 * value_scale,
                ),
                &chart.value_title,
                value_style,
                value_style.axis_title_size(),
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
fn visible_labels(intervals: &[(f64, f64)], zero: Option<usize>, clearance: f64) -> Vec<bool> {
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
                && (center - intervals[other].0).abs()
                    < (extent + intervals[other].1) / 2.0 + clearance
        }) {
            continue;
        }
        visible[index] = true;
    }
    visible
}
