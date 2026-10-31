use mote::{
    compile,
    json::to_json,
    model::{Content, Document, Element, ElementKind, Properties, Value},
};
use serde_json::json;

#[test]
fn plain_paragraph_reference_preserves_authored_model_without_output_defaults() {
    let source = include_str!("fixtures/docx/plain-paragraph/source.mote");
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/docx/plain-paragraph/model.json")).unwrap();
    let document = compile(source).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();

    assert!(document.warnings.is_empty());
    assert_eq!(actual, expected);
}

#[test]
fn json_retains_structure_types_units_and_escaped_text() {
    let document = compile("#paragraph(keep_with_next=true, spacing={after=pt(12)})[Résumé: \"paid\" #strong[yes]\\\\]").unwrap();
    let output = to_json(&document).unwrap();
    let value: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        value["content"],
        json!([{
            "kind":"paragraph", "category":"block",
            "attributes":{"keep_with_next":true, "spacing":{"after":{"value":12.0,"unit":"pt","text":"12pt"}}},
            "children":[{"kind":"text","text":"Résumé: \"paid\" "},
                {"kind":"strong","category":"inline","children":[{"kind":"text","text":"yes"}]},
                {"kind":"text","text":"\\"}]
        }])
    );
    assert_eq!(output, to_json(&document).unwrap());
}

#[test]
fn invalid_numeric_model_values_are_not_silently_emitted_as_null() {
    let document = Document {
        content: vec![Content::Element(Element {
            source_name: None,
            kind: ElementKind::Paragraph,
            attributes: Properties::from([("value".into(), Value::Number(f64::NAN))]),
            children: vec![],
        })],
        ..Document::default()
    };
    assert!(
        to_json(&document)
            .unwrap_err()
            .to_string()
            .contains("non-finite")
    );
}

#[test]
fn inspection_envelope_omits_empty_fields_and_projects_implicit_content() {
    let empty: serde_json::Value =
        serde_json::from_str(&to_json(&compile("").unwrap()).unwrap()).unwrap();
    assert_eq!(
        empty,
        json!({"page":{},"margin":{},"columns":{},"content":[]})
    );
    let document = compile("#font(name=\"z\")#font(name=\"a\",latin=\"Aptos\")#style(name=\"Z\")#style(name=\"A\",type=\"character\",based_on=\"Z\",next=\"\",link=\"Z\",font=\"a\",run={bold=true})#span[]").unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    let content = json!([{"kind":"paragraph","category":"block","children":[{"kind":"span","category":"inline"}]}]);
    assert_eq!(
        output,
        json!({
            "page":{},"margin":{},"columns":{},"content":content,
            "sections":[{"section_properties":{},"page_numbering":{},"page":{},"margin":{},"columns":{},"header_properties":{},"footer_properties":{},"content":content}],
            "fonts":[{"name":"a","latin":"Aptos","properties":{"latin":"Aptos"}},{"name":"z"}],
            "styles":[{"name":"A","type":"character","based_on":"Z","link":"Z","font":"a","run":{"bold":true},"properties":{"type":"character","based_on":"Z","next":"","link":"Z","font":"a","run":{"bold":true}}},{"name":"Z","type":"paragraph"}]
        })
    );
}

#[test]
fn text_fragment_boundaries_and_markdown_heading_markers_are_visible() {
    let document = compile("## Title\na #upper(\"b\") c\\_\nd").unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    assert_eq!(
        output["content"][0]["attributes"],
        json!({"level":2.0,"marker":"#"})
    );
    assert_eq!(
        output["content"][1]["children"],
        json!([
            {"kind":"text","text":"a "},{"kind":"text","text":"B"},
            {"kind":"text","text":" c"},{"kind":"text","text":"_"},
            {"kind":"text","text":" "},{"kind":"text","text":"d"}
        ])
    );
}
