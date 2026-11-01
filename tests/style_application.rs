use mote::{
    diagnostics::Diagnostic,
    evaluation::EvaluationOptions,
    model::{Content, Document, Element},
    syntax::{self, ExpressionKind, NodeKind},
};

fn compile(source: &str, enable_lua: bool) -> Result<Document, Diagnostic> {
    mote::compile_with_options(
        source,
        EvaluationOptions {
            enable_lua,
            ..Default::default()
        },
    )
}

fn compare_models(shorthand: &str, ordinary: &str) {
    for enable_lua in [false, true] {
        assert_eq!(
            mote::json::to_json(&compile(shorthand, enable_lua).unwrap()).unwrap(),
            mote::json::to_json(&compile(ordinary, enable_lua).unwrap()).unwrap(),
            "Lua enabled: {enable_lua}; source: {shorthand}"
        );
    }
}

fn find_element<'content>(content: &'content [Content], name: &str) -> Option<&'content Element> {
    content.iter().find_map(|content| match content {
        Content::Element(element) if element.kind.as_text() == name => Some(element),
        Content::Element(element) => find_element(&element.children, name),
        _ => None,
    })
}

#[test]
fn every_named_style_directive_preserves_ordinary_arguments_and_content() {
    for (directive, arguments, body) in [
        ("paragraph", "alignment = \"center\"", "[Text]"),
        ("heading", "2", "[Heading]"),
        ("span", "bold = false", "[Text]"),
        ("strong", "", "[Text]"),
        ("emphasis", "", "[Text]"),
        ("code", "\"Literal #span[text]\"", ""),
        ("raw", "\"Literal #paragraph[text]\"", ""),
        ("list", "", "[#item[First]#item[Second]]"),
        ("ordered_list", "start = 3", "[#item[First]]"),
        ("blockquote", "", "[#paragraph[Quoted]]"),
        ("term", "", "[Term]"),
        ("definition", "", "[#paragraph[Definition]]"),
        ("table", "width = pt(200)", "[#row[#cell[Value]]]"),
        ("cell", "", "[Value]"),
        ("strike", "kind = \"double\"", "[Removed]"),
        ("highlight", "fill = \"#ffff00\"", "[Highlighted]"),
        ("small_caps", "", "[Small capitals]"),
        ("subscript", "", "[2]"),
        ("superscript", "", "[2]"),
    ] {
        let (prefix, suffix) = match directive {
            "term" | "definition" => ("#definition_list[", "]"),
            "cell" => ("#table[#row[", "]]"),
            _ => ("", ""),
        };
        let properties = if arguments.is_empty() {
            String::new()
        } else {
            format!(" ({arguments})")
        };
        let ordinary_properties = if arguments.is_empty() {
            "style = 'Chosen'".to_owned()
        } else {
            format!("style = 'Chosen', {arguments}")
        };
        let shorthand = format!("{prefix}#{directive} 'Chosen'{properties}{body}{suffix}");
        let ordinary = format!("{prefix}#{directive}({ordinary_properties}){body}{suffix}");
        compare_models(&shorthand, &ordinary);
        let document = compile(&shorthand, false).unwrap();
        let element = find_element(&document.content, directive).unwrap();
        assert_eq!(element.attributes["style"].to_text(), "Chosen");
    }
}

#[test]
fn header_strings_retain_source_ranges_and_do_not_consume_heading_levels() {
    let source = "#heading \"Résumé \\\"Lead\\\"\" \t(2, alignment = \"center\")[Title]";
    for tree in [syntax::parse(source), syntax::parse_with_lua(source)] {
        let tree = tree.unwrap();
        let NodeKind::Call(call) = &tree.nodes[0].kind else {
            panic!("expected heading call")
        };
        let style = &call.arguments[0];
        assert_eq!(style.name.as_deref(), Some("style"));
        assert_eq!(
            style.value.kind,
            ExpressionKind::String("Résumé \"Lead\"".into())
        );
        assert_eq!(style.span, style.value.span);
        assert_eq!(
            &source[style.span.start..style.span.end],
            "\"Résumé \\\"Lead\\\"\""
        );
        assert_eq!(call.arguments[1].name, None);
        assert_eq!(call.arguments[2].name.as_deref(), Some("alignment"));
        assert_eq!(call.bodies.len(), 1);
        assert_eq!(
            &source[tree.nodes[0].span.start..tree.nodes[0].span.end],
            source
        );
    }
}

#[test]
fn nested_shorthand_preserves_inheritance_direct_overrides_and_sibling_boundaries() {
    let declarations = r#"#style "Body" (run = { bold = true })
#style "Lead" extends "Body" (run = { italic = true })
#style "Accent" (type = "character", run = { bold = true })"#;
    compare_models(
        &format!(
            r#"{declarations}
#paragraph "Lead" (alignment = "center")[Before #span 'Accent' (bold = false)[inside] after.]
#paragraph "Body"[Second.]"#
        ),
        &format!(
            r#"{declarations}
#paragraph(style = "Lead", alignment = "center")[Before #span(style = 'Accent', bold = false)[inside] after.]
#paragraph(style = "Body")[Second.]"#
        ),
    );
    compare_models(
        r#"#paragraph "Outer" (properties = { style = "Inner", alignment = "right" })[Text]"#,
        r#"#paragraph(style = "Outer", properties = { style = "Inner", alignment = "right" })[Text]"#,
    );
    compare_models(r#"#paragraph ""[Text]"#, r#"#paragraph(style = "")[Text]"#);
}

#[test]
fn lua_properties_and_existing_dynamic_style_arguments_keep_their_meaning() {
    let prefix = r#"#let(chosen = "Lead", options = { style = "Body" }, size = 10)"#;
    let shorthand = format!(
        r#"{prefix}#paragraph "Lead" (properties = options)[#span 'Accent' (size = pt(size + 2))[Text]]"#
    );
    let ordinary = format!(
        r#"{prefix}#paragraph(style = chosen, properties = options)[#span(style = 'Accent', size = pt(size + 2))[Text]]"#
    );
    assert_eq!(
        mote::json::to_json(&compile(&shorthand, true).unwrap()).unwrap(),
        mote::json::to_json(&compile(&ordinary, true).unwrap()).unwrap()
    );
}

#[test]
fn bare_references_keep_quoted_prose_when_no_same_line_call_delimiter_follows() {
    for content in [
        "#paragraph \"Lead\"",
        "#paragraph \"Lead\" followed by prose",
        "#paragraph \"Lead\"\n[Text]",
        "#paragraph\n\"Lead\"[Text]",
        "#paragraph \"Lead\"\r\n[Text]",
        "#paragraph \"Lead\"\r[Text]",
        "#paragraph \"Lead\ncontinued\"[Text]",
        "#paragraph \"unfinished",
        "#paragraph 'unfinished",
        "#paragraph \"unfinished\\",
    ] {
        for tree in [syntax::parse(content), syntax::parse_with_lua(content)] {
            assert!(
                matches!(&tree.unwrap().nodes[0].kind, NodeKind::Reference(name) if name == "paragraph")
            );
        }
        compare_models(
            &format!("#let(paragraph = \"Label\"){content}"),
            &format!(
                "#let(value = \"Label\"){}",
                content.replacen("#paragraph", "#value", 1)
            ),
        );
    }
}

#[test]
fn complete_shorthand_has_precedence_over_a_same_named_binding() {
    compare_models(
        r#"#let(paragraph = "Label")#paragraph "Lead"[Text]"#,
        r#"#paragraph(style = "Lead")[Text]"#,
    );
    compare_models(
        "#paragraph\t'Lead'\t[Text]",
        "#paragraph(style = 'Lead')[Text]",
    );
}

#[test]
fn pattern_fields_metadata_hints_and_custom_names_do_not_gain_style_headers() {
    for name in [
        "underline",
        "divider",
        "image",
        "figure",
        "definition_list",
        "row",
        "link",
        "shape",
        "chart",
        "box",
        "caption",
        "custom",
    ] {
        let tree = syntax::parse(&format!("#{name} \"Lead\"[Text]")).unwrap();
        assert!(matches!(&tree.nodes[0].kind, NodeKind::Reference(reference) if reference == name));
    }
    compare_models(
        r#"#let(custom = "Label")#custom "Lead"[Text]"#,
        r#"#let(value = "Label")#value "Lead"[Text]"#,
    );
    let document = compile(r#"#underline(style = "wave")[Text]"#, false).unwrap();
    assert_eq!(
        find_element(&document.content, "underline")
            .unwrap()
            .attributes["style"]
            .to_text(),
        "wave"
    );
}

#[test]
fn content_literals_do_not_apply_styles() {
    for source in [
        "`#paragraph \"Lead\"[Text]`",
        "```mote\n#paragraph \"Lead\"[Text]\n```",
        r#"#raw('#paragraph "Lead"[Text]')"#,
    ] {
        let document = compile(source, false).unwrap();
        assert!(
            find_element(&document.content, "paragraph")
                .is_none_or(|element| !element.attributes.contains_key("style"))
        );
        assert!(
            document
                .content
                .iter()
                .any(|content| content.to_text().contains("#paragraph \"Lead\"[Text]"))
        );
    }
}

#[test]
fn functions_shadowing_eligible_names_keep_ordinary_call_resolution() {
    let prefix = "#function paragraph(style, body) return style end\n";
    let shorthand = compile(&format!(r#"{prefix}#paragraph "Lead"[Text]"#), true).unwrap();
    let ordinary = compile(
        &format!(r#"{prefix}#paragraph(style = "Lead")[Text]"#),
        true,
    )
    .unwrap();
    assert_eq!(
        mote::json::to_json(&shorthand).unwrap(),
        mote::json::to_json(&ordinary).unwrap()
    );
    assert_eq!(shorthand.content[0].to_text(), "Lead");
}

#[test]
fn conflicting_style_arguments_report_the_original_argument_span() {
    let source = "Résumé\n#paragraph \"Lead\" (style = \"Body\")[Text]";
    for error in [
        syntax::parse(source).unwrap_err(),
        syntax::parse_with_lua(source).unwrap_err(),
    ] {
        assert!(error.message.contains("style is already specified"));
        assert_eq!(
            &source[error.span.start..error.span.end],
            "style = \"Body\""
        );
        assert_eq!(error.location(source), (2, 20));
    }
    compare_models(
        r#"#paragraph(style = "Before", style = "After")[Text]"#,
        r#"#paragraph(style = "After")[Text]"#,
    );
}

#[test]
fn recognized_headers_keep_argument_and_body_errors_in_both_expression_modes() {
    for (source, expected) in [
        (r#"#paragraph "Lead" ("Text")[Body]"#, "positional"),
        (r#"#paragraph "Lead" ()"#, "body"),
        (r#"#paragraph "Lead" (alignment = 7)[Body]"#, "alignment"),
        (r#"#paragraph "Lead"[Unclosed"#, "missing closing"),
        (
            r#"#paragraph "Lead" (alignment = "center""#,
            "expected ',' or ')'",
        ),
    ] {
        for enable_lua in [false, true] {
            let error = compile(source, enable_lua).unwrap_err();
            assert!(error.message.contains(expected), "{source}: {error}");
            assert!(source.is_char_boundary(error.span.start));
            assert!(source.is_char_boundary(error.span.end));
        }
    }
}

#[test]
fn truncated_unicode_applications_never_produce_invalid_source_ranges() {
    let source = r#"#paragraph "Résumé" (alignment = "center")[Text]"#;
    for end in source.char_indices().map(|(offset, _)| offset) {
        let prefix = &source[..end];
        for result in [syntax::parse(prefix), syntax::parse_with_lua(prefix)] {
            if let Err(error) = result {
                assert!(error.span.start <= error.span.end);
                assert!(error.span.end <= prefix.len());
                assert!(prefix.is_char_boundary(error.span.start));
                assert!(prefix.is_char_boundary(error.span.end));
            }
        }
    }
}
