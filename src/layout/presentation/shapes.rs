use crate::{
    layout::{
        LayoutError, PageOperation, PaintedPath, PathPaint, TextPage,
        drawing_text::{self, FittedText},
        drawings::geometry,
    },
    measurement::TextMeasurer,
    model::{Document, Element, Properties},
    presentation::Theme,
    properties::{number, object, text},
};

pub(crate) fn fit(
    document: &Document,
    element: &Element,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
) -> Result<FittedText, LayoutError> {
    let attributes = crate::drawings::text_attributes(&element.attributes);
    let paragraphs = crate::presentation::text::resolve(
        document,
        &element.children,
        &attributes,
        theme,
        crate::presentation::text::TextDefaults::Resolved,
    )?;
    let mut bounds = geometry::text_rectangle(
        &text(&element.attributes, "kind"),
        geometry::rectangle(&element.attributes),
    );
    bounds[1] -= geometry::points(attributes.get("size"), 18.0).max(0.0) * 0.055;
    drawing_text::fit(&paragraphs, bounds, &attributes, measurer)
}
pub(super) fn place(
    document: &Document,
    element: &Element,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
    page: &mut TextPage,
) -> Result<(), LayoutError> {
    let bounds = geometry::rectangle(&element.attributes);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return Ok(());
    }
    page.operations.extend(
        geometry::shape(&element.attributes)
            .into_iter()
            .map(PageOperation::Path),
    );
    let fitted = fit(document, element, theme, measurer)?;
    let rotation =
        (number(&element.attributes, "rotation").unwrap_or(0.0) * 60000.0).round() / 60000.0;
    if rotation != 0.0 {
        page.operations.push(PageOperation::PushRotation {
            degrees: rotation,
            center: [bounds[0] + bounds[2] / 2.0, bounds[1] + bounds[3] / 2.0],
        });
    }
    page.append_content(fitted.page);
    if rotation != 0.0 {
        page.operations.push(PageOperation::PopRotation);
    }
    Ok(())
}
pub(super) fn connector(attributes: &Properties) -> Vec<PageOperation> {
    let endpoint = |name| {
        let fields = object(attributes, name);
        ["x", "y"].map(|field| geometry::points(fields.and_then(|fields| fields.get(field)), 0.0))
    };
    let start = endpoint("from");
    let end = endpoint("to");
    if start == end {
        return Vec::new();
    }
    let mut border = object(attributes, "border").cloned().unwrap_or_default();
    for name in ["opacity", "cap"] {
        if let Some(value) = attributes.get(name) {
            border.entry(name.into()).or_insert_with(|| value.clone());
        }
    }
    let Some(stroke) = geometry::stroke(&border) else {
        return Vec::new();
    };
    let mut output = vec![PageOperation::Path(PaintedPath {
        points: vec![start, end],
        closed: false,
        fill: None,
        stroke: Some(stroke.clone()),
    })];
    if let Some(arrow) = object(attributes, "arrow") {
        for (prefix, tip, other) in [("end", end, start), ("start", start, end)] {
            let width = marker_dimension(arrow, &[&format!("{prefix}_size"), "size"], stroke.width);
            let length = marker_dimension(
                arrow,
                &[
                    &format!("{prefix}_length"),
                    "length",
                    &format!("{prefix}_size"),
                    "size",
                ],
                stroke.width,
            );
            if let Some(path) = marker(
                tip,
                other,
                &text(arrow, prefix),
                width,
                length,
                stroke.paint.clone(),
            ) {
                output.push(PageOperation::Path(path));
            }
        }
    }
    output
}
fn marker(
    tip: [f64; 2],
    other: [f64; 2],
    kind: &str,
    width: f64,
    marker_length: f64,
    paint: PathPaint,
) -> Option<PaintedPath> {
    if kind.is_empty() || kind == "none" {
        return None;
    }
    let length = (tip[0] - other[0]).hypot(tip[1] - other[1]);
    let direction = [(tip[0] - other[0]) / length, (tip[1] - other[1]) / length];
    let base = [
        tip[0] - direction[0] * marker_length,
        tip[1] - direction[1] * marker_length,
    ];
    let half_width = width * 0.55;
    let first = [
        base[0] - direction[1] * half_width,
        base[1] + direction[0] * half_width,
    ];
    let last = [
        base[0] + direction[1] * half_width,
        base[1] - direction[0] * half_width,
    ];
    let points = match kind {
        "oval" => (0..24)
            .map(|index| {
                let angle = index as f64 * std::f64::consts::TAU / 24.0;
                if width == marker_length {
                    return [
                        tip[0] + angle.cos() * width / 2.0,
                        tip[1] + angle.sin() * width / 2.0,
                    ];
                }
                [
                    tip[0] + direction[0] * angle.cos() * marker_length / 2.0
                        - direction[1] * angle.sin() * width / 2.0,
                    tip[1]
                        + direction[1] * angle.cos() * marker_length / 2.0
                        + direction[0] * angle.sin() * width / 2.0,
                ]
            })
            .collect(),
        "diamond" => vec![
            [
                tip[0] + direction[0] * marker_length / 2.0,
                tip[1] + direction[1] * marker_length / 2.0,
            ],
            [
                tip[0] - direction[1] * width / 2.0,
                tip[1] + direction[0] * width / 2.0,
            ],
            [
                tip[0] - direction[0] * marker_length / 2.0,
                tip[1] - direction[1] * marker_length / 2.0,
            ],
            [
                tip[0] + direction[1] * width / 2.0,
                tip[1] - direction[0] * width / 2.0,
            ],
        ],
        "stealth" => vec![
            tip,
            first,
            [
                tip[0] - direction[0] * marker_length * 0.62,
                tip[1] - direction[1] * marker_length * 0.62,
            ],
            last,
        ],
        _ => vec![tip, first, last],
    };
    Some(PaintedPath {
        points,
        closed: true,
        fill: Some(paint),
        stroke: None,
    })
}

fn marker_dimension(attributes: &Properties, names: &[&str], stroke_width: f64) -> f64 {
    let value = names
        .iter()
        .map(|name| text(attributes, name))
        .find(|value| !value.is_empty())
        .unwrap_or_default();
    let factor = match value.as_str() {
        "small" | "sm" => 2.0,
        "large" | "lg" => 5.0,
        _ => 3.0,
    };
    (stroke_width * factor).max(3.0)
}
