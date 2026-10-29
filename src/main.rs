use std::{
    env,
    ffi::OsString,
    io::{self, Read, Write},
    process::ExitCode,
};

use mote::source::{
    AssetRequest, CachingAssetResolver, FileAssetResolver, SourceFile, load_source,
};

const HELP: &str = "Usage: mote [--format json|docx|pdf|png] [-o OUTPUT] [--dpi 72] [SOURCE|-]\n\nEvaluate Mote source and generate JSON, DOCX, PDF or PNG pages.\nRead UTF-8 source from SOURCE, or standard input when omitted or '-'.\nDOCX requires an output path. PDF and PNG also require an output path.\nPNG writes PREFIX-1.png, PREFIX-2.png, ... at the requested DPI (default 72).\nUnsupported native structure produces an error. PowerPoint output is not implemented.\n";

fn main() -> ExitCode {
    match run_command() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            let _ = writeln!(io::stderr().lock(), "{message}");
            ExitCode::FAILURE
        }
    }
}

fn run_command() -> Result<(), String> {
    let mut arguments = env::args_os().skip(1);
    let mut input: Option<OsString> = None;
    let mut literal_arguments = false;
    let mut format = String::from("json");
    let mut output_path: Option<OsString> = None;
    let mut dots_per_inch = None;
    while let Some(argument) = arguments.next() {
        match argument.to_str().filter(|_| !literal_arguments) {
            Some("--help" | "-h") => return write_stdout(HELP),
            Some("--") => literal_arguments = true,
            Some("--format" | "-format") => {
                format = arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .filter(|value| matches!(value.as_str(), "json" | "docx" | "pdf" | "png"))
                    .ok_or("--format requires 'json', 'docx', 'pdf' or 'png'")?;
            }
            Some("--dpi" | "-dpi") => {
                dots_per_inch = Some(
                    arguments
                        .next()
                        .and_then(|value| value.into_string().ok())
                        .and_then(|value| value.parse::<u32>().ok())
                        .filter(|value| (1..=9600).contains(value))
                        .ok_or("--dpi requires an integer from 1 to 9600")?,
                );
            }
            Some("--output" | "-o") => {
                output_path = Some(arguments.next().ok_or("--output requires a path")?);
            }
            Some(flag) if flag.starts_with('-') && flag != "-" => {
                return Err(format!("unknown option: {flag}"));
            }
            _ => {
                if input.replace(argument).is_some() {
                    return Err(
                        "provide one source file; multiple inputs are not supported yet".into(),
                    );
                }
            }
        }
    }
    if format != "json" && output_path.is_none() {
        return Err(format!(
            "{} output requires --output or -o",
            format.to_uppercase()
        ));
    }
    if dots_per_inch.is_some() && format != "png" {
        return Err("--dpi applies only to PNG output".into());
    }
    if let (Some(input_path), Some(output_path)) = (&input, &output_path)
        && input_path != "-"
        && let (Ok(input_path), Ok(output_path)) = (
            std::fs::canonicalize(input_path),
            std::fs::canonicalize(output_path),
        )
        && input_path == output_path
    {
        return Err("source and output paths must be different".into());
    }
    let files = FileAssetResolver::from_current_directory().map_err(|error| error.to_string())?;
    let resolver = CachingAssetResolver::new(&files);
    let source = match input {
        Some(path) if path != "-" => load_source(
            &resolver,
            AssetRequest {
                reference: &path,
                source_name: None,
            },
        )
        .map_err(|error| error.to_string())?,
        _ => {
            let mut source = String::new();
            io::stdin()
                .read_to_string(&mut source)
                .map_err(|error| format!("<stdin>: {error}"))?;
            SourceFile {
                name: "<stdin>".into(),
                text: source,
            }
        }
    };
    let name = source.name.to_string_lossy();
    let source_name = (source.name != "<stdin>").then_some(source.name.as_os_str());
    let document =
        mote::compile_with_assets(&source.text, &resolver, source_name).map_err(|error| {
            let (line, column) = error.location(&source.text);
            format!("{name}:{line}:{column}: {}", error.message)
        })?;
    for warning in &document.warnings {
        let (line, column) = warning.location(&source.text);
        writeln!(
            io::stderr().lock(),
            "{name}:{line}:{column}: warning: {}",
            warning.message
        )
        .map_err(|error| format!("standard error: {error}"))?;
    }
    let bytes = if matches!(format.as_str(), "pdf" | "png") {
        if format == "png" {
            let rendered = mote::render::emit_png(
                &document,
                &resolver,
                source_name,
                &mote::render::PngOptions {
                    dots_per_inch: dots_per_inch.unwrap_or(72),
                    ..Default::default()
                },
            )
            .map_err(|error| format!("{name}: {error}"))?;
            print_warnings(&name, &rendered.warnings)?;
            return write_png_pages(output_path.as_ref().unwrap(), source_name, &rendered.output);
        }
        let rendered = mote::render::emit_pdf(&document, &resolver, source_name)
            .map_err(|error| format!("{name}: {error}"))?;
        print_warnings(&name, &rendered.warnings)?;
        rendered.output
    } else if format == "docx" {
        let generated = mote::docx::emit_with_assets(&document, &resolver, source_name)
            .map_err(|error| format!("{name}: {error}"))?;
        print_warnings(&name, &generated.warnings)?;
        generated.bytes
    } else {
        let mut json =
            mote::json::to_json(&document).map_err(|error| format!("JSON output: {error}"))?;
        json.push('\n');
        json.into_bytes()
    };
    if let Some(path) = output_path {
        std::fs::write(&path, bytes).map_err(|error| format!("{}: {error}", path.to_string_lossy()))
    } else {
        write_stdout_bytes(&bytes)
    }
}

fn print_warnings(name: &str, warnings: &[String]) -> Result<(), String> {
    for warning in warnings {
        writeln!(io::stderr().lock(), "{name}: warning: {warning}")
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn write_png_pages(
    output: &std::ffi::OsStr,
    source: Option<&std::ffi::OsStr>,
    pages: &[mote::render::RasterPage],
) -> Result<(), String> {
    let mut prefix = std::path::PathBuf::from(output);
    if prefix.is_dir()
        || output
            .to_string_lossy()
            .ends_with(std::path::MAIN_SEPARATOR)
    {
        prefix.push("page");
    }
    if prefix
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("png"))
    {
        prefix.set_extension("");
    }
    let paths: Vec<_> = pages
        .iter()
        .map(|page| {
            let mut path = prefix.as_os_str().to_os_string();
            path.push(format!("-{}.png", page.number));
            std::path::PathBuf::from(path)
        })
        .collect();
    if let Some(source) = source.and_then(|path| std::fs::canonicalize(path).ok()) {
        for path in &paths {
            if std::fs::canonicalize(path).is_ok_and(|path| path == source) {
                return Err("source and output paths must be different".into());
            }
        }
    }
    for (page, path) in pages.iter().zip(paths) {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        std::fs::write(&path, &page.bytes)
            .map_err(|error| format!("{}: {error}", path.display()))?;
    }
    Ok(())
}

fn write_stdout(text: &str) -> Result<(), String> {
    write_stdout_bytes(text.as_bytes())
}

fn write_stdout_bytes(bytes: &[u8]) -> Result<(), String> {
    match io::stdout().lock().write_all(bytes) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(format!("standard output: {error}")),
    }
}
