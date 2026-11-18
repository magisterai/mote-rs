//! Reserve page-bottom note space at the first placed reference line.
use super::NoteLayout;
use crate::layout::{FlowCursor, LayoutError, LinePlacement, PageGeometry, TextPage};
use crate::layout::{
    blocks::PreparedParagraph,
    columns::ColumnSeparators,
    document::{
        LeadingSpace, ensure_space, fitting_height, next_area, page_content_bottom, pass_block,
        place_leading,
    },
    painting::ParagraphPainting,
    sections::FlowBoundary,
    tolerances::{AREA_START_TOLERANCE, FIT_TOLERANCE},
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
        // The foot of a page belongs to the section that owns the page, which
        // is an earlier section if a continuous section shares the page.
        let foot = |pages: &[TextPage], page_number: usize| {
            pages[page_number - 1]
                .geometry
                .unwrap_or(geometry)
                .content_bottom()
        };
        let paragraph = &prepared.text;
        let height = fitting_height(&paragraph.lines);
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
            && cursor.y + leading + height
                > self.body_bottom(&candidate, foot(pages, cursor.page_number)) + FIT_TOLERANCE
        {
            warnings.push("native footnote reservation takes precedence over paragraph keep and widow/orphan preferences".into());
        }
        let first_height = paragraph.lines[0].height - paragraph.lines[0].trailing_space;
        if leading > 0.0 {
            place_leading(
                cursor,
                pages,
                columns,
                geometry,
                boundary,
                LeadingSpace {
                    height: leading,
                    fit_height: leading + first_height,
                },
            );
        }
        let available =
            page_content_bottom(pages, geometry, boundary, cursor.page_number) - cursor.current_top;
        let required =
            if paragraph.style.keep_together && leading + height <= available + FIT_TOLERANCE {
                minimum.max(height)
            } else {
                minimum
            };
        if required > 0.0 && cursor.y > cursor.current_top + AREA_START_TOLERANCE {
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
                ensure_space(
                    cursor,
                    pages,
                    columns,
                    geometry,
                    boundary,
                    line.height - line.trailing_space,
                );
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
                    let bottom = self.body_bottom(&candidate, foot(pages, cursor.page_number));
                    let fits =
                        cursor.y + line.height - line.trailing_space <= bottom + FIT_TOLERANCE;
                    let fresh = cursor.y <= geometry.margin_top + AREA_START_TOLERANCE;
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
            let mut frame = paragraph.line_frame(index);
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
            pass_block(cursor, columns, geometry, line.height);
        }
        Ok(())
    }
}
