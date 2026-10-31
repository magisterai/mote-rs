//! Resolve loose slide content to positioned objects before choosing an output.
use crate::model::{Content, Element, ElementKind, Properties, Value};

const CONTENT_LEFT: i64 = 640080;
const CONTENT_RIGHT: i64 = 640080;
const TITLE_TOP: i64 = 365760;
const TITLE_HEIGHT: i64 = 731520;
const CONTENT_TOP: i64 = 1280160;
const CONTENT_BOTTOM: i64 = 457200;
const CONTENT_GAP: i64 = 182880;

pub(super) fn elements(
    content: &[Content],
    [width, height]: [i64; 2],
    mut chart_fits: impl FnMut(&Element, [i64; 2]) -> Result<bool, String>,
) -> Result<Vec<Content>, String> {
    let content: Vec<_> = content
        .iter()
        .filter(|node| meaningful(node))
        .cloned()
        .collect();
    if content.iter().all(positioned) {
        return Ok(content);
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
        if matches!(node, Content::Element(element) if matches!(element.kind, ElementKind::Image | ElementKind::Table | ElementKind::Chart))
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
        return Ok(output);
    }
    for columns in 1..=groups.len() {
        let rows = groups.len().div_ceil(columns);
        let frame_width = (content_width - CONTENT_GAP * (columns as i64 - 1)) / columns as i64;
        let frame_height = (content_height - CONTENT_GAP * (rows as i64 - 1)) / rows as i64;
        if frame_width <= 0 || frame_height <= 0 {
            continue;
        }
        let mut fits = true;
        for group in &groups {
            if let [Content::Element(chart)] = group.as_slice()
                && chart.kind == ElementKind::Chart
                && !chart_fits(chart, [frame_width, frame_height])?
            {
                fits = false;
                break;
            }
        }
        if !fits {
            continue;
        }
        for (index, group) in groups.into_iter().enumerate() {
            output.push(container(
                format!("content_{}", index + 1),
                [
                    CONTENT_LEFT + (index % columns) as i64 * (frame_width + CONTENT_GAP),
                    CONTENT_TOP + (index / columns) as i64 * (frame_height + CONTENT_GAP),
                    frame_width,
                    frame_height,
                ],
                group,
            ));
        }
        return Ok(output);
    }
    Err("automatic slide content does not fit; use positioned boxes, a larger slide, or fewer charts".into())
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
