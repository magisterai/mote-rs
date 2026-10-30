use crate::{
    model::{Document, Element, Properties, Value},
    properties::{boolean, object, text},
    tables::GridCell,
};
#[derive(Default)]
pub(crate) struct CellFormat {
    pub cell: Properties,
    pub paragraph: Properties,
    pub text: Properties,
}
const CELL_FIELDS: &[&str] = &[
    "style",
    "width",
    "fill",
    "vertical_alignment",
    "alignment",
    "margin",
    "borders",
    "text_direction",
    "no_wrap",
];
impl CellFormat {
    fn apply(&mut self, properties: &Properties) {
        for name in CELL_FIELDS {
            if let Some(value) = properties.get(*name) {
                self.cell.insert((*name).into(), value.clone());
            }
        }
    }
    fn region(&mut self, properties: Option<&Properties>) {
        let Some(properties) = properties else {
            return;
        };
        let mut cell = properties.clone();
        cell.remove("style");
        cell.remove("width");
        cell.extend(object(properties, "cell").cloned().unwrap_or_default());
        self.apply(&cell);
        self.paragraph
            .extend(object(properties, "paragraph").cloned().unwrap_or_default());
        self.text
            .extend(object(properties, "text").cloned().unwrap_or_default());
    }
}
pub(super) fn style(document: &Document, table: &Element) -> Properties {
    let name = text(&table.attributes, "style");
    let normalize = |name: &str| {
        name.chars()
            .filter(char::is_ascii_alphanumeric)
            .flat_map(char::to_lowercase)
            .collect::<String>()
    };
    let Some(style) = document.styles.get(&name).or_else(|| {
        document
            .styles
            .values()
            .find(|style| normalize(&style.name) == normalize(&name))
    }) else {
        return Properties::new();
    };
    if text(&style.properties, "type") != "table" {
        return Properties::new();
    }
    crate::tables::styles::declared(style)
}
pub(super) fn resolve(
    document: &Document,
    table: &Element,
    style: &Properties,
    cell: &GridCell,
    rows: usize,
    columns: usize,
) -> CellFormat {
    let mut format = CellFormat::default();
    if let Some(defaults) = object(style, "cell") {
        format.apply(defaults);
    }
    format
        .paragraph
        .extend(object(style, "paragraph").cloned().unwrap_or_default());
    format
        .text
        .extend(object(style, "text").cloned().unwrap_or_default());
    if let Some(declaration) = document.styles.get(&text(&table.attributes, "style"))
        && let Some(font) = declaration.properties.get("font")
    {
        format
            .text
            .entry("font".into())
            .or_insert_with(|| font.clone());
    }
    if let Some(defaults) = object(&table.attributes, "cell") {
        format.apply(defaults);
    }
    if let Some(banding) = object(style, "banding") {
        format.region(object(
            banding,
            if cell.origin_column.is_multiple_of(2) {
                "odd_column"
            } else {
                "even_column"
            },
        ));
        format.region(object(
            banding,
            if cell.origin_row.is_multiple_of(2) {
                "odd"
            } else {
                "even"
            },
        ));
    }
    for (name, applies) in [
        ("first_row", cell.origin_row == 0),
        ("last_row", cell.origin_row + cell.row_span >= rows),
        ("first_column", cell.origin_column == 0),
        (
            "last_column",
            cell.origin_column + cell.column_span >= columns,
        ),
    ] {
        if applies {
            format.region(object(style, name));
        }
    }
    format.apply(&cell.element.attributes);
    if let Some(alignment) = format.cell.get("alignment") {
        format
            .paragraph
            .entry("alignment".into())
            .or_insert_with(|| alignment.clone());
    }
    format
}
pub(crate) fn option_enabled(properties: &Properties, name: &str) -> bool {
    object(properties, name).is_some() || boolean(properties, name)
}
pub(crate) fn border_for_side(
    cell: Option<&Properties>,
    table: &Properties,
    side: &str,
    position: &GridCell,
    rows: usize,
    columns: usize,
) -> Option<Properties> {
    let value = cell
        .and_then(|cell| cell.get(side).or_else(|| cell.get("all")))
        .or_else(|| {
            let outside = match side {
                "top" => position.origin_row == 0,
                "bottom" => position.origin_row + position.row_span >= rows,
                "left" => position.origin_column == 0,
                _ => position.origin_column + position.column_span >= columns,
            };
            let names = if outside {
                [side, "outside", "all"]
            } else if matches!(side, "top" | "bottom") {
                ["insideH", "inside", "all"]
            } else {
                ["insideV", "inside", "all"]
            };
            names.iter().find_map(|name| table.get(*name))
        })?;
    Some(match value {
        Value::Object(properties) => properties.clone(),
        _ => Properties::from([("style".into(), value.clone())]),
    })
}
