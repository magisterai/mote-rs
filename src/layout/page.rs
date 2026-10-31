//! Painter-neutral page plans and composition of prepared content.
use super::{PageGeometry, PageOperation, TextLine};

#[cfg(test)]
mod tests;

/// Absolute text positions. Painting and non-text block layout are separate stages.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedTextLine {
    pub top: f64,
    pub line: TextLine,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextPage {
    /// Section geometry of the physical page; absent on manually assembled plans.
    pub geometry: Option<PageGeometry>,
    pub section_index: usize,
    pub page_number: usize,
    pub lines: Vec<PlacedTextLine>,
    pub anchors: std::collections::BTreeMap<String, [f64; 2]>,
    pub operations: Vec<PageOperation>,
    /// Body-flow boundary after reserving space for notes on this page.
    pub note_body_bottom: Option<f64>,
}
impl TextPage {
    /// Append drawing content without changing the receiving page's geometry,
    /// numbering or flow state. Text operations retain their local line identity.
    pub(crate) fn append_content(&mut self, page: Self) {
        let offset = self.lines.len();
        self.lines.extend(page.lines);
        self.anchors.extend(page.anchors);
        self.operations.extend(
            page.operations
                .into_iter()
                .map(|operation| match operation {
                    PageOperation::Text(index) => PageOperation::Text(index + offset),
                    other => other,
                }),
        );
    }

    /// Translate a prepared fragment before appending it. Clip rectangles and
    /// rotation centers move with the content; extents and stroke widths do not.
    pub(super) fn append_content_at(&mut self, mut content: Self, origin: [f64; 2]) {
        for line in &mut content.lines {
            line.top += origin[1];
            line.line.translate(origin[0], origin[1]);
        }
        for position in content.anchors.values_mut() {
            position[0] += origin[0];
            position[1] += origin[1];
        }
        for operation in &mut content.operations {
            operation.translate(origin);
        }
        self.append_content(content);
    }

    pub(super) fn place_anchors(&mut self, names: &[String], position: [f64; 2]) {
        for name in names {
            self.anchors.entry(name.clone()).or_insert(position);
        }
    }
    pub(super) fn push_line(&mut self, line: PlacedTextLine) {
        for fragment in &line.line.fragments {
            self.place_anchors(&fragment.anchors, [fragment.x, line.top]);
        }
        self.operations.push(PageOperation::Text(self.lines.len()));
        self.lines.push(line);
    }
    pub fn text_lines(&self) -> impl Iterator<Item = &TextLine> {
        self.lines
            .iter()
            .map(|placed| &placed.line)
            .chain(
                self.operations
                    .iter()
                    .filter_map(|operation| match operation {
                        PageOperation::ListMarker(marker) => marker.text.as_ref(),
                        _ => None,
                    }),
            )
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentTextPlan {
    pub geometry: PageGeometry,
    pub pages: Vec<TextPage>,
    /// Observable layout fallbacks shared by every painter.
    pub warnings: Vec<String>,
}
