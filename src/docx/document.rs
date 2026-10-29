use super::{
    Error, block_quotes, check_attributes, configuration, controls::Controls, headings, lists,
    numbering::Numbering, paragraphs, styles::Styles, xml,
};
use crate::model::{Content, Document, ElementKind, Value};

#[derive(Default)]
pub(super) struct DocumentState {
    pub numbering: Numbering,
    pub controls: Controls,
    pub resources: super::resources::Resources,
}

pub(super) fn write_document(
    document: &Document,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let mut body = String::new();
    let mut at_flow_start = true;
    let mut pending_column_break = false;
    for content in &document.content {
        let Content::Element(element) = content else {
            return Err(Error::Unsupported(
                "top-level text outside a paragraph".into(),
            ));
        };
        if element.kind == ElementKind::Break {
            check_attributes(&element.attributes, &["type"], "break")?;
            if !element.children.is_empty() {
                return Err(Error::InvalidValue("break cannot contain children".into()));
            }
            let kind = element
                .attributes
                .get("type")
                .map(Value::to_text)
                .unwrap_or_else(|| "page".into());
            let paragraph = break_paragraph(&kind)?;
            if pending_column_break {
                body.push_str(break_paragraph("column")?);
            }
            pending_column_break = kind == "column";
            if !pending_column_break {
                body.push_str(paragraph);
                at_flow_start = false;
            }
            continue;
        }
        let leading_column_break = std::mem::take(&mut pending_column_break);
        if leading_column_break
            && !matches!(
                element.kind,
                ElementKind::Paragraph | ElementKind::Heading | ElementKind::Blockquote
            )
        {
            body.push_str(break_paragraph("column")?);
        }
        let mut numbering_reference = None;
        let attributes = match element.kind {
            ElementKind::Image => {
                body.push_str(&super::images::write(
                    element,
                    styles,
                    &mut state.resources,
                )?);
                at_flow_start = false;
                continue;
            }
            ElementKind::Paragraph => element.attributes.clone(),
            ElementKind::Heading => {
                let level = headings::level(element)? - 1;
                numbering_reference = state
                    .numbering
                    .for_heading(element, level)?
                    .map(|identifier| (identifier, level));
                headings::attributes(element, at_flow_start || leading_column_break)?
            }
            ElementKind::Blockquote => {
                body.push_str(&block_quotes::write(
                    element,
                    styles,
                    leading_column_break,
                    &mut state.resources,
                )?);
                at_flow_start = false;
                continue;
            }
            ElementKind::List | ElementKind::OrderedList => {
                body.push_str(&lists::write(element, 0, styles, state)?);
                at_flow_start = false;
                continue;
            }
            _ => {
                return Err(Error::Unsupported(format!(
                    "{} content",
                    element.kind.as_text()
                )));
            }
        };
        body.push_str(&paragraphs::write(
            &attributes,
            &element.children,
            styles,
            numbering_reference,
            leading_column_break,
            &mut state.resources,
        )?);
        at_flow_start = false;
    }
    if pending_column_break {
        body.push_str(break_paragraph("column")?);
    }
    body.push_str(&configuration::section_properties(document)?);
    Ok(format!(
        r#"{}<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="w14 wps"><w:body>{body}</w:body></w:document>"#,
        xml::HEADER
    ))
}

fn break_paragraph(kind: &str) -> Result<&'static str, Error> {
    match kind {
        "page" => Ok("<w:p><w:r><w:br w:type=\"page\"/></w:r></w:p>"),
        "column" => Ok("<w:p><w:r><w:br w:type=\"column\"/></w:r></w:p>"),
        "line" => Ok("<w:p><w:r><w:br/></w:r></w:p>"),
        _ => Err(Error::Unsupported(format!("break type {kind:?}"))),
    }
}
