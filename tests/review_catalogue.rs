//! Editable review sources remain executable without storing rendered expectations.
use mote::{
    evaluation::EvaluationOptions,
    render::{self, LayoutMode},
    source::FileAssetResolver,
};
use std::{
    fs,
    io::{Cursor, Read},
    path::Path,
};

#[test]
fn catalogue_cases_preserve_page_counts_and_emit_well_formed_editable_packages() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cases");
    let cases: serde_json::Value =
        serde_json::from_str(include_str!("cases/catalogue.json")).unwrap();
    let resolver = FileAssetResolver::new(&root).unwrap();
    let mut failures = Vec::new();
    for case in cases.as_array().unwrap() {
        let identifier = case["identifier"].as_str().unwrap();
        let result = check_case(&root, &resolver, case);
        if let Err(error) = result {
            failures.push(format!("{identifier}: {error}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn check_case(
    root: &Path,
    resolver: &FileAssetResolver,
    case: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    for font in case["required_fonts"].as_array().unwrap() {
        let font = font.as_str().unwrap();
        if !Path::new(font).is_file() {
            return Err(format!("required font file is unavailable: {font}").into());
        }
    }
    let source = root.join(case["source"].as_str().unwrap());
    let document = mote::compile_with_options(
        &fs::read_to_string(&source)?,
        EvaluationOptions {
            asset_resolver: Some(resolver),
            source_name: Some(source.as_os_str()),
            enable_lua: case["evaluation"] == "lua",
            ..Default::default()
        },
    )?;
    let layout = if case["application"] == "word" {
        LayoutMode::Document
    } else {
        LayoutMode::Presentation
    };
    let plan = render::plan_with_layout(&document, resolver, Some(source.as_os_str()), layout)?;
    let expected = case["pages"].as_u64().unwrap() as usize;
    if plan.output.pages.len() != expected {
        return Err(format!(
            "expected {expected} pages, found {}",
            plan.output.pages.len()
        )
        .into());
    }
    let package = match layout {
        LayoutMode::Document => {
            mote::docx::emit_with_assets(&document, resolver, Some(source.as_os_str()))?.bytes
        }
        _ => mote::pptx::emit_with_assets(&document, resolver, Some(source.as_os_str()))?.bytes,
    };
    let mut archive = zip::ZipArchive::new(Cursor::new(package))?;
    for index in 0..archive.len() {
        let mut part = archive.by_index(index)?;
        let name = part.name()?.into_owned();
        if name.ends_with(".xml") || name.ends_with(".rels") {
            let mut source = String::new();
            part.read_to_string(&mut source)?;
            roxmltree::Document::parse(&source).map_err(|error| format!("{name}: {error}"))?;
        }
    }
    Ok(())
}
