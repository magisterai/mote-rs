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
            "recovery",
            "recovery-slides",
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
        "<m:oMath><unknown:r/></m:oMath>",
        "<m:oMath unknown:flag=\"1\"/>",
        "<m:oMath m:flag=\"1\" m:flag=\"2\"/>",
        "<m:oMath><m:r xmlns:local=\"urn:local\"/><local:r/></m:oMath>",
        "<?xml version=\"1.0\"?><m:oMath/>",
        "<!DOCTYPE m:oMath><m:oMath/>",
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
fn unfinished_delimiters_preserve_whole_sources_in_editable_equations() {
    for equation in [
        r"\left",
        r"\left(",
        r"\left(x",
        r"\left(x\right",
        r"\left(x\right\",
        r"\right)",
        r"\left\right)",
        r"a+\left[x<y",
        r"\left(a+\left[b\right]",
        r"\sqrt[\left(x]{y}",
        r"\begin{matrix}\left(x&y\end{matrix}",
    ] {
        let source = format!("#math({})", serde_json::to_string(equation).unwrap());
        let document = mote::compile(&source).unwrap();
        let resolver = mote::source::FileAssetResolver::from_current_directory().unwrap();
        let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
        assert!(
            generated
                .warnings
                .iter()
                .any(|warning| warning.contains("incomplete math delimiters"))
        );
        let parts = package_support::unpack(generated.bytes);
        let xml = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        assert!(
            xml.descendants().any(|node| node.text() == Some(equation)),
            "{equation}"
        );
        assert!(!parts["word/document.xml"].contains('�'));
        let slide = mote::compile(&presentation_source(&source)).unwrap();
        let generated = mote::pptx::emit_with_warnings(&slide).unwrap();
        assert!(
            generated
                .warnings
                .iter()
                .any(|warning| warning.contains("incomplete math delimiters"))
        );
        let parts = package_support::unpack(generated.bytes);
        let xml = roxmltree::Document::parse(&parts["ppt/slides/slide1.xml"]).unwrap();
        assert!(
            xml.descendants().any(|node| node.text() == Some(equation)),
            "{equation}"
        );
        assert!(!parts["ppt/slides/slide1.xml"].contains('�'));
    }
}

#[test]
fn raw_equations_inherit_the_destination_namespace_context() {
    for (source, presentation) in [
        (
            r#"<m:oMath mc:Ignorable="w14"><m:r><m:t>x</m:t></m:r></m:oMath>"#,
            false,
        ),
        (
            r#"<m:oMath mc:Ignorable="a14"><m:r><m:t>x</m:t></m:r></m:oMath>"#,
            true,
        ),
    ] {
        let equation = format!(
            "#math({},format=\"omml\")",
            serde_json::to_string(source).unwrap()
        );
        let (parts, part) = if presentation {
            let document = mote::compile(&presentation_source(&equation)).unwrap();
            let generated = mote::pptx::emit_with_warnings(&document).unwrap();
            assert!(generated.warnings.is_empty(), "{:?}", generated.warnings);
            (
                package_support::unpack(generated.bytes),
                "ppt/slides/slide1.xml",
            )
        } else {
            let document = mote::compile(&equation).unwrap();
            let generated = mote::docx::emit_with_assets(
                &document,
                &mote::source::FileAssetResolver::from_current_directory().unwrap(),
                None,
            )
            .unwrap();
            assert!(generated.warnings.is_empty(), "{:?}", generated.warnings);
            (
                package_support::unpack(generated.bytes),
                "word/document.xml",
            )
        };
        roxmltree::Document::parse(&parts[part]).unwrap();
        assert!(parts[part].contains(source));
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

fn presentation_source(content: &str) -> String {
    format!(
        "#presentation()\n#slide[\n\
         #box(position={{x=pt(20),y=pt(20),width=pt(600),height=pt(200)}})[{content}]\n]"
    )
}
