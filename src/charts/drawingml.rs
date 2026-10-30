//! Office chart XML, independent of the containing document or presentation.
mod formatting;
mod series;
use super::{Chart, Error};
use crate::xml::{HEADER, escape};
use formatting::{data_labels, label_text, shape_properties, title};

pub(crate) const CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
pub(crate) const GRAPHIC_DATA_URI: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";

pub(crate) struct Options {
    pub part_number: usize,
    pub label_baseline: i32,
}
impl Options {
    fn category_axis(&self) -> usize {
        100000 + self.part_number.max(1) * 10 + 1
    }
    fn value_axis(&self) -> usize {
        self.category_axis() + 1
    }
}
pub(crate) fn write(chart: &Chart, options: &Options) -> Result<String, Error> {
    let mut output = format!(
        r#"{HEADER}<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><c:date1904 val="0"/><c:lang val="en-US"/><c:roundedCorners val="0"/><c:chart>{}<c:plotArea><c:layout/>"#,
        title(&chart.title)?
    );
    match chart.kind.as_str() {
        "pie" | "doughnut" => output.push_str(&write_pie(chart)?),
        _ => output.push_str(&write_cartesian(chart, options)?),
    }
    if let Some(properties) = &chart.plot_area {
        output.push_str(&shape_properties(properties)?);
    }
    output.push_str("</c:plotArea>");
    if chart.legend.enabled {
        output.push_str(&format!(
            r#"<c:legend><c:legendPos val="{}"/><c:layout/><c:overlay val="0"/>{}</c:legend>"#,
            escape(&chart.legend.position)?,
            label_text(options.label_baseline)
        ));
    }
    output.push_str(r#"<c:plotVisOnly val="1"/><c:dispBlanksAs val="gap"/><c:showDLblsOverMax val="0"/></c:chart>"#);
    output.push_str(&shape_properties(&chart.chart_area)?);
    output.push_str(r#"<c:txPr><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr sz="1000"/></a:pPr><a:endParaRPr lang="en-US"/></a:p></c:txPr><c:externalData r:id="rId1"><c:autoUpdate val="0"/></c:externalData><c:printSettings><c:headerFooter/><c:pageMargins b="0.75" l="0.7" r="0.7" t="0.75" header="0.3" footer="0.3"/><c:pageSetup/></c:printSettings></c:chartSpace>"#);
    Ok(output)
}
fn write_cartesian(chart: &Chart, options: &Options) -> Result<String, Error> {
    let (tag, series_kind, grouping) = match chart.kind.as_str() {
        "line" => ("lineChart", series::Kind::Line, "standard"),
        "area" => ("areaChart", series::Kind::Area, "standard"),
        "stacked_area" => ("areaChart", series::Kind::Area, "stacked"),
        "percent_stacked_area" => ("areaChart", series::Kind::Area, "percentStacked"),
        "stacked_bar" | "stacked_column" => ("barChart", series::Kind::Bar, "stacked"),
        "percent_stacked_bar" | "percent_stacked_column" => {
            ("barChart", series::Kind::Bar, "percentStacked")
        }
        _ => ("barChart", series::Kind::Bar, "clustered"),
    };
    let mut output = format!("<c:{tag}>");
    if series_kind == series::Kind::Bar {
        let direction = if matches!(
            chart.kind.as_str(),
            "bar" | "stacked_bar" | "percent_stacked_bar"
        ) {
            "bar"
        } else {
            "col"
        };
        output.push_str(&format!(r#"<c:barDir val="{direction}"/>"#));
    }
    output.push_str(&format!(
        r#"<c:grouping val="{grouping}"/><c:varyColors val="0"/>"#
    ));
    for (index, series) in chart.series.iter().enumerate() {
        output.push_str(&series::write(index, series, chart, series_kind)?);
    }
    output.push_str(&data_labels(&chart.data_labels, &chart.kind)?);
    if series_kind == series::Kind::Bar {
        if let Some(gap) = chart.gap_width {
            output.push_str(&format!(r#"<c:gapWidth val="{gap}"/>"#));
        }
        if let Some(overlap) = chart
            .overlap
            .or_else(|| (grouping != "clustered").then_some(100))
        {
            output.push_str(&format!(r#"<c:overlap val="{overlap}"/>"#));
        }
    }
    if series_kind == series::Kind::Line {
        output.push_str(r#"<c:smooth val="0"/>"#);
    }
    let category_axis = options.category_axis();
    let value_axis = options.value_axis();
    output.push_str(&format!(
        r#"<c:axId val="{category_axis}"/><c:axId val="{value_axis}"/></c:{tag}>"#
    ));
    output.push_str(&formatting::axes(chart, options)?);
    Ok(output)
}
fn write_pie(chart: &Chart) -> Result<String, Error> {
    let tag = if chart.kind == "doughnut" {
        "doughnutChart"
    } else {
        "pieChart"
    };
    let fallback = super::Series {
        name: "Series 1".into(),
        values: Vec::new(),
    };
    let series = chart.series.first().unwrap_or(&fallback);
    let mut output = format!(
        r#"<c:{tag}><c:varyColors val="1"/>{}{}"#,
        series::write(0, series, chart, series::Kind::Pie)?,
        data_labels(&chart.data_labels, &chart.kind)?
    );
    if chart.kind == "doughnut" {
        output.push_str(r#"<c:holeSize val="50"/>"#);
    }
    output.push_str(&format!("</c:{tag}>"));
    Ok(output)
}
