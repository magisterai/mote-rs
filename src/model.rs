//! Evaluated content, independent of output format and layout.

use std::collections::BTreeMap;

pub type Properties = BTreeMap<String, Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Object(Properties),
    Array(Vec<Value>),
    Color(String),
    Length { value: f64, unit: Unit },
}

impl Value {
    /// Textual representation used when a property explicitly requests coercion.
    pub fn to_text(&self) -> String {
        match self {
            Self::String(text) | Self::Color(text) => text.clone(),
            Self::Number(number) => number.to_string(),
            Self::Boolean(boolean) => boolean.to_string(),
            Self::Nil => String::new(),
            Self::Length { value, unit } => format!("{value}{}", unit.as_text()),
            Self::Array(values) => values
                .iter()
                .map(Self::to_text)
                .collect::<Vec<_>>()
                .join(", "),
            Self::Object(fields) => format!(
                "{{{}}}",
                fields
                    .iter()
                    .map(|(name, value)| format!("{name}: {}", value.to_text()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Point,
    Inch,
    Centimetre,
    Millimetre,
    Twip,
}

impl Unit {
    pub fn as_text(self) -> &'static str {
        match self {
            Self::Point => "pt",
            Self::Inch => "in",
            Self::Centimetre => "cm",
            Self::Millimetre => "mm",
            Self::Twip => "twip",
        }
    }

    pub(crate) fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "pt" => Self::Point,
            "in" | "inch" => Self::Inch,
            "cm" => Self::Centimetre,
            "mm" => Self::Millimetre,
            "twip" => Self::Twip,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    /// Authored document arguments, with later arguments replacing earlier ones.
    pub properties: Properties,
    /// Normalized geometry, merged by field across document declarations.
    pub page: Properties,
    pub margin: Properties,
    pub columns: ColumnConfiguration,
    /// Retained default_* and language arguments; output support is separate.
    pub defaults: Properties,
    pub content: Vec<Content>,
    pub fonts: BTreeMap<String, FontDeclaration>,
    pub styles: BTreeMap<String, StyleDeclaration>,
    /// Non-fatal evaluation diagnostics, kept separate from JSON inspection.
    pub warnings: Vec<crate::diagnostics::Diagnostic>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColumnConfiguration {
    /// Parsed count, separate from its authored numeric or string property.
    pub count: Option<i64>,
    pub properties: Properties,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontDeclaration {
    pub name: String,
    pub properties: Properties,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleDeclaration {
    pub name: String,
    pub kind: StyleKind,
    pub properties: Properties,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleKind {
    Paragraph,
    Character,
    Table,
}

impl StyleKind {
    pub fn as_text(self) -> &'static str {
        match self {
            Self::Paragraph => "paragraph",
            Self::Character => "character",
            Self::Table => "table",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Text(String),
    Element(Element),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub kind: ElementKind,
    pub attributes: Properties,
    pub children: Vec<Content>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Paragraph,
    Heading,
    Span,
    Link,
    Strong,
    Emphasis,
    Code,
    LineBreak,
    Tab,
    Break,
    List,
    OrderedList,
    ListItem,
    Blockquote,
}

impl ElementKind {
    pub fn as_text(self) -> &'static str {
        match self {
            Self::Paragraph => "paragraph",
            Self::Heading => "heading",
            Self::Span => "span",
            Self::Link => "link",
            Self::Strong => "strong",
            Self::Emphasis => "emphasis",
            Self::Code => "code",
            Self::LineBreak => "line_break",
            Self::Tab => "tab",
            Self::Break => "break",
            Self::List => "list",
            Self::OrderedList => "ordered_list",
            Self::ListItem => "list_item",
            Self::Blockquote => "blockquote",
        }
    }

    pub fn is_block(self) -> bool {
        !matches!(
            self,
            Self::Link
                | Self::Span
                | Self::Strong
                | Self::Emphasis
                | Self::Code
                | Self::LineBreak
                | Self::Tab
        )
    }
}
