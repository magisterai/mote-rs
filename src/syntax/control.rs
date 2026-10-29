//! Conditional content and braced iteration bodies retain ordinary markup nodes.
use super::{
    Diagnostic, Node, NodeKind, Parser, expressions::ArgumentListKind, markup::MarkupBoundary,
};
impl Parser<'_> {
    pub(super) fn parse_control(&mut self, name: &str, start: usize) -> Result<Node, Diagnostic> {
        let kind = if name == "if" {
            if self.peek_character() != Some('(') {
                return Err(self.error("if requires a parenthesized condition"));
            }
            let arguments = self.parse_arguments(ArgumentListKind::Call)?;
            let condition = arguments
                .into_iter()
                .next()
                .ok_or_else(|| self.error("if requires a condition"))?
                .value;
            if self.peek_character() != Some('[') {
                return Err(self.error("if requires a bracketed body"));
            }
            self.advance_character();
            let consequence = self.parse_markup(MarkupBoundary::Body)?;
            let after_consequence = self.position;
            self.skip_whitespace();
            let alternative = if self.starts_with("else")
                && !self.source[self.position + 4..]
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
            {
                self.position += 4;
                self.skip_horizontal_whitespace();
                if self.peek_character() != Some('[') {
                    return Err(self.error("else requires a bracketed body"));
                }
                self.advance_character();
                Some(self.parse_markup(MarkupBoundary::Body)?)
            } else {
                self.position = after_consequence;
                None
            };
            NodeKind::Conditional {
                condition,
                consequence,
                alternative,
            }
        } else {
            self.skip_horizontal_whitespace();
            let binding = self.parse_identifier()?;
            self.skip_horizontal_whitespace();
            if !self.starts_with("in")
                || self.source[self.position + 2..]
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
            {
                return Err(self.error("for requires 'in' before its iterable"));
            }
            self.position += 2;
            let iterable = if self.allow_lua {
                self.parse_script_expression(true)?
            } else {
                self.parse_expression()?
            };
            self.skip_whitespace();
            if self.peek_character() != Some('{')
                || !self.source[..self.position].ends_with(char::is_whitespace)
            {
                return Err(self.error(
                    "for requires a braced body separated from its iterable by whitespace",
                ));
            }
            self.advance_character();
            let body = self.parse_markup(MarkupBoundary::BracedBody)?;
            NodeKind::Iteration {
                binding,
                iterable,
                body,
            }
        };
        Ok(Node {
            kind,
            span: self.span(start),
        })
    }
}
