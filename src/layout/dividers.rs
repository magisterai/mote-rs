//! Horizontal document rules with ordinary block spacing and flow.
use super::{LayoutStyleResolver, PageOperation, Rule, RuleRole, TextPage, style_values};
use crate::{
    model::{Element, Value},
    properties,
};

pub(super) struct Divider {
    pub width: f64,
    pub color: String,
    pub before: f64,
    pub after: f64,
    pub anchors: Vec<String>,
    pub clamped: bool,
}
impl Divider {
    pub fn prepare(element: &Element, styles: &LayoutStyleResolver<'_>) -> Self {
        let mut element = element.clone();
        element
            .attributes
            .entry("style".into())
            .or_insert(Value::String("HorizontalRule".into()));
        let style = styles.paragraph(&element);
        let width = element
            .attributes
            .get("width")
            .and_then(style_values::points)
            .unwrap_or(1.0);
        let spacing = properties::object(&element.attributes, "spacing");
        let before = spacing
            .and_then(|spacing| {
                spacing
                    .get("before")
                    .or_else(|| spacing.get("spacing_before"))
            })
            .and_then(style_values::points);
        let after = spacing
            .and_then(|spacing| {
                spacing
                    .get("after")
                    .or_else(|| spacing.get("spacing_after"))
            })
            .and_then(style_values::points);
        Self {
            width: if width <= 0.0 { 1.0 } else { width },
            clamped: width <= 0.0,
            color: style_values::color(&element.attributes, "color")
                .or_else(|| styles.border_color(&element))
                .unwrap_or(style.text.color),
            before: before.unwrap_or(style.spacing_before + 2.0),
            after: after.unwrap_or(style.spacing_after),
            anchors: Vec::new(),
        }
    }
    pub fn place(&self, page: &mut TextPage, left: f64, top: f64, width: f64) {
        page.place_anchors(&self.anchors, [left, top]);
        page.operations.push(PageOperation::Rule(Rule {
            role: RuleRole::Divider,
            start: [left, top + self.width / 2.0],
            end: [left + width, top + self.width / 2.0],
            width: self.width,
            color: self.color.clone(),
        }));
    }
}
