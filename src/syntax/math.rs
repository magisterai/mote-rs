use super::{Argument, Call, Expression, ExpressionKind, Node, NodeKind, Parser};
impl Parser<'_> {
    pub(super) fn parse_math(&mut self) -> Option<Node> {
        let start = self.position;
        let (opening, closing, display) = if self.starts_with("\\(") {
            ("\\(", "\\)", "inline")
        } else if self.starts_with("\\[") {
            ("\\[", "\\]", "block")
        } else if self.starts_with("$") {
            if !dollar_can_open(self.source, start) {
                return None;
            }
            ("$", "$", "inline")
        } else {
            return None;
        };
        let content_start = start + opening.len();
        let mut closing_position = None;
        let mut escaped = false;
        for (offset, character) in self.source[content_start..].char_indices() {
            if opening == "$" {
                if matches!(character, '\n' | '\r') {
                    break;
                }
                if escaped {
                    escaped = false;
                    continue;
                }
                if character == '\\' {
                    escaped = true;
                    continue;
                }
            }
            if self.source[content_start + offset..].starts_with(closing) {
                closing_position = Some(content_start + offset);
                break;
            }
        }
        let end = closing_position?;
        self.position = end + closing.len();
        let span = self.span(start);
        let arguments = [
            ("source", self.source[content_start..end].to_owned()),
            ("display", display.into()),
            ("format", "latex".into()),
        ]
        .into_iter()
        .map(|(name, text)| Argument {
            name: Some(name.into()),
            value: Expression {
                kind: ExpressionKind::String(text),
                span,
            },
            span,
        })
        .collect();
        Some(Node {
            kind: NodeKind::Call(Call {
                from_markdown: true,
                name: "math".into(),
                arguments,
                bodies: Vec::new(),
            }),
            span,
        })
    }
}

pub(super) fn dollar_can_open(source: &str, position: usize) -> bool {
    source[position + 1..].chars().next().is_some_and(|next| {
        next != '\0'
            && next != '$'
            && !next.is_whitespace()
            && unicode_general_category::get_general_category(next)
                != unicode_general_category::GeneralCategory::DecimalNumber
    })
}
