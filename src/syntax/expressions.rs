//! Argument lists, identifiers, and literal/helper expressions.

use super::{Argument, Diagnostic, Expression, ExpressionKind, Parser};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ArgumentListKind {
    Call,
    Collection,
}

impl ArgumentListKind {
    fn closing_delimiter(self) -> char {
        match self {
            Self::Call => ')',
            Self::Collection => '}',
        }
    }
}

impl Parser<'_> {
    pub(super) fn parse_identifier(&mut self) -> Result<String, Diagnostic> {
        let start = self.position;
        if !self.peek_character().is_some_and(is_identifier_start) {
            return Err(self.error("expected an identifier beginning with a letter or underscore"));
        }
        self.advance_character();
        while self
            .peek_character()
            .is_some_and(|character| is_identifier_start(character) || character.is_ascii_digit())
        {
            self.advance_character();
        }
        Ok(self.source[start..self.position].to_owned())
    }

    pub(super) fn parse_path(&mut self) -> Result<String, Diagnostic> {
        let mut name = self.parse_identifier()?;
        while self.peek_character() == Some('.')
            && self.source[self.position + 1..]
                .chars()
                .next()
                .is_some_and(is_identifier_start)
        {
            self.advance_character();
            name.push('.');
            name.push_str(&self.parse_identifier()?);
        }
        Ok(name)
    }

    // The opening delimiter is at the current position.
    pub(super) fn parse_arguments(
        &mut self,
        kind: ArgumentListKind,
    ) -> Result<Vec<Argument>, Diagnostic> {
        let closing_delimiter = kind.closing_delimiter();
        self.advance_character();
        self.skip_whitespace();
        let mut arguments = Vec::new();
        loop {
            if self.peek_character() == Some(closing_delimiter) {
                self.advance_character();
                if kind == ArgumentListKind::Collection
                    && arguments
                        .iter()
                        .any(|argument: &Argument| argument.name.is_some())
                    && arguments.iter().any(|argument| argument.name.is_none())
                {
                    return Err(self.error("collections cannot mix named fields and array entries"));
                }
                return Ok(arguments);
            }
            let start = self.position;
            let mut name = None;
            if self.peek_character().is_some_and(is_identifier_start) {
                let candidate = self.parse_identifier()?;
                self.skip_whitespace();
                if self.peek_character() == Some('=') && !self.starts_with("==") {
                    self.advance_character();
                    name = Some(candidate);
                } else {
                    self.position = start;
                }
            }
            let value = self.parse_expression()?;
            arguments.push(Argument {
                name,
                value,
                span: self.span(start),
            });
            self.skip_whitespace();
            match self.peek_character() {
                Some(character) if character == closing_delimiter => continue,
                Some(',') => {
                    self.advance_character();
                    self.skip_whitespace();
                }
                _ => return Err(self.error(format!("expected ',' or '{closing_delimiter}'"))),
            }
        }
    }

    pub(super) fn parse_expression(&mut self) -> Result<Expression, Diagnostic> {
        if self.allow_lua {
            return self.parse_script_expression(false);
        }
        self.enter_nested_construct()?;
        self.skip_whitespace();
        let start = self.position;
        let kind = match self.peek_character() {
            Some('\'' | '"') => ExpressionKind::String(self.parse_string()?),
            Some('{') => {
                let entries = self.parse_arguments(ArgumentListKind::Collection)?;
                if entries.first().is_some_and(|entry| entry.name.is_some()) {
                    ExpressionKind::Object(entries)
                } else {
                    ExpressionKind::Array(entries.into_iter().map(|entry| entry.value).collect())
                }
            }
            Some(character) if character.is_ascii_digit() || character == '-' => {
                if character == '-' {
                    self.advance_character();
                }
                let integer_start = self.position;
                while self
                    .peek_character()
                    .is_some_and(|character| character.is_ascii_digit())
                {
                    self.advance_character();
                }
                let mut digit_count = self.position - integer_start;
                if self.peek_character() == Some('.') {
                    self.advance_character();
                    let fraction_start = self.position;
                    while self
                        .peek_character()
                        .is_some_and(|character| character.is_ascii_digit())
                    {
                        self.advance_character();
                    }
                    digit_count += self.position - fraction_start;
                }
                if digit_count == 0 {
                    return Err(self.error("expected digits in number"));
                }
                let value: f64 = self.source[start..self.position]
                    .parse()
                    .map_err(|_| self.error("invalid number"))?;
                if !value.is_finite() {
                    return Err(self.error("number must be finite"));
                }
                ExpressionKind::Number(value)
            }
            Some(character) if is_identifier_start(character) => {
                let mut name = self.parse_identifier()?;
                while self.peek_character() == Some('.') {
                    self.advance_character();
                    let start = self.position;
                    while self.peek_character().is_some_and(|character| {
                        is_identifier_start(character) || character.is_ascii_digit()
                    }) {
                        self.advance_character();
                    }
                    if start == self.position {
                        break;
                    }
                    name.push('.');
                    name.push_str(&self.source[start..self.position]);
                }
                self.skip_whitespace();
                match name.as_str() {
                    "true" => ExpressionKind::Boolean(true),
                    "false" => ExpressionKind::Boolean(false),
                    "nil" => ExpressionKind::Nil,
                    _ if self.peek_character() == Some('(') => ExpressionKind::Call {
                        name,
                        arguments: self.parse_arguments(ArgumentListKind::Call)?,
                    },
                    _ => ExpressionKind::Path(name),
                }
            }
            _ => return Err(self.error("expected a value")),
        };
        self.depth -= 1;
        Ok(Expression {
            kind,
            span: self.span(start),
        })
    }

    fn parse_string(&mut self) -> Result<String, Diagnostic> {
        let quote = self
            .advance_character()
            .expect("string starts with a quote");
        let mut value = String::new();
        loop {
            match self.advance_character() {
                Some(character) if character == quote => return Ok(value),
                Some('\\') => match self.advance_character() {
                    Some('n') => value.push('\n'),
                    Some('r') => value.push('\r'),
                    Some('t') => value.push('\t'),
                    Some(character) => value.push(character),
                    None => return Err(self.error("unterminated string escape")),
                },
                Some(character) => value.push(character),
                None => return Err(self.error("unterminated string")),
            }
        }
    }
}

fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}
