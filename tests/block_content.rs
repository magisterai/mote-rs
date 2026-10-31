#[path = "support/language_capture.rs"]
mod language_capture;
use mote::model::{Content, ElementKind};

#[test]
fn literal_blocks_definitions_and_rules_match_captured_outputs() {
    language_capture::compare("block-content", &["document", "slides"]);
}

#[test]
fn literal_fences_preserve_content_and_unclosed_fences_remain_text() {
    for source in [
        "```text\n  #paragraph[literal]\tend\n\n```",
        "~~~text\r\n  #paragraph[literal]\tend\r\n\r\n~~~",
    ] {
        let document = mote::compile(source).unwrap();
        let Content::Element(element) = &document.content[0] else {
            panic!("raw block");
        };
        assert_eq!(element.kind, ElementKind::Raw);
        assert_eq!(
            element.children,
            vec![Content::Text("  #paragraph[literal]\tend\n\n".into())]
        );
        assert_eq!(element.attributes["language"].to_text(), "text");
    }
    for source in ["```\nUnclosed", "~~~\nUnclosed"] {
        let document = mote::compile(source).unwrap();
        let Content::Element(element) = &document.content[0] else {
            panic!("paragraph");
        };
        assert_eq!(element.kind, ElementKind::Paragraph);
    }
    let document = mote::compile("#raw(12)\n#raw[body]\n#raw()").unwrap();
    assert_eq!(document.content.len(), 3);
}

#[test]
fn definition_lists_validate_children_and_keep_each_table_cell_paragraph() {
    for source in [
        "#definition_list[text]",
        "#definition_list[#paragraph[wrong]]",
        "#term[#paragraph[block]]",
        "#divider[body]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let empty = mote::compile("#definition_list[]").unwrap();
    assert!(
        empty
            .warnings
            .iter()
            .any(|warning| warning.message.contains("empty"))
    );
    let source = "#table[#row[#cell[#definition_list[#term[Term]#definition[First.\n\nSecond.]]]]]";
    let document = mote::compile(source).unwrap();
    let bytes = mote::docx::emit(&document).unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    use std::io::Read;
    archive
        .by_name("word/document.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    let tree = roxmltree::Document::parse(&xml).unwrap();
    let paragraphs: Vec<_> = tree
        .descendants()
        .filter(|node| {
            node.has_tag_name((
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "p",
            ))
        })
        .collect();
    assert_eq!(paragraphs.len(), 3);
    let texts: Vec<String> = paragraphs
        .iter()
        .map(|node| {
            node.descendants()
                .filter_map(|child| child.text().filter(|_| child.is_text()))
                .collect()
        })
        .collect();
    assert_eq!(texts, ["Term", "First.", "Second."]);
}

#[test]
fn block_attribute_and_markdown_contracts_are_explicit() {
    for source in ["---", "***", "___"] {
        let document = mote::compile(source).unwrap();
        assert!(
            matches!(&document.content[0], Content::Element(element) if element.kind == ElementKind::Divider)
        );
    }
    let nested = mote::compile("#blockquote[---]").unwrap();
    let Content::Element(quote) = &nested.content[0] else {
        panic!("quote")
    };
    assert!(
        matches!(&quote.children[0], Content::Element(element) if element.kind == ElementKind::Divider)
    );
    let heading = mote::compile("Title\n---").unwrap();
    assert!(
        matches!(&heading.content[0], Content::Element(element) if element.kind == ElementKind::Heading)
    );
    for source in [
        "#raw(language=12)",
        "#raw(preserve_space=\"yes\")",
        "#raw[#heading[block]]",
        "#divider(width=-1)",
        "#divider(color=\"bad\")",
        "#definition_list(style=12)[]",
        "#definition(style=12)[body]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let document =
        mote::compile("#definition_list[#term(style=12)[Term]#definition[Definition]]").unwrap();
    let Content::Element(list) = &document.content[0] else {
        panic!("list")
    };
    let Content::Element(term) = &list.children[0] else {
        panic!("term")
    };
    assert_eq!(term.attributes["style"].to_text(), "12");
}

#[test]
fn raw_run_properties_are_native_and_slide_controls_with_word_limits() {
    let document = mote::compile(
        "#raw(\"Literal\",font=\"Courier\",size=pt(18),color=\"#2563eb\",preserve_space=true)",
    )
    .unwrap();
    assert!(mote::docx::emit(&document).is_err());
    let resolver = mote::source::FileAssetResolver::new(env!("CARGO_MANIFEST_DIR")).unwrap();
    let planned = mote::render::plan(&document, &resolver, None).unwrap();
    let fragment = &planned.output.pages[0].lines[0].line.fragments[0];
    assert_eq!(fragment.style.measurement.size_points, 18.0);
    assert_eq!(fragment.style.color, "2563EB");
    let planned = mote::render::plan_with_layout(
        &document,
        &resolver,
        None,
        mote::render::LayoutMode::Presentation,
    )
    .unwrap();
    assert!(
        planned
            .warnings
            .iter()
            .all(|warning| !warning.contains("preserve_space"))
    );
    assert_eq!(
        planned.output.pages[0].lines[0].line.fragments[0]
            .style
            .measurement
            .size_points,
        18.0
    );
}
