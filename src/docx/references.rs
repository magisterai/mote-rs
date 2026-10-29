//! Word bookmark ranges and editable cross-reference fields.
use super::{
    Error, resources::Resources, run_properties::RunFormatting, runs, styles::Styles, xml,
};
use crate::model::{Content, Element, ElementKind};

pub(super) fn start(
    element: &Element,
    resources: &mut Resources,
) -> Result<(String, Option<usize>), Error> {
    let name = crate::properties::text(&element.attributes, "name");
    if name.is_empty() {
        return Ok((String::new(), None));
    }
    let name = if element.kind == ElementKind::Label {
        crate::references::label_bookmark("MoteL", &name)
    } else {
        crate::references::bookmark_name(&name)
    };
    let identifier = resources.next_bookmark;
    resources.next_bookmark += 1;
    Ok((
        format!(
            r#"<w:bookmarkStart w:id="{identifier}" w:name="{}"/>"#,
            xml::escape(&name)?
        ),
        Some(identifier),
    ))
}
pub(super) fn end(identifier: Option<usize>) -> String {
    identifier
        .map(|identifier| format!(r#"<w:bookmarkEnd w:id="{identifier}"/>"#))
        .unwrap_or_default()
}
pub(super) fn start_pending(resources: &mut Resources) -> Result<(String, Vec<usize>), Error> {
    let pending = std::mem::take(&mut resources.pending_bookmarks);
    let mut output = String::new();
    let mut identifiers = Vec::new();
    for bookmark in pending {
        let (opening, identifier) = start(&bookmark, resources)?;
        output.push_str(&opening);
        identifiers.extend(identifier);
    }
    Ok((output, identifiers))
}
pub(super) fn end_pending(identifiers: &[usize]) -> String {
    identifiers
        .iter()
        .rev()
        .map(|identifier| end(Some(*identifier)))
        .collect()
}

pub(super) fn write(
    element: &Element,
    formatting: &RunFormatting,
    styles: &Styles<'_>,
    resources: &mut Resources,
) -> Result<String, Error> {
    let resolution = resources.anchors.resolve(element);
    let target = resolution.target;
    let display = resolution.display;
    let text = resolution.text;
    let inferred_number = resolution
        .anchor
        .is_some_and(|anchor| !anchor.number.is_empty() && !anchor.explicit_number);
    let has_number = resolution
        .anchor
        .is_some_and(|anchor| !anchor.number.is_empty());
    let target_name = crate::references::label_bookmark("MoteL", &target);
    if display == "page" || display == "number" && !has_number {
        let instruction = if display == "page" {
            format!(r" PAGEREF {target_name} \h ")
        } else {
            format!(r" REF {target_name} \n \h ")
        };
        return Ok(format!(
            r#"<w:fldSimple w:instr="{}">{}</w:fldSimple>"#,
            xml::escape(&instruction)?,
            runs::write_text(&text, formatting, styles)?
        ));
    }
    let mut linked = formatting.clone();
    linked.apply_character_style("Hyperlink", styles)?;
    if inferred_number && display == "number" {
        let target_name = crate::references::label_bookmark("MoteN", &target);
        return Ok(format!(
            r#"<w:fldSimple w:instr=" REF {target_name} \h ">{}</w:fldSimple>"#,
            runs::write_text(&text, &linked, styles)?
        ));
    }
    let children = if !element.children.is_empty()
        && crate::properties::text(&element.attributes, "fallback").is_empty()
        && text == crate::references::text(&element.children)
    {
        element.children.clone()
    } else {
        vec![Content::Text(text)]
    };
    Ok(format!(
        r#"<w:hyperlink w:anchor="{target_name}" w:history="1">{}</w:hyperlink>"#,
        runs::write_formatted(&children, &linked, styles, resources)?
    ))
}
