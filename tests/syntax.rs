use mote::syntax::{ExpressionKind, NodeKind, parse, parse_expression};

#[test]
fn markdown_and_directive_calls_remain_distinct_in_syntax() {
    let tree = parse("# Quarterly report\n\n#heading(level = 1)[Quarterly report]").unwrap();
    assert!(matches!(
        &tree.nodes[0].kind,
        NodeKind::Heading { level: 1, .. }
    ));
    let NodeKind::Call(call) = &tree.nodes[2].kind else {
        panic!("expected call")
    };
    assert_eq!(call.name, "heading");
    assert_eq!(call.arguments[0].name.as_deref(), Some("level"));
    assert_eq!(call.arguments[0].value.kind, ExpressionKind::Number(1.0));
}

#[test]
fn nested_content_keeps_structure_and_source_ranges() {
    let source = "#paragraph[Résumé: #strong[12%].]";
    let tree = parse(source).unwrap();
    assert_eq!(tree.nodes[0].span.end, source.len());
    let NodeKind::Call(call) = &tree.nodes[0].kind else {
        panic!("expected call")
    };
    assert_eq!(call.bodies[0][0].kind, NodeKind::Text("Résumé: ".into()));
    let strong = &call.bodies[0][1];
    assert_eq!(&source[strong.span.start..strong.span.end], "#strong[12%]");
    assert!(matches!(&strong.kind, NodeKind::Call(call) if call.name == "strong"));
}

#[test]
fn nested_objects_helpers_and_quoted_delimiters() {
    let expression =
        parse_expression("{ label = 'a, ) ] }', spacing = {after = pt(12),}, enabled = true, }")
            .unwrap();
    let ExpressionKind::Object(fields) = expression.kind else {
        panic!("expected object")
    };
    assert_eq!(fields.len(), 3);
    assert_eq!(
        fields[0].value.kind,
        ExpressionKind::String("a, ) ] }".into())
    );
    assert!(matches!(fields[1].value.kind, ExpressionKind::Object(_)));
    assert_eq!(fields[2].value.kind, ExpressionKind::Boolean(true));
}

#[test]
fn literal_values_and_string_escapes() {
    for (source, expected) in [
        ("-12.5", ExpressionKind::Number(-12.5)),
        ("nil", ExpressionKind::Nil),
        ("false", ExpressionKind::Boolean(false)),
        (
            r#""a\n\t\r\\\"\'é""#,
            ExpressionKind::String("a\n\t\r\\\"'é".into()),
        ),
    ] {
        assert_eq!(parse_expression(source).unwrap().kind, expected);
    }
    assert!(parse_expression(&"9".repeat(400)).is_err());
}

#[test]
fn content_escapes_do_not_become_directives_or_string_escapes() {
    let tree = parse(r"\#strong[text\] \n \\").unwrap();
    assert_eq!(
        tree.nodes[0].kind,
        NodeKind::Text("#strong[text] n \\".into())
    );
}

#[test]
fn line_endings_and_hard_breaks() {
    for ending in ["\n", "\r\n", "\r"] {
        let tree = parse(&format!(
            "first{ending}second{ending} \t{ending}third  {ending}fourth"
        ))
        .unwrap();
        assert_eq!(tree.nodes[1].kind, NodeKind::LineBreak { hard: false });
        assert_eq!(tree.nodes[3].kind, NodeKind::ParagraphBreak);
        assert_eq!(tree.nodes[5].kind, NodeKind::LineBreak { hard: true });
    }
}

#[test]
fn invalid_forms_have_usable_source_locations() {
    for source in [
        "#",
        "#bad-name[x]",
        "#paragraph[x",
        "#heading(level = )[x]",
        "#heading(level=1 2)[x]",
        "#paragraph(x='oops)",
        "#paragraph[**oops]",
        "#paragraph(keep-with-next=true)[x]",
    ] {
        let error = parse(source).expect_err(source);
        assert!(error.span.start <= error.span.end);
        assert!(error.span.end <= source.len());
        assert!(source.is_char_boundary(error.span.start));
    }
    let source = "é\r\n汉 #";
    let error = parse(source).unwrap_err();
    assert_eq!(error.location(source), (2, 4));
}

#[test]
fn incomplete_prefixes_and_deep_input_do_not_panic() {
    let source = "#paragraph(spacing={after=pt(12)})[Résumé **ready**]";
    for (end, _) in source.char_indices() {
        let _ = parse(&source[..end]);
    }
    let nested = format!("{}x{}", "#strong[".repeat(100), "]".repeat(100));
    assert!(
        parse(&nested)
            .unwrap_err()
            .message
            .contains("nesting limit")
    );
    let objects = format!("{}1{}", "{a=".repeat(100), "}".repeat(100));
    assert!(
        parse_expression(&objects)
            .unwrap_err()
            .message
            .contains("nesting limit")
    );
}
