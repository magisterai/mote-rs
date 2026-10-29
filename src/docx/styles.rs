use super::{
    Error,
    builtin_styles::{self, normalize_name},
    check_attributes, configuration, paragraph_properties,
    run_properties::{self, RunFormatting},
    values, xml,
};
use crate::{
    formatting::{ResolutionIssue, StyleResolver},
    model::{Document, Properties, StyleDeclaration, StyleKind, Value},
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Styles<'document> {
    pub document: &'document Document,
    identifiers: BTreeMap<String, String>,
    builtin_needed: BTreeSet<String>,
}

impl<'document> Styles<'document> {
    pub fn new(document: &'document Document) -> Result<Self, Error> {
        let mut identifiers = BTreeMap::new();
        let mut used = BTreeSet::new();
        for name in document.styles.keys() {
            let base: String = name.chars().filter(char::is_ascii_alphanumeric).collect();
            let base = if base.is_empty() {
                "Style".into()
            } else {
                base
            };
            let mut identifier = base.clone();
            let mut suffix = 2;
            while !used.insert(identifier.clone()) {
                identifier = format!("{base}{suffix}");
                suffix += 1;
            }
            identifiers.insert(name.clone(), identifier);
        }
        for &builtin in builtin_styles::NAMES {
            if !identifiers
                .keys()
                .any(|name| normalize_name(name) == normalize_name(builtin))
            {
                if !used.insert(builtin.into()) {
                    return Err(Error::InvalidValue(format!(
                        "style identifier collides with {builtin}"
                    )));
                }
                identifiers.insert(builtin.into(), builtin.into());
            }
        }
        let result = Self {
            document,
            identifiers,
            builtin_needed: builtin_styles::needed(document)?,
        };
        for declaration in document.styles.values() {
            result.resolved_run(&declaration.name)?;
            for relationship in ["based_on", "next", "link"] {
                if let Some(name) = declaration
                    .properties
                    .get(relationship)
                    .map(Value::to_text)
                    .filter(|name| !name.is_empty())
                {
                    result.identifier(&name)?;
                }
            }
        }
        Ok(result)
    }

    pub fn identifier(&self, name: &str) -> Result<&str, Error> {
        if let Some(identifier) = self.identifiers.get(name) {
            return Ok(identifier);
        }
        let normalized = normalize_name(name);
        if !normalized.is_empty()
            && let Some((_, identifier)) = self
                .identifiers
                .iter()
                .find(|(candidate, _)| normalize_name(candidate) == normalized)
        {
            return Ok(identifier);
        }
        Err(Error::Unsupported(format!(
            "unresolved style reference {name:?}"
        )))
    }

    pub fn resolved_run(&self, name: &str) -> Result<Properties, Error> {
        self.identifier(name)?;
        let resolved = StyleResolver::new(self.document).resolve_style(name);
        for issue in &resolved.issues {
            match issue {
                ResolutionIssue::MissingStyle(missing)
                    if builtin_styles::canonical_name(missing).is_some() => {}
                _ => {
                    return Err(Error::Unsupported(format!(
                        "style resolution issue {issue:?}"
                    )));
                }
            }
        }
        if !self.declared(name)
            && matches!(
                builtin_styles::canonical_name(name),
                Some("Code" | "Hyperlink")
            )
        {
            return Ok(builtin_styles::run_properties(name));
        }
        Ok(resolved.run)
    }

    pub fn write(&self) -> Result<String, Error> {
        let mut content = configuration::document_defaults(self.document)?;
        if !self.declared("Normal") {
            content.push_str("<w:style w:type=\"paragraph\" w:default=\"1\" w:styleId=\"Normal\"><w:name w:val=\"Normal\"/><w:qFormat/></w:style>");
        }
        for level in 1..=6 {
            let name = format!("Heading{level}");
            if self.builtin_needed.contains(&name) && !self.declared(&name) {
                content.push_str(&builtin_styles::write_heading(
                    level,
                    self.identifier("Normal")?,
                ));
            }
        }
        if self.builtin_needed.contains("Hyperlink") && !self.declared("Hyperlink") {
            content.push_str("<w:style w:type=\"character\" w:styleId=\"Hyperlink\"><w:name w:val=\"Hyperlink\"/><w:uiPriority w:val=\"99\"/><w:unhideWhenUsed/><w:rPr><w:color w:val=\"0563C1\"/><w:u w:val=\"single\"/></w:rPr></w:style>");
        }
        if self.builtin_needed.contains("Caption") && !self.declared("Caption") {
            content.push_str(&format!("<w:style w:type=\"paragraph\" w:styleId=\"Caption\"><w:name w:val=\"Caption\"/><w:basedOn w:val=\"{}\"/><w:pPr><w:spacing w:after=\"160\" w:line=\"276\" w:lineRule=\"auto\"/></w:pPr><w:rPr><w:i/><w:iCs/><w:color w:val=\"666666\"/></w:rPr></w:style>", self.identifier("Normal")?));
        }
        if self.builtin_needed.contains("BlockQuote") && !self.declared("BlockQuote") {
            let formatting = RunFormatting {
                properties: builtin_styles::run_properties("BlockQuote"),
                ..RunFormatting::default()
            };
            content.push_str(&format!("<w:style w:type=\"paragraph\" w:styleId=\"BlockQuote\"><w:name w:val=\"Block Quote\"/><w:basedOn w:val=\"{}\"/><w:pPr><w:spacing w:before=\"120\" w:after=\"120\" w:line=\"276\" w:lineRule=\"auto\"/><w:ind w:left=\"720\"/></w:pPr><w:rPr>{}</w:rPr></w:style>", self.identifier("Normal")?, formatting.write_declaration(self)?));
        }
        if self.builtin_needed.contains("Code") && !self.declared("Code") {
            let formatting = RunFormatting {
                properties: self.resolved_run("Code")?,
                ..RunFormatting::default()
            };
            content.push_str(&format!("<w:style w:type=\"character\" w:styleId=\"Code\"><w:name w:val=\"Code\"/><w:rPr>{}</w:rPr></w:style>", formatting.write_declaration(self)?));
        }
        for declaration in self.document.styles.values() {
            content.push_str(&self.write_declaration(declaration)?);
        }
        Ok(xml::word_part("styles", &content))
    }

    fn declared(&self, name: &str) -> bool {
        self.document
            .styles
            .keys()
            .any(|candidate| normalize_name(candidate) == normalize_name(name))
    }

    fn write_declaration(&self, declaration: &StyleDeclaration) -> Result<String, Error> {
        if declaration.kind == StyleKind::Table {
            return Err(Error::Unsupported("table styles".into()));
        }
        let mut accepted = paragraph_properties::ATTRIBUTES.to_vec();
        accepted.extend_from_slice(run_properties::ATTRIBUTES);
        accepted.extend_from_slice(&[
            "type",
            "based_on",
            "next",
            "link",
            "paragraph",
            "run",
            "background",
            "first_line_indent",
            "hanging_indent",
        ]);
        check_attributes(&declaration.properties, &accepted, "style declaration")?;
        let default = if declaration.name == "Normal" && declaration.kind == StyleKind::Paragraph {
            " w:default=\"1\""
        } else {
            ""
        };
        let mut result = format!(
            "<w:style w:type=\"{}\"{default} w:styleId=\"{}\"><w:name w:val=\"{}\"/>",
            declaration.kind.as_text(),
            xml::escape(self.identifier(&declaration.name)?)?,
            xml::escape(&declaration.name)?
        );
        for (name, word_name) in [("based_on", "basedOn"), ("next", "next"), ("link", "link")] {
            if let Some(reference) = declaration
                .properties
                .get(name)
                .map(Value::to_text)
                .filter(|name| !name.is_empty())
            {
                result.push_str(&format!(
                    "<w:{word_name} w:val=\"{}\"/>",
                    xml::escape(self.identifier(&reference)?)?
                ));
            }
        }
        if declaration.kind == StyleKind::Paragraph {
            result.push_str("<w:qFormat/>");
            let mut paragraph_fields = paragraph_properties::ATTRIBUTES.to_vec();
            paragraph_fields.extend_from_slice(&[
                "background",
                "first_line_indent",
                "hanging_indent",
            ]);
            let properties = declaration_group(declaration, "paragraph", &paragraph_fields)?;
            let paragraph = paragraph_properties::write_declaration(&properties)?;
            if !paragraph.is_empty() {
                result.push_str(&format!("<w:pPr>{paragraph}</w:pPr>"));
            }
        } else if declaration.properties.contains_key("paragraph") {
            return Err(Error::Unsupported(
                "paragraph properties on a character style".into(),
            ));
        }
        let mut run_fields = run_properties::ATTRIBUTES.to_vec();
        run_fields.extend_from_slice(&["fill", "background"]);
        let run = declaration_group(declaration, "run", &run_fields)?;
        let formatting = RunFormatting {
            properties: run,
            ..RunFormatting::default()
        };
        let properties = formatting.write_declaration(self)?;
        if !properties.is_empty() {
            result.push_str(&format!("<w:rPr>{properties}</w:rPr>"));
        }
        result.push_str("</w:style>");
        Ok(result)
    }
}

fn declaration_group(
    declaration: &StyleDeclaration,
    group: &str,
    fields: &[&str],
) -> Result<Properties, Error> {
    let mut result: Properties = declaration
        .properties
        .iter()
        .filter(|(name, _)| fields.contains(&name.as_str()))
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    if let Some(nested) = declaration.properties.get(group) {
        result.extend(values::object(nested)?.clone());
    }
    Ok(result)
}
