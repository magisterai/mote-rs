//! Quoted style declarations and applications lower to ordinary call arguments.

use super::{
    Argument, Diagnostic, Expression, ExpressionKind, Parser, expressions::ArgumentListKind,
};

impl Parser<'_> {
    pub(super) fn parse_style_arguments(
        &mut self,
        directive: &str,
    ) -> Result<Option<Vec<Argument>>, Diagnostic> {
        if directive == "style" {
            self.parse_named_style_arguments()
        } else if accepts_named_style(directive) {
            self.parse_applied_style_arguments()
        } else {
            Ok(None)
        }
    }

    fn parse_applied_style_arguments(&mut self) -> Result<Option<Vec<Argument>>, Diagnostic> {
        let start = self.position;
        self.skip_horizontal_whitespace();
        if !matches!(self.peek_character(), Some('\'' | '"')) {
            self.position = start;
            return Ok(None);
        }

        // A bare reference followed by quoted prose remains ordinary content.
        // Commit to the header only after a complete string and call delimiter.
        let Ok(style) = self.parse_style_header_argument("style") else {
            self.position = start;
            return Ok(None);
        };
        self.skip_horizontal_whitespace();
        if self.source[start..self.position].contains(['\r', '\n'])
            || !matches!(self.peek_character(), Some('(' | '['))
        {
            self.position = start;
            return Ok(None);
        }

        let mut arguments = vec![style];
        if self.peek_character() == Some('(') {
            let properties = self.parse_arguments(ArgumentListKind::Call)?;
            if let Some(repeated) = properties
                .iter()
                .find(|argument| argument.name.as_deref() == Some("style"))
            {
                return Err(Diagnostic::new(
                    repeated.span,
                    "style is already specified in the directive header",
                ));
            }
            arguments.extend(properties);
        }
        Ok(Some(arguments))
    }

    fn parse_named_style_arguments(&mut self) -> Result<Option<Vec<Argument>>, Diagnostic> {
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

// Only named-style references participate. A field named `style` can instead
// describe a pattern or metadata; those directives retain ordinary call syntax.
fn accepts_named_style(directive: &str) -> bool {
    matches!(
        directive,
        "paragraph"
            | "heading"
            | "span"
            | "strong"
            | "emphasis"
            | "code"
            | "raw"
            | "list"
            | "ordered_list"
            | "blockquote"
            | "term"
            | "definition"
            | "table"
            | "cell"
            | "strike"
            | "highlight"
            | "small_caps"
            | "subscript"
            | "superscript"
    )
}
