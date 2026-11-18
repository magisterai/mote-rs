use std::{
    env,
    ffi::OsString,
    io::{self, Read, Write},
    process::ExitCode,
};

use mote::source::{
    AssetRequest, CachingAssetResolver, FileAssetResolver, SourceFile, load_source,
};

const HELP: &str = "Usage: mote [--format json|docx|pptx|pdf|png] [-o OUTPUT] [--dpi 72] [--time SECONDS] [--frames RATE [--from SECONDS] [--to SECONDS]] [--today DATE] [--import-path DIRECTORY]... [SOURCE|-]\n\nEvaluate Mote source and generate JSON, DOCX, PPTX, PDF or PNG pages.\nRead UTF-8 source from SOURCE, or standard input when omitted or '-'.\nDOCX, PPTX, PDF and PNG require an output path.\nPNG writes PREFIX-1.png, PREFIX-2.png, ... at the requested DPI (default 72).\n--time SECONDS renders at a time: the binding `time` holds it, and the effects of slide objects are drawn as they stand then.\n--frames RATE writes PNG frames PREFIX-0001.png, PREFIX-0002.png, ... at RATE frames a second, from --from (default 0) to --to (default the end of the animation).\n--today DATE sets the date that the binding `today` holds (default the local date).\n--import-path DIRECTORY adds a directory searched for an import that is not found beside its source; repeatable.\n--layout document|presentation selects native layout explicitly.\nWithout --layout, an explicit presentation declaration selects slide layout.\n--lua enables Lua expressions (disabled by default).\n--lua-timeout DURATION sets the shared budget (default 10s; negative disables).\nUnsupported native structure produces an error. PPTX currently supports explicit slides with positioned text.\n";

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
    let mut layout = mote::render::LayoutMode::Automatic;
    let mut enable_lua = false;
    let mut lua_timeout = Some(std::time::Duration::from_secs(10));
    let mut today = None;
    let mut import_paths: Vec<String> = Vec::new();
    let mut time = None;
    let mut frame_rate = None;
    let mut frames_from = None;
    let mut frames_to = None;
    while let Some(argument) = arguments.next() {
        match argument.to_str().filter(|_| !literal_arguments) {
            Some(flag @ ("--time" | "--frames" | "--from" | "--to")) => {
                let seconds = arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .and_then(|value| value.parse::<f64>().ok())
                    .filter(|value| value.is_finite() && *value >= 0.0)
                    .ok_or_else(|| format!("{flag} requires a number, not negative"))?;
                match flag {
                    "--time" => time = Some(seconds),
                    "--frames" if seconds > 0.0 => frame_rate = Some(seconds),
                    "--frames" => return Err("--frames requires a positive rate".into()),
                    "--from" => frames_from = Some(seconds),
                    _ => frames_to = Some(seconds),
                }
            }
            Some("--import-path") => import_paths.push(
                arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .ok_or("--import-path requires a directory")?,
            ),
            Some("--today") => {
                today = Some(
                    arguments
                        .next()
                        .and_then(|value| value.into_string().ok())
                        .ok_or("--today requires a date (YYYY-MM-DD)")?,
                );
            }
            Some("--help" | "-h") => return write_stdout(HELP),
            Some("--") => literal_arguments = true,
            Some("--format" | "-format") => {
                format = arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .filter(|value| {
                        matches!(value.as_str(), "json" | "docx" | "pptx" | "pdf" | "png")
                    })
                    .ok_or("--format requires 'json', 'docx', 'pptx', 'pdf' or 'png'")?;
            }
            Some("--layout" | "-layout") => {
                layout = match arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .as_deref()
                {
                    Some("document") => mote::render::LayoutMode::Document,
                    Some("presentation") => mote::render::LayoutMode::Presentation,
                    _ => return Err("--layout requires 'document' or 'presentation'".into()),
                };
            }
            Some("--lua" | "-lua") => enable_lua = true,
            Some("--lua-timeout" | "-lua-timeout") => {
                let duration = arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .ok_or("--lua-timeout requires a duration")?;
                lua_timeout = parse_lua_timeout(&duration)?;
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
    if layout != mote::render::LayoutMode::Automatic && !matches!(format.as_str(), "pdf" | "png") {
        return Err("--layout applies only to PDF or PNG output".into());
    }
    if dots_per_inch.is_some() && format != "png" {
        return Err("--dpi applies only to PNG output".into());
    }
    if frame_rate.is_some() && format != "png" {
        return Err("--frames applies only to PNG output".into());
    }
    if frame_rate.is_some() && time.is_some() {
        return Err("--frames and --time cannot be given together".into());
    }
    if frame_rate.is_none() && (frames_from.is_some() || frames_to.is_some()) {
        return Err("--from and --to apply only with --frames".into());
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
    let import_path_references: Vec<&str> = import_paths.iter().map(String::as_str).collect();
    let compile = |time: Option<f64>, report: bool| -> Result<mote::model::Document, String> {
        let document = mote::compile_with_options(
            &source.text,
            mote::evaluation::EvaluationOptions {
                asset_resolver: Some(&resolver),
                source_name,
                enable_lua,
                lua_timeout,
                time,
                today: today.as_deref(),
                import_paths: &import_path_references,
            },
        )
        .map_err(|error| {
            let (line, column) = error.location(&source.text);
            let name = error
                .source
                .as_ref()
                .map_or_else(|| name.clone(), |source| source.name.to_string_lossy());
            format!("{name}:{line}:{column}: {}", error.message)
        })?;
        for warning in document.warnings.iter().filter(|_| report) {
            let (line, column) = warning.location(&source.text);
            let name = warning
                .source
                .as_ref()
                .map_or_else(|| name.clone(), |source| source.name.to_string_lossy());
            writeln!(
                io::stderr().lock(),
                "{name}:{line}:{column}: warning: {}",
                warning.message
            )
            .map_err(|error| format!("standard error: {error}"))?;
        }
        Ok(document)
    };
    if let Some(rate) = frame_rate {
        let from = frames_from.unwrap_or(0.0);
        let mut document = compile(Some(from), true)?;
        let to = match frames_to {
            Some(to) => to,
            None => mote::render::animation_end(&document),
        };
        if to <= from {
            return Err(
                "--frames requires --to, an effect or a slide duration to end the animation".into(),
            );
        }
        let count = ((to - from) * rate + 1e-9).floor() as usize + 1;
        for frame in 0..count {
            if frame > 0 {
                document = compile(Some(from + frame as f64 / rate), false)?;
            }
            let planned = mote::render::plan_with_layout(&document, &resolver, source_name, layout)
                .map_err(|error| format!("{name}: {error}"))?;
            let rendered = mote::render::paint_png(
                &planned.output,
                &mote::render::PngOptions {
                    dots_per_inch: dots_per_inch.unwrap_or(72),
                    ..Default::default()
                },
            )
            .map_err(|error| format!("{name}: {error}"))?;
            if frame == 0 {
                print_warnings(&name, &planned.warnings)?;
                print_warnings(&name, &rendered.warnings)?;
            }
            write_png_pages(
                output_path.as_ref().unwrap(),
                source_name,
                &rendered.output,
                Some(frame + 1),
            )?;
        }
        return Ok(());
    }
    let document = compile(time, true)?;
    let bytes = if matches!(format.as_str(), "pdf" | "png") {
        let planned = mote::render::plan_with_layout(&document, &resolver, source_name, layout)
            .map_err(|error| format!("{name}: {error}"))?;
        print_warnings(&name, &planned.warnings)?;
        if format == "png" {
            let rendered = mote::render::paint_png(
                &planned.output,
                &mote::render::PngOptions {
                    dots_per_inch: dots_per_inch.unwrap_or(72),
                    ..Default::default()
                },
            )
            .map_err(|error| format!("{name}: {error}"))?;
            print_warnings(&name, &rendered.warnings)?;
            return write_png_pages(
                output_path.as_ref().unwrap(),
                source_name,
                &rendered.output,
                None,
            );
        }
        mote::render::paint_pdf(&planned.output).map_err(|error| format!("{name}: {error}"))?
    } else if format == "docx" {
        let generated = mote::docx::emit_with_assets(&document, &resolver, source_name)
            .map_err(|error| format!("{name}: {error}"))?;
        print_warnings(&name, &generated.warnings)?;
        generated.bytes
    } else if format == "pptx" {
        let generated = mote::pptx::emit_with_assets(&document, &resolver, source_name)
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
    frame: Option<usize>,
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
            path.push(match frame {
                Some(frame) if pages.len() == 1 => format!("-{frame:04}.png"),
                Some(frame) => format!("-{frame:04}-{}.png", page.number),
                None => format!("-{}.png", page.number),
            });
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

fn parse_lua_timeout(value: &str) -> Result<Option<std::time::Duration>, String> {
    let (number, factor) = if let Some(number) = value.strip_suffix("ms") {
        (number, 0.001)
    } else if let Some(number) = value.strip_suffix('s') {
        (number, 1.0)
    } else if let Some(number) = value.strip_suffix('m') {
        (number, 60.0)
    } else if let Some(number) = value.strip_suffix('h') {
        (number, 3600.0)
    } else if value == "0" {
        (value, 1.0)
    } else {
        return Err("--lua-timeout requires a duration in ms, s, m or h".into());
    };
    let seconds = number
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .map(|value| value * factor)
        .ok_or("invalid --lua-timeout duration")?;
    if seconds < 0.0 {
        return Ok(None);
    }
    if seconds == 0.0 {
        return Ok(Some(std::time::Duration::from_secs(10)));
    }
    std::time::Duration::try_from_secs_f64(seconds)
        .map(Some)
        .map_err(|_| "invalid --lua-timeout duration".into())
}
