//! Evaluation of content directives and their formatting attributes.

use crate::diagnostics::{Diagnostic, Span};
use crate::model::{Content, Document, Element, ElementKind, Properties, Value};
use crate::syntax::{Call, Node, NodeKind, SyntaxTree};

mod attributes;
mod bindings;
mod configuration;
mod constructors;
mod control;
mod data;
mod declarations;
mod drawings;
mod expressions;
mod functions;
mod images;
mod notes;
mod references;
mod scripting;
mod sections;
mod tables;
use sections::EvaluationState;
mod values;

use expressions::{evaluate_helper, value_as_text};

#[derive(Clone, Copy)]
struct EvaluationContext<'source> {
    source: &'source str,
    preserve_space: bool,
}

/// Inputs and opt-in scripting settings for one compilation.
#[derive(Clone, Copy)]
pub struct EvaluationOptions<'resolver> {
    pub asset_resolver: Option<&'resolver dyn crate::source::AssetResolver>,
    pub source_name: Option<&'resolver std::ffi::OsStr>,
    pub enable_lua: bool,
    /// Shared wall-clock budget starting at first Lua use. None disables it.
    pub lua_timeout: Option<std::time::Duration>,
}
impl Default for EvaluationOptions<'_> {
    fn default() -> Self {
        Self {
            asset_resolver: None,
            source_name: None,
            enable_lua: false,
            lua_timeout: Some(std::time::Duration::from_secs(10)),
        }
    }
}

pub fn evaluate(tree: &SyntaxTree) -> Result<Document, Diagnostic> {
    evaluate_with_options(tree, EvaluationOptions::default())
}

/// Evaluate with source-relative resources, snapshotted for this evaluation.
pub fn evaluate_with_assets(
    tree: &SyntaxTree,
    resolver: &dyn crate::source::AssetResolver,
    source_name: Option<&std::ffi::OsStr>,
) -> Result<Document, Diagnostic> {
    evaluate_with_options(
        tree,
        EvaluationOptions {
            asset_resolver: Some(resolver),
            source_name,
            ..Default::default()
        },
    )
}

pub fn evaluate_with_options(
    tree: &SyntaxTree,
    options: EvaluationOptions<'_>,
) -> Result<Document, Diagnostic> {
    let assets = options
        .asset_resolver
        .map(crate::source::CachingAssetResolver::new);
    let mut state = EvaluationState::default();
    state.assets = assets
        .as_ref()
        .map(|resolver| resolver as &dyn crate::source::AssetResolver);
    state.source_name = options.source_name;
    state.enable_lua = options.enable_lua;
    state.lua_timeout = options.lua_timeout;
    evaluate_with_state(tree, state)
}

fn evaluate_with_state(
    tree: &SyntaxTree,
    mut document: EvaluationState<'_>,
) -> Result<Document, Diagnostic> {
    notes::collect_definitions(&tree.nodes, &mut document);
    document.content = evaluate_nodes(
        &tree.nodes,
        ContentMode::Document,
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
    references::validate(&document)?;
    Ok(document.finish())
}

#[derive(Clone, Copy, PartialEq)]
enum ContentMode {
    Document,
    Block,
    Inline,
    MarkdownItem,
    ListBody,
    Structural,
    Flow,
    Any,
}

impl ContentMode {
    fn is_block(self) -> bool {
        matches!(self, Self::Document | Self::Block)
    }
}

fn evaluate_nodes(
    nodes: &[Node],
    mode: ContentMode,
    context: EvaluationContext<'_>,
    document: &mut EvaluationState<'_>,
) -> Result<Vec<Content>, Diagnostic> {
    let mut result = Vec::new();
    let mut paragraph = Vec::new();
    let mut skip_declaration_trivia = false;
    let mut captured = 0;
    for node in nodes {
        if let NodeKind::FunctionDefinition {
            name,
            parameters,
            body,
        } = &node.kind
        {
            functions::define(name, parameters, body, node.span, context, document)?;
            skip_declaration_trivia = true;
            continue;
        }
        if let NodeKind::Call(call) = &node.kind
            && let Some(function) = document.bindings.function(&call.name)
        {
            let value = functions::evaluate(&function, call, node.span, context, document)?;
            skip_declaration_trivia = value == Value::Nil;
            for child in scripting::value_content(value) {
                append_evaluated_content(child, mode, node.span, &mut paragraph, &mut result)?;
            }
            continue;
        }
        if let NodeKind::Call(call) = &node.kind
            && matches!(call.name.as_str(), "header" | "footer" | "section")
        {
            if mode.is_block() {
                flush_paragraph(&mut paragraph, &mut result);
            }
            if mode == ContentMode::Document {
                document.append_content(&result[captured..]);
            }
            if call.name == "section" {
                let content = sections::evaluate_section(call, node.span, context, document)?;
                result.extend(content);
            } else {
                sections::evaluate_story(call, node.span, context, document)?;
            }
            captured = result.len();
            skip_declaration_trivia = true;
            continue;
        }
        if skip_declaration_trivia
            && (mode.is_block() || mode == ContentMode::Flow)
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
        if matches!(node.kind, NodeKind::FootnoteDefinition { .. }) {
            skip_declaration_trivia = true;
            continue;
        }
        if let NodeKind::Call(call) = &node.kind
            && call.name == "let"
        {
            bindings::evaluate(call, node.span, document)?;
            skip_declaration_trivia = true;
            continue;
        }
        if matches!(
            node.kind,
            NodeKind::Conditional { .. } | NodeKind::Iteration { .. }
        ) {
            if let Some(children) = control::evaluate(node, mode, context, document)? {
                for child in children {
                    append_evaluated_content(child, mode, node.span, &mut paragraph, &mut result)?;
                }
            } else {
                skip_declaration_trivia = true;
            }
            continue;
        }
        if mode.is_block() && matches!(node.kind, NodeKind::ParagraphBreak) {
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
            let preceding = if mode.is_block() {
                &mut paragraph
            } else {
                &mut result
            };
            if let Some(Content::Text(text)) = preceding.last_mut() {
                text.truncate(text.trim_end_matches(' ').len());
            }
        }
        let content = match &node.kind {
            NodeKind::FootnoteDefinition { .. }
            | NodeKind::FunctionDefinition { .. }
            | NodeKind::Conditional { .. }
            | NodeKind::Iteration { .. } => unreachable!(),
            NodeKind::FootnoteReference(name) => {
                notes::markdown_reference(name, node.span, context, document)?
            }
            NodeKind::Table(rows) => tables::evaluate_markdown(rows, context, document)?,
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
            NodeKind::Image {
                source,
                title,
                label,
            } => {
                let children = evaluate_nodes(label, ContentMode::Inline, context, document)?;
                fn content_text(content: &[Content]) -> String {
                    content
                        .iter()
                        .map(|node| match node {
                            Content::Text(text) => text.clone(),
                            Content::Element(element) => content_text(&element.children),
                        })
                        .collect()
                }
                let mut attributes =
                    Properties::from([("src".into(), Value::String(source.clone()))]);
                let alternative = content_text(&children);
                if !alternative.is_empty() {
                    attributes.insert("alt".into(), Value::String(alternative));
                }
                if !title.is_empty() {
                    attributes.insert("title".into(), Value::String(title.clone()));
                }
                Content::Element(Element {
                    kind: ElementKind::Image,
                    attributes,
                    children: Vec::new(),
                })
            }
            NodeKind::Call(call) if call.name == "json" => {
                if !call.bodies.is_empty() {
                    return Err(Diagnostic::new(node.span, "json does not accept a body"));
                }
                let value = evaluate_helper(&call.name, &call.arguments, node.span, document)?;
                if value == Value::Nil {
                    skip_declaration_trivia = true;
                    continue;
                }
                Content::Text(value.to_text())
            }
            NodeKind::Call(call) => evaluate_call(call, node.span, context, document)?,
            NodeKind::Reference(name) => {
                let value = if document.enable_lua {
                    scripting::evaluate(name, node.span, document)?
                } else {
                    document.bindings.resolve(name, node.span)?
                };
                if value == Value::Nil {
                    skip_declaration_trivia = true;
                    continue;
                }
                for child in scripting::value_content(value) {
                    append_evaluated_content(child, mode, node.span, &mut paragraph, &mut result)?;
                }
                continue;
            }
        };
        append_evaluated_content(content, mode, node.span, &mut paragraph, &mut result)?;
    }
    if mode.is_block() {
        flush_paragraph(&mut paragraph, &mut result);
    }
    if mode == ContentMode::Document {
        document.append_content(&result[captured..]);
    }
    if mode.is_block() {
        Ok(group_adjacent_list_items(result))
    } else {
        Ok(result)
    }
}

fn append_evaluated_content(
    mut content: Content,
    mode: ContentMode,
    span: Span,
    paragraph: &mut Vec<Content>,
    result: &mut Vec<Content>,
) -> Result<(), Diagnostic> {
    if matches!(&content, Content::Element(element) if element.kind.is_block() || element.kind.is_table_part())
    {
        if mode == ContentMode::Inline {
            return Err(Diagnostic::new(
                span,
                "inline content cannot contain a block element",
            ));
        }
        flush_paragraph(paragraph, result);
        result.push(content);
    } else if mode.is_block() {
        if let Content::Text(text) = &mut content {
            if text.trim().is_empty() {
                if text.contains("\n\n") {
                    flush_paragraph(paragraph, result);
                } else if !paragraph.is_empty() {
                    paragraph.push(content);
                }
                return Ok(());
            }
            if paragraph.is_empty() {
                *text = text.trim_start_matches([' ', '\t', '\r', '\n']).to_owned();
            } else if matches!(paragraph.last(), Some(Content::Text(previous)) if previous.ends_with('\n'))
            {
                *text = text.trim_start_matches([' ', '\t']).to_owned();
            }
        }
        paragraph.push(content);
    } else {
        result.push(content);
    }
    Ok(())
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
    }
    if matches!(buffer.last(), Some(Content::Text(text)) if text.trim().is_empty()) {
        while matches!(buffer.last(), Some(Content::Text(text)) if text.trim().is_empty()) {
            buffer.pop();
        }
    } else if let Some(Content::Text(text)) = buffer.last_mut() {
        text.truncate(text.trim_end_matches([' ', '\t', '\r', '\n']).len());
    }
    buffer.retain(|node| !matches!(node, Content::Text(text) if text.is_empty()));
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
    document: &mut EvaluationState<'_>,
) -> Result<Content, Diagnostic> {
    if let Some(signature) = constructors::Signature::find(&call.name) {
        return constructors::evaluate(signature, call, span, context, document);
    }
    if !call.bodies.is_empty() {
        return Err(Diagnostic::new(
            span,
            format!("unsupported directive: {}", call.name),
        ));
    }
    Ok(Content::Text(value_as_text(
        &evaluate_helper(&call.name, &call.arguments, span, document)?,
        span,
    )?))
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

fn evaluate_body(
    nodes: &[Node],
    mode: ContentMode,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<Vec<Content>, Diagnostic> {
    state.bindings.enter();
    let result = evaluate_nodes(nodes, mode, context, state);
    state.bindings.leave();
    result
}
