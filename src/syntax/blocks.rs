//! Literal fenced blocks are recognized before inline backtick parsing.
use super::{Node, NodeKind, Parser, Span};

impl Parser<'_> {
    pub(super) fn parse_code_fence(&mut self) -> Option<Node> {
        let start = self.position;
        let marker = self.peek_character()?;
        if !matches!(marker, '`' | '~') {
            return None;
        }
        let count = self.source[start..]
            .chars()
            .take_while(|character| *character == marker)
            .count();
        if count < 3 {
            return None;
        }
        let fence = marker.to_string().repeat(count);
        let information_start = start + count;
        let information_end = self.source[information_start..]
            .find(|character| {
                matches!(character, '\r' | '\n') || Some(character) == self.closing_delimiter
            })
            .map_or(self.source.len(), |offset| information_start + offset);
        let language = self.source[information_start..information_end]
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_owned();
        let content_start = skip_line_ending(self.source, information_end);
        let mut line_start = content_start;
        while line_start < self.source.len() {
            if self.source[line_start..].chars().next() == self.closing_delimiter {
                return None;
            }
            let line_end = self.source[line_start..]
                .find(['\r', '\n'])
                .map_or(self.source.len(), |offset| line_start + offset);
            let line = &self.source[line_start..line_end];
            let trimmed = line.trim_start_matches([' ', '\t']);
            if let Some(trailing) = trimmed.strip_prefix(&fence) {
                let trailing = trailing.trim_start_matches([' ', '\t']);
                if trailing.is_empty() || trailing.chars().next() == self.closing_delimiter {
                    self.position = if trailing.is_empty() {
                        skip_line_ending(self.source, line_end)
                    } else {
                        line_end - trailing.len()
                    };
                    return Some(Node {
                        kind: NodeKind::CodeFence {
                            text: self.source[content_start..line_start].into(),
                            language,
                        },
                        span: Span {
                            start,
                            end: self.position,
                        },
                    });
                }
            }
            // A body delimiter before the closing fence ends this literal candidate.
            if self
                .closing_delimiter
                .is_some_and(|delimiter| line.contains(delimiter))
            {
                return None;
            }
            line_start = skip_line_ending(self.source, line_end);
        }
        None
    }
}
fn skip_line_ending(source: &str, mut offset: usize) -> usize {
    if source[offset..].starts_with('\r') {
        offset += 1;
    }
    if source[offset..].starts_with('\n') {
        offset += 1;
    }
    offset
}
