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
            Ok(plan) => json!({"id":identity,"pages":pages_value(&plan)}),
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
fn pages_value(plan: &DocumentTextPlan) -> Value {
    Value::Array(
        plan.pages
            .iter()
            .map(|page| {
                json!({
                    "lines": page.lines.iter().map(line_value).collect::<Vec<_>>()
                })
            })
            .collect(),
    )
}
fn line_value(placed: &PlacedTextLine) -> Value {
    let line = &placed.line;
    json!({
        "x": line.x,
        "top": placed.top,
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
        "font": fragment.font.family,
        "variant": fragment.font.variant.as_text(),
        "color": { "r":(color >> 16) & 255, "g":(color >> 8) & 255, "b":color & 255, "a":255 },
        "missing_codepoints":fragment.missing_codepoints.iter().map(|character| *character as u32).collect::<Vec<_>>()
    })
}
