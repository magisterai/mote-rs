use super::{Error, list_layout::ListLayout, xml};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct Numbering {
    definitions: Vec<ListLayout>,
    instances: Vec<usize>,
    continued: BTreeMap<(usize, String), usize>,
}

impl Numbering {
    pub fn for_bullets(&mut self, layout: ListLayout, identity: &str, level: u8) -> usize {
        let definition = match self
            .definitions
            .iter()
            .position(|candidate| *candidate == layout)
        {
            Some(index) => index + 1,
            None => {
                self.definitions.push(layout);
                self.definitions.len()
            }
        };
        let identity = if identity.is_empty() {
            format!("level:{level}")
        } else {
            identity.into()
        };
        let key = (definition, identity);
        if let Some(identifier) = self.continued.get(&key) {
            return *identifier;
        }
        self.instances.push(definition);
        let identifier = self.instances.len();
        self.continued.insert(key, identifier);
        identifier
    }

    pub fn write(&self) -> Result<String, Error> {
        let mut content = String::new();
        for (index, layout) in self.definitions.iter().enumerate() {
            content.push_str(&format!(
                "<w:abstractNum w:abstractNumId=\"{}\">",
                index + 1
            ));
            for level in 0..9 {
                let left = layout.left_at_level(level)?;
                content.push_str(&format!("<w:lvl w:ilvl=\"{level}\"><w:start w:val=\"1\"/><w:numFmt w:val=\"bullet\"/><w:lvlText w:val=\"•\"/><w:lvlJc w:val=\"left\"/><w:pPr><w:spacing w:after=\"{}\"/><w:ind w:left=\"{left}\" w:hanging=\"{}\"/></w:pPr></w:lvl>", layout.spacing_after, layout.hanging));
            }
            content.push_str("</w:abstractNum>");
        }
        for (index, definition) in self.instances.iter().enumerate() {
            content.push_str(&format!(
                "<w:num w:numId=\"{}\"><w:abstractNumId w:val=\"{definition}\"/></w:num>",
                index + 1
            ));
        }
        Ok(xml::word_part("numbering", &content))
    }
}

pub(super) fn paragraph_reference(identifier: usize, level: u8) -> String {
    format!("<w:numPr><w:ilvl w:val=\"{level}\"/><w:numId w:val=\"{identifier}\"/></w:numPr>")
}
