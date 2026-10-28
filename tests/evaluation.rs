use mote::{
    compile,
    model::{Content, Element, ElementKind, Unit, Value},
};

fn element(content: &Content) -> &Element {
    let Content::Element(element) = content else {
        panic!("expected element")
    };
    element
}

#[test]
fn markdown_and_explicit_directives_have_the_same_meaning() {
    let markdown = "# Quarterly report\n\nRevenue increased by **12%**.";
    let directives =
        "#heading(level=1)[Quarterly report]\n\n#paragraph[Revenue increased by #strong[12%].]";
    let mut document = compile(markdown).unwrap();
    let Content::Element(heading) = &mut document.content[0] else {
        panic!("expected heading")
    };
    assert_eq!(
        heading.attributes.remove("marker"),
        Some(Value::String("#".into()))
    );
    assert_eq!(document, compile(directives).unwrap());
    assert_eq!(document.content.len(), 2);
    assert_eq!(element(&document.content[0]).kind, ElementKind::Heading);
    let paragraph = element(&document.content[1]);
    assert_eq!(
        paragraph.children[0],
        Content::Text("Revenue increased by ".into())
    );
    assert_eq!(element(&paragraph.children[1]).kind, ElementKind::Strong);
}

#[test]
fn attributes_and_helpers_produce_typed_values() {
    let document = compile("#paragraph(alignment=\"center\", keep_with_next=true, spacing={before=pt(6),after=pt(12)})[Status: #upper(\"paid\")]").unwrap();
    let paragraph = element(&document.content[0]);
    assert_eq!(
        paragraph.attributes["alignment"],
        Value::String("center".into())
    );
    assert_eq!(paragraph.attributes["keep_with_next"], Value::Boolean(true));
    let Value::Object(spacing) = &paragraph.attributes["spacing"] else {
        panic!("expected object")
    };
    assert_eq!(
        spacing["before"],
        Value::Length {
            value: 6.0,
            unit: Unit::Point
        }
    );
    assert_eq!(
        spacing["after"],
        Value::Length {
            value: 12.0,
            unit: Unit::Point
        }
    );
    assert_eq!(
        paragraph.children,
        [
            Content::Text("Status: ".into()),
            Content::Text("PAID".into())
        ]
    );
}

#[test]
fn paragraphs_flow_around_blocks_and_preserve_explicit_body_spaces() {
    let document =
        compile("  Before\nline.\n\n#heading(2)[Title]\nAfter\n#paragraph[ padded ]\n").unwrap();
    assert_eq!(document.content.len(), 4);
    assert_eq!(
        element(&document.content[0]).children,
        [
            Content::Text("Before".into()),
            Content::Text(" ".into()),
            Content::Text("line.".into())
        ]
    );
    assert_eq!(
        element(&document.content[1]).attributes["level"],
        Value::Number(2.0)
    );
    assert_eq!(
        element(&document.content[2]).children,
        [Content::Text("After".into())]
    );
    assert_eq!(
        element(&document.content[3]).children,
        [Content::Text(" padded ".into())]
    );
    assert!(compile(" \n\n\t").unwrap().content.is_empty());
    let document = compile("#paragraph[First\n\nsecond]").unwrap();
    assert_eq!(
        element(&document.content[0]).children,
        [
            Content::Text("First".into()),
            Content::Text("\n\n".into()),
            Content::Text("second".into())
        ]
    );
}

#[test]
fn heading_level_forms_agree_and_cannot_be_combined() {
    let expected = compile("#heading(level=2)[Results]").unwrap();
    for source in ["#heading(2)[Results]", "#heading(level=\"2\")[Results]"] {
        assert_eq!(compile(source).unwrap(), expected, "{source}");
    }
    for source in [
        "#heading(2, level=3)[Results]",
        "#heading(level=3, 2)[Results]",
    ] {
        let error = compile(source).expect_err(source);
        assert!(error.message.contains("both positionally and by name"));
    }
    let error = compile("#heading(\"2\")[Results]").unwrap_err();
    assert!(
        error
            .message
            .contains("positional heading level must be a number")
    );
}

#[test]
fn alignment_requires_the_exact_defined_spelling() {
    for directive in ["paragraph", "heading"] {
        for alignment in ["left", "center", "right", "justify"] {
            let source = format!("#{directive}(alignment=\"{alignment}\")[Text]");
            let document = compile(&source).unwrap();
            assert_eq!(
                element(&document.content[0]).attributes["alignment"],
                Value::String(alignment.into())
            );
        }
        for alignment in ["Center", "LEFT", " right", "justify "] {
            let source = format!("#{directive}(alignment=\"{alignment}\")[Text]");
            let error = compile(&source).expect_err(&source);
            assert!(error.message.contains("alignment must be"));
        }
    }
}

#[test]
fn empty_bodies_and_omitted_attributes_remain_distinct_from_explicit_values() {
    for (source, expected_kind) in [
        ("#paragraph[]", ElementKind::Paragraph),
        ("#heading[]", ElementKind::Heading),
    ] {
        let document = compile(source).unwrap();
        let block = element(&document.content[0]);
        assert_eq!(block.kind, expected_kind);
        assert!(block.children.is_empty());
        assert!(block.attributes.is_empty());
    }
    let document = compile("#strong[]").unwrap();
    let paragraph = element(&document.content[0]);
    assert_eq!(paragraph.kind, ElementKind::Paragraph);
    let strong = element(&paragraph.children[0]);
    assert_eq!(strong.kind, ElementKind::Strong);
    assert!(strong.children.is_empty());
    assert!(strong.attributes.is_empty());

    let document = compile("#paragraph(spacing={after=pt(0)})[]").unwrap();
    let attributes = &element(&document.content[0]).attributes;
    assert_eq!(attributes.len(), 1);
    let Value::Object(spacing) = &attributes["spacing"] else {
        panic!("expected spacing object")
    };
    assert_eq!(spacing.len(), 1);
    assert_eq!(
        spacing["after"],
        Value::Length {
            value: 0.0,
            unit: Unit::Point
        }
    );
}

#[test]
fn length_helpers_accept_numeric_magnitudes_with_exactly_one_positional_argument() {
    assert_eq!(compile("#pt(12)").unwrap(), compile("#pt(\"12\")").unwrap());
    assert_eq!(compile("#in(1)").unwrap(), compile("#inch(1)").unwrap());
    let document = compile("#pt(-1)").unwrap();
    assert_eq!(
        element(&document.content[0]).children,
        [Content::Text("-1pt".into())]
    );
    for source in ["#pt()", "#pt(value=12)", "#pt(12, 13)"] {
        let error = compile(source).expect_err(source);
        assert!(error.message.contains("requires one positional argument"));
    }
    for source in ["#pt(true)", "#pt(\"12pt\")", "#pt(\"NaN\")", "#pt(\"inf\")"] {
        let error = compile(source).expect_err(source);
        assert!(
            error
                .message
                .contains("length magnitude must be a finite number")
        );
    }
    assert!(compile("#pt(12)[]").is_err());
}

#[test]
fn invalid_calls_report_the_problem_without_producing_a_document() {
    for (source, message) in [
        ("#heading(level=0)[Title]", "positive integer"),
        ("#heading(level=1.5)[Title]", "positive integer"),
        ("#paragraph(alignment=true)[Text]", "alignment"),
        ("#paragraph(keep_with_next=1)[Text]", "boolean"),
        ("#paragraph(spacing={after=pt(-1)})[Text]", "non-negative"),
        (
            "#paragraph(spacing={after=pt(false)})[Text]",
            "finite number",
        ),
        ("#paragraph()", "content body"),
        ("#strong[x][y]", "one content body"),
        ("#paragraph[#heading(1)[Nested]]", "inline content"),
        ("#strong[#paragraph[Nested]]", "inline content"),
        ("#paragraph(2)[Text]", "positional"),
        ("#unknown[Text]", "unsupported directive"),
        ("#upper(\"paid\", true)", "one positional"),
        ("#upper(\"ß\")", "ASCII"),
        (
            "#paragraph(spacing={after=missing(12)})[Text]",
            "unsupported helper",
        ),
    ] {
        let error = compile(source).expect_err(source);
        assert!(error.message.contains(message), "{source}: {error}");
        assert!(error.span.end <= source.len());
    }
}

#[test]
fn units_strings_and_boolean_normalization() {
    for (source, expected) in [
        ("inch(1)", Unit::Inch),
        ("cm(2.5)", Unit::Centimetre),
        ("mm(4)", Unit::Millimetre),
        ("twip(20)", Unit::Twip),
        ("\"12pt\"", Unit::Point),
    ] {
        let document = compile(&format!(
            "#paragraph(keep_with_next=\"true\",spacing={{after={source}}})[Text]"
        ))
        .unwrap();
        let attributes = &element(&document.content[0]).attributes;
        assert_eq!(attributes["keep_with_next"], Value::Boolean(true));
        let Value::Object(spacing) = &attributes["spacing"] else {
            panic!("expected object")
        };
        assert!(matches!(spacing["after"], Value::Length { unit, .. } if unit == expected));
    }
}

#[test]
fn errors_point_to_the_rejected_attribute_value() {
    let source = "é\n#heading(level = 0)[Title]";
    let error = compile(source).unwrap_err();
    assert_eq!(&source[error.span.start..error.span.end], "0");
    assert_eq!(error.location(source), (2, 18));
}
