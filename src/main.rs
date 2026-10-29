use std::{
    env,
    ffi::OsString,
    io::{self, Read, Write},
    process::ExitCode,
};

use mote::source::{AssetRequest, FileAssetResolver, SourceFile, load_source};

const HELP: &str = "Usage: mote [--format json|docx] [-o OUTPUT] [SOURCE|-]\n\nEvaluate Mote source and generate JSON inspection or a Word document.\nRead UTF-8 source from SOURCE, or standard input when omitted or '-'.\nDOCX requires an output path. Unsupported DOCX features produce an error.\nNative layout, PDF, PNG, and PowerPoint output are not implemented.\n";

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
    while let Some(argument) = arguments.next() {
        match argument.to_str().filter(|_| !literal_arguments) {
            Some("--help" | "-h") => return write_stdout(HELP),
            Some("--") => literal_arguments = true,
            Some("--format" | "-format") => {
                format = arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .filter(|value| matches!(value.as_str(), "json" | "docx"))
                    .ok_or("--format requires 'json' or 'docx'")?;
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
    if format == "docx" && output_path.is_none() {
        return Err("DOCX output requires --output or -o".into());
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
    let source = match input {
        Some(path) if path != "-" => {
            let resolver = FileAssetResolver::from_current_directory()
                .map_err(|error| format!("{}: {error}", path.to_string_lossy()))?;
            load_source(
                &resolver,
                AssetRequest {
                    reference: &path,
                    source_name: None,
                },
            )
            .map_err(|error| error.to_string())?
        }
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
    let document = mote::compile(&source.text).map_err(|error| {
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
    let bytes = if format == "docx" {
        mote::docx::emit(&document).map_err(|error| format!("{name}: {error}"))?
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
