use mote::{
    compile,
    json::to_json,
    model::{Content, Element, Value},
};
use serde_json::{Value as JsonValue, json};

fn find_element<'content>(content: &'content [Content], kind: &str) -> &'content Element {
    for node in content {
        if let Content::Element(element) = node {
            if element.kind.as_text() == kind {
                return element;
            }
            if let Some(found) = find_nested_element(&element.children, kind) {
                return found;
            }
        }
    }
    panic!("missing {kind}")
}

fn find_nested_element<'content>(
    content: &'content [Content],
    kind: &str,
) -> Option<&'content Element> {
    content.iter().find_map(|node| match node {
        Content::Element(element) if element.kind.as_text() == kind => Some(element),
        Content::Element(element) => find_nested_element(&element.children, kind),
        _ => None,
    })
}

fn attributes(source: &str, kind: &str) -> JsonValue {
    let document = compile(source).unwrap_or_else(|error| panic!("{source}: {error}"));
    let element = find_element(&document.content, kind).clone();
    let document = mote::model::Document {
        content: vec![Content::Element(element)],
        ..mote::model::Document::default()
    };
    let output: JsonValue = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    output["content"][0]["attributes"].clone()
}

fn length(value: f64, unit: &str) -> JsonValue {
    json!({"value":value, "unit":unit, "text":format!("{value}{unit}")})
}

#[test]
fn paragraph_attributes_preserve_structure_and_normalize_values() {
    let actual = attributes(
        r##"#paragraph(
        style="Body", alignment="justify", spacing_before="40", spacing_after=pt(8), line_spacing="14pt",
        spacing={before="2pt",after=0,line={rule="multiple",value="1.15"}},
        indent={left="18pt",right=pt(3),first_line=20,hanging="0"},
        border={all={style="single",size=8,width=pt(1),space=1,color="#ABC"},bottom="none"},
        fill=" #AbC ", tabs={{position=inch(1),alignment="right",leader="dot"},{position="24pt",alignment="left"}},
        keep_with_next="true",keep_together=false,page_break_before="0",widow_control=true,outline_level="2"
    )[Text]"##,
        "paragraph",
    );
    assert_eq!(
        actual,
        json!({
            "style":"Body","alignment":"justify","spacing_before":"40","spacing_after":length(8.0,"pt"),"line_spacing":length(14.0,"pt"),
            "spacing":{"before":length(2.0,"pt"),"after":0.0,"line":{"rule":"multiple","value":1.15}},
            "indent":{"left":length(18.0,"pt"),"right":length(3.0,"pt"),"first_line":20.0,"hanging":"0"},
            "border":{"all":{"style":"single","size":8.0,"width":length(1.0,"pt"),"space":1.0,"color":"#ABC"},"bottom":"none"},
            "fill":"#aabbcc","tabs":[{"position":length(1.0,"in"),"alignment":"right","leader":"dot"},{"position":"24pt","alignment":"left"}],
            "keep_with_next":true,"keep_together":false,"page_break_before":false,"widow_control":true,"outline_level":2.0
        })
    );
}

#[test]
fn heading_numbering_and_paragraph_controls_keep_authored_intent() {
    let actual = attributes(
        r#"#heading(3,style="Chapter",alignment="right",
        spacing_before="4pt",spacing_after=pt(6),line_spacing=pt(12),
        spacing={line={rule="at_least",value="13pt"}},indent={right=pt(5)},
        keep_with_next=true,keep_together="false",page_break_before=true,widow_control="T",
        numbering={enabled="true",id=7,format="upper_roman",separator="-",start="2",restart="false"}
    )[Title]"#,
        "heading",
    );
    assert_eq!(
        actual,
        json!({
            "level":3.0,"style":"Chapter","alignment":"right",
            "spacing_before":length(4.0,"pt"),"spacing_after":length(6.0,"pt"),"line_spacing":length(12.0,"pt"),
            "spacing":{"line":{"rule":"at_least","value":length(13.0,"pt")}},"indent":{"right":length(5.0,"pt")},
            "keep_with_next":true,"keep_together":false,"page_break_before":true,"widow_control":true,
            "numbering":{"enabled":true,"id":"7","format":"upper_roman","separator":"-","start":2.0,"restart":false}
        })
    );
}

#[test]
fn inline_style_references_and_overrides_remain_local() {
    for directive in ["strong", "emphasis", "code"] {
        let source = format!(
            r##"#{directive}(style="Accent",font="Aptos",size="12pt",color="#ABC")[Text]"##
        );
        assert_eq!(
            attributes(&source, directive),
            json!({"style":"Accent","font":"Aptos","size":"12pt","color":"#ABC"})
        );
        let source = format!(r##"#{directive}(size=pt(12),color=color("#ABC"))[Text]"##);
        assert_eq!(
            attributes(&source, directive),
            json!({"size":length(12.0,"pt"),"color":"#aabbcc"})
        );
    }
    let document =
        compile(r##"#strong(font="Aptos")[before #emphasis(color="#f00")[inside] after] outside"##)
            .unwrap();
    let strong = find_element(&document.content, "strong");
    let emphasis = find_element(&document.content, "emphasis");
    assert_eq!(strong.attributes.len(), 1);
    assert_eq!(emphasis.attributes.len(), 1);
    assert!(!emphasis.attributes.contains_key("font"));
    assert!(
        find_element(&document.content, "paragraph")
            .attributes
            .is_empty()
    );
}

#[test]
fn list_attributes_are_normalized_without_changing_the_container_kind() {
    for directive in ["list", "ordered_list"] {
        let source = format!(
            r#"#{directive}(id=4,style="Steps",start="3",marker="*",format="lower_alpha",
            restart="F",tight=true,level="2",indent={{left="36pt",hanging=pt(18),level=pt(12)}},
            spacing={{before=pt(1),after="4pt",line={{rule="exact",value=pt(15)}}}})[#item[Text]]"#
        );
        assert_eq!(
            attributes(&source, directive),
            json!({
                "id":"4","style":"Steps","start":3.0,"marker":"*","format":"lower_alpha",
                "restart":false,"tight":true,"level":2.0,
                "indent":{"left":length(36.0,"pt"),"hanging":length(18.0,"pt"),"level":length(12.0,"pt")},
                "spacing":{"before":length(1.0,"pt"),"after":length(4.0,"pt"),"line":{"rule":"exact","value":length(15.0,"pt")}}
            })
        );
    }
}

#[test]
fn item_checkbox_metadata_does_not_invent_a_task_state() {
    let actual = attributes(
        r#"#list[#item(style="Task",marker="-",level="2",tight="true",
        indent={left=pt(54),hanging=pt(18),level=pt(9)},spacing={after=pt(0)},checked=false,
        checkbox={checked=true,kind="content_control",name=12,title=34})[Text]]"#,
        "list_item",
    );
    assert_eq!(
        actual,
        json!({
            "style":"Task","marker":"-","level":2.0,"tight":true,"checked":false,
            "indent":{"left":length(54.0,"pt"),"hanging":length(18.0,"pt"),"level":length(9.0,"pt")},"spacing":{"after":length(0.0,"pt")},
            "checkbox":{"checked":true,"kind":"content_control","name":"12","title":"34"}
        })
    );
    let nested_only = attributes(
        r#"#item(checkbox={checked=true,kind="legacy_form_field",name="approval"})[Text]"#,
        "list_item",
    );
    assert!(nested_only.get("checked").is_none());
    assert_eq!(nested_only["checkbox"]["checked"], true);
}

#[test]
fn quote_attributes_keep_raw_layout_values_and_typed_fill() {
    assert_eq!(
        attributes(
            r##"#blockquote(style="Quote",border={left={val="single",width="1pt",color="#123"}},
        fill="#AbC",alignment="center",spacing={before="2pt",after=pt(3),line={rule="multiple",value="1.2"}},
        indent={left="12pt",right=pt(1),first_line=0,hanging=0},spacing_before="4pt",spacing_after=pt(5),line_spacing="12pt"
    )[Text]"##,
            "blockquote"
        ),
        json!({
            "style":"Quote","border":{"left":{"val":"single","width":"1pt","color":"#123"}},"fill":"#aabbcc","alignment":"center",
            "spacing":{"before":"2pt","after":length(3.0,"pt"),"line":{"rule":"multiple","value":"1.2"}},
            "indent":{"left":"12pt","right":length(1.0,"pt"),"first_line":0.0,"hanging":0.0},
            "spacing_before":"4pt","spacing_after":length(5.0,"pt"),"line_spacing":"12pt"
        })
    );
}

#[test]
fn properties_override_outer_arguments_before_validation_and_preserve_error_locations() {
    for source in [
        r#"#paragraph(alignment="bad",properties={alignment="right",spacing={after=pt(2)}})[Text]"#,
        r#"#paragraph(properties={alignment="right",spacing={after=pt(2)}},alignment="bad")[Text]"#,
    ] {
        assert_eq!(
            attributes(source, "paragraph"),
            json!({"alignment":"right","spacing":{"after":length(2.0,"pt")}})
        );
    }
    for (directive, kind, body) in [
        ("paragraph", "paragraph", "Text"),
        ("heading", "heading", "Text"),
        ("strong", "strong", "Text"),
        ("emphasis", "emphasis", "Text"),
        ("code", "code", "Text"),
        ("list", "list", "#item[Text]"),
        ("ordered_list", "ordered_list", "#item[Text]"),
        ("item", "list_item", "Text"),
        ("blockquote", "blockquote", "Text"),
    ] {
        assert_eq!(
            attributes(
                &format!("#{directive}(properties={{style=\"Named\"}})[{body}]"),
                kind
            ),
            json!({"style":"Named"})
        );
    }
    let source = "é\n#paragraph(properties={alignment=\"wrong\"})[Text]";
    let error = compile(source).unwrap_err();
    assert_eq!(&source[error.span.start..error.span.end], "\"wrong\"");
    for overridden_level in ["3", "0", "\"invalid\""] {
        assert_eq!(
            attributes(
                &format!("#heading(2,properties={{level={overridden_level}}})[Text]"),
                "heading"
            )["level"],
            2.0
        );
    }
}

#[test]
fn preserved_code_keeps_nested_whitespace_and_normalizes_line_endings() {
    for ending in ["\n", "\r\n", "\r"] {
        let source = format!(
            "#code(language=\"text\",properties={{preserve_space=\"true\"}})[  first  {ending} \t{ending}#strong[bold{ending} next]  \tend{ending}]"
        );
        let document = compile(&source).unwrap();
        let code = find_element(&document.content, "code");
        assert_eq!(code.attributes["preserve_space"], Value::Boolean(true));
        assert_eq!(code.attributes["language"], Value::String("text".into()));
        assert_eq!(
            &code.children[..2],
            &[
                Content::Text("  first  ".into()),
                Content::Text("\n \t\n".into())
            ]
        );
        assert_eq!(
            find_element(&code.children, "strong").children,
            [
                Content::Text("bold".into()),
                Content::Text("\n".into()),
                Content::Text(" next".into())
            ]
        );
        assert_eq!(
            &code.children[3..],
            &[Content::Text("  \tend".into()), Content::Text("\n".into())]
        );
    }
    let document = compile("#code(\"a\\r\\nb\\rc\",preserve_space=true)[ignored]").unwrap();
    assert_eq!(
        find_element(&document.content, "code").children,
        [Content::Text("a\nb\nc".into())]
    );
    let document =
        compile("#code(preserve_space=true)[#emphasis[a\n b] #code(preserve_space=false)[c\n d]]")
            .unwrap();
    assert_eq!(
        find_element(&document.content, "emphasis").children,
        [
            Content::Text("a".into()),
            Content::Text("\n".into()),
            Content::Text(" b".into())
        ]
    );
    let outer_code = find_element(&document.content, "code");
    let inner_code = find_element(&outer_code.children, "code");
    assert_eq!(
        inner_code.attributes["preserve_space"],
        Value::Boolean(false)
    );
    assert_eq!(
        inner_code.children,
        [
            Content::Text("c".into()),
            Content::Text("\n".into()),
            Content::Text(" d".into())
        ]
    );
    let document =
        compile("#code(preserve_space=true)[  #strong[first]\n\t#emphasis[second]\n  text]")
            .unwrap();
    let code = find_element(&document.content, "code");
    assert!(
        matches!(&code.children[0], Content::Element(element) if element.kind.as_text() == "strong")
    );
    assert_eq!(code.children[1], Content::Text("\n".into()));
    assert_eq!(
        &code.children[3..],
        &[Content::Text("\n".into()), Content::Text("  text".into())]
    );
    let source = "#paragraph[a \\\nb]";
    assert_eq!(
        find_element(&compile(source).unwrap().content, "paragraph").children,
        [
            Content::Text("a ".into()),
            Content::Text("\n".into()),
            Content::Text("b".into())
        ]
    );
}

#[test]
fn tab_arrays_shorthand_and_empty_array_have_distinct_models() {
    assert_eq!(
        attributes("#paragraph(tabs={right=pt(72)})[Text]", "paragraph"),
        json!({"tabs":{"right":length(72.0,"pt")}})
    );
    assert_eq!(
        attributes("#paragraph(tabs={})[Text]", "paragraph"),
        json!({"tabs":[]})
    );
    assert_eq!(
        attributes(
            r#"#paragraph(tabs={pos=pt(24),align="center",leader="dot"})[Text]"#,
            "paragraph"
        ),
        json!({"tabs":{"pos":length(24.0,"pt"),"align":"center","leader":"dot"}})
    );
}

#[test]
fn normalization_covers_boolean_spellings_spacing_rules_and_number_formats() {
    for (text, expected) in [
        ("true", true),
        ("True", true),
        ("TRUE", true),
        ("t", true),
        ("T", true),
        ("1", true),
        ("false", false),
        ("False", false),
        ("FALSE", false),
        ("f", false),
        ("F", false),
        ("0", false),
    ] {
        for directive in ["paragraph", "heading"] {
            for property in [
                "keep_with_next",
                "keep_together",
                "page_break_before",
                "widow_control",
            ] {
                assert_eq!(
                    attributes(
                        &format!("#{directive}({property}=\"{text}\")[Text]"),
                        directive
                    )[property],
                    expected
                );
            }
        }
    }
    for rule in ["multiple", "exact", "at_least"] {
        let value = if rule == "multiple" { "1.2" } else { "pt(12)" };
        assert_eq!(
            attributes(
                &format!("#paragraph(spacing={{line={{rule=\"{rule}\",value={value}}}}})[Text]"),
                "paragraph"
            )["spacing"]["line"]["rule"],
            rule
        );
    }
    for format in [
        "decimal",
        "lower_alpha",
        "upper_alpha",
        "lower_roman",
        "upper_roman",
    ] {
        assert_eq!(
            attributes(
                &format!("#heading(numbering={{format=\"{format}\"}})[Text]"),
                "heading"
            )["numbering"]["format"],
            format
        );
        assert_eq!(
            attributes(
                &format!("#ordered_list(format=\"{format}\")[#item[Text]]"),
                "ordered_list"
            )["format"],
            format
        );
    }
    assert_eq!(
        attributes(r#"#list(format="bullet")[#item[Text]]"#, "list")["format"],
        "bullet"
    );
}

#[test]
fn invalid_normalized_values_and_argument_types_fail_with_diagnostics() {
    for source in [
        "#paragraph(style=2)[Text]",
        "#code(language=false)[Text]",
        "#code(preserve_space=1)[Text]",
        "#paragraph(fill=\"#12345g\")[Text]",
        "#paragraph(fill=color(3))[Text]",
        "#paragraph(spacing={line={value=1}})[Text]",
        "#paragraph(spacing={line={rule=\"exact\"}})[Text]",
        "#paragraph(spacing={line={rule=\"multiple\",value=0}})[Text]",
        "#paragraph(spacing={line={rule=\"multiple\",value=pt(2)}})[Text]",
        "#paragraph(spacing={line={rule=\"exact\",value=-1}})[Text]",
        "#paragraph(spacing={line={rule=\"single\",value=1}})[Text]",
        "#paragraph(indent={left=-1})[Text]",
        "#paragraph(outline_level=-1)[Text]",
        "#paragraph(properties={})[Text]",
        "#heading(numbering={format=\"bullet\"})[Text]",
        "#heading(numbering={start=1.5})[Text]",
        "#ordered_list(level=0)[#item[Text]]",
        "#list(restart=2)[#item[Text]]",
        "#item(checkbox={kind=\"widget\"})[Text]",
        "#item(checkbox={checked=1})[Text]",
        "#item(checked=\"yes\")[Text]",
    ] {
        let error = compile(source).expect_err(source);
        assert!(!error.message.is_empty(), "{source}");
        assert!(error.span.end <= source.len(), "{source}: {error}");
    }
}

#[test]
fn retained_metadata_is_accepted_without_claiming_a_formatting_effect() {
    for source in [
        "#strong(font=true)[Text]",
        "#emphasis(size=pt(-1))[Text]",
        "#strong(color=\"red\")[Text]",
        "#paragraph(indent={unknown=1})[Text]",
        "#paragraph(border={top={size=-1}})[Text]",
        "#paragraph(border={insideH=\"single\"})[Text]",
        "#paragraph(border={all={shadow=true}})[Text]",
        "#paragraph(tabs={alignment=\"right\"})[Text]",
        "#paragraph(tabs={true})[Text]",
        "#paragraph(tabs={{position=pt(-1)}})[Text]",
        "#paragraph(properties={unknown=true})[Text]",
        "#heading(numbering={label=\"Chapter\"})[Text]",
        "#blockquote(alignment=\"Center\")[Text]",
        "#line_break(properties={style=\"Named\"})",
        "#strong(kerning=2)[Text]",
    ] {
        assert!(compile(source).is_ok(), "{source}");
    }
}
