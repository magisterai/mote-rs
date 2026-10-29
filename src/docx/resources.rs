use super::{Error, package, xml};

/// Relationships accumulated in content order during one package emission.
pub(super) struct Resources {
    relationships: Vec<Relationship>,
    pub anchors: crate::references::Catalog,
    pub next_bookmark: usize,
    pub pending_bookmarks: Vec<crate::model::Element>,
    relationship_offset: usize,
    pub stories: Vec<super::sections::StoryPart>,
    pub warnings: Vec<String>,
    pub active_text: super::run_properties::RunFormatting,
    pub images: crate::media::ImageCatalog,
    pub media: Vec<(String, crate::media::ImageAsset)>,
}
pub(super) struct Relationship {
    kind: &'static str,
    target: String,
    external: bool,
}
impl Default for Resources {
    fn default() -> Self {
        Self {
            relationships: Vec::new(),
            anchors: Default::default(),
            next_bookmark: 0,
            pending_bookmarks: Vec::new(),
            relationship_offset: 4,
            stories: Vec::new(),
            warnings: Vec::new(),
            active_text: Default::default(),
            images: Default::default(),
            media: Vec::new(),
        }
    }
}
impl Resources {
    pub fn story(&mut self, kind: &'static str, name: &str) -> String {
        self.relationships.push(Relationship {
            kind,
            target: name.into(),
            external: false,
        });
        format!("rId{}", self.relationships.len() + self.relationship_offset)
    }
    pub fn enter_story(&mut self) -> Vec<Relationship> {
        self.relationship_offset = 0;
        std::mem::take(&mut self.relationships)
    }
    pub fn leave_story(&mut self, parent: Vec<Relationship>) -> Result<Option<String>, Error> {
        let result = if self.relationships.is_empty() {
            Ok(None)
        } else {
            self.relationships().map(Some)
        };
        self.relationships = parent;
        self.relationship_offset = 4;
        result
    }
    pub fn image(&mut self, asset: crate::media::ImageAsset) -> (String, String, usize) {
        let identifier = self.media.len() + 1;
        let name = format!("media/image{identifier}{}", asset.extension);
        self.media.push((name.clone(), asset));
        self.relationships.push(Relationship {
            kind: "image",
            target: name.clone(),
            external: false,
        });
        (
            format!("rId{}", self.relationships.len() + self.relationship_offset),
            name,
            identifier,
        )
    }
    pub fn hyperlink(&mut self, target: &str) -> String {
        self.relationships.push(Relationship {
            kind: "hyperlink",
            target: target.into(),
            external: true,
        });
        format!("rId{}", self.relationships.len() + self.relationship_offset)
    }
    pub fn relationships(&self) -> Result<String, Error> {
        let mut output = if self.relationship_offset == 4 {
            package::DOCUMENT_RELATIONSHIPS.replace("</Relationships>", "")
        } else {
            format!(
                r#"{}<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#,
                xml::HEADER
            )
        };
        for (index, relationship) in self.relationships.iter().enumerate() {
            let mode = if relationship.external {
                " TargetMode=\"External\""
            } else {
                ""
            };
            output.push_str(&format!("<Relationship Id=\"rId{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/{}\" Target=\"{}\"{mode}/>", index + 1 + self.relationship_offset, relationship.kind, xml::escape(&relationship.target)?));
        }
        output.push_str("</Relationships>");
        Ok(output)
    }
}
