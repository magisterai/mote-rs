use super::{
    ContentMode, EvaluationContext,
    attributes::evaluate_attributes,
    declarations::{TableGroup, normalize_table_group},
    evaluate_nodes,
    values::*,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Document, Element, ElementKind, Properties, Value},
    syntax::Call,
};

pub(super) fn evaluate(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    document: &mut Document,
) -> Result<Content, Diagnostic> {
    let kind = match call.name.as_str() {
        "table" => ElementKind::Table,
        "row" => ElementKind::Row,
        _ => ElementKind::Cell,
    };
    if call.bodies.len() != 1
        || call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
    {
        return Err(Diagnostic::new(
            span,
            format!("{} requires one body and named arguments", call.name),
        ));
    }
    let attributes = evaluate_attributes(kind, &call.arguments)?;
    let mode = if kind == ElementKind::Cell {
        ContentMode::Block
    } else {
        ContentMode::Structural
    };
    let mut children = evaluate_nodes(&call.bodies[0], mode, context, document)?;
    if kind != ElementKind::Cell {
        children.retain(|child| !matches!(child, Content::Text(text) if text.trim().is_empty()));
        let required = if kind == ElementKind::Table {
            ElementKind::Row
        } else {
            ElementKind::Cell
        };
        if children
            .iter()
            .any(|child| !matches!(child, Content::Element(element) if element.kind == required))
        {
            return Err(Diagnostic::new(
                span,
                format!(
                    "{} bodies contain only {} elements and whitespace",
                    call.name,
                    required.as_text()
                ),
            ));
        }
        if children.is_empty() {
            document.warnings.push(Diagnostic::new(
                span,
                format!(
                    "{} body is empty; emitting an empty {}",
                    call.name, call.name
                ),
            ));
        }
    }
    Ok(Content::Element(Element {
        kind,
        attributes,
        children,
    }))
}

pub(super) fn normalize(
    kind: ElementKind,
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    match (kind, name) {
        (ElementKind::Cell, "padding") => Err(Diagnostic::new(
            span,
            "padding is not supported for cells; use margin",
        )),
        (ElementKind::Cell, "width") | (ElementKind::Row, "height") => {
            normalize_length(value, span, name)
        }
        (ElementKind::Cell, "colspan" | "rowspan") => normalize_integer(value, span, name, 1),
        (ElementKind::Row, "repeat_header" | "cant_split") => normalize_boolean(value, span, name),
        (ElementKind::Row, "height_rule") => {
            normalize_enum(value, span, name, &["auto", "at_least", "exact"])
        }
        (ElementKind::Row, _) => Ok(value),
        _ => {
            let properties = Properties::from([(name.into(), value)]);
            let Value::Object(mut fields) = normalize_table_group(
                Value::Object(properties),
                span,
                if kind == ElementKind::Table {
                    TableGroup::Table
                } else {
                    TableGroup::Cell
                },
            )?
            else {
                unreachable!()
            };
            Ok(fields.remove(name).unwrap())
        }
    }
}

pub(super) fn evaluate_markdown(
    rows: &[crate::syntax::MarkdownTableRow],
    context: EvaluationContext<'_>,
    document: &mut Document,
) -> Result<Content, Diagnostic> {
    let mut children = Vec::new();
    for row in rows {
        let mut cells = Vec::new();
        for cell in &row.cells {
            let mut content = evaluate_nodes(&cell.body, ContentMode::Inline, context, document)?;
            if row.header && !content.is_empty() {
                content = vec![Content::Element(Element {
                    kind: ElementKind::Strong,
                    attributes: Properties::new(),
                    children: content,
                })];
            }
            let attributes = if cell.alignment.is_empty() {
                Properties::new()
            } else {
                Properties::from([("alignment".into(), Value::String(cell.alignment.clone()))])
            };
            cells.push(Content::Element(Element {
                kind: ElementKind::Cell,
                attributes,
                children: content,
            }));
        }
        let attributes = if row.header {
            Properties::from([("repeat_header".into(), Value::Boolean(true))])
        } else {
            Properties::new()
        };
        children.push(Content::Element(Element {
            kind: ElementKind::Row,
            attributes,
            children: cells,
        }));
    }
    Ok(Content::Element(Element {
        kind: ElementKind::Table,
        attributes: Properties::new(),
        children,
    }))
}
