//! Literal blocks, definition paragraphs and horizontal rules in Word output.
use super::{
    Error, builtin_styles, check_attributes, paragraphs, run_properties::RunFormatting,
    styles::Styles, values, xml,
};
use crate::model::{Element, ElementKind, Properties, Value};

pub(super) fn write(
    element: &Element,
    styles: &Styles<'_>,
    state: &mut super::document::DocumentState,
    leading_column_break: bool,
) -> Result<String, Error> {
    match element.kind {
        ElementKind::DefinitionList => {
            check_attributes(&element.attributes, &["style"], "definition_list")?;
            let mut output = String::new();
            for (index, paragraph) in
                crate::block_content::definition_paragraphs(element, state.table_cell_depth > 0)
                    .iter()
                    .enumerate()
            {
                output.push_str(&paragraphs::write(
                    &paragraph.attributes,
                    &paragraph.children,
                    styles,
                    None,
                    leading_column_break && index == 0,
                    &mut state.resources,
                )?);
            }
            Ok(output)
        }
        ElementKind::Raw => {
            check_attributes(
                &element.attributes,
                &["style", "language", "preserve_space"],
                "raw",
            )?;
            let mut attributes = element.attributes.clone();
            attributes.remove("language");
            attributes.remove("preserve_space");
            attributes
                .entry("style".into())
                .or_insert(Value::String("CodeBlock".into()));
            let previous = state.resources.active_text.clone();
            if crate::properties::boolean(&element.attributes, "preserve_space") {
                state.resources.active_text.fixed_tabs = true;
            }
            let result = paragraphs::write(
                &attributes,
                &element.children,
                styles,
                None,
                leading_column_break,
                &mut state.resources,
            );
            state.resources.active_text = previous;
            result
        }
        ElementKind::Divider => {
            check_attributes(
                &element.attributes,
                &["style", "color", "width", "spacing"],
                "divider",
            )?;
            let mut attributes = element.attributes.clone();
            attributes
                .entry("style".into())
                .or_insert(Value::String("HorizontalRule".into()));
            if attributes.contains_key("color") || attributes.contains_key("width") {
                let color = attributes
                    .get("color")
                    .map(values::color)
                    .transpose()?
                    .unwrap_or_else(|| "D1D5DB".into());
                let size = attributes
                    .get("width")
                    .map(values::twips)
                    .transpose()?
                    .map(|width| (width as f64 / 20.0 * 8.0).round().max(1.0))
                    .unwrap_or(4.0);
                attributes.insert(
                    "border".into(),
                    Value::Object(Properties::from([(
                        "bottom".into(),
                        Value::Object(Properties::from([
                            ("style".into(), Value::String("single".into())),
                            ("color".into(), Value::Color(color)),
                            ("size".into(), Value::Number(size)),
                            ("space".into(), Value::Number(0.0)),
                        ])),
                    )])),
                );
            }
            attributes.remove("color");
            attributes.remove("width");
            paragraphs::write(
                &attributes,
                &[],
                styles,
                None,
                leading_column_break,
                &mut state.resources,
            )
        }
        _ => unreachable!(),
    }
}

pub(super) fn write_style(name: &str, styles: &Styles<'_>) -> Result<String, Error> {
    let (display, paragraph) = match name {
        "HorizontalRule" => (
            "Horizontal Rule",
            "<w:pBdr><w:bottom w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"D1D5DB\"/></w:pBdr><w:spacing w:before=\"120\" w:after=\"160\" w:line=\"20\" w:lineRule=\"exact\"/>",
        ),
        "DefinitionTerm" => (
            "Definition Term",
            "<w:spacing w:after=\"160\" w:line=\"276\" w:lineRule=\"auto\"/>",
        ),
        "Definition" => (
            "Definition",
            "<w:spacing w:after=\"160\" w:line=\"276\" w:lineRule=\"auto\"/><w:ind w:left=\"360\"/>",
        ),
        "CodeBlock" => (
            "Code Block",
            "<w:spacing w:after=\"160\" w:line=\"276\" w:lineRule=\"auto\"/>",
        ),
        _ => unreachable!(),
    };
    let run = RunFormatting {
        properties: builtin_styles::run_properties(name),
        ..Default::default()
    }
    .write_declaration(styles)?;
    Ok(format!(
        "<w:style w:type=\"paragraph\" w:styleId=\"{name}\"><w:name w:val=\"{display}\"/><w:basedOn w:val=\"{}\"/><w:pPr>{paragraph}</w:pPr><w:rPr>{run}</w:rPr></w:style>",
        xml::escape(styles.identifier("Normal")?)?
    ))
}
