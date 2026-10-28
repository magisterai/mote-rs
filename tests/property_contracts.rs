use mote::{compile, json::to_json, model::Content};
use serde_json::{Value as JsonValue, json};

fn inspection(source: &str) -> JsonValue {
    serde_json::from_str(&to_json(&compile(source).unwrap()).unwrap()).unwrap()
}

#[test]
fn extra_properties_survive_every_content_directive() {
    for directive in [
        "paragraph",
        "heading",
        "span",
        "strong",
        "emphasis",
        "code",
        "list",
        "ordered_list",
        "item",
        "blockquote",
        "line_break",
    ] {
        let body = match directive {
            "line_break" => "",
            "list" | "ordered_list" => "[#item[Text]]",
            _ => "[Text]",
        };
        let document = compile(&format!(
            "#{directive}(extra={{label=\"outer\"}},properties={{extra={{label=\"inner\"}}}}){body}"
        ))
        .unwrap();
        let mut element = match &document.content[0] {
            Content::Element(element) => element,
            _ => panic!("expected element"),
        };
        if matches!(
            directive,
            "span" | "strong" | "emphasis" | "code" | "line_break" | "item"
        ) {
            element = match &element.children[0] {
                Content::Element(element) => element,
                _ => panic!("expected element"),
            };
        }
        assert_eq!(element.attributes.len(), 1, "{directive}");
        assert_eq!(element.attributes["extra"].to_text(), "{label: inner}");
    }
}

#[test]
fn signature_types_precede_overrides_but_only_selected_merged_fields_normalize() {
    for source in [
        "#paragraph(alignment=1,properties={alignment=\"right\"})[Text]",
        "#span(font=1,properties={font=\"Aptos\"})[Text]",
        "#code(language=false,properties={language=\"text\"})[Text]",
        "#paragraph(border=false,properties={border={all=\"none\"}})[Text]",
    ] {
        assert!(compile(source).is_err(), "{source}");
    }
    let output = inspection(
        "#span(font=1,font=\"body\",color=\"wrong\",properties={font=42,color=\"#ABC\",properties={ignored=true}})[Text]",
    );
    assert_eq!(
        output["content"][0]["children"][0]["attributes"],
        json!({"font":42.0,"color":"#aabbcc"})
    );
    let output =
        inspection("#paragraph(properties=false,properties={border=false,tabs=42,style=7})[Text]");
    assert_eq!(
        output["content"][0]["attributes"],
        json!({"border":false,"tabs":42.0,"style":7.0})
    );
    assert!(compile("#span(font=missing(),font=\"body\")[Text]").is_err());
}

#[test]
fn nested_metadata_is_retained_while_length_maps_and_selected_fields_normalize() {
    let output = inspection(
        r#"#heading(indent={custom="2pt"},spacing={custom="3pt",line={rule="multiple",value="1.5",extra=false}},numbering={label=true,prefix={text="x"},separator=nil,extra={flag=true}})[Text]"#,
    );
    let attributes = &output["content"][0]["attributes"];
    assert_eq!(
        attributes["indent"]["custom"],
        json!({"text":"2pt","unit":"pt","value":2.0})
    );
    assert_eq!(
        attributes["spacing"]["custom"],
        json!({"text":"3pt","unit":"pt","value":3.0})
    );
    assert_eq!(
        attributes["spacing"]["line"],
        json!({"rule":"multiple","value":1.5,"extra":false})
    );
    assert_eq!(
        attributes["numbering"],
        json!({"label":"true","prefix":"{text: x}","separator":"","extra":{"flag":true}})
    );
    for source in [
        "#heading(indent={custom=false})[Text]",
        "#paragraph(spacing={custom=-1})[Text]",
    ] {
        assert!(compile(source).is_err());
    }
    let output =
        inspection("#item(properties={marker=0},checkbox={name=false,title={1,2},extra=42})[Text]");
    assert_eq!(output["content"][0]["kind"], "ordered_list");
    assert_eq!(
        output["content"][0]["children"][0]["attributes"]["checkbox"],
        json!({"name":"false","title":"1, 2","extra":42.0})
    );
}
