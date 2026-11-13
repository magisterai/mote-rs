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
    Date(String),
    Length { value: f64, unit: Unit },
    Content(Vec<Content>),
    Element(Box<Element>),
}

impl Value {
    /// Textual representation used when a property explicitly requests coercion.
    pub fn to_text(&self) -> String {
        match self {
            Self::String(text) | Self::Color(text) | Self::Date(text) => text.clone(),
            Self::Number(number) => number.to_string(),
            Self::Boolean(boolean) => boolean.to_string(),
            Self::Nil => String::new(),
            Self::Content(content) => content.iter().map(Content::to_text).collect(),
            Self::Element(element) => element.children.iter().map(Content::to_text).collect(),
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
    /// Authored descriptive values, kept separate from layout configuration.
    pub metadata: Properties,
    /// Normalized geometry, merged by field across document declarations.
    pub page: Properties,
    pub margin: Properties,
    pub columns: ColumnConfiguration,
    /// Retained default_* and language arguments; output support is separate.
    pub defaults: Properties,
    pub content: Vec<Content>,
    /// Resolved imported sources in first-encounter order.
    pub imports: Vec<std::ffi::OsString>,
    pub presentation: Presentation,
    /// Resolved section snapshots; empty means one implicit document section.
    pub sections: Vec<Section>,
    pub fonts: BTreeMap<String, FontDeclaration>,
    pub styles: BTreeMap<String, StyleDeclaration>,
    /// Non-fatal evaluation diagnostics, kept separate from JSON inspection.
    pub warnings: Vec<crate::diagnostics::Diagnostic>,
}

/// A link's screen tip is its `tooltip`, or else its `title`, as a Markdown link
/// title is in HTML.
pub fn link_tooltip(attributes: &Properties) -> Option<String> {
    ["tooltip", "title"]
        .into_iter()
        .filter_map(|name| attributes.get(name))
        .map(Value::to_text)
        .find(|text| !text.trim().is_empty())
}

/// Style references ignore case and every character other than an ASCII letter or digit.
pub fn normalize_style_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

impl Document {
    /// An exact name takes precedence over a normalized match.
    pub fn find_style(&self, name: &str) -> Option<(&str, &StyleDeclaration)> {
        if let Some((identity, declaration)) = self.styles.get_key_value(name) {
            return Some((identity, declaration));
        }
        let normalized = normalize_style_name(name);
        if normalized.is_empty() {
            return None;
        }
        self.styles
            .iter()
            .find(|(identity, _)| normalize_style_name(identity) == normalized)
            .map(|(identity, declaration)| (identity.as_str(), declaration))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Presentation {
    /// Source of the current deck background, independent of later size updates.
    pub background_source_name: Option<std::ffi::OsString>,
    pub properties: Properties,
    pub size_properties: Properties,
    /// Authored declarations in stable order; replacement retains the original index.
    pub layouts: Vec<PresentationLayout>,
    /// Resolved authored dimensions, kept separate from the last size declaration.
    pub dimensions: Properties,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PresentationLayout {
    pub source_name: Option<std::ffi::OsString>,
    pub name: String,
    pub based_on: String,
    pub properties: Properties,
    pub slots: Vec<PresentationLayoutSlot>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PresentationLayoutSlot {
    pub name: String,
    /// Empty inherits the parent's kind, or becomes content for a new slot.
    pub kind: String,
    /// None inherits; Some(false) explicitly makes an inherited slot optional.
    pub required: Option<bool>,
    pub remove: bool,
    pub properties: Properties,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Section {
    pub explicit: bool,
    pub properties: Properties,
    pub section_properties: Properties,
    /// May be synthesized when returning from a continuous explicit section.
    pub start: String,
    pub page_numbering: Properties,
    pub page: Properties,
    pub margin: Properties,
    pub columns: ColumnConfiguration,
    pub header: HeaderFooter,
    pub footer: HeaderFooter,
    pub content: Vec<Content>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeaderFooter {
    /// Identity of an authored story declaration, retained through inheritance.
    pub declaration_id: usize,
    pub properties: Properties,
    pub content: Vec<Content>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColumnConfiguration {
    /// Parsed count, separate from its authored numeric or string property.
    pub count: Option<i64>,
    pub properties: Properties,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontDeclaration {
    pub source_name: Option<std::ffi::OsString>,
    pub name: String,
    pub properties: Properties,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleDeclaration {
    pub name: String,
    pub kind: StyleKind,
    pub properties: Properties,
    pub span: crate::diagnostics::Span,
    pub source_location: Option<crate::diagnostics::SourceLocation>,
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

#[derive(Debug, Clone)]
pub struct Element {
    /// Logical source owning resource references; omitted from authored JSON.
    pub source_name: Option<std::ffi::OsString>,
    /// Diagnostic provenance; excluded from authored JSON and content equality.
    pub source_location: Option<crate::diagnostics::SourceLocation>,
    pub kind: ElementKind,
    pub attributes: Properties,
    pub children: Vec<Content>,
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.source_name == other.source_name
            && self.kind == other.kind
            && self.attributes == other.attributes
            && self.children == other.children
    }
}

impl Content {
    pub fn to_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Element(element) => element.children.iter().map(Self::to_text).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Paragraph,
    Heading,
    Span,
    Link,
    Image,
    Shape,
    Chart,
    Slide,
    Box,
    Line,
    Group,
    Table,
    Row,
    Cell,
    Strong,
    Emphasis,
    Underline,
    Strike,
    Highlight,
    SmallCaps,
    Subscript,
    Superscript,
    Code,
    Raw,
    DefinitionList,
    Term,
    Definition,
    Footnote,
    Endnote,
    PageNumber,
    Math,
    Checkbox,
    Field,
    Comment,
    Bookmark,
    Label,
    Reference,
    Figure,
    Caption,
    LineBreak,
    Tab,
    Break,
    Divider,
    List,
    OrderedList,
    ListItem,
    Blockquote,
}

impl ElementKind {
    pub(crate) fn from_name(name: &str) -> Option<Self> {
        [
            Self::Paragraph,
            Self::Heading,
            Self::Span,
            Self::Link,
            Self::Image,
            Self::Shape,
            Self::Chart,
            Self::Slide,
            Self::Box,
            Self::Line,
            Self::Group,
            Self::Table,
            Self::Row,
            Self::Cell,
            Self::Strong,
            Self::Emphasis,
            Self::Underline,
            Self::Strike,
            Self::Highlight,
            Self::SmallCaps,
            Self::Subscript,
            Self::Superscript,
            Self::Code,
            Self::Raw,
            Self::DefinitionList,
            Self::Term,
            Self::Definition,
            Self::Footnote,
            Self::Endnote,
            Self::PageNumber,
            Self::Math,
            Self::Checkbox,
            Self::Field,
            Self::Comment,
            Self::Bookmark,
            Self::Label,
            Self::Reference,
            Self::Figure,
            Self::Caption,
            Self::LineBreak,
            Self::Tab,
            Self::Break,
            Self::Divider,
            Self::List,
            Self::OrderedList,
            Self::ListItem,
            Self::Blockquote,
        ]
        .into_iter()
        .find(|kind| kind.as_text() == name)
    }

    pub fn as_text(self) -> &'static str {
        match self {
            Self::Paragraph => "paragraph",
            Self::Heading => "heading",
            Self::Span => "span",
            Self::Link => "link",
            Self::Image => "image",
            Self::Shape => "shape",
            Self::Chart => "chart",
            Self::Slide => "slide",
            Self::Box => "box",
            Self::Line => "line",
            Self::Group => "group",
            Self::Table => "table",
            Self::Row => "row",
            Self::Cell => "cell",
            Self::Strong => "strong",
            Self::Emphasis => "emphasis",
            Self::Underline => "underline",
            Self::Strike => "strike",
            Self::Highlight => "highlight",
            Self::SmallCaps => "small_caps",
            Self::Subscript => "subscript",
            Self::Superscript => "superscript",
            Self::Code => "code",
            Self::Raw => "raw",
            Self::DefinitionList => "definition_list",
            Self::Term => "term",
            Self::Definition => "definition",
            Self::Bookmark => "bookmark",
            Self::Label => "label",
            Self::Reference => "ref",
            Self::Figure => "figure",
            Self::Caption => "caption",
            Self::Footnote => "footnote",
            Self::Endnote => "endnote",
            Self::PageNumber => "page_number",
            Self::Math => "math",
            Self::Checkbox => "checkbox",
            Self::Field => "field",
            Self::Comment => "comment",
            Self::LineBreak => "line_break",
            Self::Tab => "tab",
            Self::Break => "break",
            Self::Divider => "divider",
            Self::List => "list",
            Self::OrderedList => "ordered_list",
            Self::ListItem => "list_item",
            Self::Blockquote => "blockquote",
        }
    }

    pub fn is_table_part(self) -> bool {
        matches!(self, Self::Row | Self::Cell)
    }

    pub fn is_block(self) -> bool {
        !matches!(
            self,
            Self::Row
                | Self::Cell
                | Self::Link
                | Self::Span
                | Self::Strong
                | Self::Emphasis
                | Self::Underline
                | Self::Strike
                | Self::Highlight
                | Self::SmallCaps
                | Self::Subscript
                | Self::Superscript
                | Self::Code
                | Self::Bookmark
                | Self::Label
                | Self::Reference
                | Self::Footnote
                | Self::Endnote
                | Self::PageNumber
                | Self::Math
                | Self::Checkbox
                | Self::Field
                | Self::Comment
                | Self::LineBreak
                | Self::Tab
        )
    }
}

/// Wrap consecutive inline nodes in paragraphs while retaining existing blocks.
pub(crate) fn wrap_block_content(content: &[Content]) -> Vec<Content> {
    let mut result = Vec::new();
    let mut inline = Vec::new();
    for node in content {
        if matches!(node, Content::Element(element) if element.is_block()) {
            if !inline.is_empty() {
                result.push(Content::Element(Element {
                    source_name: None,
                    source_location: None,
                    kind: ElementKind::Paragraph,
                    attributes: Properties::new(),
                    children: std::mem::take(&mut inline),
                }));
            }
            result.push(node.clone());
        } else {
            inline.push(node.clone());
        }
    }
    if !inline.is_empty() {
        result.push(Content::Element(Element {
            source_name: None,
            source_location: None,
            kind: ElementKind::Paragraph,
            attributes: Properties::new(),
            children: inline,
        }));
    }
    result
}

impl Element {
    /// Some content kinds select their flow category through normalized attributes.
    pub fn is_block(&self) -> bool {
        if self.kind == ElementKind::Math {
            crate::properties::text(&self.attributes, "display") == "block"
        } else {
            self.kind.is_block()
        }
    }
}
