pub(crate) mod geometry;
mod text;
use super::{LayoutError, PageOperation, TextPage};
use crate::model::{Content, Element, ElementKind};

/// A shape or a group of a document, painted about the corner of its frame.
pub(super) struct Drawing {
    content: TextPage,
    /// The warnings that a frame cuts off its text, each less the page.
    pub cut_off_text: Vec<CutOffText>,
    pub anchors: Vec<String>,
    /// The first corner of the frame of a fixed shape, at which its anchors lie.
    origin: [f64; 2],
    /// The room that the drawing takes in the text, or nothing where the
    /// drawing is fixed on its page.
    pub flow: Option<Flow>,
}
pub(super) struct Flow {
    pub width: f64,
    pub height: f64,
    pub alignment: String,
}
impl Flow {
    /// The space that Word sets beneath the paragraph of an inline drawing.
    pub const AFTER: f64 = 8.0;
    fn left(&self, column_left: f64, column_width: f64) -> f64 {
        column_left
            + match self.alignment.as_str() {
                "center" => (column_width - self.width) / 2.0,
                "right" => column_width - self.width,
                _ => 0.0,
            }
    }
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
        let mut content = TextPage::default();
        let mut cut_off_text = Vec::new();
        paint(element, styles, measurer, &mut content, &mut cut_off_text)?;
        let points = |units: [i64; 2]| units.map(|value| value as f64 / 12700.0);
        let (origin, size) = if element.kind == ElementKind::Group {
            (
                points(crate::drawings::group_corner(element)),
                points(crate::drawings::group_size(element)),
            )
        } else {
            let bounds = geometry::rectangle(&element.attributes);
            ([bounds[0], bounds[1]], [bounds[2], bounds[3]])
        };
        Ok(Self {
            content,
            cut_off_text,
            anchors: Vec::new(),
            origin,
            flow: crate::drawings::flows(element).then(|| Flow {
                width: size[0],
                height: size[1],
                // The alignment of a shape is that of its text.
                alignment: if element.kind == ElementKind::Group {
                    crate::properties::text(&element.attributes, "alignment")
                } else {
                    String::new()
                },
            }),
        })
    }
    /// Paints a fixed drawing at the place on the page that its source states.
    pub fn place(&self, page: &mut TextPage) {
        page.place_anchors(&self.anchors, self.origin);
        page.append_content(self.content.clone());
    }
    /// Paints a drawing of the text with the corner of its frame at the top of
    /// its room in a column.
    pub fn place_in_flow(
        &self,
        flow: &Flow,
        page: &mut TextPage,
        column_left: f64,
        column_width: f64,
        top: f64,
    ) {
        let corner = [flow.left(column_left, column_width), top];
        page.place_anchors(&self.anchors, corner);
        page.append_content_at(self.content.clone(), corner);
    }
}

/// Paints an object about the corner of its container: a shape at the place of
/// its position, and the objects of a group about the corner of the group.
fn paint(
    element: &Element,
    styles: &super::LayoutStyleResolver<'_>,
    measurer: &mut crate::measurement::TextMeasurer<'_>,
    page: &mut TextPage,
    cut_off_text: &mut Vec<CutOffText>,
) -> Result<(), LayoutError> {
    let rotation = (crate::properties::number(&element.attributes, "rotation").unwrap_or(0.0)
        * 60000.0)
        .round()
        / 60000.0;
    let rotate = |page: &mut TextPage, center: [f64; 2]| {
        if rotation != 0.0 {
            page.operations.push(PageOperation::PushRotation {
                degrees: rotation,
                center,
            });
        }
    };
    match element.kind {
        ElementKind::Group => {
            let mut objects = TextPage::default();
            for child in &element.children {
                if let Content::Element(child) = child {
                    paint(child, styles, measurer, &mut objects, cut_off_text)?;
                }
            }
            let points = |units: [i64; 2]| units.map(|value| value as f64 / 12700.0);
            let corner = points(crate::drawings::group_corner(element));
            let size = points(crate::drawings::group_size(element));
            rotate(page, [corner[0] + size[0] / 2.0, corner[1] + size[1] / 2.0]);
            page.append_content_at(objects, corner);
        }
        ElementKind::Line => {
            page.operations
                .extend(super::presentation::shapes::connector(&element.attributes));
            return Ok(());
        }
        ElementKind::Shape => {
            let bounds = geometry::rectangle(&element.attributes);
            let fitted = if bounds[2] > 0.0 && bounds[3] > 0.0 {
                text::prepare(element, styles, measurer)?
            } else {
                super::drawing_text::FittedText::empty()
            };
            if fitted.is_cut_off() {
                cut_off_text.push(CutOffText {
                    described: super::overflow::document_shape(element),
                    fit: super::overflow::text_fit(element),
                });
            }
            page.operations.extend(
                geometry::shape(&element.attributes)
                    .into_iter()
                    .map(PageOperation::Path),
            );
            rotate(
                page,
                [bounds[0] + bounds[2] / 2.0, bounds[1] + bounds[3] / 2.0],
            );
            page.append_content(fitted.page);
        }
        // A document sets no box; the support warnings report one in a group.
        _ => return Ok(()),
    }
    if rotation != 0.0 {
        page.operations.push(PageOperation::PopRotation);
    }
    Ok(())
}
