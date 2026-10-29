use super::{Error, package, xml};

/// Relationships accumulated in content order during one package emission.
#[derive(Default)]
pub(super) struct Resources {
    relationships: Vec<Relationship>,
    pub warnings: Vec<String>,
    pub active_text: super::run_properties::RunFormatting,
    pub images: crate::media::ImageCatalog,
    pub media: Vec<(String, crate::media::ImageAsset)>,
}
struct Relationship {
    kind: &'static str,
    target: String,
    external: bool,
}
impl Resources {
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
            format!("rId{}", self.relationships.len() + 4),
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
        format!("rId{}", self.relationships.len() + 4)
    }
    pub fn relationships(&self) -> Result<String, Error> {
        let mut output = package::DOCUMENT_RELATIONSHIPS.replace("</Relationships>", "");
        for (index, relationship) in self.relationships.iter().enumerate() {
            let mode = if relationship.external {
                " TargetMode=\"External\""
            } else {
                ""
            };
            output.push_str(&format!("<Relationship Id=\"rId{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/{}\" Target=\"{}\"{mode}/>", index + 5, relationship.kind, xml::escape(&relationship.target)?));
        }
        output.push_str("</Relationships>");
        Ok(output)
    }
}
