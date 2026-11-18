//! The tolerances of pagination, in points. A comparison of a height with the
//! room for it, or of one position with another, takes its tolerance from
//! here.

/// The excess over the room in an area that a block, a line or a row of a
/// table may have and stay in the area, so that what fits exactly fits in
/// spite of the rounding of its height.
pub(super) const FIT_TOLERANCE: f64 = 0.001;

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
