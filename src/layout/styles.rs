use super::{LineSpacing, TextStyle, style_values as values};
use crate::{
    formatting::StyleResolver,
    measurement::FontVariant,
    model::{Document, Element, ElementKind, Properties, StyleDeclaration, StyleKind, Value},
};

/// Effective native paragraph properties. Pagination requests remain unplaced.
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphStyle {
    pub text: TextStyle,
    pub alignment: String,
    pub spacing_before: f64,
    pub spacing_after: f64,
    pub keep_with_next: bool,
    pub keep_together: bool,
    pub page_break_before: bool,
    pub widow_control: bool,
    pub line_height: f64,
    pub line_spacing: LineSpacing,
    pub line_multiplier: f64,
    pub indent_left: f64,
    pub indent_right: f64,
    pub first_line: f64,
    pub fill: Option<String>,
}
impl Default for ParagraphStyle {
    fn default() -> Self {
        Self {
            text: TextStyle::default(),
            alignment: String::new(),
            spacing_before: 0.0,
            spacing_after: 8.0,
            keep_with_next: false,
            keep_together: false,
            page_break_before: false,
            widow_control: true,
            line_height: 0.0,
            line_spacing: LineSpacing::default(),
            line_multiplier: 1.15,
            indent_left: 0.0,
            indent_right: 0.0,
            first_line: 0.0,
            fill: None,
        }
    }
}

pub struct LayoutStyleResolver<'document> {
    pub(super) document: &'document Document,
    declarations: StyleResolver<'document>,
    pub(super) anchors: crate::references::Catalog,
    pub(super) reference_pages: std::collections::BTreeMap<String, usize>,
}
impl<'document> LayoutStyleResolver<'document> {
    pub fn new(document: &'document Document) -> Self {
        Self {
            document,
            declarations: StyleResolver::new(document),
            anchors: crate::references::Catalog::build(document),
            reference_pages: Default::default(),
        }
    }
    pub fn default_text(&self) -> TextStyle {
        let mut style = TextStyle::default();
        if let Some(size) = self
            .document
            .defaults
            .get("default_size")
            .or_else(|| self.document.defaults.get("size"))
            .and_then(values::points)
        {
            style.measurement.size_points = size;
        }
        let font = values::text(&self.document.defaults, "default_font");
        if !font.is_empty() {
            self.apply_font(&mut style, &font);
        }
        style
    }
    pub(super) fn border_color(&self, element: &Element) -> Option<String> {
        fn direct(properties: &Properties) -> Option<String> {
            let Value::Object(border) = properties.get("border")? else {
                return None;
            };
            for name in ["all", "bottom"] {
                if let Some(Value::Object(side)) = border.get(name)
                    && let Some(color) = values::color(side, "color")
                {
                    return Some(color);
                }
            }
            None
        }
        let mut name = values::text(&element.attributes, "style");
        if name.is_empty() {
            name = if element.kind == ElementKind::Heading {
                format!(
                    "Heading{}",
                    element
                        .attributes
                        .get("level")
                        .and_then(values::number)
                        .unwrap_or(1.0)
                        .clamp(1.0, 6.0) as usize
                )
            } else {
                "Normal".into()
            };
        }
        direct(&element.attributes).or_else(|| {
            if self.find_declaration(&name).is_some() {
                direct(&self.declarations.resolve_style(&name).paragraph)
            } else if values::normalize_name(&name) == "horizontalrule" {
                Some("D1D5DB".into())
            } else {
                None
            }
        })
    }
    pub fn paragraph(&self, element: &Element) -> ParagraphStyle {
        let mut style = ParagraphStyle {
            text: self.default_text(),
            ..Default::default()
        };
        let explicit = values::text(&element.attributes, "style");
        if element.kind == ElementKind::Heading && explicit.is_empty() {
            let level = element
                .attributes
                .get("level")
                .and_then(values::number)
                .unwrap_or(1.0)
                .clamp(1.0, 6.0) as usize;
            self.apply_named_paragraph(&mut style, &format!("Heading{level}"));
        } else if !explicit.is_empty() {
            self.apply_named_paragraph(&mut style, &explicit);
        } else if let Some(declaration) = self.find_declaration("Normal")
            && declaration.kind == StyleKind::Paragraph
        {
            self.apply_named_paragraph(&mut style, &declaration.name);
        }
        self.apply_run(&mut style.text, &element.attributes);
        self.apply_paragraph(&mut style, &element.attributes);
        if let Some(fill) = values::color(&element.attributes, "fill") {
            style.fill = Some(fill);
        }
        if style.line_height == 0.0 {
            style.line_height = style
                .line_spacing
                .height(&style.text, style.line_multiplier);
        }
        style
    }
    pub(super) fn drawing_paragraph(
        &self,
        inherited: &TextStyle,
        properties: &Properties,
        implicit: &str,
    ) -> ParagraphStyle {
        let mut style = ParagraphStyle {
            text: inherited.clone(),
            spacing_after: 0.0,
            line_spacing: LineSpacing::Multiple(1.0),
            line_multiplier: 1.0,
            ..Default::default()
        };
        let explicit = values::text(properties, "style");
        if !explicit.is_empty() {
            self.apply_named_paragraph(&mut style, &explicit);
        } else if self.find_declaration(implicit).is_some() {
            self.apply_named_paragraph(&mut style, implicit);
        }
        self.apply_run(&mut style.text, properties);
        self.apply_paragraph(&mut style, properties);
        if style.line_height == 0.0 {
            style.line_height = style
                .line_spacing
                .height(&style.text, style.line_multiplier);
        }
        style
    }
    pub fn inline(&self, element: &Element, inherited: &TextStyle) -> TextStyle {
        let mut style = inherited.clone();
        let (mut bold, mut italic) = emphasis(style.measurement.variant);
        match element.kind {
            ElementKind::Strong => bold = true,
            ElementKind::Emphasis => italic = true,
            ElementKind::Code => style.code = true,
            _ => {}
        }
        style.measurement.variant = FontVariant::from_emphasis(bold, italic);
        let name = values::text(&element.attributes, "style");
        if element.kind == ElementKind::Code && name.is_empty() {
            self.apply_named_run(&mut style, "Code");
        }
        if !name.is_empty() {
            if self.find_declaration(&name).is_some() {
                self.apply_run(&mut style, &self.declarations.resolve_style(&name).run);
            } else if values::normalize_name(&name) == "code" {
                self.apply_named_run(&mut style, "Code");
            }
        }
        self.apply_run(&mut style, &element.attributes);
        style
    }
    fn find_declaration(&self, name: &str) -> Option<&StyleDeclaration> {
        self.document.styles.get(name).or_else(|| {
            let normalized = values::normalize_name(name);
            if normalized.is_empty() {
                return None;
            }
            self.document
                .styles
                .iter()
                .find(|(name, _)| values::normalize_name(name) == normalized)
                .map(|(_, declaration)| declaration)
        })
    }
    fn apply_font(&self, style: &mut TextStyle, name: &str) {
        style.measurement.font = name.into();
        let family;
        if let Some(font) = self.document.fonts.get(name) {
            if let Some(scale) = font.properties.get("word_line_height") {
                style.word_line_height = values::number(scale).filter(|value| *value > 0.0);
                return;
            }
            family = values::text(&font.properties, "latin");
        } else {
            family = name.into();
        }
        style.word_line_height = match family.trim().to_lowercase().as_str() {
            "mono" | "courier" | "courier new" | "couriernewpsmt" => Some(1.137391304347826),
            _ => None,
        };
    }
    pub(super) fn apply_run(&self, style: &mut TextStyle, properties: &Properties) {
        let font = values::text(properties, "font");
        if !font.is_empty() {
            self.apply_font(style, &font);
        }
        if let Some(size) = properties.get("size").and_then(values::points) {
            style.measurement.size_points = size;
        }
        if let Some(color) = values::color(properties, "color") {
            style.color = color;
        }
        if let Some(fill) = values::color(properties, "highlight") {
            style.highlight = Some(fill);
        }
        if let Some(fill) = values::color(properties, "fill") {
            style.highlight = Some(fill);
        }
        let (mut bold, mut italic) = emphasis(style.measurement.variant);
        if properties.contains_key("bold") {
            bold = values::boolean(properties, "bold");
        }
        if properties.contains_key("italic") {
            italic = values::boolean(properties, "italic");
        }
        style.measurement.variant = FontVariant::from_emphasis(bold, italic);
        if properties.contains_key("underline") {
            style.underline = values::boolean(properties, "underline");
        }
        if properties.contains_key("strike") || properties.contains_key("double_strike") {
            style.double_strike = values::boolean(properties, "double_strike");
            style.strike = style.double_strike || values::boolean(properties, "strike");
        }
        if properties.contains_key("small_caps") {
            style.small_caps = values::boolean(properties, "small_caps");
        }
    }
    fn apply_paragraph(&self, style: &mut ParagraphStyle, properties: &Properties) {
        let alignment = values::text(properties, "alignment");
        if !alignment.is_empty() {
            style.alignment = alignment;
        }
        if let Some(value) = properties.get("spacing_before").and_then(values::points) {
            style.spacing_before = value;
        }
        if let Some(value) = properties.get("spacing_after").and_then(values::points) {
            style.spacing_after = value;
        }
        let nested = match properties.get("spacing") {
            Some(Value::Object(properties)) => Some(properties),
            _ => None,
        };
        if let Some(properties) = nested {
            if let Some(value) = properties
                .get("before")
                .or_else(|| properties.get("spacing_before"))
                .and_then(values::points)
            {
                style.spacing_before = value;
            }
            if let Some(value) = properties
                .get("after")
                .or_else(|| properties.get("spacing_after"))
                .and_then(values::points)
            {
                style.spacing_after = value;
            }
        }
        if properties.contains_key("keep_with_next") {
            style.keep_with_next = values::boolean(properties, "keep_with_next");
        }
        if properties.contains_key("keep_together") {
            style.keep_together = values::boolean(properties, "keep_together");
        }
        if properties.contains_key("page_break_before") {
            style.page_break_before = values::boolean(properties, "page_break_before");
        }
        if properties.contains_key("widow_control") {
            style.widow_control = values::boolean(properties, "widow_control");
        }
        if let Some(spacing) = nested
            .and_then(|properties| {
                properties
                    .get("line")
                    .or_else(|| properties.get("line_spacing"))
            })
            .or_else(|| properties.get("line_spacing"))
            .and_then(values::spacing)
        {
            style.line_spacing = spacing;
            if let LineSpacing::Multiple(value) = spacing {
                style.line_multiplier = value;
            }
            style.line_height = spacing.height(&style.text, style.line_multiplier);
        }
        if let Some(Value::Object(indent)) = properties.get("indent") {
            if let Some(value) = indent.get("left").and_then(values::points) {
                style.indent_left = value;
            }
            if let Some(value) = indent.get("right").and_then(values::points) {
                style.indent_right = value;
            }
            if let Some(value) = indent.get("first_line").and_then(values::points) {
                style.first_line = value;
            }
            if let Some(value) = indent.get("hanging").and_then(values::points) {
                style.first_line = -value;
            }
        }
    }
    fn apply_named_paragraph(&self, style: &mut ParagraphStyle, name: &str) {
        if self.find_declaration(name).is_some() {
            let resolved = self.declarations.resolve_style(name);
            self.apply_run(&mut style.text, &resolved.run);
            self.apply_paragraph(style, &resolved.paragraph);
        } else {
            self.apply_fallback(style, name);
        }
    }
    fn apply_named_run(&self, style: &mut TextStyle, name: &str) {
        if self.find_declaration(name).is_some() {
            self.apply_run(style, &self.declarations.resolve_style(name).run);
        } else {
            self.apply_font(style, "mono");
            style.measurement.size_points = 10.0;
        }
    }
    fn apply_fallback(&self, style: &mut ParagraphStyle, name: &str) {
        let normalized = values::normalize_name(name);
        let (mut bold, mut italic) = emphasis(style.text.measurement.variant);
        let mut multiple = Some(1.15);
        match normalized.as_str() {
            "heading1" | "heading2" | "heading3" | "heading4" | "heading5" | "heading6" => {
                let level = normalized.as_bytes()[7] as usize - b'1' as usize;
                style.text.measurement.size_points = [16.0, 14.0, 12.0, 11.0, 11.0, 11.0][level];
                bold = true;
                style.spacing_before = 8.0;
                style.spacing_after = 8.0;
                style.keep_with_next = true;
                multiple = Some(1.0);
            }
            "blockquote" => {
                self.apply_font(&mut style.text, "Helvetica");
                style.text.measurement.size_points = 11.0;
                italic = true;
                style.spacing_before = 6.0;
                style.spacing_after = 6.0;
                style.indent_left += 36.0;
            }
            "caption" => {
                italic = true;
                style.text.color = "666666".into();
                style.spacing_after = 8.0;
            }
            "header" | "footer" => {
                style.text.color = "6B7280".into();
                style.spacing_after = 0.0;
            }
            "code" | "codeblock" => {
                self.apply_font(&mut style.text, "mono");
                style.text.measurement.size_points = 10.0;
                style.spacing_after = 8.0;
            }
            "definitionterm" => {
                self.apply_font(&mut style.text, "Helvetica");
                style.text.measurement.size_points = 11.0;
                bold = true;
                style.spacing_after = 8.0;
            }
            "definition" => {
                self.apply_font(&mut style.text, "Helvetica");
                style.text.measurement.size_points = 11.0;
                style.indent_left += 18.0;
                style.spacing_after = 8.0;
            }
            "horizontalrule" => {
                self.apply_font(&mut style.text, "Helvetica");
                style.text.measurement.size_points = 11.0;
                style.text.color = "D1D5DB".into();
                style.spacing_before = 6.0;
                style.spacing_after = 8.0;
                style.line_spacing = LineSpacing::Exact(1.0);
                style.line_height = 1.0;
                multiple = None;
            }
            _ => return,
        }
        style.text.measurement.variant = FontVariant::from_emphasis(bold, italic);
        if let Some(multiple) = multiple {
            style.line_spacing = LineSpacing::Multiple(multiple);
            style.line_multiplier = multiple;
            style.line_height = 0.0;
        }
    }
}
pub(super) fn emphasis(variant: FontVariant) -> (bool, bool) {
    match variant {
        FontVariant::Regular => (false, false),
        FontVariant::Bold => (true, false),
        FontVariant::Italic => (false, true),
        FontVariant::BoldItalic => (true, true),
    }
}
