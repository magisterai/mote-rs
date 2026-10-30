//! Built-in slot geometry in points, rounded before conversion to drawing units.
use super::{Layout, Slot};
use crate::model::{Properties, Unit, Value};
use std::collections::BTreeMap;

pub(super) fn layouts([width, height]: [i64; 2]) -> BTreeMap<String, Layout> {
    let width = width as f64 / 12700.0;
    let height = height as f64 / 12700.0;
    let content_left = 50.4;
    let content_top = 100.8;
    let content_width = width - 100.8;
    let content_height = height - 136.8;
    let title = [content_left, 28.8, content_width, 57.6];
    let content = [content_left, content_top, content_width, content_height];
    let gap = width * 0.025;
    let column_width = (content_width - gap) / 2.0;
    let right = content_left + column_width + gap;
    let comparison_title_height = content_height * 0.11;
    let comparison_gap = height * 0.02;
    let comparison_top = content_top + comparison_title_height + comparison_gap;
    let comparison_height = (content_height - comparison_title_height - comparison_gap).max(1.0);
    let media_height = content_height * 0.82;
    let caption_top = content_top + media_height + height * 0.02;
    let caption_height = (content_top + content_height - caption_top).max(1.0);
    [
        ("blank", vec![]),
        (
            "title_slide",
            vec![
                slot(
                    "title",
                    "text",
                    [width * 0.07, height * 0.18, width * 0.86, height * 0.17],
                ),
                slot(
                    "subtitle",
                    "text",
                    [width * 0.12, height * 0.43, width * 0.76, height * 0.14],
                ),
            ],
        ),
        (
            "title_and_content",
            vec![
                slot("title", "text", title),
                slot("content", "content", content),
            ],
        ),
        (
            "section_header",
            vec![
                slot(
                    "title",
                    "text",
                    [width * 0.07, height * 0.31, width * 0.86, height * 0.16],
                ),
                slot(
                    "subtitle",
                    "text",
                    [width * 0.07, height * 0.49, width * 0.86, height * 0.12],
                ),
            ],
        ),
        (
            "two_column",
            vec![
                slot("title", "text", title),
                slot(
                    "left",
                    "content",
                    [content_left, content_top, column_width, content_height],
                ),
                slot(
                    "right",
                    "content",
                    [right, content_top, column_width, content_height],
                ),
            ],
        ),
        (
            "comparison",
            vec![
                slot("title", "text", title),
                slot(
                    "left_title",
                    "text",
                    [
                        content_left,
                        content_top,
                        column_width,
                        comparison_title_height,
                    ],
                ),
                slot(
                    "left",
                    "content",
                    [
                        content_left,
                        comparison_top,
                        column_width,
                        comparison_height,
                    ],
                ),
                slot(
                    "right_title",
                    "text",
                    [right, content_top, column_width, comparison_title_height],
                ),
                slot(
                    "right",
                    "content",
                    [right, comparison_top, column_width, comparison_height],
                ),
            ],
        ),
        (
            "image_and_caption",
            vec![
                slot("title", "text", title),
                slot(
                    "image",
                    "image",
                    [content_left, content_top, content_width, media_height],
                ),
                slot(
                    "caption",
                    "text",
                    [content_left, caption_top, content_width, caption_height],
                ),
            ],
        ),
        (
            "chart_and_caption",
            vec![
                slot("title", "text", title),
                slot(
                    "chart",
                    "chart",
                    [content_left, content_top, content_width, media_height],
                ),
                slot(
                    "caption",
                    "text",
                    [content_left, caption_top, content_width, caption_height],
                ),
            ],
        ),
    ]
    .into_iter()
    .map(|(name, slots)| {
        (
            name.into(),
            Layout {
                properties: Properties::new(),
                slots,
            },
        )
    })
    .collect()
}
fn slot(name: &str, kind: &str, bounds: [f64; 4]) -> Slot {
    Slot {
        name: name.into(),
        kind: kind.into(),
        required: false,
        properties: Properties::from([
            (
                "position".into(),
                Value::Object(
                    ["x", "y", "width", "height"]
                        .into_iter()
                        .zip(bounds)
                        .map(|(name, value)| {
                            (
                                name.into(),
                                Value::Length {
                                    value: (value * 1000.0).round() / 1000.0,
                                    unit: Unit::Point,
                                },
                            )
                        })
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
    }
}
