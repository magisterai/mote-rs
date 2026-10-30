use super::{
    Chart, LayoutError, PageOperation, Planner,
    geometry::{ChartLayout, Rectangle},
    scale::{Scale, axis_label},
};
use crate::layout::RuleRole;

impl Planner<'_, '_> {
    pub(super) fn axes(
        &mut self,
        chart: &Chart,
        layout: &ChartLayout,
        scale: Scale,
        horizontal: bool,
    ) -> Result<Vec<PageOperation>, LayoutError> {
        let plot = layout.plot;
        let axis_color = self.axis_color.clone();
        let grid_color = super::valid_color(&chart.gridlines.color).unwrap_or(&axis_color);
        let grid_width = if chart.gridlines.width > 0 {
            chart.gridlines.width as f64 / 12700.0
        } else {
            0.75
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
        let mut value = scale.minimum;
        for _ in 0..1000 {
            if value > scale.maximum + scale.step / 2.0 {
                break;
            }
            if value.abs() < scale.step / 1000000.0 {
                value = 0.0;
            }
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
            self.text(bounds, &axis_label(value, percent), 10.0, alignment, 0.0)?;
            value += scale.step;
        }
        if horizontal {
            let zero = plot.left + plot.width * scale.ratio(0.0);
            self.rule(
                [zero, plot.top],
                [zero, plot.top + plot.height],
                0.75,
                &axis_color,
                RuleRole::ChartAxis,
            );
            self.rule(
                [plot.left, plot.top + plot.height],
                [plot.left + plot.width, plot.top + plot.height],
                0.75,
                &axis_color,
                RuleRole::ChartAxis,
            );
        } else {
            let zero = plot.top + plot.height * (1.0 - scale.ratio(0.0));
            self.rule(
                [plot.left, plot.top],
                [plot.left, plot.top + plot.height],
                0.75,
                &axis_color,
                RuleRole::ChartAxis,
            );
            self.rule(
                [plot.left, zero],
                [plot.left + plot.width, zero],
                0.75,
                &axis_color,
                RuleRole::ChartAxis,
            );
        }
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
            self.text(bounds, category, 10.0, alignment, 0.0)?;
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
                    layout.chart.left + 23.25 - plot.height / 2.0,
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
