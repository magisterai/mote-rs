use super::{FlowCursor, PageGeometry, PageOperation, Rule, TextPage};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct ColumnSeparators {
    extents: BTreeMap<usize, (usize, f64)>,
}
impl ColumnSeparators {
    pub fn note(&mut self, cursor: &FlowCursor, geometry: PageGeometry) {
        if !geometry.column_separator || geometry.column_count < 2 {
            return;
        }
        let extent = self
            .extents
            .entry(cursor.page_number)
            .or_insert((0, geometry.margin_top));
        extent.0 = extent.0.max(cursor.column_index + 1);
        extent.1 = extent.1.max(cursor.y.min(geometry.content_bottom()));
    }
    pub fn append(self, pages: &mut [TextPage], geometry: PageGeometry) {
        for (page_number, (used, bottom)) in self.extents {
            if bottom <= geometry.margin_top {
                continue;
            }
            for column in 1..used.min(geometry.column_count) {
                let horizontal = geometry.column_left(column) - geometry.column_gap / 2.0;
                pages[page_number - 1]
                    .operations
                    .push(PageOperation::Rule(Rule {
                        role: super::RuleRole::ColumnSeparator,
                        start: [horizontal, geometry.margin_top],
                        end: [horizontal, bottom],
                        width: 1.0,
                        color: "000000".into(),
                    }));
            }
        }
    }
}
