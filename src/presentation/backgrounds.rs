//! A slide override replaces the complete inherited background value.
use crate::{
    model::{Document, Element, ElementKind, Properties, Value},
    properties::text,
};
pub(crate) struct Background {
    pub color: Option<String>,
    pub image: Option<Element>,
    pub fit: String,
}
impl Background {
    pub fn resolve(document: &Document, slide: &Element) -> Self {
        let value = slide
            .attributes
            .get("background")
            .or_else(|| document.presentation.properties.get("background"));
        let mut result = Self {
            color: None,
            image: None,
            fit: String::new(),
        };
        if let Some(Value::Object(fields)) = value {
            result.color = super::color(fields.get("color"));
            result.fit = text(fields, "fit");
            let source = text(fields, "src");
            if !source.is_empty() {
                result.image = Some(Element {
                    kind: ElementKind::Image,
                    attributes: Properties::from([
                        ("src".into(), Value::String(source)),
                        (
                            "alt".into(),
                            Value::String("presentation background".into()),
                        ),
                    ]),
                    children: Vec::new(),
                });
            }
        } else {
            result.color = super::color(value);
        }
        result
    }
    pub fn container(&self, width: i64, height: i64) -> Element {
        Element {
            kind: ElementKind::Box,
            attributes: Properties::from([
                (
                    "position".into(),
                    Value::Object(Properties::from([
                        ("width".into(), Value::Number(width as f64)),
                        ("height".into(), Value::Number(height as f64)),
                    ])),
                ),
                ("fit".into(), Value::String(self.fit.clone())),
            ]),
            children: Vec::new(),
        }
    }
}
