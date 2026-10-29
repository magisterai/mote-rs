//! Markdown destinations share directive evaluation with explicit links.
use super::markup::MarkupBoundary;
use super::{Argument, Call, Expression, ExpressionKind, Node, NodeKind, Parser, Span};

impl Parser<'_> {
    pub(super) fn parse_markdown_image(&mut self) -> Option<Node> {
        let start = self.position;
        self.position += 1;
        let result = self.parse_markdown_link();
        let Some(Node {
            kind: NodeKind::Call(call),
            ..
        }) = result
        else {
            self.position = start;
            return None;
        };
        let text = |index: usize| match &call.arguments[index].value.kind {
            ExpressionKind::String(value) => value.clone(),
            _ => unreachable!(),
        };
        Some(Node {
            kind: NodeKind::Image {
                source: if call.name == "ref" {
                    format!("#{}", text(0))
                } else {
                    text(0)
                },
                title: if call.arguments.len() > 1 {
                    text(1)
                } else {
                    String::new()
                },
                label: call.bodies.into_iter().next().unwrap(),
            },
            span: self.span(start),
        })
    }
    pub(super) fn parse_markdown_link(&mut self) -> Option<Node> {
        let start = self.position;
        let depth = self.depth;
        let result = self.parse_link_body();
        if result.is_none() {
            self.position = start;
            self.depth = depth;
        }
        result
    }
    fn parse_link_body(&mut self) -> Option<Node> {
        let start = self.position;
        self.advance_character();
        let body = self.parse_markup(MarkupBoundary::Body).ok()?;
        if self.advance_character()? != '(' {
            return None;
        }
        let destination_start = self.position;
        let mut depth = 1;
        let mut quote = None;
        let mut escaped = false;
        loop {
            let character = self.advance_character()?;
            if let Some(delimiter) = quote {
                if escaped {
                    escaped = false;
                    continue;
                }
                if character == '\\' {
                    escaped = true;
                } else if character == delimiter {
                    quote = None;
                }
                continue;
            }
            match character {
                '\'' | '"' => quote = Some(character),
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
        }
        let raw = self.source[destination_start..self.position - 1].trim();
        let (destination, title) = destination(raw)?;
        Some(link_node(self.span(start), destination, title, body))
    }
    pub(super) fn parse_autolink(&mut self) -> Option<Node> {
        let start = self.position;
        let remaining = &self.source[start..];
        let (label, target, length) = if let Some(after) = remaining.strip_prefix('<') {
            let end = after.find('>')?;
            let label = &after[..end];
            if label.contains(['\n', '\r']) {
                return None;
            }
            let target = if label.starts_with("https://") || label.starts_with("http://") {
                label.to_owned()
            } else if label.contains('@')
                && !label.contains(|character: char| {
                    character.is_whitespace() || "<>".contains(character)
                })
            {
                format!("mailto:{label}")
            } else {
                return None;
            };
            (label, target, end + 2)
        } else if remaining.starts_with("https://") || remaining.starts_with("http://") {
            let end = remaining
                .find(|character: char| character.is_whitespace() || "<>\"".contains(character))
                .unwrap_or(remaining.len());
            let label = remaining[..end].trim_end_matches(['.', ',', ';', ':', '!', '?']);
            (label, label.to_owned(), label.len())
        } else {
            return None;
        };
        let label = label.to_owned();
        self.position += length;
        let span = self.span(start);
        Some(link_node(
            span,
            &target,
            "",
            vec![Node {
                kind: NodeKind::Text(label),
                span,
            }],
        ))
    }
}
fn destination(raw: &str) -> Option<(&str, &str)> {
    let (destination, remainder) = if let Some(after) = raw.strip_prefix('<') {
        let end = after.find('>')?;
        (&after[..end], after[end + 1..].trim())
    } else {
        let end = raw.find(char::is_whitespace).unwrap_or(raw.len());
        (&raw[..end], raw[end..].trim())
    };
    if destination.trim().is_empty() {
        return None;
    }
    let title = if remainder.len() >= 2
        && ((remainder.starts_with('"') && remainder.ends_with('"'))
            || (remainder.starts_with('\'') && remainder.ends_with('\''))
            || (remainder.starts_with('(') && remainder.ends_with(')')))
    {
        &remainder[1..remainder.len() - 1]
    } else {
        remainder
    };
    Some((destination, title))
}
fn link_node(span: Span, destination: &str, title: &str, body: Vec<Node>) -> Node {
    let internal = destination.starts_with('#') && destination.len() > 1;
    let mut arguments = vec![string_argument(
        if internal { "target" } else { "url" },
        if internal {
            &destination[1..]
        } else {
            destination
        },
        span,
    )];
    if internal {
        arguments.push(string_argument("display", "text", span));
    }
    if !title.is_empty() {
        arguments.push(string_argument("title", title, span));
    }
    Node {
        kind: NodeKind::Call(Call {
            from_markdown: true,
            name: if internal { "ref" } else { "link" }.into(),
            arguments,
            bodies: vec![body],
        }),
        span,
    }
}
fn string_argument(name: &str, value: &str, span: Span) -> Argument {
    Argument {
        name: Some(name.into()),
        value: Expression {
            kind: ExpressionKind::String(value.into()),
            span,
        },
        span,
    }
}
