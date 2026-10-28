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
    let document = compile(markdown).unwrap();
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
    assert_eq!(paragraph.children, [Content::Text("Status: PAID".into())]);
}

#[test]
fn paragraphs_flow_around_blocks_and_preserve_explicit_body_spaces() {
    let document =
        compile("  Before\nline.\n\n#heading(2)[Title]\nAfter\n#paragraph[ padded ]\n").unwrap();
    assert_eq!(document.content.len(), 4);
    assert_eq!(
        element(&document.content[0]).children,
        [Content::Text("Before line.".into())]
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
        (
            "#paragraph(style=\"Body\")[Text]",
            "unsupported paragraph attribute",
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
