//! Reserve page-bottom note space at the first placed reference line.
use super::NoteLayout;
use crate::layout::{FlowCursor, LayoutError, LinePlacement, PageGeometry, TextPage};
use crate::layout::{
    blocks::PreparedParagraph,
    columns::ColumnSeparators,
    document::{ensure_space, next_area, page_content_bottom},
    painting::ParagraphPainting,
    sections::FlowBoundary,
};

pub(in crate::layout) struct NoteFlowContext<'scope> {
    pub pages: &'scope mut Vec<TextPage>,
    pub cursor: &'scope mut FlowCursor,
    pub columns: &'scope mut ColumnSeparators,
    pub geometry: PageGeometry,
    pub boundary: Option<FlowBoundary>,
    pub warnings: &'scope mut Vec<String>,
}
impl NoteLayout {
    pub(in crate::layout) fn place_paragraph(
        &mut self,
        prepared: &PreparedParagraph,
        leading: f64,
        minimum: f64,
        context: NoteFlowContext<'_>,
        painting: &mut ParagraphPainting,
    ) -> Result<(), LayoutError> {
        let NoteFlowContext {
            pages,
            cursor,
            columns,
            geometry,
            boundary,
            warnings,
        } = context;
        let paragraph = &prepared.text;
        let height: f64 = paragraph.lines.iter().map(|line| line.height).sum();
        let unassigned: Vec<_> = paragraph
            .lines
            .iter()
            .flat_map(|line| &line.fragments)
            .filter_map(|fragment| fragment.footnote_number)
            .filter(|number| !self.assigned.contains(number))
            .collect();
        let mut candidate = self
            .pages
            .get(&cursor.page_number)
            .cloned()
            .unwrap_or_default();
        for number in unassigned {
            if !candidate.contains(&number) {
                candidate.push(number);
            }
        }
        if (minimum > 0.0
            || paragraph.lines.len() > 1
                && (paragraph.style.keep_together || paragraph.style.widow_control))
            && cursor.y + leading + height > self.body_bottom(&candidate, geometry) + 0.001
        {
            warnings.push("native footnote reservation takes precedence over paragraph keep and widow/orphan preferences".into());
        }
        let first_height = paragraph.lines[0].height;
        if leading > 0.0 {
            let previous_page = cursor.page_number;
            ensure_space(
                cursor,
                pages,
                columns,
                geometry,
                boundary,
                leading + first_height,
            );
            if cursor.page_number == previous_page {
                cursor.y += leading;
            }
        }
        let available =
            page_content_bottom(pages, geometry, boundary, cursor.page_number) - cursor.current_top;
        let required = if paragraph.style.keep_together && leading + height <= available + 0.001 {
            minimum.max(height)
        } else {
            minimum
        };
        if required > 0.0 && cursor.y > cursor.current_top + 0.001 {
            ensure_space(cursor, pages, columns, geometry, boundary, required);
        }
        for (index, line) in paragraph.lines.iter().enumerate() {
            let numbers: Vec<_> = line
                .fragments
                .iter()
                .filter_map(|fragment| fragment.footnote_number)
                .filter(|number| !self.assigned.contains(number))
                .collect();
            if numbers.is_empty() {
                ensure_space(cursor, pages, columns, geometry, boundary, line.height);
            } else {
                // The second attempt is always a fresh frame. An oversized note is
                // retained visibly with a warning rather than causing a retry loop.
                for attempt in 0..2 {
                    let mut candidate = self
                        .pages
                        .get(&cursor.page_number)
                        .cloned()
                        .unwrap_or_default();
                    for number in &numbers {
                        if !candidate.contains(number) {
                            candidate.push(*number);
                        }
                    }
                    let bottom = self.body_bottom(&candidate, geometry);
                    let fits = cursor.y + line.height <= bottom + 0.001;
                    let fresh = cursor.y <= geometry.margin_top + 0.001;
                    if fits || fresh || attempt == 1 {
                        if !fits {
                            warnings.push("native oversized footnote exceeds its page-bottom reservation on a fresh page".into());
                        }
                        for number in &numbers {
                            self.assigned.insert(*number);
                        }
                        self.pages.insert(cursor.page_number, candidate);
                        pages[cursor.page_number - 1].note_body_bottom = Some(bottom);
                        break;
                    }
                    next_area(cursor, pages, columns, geometry);
                }
            }
            let mut frame = if index == 0 {
                paragraph.first_frame
            } else {
                paragraph.continuation_frame
            };
            frame.x += geometry.column_left(cursor.column_index);
            painting.place_line(
                &mut pages[cursor.page_number - 1],
                prepared,
                LinePlacement {
                    line_index: index,
                    page_number: cursor.page_number,
                    column_index: cursor.column_index,
                    frame,
                    top: cursor.y,
                    height: line.height,
                },
            );
            cursor.y += line.height;
            cursor.body_started = true;
            columns.note(cursor, geometry);
        }
        Ok(())
    }
}
