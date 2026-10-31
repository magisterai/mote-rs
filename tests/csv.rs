#[path = "support/language_capture.rs"]
mod language_capture;
use mote::{
    evaluation::EvaluationOptions,
    source::{AssetRequest, ResolvedAsset},
};

fn loaded_text(
    csv: &str,
    heading: &str,
    columns: &[&str],
) -> Result<String, mote::diagnostics::Diagnostic> {
    let resolver = |_request: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "table.csv".into(),
            bytes: csv.as_bytes().to_vec(),
        })
    };
    let fields = columns
        .iter()
        .map(|name| format!("#row.{name}"))
        .collect::<Vec<_>>()
        .join("|");
    let source = format!(
        "#let(rows=csv(\"table.csv\"{heading}))\n#for row in rows {{\n#paragraph[{fields}]\n}}"
    );
    let document = mote::compile_with_assets(&source, &resolver, None)?;
    Ok(document
        .content
        .iter()
        .map(|content| content.to_text())
        .collect::<Vec<_>>()
        .join("\n"))
}
#[test]
fn csv_data_and_imported_loaders_match_captured_outputs() {
    language_capture::compare("csv", &["document", "slides"]);
}
#[test]
fn records_preserve_strings_quoted_content_and_uneven_widths() {
    assert_eq!(
        loaded_text(
            "Name,Value\r\n\"a,b\",\"two\r\nlines\"\r\n\"a\"\"b\", 12 \nShort\nWide,9,ignored\n",
            "",
            &["Name", "Value"]
        )
        .unwrap(),
        "a,b|two\nlines\na\"b| 12 \nShort|\nWide|9"
    );
    assert_eq!(
        loaded_text(
            "Report\n Name , ,Name\nAda,1,Second",
            ",heading=true",
            &["Name", "column_2", "Name_2"]
        )
        .unwrap(),
        "Ada|1|Second"
    );
    assert_eq!(
        loaded_text("Name,Name,Name_2\na,b,c", "", &["Name", "Name_2"]).unwrap(),
        "a|c"
    );
    assert_eq!(
        loaded_text("Value\n\n  \n\"\"\nlast\r", "", &["Value"]).unwrap(),
        "  \n\nlast"
    );
    for input in ["", "\n\n", "Name", "Name\r\n"] {
        assert_eq!(loaded_text(input, "", &["Name"]).unwrap(), "");
    }
    assert_eq!(
        loaded_text("Title", ",heading=\"TRUE\"", &["Name"]).unwrap(),
        ""
    );
}
#[test]
fn malformed_quotes_and_invalid_arguments_are_diagnostics() {
    for data in [
        "Name\n\"open",
        "Name\nnot\"quoted",
        "Name\n\"closed\"extra",
        "Name\n\"closed\" ,x",
    ] {
        let error = loaded_text(data, "", &["Name"]).unwrap_err();
        assert!(error.message.contains("invalid CSV"), "{error}");
        assert!(error.message.contains("line 2"), "{error}");
    }
    for source in [
        "#csv()",
        "#csv(12)",
        "#csv(\"x\",heading=12)",
        "#csv(\"x\",heading=\"yes\")",
        "#csv(\"x\",delimiter=\";\")",
        "#csv(\"x\")[body]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let source = "#let(rows=csv({heading=true},\"x.csv\"))";
    assert!(
        mote::compile_with_options(
            source,
            EvaluationOptions {
                enable_lua: true,
                ..Default::default()
            }
        )
        .is_err()
    );
}
#[test]
fn csv_requests_share_the_compilation_snapshot() {
    let calls = std::cell::Cell::new(0);
    let resolver = |_request: AssetRequest<'_>| {
        calls.set(calls.get() + 1);
        Ok(ResolvedAsset {
            name: "table.csv".into(),
            bytes: b"Name\nAda".to_vec(),
        })
    };
    let document = mote::compile_with_assets(
        "#let(first=csv(\"table.csv\"),second=csv(\"table.csv\"))\n#for row in first { #paragraph[#row.Name] }\n#for row in second { #paragraph[#row.Name] }",
        &resolver,
        None,
    )
    .unwrap();
    assert_eq!(
        document
            .content
            .iter()
            .map(|content| content.to_text())
            .collect::<Vec<_>>(),
        ["Ada", "Ada"]
    );
    assert_eq!(calls.get(), 1);
}
