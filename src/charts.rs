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
    pub colors: Vec<String>,
    pub chart_area: ShapeProperties,
    pub plot_area: Option<ShapeProperties>,
    pub gridlines: Gridlines,
    pub data_labels: DataLabels,
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
        let axis_title = |name| {
            axis.and_then(|axis| object(axis, name))
                .map(|axis| {
                    ["title", "name"]
                        .into_iter()
                        .map(|key| text(axis.get(key)))
                        .find(|title| !title.is_empty())
                        .unwrap_or_default()
                })
                .unwrap_or_default()
        };
        Ok(Self {
            title: match attributes.get("title") {
                Some(Value::Object(fields)) => text(fields.get("text")),
                value => text(value),
            },
            title_style: TitleStyle::from_value(attributes.get("title")),
            category_title: axis_title("category"),
            value_title: axis_title("value"),
            colors: appearance::colors(attributes.get("colors")),
            chart_area: ShapeProperties {
                fill: color(attributes.get("background")),
                border: Border::from_value(attributes.get("border")),
            },
            plot_area: object(attributes, "plot_area").map(ShapeProperties::from_properties),
            gridlines: Gridlines::from_value(attributes.get("gridlines")),
            data_labels: DataLabels::from_value(attributes.get("data_labels")),
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
    let text = text(value);
    let text = text.strip_prefix('#').unwrap_or(&text);
    let text = if text.len() == 3 && text.is_ascii() {
        text.chars()
            .flat_map(|character| [character, character])
            .collect()
    } else {
        text.to_owned()
    };
    text.to_uppercase()
}
fn integer(value: Option<&Value>) -> Option<i64> {
    match value? {
        Value::Number(number) if number.is_finite() => Some(*number as i64),
        Value::String(text) => text.trim().parse().ok(),
        _ => None,
    }
}
