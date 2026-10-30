use super::{
    Error, check_attributes, controls,
    document::DocumentState,
    list_layout::{self, ListLayout},
    numbering, runs,
    styles::Styles,
    values,
};
use crate::model::{Content, Element, ElementKind};

pub(super) fn write(
    list: &Element,
    inherited_level: u8,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    check_attributes(
        &list.attributes,
        &[
            "id", "start", "restart", "format", "level", "tight", "indent", "spacing",
        ],
        "list",
    )?;
    let level = list_layout::nesting_level(&list.attributes, inherited_level)?;
    let layout = ListLayout::from_list(&list.attributes)?;
    let mut numbering_identifier = None;
    let mut output = String::new();
    for child in &list.children {
        let item = match child {
            Content::Text(text) if text.trim().is_empty() => continue,
            Content::Element(element) if element.kind == ElementKind::ListItem => element,
            _ => return Err(Error::InvalidValue("lists can contain only items".into())),
        };
        check_attributes(
            &item.attributes,
            &[
                "marker", "level", "tight", "indent", "spacing", "checked", "checkbox",
            ],
            "list item",
        )?;
        let item_level = list_layout::nesting_level(&item.attributes, level)?;
        let checked = item
            .attributes
            .get("checked")
            .map(values::boolean)
            .transpose()?;
        let checkbox = controls::checkbox_properties(&item.attributes)?;
        check_attributes(&checkbox, &["checked", "kind", "name", "title"], "checkbox")?;
        if state.table_cell_depth > 0 && item.children.iter().any(|child|matches!(child,Content::Element(element) if element.kind==ElementKind::Paragraph)) {
            let identifier=if checked.is_none() {Some(match numbering_identifier {Some(identifier)=>identifier,None=>{let identifier=state.numbering.for_list(list,layout,level)?;numbering_identifier=Some(identifier);identifier}})}else{None};
            write_structured_item(&mut output,item,item_level,layout,StructuredMarker{identifier,checked,checkbox:&checkbox},styles,state)?;
            continue;
        }
        let (mut inline, nested) = split_item_children(&item.children)?;
        let bookmarks = std::mem::take(&mut state.resources.pending_bookmarks);
        inline.splice(0..0, bookmarks.into_iter().map(Content::Element));
        output.push_str("<w:p><w:pPr>");
        if checked.is_none() {
            let identifier = match numbering_identifier {
                Some(identifier) => identifier,
                None => {
                    let identifier = state.numbering.for_list(list, layout, level)?;
                    numbering_identifier = Some(identifier);
                    identifier
                }
            };
            output.push_str(&numbering::paragraph_reference(identifier, item_level));
        }
        if checked.is_some()
            || ["indent", "spacing", "tight"]
                .iter()
                .any(|name| item.attributes.contains_key(*name))
        {
            output.push_str(
                &layout
                    .for_item(&item.attributes, item_level)?
                    .write_paragraph_properties(),
            );
        }
        output.push_str("</w:pPr>");
        if let Some(checked) = checked {
            output.push_str(&state.controls.write_checkbox(
                &checkbox,
                checked,
                styles,
                &state.resources.active_text,
            )?);
            output.push_str(&runs::write_text(
                " ",
                &state.resources.active_text,
                styles,
            )?);
        }
        output.push_str(&runs::write_inlines(&inline, styles, &mut state.resources)?);
        output.push_str("</w:p>");
        for nested_list in nested {
            output.push_str(&write(nested_list, item_level + 1, styles, state)?);
        }
    }
    Ok(output)
}

fn split_item_children(children: &[Content]) -> Result<(Vec<Content>, Vec<&Element>), Error> {
    let mut inline = Vec::new();
    let mut nested = Vec::new();
    let mut pending_whitespace = Vec::new();
    for child in children {
        match child {
            Content::Element(element)
                if matches!(element.kind, ElementKind::List | ElementKind::OrderedList) =>
            {
                pending_whitespace.clear();
                nested.push(element);
            }
            Content::Text(text) if text.trim().is_empty() => pending_whitespace.push(child.clone()),
            _ => {
                inline.append(&mut pending_whitespace);
                if let Content::Element(element) = child
                    && element.kind.is_block()
                    && element.kind != ElementKind::Image
                {
                    if element.kind != ElementKind::Paragraph {
                        return Err(Error::Unsupported(format!(
                            "{} inside list item text",
                            element.kind.as_text()
                        )));
                    }
                    check_attributes(&element.attributes, &[], "paragraph inside list item")?;
                    inline.extend(element.children.clone());
                } else {
                    inline.push(child.clone());
                }
            }
        }
    }
    Ok((inline, nested))
}

struct StructuredMarker<'properties> {
    identifier: Option<usize>,
    checked: Option<bool>,
    checkbox: &'properties crate::model::Properties,
}
fn write_structured_item(
    output: &mut String,
    item: &Element,
    level: u8,
    layout: ListLayout,
    marker: StructuredMarker<'_>,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<(), Error> {
    use crate::model::{Properties, Unit, Value};
    let StructuredMarker {
        identifier,
        checked,
        checkbox,
    } = marker;
    let layout = layout.for_item(&item.attributes, level)?;
    let mut first = true;
    let mut pending = Vec::new();
    let mut write_units = |pending: &mut Vec<Content>,
                           output: &mut String,
                           state: &mut DocumentState|
     -> Result<(), Error> {
        if pending
            .iter()
            .all(|node| matches!(node,Content::Text(text) if text.trim().is_empty()))
        {
            pending.clear();
            return Ok(());
        }
        for (mut attributes, children) in crate::tables::paragraph_units(pending) {
            let override_layout = checked.is_some()
                || !first
                || ["indent", "spacing", "tight"]
                    .iter()
                    .any(|name| item.attributes.contains_key(*name));
            if override_layout {
                let mut indent = Properties::from([(
                    "left".into(),
                    Value::Length {
                        value: layout.left as f64 / 20.0,
                        unit: Unit::Point,
                    },
                )]);
                if first {
                    indent.insert(
                        "hanging".into(),
                        Value::Length {
                            value: layout.hanging as f64 / 20.0,
                            unit: Unit::Point,
                        },
                    );
                }
                attributes
                    .entry("indent".into())
                    .or_insert(Value::Object(indent));
                attributes
                    .entry("spacing_after".into())
                    .or_insert(Value::Length {
                        value: layout.spacing_after as f64 / 20.0,
                        unit: Unit::Point,
                    });
            }
            let numbering = if first {
                identifier.map(|identifier| (identifier, level))
            } else {
                None
            };
            let mut paragraph = super::paragraphs::write(
                &attributes,
                &children,
                styles,
                numbering,
                false,
                &mut state.resources,
            )?;
            if first && let Some(checked) = checked {
                let checkbox = state.controls.write_checkbox(
                    checkbox,
                    checked,
                    styles,
                    &state.resources.active_text,
                )? + &runs::write_text(" ", &state.resources.active_text, styles)?;
                let index = paragraph.find("</w:pPr>").map_or(5, |index| index + 8);
                paragraph.insert_str(index, &checkbox);
            }
            output.push_str(&paragraph);
            first = false;
        }
        pending.clear();
        Ok(())
    };
    for child in &item.children {
        if let Content::Element(nested) = child
            && matches!(nested.kind, ElementKind::List | ElementKind::OrderedList)
        {
            write_units(&mut pending, output, state)?;
            output.push_str(&write(nested, level + 1, styles, state)?);
        } else {
            pending.push(child.clone());
        }
    }
    write_units(&mut pending, output, state)
}
