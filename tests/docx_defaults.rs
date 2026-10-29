#[path = "docx/support.rs"]
mod support;

const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[test]
fn document_defaults_match_captured_models_and_packages() {
    for name in [
        "document-defaults",
        "default-overrides",
        "default-script-fonts",
    ] {
        support::compare_fixture(name);
    }
}

#[test]
fn default_size_conversion_preserves_typed_and_string_rounding_rules() {
    for (value, expected) in [
        ("12.25", "25"),
        ("pt(12.24)", "24"),
        ("\"12.24pt\"", "25"),
        ("\"12.25\"", "25"),
        ("inch(0.125)", "18"),
        ("cm(0.254)", "14"),
        ("mm(2.54)", "14"),
        ("twip(245)", "25"),
    ] {
        let package = support::package(&format!(
            "#document(default_size = {value})\n#paragraph[Text]"
        ));
        let document = roxmltree::Document::parse(&package["word/styles.xml"]).unwrap();
        let defaults = document
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "docDefaults")))
            .unwrap();
        for property in ["sz", "szCs"] {
            let size = defaults
                .descendants()
                .find(|node| node.has_tag_name((WORD_NAMESPACE, property)))
                .unwrap();
            assert_eq!(
                size.attribute((WORD_NAMESPACE, "val")),
                Some(expected),
                "{value}"
            );
        }
        assert!(
            !package["word/document.xml"].contains("<w:sz"),
            "defaults must remain inherited"
        );
    }
}

#[test]
fn default_mono_alias_and_language_use_existing_font_and_xml_rules() {
    let package = support::package(r#"#document(default_font = "mono", language = "en&\"GB")"#);
    let document = roxmltree::Document::parse(&package["word/styles.xml"]).unwrap();
    let fonts = document
        .descendants()
        .find(|node| node.has_tag_name((WORD_NAMESPACE, "rFonts")))
        .unwrap();
    assert_eq!(
        fonts.attribute((WORD_NAMESPACE, "ascii")),
        Some("Courier New")
    );
    let language = document
        .descendants()
        .find(|node| node.has_tag_name((WORD_NAMESPACE, "lang")))
        .unwrap();
    assert_eq!(language.attribute((WORD_NAMESPACE, "val")), Some("en&\"GB"));
    let package = support::package(r#"#document(default_font = "", language = "")"#);
    assert!(package["word/styles.xml"].contains("w:ascii=\"Helvetica\""));
    assert!(!package["word/styles.xml"].contains("<w:lang"));
}

#[test]
fn invalid_default_values_fail_at_output_without_rewriting_authored_arguments() {
    for fields in [
        "default_font = true",
        "default_size = 0",
        "default_size = 0.1",
        "default_size = -12",
        "default_size = true",
        "default_size = \"large\"",
        "default_size = \"1in\"",
        "default_size = 9999999999999",
        "language = 123",
        "default_color = \"#ff0000\"",
    ] {
        let document = mote::compile(&format!("#document({fields})")).unwrap();
        let before = document.clone();
        assert!(mote::docx::emit(&document).is_err(), "{fields}");
        assert_eq!(before, document);
    }
}
