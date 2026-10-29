#[path = "docx/support.rs"]
mod support;
use mote::{
    model::{Content, ElementKind},
    render,
    source::FileAssetResolver,
};

#[test]
fn link_forms_and_styles_match_captured_packages() {
    for case in ["external", "forms", "styles"] {
        support::compare_fixture(&format!("links-{case}"));
    }
}
#[test]
fn links_validate_arguments_and_body_before_output() {
    for source in [
        "#link()[text]",
        "#link(7)[text]",
        "#link(\"\")[text]",
        "#link(\"a\", \"b\")[text]",
        "#link(\"a\", url=\"b\")[text]",
        "#link(\"a\")",
        "#link(\"a\")[one][two]",
        "#link(\"a\")[#paragraph[block]]",
        "#link(\"a\",tooltip=7)[text]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let document = mote::compile("#link(\"primary\", properties={url=\"override\"})[]").unwrap();
    let Content::Element(paragraph) = &document.content[0] else {
        panic!()
    };
    let Content::Element(link) = &paragraph.children[0] else {
        panic!()
    };
    assert_eq!(link.kind, ElementKind::Link);
    assert_eq!(link.attributes["url"].to_text(), "primary");
    assert!(link.children.is_empty());
}
#[test]
fn wrapped_links_retain_targets_in_pdf_and_visual_text_in_png() {
    let source = include_str!("fixtures/links/external.mote");
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let planned = render::plan_with_fonts(&document, &resolver, None, None).unwrap();
    let lines = &planned.output.pages[0].lines;
    let linked: Vec<_> = lines
        .iter()
        .flat_map(|placed| placed.line.fragments.iter())
        .filter(|fragment| fragment.link.is_some())
        .collect();
    assert_eq!(linked.len(), 28);
    assert!(
        lines
            .iter()
            .filter(|placed| placed
                .line
                .fragments
                .iter()
                .any(|fragment| fragment.link.as_deref() == Some("https://example.com/a?x=1&y=2")))
            .count()
            > 1
    );
    let pdf = render::paint_pdf(&planned.output).unwrap();
    let serialized = String::from_utf8_lossy(&pdf);
    assert_eq!(serialized.matches("/Subtype /Link").count(), linked.len());
    assert!(serialized.contains("/URI (mailto:reader@example.com)"));
    assert!(serialized.contains("/Rect [35.39 248.77 67.19 264]"));
    let png = render::paint_png(&planned.output, &Default::default()).unwrap();
    assert_eq!(png.output.len(), 1);
    assert!(
        planned
            .warnings
            .iter()
            .any(|warning| warning.contains("tooltip"))
    );
}
#[test]
fn malformed_links_and_literal_code_remain_literal() {
    for source in [
        "[broken](",
        "[missing](destination",
        "[empty]()",
        "`[literal](url)`",
    ] {
        let document = mote::compile(source).unwrap();
        let resolver = FileAssetResolver::new(".").unwrap();
        let plan = render::plan_with_fonts(&document, &resolver, None, None).unwrap();
        assert!(
            plan.output
                .pages
                .iter()
                .flat_map(|page| page.text_lines())
                .flat_map(|line| &line.fragments)
                .all(|fragment| fragment.link.is_none())
        );
    }
}
