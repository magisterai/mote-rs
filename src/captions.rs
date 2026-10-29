//! Shared caption numbering and figure child defaults.
use crate::{
    model::{Element, ElementKind, Properties, Value},
    properties::{boolean, text},
};
use std::collections::BTreeMap;
#[derive(Clone)]
pub(crate) struct Numbering {
    pub enabled: bool,
    pub sequence: String,
    pub label: String,
    pub format: String,
    pub separator: String,
    pub restart: bool,
    pub start: u64,
}
impl Numbering {
    pub fn resolve(kind: &str, caption: &Properties, parent: &Properties) -> Self {
        let sources: Vec<_> = [caption, parent]
            .into_iter()
            .filter_map(|attributes| match attributes.get("numbering") {
                Some(Value::Object(fields)) => Some(fields),
                _ => None,
            })
            .collect();
        let enabled = sources
            .first()
            .is_some_and(|fields| !fields.contains_key("enabled") || boolean(fields, "enabled"));
        let mut fields = Properties::new();
        for source in sources.into_iter().rev() {
            fields.extend(source.clone());
        }
        let label = [text(&fields, "label"), text(&fields, "prefix")]
            .into_iter()
            .find(|value| !value.is_empty())
            .unwrap_or_else(|| {
                if kind.eq_ignore_ascii_case("table") {
                    "Table"
                } else {
                    "Figure"
                }
                .into()
            });
        let identifier = text(&fields, "id");
        Self {
            enabled,
            sequence: crate::references::bookmark_name(if identifier.is_empty() {
                &label
            } else {
                &identifier
            }),
            label,
            format: text(&fields, "format"),
            separator: {
                let separator = text(&fields, "separator");
                if separator.is_empty() {
                    ". ".into()
                } else {
                    separator
                }
            },
            restart: boolean(&fields, "restart"),
            start: crate::properties::number(&fields, "start")
                .filter(|number| *number > 0.0 && *number < u64::MAX as f64)
                .map(|number| number as u64)
                .unwrap_or(0),
        }
    }
    pub fn advance(
        &self,
        sequences: &mut BTreeMap<String, u64>,
    ) -> Result<Option<Number>, &'static str> {
        if !self.enabled {
            return Ok(None);
        }
        let previous = sequences.get(&self.sequence).copied().unwrap_or(0);
        let value = if self.restart || previous == 0 {
            self.start.max(1)
        } else {
            previous.checked_add(1).ok_or("caption counter overflow")?
        };
        sequences.insert(self.sequence.clone(), value);
        Ok(Some(Number {
            value,
            formatted: crate::numbering::format_counter(value, &self.format)?,
            sequence_started: previous > 0,
        }))
    }
}
pub(crate) struct Number {
    pub value: u64,
    pub formatted: String,
    pub sequence_started: bool,
}
pub(crate) fn aligned_child(element: &Element, alignment: &str) -> Element {
    let mut element = element.clone();
    if alignment.is_empty() {
        return element;
    }
    if matches!(
        element.kind,
        ElementKind::Paragraph
            | ElementKind::Heading
            | ElementKind::Blockquote
            | ElementKind::Caption
            | ElementKind::Table
    ) && text(&element.attributes, "alignment").is_empty()
    {
        element
            .attributes
            .insert("alignment".into(), Value::String(alignment.into()));
    } else if element.kind == ElementKind::Image
        && crate::media::image_alignment(&element.attributes).is_empty()
    {
        let mut placement = match element.attributes.get("placement") {
            Some(Value::Object(fields)) => fields.clone(),
            _ => Properties::new(),
        };
        placement.insert("align".into(), Value::String(alignment.into()));
        element
            .attributes
            .insert("placement".into(), Value::Object(placement));
    }
    element
}
