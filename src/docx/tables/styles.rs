use super::super::{
    Error, paragraph_properties,
    run_properties::{self, RunFormatting},
    styles::Styles,
    values, xml,
};
use super::properties;
use crate::{
    model::{Properties, StyleDeclaration},
    properties as property_values,
    tables::styles as table_styles,
};

pub(in crate::docx) fn write(
    declaration: &StyleDeclaration,
    styles: &Styles<'_>,
) -> Result<String, Error> {
    let attributes = table_styles::declared(declaration);
    let mut output = format!(
        "<w:style w:type=\"table\" w:styleId=\"{}\"><w:name w:val=\"{}\"/>",
        xml::escape(styles.identifier(&declaration.name)?)?,
        xml::escape(&declaration.name)?
    );
    for (name, word) in [("based_on", "basedOn"), ("next", "next"), ("link", "link")] {
        let value = property_values::text(&declaration.properties, name);
        if !value.is_empty() {
            output.push_str(&format!(
                "<w:{word} w:val=\"{}\"/>",
                xml::escape(styles.identifier(&value)?)?
            ));
        }
    }
    output.push_str(&paragraph(&attributes, false)?);
    let text = property_values::object(&attributes, "text")
        .cloned()
        .unwrap_or_else(|| {
            let mut text: Properties = declaration
                .properties
                .iter()
                .filter(|(name, _)| run_properties::ATTRIBUTES.contains(&name.as_str()))
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect();
            if let Some(run) = property_values::object(&declaration.properties, "run") {
                text.extend(run.clone());
            }
            text
        });
    output.push_str(&run(&text, &declaration.properties, styles)?);
    output.push_str(&properties::wrap("tblPr", table_properties(&attributes)?));
    let mut cell = property_values::object(&attributes, "cell")
        .cloned()
        .unwrap_or_default();
    cell.remove("margin");
    output.push_str(&properties::wrap("tcPr", properties::cell(&cell)?));
    for (name, word) in [
        ("first_row", "firstRow"),
        ("last_row", "lastRow"),
        ("first_column", "firstCol"),
        ("last_column", "lastCol"),
    ] {
        if let Some(region) = property_values::object(&attributes, name) {
            output.push_str(&conditional(word, region, &declaration.properties, styles)?);
        }
    }
    if let Some(banding) = property_values::object(&attributes, "banding") {
        for (name, word) in [
            ("odd", "band1Horz"),
            ("even", "band2Horz"),
            ("odd_column", "band1Vert"),
            ("even_column", "band2Vert"),
        ] {
            if let Some(region) = property_values::object(banding, name) {
                output.push_str(&conditional(word, region, &declaration.properties, styles)?);
            }
        }
    }
    output.push_str("</w:style>");
    Ok(output)
}
fn paragraph(properties: &Properties, region: bool) -> Result<String, Error> {
    let mut paragraph = property_values::object(properties, "paragraph")
        .cloned()
        .unwrap_or_default();
    let alignment = if region {
        properties.get("alignment")
    } else {
        None
    }
    .or_else(|| property_values::object(properties, "cell").and_then(|cell| cell.get("alignment")));
    if let Some(alignment) = alignment {
        paragraph
            .entry("alignment".into())
            .or_insert_with(|| alignment.clone());
    }
    Ok(properties::wrap(
        "pPr",
        paragraph_properties::write_declaration(&paragraph)?,
    ))
}
fn run(text: &Properties, declaration: &Properties, styles: &Styles<'_>) -> Result<String, Error> {
    let mut properties = text.clone();
    if let Some(font) = declaration.get("font") {
        properties
            .entry("font".into())
            .or_insert_with(|| font.clone());
    }
    Ok(properties::wrap(
        "rPr",
        RunFormatting {
            properties,
            ..Default::default()
        }
        .write_declaration(styles)?,
    ))
}
fn conditional(
    word: &str,
    region: &Properties,
    declaration: &Properties,
    styles: &Styles<'_>,
) -> Result<String, Error> {
    let mut contents = paragraph(region, true)?;
    if let Some(text) = property_values::object(region, "text") {
        contents.push_str(&run(text, declaration, styles)?);
    }
    let mut cell: Properties = region
        .iter()
        .filter(|(name, _)| {
            [
                "borders",
                "fill",
                "margin",
                "no_wrap",
                "text_direction",
                "vertical_alignment",
            ]
            .contains(&name.as_str())
        })
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    if let Some(nested) = property_values::object(region, "cell") {
        cell.extend(nested.clone());
    }
    contents.push_str(&properties::wrap("tcPr", properties::cell(&cell)?));
    Ok(if contents.is_empty() {
        contents
    } else {
        format!("<w:tblStylePr w:type=\"{word}\">{contents}</w:tblStylePr>")
    })
}
fn table_properties(attributes: &Properties) -> Result<String, Error> {
    let mut output = String::new();
    let (rows, columns) = table_styles::bands(attributes);
    if rows {
        output.push_str("<w:tblStyleRowBandSize w:val=\"1\"/>");
    }
    if columns {
        output.push_str("<w:tblStyleColBandSize w:val=\"1\"/>");
    }
    if let Some(width) = attributes.get("width") {
        output.push_str(&format!(
            "<w:tblW w:w=\"{}\" w:type=\"dxa\"/>",
            values::twips(width)?
        ));
    }
    let alignment = property_values::text(attributes, "alignment");
    if !alignment.is_empty() {
        let alignment = if matches!(alignment.as_str(), "center" | "right") {
            alignment.as_str()
        } else {
            "left"
        };
        output.push_str(&format!("<w:jc w:val=\"{alignment}\"/>"));
    }
    for (name, word) in [("cell_spacing", "tblCellSpacing"), ("indent", "tblInd")] {
        if let Some(value) = attributes.get(name) {
            output.push_str(&format!(
                "<w:{word} w:w=\"{}\" w:type=\"dxa\"/>",
                values::twips(value)?
            ));
        }
    }
    let mut borders = property_values::object(attributes, "borders")
        .map(|borders| properties::borders("tblBorders", borders, true))
        .transpose()?
        .unwrap_or_default();
    let cell = property_values::object(attributes, "cell");
    if borders.is_empty()
        && let Some(all) = cell
            .and_then(|cell| property_values::object(cell, "borders"))
            .and_then(|borders| borders.get("all"))
    {
        borders = properties::borders(
            "tblBorders",
            &Properties::from([("all".into(), all.clone())]),
            true,
        )?;
    }
    output.push_str(&borders);
    if property_values::text(attributes, "layout") == "fixed" {
        output.push_str("<w:tblLayout w:type=\"fixed\"/>");
    }
    if let Some(margins) = cell.and_then(|cell| property_values::object(cell, "margin")) {
        output.push_str(&properties::margins("tblCellMar", margins)?);
    }
    Ok(output)
}
pub(super) fn look(properties: &Properties) -> String {
    let (rows, columns) = table_styles::bands(properties);
    let flags = [
        table_styles::region(properties, "first_row"),
        table_styles::region(properties, "last_row"),
        table_styles::region(properties, "first_column"),
        table_styles::region(properties, "last_column"),
    ];
    if !flags.iter().any(|flag| *flag) && !rows && !columns {
        return String::new();
    }
    format!(
        "<w:tblLook w:firstRow=\"{}\" w:lastRow=\"{}\" w:firstColumn=\"{}\" w:lastColumn=\"{}\" w:noHBand=\"{}\" w:noVBand=\"{}\"/>",
        u8::from(flags[0]),
        u8::from(flags[1]),
        u8::from(flags[2]),
        u8::from(flags[3]),
        u8::from(!rows),
        u8::from(!columns)
    )
}
pub(super) fn text_formatting(
    mut text: Properties,
    styles: &Styles<'_>,
) -> Result<RunFormatting, Error> {
    let mut formatting = RunFormatting::default();
    if let Some(style) = text.remove("style") {
        formatting.apply_character_style(&style.to_text(), styles)?;
    }
    if !text.contains_key("highlight")
        && let Some(value) = text.get("background").or_else(|| text.get("fill")).cloned()
    {
        text.insert("highlight".into(), value);
    }
    text.remove("background");
    text.remove("fill");
    formatting.properties.extend(text);
    Ok(formatting)
}
