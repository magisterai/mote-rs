use mote::{
    compile,
    formatting::StyleResolver,
    json::to_json,
    model::{Content, Value},
};
use serde_json::{Value as JsonValue, json};

fn inspection(source: &str) -> JsonValue {
    serde_json::from_str(&to_json(&compile(source).unwrap()).unwrap()).unwrap()
}

#[test]
fn extra_properties_survive_every_content_directive_and_declaration() {
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
    for directive in ["font", "style"] {
        let output = inspection(&format!(
            "#{directive}(name=\"Sample\",extra=false,properties={{extra=true}})"
        ));
        assert_eq!(
            output[format!("{directive}s")][0]["properties"],
            json!({"extra":false,"properties":{"extra":true}})
        );
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

#[test]
fn declarations_keep_flat_metadata_and_resolve_textual_references() {
    let document = compile(r##"#font(name="42",latin="Aptos")
        #style(name="7",font="42",strike="true",outline_level="raw",alignment=false,spacing=false,run={font=42,strike="true",extra=false},paragraph={style=7,border=false,tabs=42})
        #span(properties={style=7})[Text]"##).unwrap();
    let properties = &document.styles["7"].properties;
    assert_eq!(properties["strike"], Value::String("true".into()));
    assert_eq!(properties["spacing"], Value::Boolean(false));
    let Value::Object(run) = &properties["run"] else {
        panic!("expected run properties")
    };
    assert_eq!(run["strike"], Value::Boolean(true));
    assert_eq!(run["font"], Value::Number(42.0));
    let Content::Element(paragraph) = &document.content[0] else {
        panic!("expected paragraph")
    };
    let Content::Element(span) = &paragraph.children[0] else {
        panic!("expected span")
    };
    let resolved = StyleResolver::new(&document).resolve_inline(span, &Default::default());
    assert_eq!(resolved.font.unwrap().latin.as_deref(), Some("Aptos"));
    assert!(compile("#style(name=\"\")").is_err());
    assert!(compile("#style(name=\"\",based_on=\"Other\")").is_ok());
    assert!(compile("#style(name=\" \" )").is_ok());
}

#[test]
fn table_style_storage_normalizes_geometry_regions_and_banding() {
    let output = inspection(
        r##"#style(name="Grid",type="table",table={
        width="100pt",indent="2pt",cell_spacing="1pt",alignment="center",layout="fixed",
        cell={margin={left="3pt"},fill="#ABC",no_wrap="true",vertical_alignment="middle",text_direction="tb_rl",extra=1},
        paragraph={spacing={after="4pt"}},text={bold="true",size="11pt"},
        first_row={fill="#def",margin={top="1pt"},no_wrap="false",alignment="right",vertical_alignment="bottom",text_direction="rtl",text={italic="true"},paragraph={keep_with_next="true"},cell={fill="#123"}},
        last_row={fill="#fff"},first_column={fill="#000"},last_column={fill="#111"},
        banding={rows="true",columns=false,odd={fill="#aaa"},even={fill="#bbb"},odd_column={fill="#ccc"},even_column={fill="#ddd"},extra="kept"},extra={flag=true}
    })"##,
    );
    let declaration = &output["styles"][0];
    assert_eq!(declaration["type"], "table");
    let table = &declaration["table"];
    assert_eq!(table, &declaration["properties"]["table"]);
    for (name, amount) in [("width", 100.0), ("indent", 2.0), ("cell_spacing", 1.0)] {
        assert_eq!(
            table[name],
            json!({"value":amount,"unit":"pt","text":format!("{amount}pt")})
        );
    }
    assert_eq!(
        table["cell"],
        json!({"margin":{"left":{"value":3.0,"unit":"pt","text":"3pt"}},"fill":"#aabbcc","no_wrap":true,"vertical_alignment":"center","text_direction":"tb_rl","extra":1.0})
    );
    assert_eq!(table["paragraph"]["spacing"]["after"]["text"], "4pt");
    assert_eq!(table["text"]["bold"], true);
    assert_eq!(table["text"]["size"]["text"], "11pt");
    assert_eq!(table["first_row"]["fill"], "#ddeeff");
    assert_eq!(table["first_row"]["no_wrap"], false);
    assert_eq!(table["first_row"]["margin"]["top"]["text"], "1pt");
    assert_eq!(table["first_row"]["paragraph"]["keep_with_next"], true);
    assert_eq!(table["first_row"]["text"]["italic"], true);
    assert_eq!(table["first_row"]["cell"]["fill"], "#112233");
    assert_eq!(table["last_row"]["fill"], "#ffffff");
    assert_eq!(table["first_column"]["fill"], "#000000");
    assert_eq!(table["last_column"]["fill"], "#111111");
    assert_eq!(
        table["banding"],
        json!({"rows":true,"columns":false,"odd":{"fill":"#aaaaaa"},"even":{"fill":"#bbbbbb"},"odd_column":{"fill":"#cccccc"},"even_column":{"fill":"#dddddd"},"extra":"kept"})
    );
    assert_eq!(table["extra"], json!({"flag":true}));
}

#[test]
fn table_style_storage_rejects_invalid_normalized_fields() {
    for properties in [
        "width=-1",
        "cell_padding=1",
        "layout=\"wide\"",
        "alignment=\"Center\"",
        "cell=false",
        "cell={margin={left=-1}}",
        "cell={no_wrap=1}",
        "cell={vertical_alignment=\"baseline\"}",
        "first_row=false",
        "first_row={text=false}",
        "first_row={paragraph=false}",
        "first_row={cell=false}",
        "first_row={text_direction=\"vertical\"}",
        "first_row={fill=\"red\"}",
        "banding=false",
        "banding={rows=1}",
        "banding={odd=false}",
    ] {
        let source = format!("#style(name=\"Grid\",table={{{properties}}})");
        assert!(compile(&source).is_err(), "{source}");
    }
}
