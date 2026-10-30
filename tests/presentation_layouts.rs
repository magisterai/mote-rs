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

#[test]
fn custom_declarations_preserve_order_and_explicit_optional_overrides() {
    let document = compile(r#"
#layout(name="derived",based_on="base")[#box(slot="title",required=false)#box(slot="old",remove=true)]
#layout(name="base")[#box(slot="superseded",position={width=1,height=1})]
#layout(name="base",based_on="blank")[#box(slot="title",kind="text",required=true,position={width=1,height=1})#box(slot="old",kind="any",position={width=1,height=1})]
#slide(layout="derived")[]
"#).unwrap();
    let declarations = &document.presentation.layouts;
    assert_eq!(
        declarations
            .iter()
            .map(|layout| layout.name.as_str())
            .collect::<Vec<_>>(),
        ["derived", "base"]
    );
    assert_eq!(declarations[0].slots[0].required, Some(false));
    assert!(declarations[0].slots[1].remove);
    assert_eq!(declarations[1].slots[0].required, Some(true));
    assert_eq!(declarations[1].slots[1].required, None);
    assert_eq!(declarations[1].slots[1].kind, "content");
    assert_eq!(declarations[1].slots[0].name, "title");
}

#[test]
fn invalid_custom_declarations_and_assignments_fail_before_output() {
    for (source, expected) in [
        ("#layout()", "requires name"),
        ("#layout('custom')[]", "named arguments"),
        ("#layout(name=' ')[]", "must not be empty"),
        ("#layout(name='title_slide')[]", "reserved"),
        ("#layout(name='custom',based_on=3)[]", "string"),
        (
            "#layout(name='custom',based_on='missing')[]",
            "unknown presentation layout",
        ),
        (
            "#layout(name='first',based_on='second')[]#layout(name='second',based_on='first')[]",
            "cycle",
        ),
        ("#layout(name='custom')[Text]", "only contain box"),
        (
            "#layout(name='custom')[#box(slot='title')#box(slot='title')]",
            "more than once",
        ),
        (
            "#layout(name='custom')[#box(slot='title')[Default content]]",
            "must not contain content",
        ),
        (
            "#layout(name='custom')[#box(slot='',position={width=1,height=1})]",
            "non-empty slot",
        ),
        (
            "#layout(name='custom')[#box(slot='title',kind='video')]",
            "layout slot kind",
        ),
        (
            "#layout(name='custom')[#box(slot='title')]",
            "no resolved position",
        ),
        (
            "#layout(name='custom')[#box(slot='title',remove=true)]",
            "unknown inherited slot",
        ),
        (
            "#layout(name='custom',based_on='title_slide')[#box(slot='title',remove=true,required=false)]",
            "cannot set properties",
        ),
        (
            "#layout(name='custom',based_on='title_slide')[#box(slot='title',remove=true,fill='#fff')]",
            "cannot set properties",
        ),
        (
            "#layout(name='custom',based_on='title_slide')[#box(slot='title',required=true)]#slide(layout='custom')[]",
            "missing required slot",
        ),
        (
            "#slide(layout='title_slide')[#box(slot='title',required=false)[Text]]",
            "only valid in layout declarations",
        ),
        (
            "#box(kind='text',position={width=1,height=1})[Text]",
            "only valid in layout declarations",
        ),
        (
            "#layout(name='custom',footer=1,properties={footer='Text'})[]",
            "footer",
        ),
    ] {
        let error = compile(source).unwrap_err();
        assert!(error.message.contains(expected), "{source}: {error:?}");
    }
}

#[test]
fn inheritance_resolves_long_forward_chains_without_recursive_stack_growth() {
    let mut source = String::new();
    for index in 0..1000 {
        source.push_str(&format!(
            "#layout(name='layout_{index}',based_on='layout_{}')\n",
            index + 1
        ));
    }
    source.push_str("#layout(name='layout_1000',based_on='title_slide')\n#slide(layout='layout_0')[#box(slot='title')[Title]]");
    let document = compile(&source).unwrap();
    assert_eq!(document.presentation.layouts.len(), 1001);
    assert!(mote::pptx::emit(&document).is_ok());
}

#[test]
fn inherited_backgrounds_use_the_callers_asset_resolver_and_logical_source() {
    use std::{cell::RefCell, ffi::OsStr};
    let requests = RefCell::new(Vec::new());
    let resolver = |request: mote::source::AssetRequest<'_>| {
        requests.borrow_mut().push((
            request.reference.to_owned(),
            request.source_name.map(OsStr::to_owned),
        ));
        assert_eq!(request.reference, OsStr::new("pattern.png"));
        Ok(mote::source::ResolvedAsset {
            name: "memory:pattern".into(),
            bytes: include_bytes!("fixtures/images/assets/pattern.png").to_vec(),
        })
    };
    let source_name = OsStr::new("memory://slides/deck.mote");
    let document = compile(r##"#presentation(size="standard")#layout(name="base",background={src="pattern.png",fit="contain",color="#fff"})#layout(name="derived",based_on="base",background={color="#eff6ff"})#slide(layout="derived")[]"##).unwrap();
    let original = document.clone();
    let plan = render::plan_with_layout(
        &document,
        &resolver,
        Some(source_name),
        render::LayoutMode::Presentation,
    )
    .unwrap();
    assert!(
        !plan
            .warnings
            .iter()
            .any(|warning| warning.contains("image"))
    );
    assert!(
        !render::paint_png(&plan.output, &Default::default())
            .unwrap()
            .output
            .is_empty()
    );
    mote::pptx::emit_with_assets(&document, &resolver, Some(source_name)).unwrap();
    assert_eq!(requests.borrow().len(), 2);
    assert!(
        requests
            .borrow()
            .iter()
            .all(|(_, source)| source.as_deref() == Some(source_name))
    );
    assert_eq!(document, original);
}

#[test]
fn inherited_unimplemented_properties_keep_output_diagnostics() {
    let document = compile("#presentation(size='standard')#layout(name='custom',based_on='title_slide',mystery=true)[#box(slot='title',text={language='fr'})]#slide(layout='custom')[#box(slot='title')[Text]]").unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    for warnings in [
        mote::pptx::emit_with_warnings(&document).unwrap().warnings,
        render::plan(&document, &resolver, None).unwrap().warnings,
    ] {
        assert!(warnings.iter().any(|warning| warning.contains("mystery")));
        assert!(warnings.iter().any(|warning| warning.contains("language")));
    }
}

#[test]
fn empty_lua_box_round_trip_remains_an_explicit_declaration_limit() {
    let error = mote::compile_with_options(
        "#let(declared=document.layout({name='custom'},document.box({slot='title',position={width=inch(5),height=inch(1)}})))",
        mote::evaluation::EvaluationOptions { enable_lua: true, ..Default::default() }
    ).unwrap_err();
    assert!(
        error.message.contains("must not contain content"),
        "{error:?}"
    );
}
