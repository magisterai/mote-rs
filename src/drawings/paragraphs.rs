//! Resolves drawing paragraph, inline, and list semantics before measurement.
use crate::{
    layout::{LayoutStyleResolver, ParagraphStyle, TextRun, TextStyle},
    measurement::FontVariant,
    model::{Content, Element, ElementKind, Properties, Unit, Value},
    properties::{self, object, text},
};

#[derive(Clone)]
pub(crate) struct DrawingParagraph {
    pub style: ParagraphStyle,
    pub runs: Vec<TextRun>,
    pub task: Option<bool>,
    pub marker: Option<DrawingListMarker>,
}
#[derive(Clone)]
pub(crate) struct DrawingListMarker {
    pub level: usize,
    pub ordered: bool,
    pub number: u64,
    pub format: String,
    pub text: String,
}
pub(crate) fn paragraph_defaults(attributes: &Properties) -> Properties {
    let mut result = object(attributes, "paragraph").cloned().unwrap_or_default();
    for name in [
        "alignment",
        "spacing",
        "spacing_before",
        "spacing_after",
        "line_spacing",
        "indent",
        "list",
    ] {
        if let Some(value) = attributes.get(name) {
            result.entry(name.into()).or_insert_with(|| value.clone());
        }
    }
    result
}
fn inherited(defaults: &Properties, attributes: &Properties) -> Properties {
    let mut result = defaults.clone();
    result.extend(attributes.clone());
    result
}
fn resolve_runs(
    content: &[Content],
    base: &TextStyle,
    styles: &LayoutStyleResolver<'_>,
) -> Vec<TextRun> {
    let mut output = Vec::new();
    for child in content {
        let element = match child {
            Content::Text(text) => {
                output.push(TextRun::new(text, base.clone()));
                continue;
            }
            Content::Element(element) => element,
        };
        let mut style = base.clone();
        match element.kind {
            ElementKind::Strong | ElementKind::Emphasis | ElementKind::Span => {
                style = styles.inline(element, &style)
            }
            ElementKind::Code => {
                style.measurement.font = if styles.document.fonts.contains_key("mono") {
                    "mono"
                } else {
                    "Courier New"
                }
                .into();
                if style.measurement.size_points <= 0.0 {
                    style.measurement.size_points = 14.0;
                }
                style.word_line_height = Some(1.137391304347826);
                style.code = true;
                let mut attributes = element.attributes.clone();
                if !attributes.contains_key("style") && styles.document.styles.contains_key("Code")
                {
                    attributes.insert("style".into(), Value::String("Code".into()));
                }
                style = styles.inline(
                    &Element {
                        kind: ElementKind::Span,
                        attributes,
                        children: Vec::new(),
                    },
                    &style,
                );
            }
            ElementKind::Tab | ElementKind::LineBreak => {
                output.push(TextRun::new(
                    if element.kind == ElementKind::Tab {
                        "\t"
                    } else {
                        "\n"
                    },
                    style,
                ));
                continue;
            }
            _ => {}
        }
        let mut children = resolve_runs(&element.children, &style, styles);
        if element.kind == ElementKind::Code
            && properties::boolean(&element.attributes, "preserve_space")
        {
            for run in &mut children {
                run.preserve_space = true;
            }
        }
        if element.kind == ElementKind::Link {
            for run in &mut children {
                run.style.color = "2563EB".into();
                run.style.underline = true;
                run.link = Some(text(&element.attributes, "url"));
            }
        }
        output.extend(children);
    }
    output
}
fn paragraph(
    content: &[Content],
    base: &TextStyle,
    attributes: &Properties,
    implicit: &str,
    styles: &LayoutStyleResolver<'_>,
) -> DrawingParagraph {
    let mut style = styles.drawing_paragraph(base, attributes, implicit);
    let runs = resolve_runs(content, &style.text, styles);
    let smallest = runs
        .iter()
        .filter(|run| !run.text.is_empty() && run.style.measurement.size_points > 0.0)
        .min_by(|first, second| {
            first
                .style
                .measurement
                .size_points
                .total_cmp(&second.style.measurement.size_points)
        });
    if let Some(run) = smallest
        && run.style.measurement.size_points < style.text.measurement.size_points
    {
        style.text = run.style.clone();
        style.line_height = style
            .line_spacing
            .height(&style.text, style.line_multiplier);
    }
    DrawingParagraph {
        style,
        runs,
        task: None,
        marker: None,
    }
}
pub(crate) fn resolve(
    content: &[Content],
    base: &TextStyle,
    defaults: &Properties,
    heading_font: Option<&str>,
    styles: &LayoutStyleResolver<'_>,
) -> Vec<DrawingParagraph> {
    let mut output = Vec::new();
    for child in content {
        let element = match child {
            Content::Text(text) => {
                for part in text
                    .trim_matches('\n')
                    .split("\n\n")
                    .map(str::trim)
                    .filter(|part| !part.is_empty())
                {
                    output.push(paragraph(
                        &[Content::Text(part.into())],
                        base,
                        defaults,
                        "",
                        styles,
                    ));
                }
                continue;
            }
            Content::Element(element) => element,
        };
        let attributes = inherited(defaults, &element.attributes);
        match element.kind {
            ElementKind::Paragraph => output.push(paragraph(
                &element.children,
                base,
                &attributes,
                "Normal",
                styles,
            )),
            ElementKind::Heading => {
                let mut base = base.clone();
                let (_, italic) = crate::layout::styles::emphasis(base.measurement.variant);
                base.measurement.variant = FontVariant::from_emphasis(true, italic);
                let level =
                    properties::number(&element.attributes, "level").unwrap_or(1.0) as usize;
                base.measurement.size_points = match level {
                    2 => 28.0,
                    3 => 24.0,
                    _ => 36.0,
                };
                if let Some(font) = heading_font {
                    base.measurement.font = font.into();
                    base.word_line_height = None;
                }
                output.push(paragraph(
                    &element.children,
                    &base,
                    &attributes,
                    &format!("Heading{}", level.clamp(1, 9)),
                    styles,
                ));
            }
            ElementKind::Blockquote => {
                let mut base = base.clone();
                let (bold, _) = crate::layout::styles::emphasis(base.measurement.variant);
                base.measurement.variant = FontVariant::from_emphasis(bold, true);
                output.push(paragraph(
                    &element.children,
                    &base,
                    &attributes,
                    "BlockQuote",
                    styles,
                ));
            }
            ElementKind::List | ElementKind::OrderedList => {
                output.extend(resolve_list(element, base, defaults, 0, styles))
            }
            _ if !element.kind.is_block() => output.push(paragraph(
                std::slice::from_ref(child),
                base,
                defaults,
                "",
                styles,
            )),
            _ => {}
        }
    }
    output
}
fn resolve_list(
    element: &Element,
    base: &TextStyle,
    defaults: &Properties,
    level: usize,
    styles: &LayoutStyleResolver<'_>,
) -> Vec<DrawingParagraph> {
    let ordered = element.kind == ElementKind::OrderedList;
    let start = properties::number(&element.attributes, "start").unwrap_or(1.0) as u64;
    let format = text(&element.attributes, "format");
    let mut number = start;
    let mut output = Vec::new();
    for child in &element.children {
        let Content::Element(item) = child else {
            continue;
        };
        if item.kind != ElementKind::ListItem {
            continue;
        }
        let mut attributes = defaults.clone();
        if let Some(list) = object(defaults, "list") {
            attributes.extend(list.clone());
            if !list.contains_key("spacing")
                && ["spacing_before", "spacing_after", "line_spacing"]
                    .iter()
                    .any(|name| list.contains_key(*name))
            {
                attributes.remove("spacing");
            }
        }
        attributes.extend(element.attributes.clone());
        attributes.extend(item.attributes.clone());
        attributes.entry("indent".into()).or_insert_with(|| {
            Value::Object(Properties::from([
                (
                    "left".into(),
                    Value::Length {
                        value: (level + 1) as f64 * 27.0,
                        unit: Unit::Point,
                    },
                ),
                (
                    "hanging".into(),
                    Value::Length {
                        value: if ordered { 19.0 } else { 13.5 },
                        unit: Unit::Point,
                    },
                ),
            ]))
        });
        let children: Vec<_> = item
            .children
            .iter()
            .filter(|child| {
                !matches!(child, Content::Element(element)
                if matches!(element.kind, ElementKind::List | ElementKind::OrderedList))
            })
            .cloned()
            .collect();
        let mut paragraph = paragraph(&children, base, &attributes, "", styles);
        let marker = if ordered {
            format!(
                "{}.",
                crate::numbering::format_counter(number, &format)
                    .unwrap_or_else(|_| number.to_string())
            )
        } else {
            [&item.attributes, &element.attributes]
                .iter()
                .map(|attributes| text(attributes, "marker"))
                .find(|marker| !matches!(marker.as_str(), "" | "-" | "+" | "*"))
                .unwrap_or_else(|| "•".into())
        };
        paragraph.marker = Some(DrawingListMarker {
            level,
            ordered,
            number,
            format: format.clone(),
            text: marker,
        });
        if item.attributes.contains_key("checked") {
            paragraph.task = Some(properties::boolean(&item.attributes, "checked"));
        }
        output.push(paragraph);
        for child in &item.children {
            if let Content::Element(nested) = child
                && matches!(nested.kind, ElementKind::List | ElementKind::OrderedList)
            {
                output.extend(resolve_list(nested, base, defaults, level + 1, styles));
            }
        }
        number += 1;
    }
    output
}
