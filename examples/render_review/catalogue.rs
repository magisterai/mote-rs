//! Review inputs and expectations, kept separate from rendering and publication.
use super::ReviewResult;
use mote::render::LayoutMode;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Application {
    Word,
    PowerPoint,
}

impl Application {
    pub fn directory(self) -> &'static str {
        match self {
            Self::Word => "word",
            Self::PowerPoint => "powerpoint",
        }
    }
    pub fn package_format(self) -> &'static str {
        match self {
            Self::Word => "docx",
            Self::PowerPoint => "pptx",
        }
    }
    pub fn layout(self) -> LayoutMode {
        match self {
            Self::Word => LayoutMode::Document,
            Self::PowerPoint => LayoutMode::Presentation,
        }
    }
    pub fn layout_name(self) -> &'static str {
        match self {
            Self::Word => "document",
            Self::PowerPoint => "presentation",
        }
    }
    pub fn review_status(self) -> &'static str {
        match self {
            Self::Word => "word_status",
            Self::PowerPoint => "powerpoint_reference_status",
        }
    }
}

pub(super) struct ReviewCase {
    pub identifier: String,
    pub application: Application,
    pub name: String,
    pub source: String,
    pub group: String,
    pub enable_lua: bool,
    pub page_count: usize,
    pub required_fonts: Vec<String>,
}

pub(super) const GROUPS: &[&str] = &["fundamentals", "text", "tables", "flow", "media"];

pub(super) fn load() -> ReviewResult<Vec<ReviewCase>> {
    parse(include_str!("../../tests/cases/catalogue.json"))
}

fn string<'value>(value: &'value Value, field: &str) -> ReviewResult<&'value str> {
    value[field]
        .as_str()
        .filter(|text| !text.is_empty())
        .ok_or_else(|| format!("catalogue requires a nonempty {field}").into())
}

fn simple_name(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-')
        })
}

fn parse(source: &str) -> ReviewResult<Vec<ReviewCase>> {
    let value: Value = serde_json::from_str(source)?;
    let entries = value
        .as_array()
        .filter(|entries| !entries.is_empty())
        .ok_or("catalogue must contain cases")?;
    let mut identifiers = BTreeSet::new();
    let mut destinations = BTreeSet::new();
    let mut cases = Vec::new();
    for entry in entries {
        let identifier = string(entry, "identifier")?;
        let name = string(entry, "name")?;
        if !simple_name(identifier) || !simple_name(name) || !identifiers.insert(identifier) {
            return Err(format!("invalid or duplicate case identifier: {identifier}").into());
        }
        let application = match string(entry, "application")? {
            "word" => Application::Word,
            "powerpoint" => Application::PowerPoint,
            other => return Err(format!("unknown review application: {other}").into()),
        };
        if !destinations.insert((application.directory(), name)) {
            return Err(format!("duplicate review destination: {name}").into());
        }
        let source = string(entry, "source")?;
        if !Path::new(source)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
            || !source.ends_with(".mote")
        {
            return Err(format!("invalid case source: {source}").into());
        }
        let group = string(entry, "group")?;
        if !GROUPS.contains(&group) {
            return Err(format!("unknown review group: {group}").into());
        }
        let enable_lua = match string(entry, "evaluation")? {
            "static" => false,
            "lua" => true,
            other => return Err(format!("unknown evaluation mode: {other}").into()),
        };
        let page_count = entry["pages"]
            .as_u64()
            .filter(|count| *count > 0)
            .and_then(|count| usize::try_from(count).ok())
            .ok_or("case requires a positive page count")?;
        let required_fonts = entry["required_fonts"]
            .as_array()
            .ok_or("case requires a font list")?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or("font paths must be strings")
            })
            .collect::<Result<Vec<_>, _>>()?;
        cases.push(ReviewCase {
            identifier: identifier.into(),
            application,
            name: name.into(),
            source: source.into(),
            group: group.into(),
            enable_lua,
            page_count,
            required_fonts,
        });
    }
    Ok(cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_has_unique_cases_existing_sources_and_explicit_evaluation() {
        let cases = load().unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cases");
        assert_eq!(
            cases
                .iter()
                .filter(|case| case.application == Application::Word)
                .count(),
            63
        );
        assert_eq!(
            cases
                .iter()
                .filter(|case| case.application == Application::PowerPoint)
                .count(),
            31
        );
        assert_eq!(cases.iter().filter(|case| case.enable_lua).count(), 3);
        for case in cases {
            assert!(root.join(&case.source).is_file(), "{}", case.source);
        }
    }

    #[test]
    fn catalogue_rejects_path_escape_duplicate_destinations_and_unknown_modes() {
        let original: Value =
            serde_json::from_str(include_str!("../../tests/cases/catalogue.json")).unwrap();
        for (field, replacement) in [
            ("source", "../outside.mote"),
            ("name", "../../outside"),
            ("evaluation", "automatic"),
            ("group", "unknown"),
        ] {
            let mut entry = original[0].clone();
            entry[field] = replacement.into();
            assert!(parse(&serde_json::json!([entry]).to_string()).is_err());
        }
        let mut duplicate = original[0].clone();
        duplicate["identifier"] = "another-case".into();
        assert!(parse(&serde_json::json!([original[0], duplicate]).to_string()).is_err());
    }
}
