//! Paint note stories into the same ordered page plan as body content.
use super::{BOTTOM_PADDING, ENDNOTE_BAND, NoteLayout, PARAGRAPH_GAP, SEPARATOR_BAND};
use crate::layout::{
    FlowCursor, LayoutError, PageGeometry, PageOperation, PlacedTextLine, Rule, RuleRole, TextPage,
};
use crate::layout::{
    columns::ColumnSeparators,
    document::{next_area, page_content_bottom},
};

impl NoteLayout {
    pub(in crate::layout) fn paint_footnotes(
        &self,
        pages: &mut [TextPage],
        geometry: PageGeometry,
    ) {
        for (number, notes) in &self.pages {
            let page = &mut pages[number - 1];
            let mut top = geometry.content_bottom() - BOTTOM_PADDING - self.stack_height(notes);
            separator(
                page,
                geometry.margin_left,
                top - SEPARATOR_BAND,
                (geometry.width - geometry.margin_left - geometry.margin_right).min(144.0),
            );
            for (index, number) in notes.iter().enumerate() {
                if index > 0 {
                    top += PARAGRAPH_GAP;
                }
                for line in &self.footnotes[number - 1].lines {
                    let mut line = line.clone();
                    line.translate(geometry.margin_left, top);
                    top += line.height;
                    page.push_line(PlacedTextLine {
                        top: top - line.height,
                        line,
                    });
                }
            }
        }
    }
    pub(in crate::layout) fn place_endnotes(
        &self,
        pages: &mut Vec<TextPage>,
        cursor: &mut FlowCursor,
        columns: &mut ColumnSeparators,
        geometry: PageGeometry,
        pending_spacing: f64,
        warnings: &mut Vec<String>,
    ) -> Result<(), LayoutError> {
        if self.endnotes.is_empty() {
            return Ok(());
        }
        let width = geometry.width - geometry.margin_left - geometry.margin_right;
        let first_height = self.endnotes[0]
            .lines
            .first()
            .map(|line| line.height)
            .unwrap_or(0.0);
        let required = ENDNOTE_BAND + first_height;
        if pending_spacing > 0.0 {
            let previous_page = cursor.page_number;
            if cursor.y + pending_spacing + required
                > page_content_bottom(pages, geometry, None, cursor.page_number)
            {
                next_area(cursor, pages, columns, geometry);
            }
            if cursor.page_number == previous_page {
                cursor.y += pending_spacing;
            }
        }
        if cursor.y + required > page_content_bottom(pages, geometry, None, cursor.page_number) {
            next_area(cursor, pages, columns, geometry);
        }
        separator(
            &mut pages[cursor.page_number - 1],
            geometry.margin_left,
            cursor.y,
            width.min(144.0),
        );
        cursor.y += ENDNOTE_BAND;
        let mut continuation = false;
        for (index, note) in self.endnotes.iter().enumerate() {
            let mut leading = if index == 0 { 0.0 } else { 5.0 };
            for line in &note.lines {
                let ordinary_bottom = if continuation {
                    geometry.height - geometry.footer_space - self.endnote_footer_height
                } else {
                    geometry.content_bottom()
                };
                let bottom = pages[cursor.page_number - 1]
                    .note_body_bottom
                    .unwrap_or(ordinary_bottom);
                if cursor.y + leading + line.height > bottom {
                    let available = geometry.height
                        - geometry.footer_space
                        - geometry.margin_top
                        - ENDNOTE_BAND;
                    if line.height > available + 0.001 {
                        warnings.push(
                            "native oversized endnote line exceeds its continuation page".into(),
                        );
                    }
                    next_area(cursor, pages, columns, geometry);
                    continuation = true;
                    separator(
                        &mut pages[cursor.page_number - 1],
                        geometry.margin_left,
                        cursor.y + 6.945,
                        width,
                    );
                    cursor.y += ENDNOTE_BAND;
                    leading = 0.0;
                }
                cursor.y += leading;
                leading = 0.0;
                let mut line = line.clone();
                line.translate(geometry.margin_left, cursor.y);
                pages[cursor.page_number - 1].push_line(PlacedTextLine {
                    top: cursor.y,
                    line: line.clone(),
                });
                cursor.y += line.height;
                cursor.body_started = true;
                columns.note(cursor, geometry);
            }
        }
        Ok(())
    }
}
fn separator(page: &mut TextPage, left: f64, top: f64, width: f64) {
    page.operations.push(PageOperation::Rule(Rule {
        role: RuleRole::NoteSeparator,
        start: [left, top],
        end: [left + width, top],
        width: 0.5,
        color: "111827".into(),
    }));
}
