use mote::{
    compile,
    json::to_json,
    model::{Content, StyleKind, Unit, Value},
};
use serde_json::json;

#[test]
fn declarations_register_without_content_and_json_preserves_authored_properties() {
    let document = compile(r#"#font(name="body",latin="Aptos",east_asian="Yu Gothic",complex_script="Arial",word_line_height=1.25)
        #style(name="Body",font="body",size="11pt")"#).unwrap();
    assert!(document.content.is_empty());
    assert_eq!(document.styles["Body"].kind, StyleKind::Paragraph);
    assert!(!document.styles["Body"].properties.contains_key("type"));
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    assert_eq!(
        output,
        json!({"page":{},"margin":{},"columns":{},"content":[],"fonts":[{"name":"body","latin":"Aptos","east_asian":"Yu Gothic","complex_script":"Arial","properties":{
            "latin":"Aptos","east_asian":"Yu Gothic","complex_script":"Arial","word_line_height":1.25
        }}],"styles":[{"name":"Body","type":"paragraph","font":"body","properties":{
            "font":"body","size":{"value":11.0,"unit":"pt","text":"11pt"}
        }}]})
    );
}

#[test]
fn later_declarations_replace_whole_entries_with_separate_registries() {
    let document = compile(
        r#"#font(name=7,latin="Aptos",east_asian="Yu Gothic")
        #font(name="7",latin="Arial")
        #style(name=7,type="character",bold=true)
        #style(name="7",italic=true)"#,
    )
    .unwrap();
    assert_eq!(document.fonts.len(), 1);
    assert_eq!(document.styles.len(), 1);
    assert_eq!(document.fonts["7"].properties.len(), 1);
    assert_eq!(document.styles["7"].properties.len(), 1);
    assert_eq!(document.styles["7"].kind, StyleKind::Paragraph);
    assert!(!document.styles["7"].properties.contains_key("bold"));
}

#[test]
fn declarations_in_evaluated_bodies_are_global_but_literal_text_is_inert() {
    let document = compile(
        r#"#paragraph[A#font(name="body",latin="Aptos")B]
        #code("chosen")[#style(name="Accent",type="character",bold=true)]
        `#font(name="literal")`"#,
    )
    .unwrap();
    assert_eq!(document.fonts.len(), 1);
    assert_eq!(document.styles.len(), 1);
    let Content::Element(paragraph) = &document.content[0] else {
        panic!("expected paragraph")
    };
    assert_eq!(
        paragraph.children,
        [Content::Text("A".into()), Content::Text("B".into())]
    );
    assert!(!document.fonts.contains_key("literal"));
}

#[test]
fn style_value_rules_cover_flat_nested_and_relationship_fields() {
    let document=compile(r##"#style(name="Detail",type="character",based_on="Base",next="Body",link="Companion",
        size="11pt",color="#ABC",fill="#def",background="#123",highlight="#456",
        bold="T",italic="false",underline=false,small_caps=true,spacing_before="2pt",spacing_after=0,
        line_spacing="12pt",first_line_indent="3pt",hanging_indent=pt(1),
        keep_with_next=true,keep_together=false,page_break_before=false,widow_control="1",outline_level="2",
        paragraph={alignment="right",spacing={before=pt(2),after="3pt",line={rule="multiple",value="1.2"}},
        indent={left="4pt",right=0,first_line=pt(1),hanging=pt(0)},fill="#abc",background="#def",
        border={bottom={width=pt(1),color="#123"}},tabs={{position=pt(20),alignment="left"}}},
        run={style="Reference",font="body",size="10pt",color="#123",fill="#456",background="#789",
        highlight="#abc",bold=false,italic=true,underline=true,small_caps=false,strike="true",double_strike="0"})"##).unwrap();
    let style = &document.styles["Detail"];
    assert_eq!(style.kind, StyleKind::Character);
    assert_eq!(style.properties["bold"], Value::Boolean(true));
    assert_eq!(style.properties["color"], Value::Color("#aabbcc".into()));
    assert_eq!(
        style.properties["first_line_indent"],
        Value::Length {
            value: 3.0,
            unit: Unit::Point
        }
    );
    let Value::Object(run) = &style.properties["run"] else {
        panic!("expected run object")
    };
    assert_eq!(run["strike"], Value::Boolean(true));
    assert_eq!(run["double_strike"], Value::Boolean(false));
    assert_eq!(
        run["size"],
        Value::Length {
            value: 10.0,
            unit: Unit::Point
        }
    );
}

#[test]
fn flat_geometry_preserves_authored_values_while_nested_paragraph_values_normalize() {
    let document = compile(
        r#"#style(name="Body",spacing={after="6pt"},indent={left="12pt"},outline_level="2",
        paragraph={spacing={after="4pt"},indent={left="8pt"},outline_level="1"})"#,
    )
    .unwrap();
    let properties = &document.styles["Body"].properties;
    let Value::Object(spacing) = &properties["spacing"] else {
        panic!("expected spacing")
    };
    assert_eq!(spacing["after"], Value::String("6pt".into()));
    assert_eq!(properties["outline_level"], Value::String("2".into()));
    let Value::Object(paragraph) = &properties["paragraph"] else {
        panic!("expected paragraph")
    };
    let Value::Object(nested_spacing) = &paragraph["spacing"] else {
        panic!("expected nested spacing")
    };
    assert_eq!(
        nested_spacing["after"],
        Value::Length {
            value: 4.0,
            unit: Unit::Point
        }
    );
    assert_eq!(paragraph["outline_level"], Value::Number(1.0));
}

#[test]
fn direct_self_inheritance_is_rejected_after_final_declaration_replacement() {
    let source = "#paragraph[Before]\n#style(name=\"Body\",based_on=\"Body\")";
    let error = compile(source).unwrap_err();
    assert_eq!(error.message, "style \"Body\" cannot be based on itself");
    assert_eq!(
        &source[error.span.start..error.span.end],
        "#style(name=\"Body\",based_on=\"Body\")"
    );
    let document =
        compile("#style(name=\"Body\",based_on=\"Body\")\n#style(name=\"Body\",bold=true)")
            .unwrap();
    assert!(!document.styles["Body"].properties.contains_key("based_on"));
    assert!(
        compile("#style(name=\"Body\",bold=true)\n#style(name=\"Body\",based_on=\"Body\")")
            .is_err()
    );
}

#[test]
fn declarations_reject_invalid_contracts_with_the_offending_source_location() {
    for source in [
        "#font()",
        "#style()",
        "#font(\"body\")",
        "#style(\"Body\")",
        "#font(name=\"body\")[]",
        "#style(name=\"Body\")[Text]",
        "#font(name=true)",
        "#font(name=\"body\",latin=\" \")",
        "#font(name=\"body\",latin=12)",
        "#font(name=\"body\",word_line_height=0)",
        "#font(name=\"body\",word_line_height=\"1.25\")",
        "#style(name=\"Body\",based_on=1)",
        "#style(name=\"Body\",run={})",
        "#style(name=\"Body\",run={bold=1})",
        "#style(name=\"Body\",spacing_after=pt(-1))",
    ] {
        let error = compile(source).expect_err(source);
        assert!(error.span.end <= source.len(), "{source}");
    }
    let source = "é\n#font(name=\"body\",word_line_height=\"bad\")";
    let error = compile(source).unwrap_err();
    assert_eq!(&source[error.span.start..error.span.end], "\"bad\"");
    let document = compile("#style(name=\"Body\",bold=1,bold=true)").unwrap();
    assert_eq!(
        document.styles["Body"].properties["bold"],
        Value::Boolean(true)
    );
    assert!(compile("#style(name=\"Body\",bold=missing(),bold=true)").is_err());
}

#[test]
fn declaration_metadata_is_retained_without_extra_type_restrictions() {
    for source in [
        "#style(name=\" \" )",
        "#font(name=\"body\",unknown=true)",
        "#style(name=\"Body\",type=\"table\")",
        "#style(name=\"Body\",properties={bold=true})",
        "#style(name=\"Body\",paragraph={style=\"Other\"})",
        "#style(name=\"Body\",run={kerning=pt(2)})",
        "#style(name=\"Body\",table={width=pt(10)})",
        "#style(name=\"Body\",strike=true)",
    ] {
        assert!(compile(source).is_ok(), "{source}");
    }
}

#[test]
fn numeric_names_warn_at_the_declaration_and_replacement_does_not_erase_warnings() {
    let source = "é\r\n#font(name=7,latin=\"Aptos\")\r\n#style(name=8,bold=true)\r\n#font(name=\"7\",latin=\"Arial\")";
    let document = compile(source).unwrap();
    assert_eq!(document.warnings.len(), 2);
    assert_eq!(
        document.warnings[0].message,
        "font name was coerced from number to string \"7\""
    );
    assert_eq!(
        document.warnings[1].message,
        "style name was coerced from number to string \"8\""
    );
    assert_eq!(document.warnings[0].location(source), (2, 1));
    assert_eq!(document.warnings[1].location(source), (3, 1));
    let location = document.warnings[0].span;
    assert_eq!(
        &source[location.start..location.end],
        "#font(name=7,latin=\"Aptos\")"
    );
    assert!(
        compile("#font(name=\"7\")#style(name=\"8\")")
            .unwrap()
            .warnings
            .is_empty()
    );
}
