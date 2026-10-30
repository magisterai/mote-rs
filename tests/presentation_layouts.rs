use mote::{
    compile,
    model::{Content, ElementKind},
    render,
    source::FileAssetResolver,
};

#[test]
fn named_slots_reject_unknown_duplicate_mistyped_and_misplaced_assignments() {
    for (source, expected) in [
        ("#slide(layout='unknown')[]", "unknown presentation layout"),
        (
            "#slide(layout='blank')[#box(slot='title')[Text]]",
            "no slot named",
        ),
        (
            "#slide(layout='title_slide')[#box(slot='title')[One]#box(slot='title')[Two]]",
            "more than once",
        ),
        (
            "#slide(layout='image_and_caption')[#box(slot='image')[Text]]",
            "requires image",
        ),
        (
            "#slide(layout='title_slide')[#box(slot='title')[#image(src='image.png')]]",
            "requires text",
        ),
        (
            "#slide(layout='title_slide')[Loose text]",
            "non-positioned content",
        ),
        ("#box(slot='title')[Text]", "direct child"),
        ("#slide[#box(slot='title')[Text]]", "direct child"),
        (
            "#slide(layout='title_slide')[#blockquote[#box(slot='title')[Text]]]",
            "non-positioned content",
        ),
        (
            "#slide(layout='title_slide')[#box(slot='')[Text]]",
            "requires position",
        ),
        ("#slide(layout=3)[]", "string"),
        ("#slide(layout=' ')[]", "must not be empty"),
    ] {
        let error = compile(source).unwrap_err();
        assert!(error.message.contains(expected), "{source}: {error:?}");
    }
}

#[test]
fn optional_slots_keep_authored_models_and_layout_order_in_outputs() {
    let document = compile("#presentation(size='standard')#slide(layout='title_slide')[#box(slot='subtitle')[Second]#box(slot='title')[First]]#slide(layout='blank')[]").unwrap();
    let original = document.clone();
    let resolver = FileAssetResolver::new(".").unwrap();
    let plan =
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    assert_eq!(plan.output.pages.len(), 2);
    let text: Vec<_> = plan.output.pages[0]
        .text_lines()
        .flat_map(|line| line.fragments.iter().map(|fragment| fragment.text.as_str()))
        .collect();
    assert_eq!(text, ["First", "Second"]);
    assert_eq!(document, original);
    let Content::Element(slide) = &document.content[0] else {
        panic!("expected slide")
    };
    assert_eq!(slide.kind, ElementKind::Slide);
    let Content::Element(first) = &slide.children[0] else {
        panic!("expected box")
    };
    assert_eq!(first.attributes["slot"].to_text(), "subtitle");
    assert!(!first.attributes.contains_key("position"));
    assert!(plan.output.pages[1].text_lines().next().is_none());
}
