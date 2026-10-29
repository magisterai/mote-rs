use super::{Error, paragraph_properties, runs, styles::Styles, whitespace, xml};
use crate::model::{Content, Document, ElementKind};

pub(super) fn write_document(document: &Document, styles: &Styles<'_>) -> Result<String, Error> {
    let mut body = String::new();
    for content in &document.content {
        let Content::Element(element) = content else {
            return Err(Error::Unsupported(
                "top-level text outside a paragraph".into(),
            ));
        };
        if element.kind != ElementKind::Paragraph {
            return Err(Error::Unsupported(format!(
                "{} content",
                element.kind.as_text()
            )));
        }
        body.push_str("<w:p>");
        let mut attributes = element.attributes.clone();
        let mut properties = String::new();
        if let Some(name) = attributes
            .remove("style")
            .map(|value| value.to_text())
            .filter(|name| !name.is_empty())
        {
            properties.push_str(&format!(
                "<w:pStyle w:val=\"{}\"/>",
                xml::escape(styles.identifier(&name)?)?
            ));
        }
        properties.push_str(&paragraph_properties::write(&attributes)?);
        if !properties.is_empty() {
            body.push_str(&format!("<w:pPr>{properties}</w:pPr>"));
        }
        body.push_str(&runs::write_inlines(
            &whitespace::trim_line_edges(&element.children),
            styles,
        )?);
        body.push_str("</w:p>");
    }
    body.push_str("<w:sectPr><w:pgSz w:w=\"12240\" w:h=\"15840\"/><w:pgMar w:top=\"1440\" w:right=\"1440\" w:bottom=\"1440\" w:left=\"1440\" w:header=\"720\" w:footer=\"720\" w:gutter=\"0\"/></w:sectPr>");
    Ok(format!(
        r#"{}<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="w14 wps"><w:body>{body}</w:body></w:document>"#,
        xml::HEADER
    ))
}
