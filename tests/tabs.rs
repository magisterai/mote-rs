#[path = "docx/support.rs"]
mod support;

use mote::{compile, json::to_json};

#[test]
fn inline_tabs_match_the_captured_model_and_package() {
    support::compare_fixture("inline-tabs");
}

#[test]
fn tabs_are_inline_nodes_and_metadata_uses_shared_merging() {
    let document = compile(
        r#"#paragraph[Left#tab(note = "outer", properties = { note = "inner", extra = 2 })Right]"#,
    )
    .unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    let tab = &output["content"][0]["children"][1];
    assert_eq!(tab["kind"], "tab");
    assert_eq!(tab["category"], "inline");
    assert_eq!(tab["attributes"]["note"], "inner");
    assert_eq!(tab["attributes"]["extra"], 2.0);
    assert!(tab.get("children").is_none());
    assert!(mote::docx::emit(&document).is_err());
}

#[test]
fn tab_body_positionals_and_nonobject_properties_are_invalid() {
    for source in ["#tab(1)", "#tab()[text]", "#tab(properties = false)"] {
        assert!(compile(source).is_err(), "{source}");
    }
}

#[test]
fn ordinary_edge_tabs_trim_while_preserved_tabs_survive() {
    for (source, expected_tabs) in [
        ("#paragraph[#tab()  text  #tab()]", 0),
        ("#paragraph[before#tab()#tab()after]", 2),
        ("#paragraph[before#line_break()#tab()after]", 0),
        ("#paragraph[before#tab()#line_break()after]", 0),
        ("#paragraph[#code(preserve_space = true)[#tab()]  text]", 1),
        ("#paragraph[text  #code(preserve_space = true)[#tab()]]", 1),
    ] {
        let package = support::package(source);
        assert_eq!(
            package["word/document.xml"].matches("<w:tab/>").count(),
            expected_tabs,
            "{source}"
        );
    }
}

#[test]
fn edge_trimming_does_not_hide_unsupported_tab_metadata() {
    for source in [
        "#paragraph[#tab(note = 1)text]",
        "#list[#item[text#tab(note = 1)]]",
    ] {
        let document = compile(source).unwrap();
        assert!(
            mote::docx::emit(&document)
                .unwrap_err()
                .to_string()
                .contains("tab attribute")
        );
    }
}

fn native_fragments(body: &str) -> Vec<(String, f64, f64)> {
    let source = format!(
        "#document(page = {{size = \"letter\"}}, margin = {{left = inch(1), right = inch(1)}})\n{body}"
    );
    let document = compile(&source).unwrap();
    let resolver = mote::source::FileAssetResolver::from_current_directory().unwrap();
    let planned = mote::render::plan(&document, &resolver, None).unwrap();
    planned
        .output
        .pages
        .iter()
        .flat_map(|page| &page.lines)
        .flat_map(|placed| &placed.line.fragments)
        .map(|fragment| (fragment.text.clone(), fragment.x, fragment.width))
        .collect()
}

fn fragment(fragments: &[(String, f64, f64)], text: &str) -> (f64, f64) {
    let (_, x, width) = fragments
        .iter()
        .find(|(candidate, _, _)| candidate.trim() == text)
        .unwrap_or_else(|| panic!("{text} in {fragments:?}"));
    (*x, *width)
}

#[test]
fn native_tab_stops_align_text_to_authored_and_default_positions() {
    let margin = 72.0;
    let right = native_fragments("#paragraph(tabs = {right = inch(4)})[Left#tab()Right]");
    let (x, width) = fragment(&right, "Right");
    assert!((x + width - (margin + 288.0)).abs() < 0.01, "{right:?}");

    let centered = native_fragments(
        "#paragraph(tabs = {position = inch(3), alignment = \"center\"})[A#tab()Centred]",
    );
    let (x, width) = fragment(&centered, "Centred");
    assert!(
        (x + width / 2.0 - (margin + 216.0)).abs() < 0.01,
        "{centered:?}"
    );

    let decimal = |amount: &str| {
        let fragments = native_fragments(&format!(
            "#paragraph(tabs = {{decimal = inch(3)}})[Fee#tab(){amount}]"
        ));
        fragment(&fragments, amount)
    };
    let (short, _) = decimal("7.5");
    let (long, _) = decimal("17.5");
    assert!(long < short);

    let leader = native_fragments(
        "#paragraph(tabs = {position = inch(4), alignment = \"right\", leader = \"dot\"})[Name#tab()12]",
    );
    let (dots_x, dots_width) = leader
        .iter()
        .find(|(text, _, _)| text.starts_with("...."))
        .map(|(_, x, width)| (*x, *width))
        .unwrap();
    let (number_x, _) = fragment(&leader, "12");
    assert!((dots_x + dots_width - number_x).abs() < 0.01, "{leader:?}");

    let defaults = native_fragments("#paragraph[A#tab()after]");
    assert!((fragment(&defaults, "after").0 - (margin + 36.0)).abs() < 0.01);
}

#[test]
fn unusable_tab_stops_report_their_missing_native_effect() {
    let document = compile("#paragraph(tabs = {stops = {position = pt(36)}})[A#tab()B]").unwrap();
    let resolver = mote::source::FileAssetResolver::from_current_directory().unwrap();
    let planned = mote::render::plan(&document, &resolver, None).unwrap();
    assert!(
        planned.warnings.iter().any(
            |warning| warning.ends_with(".tabs is retained but has no native rendering effect")
        )
    );
    let usable = compile("#paragraph(tabs = {right = pt(72)})[A#tab()B]").unwrap();
    let planned = mote::render::plan(&usable, &resolver, None).unwrap();
    assert!(
        !planned
            .warnings
            .iter()
            .any(|warning| warning.contains(".tabs"))
    );
}
