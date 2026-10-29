use super::{
    Error, check_attributes, run_properties::RunFormatting, styles::Styles, whitespace, xml,
};
use crate::model::{Content, ElementKind};

pub(super) fn write_inlines(
    children: &[Content],
    styles: &Styles<'_>,
    resources: &mut super::resources::Resources,
) -> Result<String, Error> {
    validate_controls(children)?;
    let formatting = resources.active_text.clone();
    write_formatted(
        &whitespace::trim_line_edges(children),
        &formatting,
        styles,
        resources,
    )
}

fn validate_controls(children: &[Content]) -> Result<(), Error> {
    for child in children {
        if let Content::Element(element) = child {
            if matches!(element.kind, ElementKind::LineBreak | ElementKind::Tab) {
                check_attributes(&element.attributes, &[], element.kind.as_text())?;
                if !element.children.is_empty() {
                    return Err(Error::InvalidValue(format!(
                        "{} cannot contain children",
                        element.kind.as_text()
                    )));
                }
            }
            validate_controls(&element.children)?;
        }
    }
    Ok(())
}

fn write_formatted(
    children: &[Content],
    formatting: &RunFormatting,
    styles: &Styles<'_>,
    resources: &mut super::resources::Resources,
) -> Result<String, Error> {
    let mut result = String::new();
    for child in children {
        match child {
            Content::Text(text) => result.push_str(&write_text(text, formatting, styles)?),
            Content::Element(element)
                if matches!(element.kind, ElementKind::LineBreak | ElementKind::Tab) =>
            {
                result.push_str(if element.kind == ElementKind::Tab {
                    "<w:r><w:tab/></w:r>"
                } else {
                    "<w:r><w:br/></w:r>"
                });
            }
            Content::Element(element) if element.kind == ElementKind::PageNumber => {
                let instruction = if crate::properties::text(&element.attributes, "kind") == "total"
                {
                    "NUMPAGES"
                } else {
                    "PAGE"
                };
                let number_format = crate::properties::text(&element.attributes, "format");
                let format_switch = if number_format.is_empty() {
                    String::new()
                } else {
                    format!(
                        r" \* {}",
                        super::sections::field_number_format(&number_format)
                    )
                };
                let properties = formatting.write(styles)?;
                let properties = if properties.is_empty() {
                    properties
                } else {
                    format!("<w:rPr>{properties}</w:rPr>")
                };
                result.push_str(&format!(r#"<w:fldSimple w:instr=" {instruction}{format_switch} \* MERGEFORMAT "><w:r>{properties}<w:t>1</w:t></w:r></w:fldSimple>"#));
            }
            Content::Element(element) if element.kind == ElementKind::Image => {
                if let Some(image) = super::images::write_run(element, resources)? {
                    result.push_str(&image);
                } else {
                    let label = super::values::first_text(&element.attributes, &["alt", "src"])
                        .unwrap_or_default();
                    result.push_str(&write_text(
                        &format!("[Image: {label}]"),
                        formatting,
                        styles,
                    )?);
                }
            }
            Content::Element(element) if element.kind == ElementKind::Link => {
                check_attributes(
                    &element.attributes,
                    &["url", "title", "tooltip", "target"],
                    "link",
                )?;
                let url = element
                    .attributes
                    .get("url")
                    .map(crate::model::Value::to_text)
                    .unwrap_or_default();
                if url.is_empty() {
                    result.push_str(&write_formatted(
                        &element.children,
                        formatting,
                        styles,
                        resources,
                    )?);
                } else {
                    let relationship = resources.hyperlink(&url);
                    let mut linked = formatting.clone();
                    linked.apply_character_style("Hyperlink", styles)?;
                    result.push_str(&format!(
                        "<w:hyperlink r:id=\"{relationship}\" w:history=\"1\">"
                    ));
                    result.push_str(&write_formatted(
                        &element.children,
                        &linked,
                        styles,
                        resources,
                    )?);
                    result.push_str("</w:hyperlink>");
                }
            }
            Content::Element(element) => {
                let formatting = formatting.for_element(element, styles)?;
                result.push_str(&write_formatted(
                    &element.children,
                    &formatting,
                    styles,
                    resources,
                )?);
            }
        }
    }
    Ok(result)
}

fn write_text(
    text: &str,
    formatting: &RunFormatting,
    styles: &Styles<'_>,
) -> Result<String, Error> {
    if text.is_empty() {
        return Ok(String::new());
    }
    let mut result = String::from("<w:r>");
    let properties = formatting.write(styles)?;
    if !properties.is_empty() {
        result.push_str(&format!("<w:rPr>{properties}</w:rPr>"));
    }
    for (line_index, line) in text.split('\n').enumerate() {
        if line_index > 0 {
            result.push_str("<w:br/>");
        }
        for (part_index, part) in line.split('\t').enumerate() {
            if part_index > 0 {
                result.push_str(if formatting.fixed_tabs {
                    "<w:t xml:space=\"preserve\">    </w:t>"
                } else {
                    "<w:tab/>"
                });
            }
            if !part.is_empty() {
                result.push_str(&format!(
                    "<w:t xml:space=\"preserve\">{}</w:t>",
                    xml::escape(part)?
                ));
            }
        }
    }
    result.push_str("</w:r>");
    Ok(result)
}
