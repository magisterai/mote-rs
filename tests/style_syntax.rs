use mote::{
    diagnostics::Diagnostic,
    evaluation::EvaluationOptions,
    formatting::StyleResolver,
    model::{Document, StyleKind, Value},
    render::{self, LayoutMode},
    source::{AssetRequest, FileAssetResolver, ResolvedAsset},
    syntax::{self, ExpressionKind, NodeKind},
};

#[allow(dead_code)]
#[path = "docx/support.rs"]
mod package_support;

const CALL_STYLES: &str = r##"#style(name = "Body", run = { font = "Helvetica", size = pt(11), bold = true, color = "#336699" }, paragraph = { spacing = { before = pt(2), after = pt(8) } })
#style(name = "Lead", based_on = "Body", run = { size = pt(14), bold = false }, paragraph = { spacing = { after = pt(0) } })
#style(name = "Accent", type = "character", run = { italic = true })"##;

const NAMED_STYLES: &str = r##"#style "Body" (run = { font = "Helvetica", size = pt(11), bold = true, color = "#336699" }, paragraph = { spacing = { before = pt(2), after = pt(8) } })
#style "Lead" extends "Body" (run = { size = pt(14), bold = false }, paragraph = { spacing = { after = pt(0) } })
#style "Accent" (type = "character", run = { italic = true })"##;

fn compile(source: &str, enable_lua: bool) -> Result<Document, Diagnostic> {
    mote::compile_with_options(
        source,
        EvaluationOptions {
            enable_lua,
            ..Default::default()
        },
    )
}

fn compare_models(named: &Document, call: &Document) {
    assert_eq!(
        mote::json::to_json(named).unwrap(),
        mote::json::to_json(call).unwrap()
    );
    assert_eq!(named.styles.len(), call.styles.len());
    for name in named.styles.keys() {
        assert_eq!(
            StyleResolver::new(named).resolve_style(name),
            StyleResolver::new(call).resolve_style(name),
            "resolved style {name}"
        );
    }
}

#[test]
fn quoted_headers_preserve_literal_values_and_source_ranges_in_both_modes() {
    let source = "#style \"Résumé \\\"Lead\\\"\"\r\n extends 'Corps'\n (bold = false)";
    for tree in [syntax::parse(source), syntax::parse_with_lua(source)] {
        let tree = tree.unwrap();
        assert_eq!(tree.nodes.len(), 1);
        let node = &tree.nodes[0];
        let NodeKind::Call(call) = &node.kind else {
            panic!("expected style call")
        };
        assert_eq!(call.name, "style");
        assert!(call.bodies.is_empty());
        assert_eq!(&source[node.span.start..node.span.end], source);
        for (argument, name, value, spelling) in [
            (
                &call.arguments[0],
                "name",
                "Résumé \"Lead\"",
                "\"Résumé \\\"Lead\\\"\"",
            ),
            (&call.arguments[1], "based_on", "Corps", "'Corps'"),
        ] {
            assert_eq!(argument.name.as_deref(), Some(name));
            assert_eq!(argument.value.kind, ExpressionKind::String(value.into()));
            assert_eq!(argument.span, argument.value.span);
            assert_eq!(&source[argument.span.start..argument.span.end], spelling);
        }
        assert_eq!(call.arguments[2].name.as_deref(), Some("bold"));
    }
}

#[test]
fn named_and_call_forms_share_models_inheritance_and_type_defaults() {
    for enable_lua in [false, true] {
        let named = compile(NAMED_STYLES, enable_lua).unwrap();
        let call = compile(CALL_STYLES, enable_lua).unwrap();
        compare_models(&named, &call);
        assert!(named.content.is_empty());
        assert!(named.warnings.is_empty());
        assert_eq!(named.styles["Lead"].kind, StyleKind::Paragraph);
        assert_eq!(named.styles["Accent"].kind, StyleKind::Character);
        let resolved = StyleResolver::new(&named).resolve_style("Lead");
        assert_eq!(resolved.run["bold"], Value::Boolean(false));
        assert_eq!(resolved.run["color"], Value::Color("#336699".into()));
        let Value::Object(spacing) = &resolved.paragraph["spacing"] else {
            panic!("expected paragraph spacing")
        };
        assert_eq!(spacing["before"].to_text(), "2pt");
        assert_eq!(spacing["after"].to_text(), "0pt");
    }
}

#[test]
fn both_forms_can_mix_and_replacements_remain_document_wide() {
    let named = r#"#paragraph(style = "Lead")[Before]
#style "Lead" extends "Lead" ()
#paragraph[#style "Body" (bold = true)Inside]
#style(name = "Lead", based_on = "Body", italic = true)
#style "Body" (bold = false)
`#style "Literal" ()`"#;
    let call = r#"#paragraph(style = "Lead")[Before]
#style(name = "Lead", based_on = "Lead")
#paragraph[#style(name = "Body", bold = true)Inside]
#style(name = "Lead", based_on = "Body", italic = true)
#style(name = "Body", bold = false)
`#style "Literal" ()`"#;
    for enable_lua in [false, true] {
        let document = compile(named, enable_lua).unwrap();
        compare_models(&document, &compile(call, enable_lua).unwrap());
        assert_eq!(document.styles.len(), 2);
        let resolved = StyleResolver::new(&document).resolve_style("Lead");
        assert_eq!(resolved.run["bold"], Value::Boolean(false));
        assert_eq!(resolved.run["italic"], Value::Boolean(true));
    }
}

#[test]
fn named_headers_keep_existing_parent_and_type_rules() {
    for enable_lua in [false, true] {
        let named = compile(
            r#"#style 'Base' (type = "character", bold = true)
#style "Child" extends 'Base' ()
#style "Grid" (type = "table", table = { cell = { margin = { left = pt(3) } } })
#style "Detail" (based_on = "Base", italic = true)"#,
            enable_lua,
        )
        .unwrap();
        let call = compile(
            r#"#style(name = 'Base', type = "character", bold = true)
#style(name = "Child", based_on = 'Base')
#style(name = "Grid", type = "table", table = { cell = { margin = { left = pt(3) } } })
#style(name = "Detail", based_on = "Base", italic = true)"#,
            enable_lua,
        )
        .unwrap();
        compare_models(&named, &call);
        assert_eq!(named.styles["Child"].kind, StyleKind::Paragraph);
        assert_eq!(named.styles["Grid"].kind, StyleKind::Table);
    }
}

#[test]
fn lua_properties_remain_expressions_and_header_names_remain_literals() {
    let named = r#"#let(size = 10)
#style "Base" (size = pt(size + 1))
#style "Lead" extends "Base" (bold = size > 5)"#;
    let call = r#"#let(size = 10)
#style(name = "Base", size = pt(size + 1))
#style(name = "Lead", based_on = "Base", bold = size > 5)"#;
    compare_models(
        &compile(named, true).unwrap(),
        &compile(call, true).unwrap(),
    );
    assert!(compile(named, false).is_err());
}

#[test]
fn legacy_calls_and_bare_references_keep_their_existing_rules() {
    for enable_lua in [false, true] {
        let document = compile(
            "#let(style=\"value\")#paragraph[#style followed by prose]\n#style(name=7, name=\"Final\", bold=false, bold=true)",
            enable_lua,
        ).unwrap();
        assert_eq!(document.content[0].to_text(), "value followed by prose");
        assert_eq!(
            document.styles["Final"].properties["bold"],
            Value::Boolean(true)
        );
        assert!(compile(r#"#style("Body")"#, enable_lua).is_err());
    }
    let tree = syntax::parse("#style\n\"quoted prose\"").unwrap();
    assert!(matches!(&tree.nodes[0].kind, NodeKind::Reference(name) if name == "style"));
}

#[test]
fn malformed_headers_report_source_locations_in_both_modes() {
    for (source, expected, offending) in [
        (r#"#style "Body""#, "properties in parentheses", ""),
        (r#"#style "Body" extends"#, "quoted parent style name", ""),
        (
            r#"#style "Body" extends Parent ()"#,
            "quoted parent style name",
            "P",
        ),
        (
            r#"#style "Body" extends "Base""#,
            "properties in parentheses",
            "",
        ),
        (
            r#"#style "Body" extends_more "Base" ()"#,
            "properties in parentheses",
            "e",
        ),
        (
            r#"#style "Body" (name = "Other")"#,
            "name is already specified",
            "name = \"Other\"",
        ),
        (
            r#"#style "Body" extends "Base" (based_on = "Other")"#,
            "parent is already specified",
            "based_on = \"Other\"",
        ),
        (
            r#"#style "Body" extends "unfinished"#,
            "unterminated string",
            "",
        ),
    ] {
        let source = format!("Résumé\n{source}");
        for error in [
            syntax::parse(&source).unwrap_err(),
            syntax::parse_with_lua(&source).unwrap_err(),
        ] {
            assert!(error.message.contains(expected), "{source}: {error}");
            assert_eq!(&source[error.span.start..error.span.end], offending);
            assert_eq!(error.location(&source).0, 2);
        }
    }
}

#[test]
fn named_declarations_retain_validation_and_resolution_failures() {
    for (source, expected) in [
        (
            r#"#style "Body" ()[Text]"#,
            "named arguments and no content body",
        ),
        (
            r#"#style "Body" ("Text")"#,
            "named arguments and no content body",
        ),
        (r#"#style "Body" (type = "section")"#, "style type"),
        (
            r#"#style "Body" (run = { bold = 1 })"#,
            "bold must be a boolean",
        ),
        (
            r#"#style "Body" extends "Body" ()"#,
            "cannot be based on itself",
        ),
    ] {
        for enable_lua in [false, true] {
            let error = compile(source, enable_lua).unwrap_err();
            assert!(error.message.contains(expected), "{source}: {error}");
        }
    }
    for (named, call) in [
        (
            r#"#style "Child" extends "Missing" (bold = true)"#,
            r#"#style(name = "Child", based_on = "Missing", bold = true)"#,
        ),
        (
            r#"#style "First" extends "Second" ()#style "Second" extends "First" ()"#,
            r#"#style(name = "First", based_on = "Second")#style(name = "Second", based_on = "First")"#,
        ),
    ] {
        let named = compile(named, false).unwrap();
        compare_models(&named, &compile(call, false).unwrap());
        assert!(mote::docx::emit(&named).is_err());
    }
}

#[test]
fn truncated_unicode_headers_never_produce_invalid_diagnostic_ranges() {
    let source = r#"#style "Résumé" extends 'Corps' (run = { bold = true })"#;
    for end in source.char_indices().map(|(offset, _)| offset) {
        let prefix = &source[..end];
        for result in [syntax::parse(prefix), syntax::parse_with_lua(prefix)] {
            if let Err(error) = result {
                assert!(error.span.start <= error.span.end);
                assert!(error.span.end <= prefix.len());
                assert!(prefix.is_char_boundary(error.span.start));
                assert!(prefix.is_char_boundary(error.span.end));
            }
        }
    }
}

#[test]
fn imported_named_declarations_preserve_registration_and_error_provenance() {
    for enable_lua in [false, true] {
        let resolver = |_request: AssetRequest<'_>| {
            Ok(ResolvedAsset {
                name: "library/styles.mote".into(),
                bytes: NAMED_STYLES.as_bytes().to_vec(),
            })
        };
        let document = mote::compile_with_options(
            r#"#import("styles.mote")#paragraph(style = "Lead")[Imported]"#,
            EvaluationOptions {
                asset_resolver: Some(&resolver),
                enable_lua,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            document.imports,
            [std::ffi::OsString::from("library/styles.mote")]
        );
        assert_eq!(document.styles.len(), 3);
        let resolver = |_request: AssetRequest<'_>| {
            Ok(ResolvedAsset {
                name: "library/broken.mote".into(),
                bytes: b"\n#style \"Lead\" extends Missing ()".to_vec(),
            })
        };
        let error = mote::compile_with_options(
            r#"#import("broken.mote")"#,
            EvaluationOptions {
                asset_resolver: Some(&resolver),
                enable_lua,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(error.message.contains("quoted parent style name"));
        assert_eq!(error.location(""), (2, 23));
        assert_eq!(
            error.source.unwrap().name,
            std::ffi::OsStr::new("library/broken.mote")
        );
    }
}

#[test]
fn both_forms_generate_the_same_packages_page_plans_pdf_and_png() {
    let content = r#"#paragraph(style = "Lead")[Opening #span(style = "Accent")[text].]
#paragraph(style = "Body")[A second paragraph with inherited formatting.]"#;
    let resolver = FileAssetResolver::from_current_directory().unwrap();
    for enable_lua in [false, true] {
        let named = compile(&format!("{NAMED_STYLES}\n{content}"), enable_lua).unwrap();
        let call = compile(&format!("{CALL_STYLES}\n{content}"), enable_lua).unwrap();
        compare_models(&named, &call);
        for (named_bytes, call_bytes) in [
            (
                mote::docx::emit(&named).unwrap(),
                mote::docx::emit(&call).unwrap(),
            ),
            (
                mote::pptx::emit_with_fonts(&named, &resolver, None, None)
                    .unwrap()
                    .bytes,
                mote::pptx::emit_with_fonts(&call, &resolver, None, None)
                    .unwrap()
                    .bytes,
            ),
        ] {
            let named_parts = package_support::unpack(named_bytes);
            let call_parts = package_support::unpack(call_bytes);
            assert_eq!(
                named_parts.keys().collect::<Vec<_>>(),
                call_parts.keys().collect::<Vec<_>>()
            );
            for (name, actual) in named_parts {
                package_support::compare_xml(&actual, &call_parts[&name], &name);
            }
        }
        for mode in [LayoutMode::Document, LayoutMode::Presentation] {
            let named_plan =
                render::plan_with_layout_and_fonts(&named, &resolver, None, mode, None).unwrap();
            let call_plan =
                render::plan_with_layout_and_fonts(&call, &resolver, None, mode, None).unwrap();
            assert!(!named_plan.output.pages.is_empty());
            assert_eq!(named_plan.output, call_plan.output);
            assert_eq!(named_plan.warnings, call_plan.warnings);
            assert_eq!(
                render::paint_pdf(&named_plan.output).unwrap(),
                render::paint_pdf(&call_plan.output).unwrap()
            );
            let named_pages = render::paint_png(&named_plan.output, &Default::default()).unwrap();
            let call_pages = render::paint_png(&call_plan.output, &Default::default()).unwrap();
            assert_eq!(named_pages.warnings, call_pages.warnings);
            assert_eq!(named_pages.output.len(), call_pages.output.len());
            for (named_page, call_page) in named_pages.output.iter().zip(&call_pages.output) {
                assert_eq!(named_page.bytes, call_page.bytes);
            }
        }
    }
}
