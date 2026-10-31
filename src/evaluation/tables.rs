use super::{
    ContentMode, EvaluationContext, EvaluationState,
    declarations::{TableGroup, normalize_table_group},
    evaluate_nodes,
    values::*,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Element, ElementKind, Properties, Value},
};

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
    document: &mut EvaluationState<'_>,
) -> Result<Content, Diagnostic> {
    let mut children = Vec::new();
    for row in rows {
        let mut cells = Vec::new();
        for cell in &row.cells {
            let mut content = evaluate_nodes(&cell.body, ContentMode::Inline, context, document)?;
            if row.header && !content.is_empty() {
                content = vec![Content::Element(Element {
                    source_name: document.source_name().map(ToOwned::to_owned),
                    kind: ElementKind::Strong,
                    attributes: document.bindings.element_defaults(ElementKind::Strong),
                    children: content,
                })];
            }
            let attributes = if cell.alignment.is_empty() {
                Properties::new()
            } else {
                Properties::from([("alignment".into(), Value::String(cell.alignment.clone()))])
            };
            cells.push(Content::Element(Element {
                source_name: document.source_name().map(ToOwned::to_owned),
                kind: ElementKind::Cell,
                attributes: document
                    .bindings
                    .element_attributes(ElementKind::Cell, &attributes),
                children: content,
            }));
        }
        let attributes = if row.header {
            Properties::from([("repeat_header".into(), Value::Boolean(true))])
        } else {
            Properties::new()
        };
        children.push(Content::Element(Element {
            source_name: document.source_name().map(ToOwned::to_owned),
            kind: ElementKind::Row,
            attributes: document
                .bindings
                .element_attributes(ElementKind::Row, &attributes),
            children: cells,
        }));
    }
    Ok(Content::Element(Element {
        source_name: document.source_name().map(ToOwned::to_owned),
        kind: ElementKind::Table,
        attributes: document.bindings.element_defaults(ElementKind::Table),
        children,
    }))
}
