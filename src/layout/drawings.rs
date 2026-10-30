pub(crate) mod geometry;
mod text;
use super::{LayoutError, PageOperation, PaintedPath, TextPage};
use crate::model::Element;

pub(super) struct Drawing {
    pub paths: Vec<PaintedPath>,
    pub text: TextPage,
    pub anchors: Vec<String>,
    pub origin: [f64; 2],
    pub rotation: f64,
    pub center: [f64; 2],
}
impl Drawing {
    pub fn prepare(
        element: &Element,
        styles: &super::LayoutStyleResolver<'_>,
        measurer: &mut crate::measurement::TextMeasurer<'_>,
    ) -> Result<Self, LayoutError> {
        let bounds = geometry::rectangle(&element.attributes);
        Ok(Self {
            paths: geometry::shape(&element.attributes),
            text: if bounds[2] > 0.0 && bounds[3] > 0.0 {
                text::prepare(element, styles, measurer)?
            } else {
                TextPage::default()
            },
            anchors: Vec::new(),
            origin: [bounds[0], bounds[1]],
            rotation: (crate::properties::number(&element.attributes, "rotation").unwrap_or(0.0)
                * 60000.0)
                .round()
                / 60000.0,
            center: [bounds[0] + bounds[2] / 2.0, bounds[1] + bounds[3] / 2.0],
        })
    }
    pub fn place(&self, page: &mut TextPage) {
        page.place_anchors(&self.anchors, self.origin);
        page.operations
            .extend(self.paths.iter().cloned().map(PageOperation::Path));
        if self.rotation != 0.0 {
            page.operations.push(PageOperation::PushRotation {
                degrees: self.rotation,
                center: self.center,
            });
        }
        let offset = page.lines.len();
        page.lines.extend(self.text.lines.clone());
        page.operations
            .extend(
                self.text
                    .operations
                    .iter()
                    .cloned()
                    .map(|operation| match operation {
                        PageOperation::Text(index) => PageOperation::Text(index + offset),
                        other => other,
                    }),
            );
        if self.rotation != 0.0 {
            page.operations.push(PageOperation::PopRotation);
        }
    }
}
