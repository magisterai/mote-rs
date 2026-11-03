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
            category_width = category_width.max(self.text_width(category, 10.0)?);
        }
        let mut legend_width: f64 = 0.0;
        for label in super::legend::labels(chart) {
            legend_width = legend_width.max(self.text_width(label, 10.0)?);
        }
        let value_extra = if matches!(chart.kind.as_str(), "pie" | "doughnut") {
            0.0
        } else {
            let scale = Scale::for_chart(chart, 0.0, false).validate()?;
            let percent = chart.kind.contains("percent_stacked");
            let width = self
                .text_width(&axis_label(scale.minimum, scale.step, percent), 10.0)?
                .max(self.text_width(&axis_label(scale.maximum, scale.step, percent), 10.0)?);
            (width - self.text_width("80", 10.0)?).max(0.0)
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
    let mut top = bounds.top
        + if title_height > 0.0 {
            TITLE_TOP + title_height + TITLE_GAP
        } else {
            UNTITLED_TOP
        };
    let position = super::legend::position(chart);
    let mut legend_reserve = 0.0;
    let mut legend = Rectangle::default();
    if chart.legend.enabled {
        match position {
            "bottom" => {
                legend_reserve = 24.0;
                legend = Rectangle::new(
                    bounds.left,
                    bounds.top + bounds.height - 24.0,
                    bounds.width,
                    18.0,
                );
            }
            "top" => {
                legend = Rectangle::new(bounds.left, top - 6.8, bounds.width, 18.0);
                top += 24.0;
            }
            _ => {}
        }
    }
    let (mut left, mut right, piece_reserve) = if super::horizontal(chart) {
        let category_inset = if chart.legend.enabled && matches!(position, "left" | "right") {
            11.0
        } else {
            15.75
        };
        (
            category_width.max(1.0)
                + category_inset
                + if chart.category_title.is_empty() {
                    0.0
                } else {
                    30.0
                },
            16.0 + value_extra.max(0.0) / 2.0,
            24.0 + if chart.value_title.is_empty() {
                0.0
            } else {
                31.0
            },
        )
    } else if matches!(chart.kind.as_str(), "pie" | "doughnut") {
        (0.0, 0.0, 10.0)
    } else {
        (
            26.5 + value_extra.max(0.0)
                + if chart.value_title.is_empty() {
                    0.0
                } else {
                    29.5
                },
            11.0,
            24.0 + if chart.category_title.is_empty() {
                0.0
            } else {
                31.0
            },
        )
    };
    if chart.legend.enabled && matches!(position, "left" | "right") {
        let reserve = (legend_width + super::legend::key_width(chart) + 13.0)
            .max(56.0)
            .min(bounds.width * 0.28);
        if position == "left" {
            legend = Rectangle::new(
                bounds.left + 9.0,
                bounds.top + 31.0,
                reserve - 4.0,
                bounds.height - 31.0,
            );
            left += reserve + 2.0;
        } else {
            legend = Rectangle::new(
                bounds.left + bounds.width - reserve + 2.0,
                bounds.top + 31.0,
                reserve - 4.0,
                bounds.height - 31.0,
            );
            right += reserve;
        }
    }
    let bottom = bounds.top + bounds.height - legend_reserve - piece_reserve;
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
            bounds.top + TITLE_TOP,
            bounds.width,
            title_height,
        ),
        legend,
        category_label_width: category_width,
        legend_entries: Vec::new(),
    }
}

const TITLE_SIZE: f64 = 18.0;
const TITLE_TOP: f64 = 4.0;
const TITLE_GAP: f64 = 8.8;
const UNTITLED_TOP: f64 = 12.0;

pub(super) fn title_size(chart: &Chart) -> f64 {
    chart.title_style.size.unwrap_or(TITLE_SIZE)
}

fn title_height(chart: &Chart) -> f64 {
    if chart.title.trim().is_empty() {
        0.0
    } else {
        28.0 * title_size(chart) / TITLE_SIZE
    }
}
