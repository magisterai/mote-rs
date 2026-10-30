//! Named slot validation and materialization shared by all presentation outputs.
mod builtins;
use crate::{
    model::{Content, Document, Element, ElementKind, Properties, Value},
    properties::text,
};
use std::collections::BTreeMap;

#[derive(Clone)]
struct Slot {
    name: String,
    kind: String,
    required: bool,
    properties: Properties,
}
#[derive(Clone)]
struct Layout {
    properties: Properties,
    slots: Vec<Slot>,
}
pub(crate) struct Layouts {
    definitions: BTreeMap<String, Layout>,
}
impl Layouts {
    pub(crate) fn builtin(dimensions: [i64; 2]) -> Self {
        Self {
            definitions: builtins::layouts(dimensions),
        }
    }
    pub(crate) fn apply(&self, slide: &Element) -> Result<Option<Element>, String> {
        let name = text(&slide.attributes, "layout");
        if name.is_empty() {
            return Ok(None);
        }
        let layout = self
            .definitions
            .get(&name)
            .ok_or_else(|| format!("unknown presentation layout {name:?}"))?;
        let mut supplied = BTreeMap::new();
        let mut ordinary = Vec::new();
        for node in slide
            .children
            .iter()
            .filter(|node| super::automatic::meaningful(node))
        {
            let Content::Element(element) = node else {
                return Err(format!(
                    "slide using layout {name:?} contains non-positioned content"
                ));
            };
            if !matches!(
                element.kind,
                ElementKind::Box | ElementKind::Shape | ElementKind::Line
            ) {
                return Err(format!(
                    "slide using layout {name:?} contains non-positioned content"
                ));
            }
            let slot_name = text(&element.attributes, "slot");
            if element.kind == ElementKind::Box && !slot_name.is_empty() {
                let slot = layout
                    .slots
                    .iter()
                    .find(|slot| slot.name == slot_name)
                    .ok_or_else(|| format!("layout {name:?} has no slot named {slot_name:?}"))?;
                let actual = content_kind(element);
                if slot.kind != "content" && slot.kind != actual {
                    return Err(format!(
                        "layout slot {slot_name:?} requires {} content, got {actual} content",
                        slot.kind
                    ));
                }
                if supplied.insert(slot_name.clone(), element).is_some() {
                    return Err(format!(
                        "slide supplies layout slot {slot_name:?} more than once"
                    ));
                }
            } else {
                ordinary.push(node.clone());
            }
        }
        let mut children = Vec::new();
        for slot in &layout.slots {
            match supplied.get(&slot.name) {
                Some(authored) => {
                    let mut element = (*authored).clone();
                    element.attributes = merge(&slot.properties, &element.attributes);
                    element.attributes.remove("slot");
                    children.push(Content::Element(element));
                }
                None if slot.required => {
                    return Err(format!(
                        "slide using layout {name:?} is missing required slot {:?}",
                        slot.name
                    ));
                }
                None => {}
            }
        }
        children.extend(ordinary);
        let mut attributes = merge(&layout.properties, &slide.attributes);
        // A slide background is an intentional whole-value override.
        if let Some(background) = slide.attributes.get("background") {
            attributes.insert("background".into(), background.clone());
        }
        Ok(Some(Element {
            kind: ElementKind::Slide,
            attributes,
            children,
        }))
    }
    fn validate_content(&self, nodes: &[Content], direct_slots: bool) -> Result<(), String> {
        for node in nodes {
            let Content::Element(element) = node else {
                continue;
            };
            if element.kind == ElementKind::Box {
                if !text(&element.attributes, "slot").is_empty() && !direct_slots {
                    return Err(
                        "box slot assignments require a direct child of a layout-backed slide"
                            .into(),
                    );
                }
                if ["kind", "required", "remove"]
                    .iter()
                    .any(|name| element.attributes.contains_key(*name))
                {
                    return Err(
                        "box kind, required and remove are only valid in layout declarations"
                            .into(),
                    );
                }
            }
            let named_slide = element.kind == ElementKind::Slide
                && !text(&element.attributes, "layout").is_empty();
            if named_slide {
                self.apply(element)?;
            }
            self.validate_content(&element.children, named_slide)?;
        }
        Ok(())
    }
}
pub(crate) fn validate(document: &Document) -> Result<(), String> {
    let layouts = Layouts::builtin([12192000, 6858000]);
    layouts.validate_content(&document.content, false)?;
    for section in &document.sections {
        layouts.validate_content(&section.header.content, false)?;
        layouts.validate_content(&section.footer.content, false)?;
    }
    Ok(())
}
fn content_kind(element: &Element) -> &str {
    let children: Vec<_> = element
        .children
        .iter()
        .filter(|node| super::automatic::meaningful(node))
        .collect();
    if let [Content::Element(child)] = children.as_slice() {
        match child.kind {
            ElementKind::Image => return "image",
            ElementKind::Table => return "table",
            ElementKind::Chart => return "chart",
            _ => {}
        }
    }
    "text"
}
fn merge(base: &Properties, overlay: &Properties) -> Properties {
    let mut result = base.clone();
    for (name, value) in overlay {
        let value = match (result.get(name), value) {
            (Some(Value::Object(base)), Value::Object(overlay)) => {
                Value::Object(merge(base, overlay))
            }
            _ => value.clone(),
        };
        result.insert(name.clone(), value);
    }
    result
}
