//! Declared-style resolution, independent of output defaults and font measurement.

use crate::model::{Document, Element, ElementKind, Properties, StyleDeclaration, Value};
use std::collections::BTreeSet;

const RUN_PROPERTIES: &[&str] = &[
    "style",
    "font",
    "size",
    "color",
    "fill",
    "background",
    "highlight",
    "bold",
    "italic",
    "underline",
    "small_caps",
    "strike",
    "double_strike",
];
const PARAGRAPH_PROPERTIES: &[&str] = &[
    "alignment",
    "spacing",
    "spacing_before",
    "spacing_after",
    "line_spacing",
    "indent",
    "border",
    "fill",
    "background",
    "tabs",
    "keep_with_next",
    "keep_together",
    "page_break_before",
    "widow_control",
    "outline_level",
    "first_line_indent",
    "hanging_indent",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionIssue {
    MissingStyle(String),
    CyclicStyle(String),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResolvedStyle {
    pub paragraph: Properties,
    pub run: Properties,
    pub font: Option<ResolvedFont>,
    pub issues: Vec<ResolutionIssue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFont {
    pub reference: String,
    pub declared: bool,
    pub latin: Option<String>,
    pub east_asian: Option<String>,
    pub complex_script: Option<String>,
    pub word_line_height: Option<f64>,
}

pub struct StyleResolver<'document> {
    document: &'document Document,
}

impl<'document> StyleResolver<'document> {
    pub fn new(document: &'document Document) -> Self {
        Self { document }
    }

    /// Resolve the named declaration only, retaining discovered properties when
    /// a parent is missing or repeated. No implicit output style is selected.
    pub fn resolve_style(&self, name: &str) -> ResolvedStyle {
        let mut result = ResolvedStyle::default();
        let mut chain = Vec::new();
        let mut seen = BTreeSet::new();
        let mut current_name = name;
        loop {
            let Some((identity, declaration)) = self.find_style(current_name) else {
                result
                    .issues
                    .push(ResolutionIssue::MissingStyle(current_name.into()));
                break;
            };
            if !seen.insert(identity) {
                result
                    .issues
                    .push(ResolutionIssue::CyclicStyle(identity.into()));
                break;
            }
            chain.push(declaration);
            match string_property(&declaration.properties, "based_on") {
                Some(parent) if !parent.is_empty() => current_name = parent,
                _ => break,
            }
        }
        for declaration in chain.into_iter().rev() {
            merge_paragraph_properties(
                &mut result.paragraph,
                &declaration_properties(declaration, "paragraph", PARAGRAPH_PROPERTIES),
            );
            result
                .run
                .extend(declaration_properties(declaration, "run", RUN_PROPERTIES));
        }
        self.attach_font(&mut result);
        result
    }

    /// Apply direct paragraph attributes after its explicit named style.
    /// Unnamed styles and output-specific defaults remain unspecified.
    pub fn resolve_paragraph(&self, attributes: &Properties) -> ResolvedStyle {
        let mut result = attributes
            .get("style")
            .map(Value::to_text)
            .filter(|name| !name.is_empty())
            .map_or_else(ResolvedStyle::default, |name| self.resolve_style(&name));
        merge_paragraph_properties(
            &mut result.paragraph,
            &select_properties(attributes, PARAGRAPH_PROPERTIES),
        );
        self.attach_font(&mut result);
        result
    }

    /// Apply inline intent, its named style, then direct formatting to inherited
    /// run properties. Callers pass a parent's run map; sibling maps stay intact.
    pub fn resolve_inline(&self, element: &Element, inherited: &Properties) -> ResolvedStyle {
        let mut result = ResolvedStyle {
            run: inherited.clone(),
            ..ResolvedStyle::default()
        };
        match element.kind {
            ElementKind::Strong => {
                result.run.insert("bold".into(), Value::Boolean(true));
            }
            ElementKind::Emphasis => {
                result.run.insert("italic".into(), Value::Boolean(true));
            }
            ElementKind::Underline => {
                result.run.insert("underline".into(), Value::Boolean(true));
                result.run.remove("underline_style");
                result.run.remove("underline_color");
                for (attribute, property) in
                    [("style", "underline_style"), ("color", "underline_color")]
                {
                    if let Some(value) = element.attributes.get(attribute) {
                        result.run.insert(property.into(), value.clone());
                    }
                }
            }
            ElementKind::Strike => {
                result.run.insert("strike".into(), Value::Boolean(true));
                result.run.insert(
                    "double_strike".into(),
                    Value::Boolean(string_property(&element.attributes, "kind") == Some("double")),
                );
            }
            ElementKind::SmallCaps | ElementKind::Subscript | ElementKind::Superscript => {
                result
                    .run
                    .insert(element.kind.as_text().into(), Value::Boolean(true));
            }
            _ => {}
        }
        if element.kind != ElementKind::Underline
            && let Some(name) = element
                .attributes
                .get("style")
                .map(Value::to_text)
                .filter(|name| !name.is_empty())
        {
            let named = self.resolve_style(&name);
            result.run.extend(named.run);
            result.issues = named.issues;
        }
        let mut direct = select_properties(&element.attributes, RUN_PROPERTIES);
        direct.remove("style");
        result.run.extend(direct);
        self.attach_font(&mut result);
        result
    }

    /// Resolve declared script mappings without inspecting installed fonts.
    pub fn resolve_font(&self, name: &str) -> ResolvedFont {
        let Some(declaration) = self.document.fonts.get(name) else {
            return ResolvedFont {
                reference: name.into(),
                declared: false,
                latin: Some(name.into()),
                east_asian: None,
                complex_script: None,
                word_line_height: None,
            };
        };
        ResolvedFont {
            reference: name.into(),
            declared: true,
            latin: string_property(&declaration.properties, "latin").map(str::to_owned),
            east_asian: string_property(&declaration.properties, "east_asian").map(str::to_owned),
            complex_script: string_property(&declaration.properties, "complex_script")
                .map(str::to_owned),
            word_line_height: match declaration.properties.get("word_line_height") {
                Some(Value::Number(number)) => Some(*number),
                _ => None,
            },
        }
    }

    fn attach_font(&self, style: &mut ResolvedStyle) {
        style.font = style
            .run
            .get("font")
            .map(Value::to_text)
            .filter(|name| !name.is_empty())
            .map(|name| self.resolve_font(&name));
    }

    fn find_style(&self, name: &str) -> Option<(&'document str, &'document StyleDeclaration)> {
        if let Some((identity, declaration)) = self.document.styles.get_key_value(name) {
            return Some((identity, declaration));
        }
        let normalized = normalize_style_name(name);
        if normalized.is_empty() {
            return None;
        }
        self.document
            .styles
            .iter()
            .find(|(identity, _)| normalize_style_name(identity) == normalized)
            .map(|(identity, declaration)| (identity.as_str(), declaration))
    }
}

fn normalize_style_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

fn string_property<'properties>(
    properties: &'properties Properties,
    name: &str,
) -> Option<&'properties str> {
    match properties.get(name) {
        Some(Value::String(text)) => Some(text),
        _ => None,
    }
}

fn select_properties(properties: &Properties, names: &[&str]) -> Properties {
    properties
        .iter()
        .filter(|(name, _)| names.contains(&name.as_str()))
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect()
}

fn declaration_properties(
    declaration: &StyleDeclaration,
    group: &str,
    names: &[&str],
) -> Properties {
    let mut properties = select_properties(&declaration.properties, names);
    if let Some(Value::Object(nested)) = declaration.properties.get(group) {
        properties.extend(nested.clone());
    }
    properties
}

fn merge_nested_properties(target: &mut Properties, source: &Properties) {
    for (name, value) in source {
        if let Value::Object(source_fields) = value
            && let Some(Value::Object(target_fields)) = target.get_mut(name)
        {
            merge_nested_properties(target_fields, source_fields);
        } else {
            target.insert(name.clone(), value.clone());
        }
    }
}

fn merge_paragraph_properties(target: &mut Properties, source: &Properties) {
    for (flat, aliases) in [
        ("spacing_before", ["before", "spacing_before"]),
        ("spacing_after", ["after", "spacing_after"]),
        ("line_spacing", ["line", "line_spacing"]),
    ] {
        if source.contains_key(flat)
            && let Some(Value::Object(spacing)) = target.get_mut("spacing")
        {
            for alias in aliases {
                spacing.remove(alias);
            }
            if spacing.is_empty() {
                target.remove("spacing");
            }
        }
        if let Some(Value::Object(spacing)) = source.get("spacing")
            && aliases.iter().any(|alias| spacing.contains_key(*alias))
        {
            target.remove(flat);
        }
    }
    merge_nested_properties(target, source);
}
