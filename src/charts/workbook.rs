//! Minimal editable chart workbook; no spreadsheet runtime is required.
use super::{Chart, Error};
use crate::xml::{HEADER, escape};
use std::{
    collections::BTreeMap,
    io::{Cursor, Write},
};
use zip::{ZipWriter, write::SimpleFileOptions};

pub(crate) const CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

pub(crate) fn write(chart: &Chart) -> Result<Vec<u8>, Error> {
    let strings = SharedStrings::new(chart);
    let worksheet = write_worksheet(chart, &strings);
    let shared_strings = strings.write()?;
    let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, contents) in [
        (
            "[Content_Types].xml",
            include_str!("workbook/content-types.xml"),
        ),
        ("_rels/.rels", include_str!("workbook/relationships.xml")),
        ("xl/workbook.xml", include_str!("workbook/workbook.xml")),
        (
            "xl/_rels/workbook.xml.rels",
            include_str!("workbook/workbook-relationships.xml"),
        ),
        ("xl/worksheets/sheet1.xml", worksheet.as_str()),
        ("xl/sharedStrings.xml", shared_strings.as_str()),
        ("xl/styles.xml", include_str!("workbook/styles.xml")),
        ("xl/theme/theme1.xml", include_str!("workbook/theme.xml")),
        ("docProps/core.xml", include_str!("workbook/core.xml")),
        ("docProps/app.xml", include_str!("workbook/app.xml")),
    ] {
        archive.start_file(name, options)?;
        archive.write_all(contents.as_bytes())?;
    }
    Ok(archive.finish()?.into_inner())
}
fn write_worksheet(chart: &Chart, strings: &SharedStrings<'_>) -> String {
    let columns = chart.series.len() + 1;
    let rows = chart.categories.len() + 1;
    let mut output = format!(
        r#"{HEADER}<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><dimension ref="A1:{}{rows}"/><sheetViews><sheetView tabSelected="1" workbookViewId="0"/></sheetViews><sheetFormatPr defaultRowHeight="15"/><cols><col min="1" max="1" width="10.7109375" customWidth="1"/></cols><sheetData><row r="1" spans="1:{columns}">"#,
        column_name(columns)
    );
    for (index, series) in chart.series.iter().enumerate() {
        output.push_str(&format!(
            r#"<c r="{}1" t="s"><v>{}</v></c>"#,
            column_name(index + 2),
            strings.indexes[series.name.as_str()]
        ));
    }
    output.push_str("</row>");
    for (index, category) in chart.categories.iter().enumerate() {
        let row = index + 2;
        output.push_str(&format!(
            r#"<row r="{row}" spans="1:{columns}"><c r="A{row}" s="1" t="s"><v>{}</v></c>"#,
            strings.indexes[category.as_str()]
        ));
        for (series_index, series) in chart.series.iter().enumerate() {
            let value = series.values.get(index).copied().unwrap_or_default();
            output.push_str(&format!(
                r#"<c r="{}{row}" s="1"><v>{value}</v></c>"#,
                column_name(series_index + 2)
            ));
        }
        output.push_str("</row>");
    }
    output.push_str(r#"</sheetData><pageMargins left="0.7" right="0.7" top="0.75" bottom="0.75" header="0.3" footer="0.3"/></worksheet>"#);
    output
}
struct SharedStrings<'chart> {
    values: Vec<&'chart str>,
    indexes: BTreeMap<&'chart str, usize>,
}
impl<'chart> SharedStrings<'chart> {
    fn new(chart: &'chart Chart) -> Self {
        let mut table = Self {
            values: Vec::new(),
            indexes: BTreeMap::new(),
        };
        for value in chart
            .categories
            .iter()
            .chain(chart.series.iter().map(|series| &series.name))
        {
            if !table.indexes.contains_key(value.as_str()) {
                table.indexes.insert(value, table.values.len());
                table.values.push(value);
            }
        }
        table
    }
    fn write(&self) -> Result<String, Error> {
        let count = self.values.len();
        let mut output = format!(
            r#"{HEADER}<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="{count}" uniqueCount="{count}">"#
        );
        for value in &self.values {
            output.push_str(&format!("<si><t>{}</t></si>", escape(value)?));
        }
        output.push_str("</sst>");
        Ok(output)
    }
}
pub(super) fn column_name(mut index: usize) -> String {
    let mut characters = Vec::new();
    while index > 0 {
        index -= 1;
        characters.push((b'A' + (index % 26) as u8) as char);
        index /= 26;
    }
    characters.into_iter().rev().collect()
}
pub(super) fn cell_reference(column: usize, row: usize) -> String {
    format!("Sheet1!${}${row}", column_name(column))
}
pub(super) fn range_reference(column: usize, count: usize) -> String {
    let first = cell_reference(column, 2);
    if count <= 1 {
        first
    } else {
        format!("{first}:${}${}", column_name(column), count + 1)
    }
}
