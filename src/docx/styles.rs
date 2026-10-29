use super::package;
use crate::model::{Content, Document, ElementKind};

pub(super) fn write(document: &Document) -> String {
    if contains_code(&document.content) {
        package::DEFAULT_STYLES.replace("</w:styles>", "<w:style w:type=\"character\" w:styleId=\"Code\"><w:name w:val=\"Code\"/><w:rPr><w:rFonts w:ascii=\"Courier New\" w:hAnsi=\"Courier New\"/><w:sz w:val=\"20\"/><w:szCs w:val=\"20\"/></w:rPr></w:style></w:styles>")
    } else {
        package::DEFAULT_STYLES.into()
    }
}

fn contains_code(children: &[Content]) -> bool {
    children.iter().any(|child| match child {
        Content::Text(_) => false,
        Content::Element(element) => {
            element.kind == ElementKind::Code || contains_code(&element.children)
        }
    })
}
