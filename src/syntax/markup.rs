//! Markup parsing, inline delimiters, and block/container boundaries.

use super::expressions::ArgumentListKind;
use super::{Call, Diagnostic, Node, NodeKind, Parser, Span};

#[derive(Clone, Copy, PartialEq)]
pub(super) enum MarkupBoundary {
    Source,
    TableCell,
    Body,
    BracedBody,
    Delimited(&'static str),
    Line,
    ListItem { marker_column: usize },
}

impl Parser<'_> {
    pub(super) fn parse_markup(
        &mut self,
        boundary: MarkupBoundary,
    ) -> Result<Vec<Node>, Diagnostic> {
        self.enter_nested_construct()?;
        let previous_delimiter = self.closing_delimiter;
        self.closing_delimiter = match boundary {
            MarkupBoundary::Body => Some(']'),
            MarkupBoundary::BracedBody => Some('}'),
            MarkupBoundary::Source => None,
            _ => previous_delimiter,
        };
        let mut nodes = Vec::new();
        let content_start = self.position;
        let mut continuation_start = false;
        loop {
            if boundary == MarkupBoundary::BracedBody && self.peek_character() == Some('}') {
                self.advance_character();
                break;
            }
            if boundary == MarkupBoundary::Body && self.peek_character() == Some(']') {
                self.advance_character();
                break;
            }
            match boundary {
                MarkupBoundary::Delimited(marker) if self.starts_with(marker) => {
                    self.position += marker.len();
                    break;
                }
                _ => {}
            }
            if boundary == MarkupBoundary::Line
                && (matches!(self.peek_character(), None | Some('\r' | '\n'))
                    || self.peek_character() == self.closing_delimiter)
            {
                break;
            }
            let Some(mut character) = self.peek_character() else {
                if boundary == MarkupBoundary::BracedBody {
                    return Err(self.error("missing closing '}' for iteration body"));
                }
                if boundary == MarkupBoundary::Body {
                    return Err(self.error("missing closing ']' for content body"));
                }
                if let MarkupBoundary::Delimited(marker) = boundary {
                    return Err(
                        self.error(format!("missing closing '{marker}' for inline content"))
                    );
                }
                break;
            };
            let start = self.position;
            if let MarkupBoundary::ListItem { marker_column } = boundary {
                if Some(character) == self.closing_delimiter {
                    break;
                }
                if matches!(character, '\r' | '\n') {
                    let Some(paragraph_break) = self.consume_list_continuation(marker_column)
                    else {
                        break;
                    };
                    nodes.push(Node {
                        kind: if paragraph_break {
                            NodeKind::ParagraphBreak
                        } else {
                            NodeKind::LineBreak { hard: false }
                        },
                        span: self.span(start),
                    });
                    continuation_start = true;
                    continue;
                }
            }
            if (matches!(
                boundary,
                MarkupBoundary::Source | MarkupBoundary::Body | MarkupBoundary::BracedBody
            ) && (self.is_at_line_start() || start == content_start))
                || continuation_start
            {
                continuation_start = false;
                self.skip_horizontal_whitespace();
                if let Some(node) = self.parse_footnote_definition()? {
                    nodes.push(node);
                    continue;
                }
                if let Some(node) = self.parse_markdown_table()? {
                    nodes.push(node);
                    continue;
                }
                if let Some(node) = self.parse_heading()? {
                    nodes.push(node);
                    continue;
                }
                if let Some(node) = self.parse_list_item()? {
                    nodes.push(node);
                    continue;
                }
                if let Some(node) = self.parse_blockquote()? {
                    nodes.push(node);
                    continue;
                }
                // These block forms need their own syntax and semantics.
                if ["```", "~~~"].iter().any(|marker| self.starts_with(marker))
                    || (self.starts_with("---")
                        && !self.source[self.position..]
                            .split(['\r', '\n'])
                            .next()
                            .unwrap_or_default()
                            .contains('|'))
                {
                    return Err(self.error("this Markdown block form is not supported yet"));
                }
                if self.peek_character() == Some('#') {
                    character = '#';
                } else {
                    self.position = start;
                }
            }
            if self.starts_with("![")
                && let Some(node) = self.parse_markdown_image()
            {
                nodes.push(node);
                continue;
            }
            if character == '['
                && let Some(node) = self.parse_footnote_reference()
            {
                nodes.push(node);
                continue;
            }
            if character == '['
                && let Some(node) = self.parse_markdown_link()
            {
                nodes.push(node);
                continue;
            }
            if matches!(character, '<' | 'h')
                && let Some(node) = self.parse_autolink()
            {
                nodes.push(node);
                continue;
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
                            text.ends_with("  ")
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
                    if matches!(name.as_str(), "if" | "for") {
                        nodes.push(self.parse_control(&name, start)?);
                        continue;
                    }
                    let has_arguments = self.peek_character() == Some('(');
                    let arguments = if has_arguments {
                        self.parse_arguments(ArgumentListKind::Call)?
                    } else {
                        Vec::new()
                    };
                    let mut bodies = Vec::new();
                    while self.peek_character() == Some('[') {
                        self.advance_character();
                        bodies.push(self.parse_markup(MarkupBoundary::Body)?);
                    }
                    let kind = if has_arguments || !bodies.is_empty() {
                        NodeKind::Call(Call {
                            from_markdown: false,
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
                    let body = self.parse_markup(MarkupBoundary::Delimited("**"))?;
                    nodes.push(Node {
                        kind: NodeKind::Strong(body),
                        span: self.span(start),
                    });
                }
                '_' if self.starts_with("__") => {
                    self.position += 2;
                    let body = self.parse_markup(MarkupBoundary::Delimited("__"))?;
                    nodes.push(Node {
                        kind: NodeKind::Strong(body),
                        span: self.span(start),
                    });
                }
                '*' | '_' => {
                    self.advance_character();
                    let marker = if character == '*' { "*" } else { "_" };
                    let body = self.parse_markup(MarkupBoundary::Delimited(marker))?;
                    nodes.push(Node {
                        kind: if character == '*' {
                            NodeKind::Strong(body)
                        } else {
                            NodeKind::Emphasis(body)
                        },
                        span: self.span(start),
                    });
                }
                '`' => {
                    if let Some(node) = self.parse_code_span() {
                        nodes.push(node);
                    } else {
                        self.advance_character();
                        push_text(&mut nodes, character, self.span(start));
                    }
                }
                '<' => {
                    self.advance_character();
                    nodes.push(Node {
                        kind: NodeKind::Text("<".into()),
                        span: self.span(start),
                    });
                }
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
                            nodes.push(Node {
                                kind: NodeKind::Text(value.to_string()),
                                span: self.span(start),
                            });
                        }
                        None => push_text(&mut nodes, '\\', self.span(start)),
                    }
                }
                ']' if matches!(boundary, MarkupBoundary::Delimited(_)) => {
                    return Err(
                        self.error("missing closing inline delimiter before content body ends")
                    );
                }
                ']' => {
                    self.advance_character();
                    nodes.push(Node {
                        kind: NodeKind::Text("]".into()),
                        span: self.span(start),
                    });
                }
                _ => {
                    self.advance_character();
                    push_text(&mut nodes, character, self.span(start));
                }
            }
        }
        self.depth -= 1;
        self.closing_delimiter = previous_delimiter;
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
        let body = self.parse_markup(MarkupBoundary::Line)?;
        Ok(Some(Node {
            kind: NodeKind::Heading { level, body },
            span: self.span(start),
        }))
    }

    fn parse_code_span(&mut self) -> Option<Node> {
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

    fn parse_blockquote(&mut self) -> Result<Option<Node>, Diagnostic> {
        if self.peek_character() != Some('>') {
            return Ok(None);
        }
        let start = self.position;
        self.advance_character();
        self.skip_horizontal_whitespace();
        let body = self.parse_markup(MarkupBoundary::Line)?;
        Ok(Some(Node {
            kind: NodeKind::Blockquote(body),
            span: self.span(start),
        }))
    }

    fn parse_list_item(&mut self) -> Result<Option<Node>, Diagnostic> {
        let start = self.position;
        let marker_column = self.source_column(start);
        match self.peek_character() {
            Some('-' | '+') => {
                self.advance_character();
            }
            Some(character) if character.is_ascii_digit() => {
                while self
                    .peek_character()
                    .is_some_and(|character| character.is_ascii_digit())
                {
                    self.advance_character();
                }
                if self.peek_character() != Some('.') {
                    self.position = start;
                    return Ok(None);
                }
                self.advance_character();
            }
            _ => return Ok(None),
        }
        if !matches!(self.peek_character(), Some(' ' | '\t')) {
            self.position = start;
            return Ok(None);
        }
        let marker = self.source[start..self.position].to_owned();
        self.skip_horizontal_whitespace();
        if ["[ ] ", "[x] ", "[X] ", "[ ]\t", "[x]\t", "[X]\t"]
            .iter()
            .any(|prefix| self.starts_with(prefix))
        {
            return Err(self.error("task list markers are not supported yet"));
        }
        let body = self.parse_markup(MarkupBoundary::ListItem { marker_column })?;
        Ok(Some(Node {
            kind: NodeKind::ListItem { marker, body },
            span: self.span(start),
        }))
    }

    // A continuation must be farther right than the item's marker, even across blank lines.
    fn consume_list_continuation(&mut self, marker_column: usize) -> Option<bool> {
        let start = self.position;
        let mut line_endings = 0;
        while matches!(self.peek_character(), Some('\r' | '\n')) {
            self.consume_line_ending();
            line_endings += 1;
            self.skip_horizontal_whitespace();
        }
        if matches!(self.peek_character(), None | Some(']'))
            || self.source_column(self.position) <= marker_column
        {
            self.position = start;
            return None;
        }
        Some(line_endings > 1)
    }

    fn source_column(&self, position: usize) -> usize {
        self.source[..position]
            .rsplit(['\r', '\n'])
            .next()
            .unwrap_or("")
            .chars()
            .count()
    }
}

fn push_text(nodes: &mut Vec<Node>, character: char, span: Span) {
    if let Some(Node {
        kind: NodeKind::Text(text),
        span: previous,
    }) = nodes.last_mut()
        && text != "<"
        && text != "]"
        && previous.end == span.start
        && previous.end - previous.start == text.len()
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
