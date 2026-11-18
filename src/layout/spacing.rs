//! The rules for the space between the blocks of a flow.
use super::FlowCursor;

/// States the move that drops the space above a block. The body drops the
/// space above a block that moves to a new page or to a new column. A flow
/// that departs from that rule names its departure here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LeadingDrop {
    /// The space is dropped if the block moves to a new page or to a new
    /// column.
    InNewPageOrColumn,
    /// The space is dropped if the block moves to a new page, and is kept if
    /// the block moves to a new column of the same page.
    InNewPageOnly,
}

impl FlowCursor {
    /// Returns whether the cursor stands in the page and the column of
    /// `previous`.
    pub(super) fn is_in_area_of(&self, previous: &FlowCursor) -> bool {
        self.page_number == previous.page_number && self.column_index == previous.column_index
    }
    /// Returns whether a block keeps the space above it, where `previous` is
    /// the cursor before room was sought for the block and `self` is the
    /// cursor after.
    pub(super) fn keeps_leading(&self, previous: &FlowCursor, dropped: LeadingDrop) -> bool {
        match dropped {
            LeadingDrop::InNewPageOrColumn => self.is_in_area_of(previous),
            LeadingDrop::InNewPageOnly => self.page_number == previous.page_number,
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

/// States what a flow makes of a negative space after a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NegativeSpaceAfter {
    /// A negative space after a block is taken as no space.
    TakenAsNone,
    /// A negative space after a block is kept, and draws the block below it
    /// upwards unless the space before that block exceeds it.
    Kept,
}

/// Returns the space after a block that is carried to the block below it.
pub(super) fn carried_spacing(after: f64, negative: NegativeSpaceAfter) -> f64 {
    match negative {
        NegativeSpaceAfter::TakenAsNone => after.max(0.0),
        NegativeSpaceAfter::Kept => after,
    }
}

/// Returns the space between two blocks: the greater of the space carried
/// from the first and the space before the second, and not the sum of the
/// two.
pub(super) fn collapsed_spacing(carried: f64, before: f64) -> f64 {
    carried.max(before)
}
