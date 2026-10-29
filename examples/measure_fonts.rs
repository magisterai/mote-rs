//! Emit candidate measurements for the frozen measurement request format.
use mote::measurement::{CoreFont, FontVariant, OpenTypeFont, TextMeasurement};
use serde_json::{Value, json};
use std::{collections::BTreeMap, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let fixture = arguments
        .next()
        .ok_or("expected fixture directory and output JSON path")?;
    let output = arguments.next().ok_or("expected output JSON path")?;
    if arguments.next().is_some() {
        return Err("unexpected argument".into());
    }
    let root = Path::new(&fixture);
    let requests: Value = serde_json::from_slice(&fs::read(root.join("requests.json"))?)?;
    let capture: Value = serde_json::from_slice(&fs::read(root.join("capture.json"))?)?;
    let measurements = measure_requests(root, &requests)?;
    let result = json!({"schema_version": 1, "requests_sha256": capture["requests_sha256"], "measurements": measurements});
    fs::write(output, serde_json::to_string_pretty(&result)? + "\n")?;
    Ok(())
}

fn measure_requests(root: &Path, requests: &Value) -> Result<Vec<Value>, Box<dyn Error>> {
    let fonts: BTreeMap<_, _> = requests["fonts"]
        .as_array()
        .ok_or("missing fonts")?
        .iter()
        .map(|font| (font["id"].as_str().unwrap_or_default(), font))
        .collect();
    let mut measurements = Vec::new();
    for request in requests["cases"].as_array().ok_or("missing cases")? {
        let font_identifier = request["font_id"].as_str().ok_or("missing font_id")?;
        let descriptor = fonts.get(font_identifier).ok_or("unknown font_id")?;
        let variant = match descriptor["variant"].as_str().ok_or("missing variant")? {
            "regular" => FontVariant::Regular,
            "bold" => FontVariant::Bold,
            "italic" => FontVariant::Italic,
            "bold_italic" => FontVariant::BoldItalic,
            _ => return Err("unknown variant".into()),
        };
        let text = request["text"].as_str().ok_or("missing text")?;
        let size = request["size_points"].as_f64().ok_or("missing size")?;
        let measured = if descriptor["format"] == "core" {
            let font = match descriptor["family"].as_str().ok_or("missing family")? {
                "Helvetica" => CoreFont::Helvetica,
                "Times" => CoreFont::Times,
                "Courier" => CoreFont::Courier,
                _ => return Err("unknown core font".into()),
            };
            font.measure(text, size, variant)?
        } else {
            let bytes =
                fs::read(root.join(descriptor["path"].as_str().ok_or("missing font path")?))?;
            OpenTypeFont::parse(&bytes)?.measure(
                text,
                size,
                request["layout_kerning"]
                    .as_bool()
                    .ok_or("missing kerning flag")?,
            )?
        };
        measurements.push(result_record(request, descriptor, &measured));
    }
    Ok(measurements)
}

fn result_record(request: &Value, descriptor: &Value, measured: &TextMeasurement) -> Value {
    json!({
        "id": request["id"], "font_id": request["font_id"], "variant": descriptor["variant"],
        "metric_source": if descriptor["format"] == "core" { "core" } else { "declared" },
        "prepared_text": measured.prepared_text,
        "missing_codepoints": measured.missing_codepoints.iter().map(|character| *character as u32).collect::<Vec<_>>(),
        "layout_width": measured.layout_width, "paint_width": measured.paint_width,
        "ascent": measured.extents.ascent, "descent": measured.extents.descent, "height": measured.extents.height,
    })
}
