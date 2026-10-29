use super::{HighlightRectangle, TextLine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleRole {
    Generic,
    ColumnSeparator,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub role: RuleRole,
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub width: f64,
    pub color: String,
}

/// Ordered page operations refer to text lines stored on the same page.
#[derive(Debug, Clone, PartialEq)]
pub enum PageOperation {
    Text(usize),
    Rectangle(HighlightRectangle),
    Rule(Rule),
    ListMarker(Box<ListMarker>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListMarkerKind {
    Bullet,
    Ordered,
    Task,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListMarker {
    pub kind: ListMarkerKind,
    pub checked: bool,
    pub slot: [f64; 4],
    pub baseline: f64,
    pub font_size: f64,
    pub text: Option<TextLine>,
}
