use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

fn temporary_input_path() -> PathBuf {
    static NEXT_INPUT_IDENTIFIER: AtomicUsize = AtomicUsize::new(0);
    std::env::temp_dir().join(format!(
        "mote-cli-{}-{}.mote",
        std::process::id(),
        NEXT_INPUT_IDENTIFIER.fetch_add(1, Ordering::Relaxed)
    ))
}

fn run(arguments: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mote"))
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn stdin_produces_only_valid_json() {
    let output = run(&["--format", "json", "-"], b"#heading(1)[Report]");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["content"][0]["kind"], "heading");
}

#[test]
fn invalid_source_has_a_location_and_no_partial_json() {
    let output = run(&[], "Résumé\n#heading(level=0)[Title]".as_bytes());
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("<stdin>:2:16:"), "{error}");
    assert!(error.contains("positive integer"));
}

#[test]
fn reads_source_files_and_reports_missing_paths() {
    let path = temporary_input_path();
    fs::write(&path, "Hello **world**.").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mote"))
        .arg(&path)
        .output()
        .unwrap();
    fs::remove_file(&path).unwrap();
    assert!(output.status.success());
    let missing = Command::new(env!("CARGO_BIN_EXE_mote"))
        .arg(&path)
        .output()
        .unwrap();
    assert!(!missing.status.success());
    assert!(missing.stdout.is_empty());
    assert!(String::from_utf8_lossy(&missing.stderr).contains(&*path.to_string_lossy()));
}

#[test]
fn relative_source_files_use_resolved_names_in_errors_and_warnings() {
    let path = temporary_input_path();
    let working_directory = fs::canonicalize(path.parent().unwrap()).unwrap();
    let resolved_name = working_directory.join(path.file_name().unwrap());
    let run_file = || {
        Command::new(env!("CARGO_BIN_EXE_mote"))
            .current_dir(&working_directory)
            .arg(path.file_name().unwrap())
            .output()
            .unwrap()
    };

    fs::write(&path, "é\n#font(name=7)").unwrap();
    let warning = run_file();
    assert!(warning.status.success());
    assert!(
        String::from_utf8(warning.stderr)
            .unwrap()
            .starts_with(&format!("{}:2:1: warning:", resolved_name.display()))
    );
    assert!(serde_json::from_slice::<serde_json::Value>(&warning.stdout).is_ok());

    fs::write(&path, "é\n#heading(level=0)[Title]").unwrap();
    let invalid = run_file();
    assert!(!invalid.status.success());
    assert!(invalid.stdout.is_empty());
    assert!(
        String::from_utf8(invalid.stderr)
            .unwrap()
            .starts_with(&format!("{}:2:16:", resolved_name.display()))
    );

    fs::write(&path, [0xff]).unwrap();
    let invalid = run_file();
    fs::remove_file(&path).unwrap();
    assert!(!invalid.status.success());
    assert!(invalid.stdout.is_empty());
    assert!(
        String::from_utf8(invalid.stderr)
            .unwrap()
            .starts_with(&format!("{}:", resolved_name.display()))
    );
}

#[test]
fn external_source_references_require_a_custom_resolver() {
    let output = run(&["asset://bundle/main.mote"], b"");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr).unwrap().contains(
            "asset://bundle/main.mote: external asset references require a custom resolver"
        )
    );
}

#[test]
fn invalid_options_and_invalid_utf8_fail_cleanly() {
    for arguments in [
        vec!["--format", "pdf"],
        vec!["--format"],
        vec!["--unknown"],
        vec!["one.mote", "two.mote"],
    ] {
        let output = run(&arguments, b"");
        assert!(!output.status.success(), "{arguments:?}");
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
    let output = run(&[], &[0xff]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn help_describes_the_current_output_boundary() {
    let output = run(&["--help"], b"");
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("DOCX requires an output path")
    );
}

#[test]
fn conversion_warnings_are_located_on_stderr_and_leave_json_clean() {
    let output = run(&[], "é\n#font(name=7)\n#style(name=8)".as_bytes());
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        concat!(
            "<stdin>:2:1: warning: font name was coerced from number to string \"7\"\n",
            "<stdin>:3:1: warning: style name was coerced from number to string \"8\"\n"
        )
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["fonts"][0]["name"], "7");
    assert_eq!(document["styles"][0]["name"], "8");
    assert!(document.get("warnings").is_none());
}

#[test]
fn docx_writes_a_package_and_failures_preserve_existing_output() {
    let path = temporary_input_path().with_extension("docx");
    let path_text = path.to_str().unwrap();
    let output = run(&["--format", "docx", "-o", path_text], b"Hello.");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    let bytes = fs::read(&path).unwrap();
    let archive = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    assert_eq!(archive.len(), 10);
    let invalid = run(
        &["--format", "docx", "-o", path_text],
        b"#paragraph(custom=true)[Unsupported]",
    );
    assert!(!invalid.status.success());
    assert_eq!(fs::read(&path).unwrap(), bytes);
    fs::remove_file(path).unwrap();
    assert!(!run(&["--format", "docx"], b"Hello.").status.success());
}

#[test]
fn native_outputs_write_pdf_and_numbered_png_pages() {
    let prefix = temporary_input_path().with_extension("png");
    let text = prefix.to_str().unwrap();
    let output = run(
        &["--format", "png", "--dpi", "144", "-o", text],
        b"First page.\n#break(type=\"page\")\nSecond page.",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!prefix.exists());
    for number in 1..=2 {
        let path = prefix.with_file_name(format!(
            "{}-{number}.png",
            prefix.file_stem().unwrap().to_str().unwrap()
        ));
        let bytes = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();
        let reader = png::Decoder::new(std::io::Cursor::new(bytes))
            .read_info()
            .unwrap();
        assert_eq!((reader.info().width, reader.info().height), (1224, 1584));
    }
    let pdf = prefix.with_extension("pdf");
    let output = run(
        &["--format", "pdf", "-o", pdf.to_str().unwrap()],
        b"A searchable PDF.",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(fs::read(&pdf).unwrap().starts_with(b"%PDF-1.7"));
    fs::write(&pdf, b"previous output").unwrap();
    let failed = run(
        &["--format", "pdf", "-o", pdf.to_str().unwrap()],
        b"#document(page={width=0,height=0})\nInvalid.",
    );
    assert!(!failed.status.success());
    assert_eq!(fs::read(&pdf).unwrap(), b"previous output");
    fs::remove_file(pdf).unwrap();
}

#[test]
fn native_options_and_generated_source_collision_are_rejected() {
    for arguments in [
        vec!["--format", "png"],
        vec!["--dpi", "144"],
        vec!["--format", "png", "--dpi", "0"],
        vec!["--format", "pdf", "--dpi", "144"],
    ] {
        assert!(!run(&arguments, b"Text").status.success());
    }
    let prefix = temporary_input_path();
    let mut source = prefix.as_os_str().to_owned();
    source.push("-1.png");
    fs::write(&source, b"Do not overwrite this source.").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mote"))
        .args(["--format", "png", "-o"])
        .arg(&prefix)
        .arg(&source)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(fs::read(&source).unwrap(), b"Do not overwrite this source.");
    fs::remove_file(&source).unwrap();
}
