#[path = "docx/support.rs"]
mod support;

use mote::{compile, json::to_json};
const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[test]
fn block_breaks_match_captured_models_and_packages() {
    for name in ["block-breaks", "column-breaks", "consecutive-breaks"] {
        support::compare_fixture(name);
    }
}

#[test]
fn break_defaults_and_property_precedence_are_visible_in_the_model() {
    let document = compile(
        r#"#break()#break(type = "page", properties = { type = "column", note = "kept" })"#,
    )
    .unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    assert_eq!(output["content"][0]["kind"], "break");
    assert_eq!(output["content"][0]["category"], "block");
    assert_eq!(output["content"][0]["attributes"]["type"], "page");
    assert_eq!(output["content"][1]["attributes"]["type"], "column");
    assert_eq!(output["content"][1]["attributes"]["note"], "kept");
    support::assert_ignored(
        r#"#break()#break(type = "page", properties = { type = "column", note = "kept" })"#,
        "break.note",
    );
}

#[test]
fn invalid_break_calls_and_inline_placement_are_rejected() {
    for source in [
        "#break(1)",
        "#break()[text]",
        "#break(type = false)",
        "#break(type = false, properties = { type = \"page\" })",
        "#break(type = \"unknown\")",
        "#break(type = \"section\")",
        "#break(properties = { type = 1 })",
        "#paragraph[Text#break()]",
        "#span[Text#break(type = \"line\")]",
        "#list[#break()]",
    ] {
        assert!(compile(source).is_err(), "{source}");
    }
}

#[test]
fn pending_column_breaks_survive_empty_consecutive_and_terminal_positions() {
    for (source, paragraphs, breaks) in [
        ("#break(type = \"column\")", 1, 1),
        ("#break(type = \"column\")#paragraph[]", 1, 1),
        ("#break(type = \"column\")#break(type = \"column\")", 2, 2),
        ("#break(type = \"column\")#break()#paragraph[Text]", 3, 2),
        ("#paragraph[Text]#break(type = \"column\")", 2, 1),
    ] {
        let package = support::package(source);
        let document = roxmltree::Document::parse(&package["word/document.xml"]).unwrap();
        assert_eq!(
            document
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "p")))
                .count(),
            paragraphs,
            "{source}"
        );
        assert_eq!(
            document
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "br")))
                .count(),
            breaks,
            "{source}"
        );
    }
}

#[test]
fn column_start_suppresses_only_implicit_heading_spacing() {
    for (attributes, expected) in [
        ("1", Some("0")),
        ("1, spacing_before = pt(8)", Some("160")),
        ("1, spacing = { before = pt(9) }", Some("180")),
    ] {
        let package = support::package(&format!(
            "#paragraph[Before]#break(type = \"column\")#heading({attributes})[Heading]"
        ));
        let document = roxmltree::Document::parse(&package["word/document.xml"]).unwrap();
        let heading = document
            .descendants()
            .filter(|node| node.has_tag_name((WORD_NAMESPACE, "p")))
            .nth(1)
            .unwrap();
        let spacing = heading
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "spacing")))
            .unwrap();
        assert_eq!(spacing.attribute((WORD_NAMESPACE, "before")), expected);
        assert_eq!(
            heading
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "br")))
                .count(),
            1
        );
    }
}

#[test]
fn nested_break_output_fails_instead_of_disappearing() {
    for source in [
        "#blockquote[Before#break()After]",
        "#list[#item[Before#break()After]]",
    ] {
        let document = compile(source).unwrap();
        assert!(mote::docx::emit(&document).is_err(), "{source}");
    }
}

const PAGE_BREAK_ALONE: &str = "<w:p><w:r><w:br w:type=\"page\"/></w:r></w:p>";

#[test]
fn a_page_break_after_a_paragraph_or_a_heading_is_written_at_its_end() {
    // A paragraph that holds a page break alone needs a line of room, and Word
    // leaves a page empty where the page before the break has none.
    for source in [
        "#paragraph[Before]\n#break()\n#paragraph[After]",
        "# Before\n\n#break()\n\n#paragraph[After]",
        "#paragraph[Before]\n#break()",
    ] {
        let document = &support::package(source)["word/document.xml"];
        assert!(
            document.contains("Before</w:t></w:r><w:r><w:br w:type=\"page\"/></w:r></w:p>"),
            "{source}"
        );
        assert!(!document.contains(PAGE_BREAK_ALONE), "{source}");
    }
}

#[test]
fn a_page_break_after_another_block_is_written_in_a_paragraph_of_its_own() {
    for (source, alone) in [
        ("#break()\n#paragraph[After]", 1),
        ("#table[#row[#cell[Cell]]]\n#break()\n#paragraph[After]", 1),
        // The page between two breaks stays empty.
        (
            "#paragraph[Before]\n#break()\n#break()\n#paragraph[After]",
            1,
        ),
        ("- Item\n\n#break()\n\n#paragraph[After]", 1),
    ] {
        let document = &support::package(source)["word/document.xml"];
        assert_eq!(
            document.matches(PAGE_BREAK_ALONE).count(),
            alone,
            "{source}"
        );
        assert_eq!(
            document.matches("<w:br w:type=\"page\"/>").count(),
            source.matches("#break()").count(),
            "{source}"
        );
    }
}

/// Returns the text and the top of each line of each page of a Letter
/// document with one-inch margins.
fn lines(source: &str) -> Vec<Vec<(String, f64)>> {
    let resolver = mote::source::FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let document = compile(source).unwrap();
    let planned = mote::render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    planned
        .output
        .pages
        .iter()
        .map(|page| {
            page.lines
                .iter()
                .map(|placed| {
                    let text = placed
                        .line
                        .fragments
                        .iter()
                        .map(|fragment| fragment.text.as_str())
                        .collect();
                    (text, placed.top)
                })
                .collect()
        })
        .collect()
}

#[test]
fn the_block_after_a_page_break_lies_at_the_head_of_its_page_without_the_space_before_it() {
    let after = "spacing = { before = pt(30), after = pt(20) }";
    for source in [
        format!("#paragraph[Before]\n#break()\n#paragraph({after})[After]"),
        format!("#paragraph[Before]\n#paragraph(page_break_before = true, {after})[After]"),
    ] {
        let pages = lines(&source);
        assert_eq!(pages.len(), 2, "{source}");
        assert_eq!(pages[1][0], ("After".to_owned(), 72.0), "{source}");
    }
    // The first paragraph of a document keeps the space before it.
    let pages = lines(&format!("#paragraph({after})[After]"));
    assert_eq!(pages[0][0], ("After".to_owned(), 102.0));
}

#[test]
fn a_block_line_break_is_an_empty_paragraph() {
    let broken = "#paragraph[Before]\n#break(type = \"line\")\n#paragraph[After]";
    let empty = "#paragraph[Before]\n#paragraph[]\n#paragraph[After]";
    // A paragraph that holds a line break alone occupies two lines in Word.
    let document = &support::package(broken)["word/document.xml"];
    assert!(document.contains("</w:p><w:p/><w:p>"));
    assert!(!document.contains("<w:br/>"));
    let tops = |source: &str| -> Vec<f64> {
        lines(source)[0]
            .iter()
            .filter(|(text, _)| !text.is_empty())
            .map(|(_, top)| *top)
            .collect()
    };
    assert_eq!(tops(broken), tops(empty));
}

#[test]
fn two_adjacent_tables_are_parted_by_an_empty_paragraph() {
    let table = |name: &str| format!("#table[#row[#cell[{name}]]]\n");
    let tops = |between: &str| -> Vec<f64> {
        lines(&format!("{}{between}{}", table("First"), table("Second")))[0]
            .iter()
            .filter(|(text, _)| !text.is_empty())
            .map(|(_, top)| *top)
            .collect()
    };
    // DOCX writes the paragraph, without which Word would join the tables.
    assert_eq!(tops(""), tops("#paragraph[]\n"));
    let document =
        &support::package(&format!("{}{}", table("First"), table("Second")))["word/document.xml"];
    assert!(document.contains("</w:tbl><w:p/><w:tbl>"));
}
