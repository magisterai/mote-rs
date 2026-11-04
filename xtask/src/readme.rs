//! The README examples, generated through the library as editable packages, PDF
//! downloads and native previews.
use crate::{
    Outcome,
    arguments::{Arguments, unknown},
    package, repository, validate,
};
use mote::{
    evaluation::EvaluationOptions,
    model::Document,
    render::{self, LayoutMode, PngOptions},
    source::{CachingAssetResolver, FileAssetResolver},
};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

const HELP: &str = "Usage: cargo xtask readme

Regenerate the README examples as editable DOCX and PPTX downloads, PDF downloads
of the report and deck, and native PNG previews at 144 DPI. Every package is
validated against the ECMA-376 schemas before anything is published to
docs/examples/readme/ and docs/images/readme/.
";

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Docx,
    Pptx,
    Pdf,
}

impl Format {
    fn extension(self) -> &'static str {
        match self {
            Self::Docx => "docx",
            Self::Pptx => "pptx",
            Self::Pdf => "pdf",
        }
    }
}

struct Sample {
    name: &'static str,
    formats: &'static [Format],
    /// The first layout is primary: it also lays out the PDF download.
    previews: &'static [(LayoutMode, usize)],
}

/// Pure Markdown alone is rendered in both layouts, to show one source serving
/// both formats.
const SAMPLES: [Sample; 7] = [
    Sample {
        name: "plain",
        formats: &[Format::Docx, Format::Pptx],
        previews: &[(LayoutMode::Document, 1), (LayoutMode::Presentation, 1)],
    },
    Sample {
        name: "defaults",
        formats: &[Format::Docx],
        previews: &[(LayoutMode::Document, 1)],
    },
    Sample {
        name: "directives",
        formats: &[Format::Docx],
        previews: &[(LayoutMode::Document, 1)],
    },
    Sample {
        name: "chart",
        formats: &[Format::Docx],
        previews: &[(LayoutMode::Document, 1)],
    },
    Sample {
        name: "lua",
        formats: &[Format::Docx],
        previews: &[(LayoutMode::Document, 1)],
    },
    Sample {
        name: "report",
        formats: &[Format::Docx, Format::Pdf],
        previews: &[(LayoutMode::Document, 1)],
    },
    Sample {
        name: "deck",
        formats: &[Format::Pptx, Format::Pdf],
        previews: &[(LayoutMode::Presentation, 4)],
    },
];

impl Sample {
    fn source(&self, sources: &Path) -> Result<String, String> {
        let read = |name: &str| {
            fs::read_to_string(sources.join(name))
                .map_err(|error| format!("{}: {error}", sources.join(name).display()))
        };
        match self.name {
            "plain" => read("plain.md"),
            "defaults" => Ok(read("defaults.mote")? + "\n" + &read("plain.md")?),
            name => read(&format!("{name}.mote")),
        }
    }
    fn slides(&self) -> usize {
        self.previews
            .iter()
            .find(|(layout, _)| *layout == LayoutMode::Presentation)
            .map_or(1, |(_, pages)| *pages)
    }
    /// Pure Markdown deliberately uses the engine's fallback font outlines, and
    /// Markdown heading markers and repeated table headers have no slide appearance.
    fn expects(&self, warning: &str) -> bool {
        self.name == "plain"
            && (warning.contains("fallback")
                || warning.contains("substituted")
                || warning
                    .ends_with("heading.marker is retained without a presentation output effect")
                || warning.ends_with(
                    "row.repeat_header is retained without a presentation output effect",
                ))
    }
}

struct Generation<'a> {
    sample: &'a Sample,
    source: String,
    document: Document,
    resolver: &'a CachingAssetResolver<'a>,
    warnings: BTreeSet<String>,
}

impl Generation<'_> {
    /// Warnings take the command-line form, so that the published list reads as
    /// a reader would see it from `mote`.
    fn admit(&mut self, warning: String) -> Result<(), String> {
        if !self.sample.expects(&warning) {
            return Err(warning);
        }
        self.warnings.insert(warning);
        Ok(())
    }
    fn record(&mut self, warnings: impl IntoIterator<Item = String>) -> Result<(), String> {
        warnings
            .into_iter()
            .try_for_each(|warning| self.admit(format!("<stdin>: warning: {warning}")))
    }
    fn evaluation_warnings(&mut self) -> Result<(), String> {
        let warnings: Vec<_> = self
            .document
            .warnings
            .iter()
            .map(|warning| {
                let (line, column) = warning.location(&self.source);
                let name = warning
                    .source
                    .as_ref()
                    .map_or("<stdin>".into(), |source| source.name.to_string_lossy());
                format!("{name}:{line}:{column}: warning: {}", warning.message)
            })
            .collect();
        warnings
            .into_iter()
            .try_for_each(|warning| self.admit(warning))
    }
    fn package(&mut self, format: Format) -> Result<Vec<u8>, String> {
        let failure = |error: &dyn std::fmt::Display| {
            format!("{} {}: {error}", self.sample.name, format.extension())
        };
        let (bytes, warnings) = match format {
            Format::Docx => {
                let package = mote::docx::emit_with_assets(&self.document, self.resolver, None)
                    .map_err(|error| failure(&error))?;
                (package.bytes, package.warnings)
            }
            Format::Pptx => {
                let package = mote::pptx::emit_with_assets(&self.document, self.resolver, None)
                    .map_err(|error| failure(&error))?;
                (package.bytes, package.warnings)
            }
            Format::Pdf => {
                let layout = self.sample.previews[0].0;
                let plan = render::plan_with_layout(&self.document, self.resolver, None, layout)
                    .map_err(|error| failure(&error))?;
                let pdf = render::paint_pdf(&plan.output).map_err(|error| failure(&error))?;
                (pdf, plan.warnings)
            }
        };
        self.record(warnings)?;
        if format != Format::Pdf {
            self.verify(format, &bytes)?;
        }
        Ok(bytes)
    }
    fn verify(&self, format: Format, bytes: &[u8]) -> Result<(), String> {
        let name = format!("{}.{}", self.sample.name, format.extension());
        let parts = package::parts(bytes).map_err(|error| format!("{name}: {error}"))?;
        for part in parts
            .iter()
            .filter(|part| part.name.ends_with(".xml") || part.name.ends_with(".rels"))
        {
            let text = std::str::from_utf8(&part.bytes)
                .map_err(|error| format!("{name}:{}: {error}", part.name))?;
            roxmltree::Document::parse(text)
                .map_err(|error| format!("{name}:{}: {error}", part.name))?;
        }
        if format == Format::Pptx {
            let slides = parts
                .iter()
                .filter(|part| {
                    part.name.starts_with("ppt/slides/slide") && part.name.ends_with(".xml")
                })
                .count();
            if slides != self.sample.slides() {
                return Err(format!(
                    "expected {} slides in {name}, found {slides}",
                    self.sample.slides()
                ));
            }
        }
        let failures = validate::validate_package(bytes, &name)?;
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures
                .iter()
                .map(|failure| format!("{}: {}", failure.location, failure.error))
                .collect::<Vec<_>>()
                .join("\n"))
        }
    }
    fn previews(&mut self, layout: LayoutMode, pages: usize) -> Result<Vec<Vec<u8>>, String> {
        let failure = |error: &dyn std::fmt::Display| format!("{} png: {error}", self.sample.name);
        let plan = render::plan_with_layout(&self.document, self.resolver, None, layout)
            .map_err(|error| failure(&error))?;
        let raster = render::paint_png(
            &plan.output,
            &PngOptions {
                dots_per_inch: 144,
                ..Default::default()
            },
        )
        .map_err(|error| failure(&error))?;
        self.record(plan.warnings.into_iter().chain(raster.warnings))?;
        if raster.output.len() != pages {
            return Err(format!(
                "expected {pages} previews for {}, found {}",
                self.sample.name,
                raster.output.len()
            ));
        }
        Ok(raster.output.into_iter().map(|page| page.bytes).collect())
    }
}

pub(crate) fn run(mut arguments: Arguments) -> Outcome {
    match arguments.next().as_deref() {
        None => {}
        Some("--help" | "-h") => {
            print!("{HELP}");
            return Ok(ExitCode::SUCCESS);
        }
        Some(option) => return Err(unknown(option)),
    }
    let repository = repository();
    let sources = repository.join("examples/readme");
    let downloads = repository.join("docs/examples/readme");
    let images = repository.join("docs/images/readme");
    let files = FileAssetResolver::new(&sources).map_err(|error| error.to_string())?;
    let resolver = CachingAssetResolver::new(&files);
    let mut publications: Vec<(PathBuf, Vec<u8>)> = Vec::new();
    let mut warnings = BTreeSet::new();
    for sample in &SAMPLES {
        let source = sample.source(&sources)?;
        let document = mote::compile_with_options(
            &source,
            EvaluationOptions {
                asset_resolver: Some(&resolver),
                enable_lua: sample.name == "lua",
                ..Default::default()
            },
        )
        .map_err(|error| {
            let (line, column) = error.location(&source);
            format!("{}:{line}:{column}: {}", sample.name, error.message)
        })?;
        let mut generation = Generation {
            sample,
            warnings: BTreeSet::new(),
            resolver: &resolver,
            source,
            document,
        };
        generation.evaluation_warnings()?;
        for format in sample.formats {
            let name = format!("{}.{}", sample.name, format.extension());
            publications.push((downloads.join(name), generation.package(*format)?));
        }
        for (index, (layout, pages)) in sample.previews.iter().enumerate() {
            let suffix = if index == 0 { "" } else { "-slides" };
            for (number, page) in generation
                .previews(*layout, *pages)?
                .into_iter()
                .enumerate()
            {
                let name = format!("{}{suffix}-{}.png", sample.name, number + 1);
                publications.push((images.join(name), page));
            }
        }
        let formats: Vec<_> = sample
            .formats
            .iter()
            .map(|format| format.extension().to_uppercase())
            .collect();
        println!(
            "{}: generated {} and native previews",
            sample.name,
            formats.join(", ")
        );
        warnings.append(&mut generation.warnings);
    }
    for (path, bytes) in publications {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        fs::write(&path, bytes).map_err(|error| format!("{}: {error}", path.display()))?;
    }
    for warning in warnings {
        println!("{warning}");
    }
    println!("Inspect all native previews; Office visual review is separate.");
    Ok(ExitCode::SUCCESS)
}
