//! Editable caption sequences and figure-associated bookmark ranges.
use super::{
    Error,
    document::{DocumentState, write_blocks},
    paragraphs, references, runs,
    styles::Styles,
    xml,
};
use crate::{
    captions::Numbering,
    model::{Content, Element, ElementKind, Properties, Value},
    properties::text,
};

pub(super) fn write_caption(
    caption: &Element,
    kind: &str,
    parent: &Properties,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let mut attributes = caption.attributes.clone();
    attributes.remove("numbering");
    attributes.remove("position");
    attributes
        .entry("style".into())
        .or_insert(Value::String("Caption".into()));
    let mut output = paragraphs::opening(&attributes, styles, None, &mut state.resources)?;
    let bookmarks = state.resources.pending_bookmarks.clone();
    let (opening, identifiers) = references::start_pending(&mut state.resources)?;
    output.push_str(&opening);
    let numbering = Numbering::resolve(kind, &caption.attributes, parent);
    if let Some(number) = numbering
        .advance(&mut state.resources.caption_sequences)
        .map_err(|message| Error::InvalidValue(message.into()))?
    {
        let mut number_identifiers = Vec::new();
        for bookmark in bookmarks
            .iter()
            .filter(|bookmark| bookmark.kind == ElementKind::Label)
        {
            let identifier = state.resources.next_bookmark;
            state.resources.next_bookmark += 1;
            let name =
                crate::references::label_bookmark("MoteN", &text(&bookmark.attributes, "name"));
            output.push_str(&format!(
                r#"<w:bookmarkStart w:id="{identifier}" w:name="{name}"/>"#
            ));
            number_identifiers.push(identifier);
        }
        let mut formatting = state.resources.active_text.clone();
        formatting
            .properties
            .insert("bold".into(), Value::Boolean(true));
        output.push_str(&runs::write_text(
            &format!("{} ", numbering.label),
            &formatting,
            styles,
        )?);
        let mut instruction = format!(
            r" SEQ {} \* {}",
            numbering.sequence,
            super::sections::field_number_format(&numbering.format)
        );
        if numbering.restart || !number.sequence_started && numbering.start > 0 {
            instruction.push_str(&format!(r" \r {}", number.value));
        }
        let properties = formatting.write(styles)?;
        let properties = if properties.is_empty() {
            properties
        } else {
            format!("<w:rPr>{properties}</w:rPr>")
        };
        output.push_str(&format!(
            r#"<w:fldSimple w:instr="{} "><w:r>{properties}<w:t>{}</w:t></w:r></w:fldSimple>"#,
            xml::escape(&instruction)?,
            xml::escape(&number.formatted)?
        ));
        output.push_str(&references::end_pending(&number_identifiers));
        output.push_str(&runs::write_text(
            &numbering.separator,
            &formatting,
            styles,
        )?);
    }
    output.push_str(&runs::write_inlines(
        &caption.children,
        styles,
        &mut state.resources,
    )?);
    output.push_str(&references::end_pending(&identifiers));
    output.push_str("</w:p>");
    Ok(output)
}
pub(super) fn write_figure(
    figure: &Element,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let kind = text(&figure.attributes, "kind");
    let mut alignment = text(&figure.attributes, "alignment");
    if alignment.is_empty()
        && let Some(Value::Object(placement)) = figure.attributes.get("placement")
    {
        alignment = text(placement, "align");
    }
    let mut bookmarks = std::mem::take(&mut state.resources.pending_bookmarks);
    let mut children = Vec::new();
    for child in &figure.children {
        if let Some(labels) = crate::references::standalone_labels(child) {
            bookmarks.extend(labels);
        } else {
            children.push(child);
        }
    }
    let mut output = String::new();
    state.figure_depth += 1;
    for child in children {
        if let Content::Element(element) = child {
            let element = crate::captions::aligned_child(element, &alignment);
            if element.kind == ElementKind::Caption {
                state.resources.pending_bookmarks = std::mem::take(&mut bookmarks);
                output.push_str(&write_caption(
                    &element,
                    &kind,
                    &figure.attributes,
                    styles,
                    state,
                )?);
            } else {
                output.push_str(&write_blocks(&[Content::Element(element)], styles, state)?);
            }
        } else {
            output.push_str(&write_blocks(std::slice::from_ref(child), styles, state)?);
        }
    }
    state.figure_depth -= 1;
    if !bookmarks.is_empty() {
        state.resources.pending_bookmarks = bookmarks;
        output.push_str(&paragraphs::write(
            &Properties::new(),
            &[],
            styles,
            None,
            false,
            &mut state.resources,
        )?);
    }
    Ok(output)
}
