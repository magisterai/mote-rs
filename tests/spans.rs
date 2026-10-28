use mote::{
    compile,
    json::to_json,
    model::{Content, Element, ElementKind, Properties, Unit, Value},
};

fn span(source: &str) -> Element {
    let document = compile(source).unwrap();
    let Content::Element(paragraph) = &document.content[0] else {
        panic!("expected paragraph")
    };
    let Content::Element(span) = &paragraph.children[0] else {
        panic!("expected span")
    };
    assert_eq!(span.kind, ElementKind::Span);
    span.clone()
}

#[test]
fn span_normalizes_every_scoped_attribute_and_property_overrides() {
    let span = span(
        r##"#span(style="Accent",font="body",size="12pt",color="#ABC",
        highlight=color("#def"),bold="true",italic=false,underline="0",
        small_caps=true,strike="F",double_strike=true,properties={bold=false})[Text]"##,
    );
    assert_eq!(
        span.attributes,
        Properties::from([
            ("style".into(), Value::String("Accent".into())),
            ("font".into(), Value::String("body".into())),
            (
                "size".into(),
                Value::Length {
                    value: 12.0,
                    unit: Unit::Point
                }
            ),
            ("color".into(), Value::Color("#aabbcc".into())),
            ("highlight".into(), Value::Color("#ddeeff".into())),
            ("bold".into(), Value::Boolean(false)),
            ("italic".into(), Value::Boolean(false)),
            ("underline".into(), Value::Boolean(false)),
            ("small_caps".into(), Value::Boolean(true)),
            ("strike".into(), Value::Boolean(false)),
            ("double_strike".into(), Value::Boolean(true)),
        ])
    );
    assert_eq!(span.children, [Content::Text("Text".into())]);
    assert!(!span.kind.is_block());
}

#[test]
fn spans_preserve_structure_omission_and_enclosing_whitespace_mode() {
    let empty = span("#span[]");
    assert!(empty.attributes.is_empty());
    assert!(empty.children.is_empty());
    let outer = span("#span(bold=true)[first #span(bold=false)[second] third]");
    let Content::Element(inner) = &outer.children[1] else {
        panic!("expected inner span")
    };
    assert_eq!(
        inner.attributes,
        Properties::from([("bold".into(), Value::Boolean(false))])
    );
    assert_eq!(outer.children[0], Content::Text("first ".into()));
    assert_eq!(outer.children[2], Content::Text(" third".into()));
    let document = compile("#code(preserve_space=true)[#span[a\r\n  b]]").unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    assert_eq!(
        output["content"][0]["children"][0]["children"][0]["children"],
        serde_json::json!([{ "kind":"text", "text":"a" }, {"kind":"text", "text":"\n"}, {"kind":"text", "text":"  b"}])
    );
    assert_eq!(
        span("#span[a\nb]").children,
        [
            Content::Text("a".into()),
            Content::Text(" ".into()),
            Content::Text("b".into())
        ]
    );
}

#[test]
fn spans_reject_invalid_bodies_and_normalized_values() {
    for source in [
        "#span()",
        "#span(12)[Text]",
        "#span[a][b]",
        "#span[#paragraph[Block]]",
        "#span(font=12)[Text]",
        "#span(size=pt(-1))[Text]",
        "#span(color=\"red\")[Text]",
        "#span(highlight=\"#1234\")[Text]",
        "#span(bold=1)[Text]",
        "#span(underline=\"wave\")[Text]",
        "#span(properties={})[Text]",
    ] {
        assert!(compile(source).is_err(), "{source}");
    }
}
