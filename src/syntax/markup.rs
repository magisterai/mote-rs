//! Markdown spans and container boundaries, using the enclosing source offsets.

use super::{Node, NodeKind, Parser};

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
}
