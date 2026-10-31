#[path = "support/language_capture.rs"]
mod language_capture;
use mote::model::{Content, Element, Value};
fn element(content: &Content) -> &Element {
    let Content::Element(element) = content else {
        panic!("expected element")
    };
    element
}
#[test]
fn progressive_defaults_match_captured_models_packages_and_rasters() {
    language_capture::compare("scoped-defaults", &["document", "slides"]);
}
#[test]
fn defaults_are_lexical_and_unknown_properties_remain_visible() {
    let document = mote::compile(
        r#"#set paragraph(style="Outer",future={one=1,two=2})
Before.

#section[
#set paragraph(style="Inner",future={one=nil})
Inside.
]
After.
"#,
    )
    .unwrap();
    assert_eq!(document.sections.len(), 3);
    for (section, expected) in document.sections.iter().zip(["Outer", "Inner", "Outer"]) {
        assert_eq!(
            element(&section.content[0]).attributes["style"].to_text(),
            expected
        );
    }
    let Value::Object(properties) = &element(&document.sections[1].content[0]).attributes["future"]
    else {
        panic!("object")
    };
    assert_eq!(properties["one"], Value::Nil);
    assert_eq!(properties["two"], Value::Number(2.0));
    assert!(document.warnings.is_empty());
}
#[test]
fn default_contracts_reject_bad_targets_syntax_and_values() {
    for source in [
        "#set document(default_size=pt(12))",
        "#set paragraph(12)",
        "#set paragraph(alignment=\"bad\")",
        "#set paragraph(style=\"A\")[body]",
        "#set paragraph",
        "#set line(properties=12)",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    // Required values belong to construction, not a partial default declaration.
    mote::compile(
        "#set image(width=pt(30))\n#set shape(fill=\"#ffffff\")\n#set chart(kind=\"line\")",
    )
    .unwrap();
}

#[test]
fn defaults_do_not_satisfy_required_arguments_or_change_layout_slot_removals() {
    for source in [
        "#set image(src=\"picture.png\")\n#image()",
        "#set chart(data={values={1}})\n#chart()",
        "#set shape(position={x=0,y=0,width=100,height=100})\n#shape[]",
        "#set line(from={x=0,y=0},to={x=100,y=100})\n#line()",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let source = "#set box(fill=\"#eeeeee\")\n#layout(name=\"Reduced\",based_on=\"title_and_content\")[#box(slot=\"title\",remove=true)]";
    let document = mote::compile(source).unwrap();
    assert!(
        document.presentation.layouts[0].slots[0]
            .properties
            .is_empty()
    );
}
