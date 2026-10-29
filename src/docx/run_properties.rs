use super::{Error, check_attributes, values, xml};
use crate::model::{Element, ElementKind, Properties, Value};

pub(super) const ATTRIBUTES: &[&str] = &[
    "font",
    "size",
    "color",
    "highlight",
    "bold",
    "italic",
    "underline",
    "small_caps",
    "strike",
    "double_strike",
];

#[derive(Clone, Default)]
pub(super) struct RunFormatting {
    pub properties: Properties,
    pub character_style: Option<String>,
    pub fixed_tabs: bool,
}

impl RunFormatting {
    pub fn for_element(&self, element: &Element) -> Result<Self, Error> {
        let mut result = self.clone();
        match element.kind {
            ElementKind::Span => {
                check_attributes(&element.attributes, ATTRIBUTES, "span")?;
                result.properties.extend(element.attributes.clone());
            }
            ElementKind::Strong | ElementKind::Emphasis => {
                check_attributes(&element.attributes, &[], element.kind.as_text())?;
                let property = if element.kind == ElementKind::Strong {
                    "bold"
                } else {
                    "italic"
                };
                result
                    .properties
                    .insert(property.into(), Value::Boolean(true));
            }
            ElementKind::Code => {
                check_attributes(&element.attributes, &["preserve_space", "language"], "code")?;
                result.character_style = Some("Code".into());
                result.properties.remove("font");
                result.properties.remove("size");
                if let Some(preserve_space) = element.attributes.get("preserve_space")
                    && values::boolean(preserve_space)?
                {
                    result.fixed_tabs = true;
                }
            }
            _ => {
                return Err(Error::Unsupported(format!(
                    "{} inline content",
                    element.kind.as_text()
                )));
            }
        }
        Ok(result)
    }

    pub fn write(&self) -> Result<String, Error> {
        let mut result = String::new();
        if let Some(name) = &self.character_style {
            result.push_str(&format!("<w:rStyle w:val=\"{}\"/>", xml::escape(name)?));
        }
        if let Some(name) = self.properties.get("font") {
            let name = name.to_text();
            let name = if name.trim().eq_ignore_ascii_case("mono") {
                "Courier New"
            } else {
                &name
            };
            if !name.is_empty() {
                let name = xml::escape(name)?;
                result.push_str(&format!(
                    "<w:rFonts w:ascii=\"{name}\" w:hAnsi=\"{name}\"/>"
                ));
            }
        }
        for (name, word_names) in [
            ("bold", &["b", "bCs"][..]),
            ("italic", &["i", "iCs"][..]),
            ("small_caps", &["smallCaps"][..]),
            ("strike", &["strike"][..]),
            ("double_strike", &["dstrike"][..]),
        ] {
            if let Some(value) = self.properties.get(name) {
                let enabled = values::boolean(value)?;
                for word_name in word_names {
                    result.push_str(&if enabled {
                        format!("<w:{word_name}/>")
                    } else {
                        format!("<w:{word_name} w:val=\"0\"/>")
                    });
                }
            }
        }
        if let Some(color) = self.properties.get("color") {
            result.push_str(&format!("<w:color w:val=\"{}\"/>", values::color(color)?));
        }
        if let Some(size) = self.properties.get("size") {
            let size = values::half_points(size)?;
            if size > 0 {
                result.push_str(&format!(
                    "<w:sz w:val=\"{size}\"/><w:szCs w:val=\"{size}\"/>"
                ));
            }
        }
        if let Some(underline) = self.properties.get("underline") {
            let underline = if values::boolean(underline)? {
                "single"
            } else {
                "none"
            };
            result.push_str(&format!("<w:u w:val=\"{underline}\"/>"));
        }
        if let Some(highlight) = self.properties.get("highlight") {
            result.push_str(&format!(
                "<w:shd w:val=\"clear\" w:color=\"auto\" w:fill=\"{}\"/>",
                values::color(highlight)?
            ));
        }
        Ok(result)
    }
}
