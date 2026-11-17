//! Native chart operations share text measurement and both page painters.
mod axes;
mod bars;
mod clipping;
mod curves;
mod geometry;
mod legend;
mod line_labels;
mod paths;
mod pie;
mod radial_labels;
mod scale;
mod text;
use super::{
    HighlightRectangle, LayoutError, LayoutStyleResolver, PageOperation, Rule, RuleRole, TextPage,
};
use crate::{
    charts::{Chart, Series},
    measurement::TextMeasurer,
    model::{Element, ElementKind, Properties},
};
use geometry::Rectangle;
use text::ChartMeasurement;

pub(super) struct ChartBlock {
    content: PlannedChart,
    name: String,
    pub height: f64,
    pub before: f64,
    pub after: f64,
    pub anchors: Vec<String>,
}
impl ChartBlock {
    pub fn prepare(
        element: &Element,
        column_width: f64,
        styles: &LayoutStyleResolver<'_>,
        measurer: &mut TextMeasurer<'_>,
    ) -> Result<Self, LayoutError> {
        let chart = Chart::from_element(element)
            .map_err(|error| LayoutError::Unsupported(error.to_string()))?;
        let mut width = chart.width as f64 / 12700.0;
        let mut height = chart.height as f64 / 12700.0;
        if width > column_width && width > 0.0 {
            height *= column_width / width;
            width = column_width;
        }
        if width <= 0.0 || height <= 0.0 {
            return Ok(Self {
                content: PlannedChart::default(),
                name: String::new(),
                height: 0.0,
                before: 0.0,
                after: 0.0,
                anchors: Vec::new(),
            });
        }
        let content = plan_chart(
            &chart,
            [0.0, 0.0, round_coordinate(width), round_coordinate(height)],
            "Aptos",
            crate::charts::WORD_TEXT_COLOR,
            "FFFFFF",
            false,
            measurer,
        )?;
        let paragraph = styles.paragraph(&Element {
            source_name: None,
            source_location: None,
            kind: ElementKind::Paragraph,
            attributes: Properties::new(),
            children: Vec::new(),
        });
        Ok(Self {
            content,
            name: crate::properties::text(&element.attributes, "name"),
            height,
            before: paragraph.spacing_before,
            after: paragraph.spacing_after,
            anchors: Vec::new(),
        })
    }
    pub fn place(&self, page: &mut TextPage, left: f64, top: f64) {
        page.place_anchors(&self.anchors, [left, top]);
        page.append_content_at(
            self.content.page.clone(),
            [round_coordinate(left), round_coordinate(top)],
        );
    }
    /// The warning for labels reduced to fit, naming the chart by its name,
    /// where it has one, and by the page on which it is placed.
    pub fn reduced_labels_warning(&self, page_number: usize) -> Option<String> {
        let chart = if self.name.is_empty() {
            "a chart".to_owned()
        } else {
            format!("chart {:?}", self.name)
        };
        self.content
            .reduced_labels_warning(&format!("{chart} on page {page_number}"))
    }
}
pub(crate) fn plan_slide(
    document: &crate::model::Document,
    container: &Element,
    element: &Element,
    theme: &crate::presentation::Theme,
    measurer: &mut TextMeasurer<'_>,
) -> Result<PlannedChart, LayoutError> {
    let mut chart = Chart::from_element(element)
        .map_err(|error| LayoutError::Unsupported(error.to_string()))?;
    theme.apply_chart_palette(&mut chart);
    let bounds = crate::presentation::box_bounds(&container.attributes)
        .map_err(LayoutError::Unsupported)?
        .map(|value| value as f64 / 12700.0);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return Ok(PlannedChart::default());
    }
    let font = presentation_font(document, theme);
    plan_chart(
        &chart,
        bounds,
        font,
        &theme.colors[0],
        &theme.colors[1],
        true,
        measurer,
    )
}
/// Automatic frames reserve a real plot after measured titles, axes and legends.
pub(crate) fn automatic_frame_fits(
    document: &crate::model::Document,
    element: &Element,
    theme: &crate::presentation::Theme,
    size: [i64; 2],
    measurer: &mut TextMeasurer<'_>,
) -> Result<bool, LayoutError> {
    let chart = Chart::from_element(element)
        .map_err(|error| LayoutError::Unsupported(error.to_string()))?;
    let mut measurement = ChartMeasurement::new(
        presentation_font(document, theme),
        &theme.colors[0],
        measurer,
    );
    let layout = measurement.measure_layout(
        &chart,
        Rectangle::new(0.0, 0.0, size[0] as f64 / 12700.0, size[1] as f64 / 12700.0),
    )?;
    let minimum = if matches!(chart.kind.as_str(), "pie" | "doughnut") {
        96.0
    } else {
        72.0
    };
    Ok(layout.plot.width >= minimum && layout.plot.height >= minimum)
}

// Automatic frame selection and native painting must measure the same typeface.
pub(crate) fn presentation_font<'theme>(
    document: &crate::model::Document,
    theme: &'theme crate::presentation::Theme,
) -> &'theme str {
    if document.fonts.contains_key("Aptos") {
        "Aptos"
    } else {
        &theme.body
    }
}

fn plan_chart(
    chart: &Chart,
    bounds: [f64; 4],
    font: &str,
    text_color: &str,
    background: &str,
    slide: bool,
    measurer: &mut TextMeasurer<'_>,
) -> Result<PlannedChart, LayoutError> {
    if !matches!(
        chart.kind.as_str(),
        "column"
            | "bar"
            | "stacked_column"
            | "stacked_bar"
            | "percent_stacked_column"
            | "percent_stacked_bar"
            | "line"
            | "area"
            | "stacked_area"
            | "percent_stacked_area"
            | "pie"
            | "doughnut"
    ) {
        return Err(LayoutError::Unsupported(format!(
            "native {} charts are not yet supported",
            chart.kind
        )));
    }
    let mut planner = ChartPlanner {
        measurement: ChartMeasurement::new(font, text_color, measurer),
        leader_color: text_color.into(),
        axis_label_color: crate::charts::axis_label_color(text_color),
        background: background.into(),
        slide,
        page: TextPage::default(),
        reduced_labels: Default::default(),
    };
    let layout = planner.measurement.measure_layout(
        chart,
        Rectangle::new(bounds[0], bounds[1], bounds[2], bounds[3]),
    )?;
    planner.area(
        layout.chart,
        &chart.chart_area.fill,
        &chart.chart_area.border,
    );
    if let Some(area) = &chart.plot_area {
        planner.area(layout.plot, &area.fill, &area.border);
    }
    match chart.kind.as_str() {
        "line" => planner.line_chart(chart, &layout)?,
        "pie" | "doughnut" => planner.pie_chart(chart, &layout)?,
        "area" | "stacked_area" | "percent_stacked_area" => planner.area_chart(chart, &layout)?,
        _ => planner.bars(chart, &layout)?,
    }
    planner.title(
        layout.title,
        &chart.title,
        &chart.title_style,
        chart.title_size(),
        0.0,
    )?;
    planner.legend(chart, &layout)?;
    let original_lines = std::mem::take(&mut planner.page.lines);
    for operation in &mut planner.page.operations {
        if let PageOperation::Text(index) = operation {
            planner.page.lines.push(original_lines[*index].clone());
            *index = planner.page.lines.len() - 1;
        }
    }
    Ok(PlannedChart {
        page: planner.page,
        reduced_labels: planner.reduced_labels,
    })
}
#[derive(Default)]
pub(crate) struct PlannedChart {
    pub page: TextPage,
    reduced_labels: std::collections::BTreeSet<text::LabelKind>,
}
impl PlannedChart {
    /// The warning for labels reduced to fit, naming the chart as described.
    pub fn reduced_labels_warning(&self, described: &str) -> Option<String> {
        text::reduced_labels_warning(&self.reduced_labels, described)
    }
}
struct ChartPlanner<'planner, 'assets> {
    measurement: ChartMeasurement<'planner, 'assets>,
    leader_color: String,
    axis_label_color: String,
    background: String,
    /// PowerPoint sets the text of a chart at other heights than Word does.
    slide: bool,
    page: TextPage,
    reduced_labels: std::collections::BTreeSet<text::LabelKind>,
}
impl ChartPlanner<'_, '_> {
    fn fill(&mut self, bounds: Rectangle, color: &str) {
        // Zero-valued bars retain their labels but have no painted area.
        if bounds.width == 0.0 || bounds.height == 0.0 {
            return;
        }
        self.page
            .operations
            .push(PageOperation::Rectangle(HighlightRectangle {
                x: bounds.left,
                top: bounds.top,
                width: bounds.width,
                height: bounds.height,
                color: valid_color(color).unwrap_or("2563EB").into(),
            }));
    }
    fn rule(&mut self, start: [f64; 2], end: [f64; 2], width: f64, color: &str, role: RuleRole) {
        self.page.operations.push(PageOperation::Rule(Rule {
            role,
            start,
            end,
            width,
            color: color.into(),
        }));
    }
    fn area(&mut self, bounds: Rectangle, fill: &str, border: &crate::charts::appearance::Border) {
        if let Some(fill) = valid_color(fill) {
            self.fill(bounds, fill);
        }
        if border.enabled {
            self.page.operations.push(PageOperation::OutlineRectangle {
                bounds: bounds.bounds(),
                width: if border.width > 0 {
                    border.width as f64 / 12700.0
                } else {
                    1.0
                },
                color: valid_color(&border.color).unwrap_or("D1D5DB").into(),
            });
        }
    }
}
fn valid_color(color: &str) -> Option<&str> {
    (color.len() == 6 && color.bytes().all(|byte| byte.is_ascii_hexdigit())).then_some(color)
}
fn horizontal(chart: &Chart) -> bool {
    matches!(
        chart.kind.as_str(),
        "bar" | "stacked_bar" | "percent_stacked_bar"
    )
}
fn series_value(series: &Series, index: usize) -> f64 {
    series.values.get(index).copied().unwrap_or_default()
}
fn category_total(chart: &Chart, index: usize) -> f64 {
    chart
        .series
        .iter()
        .map(|series| series_value(series, index).abs())
        .sum()
}

// Normalize by total magnitude while preserving signs. Scaling handles a finite
// category whose summed magnitudes exceed the floating-point range.
fn percentage(chart: &Chart, index: usize, value: f64) -> f64 {
    let total = category_total(chart, index);
    if total == 0.0 {
        return 0.0;
    }
    if total.is_finite() {
        return value / total * 100.0;
    }
    let largest = chart
        .series
        .iter()
        .map(|series| series_value(series, index).abs())
        .fold(0.0, f64::max);
    let scaled_total: f64 = chart
        .series
        .iter()
        .map(|series| series_value(series, index).abs() / largest)
        .sum();
    (value / largest) / scaled_total * 100.0
}

fn round_coordinate(points: f64) -> f64 {
    (points * 12700.0).round() / 12700.0
}
