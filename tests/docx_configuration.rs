#[path = "docx/support.rs"]
mod support;

const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[test]
fn page_geometry_matches_captured_models_and_packages() {
    for name in ["page-geometry", "page-overrides", "page-custom"] {
        support::compare_fixture(name);
    }
}

#[test]
fn named_sizes_and_aliases_map_to_expected_twips() {
    for (name, width, height) in [
        ("letter", "12240", "15840"),
        ("us_letter", "12240", "15840"),
        ("legal", "12240", "20160"),
        ("us_legal", "12240", "20160"),
        ("tabloid", "15840", "24480"),
        ("ledger", "15840", "24480"),
        ("a3", "16838", "23811"),
        ("A4", "11906", "16838"),
        ("a5", "8391", "11906"),
        ("custom", "12240", "15840"),
    ] {
        let package = support::package(&format!(
            "#document(page = {{ size = \"{name}\" }})\n#paragraph[Text]"
        ));
        let document = roxmltree::Document::parse(&package["word/document.xml"]).unwrap();
        let page = document
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "pgSz")))
            .unwrap();
        assert_eq!(page.attribute((WORD_NAMESPACE, "w")), Some(width), "{name}");
        assert_eq!(
            page.attribute((WORD_NAMESPACE, "h")),
            Some(height),
            "{name}"
        );
        assert!(page.attribute((WORD_NAMESPACE, "orient")).is_none());
    }
}

#[test]
fn portrait_preserves_explicit_dimensions_and_landscape_swaps_only_when_needed() {
    for (orientation, authored_width, authored_height, width, height) in [
        ("portrait", 1000, 500, "1000", "500"),
        ("landscape", 1000, 500, "1000", "500"),
        ("landscape", 500, 1000, "1000", "500"),
        ("landscape", 0, 0, "0", "0"),
    ] {
        let package = support::package(&format!(
            "#document(page = {{ width = {authored_width}, height = {authored_height}, orientation = \"{orientation}\" }})"
        ));
        let document = roxmltree::Document::parse(&package["word/document.xml"]).unwrap();
        let page = document
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "pgSz")))
            .unwrap();
        assert_eq!(page.attribute((WORD_NAMESPACE, "w")), Some(width));
        assert_eq!(page.attribute((WORD_NAMESPACE, "h")), Some(height));
    }
}

#[test]
fn unsupported_properties_and_overflow_fail_without_partial_packages() {
    for source in [
        "#document(default_size = 12)",
        "#document(page = { code = 1 })",
        "#document(margin = { inside = 20 })",
        "#document(page_numbering = { start = 2 })",
        "#document(page = { width = 999999999999 })",
        "#document(margin = { top = inch(999999999999) })",
    ] {
        let document = mote::compile(source).unwrap();
        assert!(mote::docx::emit(&document).is_err(), "{source}");
    }
}
