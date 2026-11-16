//! Editable placeholders and cached values for slide footer fields.
use super::{Error, escape};
use crate::presentation::fields::{FieldKind, SlideMetadata};

pub(super) fn write(
    metadata: &SlideMetadata,
    dimensions: [i64; 2],
    slide_number: usize,
    next_shape: &mut usize,
) -> Result<String, Error> {
    let mut output = String::new();
    for field in metadata.fields(slide_number) {
        let identifier = *next_shape;
        *next_shape += 1;
        let (name, placeholder, size, index, alignment, field_type) = match field.kind {
            FieldKind::Footer => ("Footer", "ftr", "quarter", 11, "l", None),
            FieldKind::Date { automatic } => (
                "Date",
                "dt",
                "half",
                10,
                "ctr",
                automatic.then_some("datetime4"),
            ),
            FieldKind::SlideNumber => (
                "Slide Number",
                "sldNum",
                "quarter",
                12,
                "r",
                Some("slidenum"),
            ),
        };
        let [horizontal, vertical, width, height] = field.editable_bounds(dimensions);
        let text = escape(&field.text)?;
        let style = &metadata.text;
        let mut properties = format!(
            r#"<a:rPr lang="en-US" sz="{}""#,
            (style.size * 100.0).round() as i64
        );
        for (name, stated) in [("b", style.bold), ("i", style.italic)] {
            if stated {
                properties.push_str(&format!(r#" {name}="1""#));
            }
        }
        properties.push_str(&format!(
            r#"><a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#,
            style.color
        ));
        if !style.font.is_empty() {
            properties.push_str(&format!(
                r#"<a:latin typeface="{}"/>"#,
                escape(&style.font)?
            ));
        }
        properties.push_str("</a:rPr>");
        let content = if let Some(field_type) = field_type {
            let field_identifier =
                format!("{{{slide_number:08X}-0000-0000-0000-{identifier:012X}}}");
            format!(
                r#"<a:fld id="{field_identifier}" type="{field_type}">{properties}<a:t>{text}</a:t></a:fld>"#
            )
        } else {
            format!("<a:r>{properties}<a:t>{text}</a:t></a:r>")
        };
        output.push_str(&format!(r#"<p:sp><p:nvSpPr><p:cNvPr id="{identifier}" name="{name}"/><p:cNvSpPr txBox="1"/><p:nvPr><p:ph type="{placeholder}" sz="{size}" idx="{index}"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="{horizontal}" y="{vertical}"/><a:ext cx="{width}" cy="{height}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/><a:ln><a:noFill/></a:ln></p:spPr><p:txBody><a:bodyPr wrap="square" anchor="b"/><a:lstStyle/><a:p><a:pPr algn="{alignment}"/>{content}<a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp>"#));
    }
    Ok(output)
}
