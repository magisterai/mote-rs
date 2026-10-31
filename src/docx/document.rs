use super::{
    Error, block_quotes, check_attributes, controls::Controls, headings, lists,
    numbering::Numbering, paragraphs, styles::Styles, xml,
};
use crate::model::{Content, Document, ElementKind, Value};

#[derive(Default)]
pub(super) struct DocumentState {
    pub numbering: Numbering,
    pub controls: Controls,
    pub table_cell_depth: usize,
    pub figure_depth: usize,
    pub resources: super::resources::Resources,
}

pub(super) fn write_document(
    document: &Document,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let body = super::sections::write(document, styles, state)?;
    Ok(format!(
        r#"{}<w:document{attributes}><w:body>{body}</w:body></w:document>"#,
        xml::HEADER,
        attributes = xml::ROOT_ATTRIBUTES
    ))
}

pub(super) fn write_blocks(
    content: &[Content],
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let mut body = String::new();
    let mut at_flow_start =
        state.table_cell_depth == 0 && state.figure_depth == 0 && !state.resources.notes.writing;
    let mut previous_table = false;
    let mut pending_column_break = false;
    for content in content {
        if let Some(labels) = crate::references::standalone_labels(content) {
            state.resources.pending_bookmarks.extend(labels);
            continue;
        }
        let Content::Element(element) = content else {
            return Err(Error::Unsupported(
                "top-level text outside a paragraph".into(),
            ));
        };
        if previous_table && element.kind == ElementKind::Table {
            body.push_str("<w:p/>");
        }
        previous_table = element.kind == ElementKind::Table;
        if element.kind == ElementKind::Break {
            check_attributes(&element.attributes, &["type"], "break")?;
            let kind = element
                .attributes
                .get("type")
                .map(Value::to_text)
                .unwrap_or_else(|| "page".into());
            let paragraph = break_paragraph(&kind)?;
            if !state.resources.pending_bookmarks.is_empty() {
                let (opening, identifiers) =
                    super::references::start_pending(&mut state.resources)?;
                body.push_str(
                    &paragraph
                        .replacen("<w:p>", &format!("<w:p>{opening}"), 1)
                        .replace(
                            "</w:p>",
                            &format!("{}</w:p>", super::references::end_pending(&identifiers)),
                        ),
                );
                at_flow_start = false;
                continue;
            }
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
            ElementKind::Figure => {
                body.push_str(&super::captions::write_figure(element, styles, state)?);
                at_flow_start = false;
                continue;
            }
            ElementKind::Caption => {
                body.push_str(&super::captions::write_caption(
                    element,
                    "figure",
                    &Default::default(),
                    styles,
                    state,
                )?);
                at_flow_start = false;
                continue;
            }
            ElementKind::Table => {
                body.push_str(&super::tables::write(element, styles, state)?);
                at_flow_start = false;
                continue;
            }
            ElementKind::Chart => {
                body.push_str(&super::charts::write(element, &mut state.resources)?);
                at_flow_start = false;
                continue;
            }
            ElementKind::Shape => {
                body.push_str(&super::shapes::write(element, styles, state)?);
                at_flow_start = false;
                continue;
            }
            ElementKind::Image => {
                body.push_str(&super::images::write(
                    element,
                    styles,
                    &mut state.resources,
                )?);
                at_flow_start = false;
                continue;
            }
            ElementKind::Raw | ElementKind::Divider | ElementKind::DefinitionList => {
                body.push_str(&super::block_content::write(
                    element,
                    styles,
                    state,
                    leading_column_break,
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
    if !state.resources.pending_bookmarks.is_empty() {
        body.push_str(&paragraphs::write(
            &Default::default(),
            &[],
            styles,
            None,
            false,
            &mut state.resources,
        )?);
    }
    Ok(body)
}

fn break_paragraph(kind: &str) -> Result<&'static str, Error> {
    match kind {
        "page" => Ok("<w:p><w:r><w:br w:type=\"page\"/></w:r></w:p>"),
        "column" => Ok("<w:p><w:r><w:br w:type=\"column\"/></w:r></w:p>"),
        "line" => Ok("<w:p><w:r><w:br/></w:r></w:p>"),
        _ => Err(Error::Unsupported(format!("break type {kind:?}"))),
    }
}
