//! Retain Lua source while recognizing Mote's surrounding argument delimiters.
use super::{Diagnostic, Expression, ExpressionKind, Parser};
impl Parser<'_> {
    pub(super) fn parse_script_expression(
        &mut self,
        iterable: bool,
    ) -> Result<Expression, Diagnostic> {
        self.skip_whitespace();
        let start = self.position;
        let mut delimiters = Vec::new();
        let mut blocks = 0usize;
        while let Some(character) = self.peek_character() {
            if self.skip_script_literal()? {
                continue;
            }
            if delimiters.is_empty() && blocks == 0 {
                if matches!(character, ',' | ')' | ']' | '}') {
                    break;
                }
                if iterable
                    && character == '{'
                    && self.position > start
                    && self.source[..self.position].ends_with(char::is_whitespace)
                {
                    break;
                }
            }
            if character.is_ascii_alphabetic() || character == '_' {
                let name = self.parse_identifier()?;
                match name.as_str() {
                    "function" | "if" | "for" | "while" | "repeat" => blocks += 1,
                    "end" | "until" => blocks = blocks.saturating_sub(1),
                    _ => {}
                }
                continue;
            }
            self.advance_character();
            match character {
                '(' => delimiters.push(')'),
                '[' => delimiters.push(']'),
                '{' => delimiters.push('}'),
                ')' | ']' | '}' if delimiters.pop() != Some(character) => {
                    return Err(self.error("mismatched delimiter in Lua expression"));
                }
                _ => {}
            }
            if delimiters.len() + self.depth > 64 {
                return Err(self.error("nesting limit exceeded"));
            }
        }
        if !delimiters.is_empty() || blocks != 0 {
            return Err(self.error("unclosed Lua expression"));
        }
        let end = self.source[start..self.position].trim_end().len() + start;
        if end == start {
            return Err(self.error("expected a Lua expression"));
        }
        Ok(Expression {
            kind: ExpressionKind::Script(self.source[start..end].to_owned()),
            span: super::Span { start, end },
        })
    }

    pub(super) fn skip_script_literal(&mut self) -> Result<bool, Diagnostic> {
        let comment = self.starts_with("--");
        if comment {
            self.position += 2;
        }
        if self.peek_character() == Some('[') {
            let equals = self.source[self.position + 1..]
                .chars()
                .take_while(|character| *character == '=')
                .count();
            if self.source[self.position + 1 + equals..].starts_with('[') {
                self.position += equals + 2;
                let closing = format!("]{}]", "=".repeat(equals));
                let Some(offset) = self.source[self.position..].find(&closing) else {
                    return Err(self.error("unclosed Lua long string or comment"));
                };
                self.position += offset + closing.len();
                return Ok(true);
            }
        }
        if comment {
            while !matches!(self.peek_character(), None | Some('\r' | '\n')) {
                self.advance_character();
            }
            return Ok(true);
        }
        if matches!(self.peek_character(), Some('\'' | '"')) {
            let quote = self.advance_character().unwrap();
            while let Some(character) = self.advance_character() {
                if character == quote {
                    return Ok(true);
                }
                if character == '\\' {
                    self.advance_character();
                }
            }
            return Err(self.error("unterminated Lua string"));
        }
        Ok(false)
    }
}
