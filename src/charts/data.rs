use super::{Error, Series, text_or};
use crate::model::{Properties, Value};

pub(super) fn resolve(data: Option<&Properties>) -> Result<(Vec<String>, Vec<Series>), Error> {
    let Some(data) = data else {
        return Ok((Vec::new(), Vec::new()));
    };
    let mut categories = ["categories", "labels"]
        .into_iter()
        .find_map(|name| {
            array(data.get(name))
                .map(|values| values.iter().map(Value::to_text).collect::<Vec<_>>())
        })
        .unwrap_or_default();
    let series = if let Some(series) = array(data.get("series")) {
        series
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                let Value::Object(fields) = value else {
                    return None;
                };
                array(fields.get("values")).map(|values| {
                    resolve_series(
                        text_or(fields.get("name"), &format!("Series {}", index + 1)),
                        values,
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?
    } else if let Some(values) = array(data.get("values")) {
        vec![resolve_series(
            text_or(data.get("name"), "Series 1"),
            values,
        )?]
    } else {
        Vec::new()
    };
    if categories.is_empty() {
        let count = series
            .iter()
            .map(|series| series.values.len())
            .max()
            .unwrap_or_default();
        categories = (1..=count).map(|index| index.to_string()).collect();
    }
    Ok((categories, series))
}
/// The stated members of the data that the chart cannot read. Each is passed
/// over or charted as zero, so that without a warning the chart would be drawn
/// empty or flat with no account of the cause.
pub(crate) fn warnings(data: &Properties, path: &str) -> Vec<String> {
    let mut warnings = Vec::new();
    let category_members = ["categories", "labels"].map(|name| (name, stated(data, name)));
    if !category_members
        .iter()
        .any(|(_, value)| matches!(value, Some(Value::Array(_))))
    {
        for (name, _) in category_members.iter().filter(|(_, value)| value.is_some()) {
            warnings.push(format!(
                "{path}.data.{name} is not an array and gives the chart no categories"
            ));
        }
    }
    match stated(data, "series") {
        Some(Value::Array(series)) => {
            for (index, entry) in series.iter().enumerate() {
                let path = format!("{path}.data.series entry {}", index + 1);
                match entry {
                    Value::Object(fields) => values_warnings(
                        &format!("{path} values"),
                        fields.get("values"),
                        &mut warnings,
                    ),
                    _ => warnings.push(format!(
                        "{path} is not a table with a values array and gives the chart no series"
                    )),
                }
            }
        }
        // A series member that is no array yields to the single-series form.
        series => match stated(data, "values") {
            Some(values) => {
                values_warnings(&format!("{path}.data.values"), Some(values), &mut warnings)
            }
            None if series.is_some() => warnings.push(format!(
                "{path}.data.series is not an array and gives the chart no series"
            )),
            None => {}
        },
    }
    warnings
}

fn stated<'value>(fields: &'value Properties, name: &str) -> Option<&'value Value> {
    fields.get(name).filter(|value| **value != Value::Nil)
}

fn values_warnings(path: &str, values: Option<&Value>, warnings: &mut Vec<String>) {
    let Some(Value::Array(values)) = values else {
        warnings.push(format!(
            "{path} is not an array and gives the chart no series"
        ));
        return;
    };
    for (index, value) in values.iter().enumerate() {
        if number(value).is_none() {
            warnings.push(format!(
                "{path} entry {} is not a number and is charted as zero",
                index + 1
            ));
        }
    }
}

fn number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => Some(*number),
        _ => value.to_text().trim().parse().ok(),
    }
}

fn array(value: Option<&Value>) -> Option<&[Value]> {
    match value? {
        Value::Array(values) => Some(values),
        _ => None,
    }
}
fn resolve_series(name: String, values: &[Value]) -> Result<Series, Error> {
    let values = values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let number = number(value).unwrap_or_default();
            if number.is_finite() {
                Ok(number)
            } else {
                Err(Error::InvalidValue(format!(
                    "chart series {name:?} value {} must be finite",
                    index + 1
                )))
            }
        })
        .collect::<Result<_, _>>()?;
    Ok(Series { name, values })
}

/// Check every chart before output, including charts in stories that may be flattened.
pub(crate) fn validate_document(document: &crate::model::Document) -> Result<(), Error> {
    validate_content(&document.content)?;
    for section in &document.sections {
        for content in [
            &section.content,
            &section.header.content,
            &section.footer.content,
        ] {
            validate_content(content)?;
        }
    }
    Ok(())
}

fn validate_content(content: &[crate::model::Content]) -> Result<(), Error> {
    for node in content {
        if let crate::model::Content::Element(element) = node {
            if element.kind == crate::model::ElementKind::Chart {
                resolve(crate::properties::object(&element.attributes, "data"))?;
            }
            validate_content(&element.children)?;
        }
    }
    Ok(())
}
