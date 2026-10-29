use super::{Error, list_layout::ListLayout, values, xml};
use crate::model::{Content, Element, ElementKind, Value};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberFormat {
    Bullet,
    Decimal,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
}

impl NumberFormat {
    fn from_name(name: &str) -> Result<Self, Error> {
        Ok(match name {
            "" | "decimal" => Self::Decimal,
            "bullet" => Self::Bullet,
            "lower_alpha" => Self::LowerAlpha,
            "upper_alpha" => Self::UpperAlpha,
            "lower_roman" => Self::LowerRoman,
            "upper_roman" => Self::UpperRoman,
            _ => return Err(Error::Unsupported(format!("numbering format {name:?}"))),
        })
    }

    fn word_name(self) -> &'static str {
        match self {
            Self::Bullet => "bullet",
            Self::Decimal => "decimal",
            Self::LowerAlpha => "lowerLetter",
            Self::UpperAlpha => "upperLetter",
            Self::LowerRoman => "lowerRoman",
            Self::UpperRoman => "upperRoman",
        }
    }
}

#[derive(PartialEq, Eq)]
struct NumberingDefinition {
    format: NumberFormat,
    layout: ListLayout,
}

struct NumberingInstance {
    definition: usize,
    level: u8,
    start: i64,
    restart: bool,
}

#[derive(Default)]
pub(super) struct Numbering {
    definitions: Vec<NumberingDefinition>,
    instances: Vec<NumberingInstance>,
    continued: BTreeMap<(usize, String), usize>,
}

impl Numbering {
    pub fn for_list(
        &mut self,
        list: &Element,
        layout: ListLayout,
        level: u8,
    ) -> Result<usize, Error> {
        let ordered = list.kind == ElementKind::OrderedList;
        let format = if ordered {
            NumberFormat::from_name(
                &list
                    .attributes
                    .get("format")
                    .map(Value::to_text)
                    .unwrap_or_default(),
            )?
        } else {
            NumberFormat::Bullet
        };
        let definition = self.definition_identifier(NumberingDefinition { format, layout });
        let identity = list
            .attributes
            .get("id")
            .map(Value::to_text)
            .unwrap_or_default();
        let restart = list
            .attributes
            .get("restart")
            .map(values::boolean)
            .transpose()?
            .unwrap_or(identity.is_empty());
        let start = if ordered { ordered_start(list)? } else { 1 };
        let identity = if identity.is_empty() {
            format!("level:{level}")
        } else {
            identity
        };
        Ok(self.instance_identifier(definition, identity, level, start, ordered && restart))
    }

    fn definition_identifier(&mut self, definition: NumberingDefinition) -> usize {
        match self
            .definitions
            .iter()
            .position(|candidate| *candidate == definition)
        {
            Some(index) => index + 1,
            None => {
                self.definitions.push(definition);
                self.definitions.len()
            }
        }
    }

    fn instance_identifier(
        &mut self,
        definition: usize,
        identity: String,
        level: u8,
        start: i64,
        restart: bool,
    ) -> usize {
        let key = (definition, identity);
        if !restart && let Some(identifier) = self.continued.get(&key) {
            return *identifier;
        }
        self.instances.push(NumberingInstance {
            definition,
            level,
            start,
            restart,
        });
        let identifier = self.instances.len();
        self.continued.insert(key, identifier);
        identifier
    }

    pub fn write(&self) -> Result<String, Error> {
        let mut content = String::new();
        for (index, definition) in self.definitions.iter().enumerate() {
            content.push_str(&format!(
                "<w:abstractNum w:abstractNumId=\"{}\">",
                index + 1
            ));
            for level in 0..9 {
                let layout = definition.layout;
                let left = layout.left_at_level(level)?;
                let format = definition.format.word_name();
                let label = if definition.format == NumberFormat::Bullet {
                    "•".into()
                } else {
                    format!("%{}.", level + 1)
                };
                content.push_str(&format!("<w:lvl w:ilvl=\"{level}\"><w:start w:val=\"1\"/><w:numFmt w:val=\"{format}\"/><w:lvlText w:val=\"{label}\"/><w:lvlJc w:val=\"left\"/><w:pPr><w:spacing w:after=\"{}\"/><w:ind w:left=\"{left}\" w:hanging=\"{}\"/></w:pPr></w:lvl>", layout.spacing_after, layout.hanging));
            }
            content.push_str("</w:abstractNum>");
        }
        for (index, instance) in self.instances.iter().enumerate() {
            content.push_str(&format!(
                "<w:num w:numId=\"{}\"><w:abstractNumId w:val=\"{}\"/>",
                index + 1,
                instance.definition
            ));
            if instance.restart || instance.start > 1 {
                content.push_str(&format!(
                    "<w:lvlOverride w:ilvl=\"{}\"><w:startOverride w:val=\"{}\"/></w:lvlOverride>",
                    instance.level, instance.start
                ));
            }
            content.push_str("</w:num>");
        }
        Ok(xml::word_part("numbering", &content))
    }
}

fn ordered_start(list: &Element) -> Result<i64, Error> {
    if let Some(value) = list.attributes.get("start") {
        return positive_integer(value, "numbering start");
    }
    let first_item = list.children.iter().find_map(|child| match child {
        Content::Element(element) if element.kind == ElementKind::ListItem => Some(element),
        _ => None,
    });
    let marker = first_item
        .and_then(|item| item.attributes.get("marker"))
        .map(Value::to_text)
        .unwrap_or_default();
    let mut marker = marker.trim();
    for suffix in [" [ ]", " [x]", " [X]", "\t[ ]", "\t[x]", "\t[X]"] {
        if let Some(prefix) = marker.strip_suffix(suffix) {
            marker = prefix.trim_end();
            break;
        }
    }
    let Some(digits) = marker.strip_suffix('.') else {
        return Ok(1);
    };
    if digits.is_empty() || !digits.bytes().all(|character| character.is_ascii_digit()) {
        return Ok(1);
    }
    let Ok(start) = digits.parse::<i64>() else {
        return Ok(1);
    };
    if start <= 0 {
        return Ok(1);
    }
    positive_integer(&Value::Number(start as f64), "source list start")
}

fn positive_integer(value: &Value, context: &str) -> Result<i64, Error> {
    let number = values::number(value)?;
    if number.fract() != 0.0 || number < 1.0 || number > f64::from(i32::MAX) {
        return Err(Error::InvalidValue(format!(
            "{context} must be a positive Word integer"
        )));
    }
    Ok(number as i64)
}

pub(super) fn paragraph_reference(identifier: usize, level: u8) -> String {
    format!("<w:numPr><w:ilvl w:val=\"{level}\"/><w:numId w:val=\"{identifier}\"/></w:numPr>")
}
