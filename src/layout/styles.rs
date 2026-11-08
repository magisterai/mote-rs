use super::{LineSpacing, TabAlignment, TabStop, TextStyle, style_values as values};
use crate::{
    formatting::StyleResolver,
    measurement::FontVariant,
    model::{
        Document, Element, ElementKind, Properties, StyleDeclaration, StyleKind, Value,
        normalize_style_name,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct BorderSide {
    pub width: f64,
    pub space: f64,
    pub color: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParagraphBorders {
    pub top: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub left: Option<BorderSide>,
}

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
    pub tab_stops: Vec<TabStop>,
    /// Word places the baseline of an exact line at four fifths of its height.
    pub word_exact_baseline: bool,
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
            tab_stops: Vec::new(),
            word_exact_baseline: false,
        }
    }
}

pub struct LayoutStyleResolver<'document> {
    pub(crate) document: &'document Document,
    declarations: StyleResolver<'document>,
    use_document_line_heights: bool,
    /// Single line heights of the named font files, as multiples of the size.
    font_line_heights: std::collections::BTreeMap<String, f64>,
    pub(super) anchors: crate::references::Catalog,
    pub(super) reference_pages: std::collections::BTreeMap<String, usize>,
}
impl<'document> LayoutStyleResolver<'document> {
    pub fn new(document: &'document Document) -> Self {
        Self {
            document,
            declarations: StyleResolver::new(document),
            use_document_line_heights: true,
            font_line_heights: Default::default(),
            anchors: crate::references::Catalog::build(document),
            reference_pages: Default::default(),
        }
    }
    pub(crate) fn for_drawing_text(document: &'document Document) -> Self {
        // DrawingML line spacing does not use document font calibrations.
        Self {
            use_document_line_heights: false,
            ..Self::new(document)
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
        let name = self.paragraph_style_name(element);
        direct(&element.attributes).or_else(|| {
            if self.find_declaration(&name).is_some() {
                direct(&self.declarations.resolve_style(&name).paragraph)
            } else if normalize_style_name(&name) == "horizontalrule" {
                Some("D1D5DB".into())
            } else {
                None
            }
        })
    }
    pub(super) fn borders(&self, element: &Element) -> ParagraphBorders {
        let mut borders = ParagraphBorders::default();
        let name = self.paragraph_style_name(element);
        if self.find_declaration(&name).is_some() {
            apply_borders(
                &mut borders,
                self.declarations
                    .resolve_style(&name)
                    .paragraph
                    .get("border"),
            );
        } else if normalize_style_name(&name) == "horizontalrule" {
            borders.bottom = Some(BorderSide {
                width: DEFAULT_BORDER_WIDTH,
                space: 0.0,
                color: "D1D5DB".into(),
            });
        }
        apply_borders(&mut borders, element.attributes.get("border"));
        borders
    }
    fn paragraph_style_name(&self, element: &Element) -> String {
        let name = values::text(&element.attributes, "style");
        if !name.is_empty() {
            return name;
        }
        if element.kind == ElementKind::Heading {
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
        }
    }
    pub fn paragraph(&self, element: &Element) -> ParagraphStyle {
        let mut style = ParagraphStyle {
            text: self.default_text(),
            word_exact_baseline: self.use_document_line_heights,
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
            style.line_height = style.line_spacing.height(&style.text);
        }
        style
    }
    pub(crate) fn drawing_paragraph(
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
            style.line_height = style.line_spacing.height(&style.text);
        }
        style
    }
    pub fn inline(&self, element: &Element, inherited: &TextStyle) -> TextStyle {
        let mut style = inherited.clone();
        let (mut bold, mut italic) = emphasis(style.measurement.variant);
        match element.kind {
            ElementKind::Strong => bold = true,
            ElementKind::Emphasis => italic = true,
            ElementKind::Underline => {
                style.underline = true;
                style.underline_style = values::text(&element.attributes, "style");
                style.underline_color =
                    values::parse_color(&values::text(&element.attributes, "color"))
                        .unwrap_or_default();
            }
            ElementKind::Strike => {
                style.strike = true;
                style.double_strike = values::text(&element.attributes, "kind") == "double";
            }
            ElementKind::SmallCaps => style.small_caps = true,
            ElementKind::Subscript => {
                style.subscript = true;
                style.measurement.size_points *= 0.75;
            }
            ElementKind::Superscript => {
                style.superscript = true;
                style.measurement.size_points *= 0.75;
            }
            ElementKind::Code | ElementKind::Raw => style.code = true,
            _ => {}
        }
        style.measurement.variant = FontVariant::from_emphasis(bold, italic);
        let name = values::text(&element.attributes, "style");
        if matches!(element.kind, ElementKind::Code | ElementKind::Raw) && name.is_empty() {
            self.apply_named_run(&mut style, "Code");
        }
        if !name.is_empty() {
            if self.find_declaration(&name).is_some() {
                self.apply_run(&mut style, &self.declarations.resolve_style(&name).run);
            } else if normalize_style_name(&name) == "code" {
                self.apply_named_run(&mut style, "Code");
            }
        }
        self.apply_run(&mut style, &element.attributes);
        style
    }
    fn find_declaration(&self, name: &str) -> Option<&StyleDeclaration> {
        self.document
            .find_style(name)
            .map(|(_, declaration)| declaration)
    }
    pub(crate) fn apply_font(&self, style: &mut TextStyle, name: &str) {
        style.measurement.font = name.into();
        if !self.use_document_line_heights {
            style.word_line_height = None;
            return;
        }
        let family;
        if let Some(font) = self.document.fonts.get(name) {
            if let Some(scale) = font.properties.get("word_line_height") {
                style.word_line_height = values::number(scale).filter(|value| *value > 0.0);
                return;
            }
            family = Some(values::text(&font.properties, "latin"))
                .filter(|latin| !latin.is_empty())
                .unwrap_or_else(|| name.into());
        } else {
            family = name.into();
        }
        style.word_line_height = builtin_word_line_height(&family)
            .or_else(|| self.font_line_heights.get(&family.to_lowercase()).copied());
    }
    /// Word sets a single line of a font file at the ascender, descender and
    /// line gap of its horizontal header. The core PDF fonts, which have no
    /// file, keep the default calibration.
    pub(super) fn measure_font_line_heights(
        &mut self,
        measurer: &mut crate::measurement::TextMeasurer<'_>,
    ) {
        let mut families = std::collections::BTreeSet::new();
        collect_font_names(&self.document.defaults, &mut families);
        collect_font_names(&self.document.properties, &mut families);
        for declaration in self.document.styles.values() {
            collect_font_names(&declaration.properties, &mut families);
        }
        collect_content_font_names(&self.document.content, &mut families);
        for section in &self.document.sections {
            collect_content_font_names(&section.content, &mut families);
        }
        // A declared font is measured by its name, which selects its files, and
        // recorded under its family.
        let mut requests: Vec<(String, String)> = self
            .document
            .fonts
            .iter()
            .map(|(name, font)| {
                let latin = values::text(&font.properties, "latin");
                let family = if latin.is_empty() {
                    name.clone()
                } else {
                    latin
                };
                (name.clone(), family)
            })
            .collect();
        for family in families {
            if !self.document.fonts.contains_key(&family) {
                requests.push((family.clone(), family));
            }
        }
        for (name, family) in requests {
            if family.is_empty() {
                continue;
            }
            let mut measurement = TextStyle::default().measurement;
            measurement.font = name;
            measurement.size_points = 100.0;
            if let Ok(measured) = measurer.measure(&measurement, "", false)
                && measured.font.bytes.is_some()
                && measured.measurement.extents.height > 0.0
            {
                self.font_line_heights
                    .entry(family.to_lowercase())
                    .or_insert(measured.measurement.extents.height / 100.0);
            }
        }
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
            style.line_height = spacing.height(&style.text);
        }
        if let Some(tabs) = properties.get("tabs") {
            style.tab_stops = tab_stops(tabs);
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
        let normalized = normalize_style_name(name);
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
pub(crate) fn emphasis(variant: FontVariant) -> (bool, bool) {
    match variant {
        FontVariant::Regular => (false, false),
        FontVariant::Bold => (true, false),
        FontVariant::Italic => (false, true),
        FontVariant::BoldItalic => (true, true),
    }
}

fn tab_stops(value: &Value) -> Vec<TabStop> {
    let entries = match value {
        Value::Array(entries) => entries.iter().collect(),
        Value::Object(_) => vec![value],
        _ => Vec::new(),
    };
    entries
        .into_iter()
        .filter_map(|entry| {
            let Value::Object(fields) = entry else {
                return None;
            };
            let shorthand = (fields.len() == 1)
                .then(|| fields.iter().next())
                .flatten()
                .filter(|(name, _)| {
                    matches!(name.as_str(), "left" | "center" | "right" | "decimal")
                });
            let (alignment, position) = match shorthand {
                Some((name, position)) => (name.clone(), position),
                None => {
                    let alignment = [
                        values::text(fields, "alignment"),
                        values::text(fields, "align"),
                    ]
                    .into_iter()
                    .find(|text| !text.is_empty())
                    .unwrap_or_default();
                    (
                        alignment,
                        fields.get("position").or_else(|| fields.get("pos"))?,
                    )
                }
            };
            let alignment = match alignment.as_str() {
                "center" => TabAlignment::Center,
                "right" | "end" => TabAlignment::Right,
                "decimal" => TabAlignment::Decimal,
                "" | "left" | "start" => TabAlignment::Left,
                _ => return None,
            };
            let leader = match values::text(fields, "leader").as_str() {
                "dot" => Some('.'),
                "hyphen" => Some('-'),
                "underscore" | "heavy" => Some('_'),
                "middle_dot" | "middleDot" => Some('·'),
                _ => None,
            };
            Some(TabStop {
                position: values::points(position)?,
                alignment,
                leader,
            })
        })
        .collect()
}

impl ParagraphBorders {
    /// Word adds a top or bottom border and its spacing to the paragraph's height.
    pub fn top_outset(&self) -> f64 {
        self.top.as_ref().map_or(0.0, BorderSide::outset)
    }
    pub fn bottom_outset(&self) -> f64 {
        self.bottom.as_ref().map_or(0.0, BorderSide::outset)
    }
}

impl BorderSide {
    pub fn outset(&self) -> f64 {
        self.space + self.width
    }
}

/// Matches the DOCX writer's defaults for an authored side.
const DEFAULT_BORDER_WIDTH: f64 = 0.5;
const DEFAULT_BORDER_COLOR: &str = "BFBFBF";

fn apply_borders(borders: &mut ParagraphBorders, value: Option<&Value>) {
    let Some(Value::Object(border)) = value else {
        return;
    };
    let all = border.get("all");
    for (name, side) in [
        ("top", &mut borders.top),
        ("right", &mut borders.right),
        ("bottom", &mut borders.bottom),
        ("left", &mut borders.left),
    ] {
        if let Some(value) = border.get(name).or(all) {
            *side = border_side(value);
        }
    }
}

fn border_side(value: &Value) -> Option<BorderSide> {
    let fields = match value {
        Value::String(style) if matches!(style.as_str(), "" | "none" | "nil") => return None,
        Value::String(_) => None,
        Value::Object(fields) => Some(fields),
        _ => return None,
    };
    let Some(fields) = fields else {
        return Some(BorderSide {
            width: DEFAULT_BORDER_WIDTH,
            space: 0.0,
            color: DEFAULT_BORDER_COLOR.into(),
        });
    };
    let style = [values::text(fields, "style"), values::text(fields, "val")]
        .into_iter()
        .find(|text| !text.is_empty())
        .unwrap_or_default();
    if matches!(style.as_str(), "none" | "nil") {
        return None;
    }
    let width = match fields.get("width") {
        Some(Value::Number(points)) => Some(*points),
        Some(width) => values::points(width),
        None => fields
            .get("size")
            .and_then(values::number)
            .map(|eighths| eighths / 8.0),
    };
    Some(BorderSide {
        width: width
            .filter(|width| *width > 0.0)
            .unwrap_or(DEFAULT_BORDER_WIDTH),
        space: fields.get("space").and_then(values::number).unwrap_or(0.0),
        color: values::color(fields, "color").unwrap_or_else(|| DEFAULT_BORDER_COLOR.into()),
    })
}

/// Returns the line height that Word gives a family, as a multiple of the font
/// size, where Word measured it differently from the font's own metrics. The MS
/// Gothic value was measured in Word at 11 and 22 points.
pub(super) fn builtin_word_line_height(family: &str) -> Option<f64> {
    match family.trim().to_lowercase().as_str() {
        "mono" | "courier" | "courier new" | "couriernewpsmt" => Some(1.137391304347826),
        "ms gothic" => Some(1.2964),
        "aptos" => Some(1.2131),
        _ => None,
    }
}

fn collect_font_names(properties: &Properties, families: &mut std::collections::BTreeSet<String>) {
    for (key, value) in properties {
        match value {
            Value::String(name) if key == "font" || key.ends_with("_font") || key == "latin" => {
                families.insert(name.clone());
            }
            Value::Object(nested) => collect_font_names(nested, families),
            _ => {}
        }
    }
}

fn collect_content_font_names(
    content: &[crate::model::Content],
    families: &mut std::collections::BTreeSet<String>,
) {
    for item in content {
        if let crate::model::Content::Element(element) = item {
            collect_font_names(&element.attributes, families);
            collect_content_font_names(&element.children, families);
        }
    }
}
