use super::{Error, check_attributes, values};
use crate::model::Properties;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ListLayout {
    pub left: i64,
    pub level_indent: i64,
    pub hanging: i64,
    pub spacing_after: i64,
}

impl ListLayout {
    pub fn from_list(attributes: &Properties) -> Result<Self, Error> {
        Self {
            left: 720,
            level_indent: 360,
            hanging: 360,
            spacing_after: 40,
        }
        .apply(attributes)
    }

    pub fn for_item(self, attributes: &Properties, level: u8) -> Result<Self, Error> {
        let mut layout = self.apply(attributes)?;
        layout.left = layout.left_at_level(level)?;
        if let Some(indent) = attributes.get("indent")
            && let Some(left) = values::object(indent)?.get("left")
        {
            // An item-level left indent is an absolute position.
            layout.left = values::twips(left)?;
        }
        Ok(layout)
    }

    pub fn left_at_level(self, level: u8) -> Result<i64, Error> {
        let left = self.left + i64::from(level) * self.level_indent;
        if left > i64::from(i32::MAX) {
            return Err(Error::InvalidValue(
                "list indentation exceeds the Word integer range".into(),
            ));
        }
        Ok(left)
    }

    fn apply(mut self, attributes: &Properties) -> Result<Self, Error> {
        if attributes
            .get("tight")
            .map(values::boolean)
            .transpose()?
            .unwrap_or(false)
        {
            self.spacing_after = 0;
        }
        if let Some(indent) = attributes.get("indent") {
            let indent = values::object(indent)?;
            check_attributes(indent, &["left", "hanging", "level"], "list indent")?;
            for (name, target) in [
                ("left", &mut self.left),
                ("hanging", &mut self.hanging),
                ("level", &mut self.level_indent),
            ] {
                if let Some(value) = indent.get(name) {
                    *target = values::twips(value)?;
                }
            }
        }
        if let Some(spacing) = attributes.get("spacing") {
            let spacing = values::object(spacing)?;
            check_attributes(spacing, &["after"], "list spacing")?;
            if let Some(after) = spacing.get("after") {
                self.spacing_after = values::twips(after)?;
            }
        }
        Ok(self)
    }

    pub fn write_paragraph_properties(self) -> String {
        format!(
            "<w:spacing w:after=\"{}\"/><w:ind w:left=\"{}\" w:hanging=\"{}\"/>",
            self.spacing_after, self.left, self.hanging
        )
    }
}

pub(super) fn nesting_level(attributes: &Properties, fallback: u8) -> Result<u8, Error> {
    let Some(value) = attributes.get("level") else {
        if fallback <= 8 {
            return Ok(fallback);
        }
        return Err(Error::Unsupported(
            "list nesting deeper than nine levels".into(),
        ));
    };
    let level = values::number(value)?;
    if level.fract() != 0.0 || !(1.0..=9.0).contains(&level) {
        return Err(Error::InvalidValue(
            "Word list levels must be integers from 1 through 9".into(),
        ));
    }
    Ok(level as u8 - 1)
}
