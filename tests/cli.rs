use std::{
    fs,
    io::Write,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

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
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!(
        "mote-cli-{}-{}.mote",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
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
            .contains("Only JSON inspection")
    );
}
