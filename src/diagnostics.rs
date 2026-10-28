//! Source ranges and diagnostics shared by parsing, evaluation, and document metadata.

use std::fmt;

/// A half-open UTF-8 byte range in the source passed to the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
}

impl Diagnostic {
    pub(crate) fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    /// One-based line and Unicode scalar column, with CRLF counted once.
    pub fn location(&self, source: &str) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        let mut previous_was_carriage_return = false;
        for (offset, character) in source.char_indices() {
            if offset >= self.span.start {
                break;
            }
            match character {
                '\r' => {
                    line += 1;
                    column = 1;
                }
                '\n' if previous_was_carriage_return => {}
                '\n' => {
                    line += 1;
                    column = 1;
                }
                _ => column += 1,
            }
            previous_was_carriage_return = character == '\r';
        }
        (line, column)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} (bytes {}..{})",
            self.message, self.span.start, self.span.end
        )
    }
}

impl std::error::Error for Diagnostic {}
