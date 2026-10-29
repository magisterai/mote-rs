use mote::{render, source::FileAssetResolver};

fn warnings(source: &str) -> Vec<String> {
    let document = mote::compile(source).unwrap();
    let resolver = FileAssetResolver::from_current_directory().unwrap();
    render::plan(&document, &resolver, None).unwrap().warnings
}
#[test]
fn implemented_text_properties_do_not_produce_stale_support_warnings() {
    let result = warnings(
        "#paragraph(font=\"Helvetica\",size=pt(12),border={bottom={color=\"#123\"}})[#span(double_strike=true)[Text]]",
    );
    assert!(result.is_empty(), "{result:?}");
}
#[test]
fn retained_properties_report_the_specific_native_limit() {
    let result = warnings(
        "#document(columns={count=2,equal_width=false})\n#paragraph(tabs={stops={position=pt(36)}},border={top={color=\"#123\"},bottom={color=\"#123\",width=pt(2)}})[Text]\n#list[#item(checkbox={checked=true})[Task]]",
    );
    for field in [
        "equal_width=false",
        ".tabs",
        ".border.top",
        ".border.bottom.width",
        ".checkbox",
    ] {
        assert!(
            result.iter().any(|warning| warning.contains(field)),
            "missing {field}: {result:?}"
        );
    }
}
#[test]
fn flattened_children_and_marker_only_items_are_visible_diagnostics() {
    let result = warnings(
        "#blockquote[#paragraph(alignment=\"right\")[Quoted]#list[#item[Flattened]]]\n#list[#item[]]",
    );
    assert!(
        result
            .iter()
            .any(|warning| warning.contains("flattens child blocks"))
    );
    assert!(result.iter().any(|warning| warning.contains("marker-only")));
}
