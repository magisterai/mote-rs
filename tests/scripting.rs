use mote::{evaluation::EvaluationOptions, model::Document};
use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/scripting")
}
fn compile(source: &str) -> Result<Document, mote::diagnostics::Diagnostic> {
    mote::compile_with_options(
        source,
        EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
}

#[test]
fn scripting_is_opt_in_and_retains_source_ranges() {
    let source = "#let(total=2+3)#paragraph[#total]";
    assert!(mote::compile(source).is_err());
    assert_eq!(
        compile(source).unwrap(),
        mote::compile("#paragraph[5]").unwrap()
    );
    let tree = mote::syntax::parse_with_lua(source).unwrap();
    assert!(
        mote::evaluation::evaluate(&tree)
            .unwrap_err()
            .message
            .contains("disabled")
    );
    let invalid = "Résumé\n#let(value=1+nil)";
    let error = compile(invalid).unwrap_err();
    assert_eq!(error.location(invalid).0, 2);
    assert_eq!(&invalid[error.span.start..error.span.end], "1+nil");
}

#[test]
fn only_the_supported_lua_environment_is_exposed() {
    for expression in [
        "print('x')",
        "collectgarbage()",
        "dofile('x')",
        "loadfile('x')",
        "loadstring('return 1')",
        "require('os')",
        "getfenv()",
        "setfenv(1,{})",
        "io.open('x')",
        "os.execute('x')",
        "debug.getinfo(1)",
        "string.upper('x')",
        "table.insert({},1)",
    ] {
        assert!(
            compile(&format!("#let(value={expression})")).is_err(),
            "{expression}"
        );
    }
    assert_eq!(
        compile("#let(value=tostring(type(tonumber('12'))))#value").unwrap(),
        mote::compile("number").unwrap()
    );
    for expression in [
        "1/0",
        "0/0",
        "(function() local value={}; value.self=value; return value end)()",
    ] {
        assert!(
            compile(&format!("#let(value={expression})")).is_err(),
            "{expression}"
        );
    }
}

#[test]
fn one_budget_covers_all_lua_expressions_and_cannot_be_swallowed() {
    let options = EvaluationOptions {
        enable_lua: true,
        lua_timeout: Some(Duration::from_millis(15)),
        ..Default::default()
    };
    let start = Instant::now();
    let error =
        mote::compile_with_options("#let(value=(function() while true do end end)())", options)
            .unwrap_err();
    assert!(error.message.contains("time budget exceeded"));
    assert!(start.elapsed() < Duration::from_secs(2));
    let source = "#let(value=(function() return pcall(function() while true do end end) end)())";
    assert!(
        mote::compile_with_options(source, options)
            .unwrap_err()
            .message
            .contains("time budget exceeded")
    );
    let resolver = |_: mote::source::AssetRequest<'_>| {
        std::thread::sleep(Duration::from_millis(60));
        Ok(mote::source::ResolvedAsset {
            name: "slow.json".into(),
            bytes: b"1".to_vec(),
        })
    };
    let options = EvaluationOptions {
        asset_resolver: Some(&resolver),
        lua_timeout: Some(Duration::from_millis(100)),
        ..options
    };
    assert!(
        mote::compile_with_options(
            "#let(first=json('first.json'), second=json('second.json'))",
            options
        )
        .unwrap_err()
        .message
        .contains("time budget exceeded")
    );
}

#[test]
fn lua_cli_accepts_explicit_enablement_and_duration_controls() {
    use std::process::Command;
    for arguments in [
        vec!["--lua"],
        vec!["-lua", "-lua-timeout", "-1s"],
        vec!["--lua", "--lua-timeout", "0"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_mote"))
            .args(arguments)
            .arg(fixtures().join("expressions.mote"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let disabled = Command::new(env!("CARGO_BIN_EXE_mote"))
        .arg(fixtures().join("expressions.mote"))
        .output()
        .unwrap();
    assert!(!disabled.status.success());
}

#[test]
fn scripting_capture_inputs_remain_frozen() {
    use sha2::{Digest, Sha256};
    for entry in fs::read_dir(fixtures().join("captured")).unwrap() {
        let folder = entry.unwrap().path();
        let case = folder.file_name().unwrap().to_str().unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!(
                "{:x}",
                Sha256::digest(fs::read(fixtures().join(format!("{case}.mote"))).unwrap())
            ),
            manifest["source_sha256"]
        );
        for (path, expected) in manifest["assets"].as_object().unwrap() {
            assert_eq!(
                format!(
                    "{:x}",
                    Sha256::digest(fs::read(fixtures().join(path)).unwrap())
                ),
                expected.as_str().unwrap()
            );
        }
    }
}
