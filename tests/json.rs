use mote::{
    compile,
    json::to_json,
    model::{Content, Document, Element, ElementKind, Properties, Value},
};
use serde_json::json;

#[test]
fn json_retains_structure_types_units_and_escaped_text() {
    let document = compile("#paragraph(keep_with_next=true, spacing={after=pt(12)})[Résumé: \"paid\" #strong[yes]\\\\]").unwrap();
    let output = to_json(&document).unwrap();
    let value: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        value,
        json!({"content":[{
            "kind":"paragraph", "category":"block",
            "attributes":{"keep_with_next":true, "spacing":{"after":{"value":12.0,"unit":"pt","text":"12pt"}}},
            "children":[{"kind":"text","text":"Résumé: \"paid\" "},
                {"kind":"strong","category":"inline","children":[{"kind":"text","text":"yes"}]},
                {"kind":"text","text":"\\"}]
        }]})
    );
    assert_eq!(output, to_json(&document).unwrap());
}

#[test]
fn invalid_numeric_model_values_are_not_silently_emitted_as_null() {
    let document = Document {
        content: vec![Content::Element(Element {
            kind: ElementKind::Paragraph,
            attributes: Properties::from([("value".into(), Value::Number(f64::NAN))]),
            children: vec![],
        })],
    };
    assert!(
        to_json(&document)
            .unwrap_err()
            .to_string()
            .contains("non-finite")
    );
}
