use mote::evaluation::EvaluationOptions;
fn compile(source: &str) -> Result<mote::model::Document, mote::diagnostics::Diagnostic> {
    mote::compile_with_options(
        source,
        EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
}
#[test]
fn element_constructors_share_attribute_normalization_and_property_precedence() {
    for (expression, source) in [
        (
            "document.paragraph({alignment='center',spacing={after='8pt'}},'Text')",
            "#paragraph(alignment='center',spacing={after=pt(8)})[Text]",
        ),
        (
            "document.heading({level=2,properties={alignment='center'}},'Title')",
            "#heading(2,properties={alignment='center'})[Title]",
        ),
        (
            "document.span({color='#abc',bold='true'},'Text')",
            "#span(color='#abc',bold='true')[Text]",
        ),
        (
            "document.code({preserve_space=true},'a\\r\\n  b')",
            "#code(\"a\\n  b\",preserve_space=true)",
        ),
    ] {
        assert_eq!(
            compile(&format!("#let(content={expression})#content")).unwrap(),
            mote::compile(source).unwrap(),
            "{expression}"
        );
    }
}
#[test]
fn constructors_reject_invalid_bodies_values_and_argument_shapes() {
    for expression in [
        "document.paragraph()",
        "document.paragraph({alignment='diagonal'},'Text')",
        "document.paragraph(document.heading(1,'Block'))",
        "document.span({bold='maybe'},'Text')",
        "document.list('text')",
        "document.table(document.cell('wrong parent'))",
        "document.row(document.paragraph('wrong parent'))",
        "document.image('x.png','body')",
        "document.link({url='https://example.com'})",
        "document.document('text')",
        "document.font({name='body',latin=''})",
        "document.header()",
        "document.header({unexpected=true},'Header')",
        "document.section({})",
    ] {
        assert!(
            compile(&format!("#let(content={expression})#content")).is_err(),
            "{expression}"
        );
    }
}
#[test]
fn namespace_uses_native_constructors_and_custom_function_names_remain_distinct() {
    let source =
        "#function paragraph(body) return body end\n#let(found=type(document.paragraph))#found";
    assert_eq!(compile(source).unwrap(), mote::compile("nil").unwrap());
    assert!(
        compile("#function custom() return 'text' end\n#let(found=document.custom())").is_err()
    );
}
#[test]
fn lua_bodies_are_prebuilt_nodes_and_keep_their_existing_structure() {
    let document = compile("#let(content=document.blockquote('Raw text'))#content").unwrap();
    let mote::model::Content::Element(element) = &document.content[0] else {
        panic!("missing quote")
    };
    assert_eq!(
        element.children,
        vec![mote::model::Content::Text("Raw text".into())]
    );
    let document =
        compile("#let(content=document.blockquote(document.paragraph('Wrapped')))#content")
            .unwrap();
    assert_eq!(
        document,
        mote::compile("#blockquote[#paragraph[Wrapped]]").unwrap()
    );
}
