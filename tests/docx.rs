#[path = "docx/support.rs"]
mod support;

#[test]
fn plain_paragraph_package_matches_reference() {
    let parts = support::compare_fixture("plain-paragraph");
    let metadata = roxmltree::Document::parse(&parts["docProps/core.xml"]).unwrap();
    let timestamp = |name| {
        metadata
            .descendants()
            .find(|node| node.has_tag_name(("http://purl.org/dc/terms/", name)))
            .unwrap()
            .text()
            .unwrap()
    };
    assert_eq!(timestamp("created"), timestamp("modified"));
    assert_eq!(timestamp("created").len(), 20);
    assert!(timestamp("created").ends_with('Z'));
}

#[test]
fn paragraphs_generate_content_independently_of_the_reference() {
    let parts = support::package("First paragraph.\n\nSecond & <third>.");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let paragraphs: Vec<_> = document
        .descendants()
        .filter(|node| {
            node.has_tag_name((
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "p",
            ))
        })
        .collect();
    assert_eq!(paragraphs.len(), 2);
    let text: Vec<_> = document
        .descendants()
        .filter(|node| {
            node.has_tag_name((
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "t",
            ))
        })
        .map(|node| node.text().unwrap())
        .collect();
    assert_eq!(text, ["First paragraph.", "Second & ", "<", "third>."]);
}

#[test]
fn names_without_a_word_mapping_warn_and_leave_the_package_as_it_is_without_them() {
    for (source, name, without) in [
        (
            "#paragraph(custom=true)[Text]",
            "paragraph.custom",
            "#paragraph[Text]",
        ),
        ("#span(custom=true)[Text]", "span.custom", "#span[Text]"),
        (
            "#font(name=\"Body\", pitch=\"fixed\")",
            "font.pitch",
            "#font(name=\"Body\")",
        ),
        (
            "#paragraph(indent={left=pt(4),start=pt(4)})[Text]",
            "indent.start",
            "#paragraph(indent={left=pt(4)})[Text]",
        ),
        ("#strong(size=pt(12))[Text]", "strong.size", "#strong[Text]"),
        (
            "#emphasis(font=\"Arial\")[Text]",
            "emphasis.font",
            "#emphasis[Text]",
        ),
        ("#code(color=\"#123\")[Text]", "code.color", "#code[Text]"),
        (
            "#heading(level=1, custom=true)[Unknown]",
            "heading.custom",
            "#heading(level=1)[Unknown]",
        ),
        (
            "#heading(numbering={enabled=true,custom=3})[Numbered]",
            "heading.numbering.custom",
            "#heading(numbering={enabled=true})[Numbered]",
        ),
    ] {
        let actual = support::assert_ignored(source, name);
        let expected = support::package(without);
        for part in ["word/document.xml", "word/styles.xml", "word/fontTable.xml"] {
            support::compare_xml(&actual[part], &expected[part], part);
        }
    }
}

#[test]
fn a_warning_about_an_ignored_name_states_its_source_location() {
    let source = "First.\n\n#paragraph[Second #span(custom=true)[text].]";
    let document = mote::compile_with_options(
        source,
        mote::evaluation::EvaluationOptions {
            source_name: Some(std::ffi::OsStr::new("report.mote")),
            ..Default::default()
        },
    )
    .unwrap();
    let resolver = mote::source::FileAssetResolver::new(".").unwrap();
    let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    assert_eq!(
        generated.warnings,
        ["DOCX ignores span.custom; the name has no Word mapping (report.mote:3:19)"]
    );
}

#[test]
fn text_breaks_and_tabs_match_reference() {
    support::compare_fixture("text-fidelity");
}

#[test]
fn illegal_xml_characters_fail_without_mutating_the_model() {
    let document = mote::compile("text\u{1}more").unwrap();
    let original = document.clone();
    let error = mote::docx::emit(&document).unwrap_err();
    assert!(error.to_string().contains("U+0001"));
    assert_eq!(document, original);
}

#[test]
fn paragraph_formatting_matches_reference() {
    support::compare_fixture("paragraph-formatting");
}

#[test]
fn invalid_output_ranges_are_explicit() {
    for source in [
        "#paragraph(outline_level=10)[Text]",
        "#paragraph(spacing_after=pt(1000000000000))[Text]",
    ] {
        assert!(
            mote::docx::emit(&mote::compile(source).unwrap()).is_err(),
            "{source}"
        );
    }
}

#[test]
fn inline_formatting_matches_reference() {
    support::compare_fixture("inline-formatting");
}

#[test]
fn named_styles_and_font_mappings_match_reference() {
    support::compare_fixture("styles-and-fonts");
}

#[test]
fn undeclared_style_references_warn_and_are_left_out() {
    for (source, name, without) in [
        (
            "#paragraph(style=\"Missing\")[Text]",
            "paragraph.style",
            "#paragraph[Text]",
        ),
        (
            "#paragraph[#span(style=\"Missing\")[Text]]",
            "span.style",
            "#paragraph[#span[Text]]",
        ),
        (
            "#style(name=\"A\",based_on=\"Missing\")",
            "style.based_on",
            "#style(name=\"A\")",
        ),
        (
            "#style(name=\"A\",next=\"Missing\")",
            "style.next",
            "#style(name=\"A\")",
        ),
        (
            "#table(style=\"Missing\")[#row[#cell[Text]]]",
            "table.style",
            "#table[#row[#cell[Text]]]",
        ),
    ] {
        let (actual, warnings) = support::package_with_warnings(source);
        let expected = format!("DOCX ignores {name}; style \"Missing\" is not declared");
        assert!(
            warnings
                .iter()
                .any(|warning| warning.starts_with(&expected)),
            "{source}: {warnings:?}"
        );
        let expected = support::package(without);
        for part in ["word/document.xml", "word/styles.xml"] {
            support::compare_xml(&actual[part], &expected[part], part);
        }
    }
}

#[test]
fn a_cycle_of_styles_fails_explicitly() {
    let source = "#style(name=\"A\",based_on=\"B\")#style(name=\"B\",based_on=\"A\")";
    assert!(mote::docx::emit(&mote::compile(source).unwrap()).is_err());
}

#[test]
fn declared_property_order_and_style_identity_match_reference() {
    support::compare_fixture("style-properties");
}

#[test]
fn builtin_style_dependencies_are_emitted_and_references_are_resolved() {
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    for (source, required_identifier) in [
        (
            r##"#style(name="Snippet",type="character",based_on="Code",color="#123")
#span(style="Snippet")[Text]"##,
            "Code",
        ),
        (
            r#"#style(name="Quotation",based_on="BlockQuote")
#paragraph(style="Quotation")[Text]"#,
            "BlockQuote",
        ),
        (
            r#"#style(name="Body",next="Heading1")
#paragraph(style="Body")[Text]"#,
            "Heading1",
        ),
        (
            r#"#style(name="Linked",type="character",link="Heading2")
#span(style="Linked")[Text]"#,
            "Heading2",
        ),
        (r#"#heading(style="heading-1")[Text]"#, "Heading1"),
        (
            r#"#style(name="normal",font="Arial")
#heading[Heading]#blockquote[Quote]"#,
            "normal",
        ),
        (
            r#"#style(name="NORMAL",font="Arial")
#heading[Heading]#blockquote[Quote]"#,
            "NORMAL",
        ),
    ] {
        let parts = support::package(source);
        let styles = roxmltree::Document::parse(&parts["word/styles.xml"]).unwrap();
        let identifiers: std::collections::BTreeSet<_> = styles
            .descendants()
            .filter(|node| node.has_tag_name((namespace, "style")))
            .map(|node| node.attribute((namespace, "styleId")).unwrap())
            .collect();
        assert!(identifiers.contains(required_identifier), "{source}");
        for node in styles.descendants().filter(|node| {
            ["basedOn", "next", "link"]
                .iter()
                .any(|name| node.has_tag_name((namespace, *name)))
        }) {
            assert!(
                identifiers.contains(node.attribute((namespace, "val")).unwrap()),
                "{source}"
            );
        }
        let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        for reference in document.descendants().filter(|node| {
            ["pStyle", "rStyle"]
                .iter()
                .any(|name| node.has_tag_name((namespace, *name)))
        }) {
            assert!(
                identifiers.contains(reference.attribute((namespace, "val")).unwrap()),
                "{source}"
            );
        }
    }
}

#[test]
fn empty_named_font_does_not_erase_an_inherited_direct_typeface() {
    let parts = support::package(
        r#"#style(name="Empty Font",type="character",font="")
#span(font="Arial")[#span(style="Empty Font")[Text]]"#,
    );
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let fonts = document
        .descendants()
        .find(|node| node.has_tag_name((namespace, "rFonts")))
        .unwrap();
    assert_eq!(fonts.attribute((namespace, "ascii")), Some("Arial"));
}

#[test]
fn headings_match_reference() {
    support::compare_fixture("headings");
    support::compare_fixture("heading-styles");
}

#[test]
fn heading_flow_start_preserves_explicit_spacing_only() {
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    for (attributes, expected) in [
        ("", "0"),
        ("spacing_before=pt(7)", "140"),
        ("spacing={before=pt(3)}", "60"),
        ("spacing={spacing_before=pt(4)}", "80"),
        ("spacing_before=pt(7),spacing={before=pt(3)}", "60"),
        ("style=\"Title\"", "0"),
    ] {
        let source = format!(
            "#style(name=\"Title\",paragraph={{spacing_before=pt(12)}})#heading({attributes})[Title]"
        );
        let parts = support::package(&source);
        let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        let spacing = document
            .descendants()
            .find(|node| node.has_tag_name((namespace, "spacing")))
            .unwrap();
        assert_eq!(
            spacing.attribute((namespace, "before")),
            Some(expected),
            "{source}"
        );
    }
    let parts = support::package("#paragraph[]\n\n#heading[Title]");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    assert!(
        !document
            .descendants()
            .any(|node| node.has_tag_name((namespace, "spacing")))
    );
}

#[test]
fn a_heading_without_a_declared_style_warns_and_a_custom_marker_fails() {
    for (source, style) in [
        ("#heading(level=7)[Missing default]", "Heading7"),
        ("#heading(style=\"Missing\")[Missing style]", "Missing"),
    ] {
        let (parts, warnings) = support::package_with_warnings(source);
        let expected = format!("DOCX ignores heading.style; style {style:?} is not declared");
        assert!(
            warnings
                .iter()
                .any(|warning| warning.starts_with(&expected)),
            "{source}: {warnings:?}"
        );
        assert!(
            !parts["word/document.xml"].contains("<w:pStyle"),
            "{source}"
        );
    }
    assert!(
        mote::docx::emit(&mote::compile("#heading(marker=\"custom\")[Marker]").unwrap()).is_err()
    );
}

#[test]
fn derived_builtin_styles_preserve_inherited_direct_formatting() {
    support::compare_fixture("derived-builtin-styles");
}

#[test]
fn empty_border_and_tab_fields_preserve_alias_and_default_selection() {
    let parts = support::compare_fixture("empty-paragraph-properties");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    for node in document.descendants().filter(|node| node.is_element()) {
        for attribute in node
            .attributes()
            .filter(|attribute| attribute.namespace() == Some(namespace))
        {
            assert!(
                !attribute.value().is_empty(),
                "empty {} on {}",
                attribute.name(),
                node.tag_name().name()
            );
        }
    }
}

#[test]
fn declared_font_files_do_not_change_the_editable_word_package() {
    let baseline = support::package(
        r#"#font(name="Body",latin="Source Sans 3")#paragraph[#span(font="Body")[AV To Wa]]"#,
    );
    let with_files = support::package(
        r#"#font(name="Body",latin="Source Sans 3",files={regular="unavailable.ttf"})#paragraph[#span(font="Body")[AV To Wa]]"#,
    );
    assert_eq!(
        baseline.keys().collect::<Vec<_>>(),
        with_files.keys().collect::<Vec<_>>()
    );
    for (name, expected) in baseline {
        support::compare_xml(&with_files[&name], &expected, &name);
    }
}

#[test]
fn headings_warn_about_direct_run_metadata_without_rejecting_the_package() {
    let document = mote::compile("#heading(2,color='#28734f')[Title]").unwrap();
    let original = document.clone();
    let resolver = mote::source::FileAssetResolver::new(".").unwrap();
    let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    assert_eq!(document, original);
    assert!(
        generated
            .warnings
            .iter()
            .any(|warning| warning.contains("paragraph.color"))
    );
    let actual = support::unpack(generated.bytes);
    let expected = support::package("#heading(2)[Title]");
    support::compare_xml(
        &actual["word/document.xml"],
        &expected["word/document.xml"],
        "heading metadata",
    );
}

#[test]
fn leaf_controls_ignore_model_children_and_labels_remain_zero_width() {
    use mote::model::{Content, ElementKind};
    let source =
        "#paragraph[Before#tab()#line_break()#label('anchor')After]#break()#paragraph[Next]";
    let mut document = mote::compile(source).unwrap();
    fn add_children(content: &mut [Content]) {
        for node in content {
            if let Content::Element(element) = node {
                if matches!(
                    element.kind,
                    ElementKind::Tab
                        | ElementKind::LineBreak
                        | ElementKind::Label
                        | ElementKind::Break
                ) {
                    element.children.push(Content::Text("{}".into()));
                } else {
                    add_children(&mut element.children);
                }
            }
        }
    }
    add_children(&mut document.content);
    let original = document.clone();
    let actual = support::unpack(mote::docx::emit(&document).unwrap());
    assert_eq!(document, original);
    let expected = support::package(source);
    support::compare_xml(
        &actual["word/document.xml"],
        &expected["word/document.xml"],
        "leaf controls",
    );
}

#[test]
fn a_signed_rotation_is_kept_and_a_signed_shadow_direction_becomes_a_positive_angle() {
    let package = support::package(
        "#shape(position={width=inch(2),height=inch(1)},rotation=-15,shadow={angle=-45})",
    );
    let document = &package["word/document.xml"];
    assert!(document.contains("rot=\"-900000\""));
    // ST_PositiveFixedAngle admits no negative direction: -45 degrees is 315.
    assert!(document.contains("dir=\"18900000\""));
}

#[test]
fn task_controls_inherit_drawing_text_without_changing_following_lists() {
    let package = support::package(
        "#shape(position={width=inch(2),height=inch(1)},text={size=pt(14),color='#112233'})[\
         #list[#item(checked=true)[Inside]#item(checked=false)[Pending]]\
         ]#list[#item(checked=true)[Outside]]",
    );
    let document = &package["word/document.xml"];
    let symbol_properties = "<w:rFonts w:ascii=\"MS Gothic\" w:hAnsi=\"MS Gothic\"/><w:color w:val=\"112233\"/><w:sz w:val=\"28\"/><w:szCs w:val=\"28\"/>";
    assert_eq!(document.matches(symbol_properties).count(), 2);
    let spacer = "<w:rPr><w:rFonts w:ascii=\"Aptos\" w:hAnsi=\"Aptos\"/><w:color w:val=\"112233\"/><w:sz w:val=\"28\"/><w:szCs w:val=\"28\"/></w:rPr><w:t xml:space=\"preserve\"> </w:t>";
    assert_eq!(document.matches(spacer).count(), 2);
    assert!(document.contains("<w:rPr><w:rFonts w:ascii=\"MS Gothic\" w:hAnsi=\"MS Gothic\"/></w:rPr><w:t xml:space=\"preserve\">☒</w:t>"));
}

const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

fn child_names(parent: roxmltree::Node<'_, '_>) -> Vec<String> {
    parent
        .children()
        .filter(roxmltree::Node::is_element)
        .map(|child| child.tag_name().name().to_owned())
        .collect()
}

#[test]
fn paragraph_properties_follow_the_sequence_of_the_schema() {
    let package = support::package(
        r##"#style(name="Lead", bold=true)
#heading(level=1, style="Lead", numbering={enabled=true}, keep_with_next=true, keep_together=true, page_break_before=false, widow_control=true, border={top="single"}, fill="#eeeeee", tabs={ {position=inch(1)} }, spacing_before=pt(6), indent={left=pt(12)}, alignment="center", outline_level=2)[Numbered]
#list[#item(indent={left=pt(20)})[Item]]"##,
    );
    let document = roxmltree::Document::parse(&package["word/document.xml"]).unwrap();
    let properties: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "pPr")))
        .map(child_names)
        .collect();
    assert_eq!(
        properties[0],
        [
            "pStyle",
            "keepNext",
            "keepLines",
            "pageBreakBefore",
            "widowControl",
            "numPr",
            "pBdr",
            "shd",
            "tabs",
            "spacing",
            "ind",
            "jc",
            "outlineLvl"
        ]
    );
    assert_eq!(properties[1], ["numPr", "spacing", "ind"]);
}

#[test]
fn a_declared_style_with_two_fills_writes_one_paragraph_shading() {
    let package = support::package(
        r##"#style(name="Shaded", fill="#ff0000", background="#00ff00")
#paragraph(style="Shaded")[Text]"##,
    );
    let styles = roxmltree::Document::parse(&package["word/styles.xml"]).unwrap();
    let properties = styles
        .descendants()
        .find(|node| {
            node.has_tag_name((WORD_NAMESPACE, "style"))
                && node.attribute((WORD_NAMESPACE, "styleId")) == Some("Shaded")
        })
        .and_then(|style| {
            style
                .children()
                .find(|child| child.has_tag_name((WORD_NAMESPACE, "pPr")))
        })
        .unwrap();
    let shadings: Vec<_> = properties
        .children()
        .filter(|child| child.has_tag_name((WORD_NAMESPACE, "shd")))
        .map(|shading| shading.attribute((WORD_NAMESPACE, "fill")).unwrap())
        .collect();
    assert_eq!(shadings, ["FF0000"]);
}

#[test]
fn underline_styles_are_written_as_members_of_the_word_enumeration() {
    for (style, word) in [
        ("single", "single"),
        ("double", "double"),
        ("dotted", "dotted"),
        ("dashed", "dash"),
        ("wave", "wave"),
        ("none", "none"),
    ] {
        let package = support::package(&format!("#underline(style=\"{style}\")[Text]"));
        assert!(
            package["word/document.xml"].contains(&format!("<w:u w:val=\"{word}\"/>")),
            "{style}"
        );
    }
}

#[test]
fn a_value_outside_a_word_enumeration_warns_and_gives_way_to_a_member() {
    let (parts, warnings) = support::package_with_warnings(include_str!(
        "fixtures/output-fallbacks/unmapped-values.mote"
    ));
    for expected in [
        "DOCX ignores border.top.style; \"zigzag\" is not among its values, so a single line is drawn",
        "DOCX ignores tabs.alignment; \"middle\" is not among its values, so the stop is aligned left",
        "DOCX ignores tabs.leader; \"stars\" is not among its values, so the stop has no leader",
        "DOCX ignores borders.insideH.style; \"frilly\" is not among its values, so a single line is drawn",
        "DOCX ignores borders.top.style; \"squiggle\" is not among its values, so a single line is drawn",
        "DOCX ignores cell.vertical_alignment; \"weird\" is not among its values, so the alignment is left to Word",
    ] {
        assert!(
            warnings.iter().any(|warning| warning.starts_with(expected)),
            "{expected}: {warnings:?}"
        );
    }
    let document = &parts["word/document.xml"];
    for expected in [
        r#"<w:top w:val="single" w:sz="4" w:space="0" w:color="BFBFBF"/><w:left w:val="double""#,
        r#"<w:bottom w:val="dashed""#,
        r#"<w:tab w:val="left" w:pos="1440"/><w:tab w:val="decimal" w:pos="2880" w:leader="dot"/><w:tab w:val="end" w:pos="4320"/>"#,
        r#"<w:insideH w:val="single""#,
        r#"<w:trHeight w:val="600" w:hRule="atLeast"/>"#,
        r#"<w:trHeight w:val="600" w:hRule="exact"/>"#,
        r#"<w:vAlign w:val="center"/>"#,
        r#"<m:rad><m:radPr><m:degHide m:val="1"/></m:radPr><m:deg/><m:e>"#,
        r#"<m:rad><m:deg><m:r>"#,
    ] {
        assert!(document.contains(expected), "{expected}");
    }
    for unknown in ["zigzag", "middle", "stars", "frilly", "squiggle"] {
        assert!(!document.contains(unknown), "{unknown}");
    }
    assert!(!parts["word/styles.xml"].contains("weird"));
}

#[test]
fn every_story_that_ends_with_a_table_is_closed_by_a_paragraph() {
    let package = support::package(include_str!(
        "fixtures/sections-stories/closing-tables.mote"
    ));
    let mut closed = Vec::new();
    for part in [
        "word/document.xml",
        "word/header1.xml",
        "word/footer1.xml",
        "word/footnotes.xml",
        "word/endnotes.xml",
        "word/comments.xml",
    ] {
        let document = roxmltree::Document::parse(&package[part]).unwrap();
        for container in document.descendants().filter(|node| {
            [
                "hdr",
                "ftr",
                "footnote",
                "endnote",
                "comment",
                "txbxContent",
                "tc",
            ]
            .iter()
            .any(|name| node.has_tag_name((WORD_NAMESPACE, *name)))
        }) {
            let children = child_names(container);
            assert_eq!(
                children.last().map(String::as_str),
                Some("p"),
                "{part}: {children:?}"
            );
            if children.iter().any(|child| child == "tbl") {
                closed.push(container.tag_name().name().to_owned());
            }
        }
        let body = document
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "body")));
        if let Some(body) = body {
            let children = child_names(body);
            assert_eq!(children[children.len() - 2..], ["p", "sectPr"]);
        }
    }
    closed.sort();
    assert_eq!(
        closed,
        [
            "comment",
            "endnote",
            "footnote",
            "ftr",
            "hdr",
            "tc",
            "txbxContent"
        ]
    );
}
