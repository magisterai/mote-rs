#[allow(dead_code)]
#[path = "docx/support.rs"]
mod support;
use mote::{evaluation::EvaluationOptions, source::FileAssetResolver};

#[test]
fn retained_wrapper_formatting_warns_without_changing_its_semantic_effect() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for directive in [
        "strike",
        "highlight",
        "small_caps",
        "subscript",
        "superscript",
    ] {
        for (property, value) in [
            ("style", "'Missing'"),
            ("font", "'Missing'"),
            ("size", "pt(16)"),
            ("color", "'#ff0000'"),
        ] {
            let source = format!("Before.\n\n#{directive}({property}={value})[Text]");
            let document = mote::compile_with_options(
                &source,
                EvaluationOptions {
                    source_name: Some(std::ffi::OsStr::new("report.mote")),
                    ..Default::default()
                },
            )
            .unwrap();
            let original = mote::json::to_json(&document).unwrap();
            let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
            assert_eq!(generated.warnings.len(), 1, "{source}");
            let warning = &generated.warnings[0];
            assert!(
                warning.contains(&format!("{directive}.{property}")),
                "{warning}"
            );
            assert!(warning.contains("report.mote:3:1"), "{warning}");
            assert!(warning.contains("surrounding span"));
            let actual = support::unpack(generated.bytes);
            let expected = support::package(&format!("Before.\n\n#{directive}[Text]"));
            for name in ["word/document.xml", "word/styles.xml"] {
                support::compare_xml(&actual[name], &expected[name], name);
            }
            assert_eq!(mote::json::to_json(&document).unwrap(), original);
        }
    }
}

#[test]
fn known_ignored_and_unknown_properties_warn() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for (source, property) in [
        (
            "#list(marker='x',style='Missing',indent={right=pt(4)},spacing={before=pt(3)})[#item[Text]]",
            "list.marker",
        ),
        (
            "#ordered_list(indent={right=pt(4)},spacing={before=pt(3)})[#item(indent={right=pt(4)},spacing={before=pt(3)})[Text]]",
            "item.spacing.before",
        ),
        (
            "#paragraph(border={width=pt(2),color='#ff0000'})[Text]",
            "paragraph.border.width",
        ),
        (
            "#blockquote(border={color='#ff0000'})[Text]",
            "blockquote.border.color",
        ),
        (
            "#style(name='Body',table={fill='#ff0000'})\n#paragraph(style='Body')[Text]",
            "table on a paragraph style",
        ),
    ] {
        let generated =
            mote::docx::emit_with_assets(&mote::compile(source).unwrap(), &resolver, None).unwrap();
        assert!(
            generated
                .warnings
                .iter()
                .any(|warning| warning.contains(property)),
            "{source}: {:?}",
            generated.warnings
        );
        assert!(!generated.bytes.is_empty());
    }
    for (source, name) in [
        ("#strike(properties={future=true})[Text]", "strike.future"),
        (
            "#list(indent={custom=pt(4)})[#item[Text]]",
            "list.indent.custom",
        ),
        (
            "#paragraph(properties={unsupported=true})[Text]",
            "paragraph.unsupported",
        ),
        ("#paragraph(style='Missing')[Text]", "paragraph.style"),
    ] {
        support::assert_ignored(source, name);
    }
}

#[test]
fn lua_round_trip_keeps_the_original_warning_location() {
    let source = "#function identity(body) return body end\n#identity[#strike(size=pt(16))[Text]]";
    let document = mote::compile_with_options(
        source,
        EvaluationOptions {
            enable_lua: true,
            source_name: Some(std::ffi::OsStr::new("round-trip.mote")),
            ..Default::default()
        },
    )
    .unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    assert!(
        generated
            .warnings
            .iter()
            .any(|warning| warning.contains("round-trip.mote:2:11")),
        "{:?}",
        generated.warnings
    );
}

#[test]
fn imported_function_warnings_use_the_definition_source_snapshot() {
    use mote::source::{AssetRequest, ResolvedAsset};
    let resolver = |_request: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "library/formatting.mote".into(),
            bytes: b"\n\n#function marked() return document.strike({size=pt(16)}, 'Text') end"
                .to_vec(),
        })
    };
    let document = mote::compile_with_options(
        "#import('formatting.mote')\n#marked()",
        EvaluationOptions {
            asset_resolver: Some(&resolver),
            source_name: Some(std::ffi::OsStr::new("caller.mote")),
            enable_lua: true,
            ..Default::default()
        },
    )
    .unwrap();
    let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    assert!(
        generated
            .warnings
            .iter()
            .any(|warning| warning.contains("library/formatting.mote:3:1")),
        "{:?}",
        generated.warnings
    );
}

#[test]
fn nested_block_fallbacks_report_the_affected_source() {
    let source =
        "#list[#item[\n#heading(2)[Title]\n#divider()\n#paragraph(alignment='right')[Body]\n]]";
    let document = mote::compile_with_options(
        source,
        EvaluationOptions {
            source_name: Some(std::ffi::OsStr::new("nested.mote")),
            ..Default::default()
        },
    )
    .unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    for (message, location) in [
        ("heading.level", "nested.mote:2:1"),
        ("omits a divider", "nested.mote:3:1"),
        ("paragraph.alignment", "nested.mote:4:1"),
    ] {
        assert!(
            generated
                .warnings
                .iter()
                .any(|warning| warning.contains(message) && warning.contains(location)),
            "{:?}",
            generated.warnings
        );
    }
}

#[test]
fn nested_story_paragraphs_keep_their_independent_formatting_context() {
    let source =
        "#list[#item[Text#footnote[#paragraph[First]#paragraph(alignment='right')[Second]]]]";
    let resolver = FileAssetResolver::new(".").unwrap();
    let generated =
        mote::docx::emit_with_assets(&mote::compile(source).unwrap(), &resolver, None).unwrap();
    assert!(
        !generated
            .warnings
            .iter()
            .any(|warning| warning.contains("paragraph.alignment")),
        "{:?}",
        generated.warnings
    );
    let package = support::unpack(generated.bytes);
    assert!(package["word/footnotes.xml"].contains("w:jc w:val=\"right\""));
}

#[test]
fn every_name_without_a_word_mapping_is_reported_and_the_package_is_written() {
    let (parts, warnings) = support::package_with_warnings(include_str!(
        "fixtures/output-fallbacks/unmapped-names.mote"
    ));
    for name in [
        "blockquote.custom",
        "border.top.sheen",
        "break.note",
        "checkbox.custom",
        "code.color",
        "divider.custom",
        "document.columns.widths",
        "document.margin.inside",
        "document.note",
        "document.page.code",
        "emphasis.font",
        "font.pitch",
        "heading.custom",
        "heading.numbering.custom",
        "heading.style",
        "indent.start",
        "item.checked",
        "item.future",
        "link.rel",
        "list.future",
        "list.indent.custom",
        "paragraph.custom",
        "paragraph.shadow",
        "paragraph.style",
        "raw.font",
        "raw.size",
        "run.glow",
        "spacing.extra",
        "span.custom",
        "span.style",
        "strike.future",
        "strong.size",
        "style.based_on",
        "style.next",
        "tab.note",
        "table.style",
        "tabs.fill",
        "underline.depth",
    ] {
        let expected = format!("DOCX ignores {name};");
        assert!(
            warnings
                .iter()
                .any(|warning| warning.starts_with(&expected)),
            "{name}: {warnings:?}"
        );
    }
    let document = &parts["word/document.xml"];
    for ignored in ["custom", "future", "Absent", "sheen"] {
        assert!(!document.contains(ignored), "{ignored}");
    }
    assert!(!parts["word/styles.xml"].contains("Absent"));
}
