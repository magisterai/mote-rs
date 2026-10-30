use super::{boolean, color, drawing_units, text, text_or};
use crate::model::{Properties, Value};

pub(crate) struct Legend {
    pub enabled: bool,
    pub position: String,
}
impl Legend {
    pub fn from_value(value: Option<&Value>, kind: &str, count: usize) -> Self {
        let position = if matches!(
            kind,
            "line" | "area" | "stacked_area" | "percent_stacked_area"
        ) {
            "t"
        } else {
            "b"
        };
        match value {
            None => Self {
                enabled: matches!(kind, "pie" | "doughnut") || count > 1,
                position: position.into(),
            },
            Some(Value::Boolean(enabled)) => Self {
                enabled: *enabled,
                position: position.into(),
            },
            Some(Value::Object(fields)) => Self {
                enabled: fields
                    .get("enabled")
                    .map(|value| boolean(Some(value)))
                    .unwrap_or(true),
                position: text_or(fields.get("position"), position),
            },
            _ => {
                let value = text(value);
                Self {
                    enabled: value != "false",
                    position: if matches!(value.as_str(), "" | "true" | "false") {
                        position.into()
                    } else {
                        value
                    },
                }
            }
        }
    }
}
#[derive(Default)]
pub(crate) struct Border {
    pub enabled: bool,
    pub color: String,
    pub width: i64,
}
impl Border {
    pub fn from_value(value: Option<&Value>) -> Self {
        match value {
            None => Self::default(),
            Some(Value::Boolean(enabled)) => Self {
                enabled: *enabled,
                ..Default::default()
            },
            Some(Value::Object(fields)) => Self {
                enabled: fields
                    .get("enabled")
                    .map(|value| boolean(Some(value)))
                    .unwrap_or(true),
                color: color(fields.get("color")),
                width: drawing_units(fields.get("width")).unwrap_or_default(),
            },
            _ => Self {
                enabled: true,
                color: color(value),
                width: 0,
            },
        }
    }
}
pub(crate) struct ShapeProperties {
    pub fill: String,
    pub border: Border,
}
impl ShapeProperties {
    pub fn from_properties(properties: &Properties) -> Self {
        Self {
            fill: ["background", "fill", "color"]
                .into_iter()
                .map(|name| color(properties.get(name)))
                .find(|color| !color.is_empty())
                .unwrap_or_default(),
            border: Border::from_value(properties.get("border")),
        }
    }
}
pub(crate) struct Gridlines {
    pub major: bool,
    pub minor: bool,
    pub color: String,
    pub width: i64,
}
impl Gridlines {
    pub fn from_value(value: Option<&Value>) -> Self {
        let mut gridlines = Self {
            major: true,
            minor: false,
            color: String::new(),
            width: 0,
        };
        match value {
            Some(Value::Boolean(major)) => gridlines.major = *major,
            Some(Value::Object(fields)) => {
                gridlines.major = fields
                    .get("major")
                    .map(|value| boolean(Some(value)))
                    .unwrap_or(true);
                gridlines.minor = boolean(fields.get("minor"));
                gridlines.color = color(fields.get("color"));
                gridlines.width = drawing_units(fields.get("width")).unwrap_or_default();
            }
            _ => {}
        }
        gridlines
    }
}
#[derive(Default)]
pub(crate) struct DataLabels {
    pub enabled: bool,
    pub position: String,
    pub value: bool,
    pub category: bool,
    pub series: bool,
    pub percentage: bool,
}
impl DataLabels {
    pub fn from_value(value: Option<&Value>) -> Self {
        match value {
            Some(Value::Boolean(enabled)) => Self {
                enabled: *enabled,
                value: *enabled,
                ..Default::default()
            },
            Some(Value::Object(fields)) => {
                let mut labels = Self {
                    enabled: fields
                        .get("enabled")
                        .map(|value| boolean(Some(value)))
                        .unwrap_or(true),
                    position: text(fields.get("position")),
                    value: boolean(fields.get("value")),
                    category: boolean(fields.get("category")),
                    series: boolean(fields.get("series")),
                    percentage: boolean(fields.get("percentage")) || boolean(fields.get("percent")),
                };
                if labels.enabled
                    && !labels.value
                    && !labels.category
                    && !labels.series
                    && !labels.percentage
                {
                    labels.value = true;
                }
                labels
            }
            _ => Self::default(),
        }
    }
}
pub(super) fn colors(value: Option<&Value>) -> Vec<String> {
    match value {
        Some(Value::Array(values)) => values
            .iter()
            .map(|value| color(Some(value)))
            .filter(|color| !color.is_empty())
            .collect(),
        Some(Value::Object(fields)) => (1..)
            .map(|index| color(fields.get(&format!("series{index}"))))
            .take_while(|color| !color.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}
