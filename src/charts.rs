//! Resolved chart data and appearance shared by editable and native output.
pub(crate) mod appearance;
mod data;
pub(crate) use data::validate_document;
pub(crate) mod drawingml;
pub(crate) mod support;
pub(crate) mod workbook;

use crate::drawings::length_in_drawing_units as drawing_units;
use crate::{
    model::{Element, Value},
    properties::object,
};
use appearance::{Border, DataLabels, Gridlines, Legend, ShapeProperties};
use std::{error::Error as StandardError, fmt};

#[derive(Debug)]
pub(crate) enum Error {
    InvalidValue(String),
    Xml(crate::xml::InvalidCharacter),
    Io(std::io::Error),
    Archive(zip::result::ZipError),
}
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(message) => formatter.write_str(message),
            Self::Xml(error) => error.fmt(formatter),
            Self::Io(error) => error.fmt(formatter),
            Self::Archive(error) => error.fmt(formatter),
        }
    }
}
impl StandardError for Error {}
impl From<crate::xml::InvalidCharacter> for Error {
    fn from(error: crate::xml::InvalidCharacter) -> Self {
        Self::Xml(error)
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Archive(error)
    }
}

pub(crate) struct Series {
    pub name: String,
    pub values: Vec<f64>,
}
#[derive(Default)]
pub(crate) struct TitleStyle {
    pub size: Option<f64>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub color: Option<String>,
    pub font: Option<String>,
}
impl TitleStyle {
    fn from_value(value: Option<&Value>) -> Self {
        let Some(Value::Object(fields)) = value else {
            return Self::default();
        };
        let flag = |name: &str| fields.get(name).map(|value| boolean(Some(value)));
        Self {
            size: drawing_units(fields.get("size")).map(|units| units as f64 / 12700.0),
            bold: flag("bold"),
            italic: flag("italic"),
            color: Some(color(fields.get("color"))).filter(|color| !color.is_empty()),
            font: Some(text(fields.get("font"))).filter(|font| !font.is_empty()),
        }
    }
}
pub(crate) struct Chart {
    pub kind: String,
    pub title: String,
    pub title_style: TitleStyle,
    pub categories: Vec<String>,
    pub series: Vec<Series>,
    pub legend: Legend,
    pub category_title: String,
    pub value_title: String,
    pub category_title_style: TitleStyle,
    pub value_title_style: TitleStyle,
    pub value_minimum: Option<f64>,
    pub value_maximum: Option<f64>,
    pub value_step: Option<f64>,
    pub colors: Vec<String>,
    pub chart_area: ShapeProperties,
    pub plot_area: Option<ShapeProperties>,
    pub gridlines: Gridlines,
    pub data_labels: DataLabels,
    /// The size in points of the axis labels, legend entries and data labels.
    pub label_size: f64,
    pub gap_width: Option<i64>,
    pub overlap: Option<i64>,
    pub width: i64,
    pub height: i64,
}
impl Chart {
    pub fn from_element(element: &Element) -> Result<Self, Error> {
        let attributes = &element.attributes;
        let (categories, series) = data::resolve(object(attributes, "data"))?;
        let kind = text_or(attributes.get("kind"), "column");
        let axis = object(attributes, "axis");
        let axis_table = |name| axis.and_then(|axis| object(axis, name));
        let axis_title = |name| {
            axis_table(name)
                .map(|axis| {
                    let title = match axis.get("title") {
                        Some(Value::Object(fields)) => text(fields.get("text")),
                        value => text(value),
                    };
                    if title.is_empty() {
                        text(axis.get("name"))
                    } else {
                        title
                    }
                })
                .unwrap_or_default()
        };
        let axis_title_style =
            |name| TitleStyle::from_value(axis_table(name).and_then(|axis| axis.get("title")));
        // A percentage scale and a chart without axes have no free value range.
        let ranged =
            !kind.contains("percent_stacked") && !matches!(kind.as_str(), "pie" | "doughnut");
        let value_bound = |name| {
            axis.and_then(|axis| object(axis, "value"))
                .and_then(|axis| match axis.get(name) {
                    Some(Value::Number(number)) if ranged && number.is_finite() => Some(*number),
                    _ => None,
                })
        };
        Ok(Self {
            value_minimum: value_bound("minimum"),
            value_maximum: value_bound("maximum"),
            value_step: value_bound("step").filter(|step| *step > 0.0),
            title: match attributes.get("title") {
                Some(Value::Object(fields)) => text(fields.get("text")),
                value => text(value),
            },
            title_style: TitleStyle::from_value(attributes.get("title")),
            category_title: axis_title("category"),
            value_title: axis_title("value"),
            category_title_style: axis_title_style("category"),
            value_title_style: axis_title_style("value"),
            colors: appearance::colors(attributes.get("colors")),
            chart_area: ShapeProperties {
                fill: color(attributes.get("background")),
                border: Border::from_value(attributes.get("border")),
            },
            plot_area: object(attributes, "plot_area").map(ShapeProperties::from_properties),
            gridlines: Gridlines::from_value(attributes.get("gridlines")),
            data_labels: DataLabels::from_value(attributes.get("data_labels")),
            label_size: object(attributes, "text")
                .and_then(|text| drawing_units(text.get("size")))
                .map(|units| units as f64 / 12700.0)
                .filter(|size| *size > 0.0)
                .unwrap_or(DEFAULT_LABEL_SIZE),
            gap_width: integer(attributes.get("gap_width")),
            overlap: integer(attributes.get("overlap")),
            width: drawing_units(attributes.get("width")).unwrap_or(5486400),
            height: drawing_units(attributes.get("height")).unwrap_or(3200400),
            legend: Legend::from_value(attributes.get("legend"), &kind, series.len()),
            kind,
            categories,
            series,
        })
    }
    pub fn color(&self, index: usize) -> &str {
        const PALETTE: [&str; 6] = ["2563EB", "059669", "D97706", "DC2626", "7C3AED", "0891B2"];
        self.colors
            .get(index)
            .map(String::as_str)
            .filter(|color| !color.is_empty())
            .unwrap_or(PALETTE[index % PALETTE.len()])
    }
    /// The ratio of the label size to the size for which the frames and
    /// reserves of native output were measured, by which native output scales
    /// them.
    pub fn label_scale(&self) -> f64 {
        self.label_size / MEASURED_LABEL_SIZE
    }
    /// The part of a point by which labels smaller than those measured change a
    /// distance that Word and PowerPoint do not scale with the label size. The
    /// distances were measured in exports of charts with labels of 9 points.
    pub fn small_label_shift(&self, points_per_scale: f64) -> f64 {
        points_per_scale * (1.0 - self.label_scale()).max(0.0)
    }
    pub fn title_size(&self) -> f64 {
        self.title_style.size.unwrap_or(DEFAULT_TITLE_SIZE)
    }
    pub fn category_label_position(&self) -> &'static str {
        if self
            .series
            .iter()
            .any(|series| series.values.iter().any(|value| *value < 0.0))
        {
            "low"
        } else {
            "nextTo"
        }
    }
}
pub(crate) const DEFAULT_LABEL_SIZE: f64 = 9.0;
/// The label size at which the label frames and reserves of native output were
/// measured against Office.
pub(crate) const MEASURED_LABEL_SIZE: f64 = 10.0;
/// The sizes that chart style 201, which Office gives a new chart, states for the
/// chart title and for an axis title.
pub(crate) const DEFAULT_TITLE_SIZE: f64 = 14.0;
pub(crate) const DEFAULT_AXIS_TITLE_SIZE: f64 = 10.0;
/// The title size at which the title frames and reserves of native output were
/// measured against Office, which draws a title of no stated size at 18 points.
pub(crate) const MEASURED_TITLE_SIZE: f64 = 18.0;
impl TitleStyle {
    pub fn axis_title_size(&self) -> f64 {
        self.size.unwrap_or(DEFAULT_AXIS_TITLE_SIZE)
    }
    /// The ratio by which native output scales the frame of an axis title.
    pub fn axis_title_scale(&self) -> f64 {
        self.axis_title_size() / MEASURED_TITLE_SIZE
    }
}
/// A distance of native output that was measured against Office for a title of
/// 18 points, at a title of the given size. Word and PowerPoint exports of
/// titles of 10 and 14 points give the part of a point by which the distance
/// changes for each point of size, which is less than the proportion of the
/// sizes would give.
pub(crate) fn title_distance(measured: f64, points_per_point: f64, size: f64) -> f64 {
    (measured - points_per_point * (MEASURED_TITLE_SIZE - size)).max(0.6 * size)
}
/// The colour and width of an unstyled gridline or category axis line in Office's
/// default chart style, which tints black text to 15% luminance offset by 85%.
pub(crate) const RULE_COLOR: &str = "D9D9D9";
pub(crate) const RULE_WIDTH: i64 = 9525;
/// A stated step that divides the value range into more intervals than these is
/// rejected, since its labels and gridlines could not be told apart.
pub(crate) const MAXIMUM_VALUE_INTERVALS: f64 = 100.0;
/// A Word chart takes its text colour from the default Office theme, whose dark
/// text colour is black.
pub(crate) const WORD_TEXT_COLOR: &str = "000000";

/// Office's default chart style draws axis labels in the text colour with its HSL
/// luminance scaled by 65% and offset by 35%, as `lumMod` and `lumOff` define.
pub(crate) fn axis_label_color(text_color: &str) -> String {
    let color = crate::properties::hex_color(text_color).unwrap_or(WORD_TEXT_COLOR.into());
    let [red, green, blue] = [0, 2, 4].map(|start| {
        f64::from(u8::from_str_radix(&color[start..start + 2], 16).unwrap_or_default()) / 255.0
    });
    let maximum = red.max(green).max(blue);
    let minimum = red.min(green).min(blue);
    let lightness = (maximum + minimum) / 2.0;
    let delta = maximum - minimum;
    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };
    let hue = if delta == 0.0 {
        0.0
    } else if maximum == red {
        ((green - blue) / delta).rem_euclid(6.0)
    } else if maximum == green {
        (blue - red) / delta + 2.0
    } else {
        (red - green) / delta + 4.0
    };
    let lightness = (lightness * 0.65 + 0.35).clamp(0.0, 1.0);
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let secondary = chroma * (1.0 - (hue.rem_euclid(2.0) - 1.0).abs());
    let (red, green, blue) = match hue as u8 {
        0 => (chroma, secondary, 0.0),
        1 => (secondary, chroma, 0.0),
        2 => (0.0, chroma, secondary),
        3 => (0.0, secondary, chroma),
        4 => (secondary, 0.0, chroma),
        _ => (chroma, 0.0, secondary),
    };
    let offset = lightness - chroma / 2.0;
    [red, green, blue]
        .map(|channel| format!("{:02X}", ((channel + offset) * 255.0).round() as u8))
        .concat()
}

fn text(value: Option<&Value>) -> String {
    value
        .map(|value| value.to_text().trim().to_owned())
        .unwrap_or_default()
}
fn text_or(value: Option<&Value>, fallback: &str) -> String {
    let text = text(value);
    if text.is_empty() {
        fallback.into()
    } else {
        text
    }
}
fn boolean(value: Option<&Value>) -> bool {
    text(value) == "true"
}
fn color(value: Option<&Value>) -> String {
    crate::properties::hex_color(&text(value)).unwrap_or_default()
}
fn integer(value: Option<&Value>) -> Option<i64> {
    match value? {
        Value::Number(number) if number.is_finite() => Some(*number as i64),
        Value::String(text) => text.trim().parse().ok(),
        _ => None,
    }
}
