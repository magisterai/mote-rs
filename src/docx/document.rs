use super::{
    Error, block_quotes, configuration, controls::Controls, headings, lists, numbering::Numbering,
    paragraphs, styles::Styles, xml,
};
use crate::model::{Content, Document, ElementKind};

#[derive(Default)]
pub(super) struct DocumentState {
    pub numbering: Numbering,
    pub controls: Controls,
}

pub(super) fn write_document(
    document: &Document,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let mut body = String::new();
    for (index, content) in document.content.iter().enumerate() {
        let Content::Element(element) = content else {
            return Err(Error::Unsupported(
                "top-level text outside a paragraph".into(),
            ));
        };
        let mut numbering_reference = None;
        let attributes = match element.kind {
            ElementKind::Paragraph => element.attributes.clone(),
            ElementKind::Heading => {
                let level = headings::level(element)? - 1;
                numbering_reference = state
                    .numbering
                    .for_heading(element, level)?
                    .map(|identifier| (identifier, level));
                headings::attributes(element, index == 0)?
            }
            ElementKind::Blockquote => {
                body.push_str(&block_quotes::write(element, styles)?);
                continue;
            }
            ElementKind::List | ElementKind::OrderedList => {
                body.push_str(&lists::write(element, 0, styles, state)?);
                continue;
            }
            _ => {
                return Err(Error::Unsupported(format!(
                    "{} content",
                    element.kind.as_text()
                )));
            }
        };
        body.push_str(&paragraphs::write(
            &attributes,
            &element.children,
            styles,
            numbering_reference,
        )?);
    }
    body.push_str(&configuration::section_properties(document)?);
    Ok(format!(
        r#"{}<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="w14 wps"><w:body>{body}</w:body></w:document>"#,
        xml::HEADER
    ))
}
