use super::{LayoutStyleResolver, TextRun, TextStyle, style_values};
use crate::model::{Content, ElementKind};

/// Resolve inline styles and merge adjacent equal runs before measuring them.
/// Block descendants contribute text; their independent block geometry is not resolved here.
pub fn resolve_inline(
    content: &[Content],
    inherited: &TextStyle,
    styles: &LayoutStyleResolver<'_>,
) -> Vec<TextRun> {
    let mut runs = Vec::new();
    collect(content, inherited, styles, false, &mut runs);
    let mut merged: Vec<TextRun> = Vec::new();
    for run in runs {
        if let Some(previous) = merged.last_mut()
            && previous.current_page_field.is_none()
            && run.current_page_field.is_none()
            && previous.footnote_number.is_none()
            && run.footnote_number.is_none()
            && run.anchors.is_empty()
            && previous.internal_target == run.internal_target
            && previous.link == run.link
            && previous.style == run.style
            && previous.preserve_space == run.preserve_space
            && previous.synthetic_tab == run.synthetic_tab
        {
            previous.text.push_str(&run.text);
        } else {
            merged.push(run);
        }
    }
    merged
}
fn collect(
    content: &[Content],
    inherited: &TextStyle,
    styles: &LayoutStyleResolver<'_>,
    preserve_space: bool,
    runs: &mut Vec<TextRun>,
) {
    for node in content {
        match node {
            Content::Text(text) => {
                let mut run = TextRun::new(text, inherited.clone());
                run.preserve_space = preserve_space;
                runs.push(run);
            }
            Content::Element(element) => {
                if matches!(element.kind, ElementKind::Footnote | ElementKind::Endnote)
                    && let Some(number) =
                        crate::properties::number(&element.attributes, super::notes::NUMBER)
                {
                    let number = number as usize;
                    let text = if element.kind == ElementKind::Footnote {
                        number.to_string()
                    } else {
                        crate::numbering::format_counter(number as u64, "lower_roman")
                            .unwrap_or_else(|_| number.to_string())
                    };
                    let mut style = inherited.clone();
                    style.superscript = true;
                    style.measurement.size_points *= 0.75;
                    let mut run = TextRun::new(text, style);
                    run.footnote_number = (element.kind == ElementKind::Footnote).then_some(number);
                    runs.push(run);
                    continue;
                }
                if element.kind == ElementKind::Label {
                    continue;
                }
                if element.kind == ElementKind::Bookmark {
                    let start = runs.len();
                    collect(&element.children, inherited, styles, preserve_space, runs);
                    let name = crate::properties::text(&element.attributes, "name")
                        .trim()
                        .to_owned();
                    if !name.is_empty()
                        && let Some(run) = runs.get_mut(start)
                    {
                        run.anchors.push(name);
                    }
                    continue;
                }
                if element.kind == ElementKind::Reference {
                    let resolution = styles.anchors.resolve(element);
                    let mut text = resolution.text;
                    let page = resolution.display == "page";
                    if page {
                        if let Some(number) = styles.reference_pages.get(&resolution.target) {
                            text = number.to_string();
                        } else if text.is_empty() || text == resolution.target {
                            text = "?".into();
                        }
                    }
                    let mut style = inherited.clone();
                    if resolution.anchor.is_some() {
                        style.color = "0563C1".into();
                        style.underline = true;
                    }
                    let start = runs.len();
                    if !page
                        && !element.children.is_empty()
                        && crate::properties::text(&element.attributes, "fallback").is_empty()
                        && text == crate::references::text(&element.children)
                    {
                        collect(&element.children, &style, styles, preserve_space, runs);
                    } else {
                        if text.is_empty() {
                            text = "[ref]".into();
                        }
                        runs.push(TextRun::new(text, style));
                    }
                    if resolution.anchor.is_some() {
                        for run in &mut runs[start..] {
                            run.internal_target = Some(resolution.target.clone());
                        }
                    }
                    continue;
                }
                if element.kind == ElementKind::PageNumber {
                    let text = element
                        .attributes
                        .get(super::fields::RESOLVED_TEXT)
                        .map(crate::model::Value::to_text)
                        .unwrap_or_else(|| "1".into());
                    let mut run = TextRun::new(text, inherited.clone());
                    if crate::properties::text(&element.attributes, "kind") != "total" {
                        run.current_page_field = Some(super::CurrentPageField {
                            identifier: crate::properties::number(
                                &element.attributes,
                                super::fields::IDENTIFIER,
                            )
                            .map(|number| number as usize),
                        });
                    }
                    runs.push(run);
                    continue;
                }
                if element.kind == ElementKind::Image {
                    let label = ["alt", "src"]
                        .iter()
                        .find_map(|name| {
                            element
                                .attributes
                                .get(*name)
                                .map(crate::model::Value::to_text)
                                .filter(|value| !value.is_empty())
                        })
                        .unwrap_or_else(|| "image".into());
                    let placeholder = if label.starts_with('[') && label.ends_with(']') {
                        label
                    } else {
                        format!("[{label}]")
                    };
                    runs.push(TextRun::new(placeholder, inherited.clone()));
                    continue;
                }
                if element.kind == ElementKind::Link {
                    let mut style = inherited.clone();
                    style.color = "0563C1".into();
                    style.underline = true;
                    let start = runs.len();
                    collect(&element.children, &style, styles, preserve_space, runs);
                    let url = element
                        .attributes
                        .get("url")
                        .map(crate::model::Value::to_text)
                        .unwrap_or_default();
                    if !url.is_empty() {
                        for run in &mut runs[start..] {
                            run.link = Some(url.clone());
                        }
                    }
                    continue;
                }
                if matches!(element.kind, ElementKind::Tab | ElementKind::LineBreak) {
                    let mut run = TextRun::new(
                        if element.kind == ElementKind::Tab {
                            "\t"
                        } else {
                            "\n"
                        },
                        inherited.clone(),
                    );
                    run.preserve_space = preserve_space;
                    runs.push(run);
                    continue;
                }
                let style = if matches!(
                    element.kind,
                    ElementKind::Span
                        | ElementKind::Strong
                        | ElementKind::Emphasis
                        | ElementKind::Code
                ) {
                    styles.inline(element, inherited)
                } else {
                    inherited.clone()
                };
                let preserve_space = preserve_space
                    || (element.kind == ElementKind::Code
                        && style_values::boolean(&element.attributes, "preserve_space"));
                collect(&element.children, &style, styles, preserve_space, runs);
            }
        }
    }
}
