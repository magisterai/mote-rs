#[path = "support/language_capture.rs"]
mod language_capture;

#[test]
fn authored_content_matches_editable_packages_and_native_layout() {
    language_capture::compare(
        "authoring-content",
        &["markdown-strike", "setext-heading", "slides"],
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
