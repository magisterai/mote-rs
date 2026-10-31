use mote::{
    compile,
    formatting::{ResolutionIssue, StyleResolver},
    model::{Content, Element, ElementKind, Properties, Unit, Value},
};

fn points(value: f64) -> Value {
    Value::Length {
        value,
        unit: Unit::Point,
    }
}
fn object(properties: &Properties, name: &str) -> Properties {
    let Value::Object(fields) = &properties[name] else {
        panic!("expected {name} object")
    };
    fields.clone()
}
fn element(content: &Content) -> &Element {
    let Content::Element(element) = content else {
        panic!("expected element")
    };
    element
}

#[test]
fn inheritance_merges_paragraph_fields_and_keeps_explicit_false_and_zero() {
    let document=compile(r##"#style(name="Base",size=pt(9),run={font="body",bold=true,color="#123"},
        paragraph={spacing={before=pt(2),after=pt(6),line={rule="multiple",value=1.2}},indent={left=pt(12),right=pt(3)},keep_with_next=true})
        #style(name="Body",based_on="Base",size=pt(11),run={bold=false},
        paragraph={spacing={after=pt(0)},indent={right=pt(0)},keep_with_next=false})
        #font(name="body",latin="Aptos")"##).unwrap();
    let original = document.clone();
    let resolved = StyleResolver::new(&document).resolve_style("Body");
    assert!(resolved.issues.is_empty());
    assert_eq!(
        resolved.run,
        Properties::from([
            ("font".into(), Value::String("body".into())),
            ("size".into(), points(11.0)),
            ("bold".into(), Value::Boolean(false)),
            ("color".into(), Value::Color("#112233".into())),
        ])
    );
    assert_eq!(
        object(&resolved.paragraph, "spacing")["before"],
        points(2.0)
    );
    assert_eq!(object(&resolved.paragraph, "spacing")["after"], points(0.0));
    assert_eq!(
        object(&resolved.paragraph, "spacing")["line"],
        Value::Object(Properties::from([
            ("rule".into(), Value::String("multiple".into())),
            ("value".into(), Value::Number(1.2))
        ]))
    );
    assert_eq!(
        object(&resolved.paragraph, "indent"),
        Properties::from([("left".into(), points(12.0)), ("right".into(), points(0.0))])
    );
    assert_eq!(resolved.paragraph["keep_with_next"], Value::Boolean(false));
    assert_eq!(resolved.font.unwrap().latin.as_deref(), Some("Aptos"));
    assert_eq!(document, original);
}

#[test]
fn nested_fields_override_flat_fields_before_inheritance_and_direct_overrides() {
    let document = compile(
        r#"#style(name="Body",size=pt(9),run={size=pt(11)},
        spacing={before=pt(1),after=pt(6)},paragraph={spacing={after=pt(3)}})
        #paragraph(style="Body",spacing_after=pt(0))[Text]"#,
    )
    .unwrap();
    let resolver = StyleResolver::new(&document);
    let named = resolver.resolve_style("Body");
    assert_eq!(named.run["size"], points(11.0));
    assert_eq!(
        object(&named.paragraph, "spacing"),
        Properties::from([("after".into(), points(3.0))])
    );
    let paragraph = element(&document.content[0]);
    let resolved = resolver.resolve_paragraph(&paragraph.attributes);
    assert_eq!(resolved.paragraph["spacing_after"], points(0.0));
    assert!(!resolved.paragraph.contains_key("spacing"));
}

#[test]
fn spacing_aliases_override_across_styles_in_both_directions_without_losing_other_fields() {
    for (base, derived, expected) in [
        (
            "spacing={before=pt(2),after=pt(3),line=pt(12)}",
            "spacing_before=pt(0)",
            "spacing_before",
        ),
        (
            "spacing_before=pt(2),spacing={after=pt(3),line=pt(12)}",
            "spacing={before=pt(0)}",
            "before",
        ),
    ] {
        let document = compile(&format!(
            "#style(name=\"Base\",{base})\n#style(name=\"Body\",based_on=\"Base\",{derived})"
        ))
        .unwrap();
        let resolved = StyleResolver::new(&document).resolve_style("Body");
        let spacing = object(&resolved.paragraph, "spacing");
        assert_eq!(spacing["after"], points(3.0));
        assert_eq!(spacing["line"], points(12.0));
        if expected == "before" {
            assert_eq!(spacing["before"], points(0.0));
            assert!(!resolved.paragraph.contains_key("spacing_before"));
        } else {
            assert_eq!(resolved.paragraph["spacing_before"], points(0.0));
            assert!(!spacing.contains_key("before"));
        }
    }
}

#[test]
fn inline_precedence_applies_intent_then_named_style_then_direct_values_without_sibling_leakage() {
    let document = compile(
        r##"#style(name="Accent",type="character",run={bold=false,color="#123",font="body"})
        #span(bold=true,size=pt(12))[#strong(style="Accent",color="#ABC")[child]#emphasis[sibling]]
        #font(name="body",latin="Aptos")"##,
    )
    .unwrap();
    let resolver = StyleResolver::new(&document);
    let outer = element(&element(&document.content[0]).children[0]);
    let outer_style = resolver.resolve_inline(outer, &Properties::new());
    let first = resolver.resolve_inline(element(&outer.children[0]), &outer_style.run);
    let second = resolver.resolve_inline(element(&outer.children[1]), &outer_style.run);
    assert_eq!(first.run["bold"], Value::Boolean(false));
    assert_eq!(first.run["size"], points(12.0));
    assert_eq!(first.run["color"], Value::String("#ABC".into()));
    assert_eq!(first.font.unwrap().latin.as_deref(), Some("Aptos"));
    assert_eq!(second.run["bold"], Value::Boolean(true));
    assert_eq!(second.run["italic"], Value::Boolean(true));
    assert!(!second.run.contains_key("color"));
    assert!(second.font.is_none());
    assert_eq!(outer_style.run.len(), 2);
}

#[test]
fn semantic_inline_resolution_retains_intent_without_treating_patterns_as_named_styles() {
    let document = compile(
        r##"#style(name="double",bold=true)
        #paragraph[#underline(style="double",color="#123")[Line]#strike(kind="double")[Removed]#highlight(fill="#fff2cc")[Marked]#small_caps[Small]#subscript[Low]#superscript[High]]"##,
    )
    .unwrap();
    let resolver = StyleResolver::new(&document);
    let inherited = Properties::from([("italic".into(), Value::Boolean(true))]);
    let children = &element(&document.content[0]).children;
    let resolved: Vec<_> = children
        .iter()
        .map(|child| resolver.resolve_inline(element(child), &inherited))
        .collect();
    for style in &resolved {
        assert!(style.issues.is_empty());
        assert_eq!(style.run["italic"], Value::Boolean(true));
        assert!(!style.run.contains_key("bold"));
    }
    assert_eq!(resolved[0].run["underline"], Value::Boolean(true));
    assert_eq!(resolved[0].run["underline_style"].to_text(), "double");
    assert_eq!(resolved[0].run["underline_color"].to_text(), "#112233");
    assert_eq!(resolved[1].run["strike"], Value::Boolean(true));
    assert_eq!(resolved[1].run["double_strike"], Value::Boolean(true));
    assert_eq!(resolved[2].run["fill"].to_text(), "#fff2cc");
    for (style, property) in resolved[3..]
        .iter()
        .zip(["small_caps", "subscript", "superscript"])
    {
        assert_eq!(style.run[property], Value::Boolean(true));
    }
    assert_eq!(inherited.len(), 1);
}

#[test]
fn exact_lookup_precedes_deterministic_normalized_lookup_and_preserves_unicode_identity() {
    let document = compile(
        r#"#style(name="Body-Text",bold=true)
        #style(name="Body_Text",italic=true)
        #style(name="标题",underline=true)"#,
    )
    .unwrap();
    let resolver = StyleResolver::new(&document);
    assert_eq!(
        resolver.resolve_style("Body_Text").run,
        Properties::from([("italic".into(), Value::Boolean(true))])
    );
    assert_eq!(
        resolver.resolve_style("body text").run,
        Properties::from([("bold".into(), Value::Boolean(true))])
    );
    assert_eq!(
        resolver.resolve_style("标题").run,
        Properties::from([("underline".into(), Value::Boolean(true))])
    );
    assert_eq!(
        resolver.resolve_style("另一个").issues,
        [ResolutionIssue::MissingStyle("另一个".into())]
    );
}

#[test]
fn missing_parents_and_cycles_retain_discovered_properties_and_report_issues() {
    let document = compile(
        r#"#style(name="Child",based_on="Missing",bold=true,next="Missing",link="Missing")
        #style(name="First",based_on="Second",bold=false)
        #style(name="Second",based_on="First",bold=true,italic=true)"#,
    )
    .unwrap();
    let resolver = StyleResolver::new(&document);
    let child = resolver.resolve_style("Child");
    assert_eq!(
        child.issues,
        [ResolutionIssue::MissingStyle("Missing".into())]
    );
    assert_eq!(
        child.run,
        Properties::from([("bold".into(), Value::Boolean(true))])
    );
    let cyclic = resolver.resolve_style("First");
    assert_eq!(
        cyclic.issues,
        [ResolutionIssue::CyclicStyle("First".into())]
    );
    assert_eq!(
        cyclic.run,
        Properties::from([
            ("bold".into(), Value::Boolean(false)),
            ("italic".into(), Value::Boolean(true))
        ])
    );
    let missing = resolver.resolve_inline(
        &Element {
            kind: ElementKind::Span,
            attributes: Properties::from([
                ("style".into(), Value::String("Absent".into())),
                ("bold".into(), Value::Boolean(false)),
            ]),
            children: vec![],
        },
        &Properties::from([("italic".into(), Value::Boolean(true))]),
    );
    assert_eq!(
        missing.issues,
        [ResolutionIssue::MissingStyle("Absent".into())]
    );
    assert_eq!(
        missing.run,
        Properties::from([
            ("bold".into(), Value::Boolean(false)),
            ("italic".into(), Value::Boolean(true))
        ])
    );
}

#[test]
fn font_aliases_use_exact_names_without_recursion_or_invented_script_fallbacks() {
    let document=compile(r#"#font(name="body",latin="Aptos",east_asian="Yu Gothic",complex_script="Arial",word_line_height=1.25)
        #font(name="partial",east_asian="body")"#).unwrap();
    let resolver = StyleResolver::new(&document);
    let body = resolver.resolve_font("body");
    assert!(body.declared);
    assert_eq!(body.latin.as_deref(), Some("Aptos"));
    assert_eq!(body.east_asian.as_deref(), Some("Yu Gothic"));
    assert_eq!(body.complex_script.as_deref(), Some("Arial"));
    assert_eq!(body.word_line_height, Some(1.25));
    let partial = resolver.resolve_font("partial");
    assert!(partial.latin.is_none());
    assert_eq!(partial.east_asian.as_deref(), Some("body"));
    let direct = resolver.resolve_font("Body");
    assert!(!direct.declared);
    assert_eq!(direct.latin.as_deref(), Some("Body"));
}

#[test]
fn forward_references_use_final_replacements_and_implicit_output_defaults_remain_absent() {
    let document = compile(
        r#"#span(style="Accent")[Ready]
        #style(name="Accent",type="character",bold=true)
        #style(name="Accent",type="character",italic=true)
        #style(name="Normal",bold=true)"#,
    )
    .unwrap();
    let resolver = StyleResolver::new(&document);
    let span = element(&element(&document.content[0]).children[0]);
    let resolved = resolver.resolve_inline(span, &Properties::new());
    assert_eq!(
        resolved.run,
        Properties::from([("italic".into(), Value::Boolean(true))])
    );
    assert!(
        resolver
            .resolve_paragraph(&Properties::new())
            .run
            .is_empty()
    );
}

#[test]
fn long_style_chains_are_resolved_iteratively() {
    let mut source = String::from("#style(name=\"Style0\",bold=true)\n");
    for index in 1..1500 {
        source.push_str(&format!(
            "#style(name=\"Style{index}\",based_on=\"Style{}\")\n",
            index - 1
        ));
    }
    let document = compile(&source).unwrap();
    let resolved = StyleResolver::new(&document).resolve_style("Style1499");
    assert!(resolved.issues.is_empty());
    assert_eq!(resolved.run["bold"], Value::Boolean(true));
}
