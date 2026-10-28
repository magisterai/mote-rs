use mote::{compile, json::to_json};
use serde_json::{Value, json};

fn content_model(source: &str) -> Value {
    serde_json::from_str(&to_json(&compile(source).unwrap()).unwrap()).unwrap()
}

#[test]
fn emphasis_and_strong_forms_preserve_nested_inline_structure() {
    for (markdown, directives) in [
        (
            "Read _this_ carefully.",
            "#paragraph[Read #emphasis[this] carefully.]",
        ),
        ("**_Both_**", "#paragraph[#strong[#emphasis[Both]]]"),
        ("_**Both**_", "#paragraph[#emphasis[#strong[Both]]]"),
        (
            "__bold__ and *bold*",
            "#paragraph[#strong[bold] and #strong[bold]]",
        ),
        (r"\_literal\_", "#paragraph[\\_literal\\_]"),
    ] {
        assert_eq!(compile(markdown).unwrap(), compile(directives).unwrap());
    }
    assert_eq!(
        content_model("#emphasis[]")["content"][0]["children"][0],
        json!({"kind":"emphasis","category":"inline","children":[]})
    );
}

#[test]
fn code_spans_preserve_literal_content_and_trim_only_one_padding_space() {
    for (source, expected) in [
        (r"`#strong[text] \_literal\_`", r"#strong[text] \_literal\_"),
        ("``a ` b``", "a ` b"),
        ("`  padded  `", " padded "),
        ("`   `", "   "),
        ("` first\nsecond `", "first\nsecond"),
        ("`first\r\nsecond`", "first\r\nsecond"),
        ("#paragraph[`]` tail]", "]"),
    ] {
        let model = content_model(source);
        assert_eq!(
            model["content"][0]["children"][0],
            json!({
                "kind":"code","category":"inline","children":[{"kind":"text","text":expected}]
            }),
            "{source}"
        );
    }
    assert_eq!(
        content_model("unmatched `text")["content"][0]["children"][0]["text"],
        "unmatched `text"
    );
}

#[test]
fn code_arguments_are_literal_while_bodies_evaluate_inline_content() {
    assert_eq!(
        compile("`#strong[text]`").unwrap(),
        compile("#code(\"#strong[text]\")").unwrap()
    );
    let body = content_model("#code[ #strong[text] ]");
    assert_eq!(
        body["content"][0]["children"][0]["children"],
        json!([
            {"kind":"text","text":" "},
            {"kind":"strong","category":"inline","children":[{"kind":"text","text":"text"}]},
            {"kind":"text","text":" "}
        ])
    );
    for source in ["#code()", "#code[]"] {
        assert_eq!(
            content_model(source)["content"][0]["children"][0]["children"],
            json!([])
        );
    }
    assert_eq!(
        compile("#code(\"chosen\")[unused]").unwrap(),
        compile("#code(\"chosen\")").unwrap()
    );
    assert_eq!(
        compile("#code(12)").unwrap(),
        compile("#code(\"12\")").unwrap()
    );
    assert_eq!(
        compile("#code(false)").unwrap(),
        compile("#code(\"false\")").unwrap()
    );
    assert_eq!(
        compile("#code[first\nsecond]").unwrap(),
        compile("#code(\"first second\")").unwrap()
    );
}

#[test]
fn hard_breaks_stay_within_paragraphs_and_explicit_breaks_keep_their_element() {
    for ending in ["\n", "\r\n", "\r"] {
        for source in [
            format!("first  {ending}second"),
            format!("first\\{ending}second"),
        ] {
            assert_eq!(
                content_model(&source),
                json!({"content":[{
                    "kind":"paragraph","category":"block","children":[{"kind":"text","text":"first\nsecond"}]
                }]})
            );
        }
    }
    assert_eq!(
        content_model("#paragraph[first#line_break()second]")["content"][0]["children"],
        json!([
            {"kind":"text","text":"first"},
            {"kind":"line_break","category":"inline","children":[]},
            {"kind":"text","text":"second"}
        ])
    );
}

#[test]
fn new_content_contracts_reject_invalid_arguments_and_children() {
    for (source, expected_message) in [
        ("#emphasis()", "requires one content body"),
        ("#emphasis(1)[Text]", "unexpected positional"),
        (
            "#emphasis[#paragraph[Block]]",
            "inline content cannot contain",
        ),
        ("#code(nil)", "string, number, or boolean"),
        ("#code(1,2)", "at most one positional"),
        ("#code[a][b]", "at most one content body"),
        (
            "#code(\"literal\")[#paragraph[Block]]",
            "inline content cannot contain",
        ),
        ("#line_break()[Text]", "no arguments or content body"),
        ("#line_break(1)", "no arguments or content body"),
    ] {
        let error = compile(source).expect_err(source);
        assert!(
            error.message.contains(expected_message),
            "{source}: {error}"
        );
    }
}
