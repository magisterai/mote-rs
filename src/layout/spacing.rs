//! The rules for the space between the blocks of a flow.
use super::FlowCursor;

/// States the move that drops the space above a block. Word drops the space
/// above a block that moves to a new page or to a new column. A flow that
/// departs from that rule names its departure here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LeadingDrop {
    /// The space is dropped if the block moves to a new page or to a new
    /// column.
    InNewPageOrColumn,
    /// The space is dropped if the block moves to a new page, and is kept if
    /// the block moves to a new column of the same page.
    InNewPageOnly,
    /// The space is kept wherever the block moves.
    Never,
}

impl FlowCursor {
    /// Returns whether a block keeps the space above it, where `previous` is
    /// the cursor before room was sought for the block and `self` is the
    /// cursor after.
    pub(super) fn keeps_leading(&self, previous: &FlowCursor, dropped: LeadingDrop) -> bool {
        match dropped {
            LeadingDrop::InNewPageOrColumn => {
                self.page_number == previous.page_number
                    && self.column_index == previous.column_index
            }
            LeadingDrop::InNewPageOnly => self.page_number == previous.page_number,
            LeadingDrop::Never => true,
        }
    }
    /// Moves the cursor past the space above a block unless the move from
    /// `previous` drops that space.
    pub(super) fn add_leading(
        &mut self,
        previous: &FlowCursor,
        leading: f64,
        dropped: LeadingDrop,
    ) {
        if self.keeps_leading(previous, dropped) {
            self.y += leading;
        }
    }
}
