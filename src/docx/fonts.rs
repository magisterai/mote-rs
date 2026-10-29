use super::{Error, check_attributes, xml};
use crate::{
    formatting::{ResolvedFont, StyleResolver},
    model::{Document, Value},
};
use std::collections::BTreeSet;

pub(super) fn run_fonts(document: &Document, reference: &str) -> Result<String, Error> {
    if reference.is_empty() {
        return Ok(String::new());
    }
    write_run_fonts(&resolve_run_fonts(document, reference))
}

fn resolve_run_fonts(document: &Document, reference: &str) -> ResolvedFont {
    let mut font = StyleResolver::new(document).resolve_font(reference);
    if !font.declared && reference.trim().eq_ignore_ascii_case("mono") {
        font.latin = Some("Courier New".into());
    }
    font
}

pub(super) fn default_run_fonts(document: &Document) -> Result<String, Error> {
    let reference = match document.defaults.get("default_font") {
        None => "Helvetica",
        Some(Value::String(reference)) if reference.is_empty() => "Helvetica",
        Some(Value::String(reference)) => reference,
        _ => {
            return Err(Error::InvalidValue(
                "default_font must be a string for DOCX output".into(),
            ));
        }
    };
    let mut font = resolve_run_fonts(document, reference);
    if font.latin.as_ref().is_none_or(String::is_empty) {
        font.latin = Some("Helvetica".into());
    }
    write_run_fonts(&font)
}

fn write_run_fonts(font: &ResolvedFont) -> Result<String, Error> {
    let mut attributes = String::new();
    for (name, typeface) in [
        ("ascii", &font.latin),
        ("hAnsi", &font.latin),
        ("eastAsia", &font.east_asian),
        ("cs", &font.complex_script),
    ] {
        if let Some(typeface) = typeface {
            attributes.push_str(&format!(" w:{name}=\"{}\"", xml::escape(typeface)?));
        }
    }
    Ok(if attributes.is_empty() {
        String::new()
    } else {
        format!("<w:rFonts{attributes}/>")
    })
}

pub(super) fn write_table(document: &Document) -> Result<String, Error> {
    let mut names: BTreeSet<String> = ["Aptos", "Aptos Display", "Courier New", "Helvetica"]
        .map(str::to_owned)
        .into_iter()
        .collect();
    for declaration in document.fonts.values() {
        check_attributes(
            &declaration.properties,
            &[
                "latin",
                "east_asian",
                "complex_script",
                "word_line_height",
                "files",
            ],
            "font declaration",
        )?;
        let font = StyleResolver::new(document).resolve_font(&declaration.name);
        for typeface in [font.latin, font.east_asian, font.complex_script]
            .into_iter()
            .flatten()
        {
            names.insert(typeface);
        }
    }
    let mut content = String::new();
    for name in names {
        content.push_str(&format!("<w:font w:name=\"{}\"/>", xml::escape(&name)?));
    }
    Ok(xml::word_part("fonts", &content))
}
