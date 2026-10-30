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
            let size = text(arrow, &format!("{prefix}_size"));
            let size = if size.is_empty() {
                text(arrow, "size")
            } else {
                size
            };
            let size = (stroke.width
                * if matches!(size.as_str(), "large" | "lg") {
                    5.0
                } else {
                    3.0
                })
            .max(3.0);
            if let Some(path) = marker(tip, other, &text(arrow, prefix), size, stroke.paint.clone())
            {
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
    size: f64,
    paint: PathPaint,
) -> Option<PaintedPath> {
    if kind.is_empty() || kind == "none" {
        return None;
    }
    let length = (tip[0] - other[0]).hypot(tip[1] - other[1]);
    let direction = [(tip[0] - other[0]) / length, (tip[1] - other[1]) / length];
    let base = [tip[0] - direction[0] * size, tip[1] - direction[1] * size];
    let half_width = size * 0.55;
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
                [
                    tip[0] + angle.cos() * size / 2.0,
                    tip[1] + angle.sin() * size / 2.0,
                ]
            })
            .collect(),
        "stealth" => vec![
            tip,
            first,
            [
                tip[0] - direction[0] * size * 0.62,
                tip[1] - direction[1] * size * 0.62,
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
