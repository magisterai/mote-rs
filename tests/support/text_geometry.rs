//! Font identities and every measured text fragment, independent of raster tolerance.
use mote::layout::DocumentTextPlan;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub fn projection(plan: &DocumentTextPlan) -> Value {
    let mut pages = Vec::new();
    for page in &plan.pages {
        let geometry = page.geometry.unwrap_or(plan.geometry);
        let mut lines: Vec<_> = page
            .lines
            .iter()
            .map(|placed| {
                let line = &placed.line;
                let fragments: Vec<_> = line
                    .fragments
                    .iter()
                    .map(|fragment| {
                        let identity = fragment
                            .font
                            .asset_name
                            .as_ref()
                            .map(|path| {
                                format!("{:x}", Sha256::digest(std::fs::read(path).unwrap()))
                            })
                            .unwrap_or_else(|| fragment.font.family.clone());
                        let missing_codepoints: Vec<_> = fragment
                            .missing_codepoints
                            .iter()
                            .map(|character| *character as u32)
                            .collect();
                        json!({
                            "text": fragment.text,
                            "x": fragment.x,
                            "baseline": fragment.baseline,
                            "top": fragment.top,
                            "width": fragment.width,
                            "height": fragment.height,
                            "advance": fragment.advance,
                            "size": fragment.style.measurement.size_points,
                            "font": identity,
                            "variant": fragment.font.variant.as_text(),
                            "missing_codepoints": missing_codepoints,
                        })
                    })
                    .collect();
                json!({
                    "x": line.x,
                    "top": placed.top,
                    "height": line.height,
                    "width": line.width,
                    "baseline": line.baseline,
                    "fragments": fragments,
                })
            })
            .collect();
        lines.sort_by_key(|line| {
            (
                (line["top"].as_f64().unwrap() * 1e7).round() as i64,
                (line["x"].as_f64().unwrap() * 1e7).round() as i64,
                line["fragments"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|fragment| fragment["text"].as_str().unwrap())
                    .collect::<String>(),
            )
        });
        pages.push(json!({"width":geometry.width,"height":geometry.height,"lines":lines}));
    }
    Value::Array(pages)
}
