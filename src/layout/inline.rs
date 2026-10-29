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
