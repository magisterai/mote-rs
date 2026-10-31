//! Parsing preserves source byte ranges and leaves call validation to evaluation.

// Keep parser-facing imports available alongside the shared diagnostics module.
pub use crate::diagnostics::{Diagnostic, Span};

mod blocks;
mod control;
mod expressions;
mod links;
mod markup;
mod notes;
mod scripting;
mod tables;

use markup::MarkupBoundary;

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxTree {
    pub source: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Text(String),
    LineBreak {
        hard: bool,
    },
    ParagraphBreak,
    Divider,
    Heading {
        level: usize,
        marker: String,
        body: Vec<Node>,
    },
    Strong(Vec<Node>),
    Emphasis(Vec<Node>),
    Code(String),
    CodeFence {
        text: String,
        language: String,
    },
    ListItem {
        marker: String,
        checked: Option<bool>,
        body: Vec<Node>,
    },
    Blockquote(Vec<Node>),
    Image {
        source: String,
        title: String,
        label: Vec<Node>,
    },
    Table(Vec<MarkdownTableRow>),
    Call(Call),
    Set(Call),
    Reference(String),
    FootnoteDefinition {
        name: String,
        body: Vec<Node>,
    },
    FootnoteReference(String),
    Conditional {
        condition: Expression,
        consequence: Vec<Node>,
        alternative: Option<Vec<Node>>,
    },
    Iteration {
        binding: String,
        iterable: Expression,
        body: Vec<Node>,
    },
    FunctionDefinition {
        name: String,
        parameters: Vec<String>,
        body: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownTableRow {
    pub header: bool,
    pub cells: Vec<MarkdownTableCell>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownTableCell {
    pub alignment: String,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub from_markdown: bool,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub bodies: Vec<Vec<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: Option<String>,
    pub value: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Object(Vec<Argument>),
    Array(Vec<Expression>),
    Call {
        name: String,
        arguments: Vec<Argument>,
    },
    Path(String),
    Script(String),
}

pub fn parse(source: &str) -> Result<SyntaxTree, Diagnostic> {
    parse_source(source, false)
}

/// Parse argument expressions as Lua source. This does not execute them.
pub fn parse_with_lua(source: &str) -> Result<SyntaxTree, Diagnostic> {
    parse_source(source, true)
}

fn parse_source(source: &str, allow_lua: bool) -> Result<SyntaxTree, Diagnostic> {
    let mut parser = Parser {
        source,
        position: 0,
        depth: 0,
        closing_delimiter: None,
        allow_lua,
    };
    Ok(SyntaxTree {
        source: source.to_owned(),
        nodes: parser.parse_markup(MarkupBoundary::Source)?,
    })
}

pub fn parse_expression(source: &str) -> Result<Expression, Diagnostic> {
    let mut parser = Parser {
        source,
        position: 0,
        depth: 0,
        closing_delimiter: None,
        allow_lua: false,
    };
    let expression = parser.parse_expression()?;
    parser.skip_whitespace();
    if parser.peek_character().is_some() {
        return Err(parser.error("unexpected text after expression"));
    }
    Ok(expression)
}

struct Parser<'source> {
    source: &'source str,
    position: usize,
    depth: usize,
    closing_delimiter: Option<char>,
    allow_lua: bool,
}

impl Parser<'_> {
    fn peek_character(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }
    fn starts_with(&self, value: &str) -> bool {
        self.source[self.position..].starts_with(value)
    }
    fn advance_character(&mut self) -> Option<char> {
        let character = self.peek_character()?;
        self.position += character.len_utf8();
        Some(character)
    }
    fn span(&self, start: usize) -> Span {
        Span {
            start,
            end: self.position,
        }
    }
    fn error(&self, message: impl Into<String>) -> Diagnostic {
        Diagnostic::new(
            Span {
                start: self.position,
                end: self.position + self.peek_character().map_or(0, char::len_utf8),
            },
            message,
        )
    }
    fn enter_nested_construct(&mut self) -> Result<(), Diagnostic> {
        if self.depth >= 64 {
            return Err(self.error("nesting limit exceeded"));
        }
        self.depth += 1;
        Ok(())
    }
    fn skip_whitespace(&mut self) {
        while matches!(self.peek_character(), Some(' ' | '\t' | '\r' | '\n')) {
            self.advance_character();
        }
    }
    fn skip_horizontal_whitespace(&mut self) {
        while matches!(self.peek_character(), Some(' ' | '\t')) {
            self.advance_character();
        }
    }
    fn consume_line_ending(&mut self) {
        if self.starts_with("\r\n") {
            self.position += 2;
        } else {
            self.advance_character();
        }
    }
    fn is_at_line_start(&self) -> bool {
        self.source[..self.position]
            .rsplit(['\r', '\n'])
            .next()
            .unwrap_or_default()
            .bytes()
            .all(|byte| matches!(byte, b' ' | b'\t'))
    }
}
