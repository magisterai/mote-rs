use super::{Error, check_attributes, xml};
use crate::{formatting::StyleResolver, model::Document};
use std::collections::BTreeSet;

pub(super) fn run_fonts(document: &Document, reference: &str) -> Result<String, Error> {
    if reference.is_empty() {
        return Ok(String::new());
    }
    let mut font = StyleResolver::new(document).resolve_font(reference);
    if !font.declared && reference.trim().eq_ignore_ascii_case("mono") {
        font.latin = Some("Courier New".into());
    }
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
            &["latin", "east_asian", "complex_script", "word_line_height"],
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
