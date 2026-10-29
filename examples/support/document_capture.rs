use mote::{
    layout::{DocumentTextPlan, LayoutError, PlacedTextLine, TextFragment, plan_document_text},
    measurement::TextMeasurer,
    source::FileAssetResolver,
};
use serde_json::{Value, json};
use std::{collections::BTreeSet, error::Error, ffi::OsStr};

pub fn capture_sources(requests: &Value) -> Result<Value, Box<dyn Error>> {
    if requests["schema_version"] != 1 {
        return Err("unsupported request schema".into());
    }
    let resolver = FileAssetResolver::from_current_directory()?;
    let mut identities = BTreeSet::new();
    let mut cases = Vec::new();
    for request in requests["cases"].as_array().ok_or("cases missing")? {
        let identity = request["id"].as_str().ok_or("case identity missing")?;
        if !identities.insert(identity) {
            return Err("duplicate case identity".into());
        }
        let source = request["source"].as_str().ok_or("source missing")?;
        let document = match mote::compile(source) {
            Ok(document) => document,
            Err(error) => {
                cases.push(json!({"id":identity,"compile_error":error.to_string()}));
                continue;
            }
        };
        let source_name = request["source_name"].as_str().map(OsStr::new);
        let mut measurer = TextMeasurer::new(&document, &resolver, source_name);
        let result = match plan_document_text(&document, &mut measurer) {
            Ok(plan) => {
                json!({"id":identity,"pages":pages_value(&plan, requests["include_operations"].as_bool().unwrap_or(false))})
            }
            Err(LayoutError::InvalidGeometry(message))
                if message.contains("line height must be positive") =>
            {
                json!({"id":identity,"invalid_line_height":true})
            }
            Err(LayoutError::Unsupported(message)) => json!({"id":identity,"unsupported":message}),
            Err(error) => json!({"id":identity,"layout_error":error.to_string()}),
        };
        cases.push(result);
    }
    Ok(json!({"schema_version":1,"cases":cases}))
}
fn pages_value(plan: &DocumentTextPlan, include_operations: bool) -> Value {
    Value::Array(
        plan.pages
            .iter()
            .map(|page| {
                let mut value =
                    json!({ "lines": page.lines.iter().map(line_value).collect::<Vec<_>>() });
                if include_operations {
                    value["operations"] = paint_operations(page);
                }
                value
            })
            .collect(),
    )
}
fn line_value(placed: &PlacedTextLine) -> Value {
    text_line_value(&placed.line, placed.top)
}
fn text_line_value(line: &mote::layout::TextLine, top: f64) -> Value {
    json!({
        "x": line.x,
        "top": top,
        "width": line.width,
        "height": line.height,
        "baseline": line.baseline,
        "fragments": line.fragments.iter().map(fragment_value).collect::<Vec<_>>()
    })
}
fn fragment_value(fragment: &TextFragment) -> Value {
    let color = u32::from_str_radix(&fragment.style.color, 16).unwrap_or(0x111827);
    json!({
        "text": fragment.text,
        "x": fragment.x,
        "baseline": fragment.baseline,
        "top": fragment.top,
        "width": fragment.width,
        "height": fragment.height,
        "advance": fragment.advance,
        "font_size": fragment.style.measurement.size_points,
        "font": if fragment.font.bytes.is_some() { "" } else { &fragment.font.family },
        "variant": fragment.font.variant.as_text(),
        "color": { "r":(color >> 16) & 255, "g":(color >> 8) & 255, "b":color & 255, "a":255 },
        "missing_codepoints":fragment.missing_codepoints.iter().map(|character| *character as u32).collect::<Vec<_>>()
    })
}

fn paint_operations(page: &mote::layout::TextPage) -> Value {
    use mote::layout::{ListMarkerKind, PageOperation};
    let mut operations = Vec::new();
    for operation in &page.operations {
        match operation {
            PageOperation::OutlineRectangle {bounds,width,color} => operations.push(json!({"kind":"outline","bounds":bounds,"width":width,"color":color})),
            PageOperation::Image(image) => operations.push(json!({"kind":"image","bounds":image.bounds,"clip":image.clip})),
            PageOperation::Text(index) => {
                let line=&page.lines[*index];
                for rectangle in &line.line.highlights { operations.push(rectangle_value(rectangle)); }
                operations.push(json!({"kind":"text","line":line_value(line)}));
                for decoration in &line.line.decorations {
                    operations.push(json!({"kind":"decoration","start":decoration.start,"end":decoration.end,"y":decoration.y,"width":decoration.width,"color":decoration.color,"decoration_kind":match decoration.kind { mote::layout::DecorationKind::Underline=>"underline", mote::layout::DecorationKind::Strike=>"strike" }}));
                }
            }
            PageOperation::Rectangle(rectangle) => operations.push(rectangle_value(rectangle)),
            PageOperation::Rule(rule) => operations.push(json!({"kind":"rule","start":rule.start,"end":rule.end,"width":rule.width,"color":rule.color})),
            PageOperation::ListMarker(marker) => operations.push(json!({"kind":"marker","marker_kind":match marker.kind {ListMarkerKind::Bullet=>"bullet",ListMarkerKind::Ordered=>"ordered",ListMarkerKind::Task=>"task"},"slot":marker.slot,"baseline":marker.baseline,"font_size":marker.font_size,"checked":marker.checked,"line":marker.text.as_ref().map(|line|text_line_value(line,line.baseline-line.height/2.0-line.fragments.iter().map(|fragment|fragment.style.measurement.size_points).fold(marker.font_size,f64::max)*0.3))})),
        }
    }
    Value::Array(operations)
}
fn rectangle_value(rectangle: &mote::layout::HighlightRectangle) -> Value {
    json!({"kind":"rectangle","bounds":[rectangle.x,rectangle.top,rectangle.width,rectangle.height],"color":rectangle.color})
}
