use mote::{
    layout::{Alignment, LineBreak, LineOptions, LineSpacing, TextRun, TextStyle, plan_text},
    measurement::{FontVariant, MeasurementStyle, TextMeasurer},
    model::{Document, FontDeclaration, Properties, Value as ModelValue},
    source::FileAssetResolver,
};
use serde_json::{Value, json};
use std::{error::Error, path::Path};

pub fn measure_cases(root: &Path, requests: &Value) -> Result<Value, Box<dyn Error>> {
    let mut document = Document::default();
    for descriptor in requests["fonts"].as_array().ok_or("fonts missing")? {
        let name = descriptor["name"].as_str().ok_or("font name missing")?;
        let path = descriptor["path"].as_str().ok_or("font path missing")?;
        document.fonts.insert(
            name.into(),
            FontDeclaration {
                source_name: None,
                name: name.into(),
                properties: Properties::from([(
                    "files".into(),
                    ModelValue::Object(Properties::from([(
                        "regular".into(),
                        ModelValue::String(path.into()),
                    )])),
                )]),
            },
        );
    }
    let source_name = root.join("requests.json").canonicalize()?;
    let resolver = FileAssetResolver::from_current_directory()?;
    let mut measurer = TextMeasurer::new(&document, &resolver, Some(source_name.as_os_str()));
    let mut cases = Vec::new();
    for case in requests["cases"].as_array().ok_or("cases missing")? {
        let mut options = LineOptions {
            width: number(case, "width"),
            first_indent: number(case, "first_indent"),
            layout_kerning: boolean(case, "layout_kerning"),
            no_wrap: boolean(case, "no_wrap"),
            alignment: match case["alignment"].as_str().unwrap_or("left") {
                "center" => Alignment::Center,
                "right" => Alignment::Right,
                "justify" => Alignment::Justify,
                _ => Alignment::Left,
            },
            spacing: match case["line_rule"].as_str().unwrap_or("multiple") {
                "exact" => LineSpacing::Exact(number(case, "line_value")),
                "at_least" => LineSpacing::AtLeast(number(case, "line_value")),
                _ => LineSpacing::Multiple(number(case, "line_value")),
            },
            ..Default::default()
        };
        options.base_style.measurement.size_points = number(case, "base_size");
        let runs: Vec<_> = case["runs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|run| TextRun {
                link: None,
                internal_target: None,
                anchors: Vec::new(),
                current_page_field: None,
                footnote_number: None,
                text: run["text"].as_str().unwrap().into(),
                preserve_space: boolean(run, "preserve_space"),
                synthetic_tab: false,
                fixed_advance: None,
                style: TextStyle {
                    measurement: MeasurementStyle {
                        font: run["font"].as_str().unwrap_or("Helvetica").into(),
                        size_points: number(run, "size_points"),
                        variant: FontVariant::from_emphasis(
                            boolean(run, "bold"),
                            boolean(run, "italic"),
                        ),
                    },
                    word_line_height: run["word_line_height"].as_f64(),
                    small_caps: boolean(run, "small_caps"),
                    code: boolean(run, "code"),
                    ..Default::default()
                },
            })
            .collect();
        let lines = plan_text(&runs, &options, &mut measurer)?;
        cases.push(json!({"id":case["id"],"lines":lines.iter().map(|line| json!({
            "x":line.x,"width":line.width,"height":line.height,"baseline":line.baseline,
            "break":match line.line_break {LineBreak::Final=>0,LineBreak::Soft=>1,LineBreak::Hard=>2},
            "fragments":line.fragments.iter().map(|fragment| json!({"text":fragment.text,"x":fragment.x,"baseline":fragment.baseline,"width":fragment.width,"advance":fragment.advance,"size_points":fragment.style.measurement.size_points,"top":fragment.top,"height":fragment.height,"missing_codepoints":fragment.missing_codepoints.iter().map(|character|*character as u32).collect::<Vec<_>>()})).collect::<Vec<_>>()
        })).collect::<Vec<_>>()}));
    }
    Ok(json!({"schema_version":1,"cases":cases}))
}
fn number(value: &Value, name: &str) -> f64 {
    value[name].as_f64().unwrap_or(0.0)
}
fn boolean(value: &Value, name: &str) -> bool {
    value[name].as_bool().unwrap_or(false)
}
