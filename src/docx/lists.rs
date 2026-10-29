use super::{
    Error, check_attributes,
    list_layout::{self, ListLayout},
    numbering::{self, Numbering},
    runs,
    styles::Styles,
    whitespace,
};
use crate::model::{Content, Element, ElementKind, Value};

pub(super) fn write(
    list: &Element,
    inherited_level: u8,
    styles: &Styles<'_>,
    numbering: &mut Numbering,
) -> Result<String, Error> {
    if list.kind != ElementKind::List {
        return Err(Error::Unsupported("ordered lists".into()));
    }
    check_attributes(
        &list.attributes,
        &[
            "id", "start", "restart", "format", "level", "tight", "indent", "spacing",
        ],
        "list",
    )?;
    let level = list_layout::nesting_level(&list.attributes, inherited_level)?;
    let layout = ListLayout::from_list(&list.attributes)?;
    let identity = list
        .attributes
        .get("id")
        .map(Value::to_text)
        .unwrap_or_default();
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
            &["marker", "level", "tight", "indent", "spacing"],
            "list item",
        )?;
        let item_level = list_layout::nesting_level(&item.attributes, level)?;
        let identifier = *numbering_identifier
            .get_or_insert_with(|| numbering.for_bullets(layout, &identity, level));
        let (inline, nested) = split_item_children(&item.children)?;
        output.push_str("<w:p><w:pPr>");
        output.push_str(&numbering::paragraph_reference(identifier, item_level));
        if ["indent", "spacing", "tight"]
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
        output.push_str(&runs::write_inlines(
            &whitespace::trim_line_edges(&inline),
            styles,
        )?);
        output.push_str("</w:p>");
        for nested_list in nested {
            output.push_str(&write(nested_list, item_level + 1, styles, numbering)?);
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
