use super::{LayoutError, style_values as values};
use crate::model::{Element, Properties, Value};
use std::collections::BTreeMap;

#[derive(Default, Clone)]
pub(super) struct HeadingNumbers {
    instances: BTreeMap<(String, String, String), HeadingInstance>,
}
#[derive(Default, Clone)]
struct HeadingInstance {
    current: BTreeMap<usize, u64>,
    starts: BTreeMap<usize, u64>,
}
impl HeadingNumbers {
    pub fn next(&mut self, heading: &Element) -> Result<Option<String>, LayoutError> {
        let Some(Value::Object(numbering)) = heading.attributes.get("numbering") else {
            return Ok(None);
        };
        if numbering.contains_key("enabled") && !values::boolean(numbering, "enabled") {
            return Ok(None);
        }
        let authored_format = values::text(numbering, "format");
        let format = match authored_format.as_str() {
            "lower_alpha" | "upper_alpha" | "lower_roman" | "upper_roman" => authored_format,
            _ => "decimal".into(),
        };
        let separator = text_or(numbering, "separator", ".");
        let identity = text_or(numbering, "id", "headings");
        let key = (format.clone(), separator.clone(), identity);
        let level = integer(&heading.attributes, "level", 1).clamp(1, 9) as usize - 1;
        if values::boolean(numbering, "restart") || !self.instances.contains_key(&key) {
            self.instances.insert(
                key.clone(),
                HeadingInstance {
                    starts: BTreeMap::from([(level, integer(numbering, "start", 1))]),
                    ..Default::default()
                },
            );
        }
        let instance = self.instances.get_mut(&key).unwrap();
        for ancestor in 0..level {
            instance
                .current
                .entry(ancestor)
                .or_insert(*instance.starts.get(&ancestor).unwrap_or(&1));
        }
        let number = match instance.current.get(&level) {
            Some(previous) => previous
                .checked_add(1)
                .ok_or(LayoutError::InvalidGeometry("heading counter overflow"))?,
            None => *instance.starts.get(&level).unwrap_or(&1),
        };
        instance.current.insert(level, number);
        instance.current.retain(|candidate, _| *candidate <= level);
        let parts = (0..=level)
            .map(|level| {
                crate::numbering::format_counter(instance.current[&level], &format)
                    .map_err(LayoutError::InvalidGeometry)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some(parts.join(&separator) + &separator))
    }
}

pub(super) fn text_or(properties: &Properties, name: &str, fallback: &str) -> String {
    let text = values::text(properties, name);
    if text.is_empty() {
        fallback.into()
    } else {
        text
    }
}
pub(super) fn integer(properties: &Properties, name: &str, fallback: u64) -> u64 {
    properties
        .get(name)
        .and_then(values::number)
        .filter(|value| *value >= 1.0 && *value < u64::MAX as f64)
        .map(|value| value as u64)
        .unwrap_or(fallback)
}
