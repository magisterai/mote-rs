use mote::{
    compile,
    json::to_json,
    syntax::{NodeKind, parse},
};
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
        json!({"kind":"emphasis","category":"inline"})
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
            {"kind":"strong","category":"inline","children":[{"kind":"text","text":"text"}]},
            {"kind":"text","text":" "}
        ])
    );
    for source in ["#code()", "#code[]"] {
        assert_eq!(
            content_model(source)["content"][0]["children"][0]["children"],
            Value::Null
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
        content_model("#code[first\nsecond]")["content"][0]["children"][0]["children"],
        json!([{ "kind":"text", "text":"first" }, {"kind":"text", "text":" "}, {"kind":"text", "text":"second"}])
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
                content_model(&source)["content"][0]["children"],
                json!([{ "kind":"text", "text":"first" }, {"kind":"text", "text":"\n"}, {"kind":"text", "text":"second"}])
            );
        }
    }
    assert_eq!(
        content_model("#paragraph[first#line_break()second]")["content"][0]["children"],
        json!([
            {"kind":"text","text":"first"},
            {"kind":"line_break","category":"inline"},
            {"kind":"text","text":"second"}
        ])
    );
}

#[test]
fn explicit_lists_require_items_and_items_normalize_paragraphs() {
    let model = content_model(
        "#ordered_list(start=\"3\")[\n #item[First]\n #item[Second\n\nAnother paragraph.]\n]",
    );
    let list = &model["content"][0];
    assert_eq!(list["kind"], "ordered_list");
    assert_eq!(list["attributes"], json!({"start":3.0}));
    assert_eq!(list["children"].as_array().unwrap().len(), 2);
    assert_eq!(list["children"][0]["children"][0]["kind"], "paragraph");
    assert_eq!(list["children"][1]["children"].as_array().unwrap().len(), 2);
    assert_eq!(
        content_model("#list[]")["content"][0]["children"],
        Value::Null
    );
    assert_eq!(
        content_model("#list[#item[]]")["content"][0]["children"][0]["children"],
        Value::Null
    );
}

#[test]
fn standalone_items_group_by_kind_and_paragraphs_end_a_group() {
    let model = content_model("#item[A]\n\n#item[B]\nText\n#item[C]\n#item(marker=\"3.\")[D]");
    let content = model["content"].as_array().unwrap();
    assert_eq!(
        content
            .iter()
            .map(|node| node["kind"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["list", "paragraph", "list", "ordered_list"]
    );
    assert_eq!(content[0]["children"].as_array().unwrap().len(), 2);
    assert_eq!(
        content[3]["children"][0]["attributes"],
        json!({"marker":"3."})
    );
}

#[test]
fn markdown_lists_preserve_markers_continuations_and_parent_boundaries() {
    for ending in ["\n", "\r\n", "\r"] {
        let source = [
            "- Parent",
            "  continuation",
            "  3. Child",
            "    - Deep",
            "- Last",
        ]
        .join(ending);
        let model = content_model(&source);
        let list = &model["content"][0];
        assert_eq!(list["kind"], "list");
        assert_eq!(list["children"].as_array().unwrap().len(), 2);
        let first = &list["children"][0];
        assert_eq!(first["attributes"], json!({"marker":"-"}));
        assert_eq!(&first["children"].as_array().unwrap()[..4], json!([{ "kind":"text", "text":"Parent" }, {"kind":"text", "text":" "}, {"kind":"text", "text":"continuation"}, {"kind":"text", "text":" "}]).as_array().unwrap());
        assert_eq!(first["children"][4]["kind"], "ordered_list");
        assert_eq!(
            first["children"][4]["children"][0]["attributes"],
            json!({"marker":"3."})
        );
        assert_eq!(
            first["children"][4]["children"][0]["children"][2]["kind"],
            "list"
        );
    }
    let model = content_model("- A\n\n  continued\n\n- B\n+ C\n3. D\n8. E");
    assert_eq!(model["content"][0]["children"].as_array().unwrap().len(), 3);
    assert_eq!(
        model["content"][0]["children"][0]["children"],
        json!([{ "kind":"text", "text":"A" }, {"kind":"text", "text":"\n\n"}, {"kind":"text", "text":"continued"}])
    );
    assert_eq!(
        model["content"][1]["children"][1]["attributes"]["marker"],
        "8."
    );
    assert!(model["content"][1].get("attributes").is_none());
}

#[test]
fn markdown_items_can_fill_an_explicit_list_without_an_extra_container() {
    let model = content_model("#list[\n- First\n- Second\n]");
    assert_eq!(model["content"][0]["children"].as_array().unwrap().len(), 2);
    assert_eq!(model["content"][0]["children"][0]["kind"], "list_item");
    let model = content_model("#list[#item[Parent\n#list[#item[Child]]]]");
    assert_eq!(
        model["content"][0]["children"][0]["children"][1]["kind"],
        "list"
    );
    let model = content_model("#list[#item[\n- Parent\n  - Child\n]]");
    let nested = &model["content"][0]["children"][0]["children"][0];
    assert_eq!(nested["kind"], "list");
    assert_eq!(nested["children"][0]["children"][2]["kind"], "list");
}

#[test]
fn blockquotes_have_block_bodies_and_markdown_quote_lines_remain_separate() {
    assert_eq!(
        compile("> Read _this_.").unwrap(),
        compile("#blockquote[Read #emphasis[this].]").unwrap()
    );
    assert_eq!(
        compile("> One\n> Two").unwrap(),
        compile("#blockquote[One]#blockquote[Two]").unwrap()
    );
    let model = content_model("#blockquote[First\n\nSecond\n#list[#item[Third]]]");
    assert_eq!(model["content"][0]["children"].as_array().unwrap().len(), 3);
    assert_eq!(model["content"][0]["children"][2]["kind"], "list");
    assert_eq!(
        content_model("#blockquote[]")["content"][0]["children"],
        Value::Null
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
        (
            "#line_break()[Text]",
            "no positional arguments or content body",
        ),
        ("#line_break(1)", "no positional arguments or content body"),
        ("#list()", "requires one content body"),
        ("#item()", "requires one content body"),
        ("#list[Text]", "only items"),
        ("#list[#paragraph[Text]]", "only items"),
        ("#list[#list[#item[Text]]]", "only items"),
        ("#list[\n- Parent\n  - Child\n]", "nested Markdown items"),
        ("#ordered_list(start=0)[#item[Text]]", "positive integer"),
        ("#ordered_list(start=1.5)[#item[Text]]", "positive integer"),
        ("#item(marker=1)[Text]", "marker must have string type"),
        (
            "#paragraph[#list[#item[Text]]]",
            "inline content cannot contain",
        ),
        ("#blockquote()", "requires one content body"),
        (
            "#paragraph[#blockquote[Text]]",
            "inline content cannot contain",
        ),
    ] {
        let error = compile(source).expect_err(source);
        assert!(
            error.message.contains(expected_message),
            "{source}: {error}"
        );
    }
}

#[test]
fn nested_markup_retains_unicode_source_ranges_and_bounded_recursion() {
    let source = "- Résumé\n  - _汉字_\n- Last";
    let tree = parse(source).unwrap();
    let NodeKind::ListItem { body, .. } = &tree.nodes[0].kind else {
        panic!("expected item");
    };
    let nested = &body[2];
    assert_eq!(&source[nested.span.start..nested.span.end], "- _汉字_");
    for source in [
        source,
        "#paragraph[``résumé ` text`` _ready_]",
        "> #emphasis[Quoted]",
    ] {
        for end in source.char_indices().map(|(position, _)| position) {
            let _ = parse(&source[..end]);
        }
    }
    let nested = (0..100)
        .map(|depth| format!("{}- item\n", "  ".repeat(depth)))
        .collect::<String>();
    assert!(
        parse(&nested)
            .unwrap_err()
            .message
            .contains("nesting limit")
    );
}

#[test]
fn markdown_task_markers_preserve_unchecked_and_absent_states() {
    let model = content_model(
        "- [x] Complete\n- [ ] Pending\n- [X] Also complete\n- Ordinary\n  - [ ] Nested",
    );
    let items = &model["content"][0]["children"];
    for (index, checked) in [(0, true), (1, false), (2, true)] {
        assert_eq!(items[index]["attributes"]["checked"], checked);
        assert_eq!(
            items[index]["attributes"]["marker"],
            ["- [x]", "- [ ]", "- [X]"][index]
        );
    }
    assert!(items[3]["attributes"].get("checked").is_none());
    let nested = items[3]["children"]
        .as_array()
        .unwrap()
        .iter()
        .find(|child| child["kind"] == "list")
        .unwrap();
    assert_eq!(nested["children"][0]["attributes"]["checked"], false);
    assert_eq!(items[0]["children"][0]["text"], "Complete");
    assert_eq!(
        content_model("1. [x] Ordered")["content"][0]["children"][0]["attributes"]["checked"],
        true
    );
}

#[test]
fn underlined_headings_retain_marker_precedence_and_inline_content() {
    let model = content_model("A **title**\n===\n\nSubheading\n---\n\n---\n---");
    let content = model["content"].as_array().unwrap();
    assert_eq!(content.len(), 3);
    for (index, level) in [(0, 1), (1, 2), (2, 2)] {
        assert_eq!(
            content[index]["attributes"]["level"].as_f64(),
            Some(level as f64)
        );
        assert_eq!(content[index]["attributes"]["marker"], "setext");
    }
    assert_eq!(content[0]["children"][1]["kind"], "strong");
    assert_eq!(content[2]["children"][0]["text"], "---");
}
