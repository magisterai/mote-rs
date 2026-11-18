//! The rules for the space between the blocks of a flow.
use super::FlowCursor;

impl FlowCursor {
    /// Returns whether the cursor stands in the page and the column of
    /// `previous`.
    pub(super) fn is_in_area_of(&self, previous: &FlowCursor) -> bool {
        self.page_number == previous.page_number && self.column_index == previous.column_index
    }
    /// Moves the cursor past the space above a block, where `previous` is the
    /// cursor before room was sought for the block. A block that has moved to
    /// a new page or to a new column leaves that space behind.
    pub(super) fn add_leading(&mut self, previous: &FlowCursor, leading: f64) {
        if self.is_in_area_of(previous) {
            self.y += leading;
        }
    }
}

/// Returns the space between two blocks: the greater of the space carried
/// from the first and the space before the second, and not the sum of the
/// two.
pub(super) fn collapsed_spacing(carried: f64, before: f64) -> f64 {
    carried.max(before)
}
