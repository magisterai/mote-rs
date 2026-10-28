//! Evaluation for the first content and attribute subset.

use crate::model::{Content, Document, Element, ElementKind, Properties, Unit, Value};
use crate::syntax::{
    Argument, Call, Diagnostic, Expression, ExpressionKind, Node, NodeKind, Span, SyntaxTree,
};

pub fn evaluate(tree: &SyntaxTree) -> Result<Document, Diagnostic> {
    Ok(Document {
        content: evaluate_nodes(&tree.nodes, false)?,
    })
}

fn evaluate_nodes(nodes: &[Node], inline: bool) -> Result<Vec<Content>, Diagnostic> {
    let mut result = Vec::new();
    let mut paragraph = Vec::new();
    for node in nodes {
        if !inline && matches!(node.kind, NodeKind::ParagraphBreak) {
            flush_paragraph(&mut paragraph, &mut result);
            continue;
        }
        let content = match &node.kind {
            NodeKind::Text(text) => Content::Text(text.clone()),
            NodeKind::LineBreak { hard } => Content::Text(if *hard { "\n" } else { " " }.into()),
            NodeKind::ParagraphBreak => Content::Text("\n\n".into()),
            NodeKind::Heading { level, body } => Content::Element(Element {
                kind: ElementKind::Heading,
                attributes: Properties::from([("level".into(), Value::Number(*level as f64))]),
                children: evaluate_nodes(body, true)?,
            }),
            NodeKind::Strong(body) => Content::Element(Element {
                kind: ElementKind::Strong,
                attributes: Properties::new(),
                children: evaluate_nodes(body, true)?,
            }),
            NodeKind::Call(call) => evaluate_call(call, node.span)?,
            NodeKind::Reference(name) => {
                return Err(Diagnostic::new(
                    node.span,
                    format!("binding references are not supported yet: {name}"),
                ));
            }
        };
        if matches!(&content, Content::Element(element) if element.kind.is_block()) {
            if inline {
                return Err(Diagnostic::new(
                    node.span,
                    "inline content cannot contain a block element",
                ));
            }
            flush_paragraph(&mut paragraph, &mut result);
            result.push(content);
            continue;
        }
        push_content(if inline { &mut result } else { &mut paragraph }, content);
    }
    if !inline {
        flush_paragraph(&mut paragraph, &mut result);
    }
    Ok(result)
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

fn evaluate_call(call: &Call, span: Span) -> Result<Content, Diagnostic> {
    let kind = match call.name.as_str() {
        "paragraph" => ElementKind::Paragraph,
        "heading" => ElementKind::Heading,
        "strong" => ElementKind::Strong,
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
    let mut attributes = Properties::new();
    let mut positional_level = None;
    for argument in &call.arguments {
        let value = evaluate_expression(&argument.value)?;
        match &argument.name {
            Some(name) => {
                attributes.insert(
                    name.clone(),
                    normalize_attribute(kind, name, value, argument.value.span)?,
                );
            }
            None if kind == ElementKind::Heading && positional_level.is_none() => {
                if !matches!(value, Value::Number(_)) {
                    return Err(Diagnostic::new(
                        argument.value.span,
                        "positional heading level must be a number",
                    ));
                }
                positional_level = Some(normalize_level(value, argument.value.span)?);
            }
            None => {
                return Err(Diagnostic::new(
                    argument.span,
                    "unexpected positional argument",
                ));
            }
        }
    }
    if let Some(level) = positional_level {
        attributes.insert("level".into(), level);
    }
    Ok(Content::Element(Element {
        kind,
        attributes,
        children: evaluate_nodes(&call.bodies[0], true)?,
    }))
}

fn normalize_attribute(
    kind: ElementKind,
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    if kind == ElementKind::Strong {
        return Err(Diagnostic::new(
            span,
            "strong attributes are not supported yet",
        ));
    }
    match name {
        "level" if kind == ElementKind::Heading => normalize_level(value, span),
        "alignment" => {
            if let Value::String(text) = value {
                let text = text.trim().to_ascii_lowercase();
                if matches!(text.as_str(), "left" | "center" | "right" | "justify") {
                    return Ok(Value::String(text));
                }
            }
            Err(Diagnostic::new(
                span,
                "alignment must be left, center, right, or justify",
            ))
        }
        "keep_with_next" | "keep_together" | "page_break_before" | "widow_control" => match value {
            Value::Boolean(_) => Ok(value),
            Value::String(text) => match text.as_str() {
                "true" | "True" | "TRUE" | "t" | "T" | "1" => Ok(Value::Boolean(true)),
                "false" | "False" | "FALSE" | "f" | "F" | "0" => Ok(Value::Boolean(false)),
                _ => Err(Diagnostic::new(span, format!("{name} must be a boolean"))),
            },
            _ => Err(Diagnostic::new(span, format!("{name} must be a boolean"))),
        },
        "spacing" => {
            let Value::Object(fields) = value else {
                return Err(Diagnostic::new(span, "spacing must be a property object"));
            };
            let mut result = Properties::new();
            for (key, value) in fields {
                if !matches!(key.as_str(), "before" | "after") {
                    return Err(Diagnostic::new(
                        span,
                        format!("unsupported spacing property: {key}"),
                    ));
                }
                result.insert(key, normalize_length(value, span)?);
            }
            Ok(Value::Object(result))
        }
        _ => Err(Diagnostic::new(
            span,
            format!("unsupported {} attribute: {name}", kind.as_text()),
        )),
    }
}

fn normalize_level(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let number = match value {
        Value::Number(number) => Some(number),
        Value::String(text) => text.parse::<u32>().ok().map(f64::from),
        _ => None,
    };
    match number {
        Some(number) if number.is_finite() && number >= 1.0 && number.fract() == 0.0 => {
            Ok(Value::Number(number))
        }
        _ => Err(Diagnostic::new(
            span,
            "heading level must be a positive integer",
        )),
    }
}

fn numeric_value(value: &Value) -> Option<f64> {
    let number = match value {
        Value::Number(number) => *number,
        Value::String(text) => text.parse::<f64>().ok()?,
        _ => return None,
    };
    number.is_finite().then_some(number)
}

fn normalize_length(value: Value, span: Span) -> Result<Value, Diagnostic> {
    if let Value::Length {
        value: magnitude, ..
    } = &value
    {
        if *magnitude >= 0.0 && magnitude.is_finite() {
            return Ok(value);
        }
    } else if let Some(magnitude) = numeric_value(&value) {
        if magnitude >= 0.0 {
            return Ok(value);
        }
    } else if let Value::String(text) = &value {
        for name in ["twip", "pt", "in", "cm", "mm"] {
            let magnitude = text
                .strip_suffix(name)
                .and_then(|text| text.trim().parse::<f64>().ok())
                .filter(|number| *number >= 0.0 && number.is_finite());
            if let Some(magnitude) = magnitude {
                return Ok(Value::Length {
                    value: magnitude,
                    unit: Unit::from_name(name).unwrap(),
                });
            }
        }
    }
    Err(Diagnostic::new(
        span,
        "spacing must be a non-negative length",
    ))
}

fn evaluate_expression(expression: &Expression) -> Result<Value, Diagnostic> {
    Ok(match &expression.kind {
        ExpressionKind::String(text) => Value::String(text.clone()),
        ExpressionKind::Number(number) => Value::Number(*number),
        ExpressionKind::Boolean(boolean) => Value::Boolean(*boolean),
        ExpressionKind::Nil => Value::Nil,
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
    if unit.is_none() && name != "upper" {
        return Err(Diagnostic::new(span, format!("unsupported helper: {name}")));
    }
    if arguments.len() != 1 || arguments[0].name.is_some() {
        return Err(Diagnostic::new(
            span,
            format!("{name} requires one positional argument"),
        ));
    }
    let value = evaluate_expression(&arguments[0].value)?;
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
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Boolean(boolean) => boolean.to_string(),
        Value::Nil => String::new(),
        Value::Length { value, unit } => format!("{value}{}", unit.as_text()),
        Value::Object(_) => {
            return Err(Diagnostic::new(
                span,
                "property objects cannot be inserted as text in this subset",
            ));
        }
    })
}
