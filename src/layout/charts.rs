//! Native chart operations share text measurement and both page painters.
mod axes;
mod bars;
mod curves;
mod geometry;
mod legend;
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

pub(super) struct ChartBlock {
    content: TextPage,
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
        ) {
            return Err(LayoutError::Unsupported(format!(
                "native {} charts are not yet supported",
                chart.kind
            )));
        }
        let mut width = chart.width as f64 / 12700.0;
        let mut height = chart.height as f64 / 12700.0;
        if width > column_width && width > 0.0 {
            height *= column_width / width;
            width = column_width;
        }
        if width <= 0.0 || height <= 0.0 {
            return Ok(Self {
                content: TextPage::default(),
                height: 0.0,
                before: 0.0,
                after: 0.0,
                anchors: Vec::new(),
            });
        }
        if matches!(
            chart.kind.as_str(),
            "line" | "area" | "stacked_area" | "percent_stacked_area"
        ) && chart.categories.len() == 1
            && !chart.series.is_empty()
        {
            return Err(LayoutError::Unsupported(
                "native line and area charts with series require at least two categories".into(),
            ));
        }
        let mut planner = Planner {
            measurer,
            page: TextPage::default(),
        };
        let layout = planner.geometry(
            &chart,
            Rectangle::new(0.0, 0.0, round_coordinate(width), round_coordinate(height)),
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
            "line" => planner.line_chart(&chart, &layout)?,
            "area" | "stacked_area" | "percent_stacked_area" => {
                planner.area_chart(&chart, &layout)?
            }
            _ => planner.bars(&chart, &layout)?,
        }
        planner.text(layout.title, &chart.title, 18.0, "center", 0.0)?;
        planner.legend(&chart, &layout)?;
        let original_lines = std::mem::take(&mut planner.page.lines);
        for operation in &mut planner.page.operations {
            if let PageOperation::Text(index) = operation {
                planner.page.lines.push(original_lines[*index].clone());
                *index = planner.page.lines.len() - 1;
            }
        }
        let paragraph = styles.paragraph(&Element {
            kind: ElementKind::Paragraph,
            attributes: Properties::new(),
            children: Vec::new(),
        });
        Ok(Self {
            content: planner.page,
            height,
            before: paragraph.spacing_before,
            after: paragraph.spacing_after,
            anchors: Vec::new(),
        })
    }
    pub fn place(&self, page: &mut TextPage, left: f64, top: f64) {
        page.place_anchors(&self.anchors, [left, top]);
        let left = round_coordinate(left);
        let top = round_coordinate(top);
        let offset = page.lines.len();
        page.lines
            .extend(self.content.lines.iter().cloned().map(|mut line| {
                line.top += top;
                line.line.translate(left, top);
                line
            }));
        for operation in &self.content.operations {
            let mut operation = operation.clone();
            match &mut operation {
                PageOperation::Text(index) => *index += offset,
                PageOperation::Rectangle(rectangle) => {
                    rectangle.x += left;
                    rectangle.top += top;
                }
                PageOperation::OutlineRectangle { bounds, .. }
                | PageOperation::PushClip(bounds) => {
                    bounds[0] += left;
                    bounds[1] += top;
                }
                PageOperation::Rule(rule) => {
                    rule.start[0] += left;
                    rule.start[1] += top;
                    rule.end[0] += left;
                    rule.end[1] += top;
                }
                PageOperation::Path(path) => {
                    for point in &mut path.points {
                        point[0] += left;
                        point[1] += top;
                    }
                }
                PageOperation::PushRotation { center, .. } => {
                    center[0] += left;
                    center[1] += top;
                }
                PageOperation::PopClip | PageOperation::PopRotation => {}
                PageOperation::Stroke { .. }
                | PageOperation::Image(_)
                | PageOperation::ListMarker(_) => {
                    unreachable!("chart planner only creates chart painting operations")
                }
            }
            page.operations.push(operation);
        }
    }
}
struct Planner<'planner, 'assets> {
    measurer: &'planner mut TextMeasurer<'assets>,
    page: TextPage,
}
impl Planner<'_, '_> {
    fn fill(&mut self, bounds: Rectangle, color: &str) {
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
        .map(|series| series_value(series, index).max(0.0))
        .sum()
}

fn round_coordinate(points: f64) -> f64 {
    (points * 12700.0).round() / 12700.0
}
