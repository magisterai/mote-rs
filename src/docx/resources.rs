use super::{Error, package, xml};

/// Relationships accumulated in content order during one package emission.
#[derive(Default)]
pub(super) struct Resources {
    relationships: Vec<Relationship>,
}
struct Relationship {
    kind: &'static str,
    target: String,
    external: bool,
}
impl Resources {
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
