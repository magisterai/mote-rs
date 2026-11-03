use mote::evaluation::EvaluationOptions;

#[test]
fn tagged_empty_lua_content_stays_empty_while_plain_maps_remain_maps() {
    let document = mote::compile_with_options("#function identity(body) return body end\n#paragraph[Before#identity[]after]\n#function empty() return document.paragraph(document.tab(),document.span('')) end\n#empty()", EvaluationOptions { enable_lua:true,..Default::default() }).unwrap();
    assert!(
        !document
            .content
            .iter()
            .map(|node| node.to_text())
            .collect::<String>()
            .contains("{}")
    );
    let plain_map = mote::compile_with_options(
        "#function empty() return {} end\n#empty()",
        EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        plain_map
            .content
            .iter()
            .map(|node| node.to_text())
            .collect::<String>(),
        "{}"
    );
}
