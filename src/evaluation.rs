//! Evaluation of content directives and their formatting attributes.

use crate::model::{Content, Document, Element, ElementKind, Properties, Unit, Value};
use crate::syntax::{
    Argument, Call, Diagnostic, Expression, ExpressionKind, Node, NodeKind, Span, SyntaxTree,
};

mod attributes;
mod values;

use attributes::evaluate_attributes;
use values::{normalize_color, numeric_value};

#[derive(Clone, Copy)]
struct EvaluationContext<'source> {
    source: &'source str,
    preserve_space: bool,
}

pub fn evaluate(tree: &SyntaxTree) -> Result<Document, Diagnostic> {
    Ok(Document {
        content: evaluate_nodes(
            &tree.nodes,
            ContentMode::Block,
            EvaluationContext {
                source: &tree.source,
                preserve_space: false,
            },
        )?,
    })
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
) -> Result<Vec<Content>, Diagnostic> {
    let mut result = Vec::new();
    let mut paragraph = Vec::new();
    for node in nodes {
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
                attributes: Properties::from([("level".into(), Value::Number(*level as f64))]),
                children: evaluate_nodes(body, ContentMode::Inline, context)?,
            }),
            NodeKind::Strong(body) => Content::Element(Element {
                kind: ElementKind::Strong,
                attributes: Properties::new(),
                children: evaluate_nodes(body, ContentMode::Inline, context)?,
            }),
            NodeKind::Emphasis(body) => Content::Element(Element {
                kind: ElementKind::Emphasis,
                attributes: Properties::new(),
                children: evaluate_nodes(body, ContentMode::Inline, context)?,
            }),
            NodeKind::Code(text) => Content::Element(Element {
                kind: ElementKind::Code,
                attributes: Properties::new(),
                children: vec![Content::Text(text.clone())],
            }),
            NodeKind::ListItem { marker, body } => Content::Element(Element {
                kind: ElementKind::ListItem,
                attributes: Properties::from([("marker".into(), Value::String(marker.clone()))]),
                children: evaluate_nodes(body, ContentMode::MarkdownItem, context)?,
            }),
            NodeKind::Blockquote(body) => Content::Element(Element {
                kind: ElementKind::Blockquote,
                attributes: Properties::new(),
                children: evaluate_nodes(body, ContentMode::Block, context)?,
            }),
            NodeKind::Call(call) => evaluate_call(call, node.span, context)?,
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
        push_content(
            if mode == ContentMode::Block {
                &mut paragraph
            } else {
                &mut result
            },
            content,
        );
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
                let ordered = matches!(element.attributes.get("marker"), Some(Value::String(marker)) if marker.as_bytes().first().is_some_and(u8::is_ascii_digit));
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
        {
            result.push(Content::Element(current_list.take().unwrap()));
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

fn push_content(output: &mut Vec<Content>, content: Content) {
    if let Content::Text(text) = &content {
        if text.is_empty() {
            return;
        }
        if let Some(Content::Text(previous)) = output.last_mut() {
            previous.push_str(text);
            return;
        }
    }
    output.push(content);
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
) -> Result<Content, Diagnostic> {
    if call.name == "code" {
        return evaluate_code(call, span, context);
    }
    if call.name == "line_break" {
        if !call.arguments.is_empty() || !call.bodies.is_empty() {
            return Err(Diagnostic::new(
                span,
                "line_break accepts no arguments or content body",
            ));
        }
        return Ok(Content::Element(Element {
            kind: ElementKind::LineBreak,
            attributes: Properties::new(),
            children: Vec::new(),
        }));
    }
    let kind = match call.name.as_str() {
        "paragraph" => ElementKind::Paragraph,
        "heading" => ElementKind::Heading,
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
    let mut children = evaluate_nodes(&call.bodies[0], body_mode, context)?;
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
        evaluate_nodes(body, ContentMode::Inline, context)?
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

fn evaluate_expression(expression: &Expression) -> Result<Value, Diagnostic> {
    Ok(match &expression.kind {
        ExpressionKind::String(text) => Value::String(text.clone()),
        ExpressionKind::Number(number) => Value::Number(*number),
        ExpressionKind::Boolean(boolean) => Value::Boolean(*boolean),
        ExpressionKind::Nil => Value::Nil,
        ExpressionKind::Array(entries) => Value::Array(
            entries
                .iter()
                .map(evaluate_expression)
                .collect::<Result<_, _>>()?,
        ),
        ExpressionKind::Object(fields) => {
            let mut object = Properties::new();
            for field in fields {
                let Some(name) = &field.name else {
                    return Err(Diagnostic::new(field.span, "object fields must be named"));
                };
                object.insert(name.clone(), evaluate_expression(&field.value)?);
            }
            Value::Object(object)
        }
        ExpressionKind::Call { name, arguments } => {
            evaluate_helper(name, arguments, expression.span)?
        }
        ExpressionKind::Path(name) => {
            return Err(Diagnostic::new(
                expression.span,
                format!("binding references are not supported yet: {name}"),
            ));
        }
    })
}

fn evaluate_helper(name: &str, arguments: &[Argument], span: Span) -> Result<Value, Diagnostic> {
    let unit = Unit::from_name(name);
    if unit.is_none() && !matches!(name, "upper" | "color") {
        return Err(Diagnostic::new(span, format!("unsupported helper: {name}")));
    }
    if arguments.len() != 1 || arguments[0].name.is_some() {
        return Err(Diagnostic::new(
            span,
            format!("{name} requires one positional argument"),
        ));
    }
    let value = evaluate_expression(&arguments[0].value)?;
    if name == "color" {
        return normalize_color(value, arguments[0].value.span, "color");
    }
    if let Some(unit) = unit {
        let Some(magnitude) = numeric_value(&value) else {
            return Err(Diagnostic::new(
                arguments[0].value.span,
                "length magnitude must be a finite number",
            ));
        };
        return Ok(Value::Length {
            value: magnitude,
            unit,
        });
    }
    let text = value_as_text(&value, arguments[0].value.span)?;
    if !text.is_ascii() {
        return Err(Diagnostic::new(
            span,
            "upper currently supports ASCII text only",
        ));
    }
    Ok(Value::String(text.to_ascii_uppercase()))
}

fn value_as_text(value: &Value, span: Span) -> Result<String, Diagnostic> {
    Ok(match value {
        Value::String(text) | Value::Color(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Boolean(boolean) => boolean.to_string(),
        Value::Nil => String::new(),
        Value::Length { value, unit } => format!("{value}{}", unit.as_text()),
        Value::Object(_) | Value::Array(_) => {
            return Err(Diagnostic::new(
                span,
                "property objects and arrays cannot be inserted as text",
            ));
        }
    })
}
