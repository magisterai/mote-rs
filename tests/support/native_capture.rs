#[allow(dead_code)]
#[path = "json_compare.rs"]
mod json_compare;
use mote::layout::{DocumentTextPlan, PageOperation};
use serde_json::{Value, json};
use std::io::Cursor;
pub fn projection(plan: &DocumentTextPlan) -> Value {
    let pages: Vec<_> = plan
        .pages
        .iter()
        .map(|page| {
            let lines: Vec<_> = page
                .lines
                .iter()
                .map(|placed| {
                    let fragments: Vec<_> = placed
                        .line
                        .fragments
                        .iter()
                        .map(|fragment| {
                            json!({
                                "text": fragment.text,
                                "x": fragment.x,
                                "baseline": fragment.baseline,
                                "advance": fragment.advance,
                                "size": fragment.style.measurement.size_points,
                            })
                        })
                        .collect();
                    json!({
                        "top": placed.top,
                        "height": placed.line.height,
                        "x": placed.line.x,
                        "width": if fragments.is_empty() { 0.0 } else { placed.line.width },
                        "fragments": fragments,
                    })
                })
                .collect();
            let rules: Vec<_> = page
                .operations
                .iter()
                .filter_map(|operation| {
                    let (rule, dash) = match operation {
                        PageOperation::Rule(rule) => (rule, &[][..]),
                        PageOperation::Stroke { rule, dash, .. } => (rule, dash.as_slice()),
                        _ => return None,
                    };
                    Some(json!({
                        "start": rule.start,
                        "end": rule.end,
                        "width": rule.width,
                        "color": rule.color,
                        "dash": dash,
                    }))
                })
                .collect();
            let images: Vec<_> = page
                .operations
                .iter()
                .filter_map(|operation| match operation {
                    PageOperation::Image(image) => Some(image.bounds),
                    _ => None,
                })
                .collect();
            json!({ "lines": lines, "rules": rules, "images": images })
        })
        .collect();
    json!({ "pages": pages })
}

/// Unless `MOTE_REFRESH_CAPTURES` is set, a harness compares its output with the
/// stored capture. When it is set after an intended layout change, a harness that
/// supports refresh writes its current output in place of the capture instead, so
/// that the change is reviewed in the resulting diff.
#[allow(dead_code)]
pub fn refreshing() -> bool {
    std::env::var_os("MOTE_REFRESH_CAPTURES").is_some()
}

/// Writes a refreshed capture file and its fingerprint in `native-capture.json`.
#[allow(dead_code)]
pub fn replace(folder: &std::path::Path, name: &str, bytes: &[u8]) {
    replace_recorded(folder, "native-capture.json", name, bytes);
}

/// Writes a refreshed capture file and its fingerprint in the named manifest.
#[allow(dead_code)]
pub fn replace_recorded(folder: &std::path::Path, manifest: &str, name: &str, bytes: &[u8]) {
    use sha2::{Digest, Sha256};
    let path = folder.join(name);
    let previous = format!("{:x}", Sha256::digest(std::fs::read(&path).unwrap()));
    std::fs::write(&path, bytes).unwrap();
    let manifest_path = folder.join(manifest);
    let manifest = std::fs::read_to_string(&manifest_path).unwrap();
    let current = format!("{:x}", Sha256::digest(bytes));
    std::fs::write(manifest_path, manifest.replace(&previous, &current)).unwrap();
}

/// Rewrites a captured JSON file and its fingerprint in `native-capture.json`,
/// changing only the values that moved.
#[allow(dead_code)]
pub fn replace_json(folder: &std::path::Path, name: &str, actual: &Value) {
    replace_json_recorded(folder, "native-capture.json", name, actual);
}

/// Rewrites a captured JSON file and its fingerprint in the named manifest.
#[allow(dead_code)]
pub fn replace_json_recorded(
    folder: &std::path::Path,
    manifest: &str,
    name: &str,
    actual: &Value,
) {
    let stored = std::fs::read_to_string(folder.join(name)).unwrap();
    let text = json_compare::rewrite(&stored, actual);
    replace_recorded(folder, manifest, name, text.as_bytes());
}

/// Rewrites a captured JSON file in place, changing only the values that moved.
#[allow(dead_code)]
pub fn rewrite_json(path: &std::path::Path, actual: &Value) {
    let stored = std::fs::read_to_string(path).unwrap();
    std::fs::write(path, json_compare::rewrite(&stored, actual)).unwrap();
}

pub fn compare(actual: &Value, expected: &Value, path: &str) {
    if let Some(difference) = mismatch(actual, expected, path) {
        panic!("{difference}");
    }
}

/// The first difference beyond the geometry tolerance, located by its path.
pub fn mismatch(actual: &Value, expected: &Value, path: &str) -> Option<String> {
    match (actual, expected) {
        (Value::Number(actual), Value::Number(expected)) => {
            ((actual.as_f64().unwrap() - expected.as_f64().unwrap()).abs() > 1e-8)
                .then(|| format!("{path}: {actual} != {expected}"))
        }
        (Value::Array(actual), Value::Array(expected)) => {
            if actual.len() != expected.len() {
                return Some(format!("{path}: array length"));
            }
            actual
                .iter()
                .zip(expected)
                .enumerate()
                .find_map(|(index, (actual, expected))| {
                    mismatch(actual, expected, &format!("{path}[{index}]"))
                })
        }
        (Value::Object(actual), Value::Object(expected)) => {
            if !actual.keys().eq(expected.keys()) {
                return Some(format!("{path}: fields"));
            }
            actual.iter().find_map(|(key, actual)| {
                mismatch(actual, &expected[key], &format!("{path}.{key}"))
            })
        }
        _ => (actual != expected).then(|| format!("{path}: {actual} != {expected}")),
    }
}
pub fn pixels(bytes: &[u8]) -> (u32, u32, Vec<u8>) {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA);
    let mut reader = decoder.read_info().unwrap();
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut pixels).unwrap();
    pixels.truncate(info.buffer_size());
    assert_eq!(info.color_type, png::ColorType::Rgba);
    (info.width, info.height, pixels)
}
