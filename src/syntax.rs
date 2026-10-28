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
        let mut previous_cr = false;
        for (offset, ch) in source.char_indices() {
            if offset >= self.span.start {
                break;
            }
            match ch {
                '\r' => {
                    line += 1;
                    column = 1;
                }
                '\n' if previous_cr => {}
                '\n' => {
                    line += 1;
                    column = 1;
                }
                _ => column += 1,
            }
            previous_cr = ch == '\r';
        }
        (line, column)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
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
        nodes: parser.markup(End::Source)?,
    })
}

pub fn parse_expression(source: &str) -> Result<Expression, Diagnostic> {
    let mut parser = Parser {
        source,
        position: 0,
        depth: 0,
    };
    let expression = parser.expression()?;
    parser.space();
    if parser.peek().is_some() {
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

struct Parser<'a> {
    source: &'a str,
    position: usize,
    depth: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }
    fn starts(&self, value: &str) -> bool {
        self.source[self.position..].starts_with(value)
    }
    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8();
        Some(ch)
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
                end: self.position + self.peek().map_or(0, char::len_utf8),
            },
            message,
        )
    }
    fn enter(&mut self) -> Result<(), Diagnostic> {
        if self.depth >= 64 {
            return Err(self.error("nesting limit exceeded"));
        }
        self.depth += 1;
        Ok(())
    }
    fn space(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\r' | '\n')) {
            self.bump();
        }
    }
    fn horizontal_space(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t')) {
            self.bump();
        }
    }
    fn newline(&mut self) {
        if self.starts("\r\n") {
            self.position += 2;
        } else {
            self.bump();
        }
    }
    fn line_start(&self) -> bool {
        self.position == 0 || matches!(self.source.as_bytes()[self.position - 1], b'\r' | b'\n')
    }

    fn markup(&mut self, end: End) -> Result<Vec<Node>, Diagnostic> {
        self.enter()?;
        let mut nodes = Vec::new();
        let content_start = self.position;
        loop {
            if end == End::Body && self.peek() == Some(']') {
                self.bump();
                break;
            }
            if end == End::Strong && self.starts("**") {
                self.position += 2;
                break;
            }
            if end == End::Line && matches!(self.peek(), None | Some('\r' | '\n' | ']')) {
                break;
            }
            let Some(ch) = self.peek() else {
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
                && (self.line_start() || start == content_start)
            {
                self.horizontal_space();
                if let Some(node) = self.heading()? {
                    nodes.push(node);
                    continue;
                }
                // These block forms need their own syntax and semantics.
                if ["- ", "+ ", ">", "```", "~~~", "---"]
                    .iter()
                    .any(|s| self.starts(s))
                {
                    return Err(self.error("this Markdown block form is not supported yet"));
                }
                self.position = start;
            }
            match ch {
                '\r' | '\n' => {
                    self.newline();
                    let after_first = self.position;
                    self.horizontal_space();
                    if matches!(self.peek(), Some('\r' | '\n')) {
                        self.newline();
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
                    self.bump();
                    let name = self.path()?;
                    if self.peek() == Some('-') {
                        return Err(self.error("names use underscores, not hyphens"));
                    }
                    let has_arguments = self.peek() == Some('(');
                    let arguments = if has_arguments {
                        self.arguments(')', false)?
                    } else {
                        Vec::new()
                    };
                    let mut bodies = Vec::new();
                    while self.peek() == Some('[') {
                        self.bump();
                        bodies.push(self.markup(End::Body)?);
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
                '*' if self.starts("**") => {
                    self.position += 2;
                    let body = self.markup(End::Strong)?;
                    nodes.push(Node {
                        kind: NodeKind::Strong(body),
                        span: self.span(start),
                    });
                }
                '*' | '_' | '`' => return Err(self.error(
                    "this inline Markdown form is not supported yet; escape literal punctuation",
                )),
                '\\' => {
                    self.bump();
                    match self.peek() {
                        Some('[' | '(') => {
                            return Err(self.error("mathematical notation is not supported yet"));
                        }
                        Some('\r' | '\n') => {
                            self.newline();
                            nodes.push(Node {
                                kind: NodeKind::LineBreak { hard: true },
                                span: self.span(start),
                            });
                        }
                        Some(value) => {
                            self.bump();
                            push_text(&mut nodes, value, self.span(start));
                        }
                        None => push_text(&mut nodes, '\\', self.span(start)),
                    }
                }
                ']' if end == End::Strong => {
                    return Err(self.error("missing closing '**' before content body ends"));
                }
                _ => {
                    self.bump();
                    push_text(&mut nodes, ch, self.span(start));
                }
            }
        }
        self.depth -= 1;
        Ok(nodes)
    }

    fn heading(&mut self) -> Result<Option<Node>, Diagnostic> {
        let start = self.position;
        while self.peek() == Some('#') {
            self.bump();
        }
        let level = self.position - start;
        if level == 0 || !matches!(self.peek(), Some(' ' | '\t')) {
            self.position = start;
            return Ok(None);
        }
        self.horizontal_space();
        let body = self.markup(End::Line)?;
        Ok(Some(Node {
            kind: NodeKind::Heading { level, body },
            span: self.span(start),
        }))
    }

    fn identifier(&mut self) -> Result<String, Diagnostic> {
        let start = self.position;
        if !self.peek().is_some_and(identifier_start) {
            return Err(self.error("expected an identifier beginning with a letter or underscore"));
        }
        self.bump();
        while self
            .peek()
            .is_some_and(|c| identifier_start(c) || c.is_ascii_digit())
        {
            self.bump();
        }
        Ok(self.source[start..self.position].to_owned())
    }

    fn path(&mut self) -> Result<String, Diagnostic> {
        let mut name = self.identifier()?;
        while self.peek() == Some('.') {
            self.bump();
            name.push('.');
            name.push_str(&self.identifier()?);
        }
        Ok(name)
    }

    // The opening delimiter is at the current position.
    fn arguments(&mut self, close: char, object: bool) -> Result<Vec<Argument>, Diagnostic> {
        self.bump();
        self.space();
        let mut arguments = Vec::new();
        loop {
            if self.peek() == Some(close) {
                self.bump();
                if object && arguments.is_empty() {
                    return Err(self.error("empty collections are not supported yet"));
                }
                return Ok(arguments);
            }
            let start = self.position;
            let mut name = None;
            if self.peek().is_some_and(identifier_start) {
                let candidate = self.identifier()?;
                self.space();
                if self.peek() == Some('=') && !self.starts("==") {
                    self.bump();
                    name = Some(candidate);
                } else {
                    self.position = start;
                }
            }
            if object && name.is_none() {
                return Err(self.error("expected a named property; arrays are not supported yet"));
            }
            let value = self.expression()?;
            arguments.push(Argument {
                name,
                value,
                span: self.span(start),
            });
            self.space();
            match self.peek() {
                Some(c) if c == close => continue,
                Some(',') => {
                    self.bump();
                    self.space();
                }
                _ => return Err(self.error(format!("expected ',' or '{close}'"))),
            }
        }
    }

    fn expression(&mut self) -> Result<Expression, Diagnostic> {
        self.enter()?;
        self.space();
        let start = self.position;
        let kind = match self.peek() {
            Some('\'' | '"') => ExpressionKind::String(self.string()?),
            Some('{') => ExpressionKind::Object(self.arguments('}', true)?),
            Some(c) if c.is_ascii_digit() || c == '-' => {
                if c == '-' {
                    self.bump();
                }
                let digits_start = self.position;
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.bump();
                }
                let mut digits = self.position - digits_start;
                if self.peek() == Some('.') {
                    self.bump();
                    let fraction_start = self.position;
                    while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                        self.bump();
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
            Some(c) if identifier_start(c) => {
                let name = self.path()?;
                self.space();
                match name.as_str() {
                    "true" => ExpressionKind::Boolean(true),
                    "false" => ExpressionKind::Boolean(false),
                    "nil" => ExpressionKind::Nil,
                    _ if self.peek() == Some('(') => ExpressionKind::Call {
                        name,
                        arguments: self.arguments(')', false)?,
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

    fn string(&mut self) -> Result<String, Diagnostic> {
        let quote = self.bump().expect("string starts with a quote");
        let mut value = String::new();
        loop {
            match self.bump() {
                Some(c) if c == quote => return Ok(value),
                Some('\\') => match self.bump() {
                    Some('n') => value.push('\n'),
                    Some('r') => value.push('\r'),
                    Some('t') => value.push('\t'),
                    Some(c) => value.push(c),
                    None => return Err(self.error("unterminated string escape")),
                },
                Some(c) => value.push(c),
                None => return Err(self.error("unterminated string")),
            }
        }
    }
}

fn identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn push_text(nodes: &mut Vec<Node>, ch: char, span: Span) {
    if let Some(Node {
        kind: NodeKind::Text(text),
        span: previous,
    }) = nodes.last_mut()
    {
        text.push(ch);
        previous.end = span.end;
    } else {
        nodes.push(Node {
            kind: NodeKind::Text(ch.to_string()),
            span,
        });
    }
}
