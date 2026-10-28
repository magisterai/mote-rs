//! Parsing preserves source byte ranges and leaves call validation to evaluation.

use std::fmt;

/// A half-open UTF-8 byte range in the source passed to the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
}

impl Diagnostic {
    pub(crate) fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    /// One-based line and Unicode scalar column, with CRLF counted once.
    pub fn location(&self, source: &str) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        let mut previous_was_carriage_return = false;
        for (offset, character) in source.char_indices() {
            if offset >= self.span.start {
                break;
            }
            match character {
                '\r' => {
                    line += 1;
                    column = 1;
                }
                '\n' if previous_was_carriage_return => {}
                '\n' => {
                    line += 1;
                    column = 1;
                }
                _ => column += 1,
            }
            previous_was_carriage_return = character == '\r';
        }
        (line, column)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} (bytes {}..{})",
            self.message, self.span.start, self.span.end
        )
    }
}

impl std::error::Error for Diagnostic {}

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxTree {
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
    LineBreak { hard: bool },
    ParagraphBreak,
    Heading { level: usize, body: Vec<Node> },
    Strong(Vec<Node>),
    Call(Call),
    Reference(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
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
    Call {
        name: String,
        arguments: Vec<Argument>,
    },
    Path(String),
}

pub fn parse(source: &str) -> Result<SyntaxTree, Diagnostic> {
    let mut parser = Parser {
        source,
        position: 0,
        depth: 0,
    };
    Ok(SyntaxTree {
        nodes: parser.parse_markup(End::Source)?,
    })
}

pub fn parse_expression(source: &str) -> Result<Expression, Diagnostic> {
    let mut parser = Parser {
        source,
        position: 0,
        depth: 0,
    };
    let expression = parser.parse_expression()?;
    parser.skip_whitespace();
    if parser.peek_character().is_some() {
        return Err(parser.error("unexpected text after expression"));
    }
    Ok(expression)
}

#[derive(Clone, Copy, PartialEq)]
enum End {
    Source,
    Body,
    Strong,
    Line,
}

struct Parser<'source> {
    source: &'source str,
    position: usize,
    depth: usize,
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
        self.position == 0 || matches!(self.source.as_bytes()[self.position - 1], b'\r' | b'\n')
    }

    fn parse_markup(&mut self, end: End) -> Result<Vec<Node>, Diagnostic> {
        self.enter_nested_construct()?;
        let mut nodes = Vec::new();
        let content_start = self.position;
        loop {
            if end == End::Body && self.peek_character() == Some(']') {
                self.advance_character();
                break;
            }
            if end == End::Strong && self.starts_with("**") {
                self.position += 2;
                break;
            }
            if end == End::Line && matches!(self.peek_character(), None | Some('\r' | '\n' | ']')) {
                break;
            }
            let Some(character) = self.peek_character() else {
                if end == End::Body {
                    return Err(self.error("missing closing ']' for content body"));
                }
                if end == End::Strong {
                    return Err(self.error("missing closing '**' for strong content"));
                }
                break;
            };
            let start = self.position;
            if matches!(end, End::Source | End::Body)
                && (self.is_at_line_start() || start == content_start)
            {
                self.skip_horizontal_whitespace();
                if let Some(node) = self.parse_heading()? {
                    nodes.push(node);
                    continue;
                }
                // These block forms need their own syntax and semantics.
                if ["- ", "+ ", ">", "```", "~~~", "---"]
                    .iter()
                    .any(|marker| self.starts_with(marker))
                {
                    return Err(self.error("this Markdown block form is not supported yet"));
                }
                self.position = start;
            }
            match character {
                '\r' | '\n' => {
                    self.consume_line_ending();
                    let after_first = self.position;
                    self.skip_horizontal_whitespace();
                    if matches!(self.peek_character(), Some('\r' | '\n')) {
                        self.consume_line_ending();
                        nodes.push(Node {
                            kind: NodeKind::ParagraphBreak,
                            span: self.span(start),
                        });
                    } else {
                        self.position = after_first;
                        let hard = if let Some(Node {
                            kind: NodeKind::Text(text),
                            ..
                        }) = nodes.last_mut()
                        {
                            if text.ends_with("  ") {
                                text.truncate(text.trim_end_matches(' ').len());
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        nodes.push(Node {
                            kind: NodeKind::LineBreak { hard },
                            span: self.span(start),
                        });
                    }
                }
                '#' => {
                    self.advance_character();
                    let name = self.parse_path()?;
                    if self.peek_character() == Some('-') {
                        return Err(self.error("names use underscores, not hyphens"));
                    }
                    let has_arguments = self.peek_character() == Some('(');
                    let arguments = if has_arguments {
                        self.parse_arguments(')', false)?
                    } else {
                        Vec::new()
                    };
                    let mut bodies = Vec::new();
                    while self.peek_character() == Some('[') {
                        self.advance_character();
                        bodies.push(self.parse_markup(End::Body)?);
                    }
                    let kind = if has_arguments || !bodies.is_empty() {
                        NodeKind::Call(Call {
                            name,
                            arguments,
                            bodies,
                        })
                    } else {
                        NodeKind::Reference(name)
                    };
                    nodes.push(Node {
                        kind,
                        span: self.span(start),
                    });
                }
                '*' if self.starts_with("**") => {
                    self.position += 2;
                    let body = self.parse_markup(End::Strong)?;
                    nodes.push(Node {
                        kind: NodeKind::Strong(body),
                        span: self.span(start),
                    });
                }
                '*' | '_' | '`' => return Err(self.error(
                    "this inline Markdown form is not supported yet; escape literal punctuation",
                )),
                '\\' => {
                    self.advance_character();
                    match self.peek_character() {
                        Some('[' | '(') => {
                            return Err(self.error("mathematical notation is not supported yet"));
                        }
                        Some('\r' | '\n') => {
                            self.consume_line_ending();
                            nodes.push(Node {
                                kind: NodeKind::LineBreak { hard: true },
                                span: self.span(start),
                            });
                        }
                        Some(value) => {
                            self.advance_character();
                            push_text(&mut nodes, value, self.span(start));
                        }
                        None => push_text(&mut nodes, '\\', self.span(start)),
                    }
                }
                ']' if end == End::Strong => {
                    return Err(self.error("missing closing '**' before content body ends"));
                }
                _ => {
                    self.advance_character();
                    push_text(&mut nodes, character, self.span(start));
                }
            }
        }
        self.depth -= 1;
        Ok(nodes)
    }

    fn parse_heading(&mut self) -> Result<Option<Node>, Diagnostic> {
        let start = self.position;
        while self.peek_character() == Some('#') {
            self.advance_character();
        }
        let level = self.position - start;
        if level == 0 || !matches!(self.peek_character(), Some(' ' | '\t')) {
            self.position = start;
            return Ok(None);
        }
        self.skip_horizontal_whitespace();
        let body = self.parse_markup(End::Line)?;
        Ok(Some(Node {
            kind: NodeKind::Heading { level, body },
            span: self.span(start),
        }))
    }

    fn parse_identifier(&mut self) -> Result<String, Diagnostic> {
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

    fn parse_path(&mut self) -> Result<String, Diagnostic> {
        let mut name = self.parse_identifier()?;
        while self.peek_character() == Some('.') {
            self.advance_character();
            name.push('.');
            name.push_str(&self.parse_identifier()?);
        }
        Ok(name)
    }

    // The opening delimiter is at the current position.
    fn parse_arguments(&mut self, close: char, object: bool) -> Result<Vec<Argument>, Diagnostic> {
        self.advance_character();
        self.skip_whitespace();
        let mut arguments = Vec::new();
        loop {
            if self.peek_character() == Some(close) {
                self.advance_character();
                if object && arguments.is_empty() {
                    return Err(self.error("empty collections are not supported yet"));
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
            if object && name.is_none() {
                return Err(self.error("expected a named property; arrays are not supported yet"));
            }
            let value = self.parse_expression()?;
            arguments.push(Argument {
                name,
                value,
                span: self.span(start),
            });
            self.skip_whitespace();
            match self.peek_character() {
                Some(character) if character == close => continue,
                Some(',') => {
                    self.advance_character();
                    self.skip_whitespace();
                }
                _ => return Err(self.error(format!("expected ',' or '{close}'"))),
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, Diagnostic> {
        self.enter_nested_construct()?;
        self.skip_whitespace();
        let start = self.position;
        let kind = match self.peek_character() {
            Some('\'' | '"') => ExpressionKind::String(self.parse_string()?),
            Some('{') => ExpressionKind::Object(self.parse_arguments('}', true)?),
            Some(character) if character.is_ascii_digit() || character == '-' => {
                if character == '-' {
                    self.advance_character();
                }
                let digits_start = self.position;
                while self
                    .peek_character()
                    .is_some_and(|character| character.is_ascii_digit())
                {
                    self.advance_character();
                }
                let mut digits = self.position - digits_start;
                if self.peek_character() == Some('.') {
                    self.advance_character();
                    let fraction_start = self.position;
                    while self
                        .peek_character()
                        .is_some_and(|character| character.is_ascii_digit())
                    {
                        self.advance_character();
                    }
                    digits += self.position - fraction_start;
                }
                if digits == 0 {
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
                let name = self.parse_path()?;
                self.skip_whitespace();
                match name.as_str() {
                    "true" => ExpressionKind::Boolean(true),
                    "false" => ExpressionKind::Boolean(false),
                    "nil" => ExpressionKind::Nil,
                    _ if self.peek_character() == Some('(') => ExpressionKind::Call {
                        name,
                        arguments: self.parse_arguments(')', false)?,
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

fn push_text(nodes: &mut Vec<Node>, character: char, span: Span) {
    if let Some(Node {
        kind: NodeKind::Text(text),
        span: previous,
    }) = nodes.last_mut()
    {
        text.push(character);
        previous.end = span.end;
    } else {
        nodes.push(Node {
            kind: NodeKind::Text(character.to_string()),
            span,
        });
    }
}
