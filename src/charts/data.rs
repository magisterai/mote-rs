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
            let number = match value {
                Value::Number(number) => *number,
                _ => value.to_text().trim().parse::<f64>().unwrap_or_default(),
            };
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
