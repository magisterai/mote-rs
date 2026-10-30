use super::{Error, resources::Resources, xml};
use crate::{
    charts::{self, Chart},
    model::Element,
};

pub(super) struct ChartPart {
    pub name: String,
    pub content: String,
    pub relationships: String,
    pub workbook_name: String,
    pub workbook: Vec<u8>,
}
pub(super) fn write(element: &Element, resources: &mut Resources) -> Result<String, Error> {
    let chart = Chart::from_element(element)?;
    let number = resources.charts.len() + 1;
    let name = format!("charts/chart{number}.xml");
    let workbook_name = format!("embeddings/chart{number}.xlsx");
    let options = charts::drawingml::Options {
        part_number: number,
        label_baseline: -10000,
    };
    let part = ChartPart {
        content: charts::drawingml::write(&chart, &options)?,
        relationships: format!(
            r#"{}<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" Target="../{workbook_name}"/></Relationships>"#,
            xml::HEADER
        ),
        workbook: charts::workbook::write(&chart)?,
        name: name.clone(),
        workbook_name,
    };
    let relationship = resources.story("chart", &name);
    resources.charts.push(part);
    let identifier = resources.next_drawing();
    let name = super::values::first_text(&element.attributes, &["name"])
        .unwrap_or_else(|| format!("Chart {identifier}"));
    let paragraph = super::paragraph_properties::write(&Default::default())?;
    let paragraph = if paragraph.is_empty() {
        paragraph
    } else {
        format!("<w:pPr>{paragraph}</w:pPr>")
    };
    let (bookmarks, identifiers) = super::references::start_pending(resources)?;
    let output = format!(
        r#"<w:p>{paragraph}{bookmarks}<w:r><w:drawing><wp:inline><wp:extent cx="{}" cy="{}"/><wp:docPr id="{identifier}" name="{}"/><wp:cNvGraphicFramePr/><a:graphic><a:graphicData uri="{}"><c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" r:id="{relationship}"/></a:graphicData></a:graphic></wp:inline></w:drawing></w:r>{}</w:p>"#,
        chart.width,
        chart.height,
        xml::escape(&name)?,
        charts::drawingml::GRAPHIC_DATA_URI,
        super::references::end_pending(&identifiers)
    );
    Ok(output)
}
impl From<charts::Error> for Error {
    fn from(error: charts::Error) -> Self {
        match error {
            charts::Error::Io(error) => Self::Io(error),
            charts::Error::Archive(error) => Self::Archive(error),
            _ => Self::InvalidValue(error.to_string()),
        }
    }
}
