use crate::{
    model::{Document, Element, Properties, StyleDeclaration, Value},
    properties as property_values,
};
use std::collections::BTreeSet;

pub(crate) fn declared(style: &StyleDeclaration) -> Properties {
    let mut properties = style.properties.clone();
    properties.remove("paragraph");
    properties.remove("run");
    properties.remove("table");
    if let Some(table) = property_values::object(&style.properties, "table") {
        properties.extend(table.clone());
    }
    properties
}
pub(crate) fn resolved(document: &Document, name: &str) -> Properties {
    let mut chain = Vec::new();
    let mut seen = BTreeSet::new();
    let mut current = name.to_owned();
    loop {
        let normalized: String = current
            .to_lowercase()
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect();
        if normalized.is_empty() {
            break;
        }
        let declaration = document.styles.get(&current).or_else(|| {
            document
                .styles
                .iter()
                .find(|(name, _)| {
                    name.to_lowercase()
                        .chars()
                        .filter(char::is_ascii_alphanumeric)
                        .collect::<String>()
                        == normalized
                })
                .map(|(_, style)| style)
        });
        let Some(style) = declaration else { break };
        if !seen.insert(style.name.clone()) {
            break;
        }
        chain.push(style);
        current = property_values::text(&style.properties, "based_on");
    }
    let mut properties = Properties::new();
    for style in chain.into_iter().rev() {
        merge_nested(&mut properties, &declared(style));
    }
    properties
}
pub(crate) fn merge_nested(target: &mut Properties, overlay: &Properties) {
    for (name, value) in overlay {
        if let (Some(Value::Object(existing)), Value::Object(fields)) =
            (target.get_mut(name), value)
        {
            merge_nested(existing, fields);
        } else {
            target.insert(name.clone(), value.clone());
        }
    }
}
pub(crate) fn bands(properties: &Properties) -> (bool, bool) {
    let Some(banding) = property_values::object(properties, "banding") else {
        return (false, false);
    };
    (
        property_values::boolean(banding, "rows")
            || region(banding, "odd")
            || region(banding, "even"),
        property_values::boolean(banding, "columns")
            || region(banding, "odd_column")
            || region(banding, "even_column"),
    )
}
pub(crate) fn region(properties: &Properties, name: &str) -> bool {
    property_values::object(properties, name).is_some_and(|region| !region.is_empty())
}
pub(crate) fn regions(
    document: &Document,
    table: &Element,
    cell: &Element,
    row: usize,
    row_count: usize,
) -> Vec<Properties> {
    let base = resolved(document, &property_values::text(&table.attributes, "style"));
    let mut regions = vec![base.clone()];
    if let Some(banding) = property_values::object(&base, "banding") {
        let has_header = region(&base, "first_row");
        if !has_header || row > 0 {
            let name = if (row - usize::from(has_header)).is_multiple_of(2) {
                "odd"
            } else {
                "even"
            };
            if let Some(band) = property_values::object(banding, name) {
                regions.push(band.clone());
            }
        }
    }
    let cell_style = resolved(document, &property_values::text(&cell.attributes, "style"));
    for properties in [&base, &cell_style] {
        for (name, applies) in [("first_row", row == 0), ("last_row", row + 1 == row_count)] {
            if applies && let Some(region) = property_values::object(properties, name) {
                regions.push(region.clone());
            }
        }
    }
    regions
}
pub(crate) fn text(
    document: &Document,
    table: &Element,
    cell: &Element,
    row: usize,
    row_count: usize,
) -> Properties {
    let mut text = Properties::new();
    for region in regions(document, table, cell, row, row_count) {
        if let Some(properties) = property_values::object(&region, "text") {
            for (name, value) in properties {
                text.entry(name.clone()).or_insert_with(|| value.clone());
            }
        }
    }
    text
}
