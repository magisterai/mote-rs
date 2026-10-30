mod geometry;
use super::{LayoutError, PageOperation, PaintedPath, TextPage};
use crate::model::Element;

pub(super) struct Drawing {
    pub paths: Vec<PaintedPath>,
    pub anchors: Vec<String>,
    pub origin: [f64; 2],
}
impl Drawing {
    pub fn prepare(element: &Element) -> Result<Self, LayoutError> {
        if !element.children.is_empty() {
            return Err(LayoutError::Unsupported(
                "native shape text is not yet supported".into(),
            ));
        }
        let bounds = geometry::rectangle(&element.attributes);
        Ok(Self {
            paths: geometry::shape(&element.attributes),
            anchors: Vec::new(),
            origin: [bounds[0], bounds[1]],
        })
    }
    pub fn place(&self, page: &mut TextPage) {
        page.place_anchors(&self.anchors, self.origin);
        page.operations
            .extend(self.paths.iter().cloned().map(PageOperation::Path));
    }
}
