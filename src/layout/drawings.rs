pub(crate) mod geometry;
mod text;
use super::{LayoutError, PageOperation, PaintedPath, TextPage};
use crate::model::Element;

pub(super) struct Drawing {
    pub paths: Vec<PaintedPath>,
    pub text: TextPage,
    /// The warning that the frame cuts off the text, less the page, where it
    /// does.
    pub cut_off_text: Option<CutOffText>,
    pub anchors: Vec<String>,
    pub origin: [f64; 2],
    pub rotation: f64,
    pub center: [f64; 2],
}
pub(super) struct CutOffText {
    described: String,
    fit: String,
}
impl CutOffText {
    pub fn warning(&self, page_number: usize) -> String {
        super::overflow::frame_text(
            &format!("{} on page {page_number}", self.described),
            &self.fit,
        )
    }
}
impl Drawing {
    pub fn prepare(
        element: &Element,
        styles: &super::LayoutStyleResolver<'_>,
        measurer: &mut crate::measurement::TextMeasurer<'_>,
    ) -> Result<Self, LayoutError> {
        let bounds = geometry::rectangle(&element.attributes);
        let fitted = if bounds[2] > 0.0 && bounds[3] > 0.0 {
            text::prepare(element, styles, measurer)?
        } else {
            super::drawing_text::FittedText::empty()
        };
        Ok(Self {
            paths: geometry::shape(&element.attributes),
            cut_off_text: fitted.is_cut_off().then(|| CutOffText {
                described: super::overflow::document_shape(element),
                fit: super::overflow::text_fit(element),
            }),
            text: fitted.page,
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
        page.append_content(self.text.clone());
        if self.rotation != 0.0 {
            page.operations.push(PageOperation::PopRotation);
        }
    }
}
