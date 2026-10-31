//! Section capture and story declarations, independent of output formatting.
use super::{
    ContentMode, EvaluationContext, evaluate_body, expressions::evaluate_named_arguments, values::*,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Document, HeaderFooter, Properties, Section, Value},
    syntax::Call,
};
use std::ops::{Deref, DerefMut};

#[derive(Default)]
struct StoryCapture {
    header: Option<HeaderFooter>,
    footer: Option<HeaderFooter>,
}
#[derive(Default)]
pub(super) struct EvaluationState<'resolver> {
    pub assets: Option<&'resolver dyn crate::source::AssetResolver>,
    pub source: std::sync::Arc<crate::source::SourceFile>,
    pub imported: std::collections::BTreeSet<std::ffi::OsString>,
    pub importing: Vec<std::ffi::OsString>,
    document: Document,
    pub bindings: super::bindings::Bindings,
    pub enable_lua: bool,
    pub lua_timeout: Option<std::time::Duration>,
    pub lua: std::cell::OnceCell<std::rc::Rc<super::scripting::Runtime>>,
    pub markdown_notes: std::collections::BTreeMap<String, super::notes::Definition>,
    pub labels: Vec<super::references::NamedLocation>,
    pub references: Vec<super::references::NamedLocation>,
    pub story_declaration: Option<usize>,
    default_header: HeaderFooter,
    default_footer: HeaderFooter,
    captures: Vec<StoryCapture>,
    new_implicit_section: bool,
    declared_sections: bool,
}
impl Deref for EvaluationState<'_> {
    type Target = Document;
    fn deref(&self) -> &Document {
        &self.document
    }
}
impl DerefMut for EvaluationState<'_> {
    fn deref_mut(&mut self) -> &mut Document {
        &mut self.document
    }
}
impl EvaluationState<'_> {
    pub fn warn(&mut self, diagnostic: Diagnostic) {
        let diagnostic = if self.source_name().is_some() {
            diagnostic.with_source(self.source.clone())
        } else {
            diagnostic
        };
        self.warnings.push(diagnostic);
    }
    pub fn source_name(&self) -> Option<&std::ffi::OsStr> {
        (!self.source.name.is_empty()).then_some(self.source.name.as_os_str())
    }
    pub fn append_content(&mut self, content: &[Content]) {
        if content.is_empty() {
            return;
        }
        if self.document.sections.is_empty() || self.new_implicit_section {
            let continuous = self
                .document
                .sections
                .last()
                .is_some_and(|section| section.explicit && section.start == "continuous");
            self.document.sections.push(Section {
                page: self.document.page.clone(),
                margin: self.document.margin.clone(),
                columns: self.document.columns.clone(),
                header: self.default_header.clone(),
                footer: self.default_footer.clone(),
                start: if continuous {
                    "continuous".into()
                } else {
                    String::new()
                },
                ..Default::default()
            });
            self.new_implicit_section = false;
        }
        self.document
            .sections
            .last_mut()
            .unwrap()
            .content
            .extend_from_slice(content);
    }
    pub(super) fn set_story(&mut self, header: bool, story: HeaderFooter) {
        self.declared_sections = true;
        if let Some(capture) = self.captures.last_mut() {
            *if header {
                &mut capture.header
            } else {
                &mut capture.footer
            } = Some(story);
        } else {
            *if header {
                &mut self.default_header
            } else {
                &mut self.default_footer
            } = story;
            if self
                .document
                .sections
                .last()
                .is_some_and(|section| !section.content.is_empty())
            {
                self.new_implicit_section = true;
            }
        }
    }
    pub fn finish(mut self) -> Document {
        if self.declared_sections {
            for section in &mut self.document.sections {
                section.content = super::group_adjacent_list_items(
                    std::mem::take(&mut section.content),
                    &self.bindings,
                );
            }
        } else {
            self.document.sections.clear();
        }
        self.document
    }
}
fn check_body(call: &Call, span: Span) -> Result<(), Diagnostic> {
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
    Ok(())
}
pub(super) fn evaluate_story(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    check_body(call, span)?;
    let arguments = evaluate_named_arguments(&call.arguments, state)?;
    let properties = story_properties(&call.name, arguments, span)?;
    let previous_story = state.story_declaration.replace(span.start + 1);
    let content = evaluate_body(&call.bodies[0], ContentMode::Block, context, state);
    state.story_declaration = previous_story;
    let content = content?;
    state.set_story(
        call.name == "header",
        HeaderFooter {
            declaration_id: span.start + 1,
            properties,
            content,
        },
    );
    Ok(())
}
pub(super) fn story_properties(
    directive: &str,
    arguments: std::collections::BTreeMap<String, super::expressions::EvaluatedArgument>,
    span: Span,
) -> Result<Properties, Diagnostic> {
    for (name, argument) in &arguments {
        if !matches!(name.as_str(), "properties" | "height" | "distance") {
            return Err(Diagnostic::new(
                argument.span,
                format!("unknown {directive} argument {name:?}"),
            ));
        }
    }
    let mut properties = arguments
        .get("properties")
        .map(|argument| require_object(argument.value.clone(), argument.span, "properties"))
        .transpose()?
        .unwrap_or_default();
    for name in ["height", "distance"] {
        if let Some(argument) = arguments.get(name) {
            properties.insert(name.into(), argument.value.clone());
        }
        if let Some(value) = properties.get(name).cloned() {
            properties.insert(name.into(), normalize_length(value, span, name)?);
        }
    }
    Ok(properties)
}
pub(super) fn evaluate_section(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<Vec<Content>, Diagnostic> {
    check_body(call, span)?;
    state.declared_sections = true;
    let arguments = evaluate_named_arguments(&call.arguments, state)?;
    let properties: Properties = arguments
        .iter()
        .map(|(name, argument)| (name.clone(), argument.value.clone()))
        .collect();
    for name in properties.keys() {
        super::configuration::reject_flat_property(name, span)?;
        if matches!(
            name.as_str(),
            "break"
                | "vertical_alignment"
                | "restart_page_numbering"
                | "page_number_start"
                | "page_number_format"
        ) || name.starts_with("section_")
            || name.starts_with("page_number_")
            || name.starts_with("page_numbering_")
        {
            return Err(Diagnostic::new(
                span,
                format!("{name} must be supplied in properties or page_numbering"),
            ));
        }
    }
    let mut section = Section {
        explicit: true,
        properties: properties.clone(),
        page: state.page.clone(),
        margin: state.margin.clone(),
        columns: state.columns.clone(),
        ..Default::default()
    };
    if let Some(value) = properties.get("page") {
        section
            .page
            .extend(super::configuration::normalize_geometry(
                "page",
                value.clone(),
                span,
            )?);
    }
    if let Some(value) = properties.get("margin") {
        section
            .margin
            .extend(super::configuration::normalize_geometry(
                "margin",
                value.clone(),
                span,
            )?);
    }
    if let Some(value) = properties.get("columns") {
        super::configuration::apply_columns(&mut section.columns, value.clone(), span)?;
    }
    if let Some(value) = properties.get("properties") {
        section.section_properties = require_object(value.clone(), span, "properties")?;
        for (name, value) in &mut section.section_properties {
            *value = match name.as_str() {
                "break" => {
                    return Err(Diagnostic::new(
                        span,
                        "use properties.start instead of properties.break",
                    ));
                }
                "start" => normalize_enum(
                    value.clone(),
                    span,
                    "section start",
                    &[
                        "next_page",
                        "continuous",
                        "even_page",
                        "odd_page",
                        "next_column",
                    ],
                )?,
                "vertical_alignment" => {
                    let value = if value.to_text() == "both" {
                        Value::String("justified".into())
                    } else {
                        value.clone()
                    };
                    normalize_enum(
                        value,
                        span,
                        "vertical alignment",
                        &["top", "center", "bottom", "justified"],
                    )?
                }
                _ => value.clone(),
            };
        }
        section.start = section
            .section_properties
            .get("start")
            .map(Value::to_text)
            .unwrap_or_default();
    }
    if let Some(value) = properties.get("page_numbering") {
        section.page_numbering = require_object(value.clone(), span, "page_numbering")?;
        for (name, value) in &mut section.page_numbering {
            *value = match name.as_str() {
                "restart" => normalize_boolean(value.clone(), span, "page_numbering.restart")?,
                "start" => normalize_integer(value.clone(), span, "page_numbering.start", 1)?,
                "format" => Value::String(value.to_text()),
                _ => value.clone(),
            };
        }
    }
    state.captures.push(StoryCapture::default());
    let content = evaluate_body(&call.bodies[0], ContentMode::Block, context, state)?;
    let capture = state.captures.pop().unwrap();
    section.header = capture
        .header
        .unwrap_or_else(|| state.default_header.clone());
    section.footer = capture
        .footer
        .unwrap_or_else(|| state.default_footer.clone());
    section.content = content.clone();
    state.document.sections.push(section);
    state.new_implicit_section = true;
    Ok(content)
}
