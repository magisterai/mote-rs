//! Data returned by functions, and zero-based static paths, with Lua enabled.
use mote::{
    evaluation::EvaluationOptions,
    model::{Content, Document, ElementKind},
    source::{AssetRequest, FileAssetResolver, ResolvedAsset},
};
use std::io::{Cursor, Read};

const NUMBERS: &str = "#function numbers()\n  return { 1, 2, 3 }\nend\n";

fn compile(source: &str) -> Result<Document, mote::diagnostics::Diagnostic> {
    mote::compile_with_options(
        source,
        EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
}

fn text(source: &str) -> String {
    compile(source)
        .unwrap()
        .content
        .iter()
        .map(Content::to_text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// The element kinds and the text of content, with adjacent text joined.
fn outline(content: &[Content]) -> String {
    let mut outline = String::new();
    let mut text = String::new();
    for node in content {
        match node {
            Content::Text(part) => text.push_str(part),
            Content::Element(element) => {
                if !text.is_empty() {
                    outline.push_str(&format!("'{text}'"));
                    text.clear();
                }
                outline.push_str(&format!(
                    "{}({})",
                    element.kind.as_text(),
                    self::outline(&element.children)
                ));
            }
        }
    }
    if !text.is_empty() {
        outline.push_str(&format!("'{text}'"));
    }
    outline
}

fn regions(source: &str, enable_lua: bool) -> Result<String, mote::diagnostics::Diagnostic> {
    let resolver = |_request: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "regions.csv".into(),
            bytes: b"region,revenue\nNorth,62\nSouth,47\n".to_vec(),
        })
    };
    let document = mote::compile_with_options(
        &format!("#let(rows = csv(\"regions.csv\"))\n{source}"),
        EvaluationOptions {
            enable_lua,
            asset_resolver: Some(&resolver),
            ..Default::default()
        },
    )?;
    Ok(document
        .content
        .iter()
        .map(Content::to_text)
        .collect::<Vec<_>>()
        .join("\n"))
}

fn chart_xml(document: &Document) -> String {
    let resolver = FileAssetResolver::new(".").unwrap();
    let package = mote::docx::emit_with_assets(document, &resolver, None).unwrap();
    assert!(package.warnings.is_empty(), "{:?}", package.warnings);
    let mut archive = zip::ZipArchive::new(Cursor::new(package.bytes)).unwrap();
    let mut content = String::new();
    archive
        .by_name("word/charts/chart1.xml")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    content
}

#[test]
fn an_array_result_is_data_in_a_binding_a_length_a_path_and_a_loop() {
    assert_eq!(
        text(&format!(
            "{NUMBERS}#let(values = numbers())\n#let(count = #values, first = values[1], second = values.1)\n#paragraph[count #count first #first second #second]"
        )),
        "count 3 first 1 second 2"
    );
    assert_eq!(
        text(&format!(
            "{NUMBERS}#let(values = numbers())\n#for value in values {{#paragraph[item #value]}}\n#for value in numbers() {{#paragraph[direct #value]}}"
        )),
        "item 1\nitem 2\nitem 3\ndirect 1\ndirect 2\ndirect 3"
    );
    let nested = "#function records()\n  return { { name = \"North\", totals = { 1, 2 } }, { name = \"South\", totals = { 3 } } }\nend\n#let(rows = records(), last = rows[2].totals[1])\n#let(first = rows.0.name)\n#for row in rows {#paragraph[#row.name]}\n#paragraph[#first #last]";
    assert_eq!(text(nested), "North\nSouth\nNorth 3");
    assert_eq!(
        text(
            "#function words()\n  return { \"a\", \"b\" }\nend\n#function count(values)\n  return #values\nend\n#let(total = count(words()))\n#paragraph[#total]"
        ),
        "2"
    );
}

#[test]
fn an_array_result_supplies_a_chart_and_the_rows_of_a_table() {
    let document = compile(&format!(
        "{NUMBERS}#function names()\n  return {{ \"A\", \"B\", \"C\" }}\nend\n#let(values = numbers(), labels = names())\n#chart(data = {{ categories = labels, series = {{ {{ name = \"Count\", values = values }} }} }})"
    ))
    .unwrap();
    let xml = chart_xml(&document);
    for expected in [
        "<c:v>A</c:v>",
        "<c:v>C</c:v>",
        "<c:v>1</c:v>",
        "<c:v>2</c:v>",
        "<c:v>3</c:v>",
    ] {
        assert!(xml.contains(expected), "{expected}: {xml}");
    }

    let document = compile(
        "#function records()\n  return { { name = \"North\", revenue = 62 }, { name = \"South\", revenue = 47 } }\nend\n#table[#for record in records() {#row[#cell[#record.name] #cell[#record.revenue]]}]",
    )
    .unwrap();
    let Content::Element(table) = &document.content[0] else {
        panic!("{:?}", document.content);
    };
    assert_eq!(table.kind, ElementKind::Table);
    let rows: Vec<String> = table
        .children
        .iter()
        .filter_map(|row| match row {
            Content::Element(row) if row.kind == ElementKind::Row => Some(
                row.children
                    .iter()
                    .map(Content::to_text)
                    .collect::<Vec<_>>()
                    .join("|"),
            ),
            _ => None,
        })
        .collect();
    assert_eq!(rows, ["North|62", "South|47"]);
}

#[test]
fn a_result_called_in_content_is_inserted_as_content() {
    let callout =
        "#function callout(title, body)\n  return {document.strong(title), \": \", body}\nend\n";
    let expected = "paragraph(strong('Note')': Figures are 'strong('unaudited')'.')";
    assert_eq!(
        outline(
            &compile(&format!(
                "{callout}#paragraph[#callout(\"Note\")[Figures are *unaudited*.]]"
            ))
            .unwrap()
            .content
        ),
        expected
    );
    // An array that holds content is inserted in order through a binding and
    // through another function.
    assert_eq!(
        outline(
            &compile(
                "#function parts()\n  return {document.strong(\"Note\"), \": Figures are \"}\nend\n#function unaudited()\n  return {parts(), document.strong(\"unaudited\"), \".\"}\nend\n#let(note = unaudited())\n#paragraph[#note]"
            )
            .unwrap()
            .content
        ),
        expected
    );
    assert_eq!(
        text(&format!(
            "{NUMBERS}#paragraph[#numbers()]\n#let(values = numbers())\n#paragraph[#values]"
        )),
        "123\n1, 2, 3"
    );
}

#[test]
fn chart_data_the_chart_cannot_read_is_reported_by_every_output() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for (data, expected) in [
        (
            "{ categories = \"A\", values = { 1 } }",
            "chart.data.categories is not an array and gives the chart no categories",
        ),
        (
            "{ values = 7 }",
            "chart.data.values is not an array and gives the chart no series",
        ),
        (
            "{ series = { name = \"One\" } }",
            "chart.data.series is not an array and gives the chart no series",
        ),
        (
            "{ series = { { name = \"One\", values = 7 }, { values = { 1 } } } }",
            "chart.data.series entry 1 values is not an array and gives the chart no series",
        ),
        (
            "{ series = { 17, { values = { 1 } } } }",
            "chart.data.series entry 1 is not a table with a values array and gives the chart no series",
        ),
        (
            "{ values = { 1, \"many\", { name = \"three\" } } }",
            "chart.data.values entry 2 is not a number and is charted as zero",
        ),
        (
            "{ values = { 1, \"many\", { name = \"three\" } } }",
            "chart.data.values entry 3 is not a number and is charted as zero",
        ),
    ] {
        let chart = format!("#chart(data = {data})");
        let document = mote::compile(&chart).unwrap();
        let deck = mote::compile(&format!(
            "#slide[#box(position={{x=pt(10),y=pt(10),width=pt(300),height=pt(200)}})[{chart}]]"
        ))
        .unwrap();
        for (output, warnings) in [
            (
                "document PDF and PNG",
                mote::render::plan(&document, &resolver, None)
                    .unwrap()
                    .warnings,
            ),
            (
                "DOCX",
                mote::docx::emit_with_assets(&document, &resolver, None)
                    .unwrap()
                    .warnings,
            ),
            (
                "slide PDF and PNG",
                mote::render::plan_with_layout(
                    &deck,
                    &resolver,
                    None,
                    mote::render::LayoutMode::Presentation,
                )
                .unwrap()
                .warnings,
            ),
            (
                "PPTX",
                mote::pptx::emit_with_warnings(&deck).unwrap().warnings,
            ),
        ] {
            assert!(
                warnings.iter().any(|warning| warning.contains(expected)),
                "{data} {output}: {warnings:?}"
            );
        }
    }
    // A content body is not an array of numbers.
    let document = compile(
        "#function plot(body)\n  return document.chart({ data = { values = body } })\nend\n#plot()[1 2 3]",
    )
    .unwrap();
    let warnings = mote::docx::emit_with_assets(&document, &resolver, None)
        .unwrap()
        .warnings;
    assert!(
        warnings
            .iter()
            .any(|warning| warning.contains("chart.data.values is not an array")),
        "{warnings:?}"
    );
    let valid = mote::compile(
        "#chart(data = { categories = { \"A\", 2 }, series = { { values = { 1, \"2.5\" } } } })",
    )
    .unwrap();
    assert!(
        mote::docx::emit_with_assets(&valid, &resolver, None)
            .unwrap()
            .warnings
            .is_empty()
    );
}

#[test]
fn a_static_numeric_path_in_an_expression_means_the_same_with_lua_enabled() {
    for source in [
        "#let(first = rows.0.region, second = rows.1)\n#paragraph[#first #second.region]",
        "#let(name = upper(rows.1.region), width = inch(rows.0.revenue))\n#paragraph[#name #width]",
        "#let(pair = { rows.0.region, rows.1.region }, entry = { name = rows.1.region })\n#paragraph[#pair.1 #entry.name]",
        "#for row in { rows.1, rows.0 } {#paragraph[#row.region]}",
        "#if(rows.0.region)[#paragraph(alignment = lower(rows.0.region) and \"center\" or \"center\")[centered]]",
        // A numeric segment in content is text, with Lua and without.
        "#paragraph[#rows.0.region]",
    ] {
        let expected = regions(source, false);
        if source.contains(" and ") {
            assert!(expected.is_err(), "{source}");
        } else {
            assert_eq!(regions(source, true), expected, "{source}");
        }
        regions(source, true).unwrap();
    }
    assert_eq!(
        regions(
            "#let(total = rows.0.revenue + rows.1.revenue, label = rows.0.region .. \"/\" .. rows[2].region)\n#paragraph[#total #label]",
            true
        )
        .unwrap(),
        "109 North/South"
    );
    assert_eq!(
        regions(
            "#let(grid = { { 1, 2 }, { 3, 4 } }, keyed = { [\"0\"] = \"zero\", name = \"keyed\" })\n#let(cell = grid.1.0 + 0, field = keyed.0 .. \"\")\n#paragraph[#cell #field]",
            true
        )
        .unwrap(),
        "3 zero"
    );
}

#[test]
fn an_unresolved_or_unrecognized_static_numeric_path_is_explained() {
    for source in [
        "#let(missing = upper(rows.5.region))",
        "#let(missing = rows.0.region.0 .. \"\")",
        "#let(missing = 1 + absent.0)",
    ] {
        let message = regions(source, true).unwrap_err().message;
        assert!(
            message.contains("cannot resolve zero-based path segment"),
            "{source}: {message}"
        );
    }
    for source in [
        "#let(region = (rows).0.region)",
        "#let(region = rows[1].0)",
        "#function first(rows)\n  return rows.0.region\nend\n#first(rows)",
    ] {
        let message = regions(source, true).unwrap_err().message;
        assert!(
            message.contains("a zero-based static path such as rows.0 is not Lua")
                && message.contains("rows[1]"),
            "{source}: {message}"
        );
    }
    for enable_lua in [false, true] {
        let message = regions("#rows[1].region", enable_lua).unwrap_err().message;
        assert!(
            message.contains("rows is a value, which takes no bracketed body"),
            "{message}"
        );
    }
}

const NAMES: &str = "#function names()\n  return { \"A\", \"B\" }\nend\n";

#[test]
fn a_function_call_leaves_the_bindings_of_its_expression_visible() {
    let document = compile(&format!(
        "{NAMES}#let(values = {{ 1, 2 }})\n#let(data = {{ categories = names(), values = values }})\n#chart(kind = \"bar\", title = \"Totals\", data = data)"
    ))
    .unwrap();
    let xml = chart_xml(&document);
    for expected in [
        "<c:v>A</c:v>",
        "<c:v>B</c:v>",
        "<c:v>1</c:v>",
        "<c:v>2</c:v>",
    ] {
        assert!(xml.contains(expected), "{expected}: {xml}");
    }

    for (case, source, expected) in [
        (
            "a binding made after the definition, read after the call",
            "#let(values = { 1, 2 })\n#let(result = #names() + #values)\n#paragraph[#result]",
            "4",
        ),
        (
            "a binding made after the definition, read before the call",
            "#let(values = { 1, 2, 3 })\n#let(result = #values + #names())\n#paragraph[#result]",
            "5",
        ),
        (
            "a binding made by an earlier argument of the same let",
            "#let(count = 7, result = #names() + count)\n#paragraph[#result]",
            "9",
        ),
        (
            "two calls in one expression",
            "#let(count = 7)\n#let(result = #names() + #names() + count)\n#paragraph[#result]",
            "11",
        ),
        (
            "a loop variable",
            "#for item in { 10, 20 } {#let(result = #names() + item)#paragraph[#result]}",
            "12\n22",
        ),
        (
            "a directive argument",
            "#let(count = 2)\n#if(#names() == count)[#paragraph[equal]] else [#paragraph[unequal]]",
            "equal",
        ),
        (
            "a function defined after the function called",
            "#function double(value)\n  return value * 2\nend\n#let(result = #names() + double(3))\n#paragraph[#result]",
            "8",
        ),
        (
            "a call that is the argument of a later function",
            "#function size(values)\n  return #values\nend\n#let(count = 7)\n#let(result = size(names()) + count)\n#paragraph[#result]",
            "9",
        ),
        (
            "a call that itself calls a function",
            "#function size()\n  return #names()\nend\n#let(count = 7)\n#let(result = size() + count)\n#paragraph[#result]",
            "9",
        ),
        (
            "a call that fails within a protected call",
            "#function failure()\n  error(\"failure\")\nend\n#let(count = 7)\n#let(result = tostring(pcall(failure)) .. count)\n#paragraph[#result]",
            "false7",
        ),
    ] {
        assert_eq!(text(&format!("{NAMES}{source}")), expected, "{case}");
    }
}

#[test]
fn a_function_body_keeps_its_captured_bindings_and_its_parameters() {
    assert_eq!(
        text(
            "#let(label = \"first\")\n#function captured(suffix)\n  return label .. suffix\nend\n#let(label = \"second\", later = \"later\")\n#function absent()\n  return tostring(later)\nend\n#let(later = \"changed\")\n#let(result = captured(\"!\") .. \" \" .. label .. \" \" .. absent() .. \" \" .. later)\n#paragraph[#result]"
        ),
        "first! second later changed"
    );
    for (case, source, expected) in [
        (
            "a captured value read before and after a call, beside a parameter",
            "#let(label = \"first\")\n#function inner()\n  return label\nend\n#let(label = \"second\")\n#function outer(parameter)\n  return label .. \"/\" .. inner() .. \"/\" .. label .. \"/\" .. parameter\nend\n#let(result = outer(\"given\") .. \"/\" .. label)\n#paragraph[#result]",
            "second/first/second/given/second",
        ),
        (
            "calls nested three deep",
            "#let(label = \"first\")\n#function innermost()\n  return label\nend\n#let(label = \"second\")\n#function middle()\n  return label .. \"(\" .. innermost() .. \")\" .. label\nend\n#let(label = \"third\")\n#function outermost()\n  return label .. \"(\" .. middle() .. \")\" .. label\nend\n#let(label = \"fourth\")\n#let(result = outermost() .. \" \" .. label)\n#paragraph[#result]",
            "third(second(first)second)third fourth",
        ),
        (
            "a function that calls itself through a retained alias",
            "#let(label = \"first\")\n#function countdown(count)\n  if count == 0 then return label end\n  return label .. count .. \"(\" .. countdown_again(count - 1) .. \")\" .. count .. label\nend\n#let(saved = (function() countdown_again = countdown; return true end)())\n#let(label = \"second\")\n#let(result = countdown(2) .. \" \" .. label)\n#paragraph[#result]",
            "first2(first1(first)1first)2first second",
        ),
        (
            "two functions that call each other through retained aliases",
            "#let(label = \"even\")\n#function even(count)\n  if count == 0 then return label end\n  return label .. \"(\" .. retained_odd(count - 1) .. \")\" .. label\nend\n#let(label = \"odd\")\n#function odd(count)\n  if count == 0 then return label end\n  return label .. \"(\" .. retained_even(count - 1) .. \")\" .. label\nend\n#let(saved = (function() retained_odd = odd; retained_even = even; return true end)())\n#let(label = \"outside\")\n#let(result = even(3) .. \" \" .. label)\n#paragraph[#result]",
            "even(odd(even(odd)even)odd)even outside",
        ),
        (
            "a callee that fails within a protected call",
            "#let(label = \"first\")\n#function failure()\n  error(label)\nend\n#let(label = \"second\")\n#function outer()\n  return label .. \"/\" .. tostring(pcall(failure)) .. \"/\" .. label\nend\n#let(result = outer() .. \"/\" .. label)\n#paragraph[#result]",
            "second/false/second/second",
        ),
        (
            "a callee that assigns a Lua global that is no binding",
            "#let(label = \"first\")\n#function inner()\n  marker = \"assigned\"\n  return label\nend\n#let(label = \"second\")\n#function outer()\n  return inner() .. \"/\" .. marker .. \"/\" .. label\nend\n#let(result = outer() .. \"/\" .. marker .. \"/\" .. label)\n#paragraph[#result]",
            "first/assigned/second/assigned/second",
        ),
    ] {
        assert_eq!(text(source), expected, "{case}");
    }
}
