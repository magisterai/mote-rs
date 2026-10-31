#[path = "support/language_capture.rs"]
mod language_capture;

use mote::{
    evaluation::EvaluationOptions, layout::PageOperation, render, source::FileAssetResolver,
};
use std::{fs, path::PathBuf};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rendering-audit")
}

fn compile_case(name: &str) -> (mote::model::Document, FileAssetResolver, PathBuf) {
    let path = fixtures().join(format!("{name}.mote"));
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    let document = mote::compile_with_options(
        &fs::read_to_string(&path).unwrap(),
        EvaluationOptions {
            asset_resolver: Some(&resolver),
            source_name: Some(path.as_os_str()),
            enable_lua: true,
            ..Default::default()
        },
    )
    .unwrap();
    (document, resolver, path)
}

#[test]
fn combined_rendering_cases_preserve_models_packages_and_native_pages() {
    let cases: serde_json::Value =
        serde_json::from_slice(&fs::read(fixtures().join("cases.json")).unwrap()).unwrap();
    let successful: Vec<_> = cases
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case.get("expected_error").is_none())
        .map(|case| case["name"].as_str().unwrap())
        .collect();
    language_capture::compare("rendering-audit", &successful);
}

#[test]
fn loose_charts_report_unsupported_content_instead_of_emitting_an_empty_slide() {
    let (document, resolver, path) = compile_case("slide-charts");
    let expected = "presentation text boxes do not yet support chart";
    let editable = mote::pptx::emit_with_assets(&document, &resolver, Some(path.as_os_str()));
    assert!(matches!(editable, Err(error) if error.to_string().contains(expected)));
    let native = render::plan(&document, &resolver, Some(path.as_os_str()));
    assert!(native.unwrap_err().to_string().contains(expected));
}

#[test]
fn zero_chart_values_keep_labels_without_degenerate_paint_operations() {
    let (document, resolver, path) = compile_case("slide-positioned-charts");
    let planned = render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
    let page = &planned.output.pages[0];
    assert!(page.lines.iter().any(|line| {
        line.line
            .fragments
            .iter()
            .any(|fragment| fragment.text == "0")
    }));
    for operation in &page.operations {
        if let PageOperation::Rectangle(rectangle) = operation {
            assert!(rectangle.width > 0.0 && rectangle.height > 0.0);
        }
    }
    assert!(
        render::paint_pdf(&planned.output)
            .unwrap()
            .starts_with(b"%PDF-")
    );
    assert_eq!(
        render::paint_png(&planned.output, &Default::default())
            .unwrap()
            .output
            .len(),
        1
    );
}

#[test]
fn semantic_wrappers_preserve_page_notes_and_definition_endnotes() {
    let (document, resolver, path) = compile_case("document-formatted-notes");
    let planned = render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
    assert_eq!(planned.output.pages.len(), 1);
    let lines: Vec<_> = planned.output.pages[0]
        .lines
        .iter()
        .map(|line| {
            (
                line.top,
                line.line
                    .fragments
                    .iter()
                    .map(|fragment| fragment.text.as_str())
                    .collect::<String>(),
            )
        })
        .collect();
    for number in 1..=6 {
        let prefix = format!("{number} Note {number}:");
        let (top, _) = lines
            .iter()
            .find(|(_, text)| text.starts_with(&prefix))
            .unwrap();
        assert!(*top > 370.0, "note {number} must remain below the body");
    }
    assert!(
        lines
            .iter()
            .any(|(_, text)| text.starts_with("7 A term note"))
    );
    assert!(
        lines
            .iter()
            .any(|(_, text)| text.starts_with("i An endnote"))
    );
}
