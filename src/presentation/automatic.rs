//! Resolve loose slide content to positioned objects before choosing an output.
use crate::model::{Content, Element, ElementKind, Properties, Value};

const CONTENT_LEFT: i64 = 640080;
const CONTENT_RIGHT: i64 = 640080;
const TITLE_TOP: i64 = 365760;
const TITLE_HEIGHT: i64 = 731520;
const CONTENT_TOP: i64 = 1280160;
const CONTENT_BOTTOM: i64 = 457200;
const CONTENT_GAP: i64 = 182880;

pub(super) fn elements(content: &[Content], [width, height]: [i64; 2]) -> Vec<Content> {
    let content: Vec<_> = content
        .iter()
        .filter(|node| meaningful(node))
        .cloned()
        .collect();
    if content.iter().all(positioned) {
        return content;
    }
    let content_width = (width - CONTENT_LEFT - CONTENT_RIGHT).max(1);
    let content_height = (height - CONTENT_TOP - CONTENT_BOTTOM).max(1);
    let mut output = Vec::new();
    let remaining = if matches!(content.first(), Some(Content::Element(element)) if element.kind == ElementKind::Heading)
    {
        output.push(container(
            "title".into(),
            [CONTENT_LEFT, TITLE_TOP, content_width, TITLE_HEIGHT],
            vec![content[0].clone()],
        ));
        &content[1..]
    } else {
        content.as_slice()
    };
    let mut groups = Vec::new();
    let mut text = Vec::new();
    for node in remaining {
        if matches!(node, Content::Element(element) if matches!(element.kind, ElementKind::Image | ElementKind::Table))
        {
            if !text.is_empty() {
                groups.push(std::mem::take(&mut text));
            }
            groups.push(vec![node.clone()]);
        } else {
            text.push(node.clone());
        }
    }
    if !text.is_empty() {
        groups.push(text);
    }
    if groups.is_empty() {
        return output;
    }
    let group_count = groups.len() as i64;
    let group_height = (content_height - CONTENT_GAP * (group_count - 1)) / group_count;
    for (index, group) in groups.into_iter().enumerate() {
        output.push(container(
            format!("content_{}", index + 1),
            [
                CONTENT_LEFT,
                CONTENT_TOP + index as i64 * (group_height + CONTENT_GAP),
                content_width,
                group_height,
            ],
            group,
        ));
    }
    output
}
pub(super) fn meaningful(content: &Content) -> bool {
    !matches!(content, Content::Text(text) if text.trim().is_empty())
}
fn positioned(content: &Content) -> bool {
    matches!(content, Content::Element(element) if matches!(element.kind, ElementKind::Box | ElementKind::Shape | ElementKind::Line))
}
fn container(name: String, bounds: [i64; 4], children: Vec<Content>) -> Content {
    Content::Element(Element {
        source_name: None,
        kind: ElementKind::Box,
        attributes: Properties::from([
            ("name".into(), Value::String(name)),
            (
                "position".into(),
                Value::Object(
                    ["x", "y", "width", "height"]
                        .into_iter()
                        .zip(bounds)
                        .map(|(name, value)| (name.into(), Value::Number(value as f64)))
                        .collect(),
                ),
            ),
            (
                "text".into(),
                Value::Object(Properties::from([(
                    "fit".into(),
                    Value::String("shrink".into()),
                )])),
            ),
        ]),
        children,
    })
}
