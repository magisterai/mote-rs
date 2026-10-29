//! Evaluation of content directives and their formatting attributes.

use crate::diagnostics::{Diagnostic, Span};
use crate::model::{Content, Document, Element, ElementKind, Properties, Value};
use crate::syntax::{Call, Node, NodeKind, SyntaxTree};

mod attributes;
mod configuration;
mod declarations;
mod expressions;
mod values;

use attributes::evaluate_attributes;
use expressions::{evaluate_expression, evaluate_helper, value_as_text};

#[derive(Clone, Copy)]
struct EvaluationContext<'source> {
    source: &'source str,
    preserve_space: bool,
}

pub fn evaluate(tree: &SyntaxTree) -> Result<Document, Diagnostic> {
    let mut document = Document::default();
    document.content = evaluate_nodes(
        &tree.nodes,
        ContentMode::Block,
        EvaluationContext {
            source: &tree.source,
            preserve_space: false,
        },
        &mut document,
    )?;
    for style in document.styles.values() {
        let parent = style
            .properties
            .get("based_on")
            .map(Value::to_text)
            .unwrap_or_default();
        if parent == style.name {
            return Err(Diagnostic::new(
                style.span,
                format!("style {:?} cannot be based on itself", style.name),
            ));
        }
    }
    Ok(document)
}

#[derive(Clone, Copy, PartialEq)]
enum ContentMode {
    Block,
    Inline,
    MarkdownItem,
    ListBody,
}

fn evaluate_nodes(
    nodes: &[Node],
    mode: ContentMode,
    context: EvaluationContext<'_>,
    document: &mut Document,
) -> Result<Vec<Content>, Diagnostic> {
    let mut result = Vec::new();
    let mut paragraph = Vec::new();
    let mut skip_declaration_trivia = false;
    for node in nodes {
        if skip_declaration_trivia
            && mode == ContentMode::Block
            && (matches!(
                node.kind,
                NodeKind::LineBreak { .. } | NodeKind::ParagraphBreak
            ) || matches!(&node.kind, NodeKind::Text(text) if text.trim().is_empty()))
        {
            skip_declaration_trivia = false;
            continue;
        }
        skip_declaration_trivia = false;
        if let NodeKind::Call(call) = &node.kind
            && call.name == "document"
        {
            configuration::evaluate(call, node.span, document)?;
            skip_declaration_trivia = true;
            continue;
        }
        if let NodeKind::Call(call) = &node.kind
            && matches!(call.name.as_str(), "font" | "style")
        {
            declarations::evaluate_declaration(call, node.span, document)?;
            skip_declaration_trivia = true;
            continue;
        }
        if mode == ContentMode::Block && matches!(node.kind, NodeKind::ParagraphBreak) {
            flush_paragraph(&mut paragraph, &mut result);
            continue;
        }
        if !context.preserve_space
            && matches!(node.kind, NodeKind::LineBreak { hard: true })
            && !context
                .source
                .get(node.span.start..node.span.end)
                .is_some_and(|source| source.starts_with('\\'))
        {
            let preceding = if mode == ContentMode::Block {
                &mut paragraph
            } else {
                &mut result
            };
            if let Some(Content::Text(text)) = preceding.last_mut() {
                text.truncate(text.trim_end_matches(' ').len());
            }
        }
        let content = match &node.kind {
            NodeKind::Text(text) => Content::Text(text.clone()),
            NodeKind::LineBreak { hard } => Content::Text(
                if *hard || context.preserve_space {
                    "\n"
                } else {
                    " "
                }
                .into(),
            ),
            NodeKind::ParagraphBreak => Content::Text(if context.preserve_space {
                normalize_line_endings(
                    context
                        .source
                        .get(node.span.start..node.span.end)
                        .unwrap_or("\n\n"),
                )
            } else {
                "\n\n".into()
            }),
            NodeKind::Heading { level, body } => Content::Element(Element {
                kind: ElementKind::Heading,
                attributes: Properties::from([
                    ("level".into(), Value::Number(*level as f64)),
                    ("marker".into(), Value::String("#".into())),
                ]),
                children: evaluate_nodes(body, ContentMode::Inline, context, document)?,
            }),
            NodeKind::Strong(body) => Content::Element(Element {
                kind: ElementKind::Strong,
                attributes: Properties::new(),
                children: evaluate_nodes(body, ContentMode::Inline, context, document)?,
            }),
            NodeKind::Emphasis(body) => Content::Element(Element {
                kind: ElementKind::Emphasis,
                attributes: Properties::new(),
                children: evaluate_nodes(body, ContentMode::Inline, context, document)?,
            }),
            NodeKind::Code(text) => Content::Element(Element {
                kind: ElementKind::Code,
                attributes: Properties::new(),
                children: vec![Content::Text(text.clone())],
            }),
            NodeKind::ListItem { marker, body } => Content::Element(Element {
                kind: ElementKind::ListItem,
                attributes: Properties::from([("marker".into(), Value::String(marker.clone()))]),
                children: evaluate_nodes(body, ContentMode::MarkdownItem, context, document)?,
            }),
            NodeKind::Blockquote(body) => Content::Element(Element {
                kind: ElementKind::Blockquote,
                attributes: Properties::new(),
                children: evaluate_nodes(body, ContentMode::Block, context, document)?,
            }),
            NodeKind::Call(call) => evaluate_call(call, node.span, context, document)?,
            NodeKind::Reference(name) => {
                return Err(Diagnostic::new(
                    node.span,
                    format!("binding references are not supported yet: {name}"),
                ));
            }
        };
        if matches!(&content, Content::Element(element) if element.kind.is_block()) {
            if mode == ContentMode::Inline {
                return Err(Diagnostic::new(
                    node.span,
                    "inline content cannot contain a block element",
                ));
            }
            flush_paragraph(&mut paragraph, &mut result);
            result.push(content);
            continue;
        }
        let destination = if mode == ContentMode::Block {
            &mut paragraph
        } else {
            &mut result
        };
        destination.push(content);
    }
    if mode == ContentMode::Block {
        flush_paragraph(&mut paragraph, &mut result);
    }
    if mode == ContentMode::Block {
        Ok(group_adjacent_list_items(result))
    } else {
        Ok(result)
    }
}

fn group_adjacent_list_items(content: Vec<Content>) -> Vec<Content> {
    let mut result = Vec::new();
    let mut current_list: Option<Element> = None;
    for mut node in content {
        // Explicit containers have already validated their item-only bodies.
        match &mut node {
            Content::Element(element)
                if !matches!(element.kind, ElementKind::List | ElementKind::OrderedList) =>
            {
                element.children = group_adjacent_list_items(std::mem::take(&mut element.children));
            }
            _ => {}
        }
        let list_kind = match &node {
            Content::Element(element) if element.kind == ElementKind::ListItem => {
                let ordered = element
                    .attributes
                    .get("marker")
                    .map(Value::to_text)
                    .is_some_and(|marker| {
                        marker.as_bytes().first().is_some_and(u8::is_ascii_digit)
                    });
                Some(if ordered {
                    ElementKind::OrderedList
                } else {
                    ElementKind::List
                })
            }
            _ => None,
        };
        if current_list
            .as_ref()
            .is_some_and(|list| Some(list.kind) != list_kind)
            && let Some(list) = current_list.take()
        {
            result.push(Content::Element(list));
        }
        if let Some(kind) = list_kind {
            current_list
                .get_or_insert_with(|| Element {
                    kind,
                    attributes: Properties::new(),
                    children: Vec::new(),
                })
                .children
                .push(node);
        } else {
            result.push(node);
        }
    }
    if let Some(list) = current_list {
        result.push(Content::Element(list));
    }
    result
}

fn flush_paragraph(buffer: &mut Vec<Content>, result: &mut Vec<Content>) {
    if let Some(Content::Text(text)) = buffer.first_mut() {
        *text = text.trim_start_matches([' ', '\t', '\r', '\n']).to_owned();
        if text.is_empty() {
            buffer.remove(0);
        }
    }
    if let Some(Content::Text(text)) = buffer.last_mut() {
        text.truncate(text.trim_end_matches([' ', '\t', '\r', '\n']).len());
        if text.is_empty() {
            buffer.pop();
        }
    }
    if !buffer.is_empty() {
        result.push(Content::Element(Element {
            kind: ElementKind::Paragraph,
            attributes: Properties::new(),
            children: std::mem::take(buffer),
        }));
    }
}

fn evaluate_call(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    document: &mut Document,
) -> Result<Content, Diagnostic> {
    if call.name == "code" {
        return evaluate_code(call, span, context, document);
    }
    if matches!(call.name.as_str(), "line_break" | "tab" | "break") {
        let kind = match call.name.as_str() {
            "tab" => ElementKind::Tab,
            "break" => ElementKind::Break,
            _ => ElementKind::LineBreak,
        };
        if call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
            || !call.bodies.is_empty()
        {
            return Err(Diagnostic::new(
                span,
                format!(
                    "{} accepts no positional arguments or content body",
                    call.name
                ),
            ));
        }
        let mut attributes = evaluate_attributes(kind, &call.arguments)?;
        if kind == ElementKind::Break {
            attributes
                .entry("type".into())
                .or_insert_with(|| Value::String("page".into()));
        }
        return Ok(Content::Element(Element {
            kind,
            attributes,
            children: Vec::new(),
        }));
    }
    let kind = match call.name.as_str() {
        "paragraph" => ElementKind::Paragraph,
        "heading" => ElementKind::Heading,
        "span" => ElementKind::Span,
        "strong" => ElementKind::Strong,
        "emphasis" => ElementKind::Emphasis,
        "list" => ElementKind::List,
        "ordered_list" => ElementKind::OrderedList,
        "item" => ElementKind::ListItem,
        "blockquote" => ElementKind::Blockquote,
        _ => {
            if !call.bodies.is_empty() {
                return Err(Diagnostic::new(
                    span,
                    format!("unsupported directive: {}", call.name),
                ));
            }
            return Ok(Content::Text(value_as_text(
                &evaluate_helper(&call.name, &call.arguments, span)?,
                span,
            )?));
        }
    };
    if call.bodies.len() != 1 {
        return Err(Diagnostic::new(
            span,
            format!("{} requires one content body", call.name),
        ));
    }
    if kind == ElementKind::Heading
        && call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
        && call
            .arguments
            .iter()
            .any(|argument| argument.name.as_deref() == Some("level"))
    {
        return Err(Diagnostic::new(
            span,
            "heading level cannot be supplied both positionally and by name",
        ));
    }
    let permitted_positional_arguments = usize::from(kind == ElementKind::Heading);
    if let Some(argument) = call
        .arguments
        .iter()
        .filter(|argument| argument.name.is_none())
        .nth(permitted_positional_arguments)
    {
        return Err(Diagnostic::new(
            argument.span,
            "unexpected positional argument",
        ));
    }
    let attributes = evaluate_attributes(kind, &call.arguments)?;
    let body_mode = match kind {
        ElementKind::List | ElementKind::OrderedList => ContentMode::ListBody,
        ElementKind::ListItem | ElementKind::Blockquote => ContentMode::Block,
        _ => ContentMode::Inline,
    };
    let mut children = evaluate_nodes(&call.bodies[0], body_mode, context, document)?;
    if body_mode == ContentMode::ListBody {
        children.retain(|child| !matches!(child, Content::Text(text) if text.trim().is_empty()));
        if children.iter().any(|child| !matches!(child, Content::Element(element) if element.kind == ElementKind::ListItem)) {
            return Err(Diagnostic::new(span, "list bodies accept only items and separating whitespace"));
        }
        for child in &children {
            match child {
                Content::Element(item) if item.children.iter().any(|child| matches!(child, Content::Element(element) if element.kind == ElementKind::ListItem)) => {
                    return Err(Diagnostic::new(span, "nested Markdown items in an explicit list require an item body or a nested list container"));
                }
                _ => {}
            }
        }
    }
    Ok(Content::Element(Element {
        kind,
        attributes,
        children,
    }))
}

fn evaluate_code(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    document: &mut Document,
) -> Result<Content, Diagnostic> {
    if call.bodies.len() > 1 {
        return Err(Diagnostic::new(
            span,
            "code accepts at most one content body",
        ));
    }
    let positional: Vec<_> = call
        .arguments
        .iter()
        .filter(|argument| argument.name.is_none())
        .collect();
    if positional.len() > 1 {
        return Err(Diagnostic::new(
            span,
            "code accepts at most one positional argument",
        ));
    }
    let attributes = evaluate_attributes(ElementKind::Code, &call.arguments)?;
    let preserve_space =
        context.preserve_space || attributes.get("preserve_space") == Some(&Value::Boolean(true));
    let context = EvaluationContext {
        preserve_space,
        ..context
    };
    let mut children = if let Some(body) = call.bodies.first() {
        evaluate_nodes(body, ContentMode::Inline, context, document)?
    } else {
        Vec::new()
    };
    if let Some(argument) = positional.first() {
        let value = evaluate_expression(&argument.value)?;
        if !matches!(
            value,
            Value::String(_) | Value::Number(_) | Value::Boolean(_)
        ) {
            return Err(Diagnostic::new(
                argument.value.span,
                "code text must be a string, number, or boolean",
            ));
        }
        children = vec![Content::Text(value_as_text(&value, argument.value.span)?)];
    }
    if preserve_space {
        normalize_preserved_content(&mut children);
    }
    Ok(Content::Element(Element {
        kind: ElementKind::Code,
        attributes,
        children,
    }))
}

fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn normalize_preserved_content(content: &mut [Content]) {
    for node in content {
        match node {
            Content::Text(text) => *text = normalize_line_endings(text),
            Content::Element(element) => normalize_preserved_content(&mut element.children),
        }
    }
}
