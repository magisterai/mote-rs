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
    Length { value: f64, unit: Unit },
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

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub content: Vec<Content>,
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
    Strong,
    Emphasis,
    Code,
    LineBreak,
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
            Self::Strong => "strong",
            Self::Emphasis => "emphasis",
            Self::Code => "code",
            Self::LineBreak => "line_break",
            Self::List => "list",
            Self::OrderedList => "ordered_list",
            Self::ListItem => "list_item",
            Self::Blockquote => "blockquote",
        }
    }

    pub fn is_block(self) -> bool {
        !matches!(
            self,
            Self::Strong | Self::Emphasis | Self::Code | Self::LineBreak
        )
    }
}
