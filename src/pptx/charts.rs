//! Editable chart parts retain caches and an independently editable workbook.
use super::{
    Error, escape,
    package::{Relationship, relationships},
};
use crate::{
    charts::{self, Chart},
    model::Element,
    presentation::Theme,
};
pub(super) struct ChartPart {
    pub number: usize,
    pub xml: String,
    pub relationships: String,
    pub workbook: Vec<u8>,
}
pub(super) fn write(
    container: &Element,
    element: &Element,
    theme: &Theme,
    next_shape: &mut usize,
    relations: &mut Vec<Relationship>,
    parts: &mut Vec<ChartPart>,
) -> Result<String, Error> {
    let mut chart = Chart::from_element(element).map_err(|error| Error(error.to_string()))?;
    theme.apply_chart_palette(&mut chart);
    let number = parts.len() + 1;
    let workbook = charts::workbook::write(&chart).map_err(|error| Error(error.to_string()))?;
    let xml = charts::drawingml::write(
        &chart,
        &charts::drawingml::Options {
            part_number: number,
            label_baseline: 0,
        },
    )
    .map_err(|error| Error(error.to_string()))?;
    let relationships = relationships(&[Relationship {
        kind: "package",
        target: format!("../embeddings/chart{number}.xlsx"),
        external: false,
    }])?;
    parts.push(ChartPart {
        number,
        xml,
        relationships,
        workbook,
    });
    relations.push(Relationship {
        kind: "chart",
        target: format!("../charts/chart{number}.xml"),
        external: false,
    });
    let relationship = relations.len();
    let identifier = *next_shape;
    *next_shape += 1;
    let name = crate::properties::text(&container.attributes, "name");
    let name = escape(&if name.is_empty() {
        format!("Chart {identifier}")
    } else {
        name
    })?;
    let [horizontal, vertical, width, height] =
        crate::presentation::box_bounds(&container.attributes).map_err(Error)?;
    Ok(format!(
        r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="{identifier}" name="{name}"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr><p:xfrm><a:off x="{horizontal}" y="{vertical}"/><a:ext cx="{width}" cy="{height}"/></p:xfrm><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" r:id="rId{relationship}"/></a:graphicData></a:graphic></p:graphicFrame>"#
    ))
}
