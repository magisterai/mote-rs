//! Shared document anchor metadata and reference text selection.
use crate::model::{Content, Document, Element, ElementKind, Properties, Value};
use std::collections::BTreeMap;
#[derive(Default, Clone)]
pub(crate) struct Anchor {
    pub title: String,
    pub number: String,
    pub explicit_number: bool,
}
#[derive(Default)]
pub(crate) struct Catalog {
    pub anchors: BTreeMap<String, Anchor>,
    sequences: BTreeMap<String, u64>,
}
pub(crate) struct Resolution<'catalog> {
    pub target: String,
    pub display: String,
    pub text: String,
    pub anchor: Option<&'catalog Anchor>,
}
impl Catalog {
    pub fn build(document: &Document) -> Self {
        let mut catalog = Self::default();
        if document.sections.is_empty() {
            catalog.scan_flow(&document.content);
        } else {
            let mut stories = std::collections::BTreeSet::new();
            for section in &document.sections {
                if section.header.declaration_id == 0
                    || stories.insert(section.header.declaration_id)
                {
                    catalog.scan_flow(&section.header.content);
                }
                catalog.scan_flow(&section.content);
                if section.footer.declaration_id == 0
                    || stories.insert(section.footer.declaration_id)
                {
                    catalog.scan_flow(&section.footer.content);
                }
            }
        }
        catalog
    }
    fn register(&mut self, element: &Element) -> Option<String> {
        let name = crate::properties::text(&element.attributes, "name")
            .trim()
            .to_owned();
        if name.is_empty() || self.anchors.contains_key(&name) {
            return None;
        }
        let mut title = crate::properties::text(&element.attributes, "title")
            .trim()
            .to_owned();
        if title.is_empty() && element.kind == ElementKind::Bookmark {
            title = text(&element.children).trim().to_owned();
        }
        let number = crate::properties::text(&element.attributes, "number")
            .trim()
            .to_owned();
        self.anchors.insert(
            name.clone(),
            Anchor {
                title,
                explicit_number: !number.is_empty(),
                number,
            },
        );
        Some(name)
    }
    fn scan_flow(&mut self, nodes: &[Content]) {
        let mut pending = Vec::new();
        for node in nodes {
            if let Some(labels) = standalone_labels(node) {
                pending.extend(labels.iter().filter_map(|label| self.register(label)));
                continue;
            }
            if matches!(node, Content::Text(text) if text.trim().is_empty()) {
                continue;
            }
            if let Content::Element(element) = node
                && element.kind == ElementKind::Figure
            {
                self.scan_figure(element, std::mem::take(&mut pending));
                continue;
            }
            for name in pending.drain(..) {
                if let Some(anchor) = self.anchors.get_mut(&name)
                    && anchor.title.is_empty()
                {
                    anchor.title = text(std::slice::from_ref(node)).trim().to_owned();
                }
            }
            self.scan_node(node);
        }
    }
    fn scan_node(&mut self, node: &Content) {
        if let Content::Element(element) = node {
            if element.kind == ElementKind::Figure {
                self.scan_figure(element, Vec::new());
                return;
            }
            if element.kind == ElementKind::Caption {
                let _ = crate::captions::Numbering::resolve(
                    "figure",
                    &element.attributes,
                    &Properties::new(),
                )
                .advance(&mut self.sequences);
            }
            if matches!(element.kind, ElementKind::Label | ElementKind::Bookmark) {
                self.register(element);
            }
            if element
                .children
                .iter()
                .any(|child| matches!(child, Content::Element(element) if element.kind.is_block()))
            {
                self.scan_flow(&element.children);
            } else {
                for child in &element.children {
                    self.scan_node(child);
                }
            }
        }
    }
    fn scan_figure(&mut self, figure: &Element, mut labels: Vec<String>) {
        let mut association = None;
        for child in &figure.children {
            if let Some(declarations) = standalone_labels(child) {
                labels.extend(declarations.iter().filter_map(|label| self.register(label)));
                continue;
            }
            if let Content::Element(caption) = child
                && caption.kind == ElementKind::Caption
            {
                let numbering = crate::captions::Numbering::resolve(
                    &crate::properties::text(&figure.attributes, "kind"),
                    &caption.attributes,
                    &figure.attributes,
                );
                let number = numbering
                    .advance(&mut self.sequences)
                    .ok()
                    .flatten()
                    .map(|number| {
                        format!("{} {}", numbering.label, number.formatted)
                            .trim()
                            .to_owned()
                    })
                    .unwrap_or_default();
                association.get_or_insert((text(&caption.children).trim().to_owned(), number));
                for child in &caption.children {
                    self.scan_node(child);
                }
            } else {
                self.scan_node(child);
            }
        }
        if let Some((title, number)) = association {
            for label in labels {
                if let Some(anchor) = self.anchors.get_mut(&label) {
                    if anchor.title.is_empty() {
                        anchor.title = title.clone();
                    }
                    if anchor.number.is_empty() {
                        anchor.number = number.clone();
                    }
                }
            }
        }
    }
    pub fn resolve<'catalog>(&'catalog self, element: &Element) -> Resolution<'catalog> {
        let target = target(&element.attributes);
        let anchor = self.anchors.get(&target);
        let mut display = crate::properties::text(&element.attributes, "display")
            .trim()
            .to_owned();
        if display.is_empty() || display == "auto" {
            display = crate::properties::text(&element.attributes, "kind")
                .trim()
                .to_owned();
        }
        if display.is_empty() {
            display = "auto".into();
        }
        let authored = text(&element.children);
        let fallback = crate::properties::text(&element.attributes, "fallback");
        let text = if display == "number" && anchor.is_some_and(|anchor| !anchor.number.is_empty())
        {
            anchor.unwrap().number.clone()
        } else if !fallback.is_empty() {
            fallback
        } else if !authored.is_empty() && !matches!(authored.as_str(), "title" | "text" | "auto") {
            authored
        } else if matches!(display.as_str(), "title" | "text" | "auto")
            && anchor.is_some_and(|anchor| !anchor.title.is_empty())
        {
            anchor.unwrap().title.clone()
        } else {
            target.clone()
        };
        Resolution {
            target,
            display,
            text,
            anchor,
        }
    }
}
pub(crate) fn target(attributes: &Properties) -> String {
    match attributes.get("target") {
        Some(Value::String(value)) => value.trim().into(),
        Some(Value::Object(fields)) => fields
            .get("label")
            .or_else(|| fields.get("name"))
            .map(Value::to_text)
            .unwrap_or_default()
            .trim()
            .into(),
        _ => String::new(),
    }
}
pub(crate) fn text(nodes: &[Content]) -> String {
    nodes
        .iter()
        .map(|node| match node {
            Content::Text(text) => text.clone(),
            Content::Element(element) => text(&element.children),
        })
        .collect()
}
pub(crate) fn standalone_labels(node: &Content) -> Option<Vec<Element>> {
    let Content::Element(element) = node else {
        return None;
    };
    if element.kind == ElementKind::Label {
        return Some(vec![element.clone()]);
    }
    if element.kind != ElementKind::Paragraph {
        return None;
    }
    let mut labels = Vec::new();
    for child in &element.children {
        match child {
            Content::Text(text) if text.trim().is_empty() => {}
            Content::Element(element) if element.kind == ElementKind::Label => {
                labels.push(element.clone())
            }
            _ => return None,
        }
    }
    (!labels.is_empty()).then_some(labels)
}
pub(crate) fn bookmark_name(input: &str) -> String {
    let normalized: String = input
        .chars()
        .enumerate()
        .map(|(index, character)| {
            if character.is_ascii_alphabetic()
                || character == '_'
                || index > 0 && character.is_ascii_digit()
            {
                character
            } else {
                '_'
            }
        })
        .collect();
    let normalized = normalized.trim_matches('_');
    if normalized.is_empty() {
        "bookmark".into()
    } else if normalized.starts_with(|character: char| character.is_ascii_digit()) {
        format!("b_{normalized}")
    } else {
        normalized.into()
    }
}
pub(crate) fn label_bookmark(prefix: &str, name: &str) -> String {
    use sha2::{Digest, Sha256};
    let name = name.trim();
    let hint = bookmark_name(name);
    let hint = &hint[..hint.len().min(15)];
    let hash = Sha256::digest(name.as_bytes());
    let suffix: String = hash[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{prefix}_{hint}_{suffix}")
}
