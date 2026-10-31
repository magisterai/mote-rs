//! Shared element signatures, attribute normalization, and content validation.
use super::{
    ContentMode, EvaluationContext, EvaluationState, attributes, evaluate_body,
    expressions::{self, EvaluatedArguments},
    references::NamedLocation,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Element, ElementKind, Properties, Value},
    syntax::Call,
};

pub(super) const ELEMENT_NAMES: &[&str] = &[
    "paragraph",
    "heading",
    "span",
    "link",
    "image",
    "shape",
    "chart",
    "slide",
    "box",
    "line",
    "table",
    "row",
    "cell",
    "strong",
    "emphasis",
    "underline",
    "strike",
    "highlight",
    "small_caps",
    "subscript",
    "superscript",
    "code",
    "raw",
    "definition_list",
    "term",
    "definition",
    "divider",
    "footnote",
    "endnote",
    "page_number",
    "bookmark",
    "label",
    "ref",
    "figure",
    "caption",
    "line_break",
    "tab",
    "break",
    "list",
    "ordered_list",
    "item",
    "blockquote",
];

#[derive(Clone, Copy)]
pub(super) struct Signature {
    pub kind: ElementKind,
}
impl Signature {
    pub fn find(name: &str) -> Option<Self> {
        if !ELEMENT_NAMES.contains(&name) {
            return None;
        }
        Some(Self {
            kind: ElementKind::from_name(if name == "item" { "list_item" } else { name })?,
        })
    }
    pub fn primary(self) -> Option<&'static str> {
        match self.kind {
            ElementKind::Heading => Some("level"),
            ElementKind::Link => Some("url"),
            ElementKind::Image => Some("src"),
            ElementKind::Bookmark | ElementKind::Label => Some("name"),
            ElementKind::Reference => Some("target"),
            ElementKind::Code | ElementKind::Raw => Some("text"),
            _ => None,
        }
    }
    pub fn body_allowed(self) -> bool {
        !matches!(
            self.kind,
            ElementKind::Image
                | ElementKind::Chart
                | ElementKind::Line
                | ElementKind::Label
                | ElementKind::Reference
                | ElementKind::PageNumber
                | ElementKind::LineBreak
                | ElementKind::Tab
                | ElementKind::Break
                | ElementKind::Divider
        )
    }
    fn body_required(self) -> bool {
        self.body_allowed()
            && !matches!(
                self.kind,
                ElementKind::Code | ElementKind::Raw | ElementKind::Bookmark | ElementKind::Shape
            )
    }
    pub fn body_mode(self) -> ContentMode {
        match self.kind {
            ElementKind::List | ElementKind::OrderedList => ContentMode::ListBody,
            ElementKind::Table | ElementKind::Row => ContentMode::Structural,
            ElementKind::Shape
            | ElementKind::Slide
            | ElementKind::Box
            | ElementKind::Cell
            | ElementKind::ListItem
            | ElementKind::Blockquote
            | ElementKind::Figure
            | ElementKind::Footnote
            | ElementKind::Endnote
            | ElementKind::Definition => ContentMode::Block,
            ElementKind::DefinitionList => ContentMode::Structural,
            _ => ContentMode::Inline,
        }
    }
    pub fn prepare(
        self,
        mut arguments: EvaluatedArguments,
        bodies: usize,
        from_markdown: bool,
        span: Span,
        defaults: &Properties,
    ) -> Result<PreparedElement, Diagnostic> {
        if matches!(self.kind, ElementKind::Code | ElementKind::Raw) {
            if bodies > 1 {
                return Err(Diagnostic::new(
                    span,
                    "code accepts at most one content body",
                ));
            }
            if arguments.positional.len() > 1 {
                return Err(Diagnostic::new(
                    span,
                    "code accepts at most one positional argument",
                ));
            }
        }
        if matches!(
            self.kind,
            ElementKind::LineBreak
                | ElementKind::Tab
                | ElementKind::Break
                | ElementKind::Divider
                | ElementKind::PageNumber
        ) && (bodies > 0 || !arguments.positional.is_empty())
        {
            return Err(Diagnostic::new(
                span,
                format!(
                    "{} accepts no positional arguments or content body",
                    self.kind.as_text()
                ),
            ));
        }
        let declares_slot = self.kind == ElementKind::Box && (arguments.named.contains_key("slot") || defaults.contains_key("slot") || arguments.named.get("properties").is_some_and(|argument| matches!(&argument.value, Value::Object(fields) if fields.contains_key("slot"))));
        if (self.body_required() && !declares_slot && bodies != 1)
            || bodies > usize::from(self.body_allowed() || from_markdown)
        {
            return Err(Diagnostic::new(
                span,
                format!(
                    "{} requires {} content body",
                    self.kind.as_text(),
                    if self.body_required() {
                        "one"
                    } else {
                        "at most one permitted"
                    }
                ),
            ));
        }
        if arguments.positional.len() > usize::from(self.primary().is_some()) {
            return Err(Diagnostic::new(span, "unexpected positional argument"));
        }
        if let Some(primary) = self.primary()
            && !matches!(self.kind, ElementKind::Code | ElementKind::Raw)
            && !arguments.positional.is_empty()
            && arguments.named.contains_key(primary)
        {
            return Err(Diagnostic::new(
                span,
                "primary argument cannot be supplied both positionally and by name",
            ));
        }
        if matches!(
            self.kind,
            ElementKind::Image | ElementKind::Link | ElementKind::Bookmark | ElementKind::Label
        ) {
            let primary = self.primary().unwrap();
            if arguments.positional.is_empty() && !arguments.named.contains_key(primary) {
                return Err(Diagnostic::new(
                    span,
                    format!(
                        "{} requires an explicit {primary} argument",
                        self.kind.as_text()
                    ),
                ));
            }
        }
        for required in match self.kind {
            ElementKind::Shape => &["position"][..],
            ElementKind::Line => &["from", "to"][..],
            _ => &[][..],
        } {
            if !arguments.named.contains_key(*required) {
                return Err(Diagnostic::new(
                    span,
                    format!(
                        "{} requires an explicit {required} argument",
                        self.kind.as_text()
                    ),
                ));
            }
        }
        let text = if matches!(self.kind, ElementKind::Code | ElementKind::Raw) {
            arguments.positional.pop()
        } else {
            None
        };
        if self.kind == ElementKind::Chart && !arguments.named.contains_key("data") {
            return Err(Diagnostic::new(
                span,
                "chart requires a named data argument",
            ));
        }
        let mut attributes = attributes::normalize_arguments(self.kind, arguments, defaults)?;
        if self.kind == ElementKind::Break {
            attributes
                .entry("type".into())
                .or_insert_with(|| Value::String("page".into()));
        }
        match self.kind {
            ElementKind::Image | ElementKind::Link | ElementKind::Bookmark | ElementKind::Label => {
                let primary = self.primary().unwrap();
                if attributes.get(primary).is_none_or(|value| {
                    if self.kind == ElementKind::Label {
                        value.to_text().trim().is_empty()
                    } else {
                        value.to_text().is_empty()
                    }
                }) {
                    return Err(Diagnostic::new(
                        span,
                        format!("{} requires {primary}", self.kind.as_text()),
                    ));
                }
            }
            ElementKind::Reference
                if !matches!(
                    attributes.get("target"),
                    Some(Value::String(_) | Value::Object(_))
                ) || crate::references::target(&attributes).is_empty() =>
            {
                return Err(Diagnostic::new(span, "ref requires a target label"));
            }
            _ => {}
        }
        if matches!(
            self.kind,
            ElementKind::Shape | ElementKind::Box | ElementKind::Line
        ) && !(self.kind == ElementKind::Box
            && !crate::properties::text(&attributes, "slot").is_empty())
        {
            for required in if self.kind != ElementKind::Line {
                &["position"][..]
            } else {
                &["from", "to"][..]
            } {
                if !attributes.contains_key(*required) {
                    return Err(Diagnostic::new(
                        span,
                        format!("{} requires {required}", self.kind.as_text()),
                    ));
                }
            }
        }
        let text = text
            .map(|argument| {
                if !matches!(
                    argument.value,
                    Value::String(_) | Value::Number(_) | Value::Boolean(_)
                ) {
                    return Err(Diagnostic::new(
                        argument.span,
                        "code text must be a string, number, or boolean",
                    ));
                }
                Ok(argument.value.to_text())
            })
            .transpose()?;
        Ok(PreparedElement {
            signature: self,
            attributes,
            text,
        })
    }
}
pub(super) struct PreparedElement {
    pub signature: Signature,
    pub attributes: Properties,
    text: Option<String>,
}
impl PreparedElement {
    pub fn preserve_space(&self, inherited: bool) -> bool {
        inherited
            || matches!(self.signature.kind, ElementKind::Code | ElementKind::Raw)
                && self.attributes.get("preserve_space") == Some(&Value::Boolean(true))
    }
    pub fn finish(
        self,
        mut children: Vec<Content>,
        span: Span,
        state: &mut EvaluationState<'_>,
    ) -> Result<Element, Diagnostic> {
        let kind = self.signature.kind;
        if self.signature.body_mode() == ContentMode::Inline && children.iter().any(|child| matches!(child,Content::Element(element) if element.kind.is_block() || element.kind.is_table_part())) {
            return Err(Diagnostic::new(span,"inline content cannot contain a block element"));
        }
        if kind == ElementKind::DefinitionList {
            children
                .retain(|child| !matches!(child, Content::Text(text) if text.trim().is_empty()));
            if children.iter().any(|child| !matches!(child, Content::Element(element) if matches!(element.kind, ElementKind::Term | ElementKind::Definition))) {
                return Err(Diagnostic::new(span, "definition_list bodies accept only term and definition elements"));
            }
            if children.is_empty() {
                state
                    .warnings
                    .push(Diagnostic::new(span, "definition_list body is empty"));
            }
        }
        let required_child = match kind {
            ElementKind::Table => Some(ElementKind::Row),
            ElementKind::Row => Some(ElementKind::Cell),
            ElementKind::List | ElementKind::OrderedList => Some(ElementKind::ListItem),
            _ => None,
        };
        if let Some(required) = required_child {
            children.retain(|child| !matches!(child,Content::Text(text) if text.trim().is_empty()));
            if children
                .iter()
                .any(|child| !matches!(child,Content::Element(element) if element.kind == required))
            {
                if matches!(kind, ElementKind::List | ElementKind::OrderedList) {
                    return Err(Diagnostic::new(
                        span,
                        "list bodies accept only items and separating whitespace",
                    ));
                }
                return Err(Diagnostic::new(
                    span,
                    format!(
                        "{} bodies contain only {} elements and whitespace",
                        kind.as_text(),
                        required.as_text()
                    ),
                ));
            }
            if matches!(kind,ElementKind::List | ElementKind::OrderedList) && children.iter().any(|child| matches!(child,Content::Element(item) if item.children.iter().any(|child| matches!(child,Content::Element(element) if element.kind == ElementKind::ListItem)))) {
                return Err(Diagnostic::new(span,"nested Markdown items in an explicit list require an item body or a nested list container"));
            }
            if children.is_empty() && matches!(kind, ElementKind::Table | ElementKind::Row) {
                state.warn(Diagnostic::new(
                    span,
                    format!(
                        "{} body is empty; emitting an empty {}",
                        kind.as_text(),
                        kind.as_text()
                    ),
                ));
            }
        }
        if matches!(
            kind,
            ElementKind::Shape | ElementKind::Slide | ElementKind::Box
        ) {
            children.retain(|child| !matches!(child,Content::Text(text) if text.trim().is_empty()));
        }
        if matches!(kind, ElementKind::Slide | ElementKind::Box) {
            super::presentation::validate_children(kind, &children, span)?;
            if children.is_empty()
                && !(kind == ElementKind::Box
                    && !crate::properties::text(&self.attributes, "slot").is_empty())
            {
                state.warn(Diagnostic::new(
                    span,
                    format!(
                        "{} body is empty; emitting an empty {}",
                        kind.as_text(),
                        kind.as_text()
                    ),
                ));
            }
        }
        if kind == ElementKind::Figure && children.iter().filter(|child| matches!(child,Content::Element(element) if element.kind == ElementKind::Caption)).count() > 1 {
            return Err(Diagnostic::new(span,"figure can contain at most one caption"));
        }
        if let Some(text) = self.text {
            children = vec![Content::Text(text)];
        }
        if kind == ElementKind::Label {
            state.labels.push(NamedLocation {
                name: self.attributes["name"].to_text(),
                span,
                story_declaration: state.story_declaration,
            });
        } else if kind == ElementKind::Reference {
            let target = match &self.attributes["target"] {
                Value::Object(fields) => fields
                    .get("label")
                    .or_else(|| fields.get("name"))
                    .map(Value::to_text)
                    .unwrap_or_default(),
                value => value.to_text(),
            };
            state.references.push(NamedLocation {
                name: target,
                span,
                story_declaration: state.story_declaration,
            });
        }
        Ok(Element {
            source_name: state.source_name().map(ToOwned::to_owned),
            kind,
            attributes: self.attributes,
            children,
        })
    }
}

pub(super) fn evaluate(
    signature: Signature,
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<Content, Diagnostic> {
    let arguments = expressions::evaluate_arguments(&call.arguments, state)?;
    let prepared = signature.prepare(
        arguments,
        call.bodies.len(),
        call.from_markdown,
        span,
        &state.bindings.element_defaults(signature.kind),
    )?;
    let preserve_space = prepared.preserve_space(context.preserve_space);
    let context = EvaluationContext {
        preserve_space,
        ..context
    };
    let mut children = call
        .bodies
        .first()
        .map(|body| evaluate_body(body, signature.body_mode(), context, state))
        .transpose()?
        .unwrap_or_default();
    if preserve_space && matches!(signature.kind, ElementKind::Code | ElementKind::Raw) {
        super::normalize_preserved_content(&mut children);
    }
    let mut element = prepared.finish(children, span, state)?;
    if preserve_space && matches!(signature.kind, ElementKind::Code | ElementKind::Raw) {
        super::normalize_preserved_content(&mut element.children);
    }
    Ok(Content::Element(element))
}
