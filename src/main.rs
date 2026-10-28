use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Read, Write},
    process::ExitCode,
};

const HELP: &str = "Usage: mote [--format json] [SOURCE|-]\n\nEvaluate the supported Mote language subset and print its document model as JSON.\nRead UTF-8 source from SOURCE, or standard input when omitted or '-'.\nOnly JSON inspection is available; layout and Office output are not implemented.\n";

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
    while let Some(argument) = arguments.next() {
        match argument.to_str().filter(|_| !literal_arguments) {
            Some("--help" | "-h") => return write_stdout(HELP),
            Some("--") => literal_arguments = true,
            Some("--format" | "-format") => {
                if arguments.next().as_deref() != Some(std::ffi::OsStr::new("json")) {
                    return Err(
                        "--format requires 'json'; other output formats are not implemented".into(),
                    );
                }
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
    let (name, source) = match input {
        Some(path) if path != "-" => {
            let name = path.to_string_lossy().into_owned();
            let source = fs::read_to_string(&path).map_err(|error| format!("{name}: {error}"))?;
            (name, source)
        }
        _ => {
            let mut source = String::new();
            io::stdin()
                .read_to_string(&mut source)
                .map_err(|error| format!("<stdin>: {error}"))?;
            ("<stdin>".into(), source)
        }
    };
    let document = mote::compile(&source).map_err(|error| {
        let (line, column) = error.location(&source);
        format!("{name}:{line}:{column}: {}", error.message)
    })?;
    let mut json =
        mote::json::to_json(&document).map_err(|error| format!("JSON output: {error}"))?;
    for warning in &document.warnings {
        let (line, column) = warning.location(&source);
        writeln!(
            io::stderr().lock(),
            "{name}:{line}:{column}: warning: {}",
            warning.message
        )
        .map_err(|error| format!("standard error: {error}"))?;
    }
    json.push('\n');
    write_stdout(&json)
}

fn write_stdout(text: &str) -> Result<(), String> {
    match io::stdout().lock().write_all(text.as_bytes()) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(format!("standard output: {error}")),
    }
}
