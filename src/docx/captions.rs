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
    let last = children.len().saturating_sub(1);
    for (index, child) in children.into_iter().enumerate() {
        let block = if let Content::Element(element) = child {
            let element = crate::captions::aligned_child(element, &alignment);
            if element.kind == ElementKind::Caption {
                state.resources.pending_bookmarks = std::mem::take(&mut bookmarks);
                write_caption(&element, &kind, &figure.attributes, styles, state)?
            } else {
                write_blocks(&[Content::Element(element)], styles, state)?
            }
        } else {
            write_blocks(std::slice::from_ref(child), styles, state)?
        };
        if index < last {
            output.push_str(&keep_with_next(&block));
        } else {
            output.push_str(&block);
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

/// Returns the blocks with `w:keepNext` on each paragraph, so that Word sets
/// the blocks on the page of the block after them. Word keeps a table row with
/// what follows only if the paragraphs of the row ask for it, so the paragraphs
/// within table cells are included. The paragraphs of a text box are not in
/// the flow of the page and are left as they are, as is a paragraph that
/// states `w:keepNext` already.
fn keep_with_next(blocks: &str) -> String {
    const TEXT_BOX_OPENING: &str = "<w:txbxContent>";
    const TEXT_BOX_CLOSING: &str = "</w:txbxContent>";
    const EMPTY_PARAGRAPH: &str = "<w:p/>";
    const PARAGRAPH_OPENING: &str = "<w:p>";
    const PROPERTIES_OPENING: &str = "<w:pPr>";
    const KEEP: &str = "<w:keepNext/>";
    let mut output = String::with_capacity(blocks.len());
    let mut rest = blocks;
    let mut text_box_depth = 0usize;
    while let Some((position, token)) = [
        TEXT_BOX_OPENING,
        TEXT_BOX_CLOSING,
        EMPTY_PARAGRAPH,
        PARAGRAPH_OPENING,
    ]
    .into_iter()
    .filter_map(|token| rest.find(token).map(|position| (position, token)))
    .min()
    {
        output.push_str(&rest[..position]);
        rest = &rest[position + token.len()..];
        if token == TEXT_BOX_OPENING {
            text_box_depth += 1;
        } else if token == TEXT_BOX_CLOSING {
            text_box_depth = text_box_depth.saturating_sub(1);
        }
        if text_box_depth > 0 || token == TEXT_BOX_CLOSING {
            output.push_str(token);
        } else if token == EMPTY_PARAGRAPH {
            output.push_str(&format!(
                "{PARAGRAPH_OPENING}{PROPERTIES_OPENING}{KEEP}</w:pPr></w:p>"
            ));
        } else if let Some(properties) = rest.strip_prefix(PROPERTIES_OPENING) {
            output.push_str(PARAGRAPH_OPENING);
            output.push_str(PROPERTIES_OPENING);
            let end = properties.find("</w:pPr>").unwrap_or(properties.len());
            if properties[..end].contains("<w:keepNext") {
                rest = properties;
                continue;
            }
            // The schema places `w:keepNext` directly after `w:pStyle`.
            let style_end = if properties.starts_with("<w:pStyle ") {
                properties.find("/>").map_or(0, |position| position + 2)
            } else {
                0
            };
            output.push_str(&properties[..style_end]);
            output.push_str(KEEP);
            rest = &properties[style_end..];
        } else {
            output.push_str(&format!(
                "{PARAGRAPH_OPENING}{PROPERTIES_OPENING}{KEEP}</w:pPr>"
            ));
        }
    }
    output.push_str(rest);
    output
}
