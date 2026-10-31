#[path = "support/language_capture.rs"]
mod language_capture;

#[test]
fn semantic_wrappers_match_models_editable_packages_and_native_rasters() {
    language_capture::compare("semantic-formatting", &["document", "slides", "overrides"]);
}

#[test]
fn formatting_contracts_reject_invalid_values_and_keep_output_limits_explicit() {
    for source in [
        "#underline(style=\"wavy\")[Text]",
        "#strike(kind=\"none\")[Text]",
        "#highlight(fill=\"bad\")[Text]",
        "#superscript()",
        "#subscript[\n#paragraph[Block]\n]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    for directive in [
        "strike",
        "highlight",
        "small_caps",
        "subscript",
        "superscript",
    ] {
        let document = mote::compile(&format!("#{directive}(size=pt(16))[Text]")).unwrap();
        assert!(
            mote::docx::emit(&document).is_err(),
            "{directive}: no direct size mapping"
        );
    }
}

#[test]
fn semantic_attributes_use_the_shared_value_and_override_contracts() {
    use mote::model::{Content, Value};
    for directive in [
        "underline",
        "strike",
        "highlight",
        "small_caps",
        "subscript",
        "superscript",
    ] {
        let source = format!(
            "#{directive}(font=\"body\",size=\"12pt\",color=\"#abc\",properties={{size=pt(16),future=true}})[Text]"
        );
        let document = mote::compile(&source).unwrap();
        let Content::Element(paragraph) = &document.content[0] else {
            panic!("paragraph")
        };
        let Content::Element(element) = &paragraph.children[0] else {
            panic!("inline")
        };
        assert_eq!(element.attributes["size"].to_text(), "16pt");
        assert_eq!(element.attributes["color"].to_text(), "#aabbcc");
        assert_eq!(element.attributes["font"].to_text(), "body");
        assert_eq!(element.attributes["future"], Value::Boolean(true));
        for argument in ["font=12", "style=12", "size=-1", "color=true"] {
            assert!(mote::compile(&format!("#{directive}({argument})[Text]")).is_err());
        }
    }
}
