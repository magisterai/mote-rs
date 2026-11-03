#[path = "support/language_capture.rs"]
mod language_capture;

#[test]
fn authored_content_matches_editable_packages_and_native_layout() {
    language_capture::compare(
        "authoring-content",
        &[
            "markdown-strike",
            "setext-heading",
            "styled-list-paragraph",
            "styled-quote-paragraph",
            "quoted-list-content",
            "nested-list-blocks",
            "nested-markdown-blocks",
            "table-quote-paragraphs",
            "table-definition-paragraphs",
            "table-list-line-spacing",
            "table-authored-spacing",
            "long-url",
            "slides",
        ],
    );
}

#[test]
fn markdown_strike_preserves_escapes_literal_code_and_unmatched_delimiters() {
    use mote::model::{Content, ElementKind};
    let document = mote::compile(
        r"Before ~~removed **bold**~~; \~\~literal\~\~, `~~code~~`, and ~~unfinished.",
    )
    .unwrap();
    let Content::Element(paragraph) = &document.content[0] else {
        panic!("paragraph")
    };
    let strikes: Vec<_> = paragraph
        .children
        .iter()
        .filter_map(|child| match child {
            Content::Element(element) if element.kind == ElementKind::Strike => Some(element),
            _ => None,
        })
        .collect();
    assert_eq!(strikes.len(), 1);
    assert_eq!(
        strikes[0]
            .children
            .iter()
            .map(Content::to_text)
            .collect::<String>(),
        "removed bold"
    );
    assert!(strikes[0].children.iter().any(
        |child| matches!(child, Content::Element(element) if element.kind == ElementKind::Strong)
    ));
    assert_eq!(
        paragraph
            .children
            .iter()
            .map(Content::to_text)
            .collect::<String>(),
        "Before removed bold; ~~literal~~, ~~code~~, and ~~unfinished."
    );
}

#[test]
fn table_list_spacing_preserves_item_spacing_and_paragraph_overrides() {
    use mote::{render, source::FileAssetResolver};
    let resolver = FileAssetResolver::new(".").unwrap();
    let source = include_str!("fixtures/authoring-content/table-list-line-spacing.mote");
    for (source, distance) in [
        (source.to_owned(), 14.0),
        (source.replace("#list[", "#list(tight=true)["), 12.0),
        (
            source.replace("#item[", "#item(spacing={after=pt(7)})["),
            20.14933,
        ),
    ] {
        let document = mote::compile(&source).unwrap();
        let plan = render::plan(&document, &resolver, None).unwrap();
        let lines = &plan.output.pages[0].lines;
        assert!((lines[1].top - lines[0].top - distance).abs() < 1e-8);
        assert!((lines[2].top - lines[1].top - distance).abs() < 1e-8);
    }
}
