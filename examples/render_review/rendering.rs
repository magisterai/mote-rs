use super::{Application, ReviewCase, ReviewResult};
use mote::{
    diagnostics::Diagnostic,
    evaluation::EvaluationOptions,
    render::{self, PngOptions},
    source::{CachingAssetResolver, FileAssetResolver},
};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};

pub(super) struct ReviewArtifacts {
    pub native_files: BTreeMap<String, Vec<u8>>,
    pub manifest: Vec<u8>,
}

pub(super) fn fingerprint(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn diagnostic_message(diagnostic: &Diagnostic, source: &str, path: &Path) -> String {
    let (line, column) = diagnostic.location(source);
    let name = diagnostic
        .source
        .as_ref()
        .map_or_else(|| path.as_os_str(), |source| source.name.as_os_str());
    format!(
        "{}:{line}:{column}: {}",
        name.to_string_lossy(),
        diagnostic.message
    )
}

pub(super) fn generate(repository: &Path, case: &ReviewCase) -> ReviewResult<ReviewArtifacts> {
    let relative_source = format!("tests/cases/{}.mote", case.source_name);
    let path = repository.join(&relative_source);
    let source_bytes = fs::read(&path)?;
    let source = std::str::from_utf8(&source_bytes)?;
    let files = FileAssetResolver::new(repository)?;
    let resolver = CachingAssetResolver::new(&files);
    let document = mote::compile_with_options(
        source,
        EvaluationOptions {
            asset_resolver: Some(&resolver),
            source_name: Some(path.as_os_str()),
            ..Default::default()
        },
    )
    .map_err(|error| diagnostic_message(&error, source, &path))?;
    let evaluation_warnings: Vec<_> = document
        .warnings
        .iter()
        .map(|warning| diagnostic_message(warning, source, &path))
        .collect();
    let (package_bytes, package_warnings) = match case.application {
        Application::Word => {
            let package =
                mote::docx::emit_with_assets(&document, &resolver, Some(path.as_os_str()))?;
            (package.bytes, package.warnings)
        }
        Application::PowerPoint => {
            let package =
                mote::pptx::emit_with_assets(&document, &resolver, Some(path.as_os_str()))?;
            (package.bytes, package.warnings)
        }
    };
    let plan = render::plan_with_layout(
        &document,
        &resolver,
        Some(path.as_os_str()),
        case.application.layout(),
    )?;
    if plan.output.pages.len() != case.page_count {
        return Err(format!(
            "{}: expected {} pages, found {}",
            case.name,
            case.page_count,
            plan.output.pages.len()
        )
        .into());
    }
    let pdf = render::paint_pdf(&plan.output)?;
    let raster = render::paint_png(
        &plan.output,
        &PngOptions {
            dots_per_inch: 144,
            ..Default::default()
        },
    )?;
    let expected_pages: Vec<_> = (1..=case.page_count).collect();
    if raster
        .output
        .iter()
        .map(|page| page.number)
        .collect::<Vec<_>>()
        != expected_pages
    {
        return Err(format!("{}: PNG page numbers do not match the layout", case.name).into());
    }
    if fs::read(&path)? != source_bytes {
        return Err(format!("{}: source changed during rendering", case.name).into());
    }
    let mut native_files = BTreeMap::from([
        (format!("{}.mote", case.name), source_bytes),
        (
            format!("{}.{}", case.name, case.application.package_format()),
            package_bytes,
        ),
        (format!("{}.pdf", case.name), pdf),
    ]);
    for page in raster.output {
        native_files.insert(format!("{}-{}.png", case.name, page.number), page.bytes);
    }
    let mut manifest = BTreeMap::from([
        ("status".into(), "complete".into()),
        ("case".into(), case.name.into()),
        ("application".into(), case.application.directory().into()),
        ("source".into(), relative_source),
        ("fixture".into(), format!("{}.mote", case.name)),
        (
            "fixture_sha256".into(),
            fingerprint(&native_files[&format!("{}.mote", case.name)]),
        ),
        ("layout".into(), case.application.layout_name().into()),
        ("native_pdf_pages".into(), case.page_count.to_string()),
        ("native_png_pages".into(), case.page_count.to_string()),
        ("review_dpi".into(), "144".into()),
        ("office_review".into(), "unreviewed".into()),
        (case.application.review_status().into(), "skipped".into()),
    ]);
    let mut png_warnings = plan.warnings.clone();
    png_warnings.extend(raster.warnings);
    for (format, messages) in [
        (case.application.package_format(), package_warnings),
        ("pdf", plan.warnings),
        ("png", png_warnings),
    ] {
        let mut warnings = evaluation_warnings.clone();
        warnings.extend(messages);
        manifest.insert(format!("{format}_warnings"), warnings.len().to_string());
        manifest.insert(
            format!("{format}_warning_messages"),
            serde_json::to_string(&warnings)?,
        );
    }
    for (name, bytes) in &native_files {
        manifest.insert(format!("{name}_sha256"), fingerprint(bytes));
    }
    let manifest: String = manifest
        .iter()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect();
    Ok(ReviewArtifacts {
        native_files,
        manifest: manifest.into_bytes(),
    })
}
