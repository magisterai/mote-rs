#[path = "support/language_capture.rs"]
mod language_capture;
use language_capture::xml_support as package_support;

#[test]
fn equations_preserve_models_editable_structure_and_native_fallback_geometry() {
    language_capture::compare(
        "mathematics",
        &[
            "expressions",
            "edges",
            "matrices",
            "raw-and-lua",
            "slides",
            "stories",
        ],
    );
}

#[test]
fn invalid_equation_arguments_are_diagnosed() {
    for source in [
        r#"#math()"#,
        r#"#math(" ")"#,
        r#"#math(true)"#,
        r#"#math("x",format="tex")"#,
        r#"#math("x",display="center")"#,
        r#"#math("x")[body]"#,
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
}

#[test]
fn malformed_raw_math_cannot_corrupt_an_office_package() {
    for equation in [
        "<m:oMath>",
        "<w:p/>",
        "<m:oMath/><m:oMath/>",
        "<m:oMath/>trailing",
        "<foreign:oMath/>",
        "<m:oMath xmlns:m=\"urn:unrelated\"/>",
    ] {
        let quoted = serde_json::to_string(equation).unwrap();
        let source = format!("#math({quoted},format=\"omml\")");
        let document = mote::compile(&source).unwrap();
        let generated = mote::docx::emit_with_assets(
            &document,
            &mote::source::FileAssetResolver::from_current_directory().unwrap(),
            None,
        )
        .unwrap();
        assert!(
            generated
                .warnings
                .iter()
                .any(|warning| warning.contains("invalid OMML"))
        );
        let parts = package_support::unpack(generated.bytes);
        let xml = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        assert!(xml.descendants().any(|node| node.text() == Some(equation)));
        let source = format!(
            "#presentation()\n#slide[#box(position={{x=pt(0),y=pt(0),width=pt(400),height=pt(100)}})[{source}]]"
        );
        let document = mote::compile(&source).unwrap();
        let generated = mote::pptx::emit_with_warnings(&document).unwrap();
        assert!(
            generated
                .warnings
                .iter()
                .any(|warning| warning.contains("invalid OMML"))
        );
        let parts = package_support::unpack(generated.bytes);
        let xml = roxmltree::Document::parse(&parts["ppt/slides/slide1.xml"]).unwrap();
        assert!(xml.descendants().any(|node| node.text() == Some(equation)));
    }
}

#[test]
fn bounded_parser_warns_for_unknown_commands_and_deep_groups() {
    let deep = format!("{}x{}", "{".repeat(150), "}".repeat(150));
    for (equation, message) in [
        (r"\unsupported{x}".to_owned(), "unsupported math command"),
        (
            r"\begin{unknown}x\end{unknown}".to_owned(),
            "unsupported math environment",
        ),
        (deep, "nesting limit"),
    ] {
        let source = format!("#math({})", serde_json::to_string(&equation).unwrap());
        let document = mote::compile(&source).unwrap();
        let generated = mote::docx::emit_with_assets(
            &document,
            &mote::source::FileAssetResolver::from_current_directory().unwrap(),
            None,
        )
        .unwrap();
        assert!(
            generated
                .warnings
                .iter()
                .any(|warning| warning.contains(message)),
            "{message}: {:?}",
            generated.warnings
        );
        let parts = package_support::unpack(generated.bytes);
        roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    }
}

#[test]
fn native_equations_use_explicit_source_fallbacks() {
    let document = mote::compile(r"Inline $x^2$.\n\[a+b\]").unwrap();
    let resolver = mote::source::FileAssetResolver::from_current_directory().unwrap();
    let generated = mote::render::plan(&document, &resolver, None).unwrap();
    assert!(
        generated
            .warnings
            .iter()
            .any(|warning| warning.contains("mathematical typesetting is deferred"))
    );
    assert!(
        generated
            .output
            .pages
            .iter()
            .flat_map(|page| &page.lines)
            .flat_map(|line| &line.line.fragments)
            .any(|fragment| fragment.text == "x^2"
                && fragment.font.variant == mote::measurement::FontVariant::Italic)
    );
}

#[test]
fn invalid_xml_characters_fail_before_math_serialization() {
    let source = format!("#math(\"x{}y\")", '\u{1}');
    let document = mote::compile(&source).unwrap();
    let error = mote::docx::emit(&document).err().unwrap();
    assert!(error.to_string().contains("U+0001"));
}
