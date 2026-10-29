use super::{Error, headings};
use crate::model::{Content, Document, ElementKind, Properties, Unit, Value};
use std::collections::BTreeSet;

pub(super) const NAMES: &[&str] = &[
    "Normal",
    "Heading1",
    "Heading2",
    "Heading3",
    "Heading4",
    "Heading5",
    "Heading6",
    "BlockQuote",
    "Code",
    "Hyperlink",
    "Caption",
    "Header",
    "Footer",
];

pub(super) fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

pub(super) fn canonical_name(name: &str) -> Option<&'static str> {
    let normalized = normalize_name(name);
    NAMES
        .iter()
        .copied()
        .find(|candidate| normalize_name(candidate) == normalized)
}

pub(super) fn needed(document: &Document) -> Result<BTreeSet<String>, Error> {
    let mut names = BTreeSet::from(["Normal".into()]);
    collect(&document.content, &mut names)?;
    for section in &document.sections {
        for (style, story) in [("Header", &section.header), ("Footer", &section.footer)] {
            if !story.content.is_empty() {
                names.insert(style.into());
            }
            collect(&story.content, &mut names)?;
        }
    }
    for declaration in document.styles.values() {
        for relationship in ["based_on", "next", "link"] {
            if let Some(name) = declaration
                .properties
                .get(relationship)
                .and_then(|value| canonical_name(&value.to_text()))
            {
                names.insert(name.into());
            }
        }
    }
    Ok(names)
}

fn collect(content: &[Content], names: &mut BTreeSet<String>) -> Result<(), Error> {
    for child in content {
        let Content::Element(element) = child else {
            continue;
        };
        if element.kind == ElementKind::Image {
            names.insert("Caption".into());
        }
        if element.kind == ElementKind::Link {
            names.insert("Hyperlink".into());
        }
        let style = element
            .attributes
            .get("style")
            .map(Value::to_text)
            .unwrap_or_default();
        if let Some(name) = canonical_name(&style) {
            names.insert(name.into());
        }
        if style.is_empty() {
            match element.kind {
                ElementKind::Code => {
                    names.insert("Code".into());
                }
                ElementKind::Blockquote => {
                    names.insert("BlockQuote".into());
                }
                ElementKind::Heading => {
                    let level = headings::level(element)?;
                    if level <= 6 {
                        names.insert(format!("Heading{level}"));
                    }
                }
                _ => {}
            }
        }
        collect(&element.children, names)?;
    }
    Ok(())
}

pub(super) fn run_properties(name: &str) -> Properties {
    let mut properties = Properties::new();
    match canonical_name(name) {
        Some("BlockQuote") => {
            properties.insert("font".into(), Value::String("Helvetica".into()));
            properties.insert("italic".into(), Value::Boolean(true));
            properties.insert(
                "size".into(),
                Value::Length {
                    value: 11.0,
                    unit: Unit::Point,
                },
            );
        }
        Some("Hyperlink") => {
            properties.insert("color".into(), Value::String("0563C1".into()));
            properties.insert("underline".into(), Value::Boolean(true));
        }
        Some("Code") => {
            properties.insert("font".into(), Value::String("mono".into()));
            properties.insert(
                "size".into(),
                Value::Length {
                    value: 10.0,
                    unit: Unit::Point,
                },
            );
        }
        Some(name) if name.starts_with("Heading") => {
            let level: usize = name[7..].parse().unwrap();
            properties.insert("bold".into(), Value::Boolean(true));
            properties.insert(
                "size".into(),
                Value::Length {
                    value: [16.0, 14.0, 12.0, 11.0, 11.0, 11.0][level - 1],
                    unit: Unit::Point,
                },
            );
        }
        _ => {}
    }
    properties
}

pub(super) fn write_heading(level: u8, normal_identifier: &str) -> String {
    let size = [32, 28, 24, 22, 22, 22][usize::from(level - 1)];
    let outline = level - 1;
    format!(
        "<w:style w:type=\"paragraph\" w:styleId=\"Heading{level}\"><w:name w:val=\"Heading {level}\"/><w:basedOn w:val=\"{normal_identifier}\"/><w:next w:val=\"{normal_identifier}\"/><w:qFormat/><w:pPr><w:keepNext/><w:spacing w:before=\"160\" w:after=\"160\" w:line=\"240\" w:lineRule=\"auto\"/><w:outlineLvl w:val=\"{outline}\"/></w:pPr><w:rPr><w:b/><w:bCs/><w:sz w:val=\"{size}\"/><w:szCs w:val=\"{size}\"/></w:rPr></w:style>"
    )
}
