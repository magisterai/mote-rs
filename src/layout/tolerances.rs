//! The tolerances of pagination, in points. A comparison of a height with the
//! room for it, or of one position with another, takes its tolerance from
//! here, so that two flows that differ in a tolerance differ visibly.

/// The excess over the room in an area that a block may have and stay in the
/// area, which is none. The tolerance holds for a block of the body and the
/// space above it, for the blocks kept together on a page, for the header
/// rows repeated above a row of a table, for a line of a paragraph without
/// widow control, for a line without a footnote reference in a paragraph that
/// has one, and for a line of the endnotes.
pub(super) const BLOCK_FIT_TOLERANCE: f64 = 0.0;

/// The excess over the room in an area that a line may have and stay in the
/// area. The tolerance holds for a line of a paragraph under widow control, a
/// line with a footnote reference, a paragraph kept together that has a
/// footnote reference, and a line of a header or footer. The warning of an
/// endnote line taller than its page uses the tolerance also.
pub(super) const LINE_FIT_TOLERANCE: f64 = 0.001;

/// The excess over the room in an area that a row of a table, or the part of
/// a row above a cut, may have and stay in the area.
pub(super) const ROW_FIT_TOLERANCE: f64 = 0.01;

/// The distance from the top of an area within which the cursor is taken to
/// stand at the top.
pub(super) const AREA_START_TOLERANCE: f64 = 0.001;

/// The distance within which two vertical positions in a table are taken to
/// coincide, and the height below which a part of a row is taken to be empty.
pub(super) const TABLE_POSITION_TOLERANCE: f64 = 0.01;

/// The difference of heights below which the search for the height of
/// balanced columns ends, and the least amount by which a balanced height
/// falls short of a whole column.
pub(super) const BALANCE_TOLERANCE: f64 = 0.01;

/// The least height of the area that is left to a section that begins at the
/// foot of a full page. The paragraph flow requires an area of positive
/// height.
pub(super) const LEAST_AREA_HEIGHT: f64 = 0.01;
