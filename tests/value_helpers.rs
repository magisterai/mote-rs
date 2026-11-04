#[path = "support/language_capture.rs"]
mod language_capture;

fn model(source: &str, lua: bool) -> serde_json::Value {
    let document = mote::compile_with_options(
        source,
        mote::evaluation::EvaluationOptions {
            enable_lua: lua,
            ..Default::default()
        },
    )
    .unwrap();
    serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap()
}

#[test]
fn helpers_preserve_static_and_lua_models_packages_and_pages() {
    language_capture::compare("value-helpers", &["static", "lua"]);
}

#[test]
fn simple_casing_matches_every_captured_cased_scalar() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/value-helpers/unicode-casing.json")).unwrap();
    for helper in ["upper", "lower"] {
        let source = format!(
            "#{helper}({})",
            serde_json::to_string(fixture["source"].as_str().unwrap()).unwrap()
        );
        let expected = fixture[helper].as_str().unwrap();
        let actual = model(&source, false);
        assert_eq!(
            actual["sections"][0]["content"][0]["children"][0]["text"],
            expected
        );
    }
}

#[test]
fn named_currency_in_document_namespace_is_distinct_from_global_arguments() {
    let actual = model(
        "#function money()\nreturn document.format_money({currency=\"eur\"},1234.5)\nend\n#money()",
        true,
    );
    let expected = model(r#"#format_money(1234.5,currency="EUR")"#, false);
    assert_eq!(actual, expected);
    assert!(
        mote::compile_with_options(
            "#function money()\nreturn format_money({currency=\"eur\"},1234.5)\nend\n#money()",
            mote::evaluation::EvaluationOptions {
                enable_lua: true,
                ..Default::default()
            }
        )
        .is_err()
    );
}

fn text(source: &str, lua: bool) -> String {
    model(source, lua)["sections"][0]["content"][0]["children"][0]["text"]
        .as_str()
        .unwrap()
        .to_owned()
}

#[test]
fn format_number_rounds_to_its_decimals_and_groups_only_on_request() {
    for (source, expected) in [
        ("#format_number(171.456, 1)", "171.5"),
        ("#format_number(2024)", "2024"),
        ("#format_number(2.5)", "2"),
        ("#format_number(1.005, 2)", "1.00"),
        ("#format_number(-0.004, 2)", "0.00"),
        (
            "#format_number(-1234567.891, 2, grouping=true)",
            "-1,234,567.89",
        ),
        (r#"#format_number("12", decimals=3)"#, "12.000"),
        ("#format_number(999.5, grouping=true)", "1,000"),
    ] {
        assert_eq!(text(source, false), expected, "{source}");
    }
    assert_eq!(
        text(
            "#function number()\nreturn document.format_number({grouping=true},9876.5,1)\nend\n#number()",
            true
        ),
        "9,876.5"
    );
    assert_eq!(
        text(
            "#function number()\nreturn format_number(0.125,2)\nend\n#number()",
            true
        ),
        "0.12"
    );
}

#[test]
fn helpers_reject_duplicate_arguments_bad_types_and_invalid_dates() {
    for source in [
        r#"#upper()"#,
        r#"#lower("a","b")"#,
        r#"#format_money(1,"USD",currency="EUR")"#,
        r#"#format_money(true)"#,
        r#"#format_money("NaN")"#,
        r#"#format_number()"#,
        r#"#format_number(1,2,decimals=2)"#,
        r#"#format_number(1,2.5)"#,
        r#"#format_number(1,16)"#,
        r#"#format_number(1,-1)"#,
        r#"#format_number(1,"2")"#,
        r#"#format_number(1,grouping="yes")"#,
        r#"#format_number(1,currency="EUR")"#,
        r#"#format_number("NaN")"#,
        r#"#length(1,"inch")"#,
        r#"#length(" 1 ")"#,
        r#"#date("2023-02-29")"#,
        r#"#date("1900-02-29")"#,
        r#"#date("2024-13-01")"#,
        r#"#date("2024-01-01T00:00:00Z")"#,
        r#"#date(2024)"#,
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    assert!(mote::compile(r#"#date("2000-02-29")"#).is_ok());
}
