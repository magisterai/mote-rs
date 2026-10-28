//! Markdown spans and container boundaries, using the enclosing source offsets.

use super::{Diagnostic, End, Node, NodeKind, Parser};

impl Parser<'_> {
    pub(super) fn parse_code_span(&mut self) -> Option<Node> {
        let start = self.position;
        let marker_length = self.source[start..]
            .bytes()
            .take_while(|byte| *byte == b'`')
            .count();
        let marker = &self.source[start..start + marker_length];
        let content_start = start + marker_length;
        let closing = self.source[content_start..].find(marker)? + content_start;
        let mut text = &self.source[content_start..closing];
        if text.len() >= 2
            && text.starts_with(' ')
            && text.ends_with(' ')
            && !text.trim().is_empty()
        {
            text = &text[1..text.len() - 1];
        }
        self.position = closing + marker_length;
        Some(Node {
            kind: NodeKind::Code(text.to_owned()),
            span: self.span(start),
        })
    }

    pub(super) fn parse_list_item(&mut self) -> Result<Option<Node>, Diagnostic> {
        let start = self.position;
        let marker_column = self.source_column(start);
        match self.peek_character() {
            Some('-' | '+') => {
                self.advance_character();
            }
            Some(character) if character.is_ascii_digit() => {
                while self
                    .peek_character()
                    .is_some_and(|character| character.is_ascii_digit())
                {
                    self.advance_character();
                }
                if self.peek_character() != Some('.') {
                    self.position = start;
                    return Ok(None);
                }
                self.advance_character();
            }
            _ => return Ok(None),
        }
        if !matches!(self.peek_character(), Some(' ' | '\t')) {
            self.position = start;
            return Ok(None);
        }
        let marker = self.source[start..self.position].to_owned();
        self.skip_horizontal_whitespace();
        if ["[ ] ", "[x] ", "[X] ", "[ ]\t", "[x]\t", "[X]\t"]
            .iter()
            .any(|prefix| self.starts_with(prefix))
        {
            return Err(self.error("task list markers are not supported yet"));
        }
        let body = self.parse_markup(End::ListItem { marker_column })?;
        Ok(Some(Node {
            kind: NodeKind::ListItem { marker, body },
            span: self.span(start),
        }))
    }

    // A continuation must be farther right than the item's marker, even across blank lines.
    pub(super) fn consume_list_continuation(&mut self, marker_column: usize) -> Option<bool> {
        let start = self.position;
        let mut line_endings = 0;
        while matches!(self.peek_character(), Some('\r' | '\n')) {
            self.consume_line_ending();
            line_endings += 1;
            self.skip_horizontal_whitespace();
        }
        if matches!(self.peek_character(), None | Some(']'))
            || self.source_column(self.position) <= marker_column
        {
            self.position = start;
            return None;
        }
        Some(line_endings > 1)
    }

    fn source_column(&self, position: usize) -> usize {
        self.source[..position]
            .rsplit(['\r', '\n'])
            .next()
            .unwrap_or("")
            .chars()
            .count()
    }
}
