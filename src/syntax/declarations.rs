//! Named style headers lower to ordinary declaration arguments.

use super::{
    Argument, Diagnostic, Expression, ExpressionKind, Parser, expressions::ArgumentListKind,
};

impl Parser<'_> {
    pub(super) fn parse_named_style_arguments(
        &mut self,
    ) -> Result<Option<Vec<Argument>>, Diagnostic> {
        let start = self.position;
        self.skip_horizontal_whitespace();
        if !matches!(self.peek_character(), Some('\'' | '"')) {
            self.position = start;
            return Ok(None);
        }

        let mut header = vec![self.parse_style_header_argument("name")?];
        self.skip_whitespace();
        let has_parent = self.source[self.position..]
            .strip_prefix("extends")
            .is_some_and(|remaining| {
                !remaining.starts_with(|character: char| {
                    character.is_ascii_alphanumeric() || character == '_'
                })
            });
        if has_parent {
            self.position += "extends".len();
            self.skip_whitespace();
            if !matches!(self.peek_character(), Some('\'' | '"')) {
                return Err(self.error("extends requires a quoted parent style name"));
            }
            header.push(self.parse_style_header_argument("based_on")?);
            self.skip_whitespace();
        }
        if self.peek_character() != Some('(') {
            return Err(self.error("named style requires properties in parentheses"));
        }

        let arguments = self.parse_arguments(ArgumentListKind::Call)?;
        for argument in &arguments {
            let message = match argument.name.as_deref() {
                Some("name") => Some("style name is already specified in the declaration header"),
                Some("based_on") if has_parent => {
                    Some("style parent is already specified by extends")
                }
                _ => None,
            };
            if let Some(message) = message {
                return Err(Diagnostic::new(argument.span, message));
            }
        }
        header.extend(arguments);
        Ok(Some(header))
    }

    fn parse_style_header_argument(&mut self, name: &str) -> Result<Argument, Diagnostic> {
        let start = self.position;
        let value = self.parse_string()?;
        let span = self.span(start);
        Ok(Argument {
            name: Some(name.into()),
            value: Expression {
                kind: ExpressionKind::String(value),
                span,
            },
            span,
        })
    }
}
