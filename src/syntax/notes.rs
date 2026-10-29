//! Markdown note markers and one-line definitions.
use super::{Diagnostic, Node, NodeKind, Parser, markup::MarkupBoundary};
impl Parser<'_> {
    fn footnote_marker(&mut self) -> Option<String> {
        let remainder = self.source[self.position..].strip_prefix("[^")?;
        let end = remainder.find(']')?;
        if end == 0 || remainder[..end].contains(['\n', '\r']) {
            return None;
        }
        let name = remainder[..end].to_owned();
        self.position += end + 3;
        Some(name)
    }
    pub(super) fn parse_footnote_definition(&mut self) -> Result<Option<Node>, Diagnostic> {
        let start = self.position;
        let Some(name) = self.footnote_marker() else {
            return Ok(None);
        };
        if self.peek_character() != Some(':') {
            self.position = start;
            return Ok(None);
        }
        self.advance_character();
        self.skip_horizontal_whitespace();
        let body = self.parse_markup(MarkupBoundary::Line)?;
        Ok(Some(Node {
            kind: NodeKind::FootnoteDefinition { name, body },
            span: self.span(start),
        }))
    }
    pub(super) fn parse_footnote_reference(&mut self) -> Option<Node> {
        let start = self.position;
        let name = self.footnote_marker()?;
        Some(Node {
            kind: NodeKind::FootnoteReference(name),
            span: self.span(start),
        })
    }
}
