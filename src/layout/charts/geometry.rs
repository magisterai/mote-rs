use super::{
    Chart, LayoutError,
    scale::{Scale, axis_label},
    text::ChartMeasurement,
};

#[derive(Clone, Copy, Default)]
pub(super) struct Rectangle {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}
impl Rectangle {
    pub fn new(left: f64, top: f64, width: f64, height: f64) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }
    pub fn bounds(self) -> [f64; 4] {
        [self.left, self.top, self.width, self.height]
    }
    pub fn center(self) -> [f64; 2] {
        [self.left + self.width / 2.0, self.top + self.height / 2.0]
    }
    pub fn category_center(self, count: usize, index: usize) -> f64 {
        if count == 0 {
            self.left + self.width / 2.0
        } else {
            self.left + self.width * (index as f64 + 0.5) / count as f64
        }
    }
}
pub(super) struct ChartLayout {
    pub chart: Rectangle,
    pub plot: Rectangle,
    pub title: Rectangle,
    pub legend: Rectangle,
    pub category_label_width: f64,
    pub legend_entries: Vec<super::legend::LegendEntry>,
}
impl ChartMeasurement<'_, '_> {
    pub(super) fn measure_layout(
        &mut self,
        chart: &Chart,
        bounds: Rectangle,
    ) -> Result<ChartLayout, LayoutError> {
        let mut category_width: f64 = 0.0;
        for category in &chart.categories {
            category_width = category_width.max(self.text_width(category, chart.label_size)?);
        }
        let mut legend_width: f64 = 0.0;
        for label in super::legend::labels(chart) {
            legend_width = legend_width.max(self.text_width(label, chart.label_size)?);
        }
        let value_extra = if matches!(chart.kind.as_str(), "pie" | "doughnut") {
            0.0
        } else {
            let scale = Scale::for_chart(chart, 0.0, 0.0, false).validate()?;
            let percent = chart.kind.contains("percent_stacked");
            let unit = scale.label_unit();
            let mut width = self
                .text_width(&axis_label(scale.minimum, unit, percent), chart.label_size)?
                .max(self.text_width(&axis_label(scale.maximum, unit, percent), chart.label_size)?);
            // The ticks of a stated range or step may have more digits than its ends.
            if chart.value_minimum.is_some() || chart.value_step.is_some() {
                let intervals = ((scale.maximum - scale.minimum) / scale.step).floor() as usize;
                for index in 1..=intervals.min(100) {
                    let tick = scale.minimum + index as f64 * scale.step;
                    width = width
                        .max(self.text_width(&axis_label(tick, unit, percent), chart.label_size)?);
                }
            }
            // Office keeps less space for labels of one digit than for two.
            width - self.text_width("80", chart.label_size)?
        };
        let mut layout = resolve(bounds, chart, category_width, legend_width, value_extra);
        self.arrange_legend(chart, &mut layout)?;
        Ok(layout)
    }
}
fn resolve(
    bounds: Rectangle,
    chart: &Chart,
    category_width: f64,
    legend_width: f64,
    value_extra: f64,
) -> ChartLayout {
    let title_height = title_height(chart);
    let label_scale = chart.label_scale();
    let mut top = bounds.top
        + if title_height > 0.0 {
            TITLE_TOP + title_distance(28.0, CHART_TITLE_RESERVE, chart.title_size()) + TITLE_GAP
        } else {
            UNTITLED_TOP
        };
    let position = super::legend::position(chart);
    let mut legend_reserve = 0.0;
    let mut legend = Rectangle::default();
    if chart.legend.enabled {
        match position {
            "bottom" => {
                legend_reserve = 24.0 * label_scale + chart.small_label_shift(13.0);
                legend = Rectangle::new(
                    bounds.left,
                    bounds.top + bounds.height - 24.0 * label_scale - chart.small_label_shift(5.5),
                    bounds.width,
                    18.0 * label_scale,
                );
            }
            "top" => {
                legend = Rectangle::new(
                    bounds.left,
                    top - 4.2 + chart.small_label_shift(4.1),
                    bounds.width,
                    18.0 * label_scale,
                );
                top += 24.0 * label_scale + chart.small_label_shift(12.5);
            }
            _ => {}
        }
    }
    let (mut left, mut right, piece_reserve) = if super::horizontal(chart) {
        let category_inset = if chart.legend.enabled && matches!(position, "left" | "right") {
            11.0 * label_scale
        } else {
            15.75 * label_scale
        } + chart.small_label_shift(6.4);
        (
            category_width.max(1.0)
                + category_inset
                + if chart.category_title.is_empty() {
                    0.0
                } else {
                    title_distance(
                        30.0,
                        AXIS_TITLE_RESERVE,
                        chart.category_title_style.axis_title_size(),
                    )
                },
            16.0 * label_scale.max(1.0) + value_extra.max(0.0) / 2.0,
            24.0 * label_scale
                + if chart.value_title.is_empty() {
                    0.0
                } else {
                    title_distance(
                        31.0,
                        BOTTOM_TITLE_RESERVE,
                        chart.value_title_style.axis_title_size(),
                    )
                },
        )
    } else if matches!(chart.kind.as_str(), "pie" | "doughnut") {
        (0.0, 0.0, 10.0)
    } else {
        (
            26.5 * label_scale
                + chart.small_label_shift(5.5)
                + value_extra
                + if chart.value_title.is_empty() {
                    0.0
                } else {
                    title_distance(
                        29.5,
                        AXIS_TITLE_RESERVE,
                        chart.value_title_style.axis_title_size(),
                    )
                },
            11.0,
            25.5 * label_scale
                + if chart.category_title.is_empty() {
                    0.0
                } else {
                    title_distance(
                        31.0,
                        BOTTOM_TITLE_RESERVE,
                        chart.category_title_style.axis_title_size(),
                    )
                },
        )
    };
    // A side legend is centred beneath the space of the chart title.
    let side_legend_top = if title_height > 0.0 {
        title_distance(31.0, 0.94, chart.title_size())
    } else {
        31.0
    };
    if chart.legend.enabled && matches!(position, "left" | "right") {
        let reserve = (legend_width
            + super::legend::key_width(chart)
            + 17.0 * label_scale
            + chart.small_label_shift(4.5))
        .max(56.0)
        .min(bounds.width * 0.28);
        if position == "left" {
            legend = Rectangle::new(
                bounds.left + 9.0,
                bounds.top + side_legend_top,
                reserve - 4.0,
                bounds.height - side_legend_top,
            );
            left += reserve + 2.0;
        } else {
            legend = Rectangle::new(
                bounds.left + bounds.width - reserve + 2.0,
                bounds.top + side_legend_top,
                reserve - 4.0,
                bounds.height - side_legend_top,
            );
            right += reserve;
        }
    }
    // Office raises the plot of a chart with small labels by more at its top
    // than at its foot.
    let cartesian = !matches!(chart.kind.as_str(), "pie" | "doughnut");
    // A chart title fixes the top of the plot whatever the size of the labels.
    if cartesian && title_height == 0.0 {
        top -= chart.small_label_shift(15.0);
    }
    if super::horizontal(chart) {
        top -= chart.small_label_shift(14.0);
    }
    let bottom = bounds.top + bounds.height
        - legend_reserve
        - piece_reserve
        - if cartesian {
            chart.small_label_shift(6.0)
        } else {
            0.0
        };
    ChartLayout {
        chart: bounds,
        plot: Rectangle::new(
            bounds.left + left,
            top,
            (bounds.width - left - right).max(1.0),
            (bottom - top).max(1.0),
        ),
        title: Rectangle::new(
            bounds.left,
            bounds.top + title_distance(TITLE_TOP + 14.0, CHART_TITLE_MIDDLE, chart.title_size())
                - title_height / 2.0,
            bounds.width,
            title_height,
        ),
        legend,
        category_label_width: category_width,
        legend_entries: Vec::new(),
    }
}

use crate::charts::{MEASURED_TITLE_SIZE as TITLE_SIZE, title_distance};
/// The points of space that a chart title and an axis title give up for each
/// point of size below 18.
const CHART_TITLE_RESERVE: f64 = 1.31;
const AXIS_TITLE_RESERVE: f64 = 1.05;
/// The same for an axis title beneath the plot.
const BOTTOM_TITLE_RESERVE: f64 = 1.21;
/// The points by which the middle of a chart title rises for each point of size
/// below 18.
const CHART_TITLE_MIDDLE: f64 = 0.665;
const TITLE_TOP: f64 = 5.1;
const TITLE_GAP: f64 = 8.8;
const UNTITLED_TOP: f64 = 12.0;

fn title_height(chart: &Chart) -> f64 {
    if chart.title.trim().is_empty() {
        0.0
    } else {
        28.0 * chart.title_size() / TITLE_SIZE
    }
}
