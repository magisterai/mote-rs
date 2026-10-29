use super::{Error, check_attributes, fonts, styles::Styles, values, xml};
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
    pub fn for_element(&self, element: &Element, styles: &Styles<'_>) -> Result<Self, Error> {
        let mut result = self.clone();
        match element.kind {
            ElementKind::Span => {
                let mut supported = ATTRIBUTES.to_vec();
                supported.push("style");
                check_attributes(&element.attributes, &supported, "span")?;
                if let Some(name) = element
                    .attributes
                    .get("style")
                    .map(Value::to_text)
                    .filter(|name| !name.is_empty())
                {
                    result.apply_character_style(&name, styles)?;
                }
                result.properties.extend(
                    element
                        .attributes
                        .iter()
                        .filter(|(name, value)| {
                            name.as_str() != "style"
                                && !(name.as_str() == "font" && value.to_text().is_empty())
                        })
                        .map(|(name, value)| (name.clone(), value.clone())),
                );
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
                check_attributes(
                    &element.attributes,
                    &["style", "preserve_space", "language"],
                    "code",
                )?;
                let name = element
                    .attributes
                    .get("style")
                    .map(Value::to_text)
                    .filter(|name| !name.is_empty())
                    .unwrap_or_else(|| "Code".into());
                result.apply_character_style(&name, styles)?;
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

    pub(super) fn apply_character_style(
        &mut self,
        name: &str,
        styles: &Styles<'_>,
    ) -> Result<(), Error> {
        let supplied = styles.resolved_run(name)?;
        self.character_style = Some(styles.identifier(name)?.into());
        // Character styles replace only the inherited direct properties they supply.
        if supplied
            .get("font")
            .is_some_and(|value| !value.to_text().is_empty())
        {
            self.properties.remove("font");
        }
        for property in ["size", "color", "bold", "italic", "underline", "small_caps"] {
            if supplied.contains_key(property) {
                self.properties.remove(property);
            }
        }
        if ["fill", "background", "highlight"]
            .iter()
            .any(|name| supplied.contains_key(*name))
        {
            self.properties.remove("highlight");
        }
        if supplied.contains_key("strike") || supplied.contains_key("double_strike") {
            self.properties.remove("strike");
            self.properties.remove("double_strike");
        }
        Ok(())
    }

    pub fn write(&self, styles: &Styles<'_>) -> Result<String, Error> {
        self.write_properties(styles, false)
    }

    pub fn write_declaration(&self, styles: &Styles<'_>) -> Result<String, Error> {
        self.write_properties(styles, true)
    }

    fn write_properties(&self, styles: &Styles<'_>, declaration: bool) -> Result<String, Error> {
        let mut supported = ATTRIBUTES.to_vec();
        if declaration {
            supported.extend_from_slice(&["fill", "background"]);
        }
        check_attributes(&self.properties, &supported, "run")?;
        let mut result = String::new();
        if let Some(name) = &self.character_style {
            result.push_str(&format!("<w:rStyle w:val=\"{}\"/>", xml::escape(name)?));
        }
        if let Some(name) = self.properties.get("font") {
            result.push_str(&fonts::run_fonts(styles.document, &name.to_text())?);
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
        if declaration {
            for name in ["fill", "background", "highlight"] {
                if let Some(color) = self.properties.get(name) {
                    result.push_str(&shading(color)?);
                }
            }
        }
        if let Some(size) = self.properties.get("size") {
            let size = values::half_points(size)?;
            if declaration && size == 0 {
                return Err(Error::InvalidValue(
                    "declared Word style size must be positive".into(),
                ));
            }
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
        if !declaration && let Some(highlight) = self.properties.get("highlight") {
            result.push_str(&shading(highlight)?);
        }
        Ok(result)
    }
}

fn shading(color: &Value) -> Result<String, Error> {
    Ok(format!(
        "<w:shd w:val=\"clear\" w:color=\"auto\" w:fill=\"{}\"/>",
        values::color(color)?
    ))
}
