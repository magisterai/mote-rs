#[allow(dead_code)]
#[path = "docx/support.rs"]
mod xml_support;
use mote::{evaluation::EvaluationOptions, source::FileAssetResolver};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Read},
    path::Path,
};

fn normalize_numbers(value: &mut Value) {
    match value {
        Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        Value::Object(fields) => fields.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}

#[test]
fn charts_match_captured_models_and_complete_editable_packages() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/charts");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in [
        "data",
        "defaults",
        "edge-data",
        "stories",
        "wide-data",
        "kinds",
        "labels",
        "legends",
        "lua",
        "overrides",
        "properties",
    ] {
        let path = root.join(format!("{case}.mote"));
        let source = fs::read_to_string(&path).unwrap();
        let folder = root.join("captured").join(case);
        let manifest: Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        for (path, expected) in manifest["assets"].as_object().unwrap() {
            assert_eq!(
                format!("{:x}", Sha256::digest(fs::read(root.join(path)).unwrap())),
                expected.as_str().unwrap()
            );
        }
        let document = mote::compile_with_options(
            &source,
            EvaluationOptions {
                enable_lua: true,
                asset_resolver: Some(&resolver),
                source_name: Some(path.as_os_str()),
                ..Default::default()
            },
        )
        .unwrap();
        let mut actual: Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: Value =
            serde_json::from_slice(&fs::read(folder.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}");
        let original = document.clone();
        let package =
            mote::docx::emit_with_assets(&document, &resolver, Some(path.as_os_str())).unwrap();
        compare_package(&package.bytes, &manifest["parts"], &root, case);
        assert_eq!(document, original);
    }
}

#[test]
fn invalid_chart_signatures_and_formatting_are_rejected_before_output() {
    for source in [
        "#chart()",
        "#chart(properties={data={values={1}}})",
        "#chart(data={1,2})",
        "#chart({values={1}})",
        "#chart(data={values={1}})[body]",
        "#chart(data={values={1}},kind='scatter')",
        "#chart(data={values={1}},title=true)",
        "#chart(data={values={1}},width=-1)",
        "#chart(data={values={1}},legend='r')",
        "#chart(data={values={1}},legend={position='b'})",
        "#chart(data={values={1}},legend={enabled=1})",
        "#chart(data={values={1}},colors={'red'})",
        "#chart(data={values={1}},colors={metadata=true})",
        "#chart(data={values={1}},border={width=-1})",
        "#chart(data={values={1}},plot_area='false')",
        "#chart(data={values={1}},gridlines={major=1})",
        "#chart(data={values={1}},data_labels={position='automatic'})",
        "#chart(data={values={1}},gap_width=1.5)",
        "#chart(data={values={1}},overlap='-2.5')",
        "#chart(data={values={1}},style=17)",
        "#chart(data={values={1}},text=pt(14))",
        "#chart(data={values={1}},text='large')",
        "#chart(data={values={1}},text={size=pt(0)})",
        "#chart(data={values={1}},text={size=14})",
        "#chart(data={values={1}},text={size='large'})",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
}

fn compare_package(bytes: &[u8], manifest: &Value, root: &Path, context: &str) {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let entries = manifest.as_object().unwrap();
    assert_eq!(archive.len(), entries.len(), "{context}: part count");
    let names: std::collections::BTreeSet<_> = archive
        .file_names()
        .map(|name| name.unwrap().into_owned())
        .collect();
    assert_eq!(names.len(), archive.len(), "{context}: duplicate parts");
    assert_eq!(
        names,
        entries.keys().cloned().collect(),
        "{context}: part names"
    );
    for (name, entry) in entries {
        let context = format!("{context}/{name}");
        let mut actual = Vec::new();
        archive
            .by_name(name)
            .unwrap()
            .read_to_end(&mut actual)
            .unwrap();
        if let Some(parts) = entry.get("parts") {
            compare_package(&actual, parts, root, &context);
        } else {
            let expected = fs::read(root.join(entry["path"].as_str().unwrap())).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&expected)),
                entry["sha256"],
                "{context}: capture integrity"
            );
            xml_support::compare_xml(
                std::str::from_utf8(&actual).unwrap(),
                std::str::from_utf8(&expected).unwrap(),
                &context,
            );
        }
    }
}

#[test]
fn non_finite_chart_data_cannot_create_an_editable_package() {
    for number in ["NaN", "Inf", "-Inf", "Infinity", "1e999"] {
        let source = format!("#chart(data={{values={{1,\"{number}\"}}}})");
        let document = mote::compile(&source).unwrap();
        let error = mote::docx::emit(&document).unwrap_err().to_string();
        assert!(error.contains("must be finite"), "{number}: {error}");
    }
}

fn chart_part(package: &[u8], name: &str) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(package)).unwrap();
    let mut content = String::new();
    archive
        .by_name(name)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    content
}

#[test]
fn unstyled_chart_rules_follow_the_office_default_chart_style() {
    let rule = |width: i64, color: &str| {
        format!(
            r#"<a:ln w="{width}" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:srgbClr val="{color}"/></a:solidFill><a:round/></a:ln>"#
        )
    };
    let label_fill =
        |color: &str| format!(r#"<a:solidFill><a:srgbClr val="{color}"/></a:solidFill>"#);
    let chart = "#chart(data={categories={'A','B'},values={1,2}})";
    let document = mote::compile(chart).unwrap();
    let xml = chart_part(
        &mote::docx::emit(&document).unwrap(),
        "word/charts/chart1.xml",
    );
    let gridlines = format!(
        "<c:majorGridlines><c:spPr>{}</c:spPr></c:majorGridlines>",
        rule(9525, "D9D9D9")
    );
    assert!(xml.contains(&gridlines), "{xml}");
    assert!(xml.contains(&format!(
        "<c:spPr><a:noFill/>{}</c:spPr>",
        rule(9525, "D9D9D9")
    )));
    assert!(xml.contains("<c:spPr><a:noFill/><a:ln><a:noFill/></a:ln></c:spPr>"));
    assert_eq!(xml.matches(r#"<c:majorTickMark val="none"/>"#).count(), 2);
    assert_eq!(xml.matches(&label_fill("595959")).count(), 2);

    let styled = "#chart(data={values={1,2}},gridlines={minor=true,color='#FF0000',width=pt(2)})";
    let xml = chart_part(
        &mote::docx::emit(&mote::compile(styled).unwrap()).unwrap(),
        "word/charts/chart1.xml",
    );
    for name in ["majorGridlines", "minorGridlines"] {
        let expected = format!(
            "<c:{name}><c:spPr>{}</c:spPr></c:{name}>",
            rule(25400, "FF0000")
        );
        assert!(xml.contains(&expected), "{name}: {xml}");
    }

    for (text, label) in [("#000000", "595959"), ("#FF0000", "FF5959")] {
        let source = format!(
            "#presentation(theme={{colors={{text='{text}'}}}})\n#slide[#box(position={{x=pt(10),y=pt(10),width=pt(300),height=pt(200)}})[{chart}]]"
        );
        let xml = chart_part(
            &mote::pptx::emit(&mote::compile(&source).unwrap()).unwrap(),
            "ppt/charts/chart1.xml",
        );
        assert!(xml.contains(&gridlines), "{xml}");
        assert_eq!(xml.matches(&label_fill(label)).count(), 2, "{text}");
    }
}

#[test]
fn chart_text_size_is_a_positive_length_and_the_only_member() {
    let document = mote::compile("#chart(data={values={1}},text={size='14pt'})").unwrap();
    let mote::model::Content::Element(chart) = &document.content[0] else {
        panic!("the document begins with the chart");
    };
    let mote::model::Value::Object(text) = &chart.attributes["text"] else {
        panic!("the text argument remains a property table");
    };
    assert_eq!(text.len(), 1);
    assert_eq!(text["size"].to_text(), "14pt");

    // An unknown member is retained, and the output reports it.
    let unknown_member = mote::compile(
        "#slide[#box(position={width=pt(300),height=pt(200)})[\
         #chart(data={values={1}},text={size=pt(14),bold=true})]]",
    )
    .unwrap();
    let warnings = mote::pptx::emit_with_warnings(&unknown_member)
        .unwrap()
        .warnings;
    assert!(
        warnings
            .iter()
            .any(|warning| warning.contains("chart.text.bold is retained")),
        "{warnings:?}"
    );
    let wrong_size = mote::compile("#chart(data={values={1}},text={size=pt(0)})")
        .unwrap_err()
        .to_string();
    assert!(
        wrong_size.contains("text.size must be a positive length"),
        "{wrong_size}"
    );
    assert!(mote::compile("#chart(data={values={1}},text=pt(14))").is_err());
}

#[test]
fn chart_text_size_sets_the_label_size_of_editable_charts() {
    const DATA: &str =
        "data={categories={'A','B'},series={{name='One',values={1,2}},{name='Two',values={2,3}}}}";
    let chart = |arguments: &str| {
        format!(
            "#chart({DATA},title='Totals',axis={{value={{title='Units'}}}},data_labels=true{arguments})"
        )
    };
    let slide = |chart: &str| {
        format!(
            "#presentation(size='wide')\n#slide[#box(position={{x=pt(10),y=pt(10),width=pt(600),height=pt(400)}})[{chart}]]"
        )
    };
    let parts = |arguments: &str| {
        let source = chart(arguments);
        let word = mote::docx::emit_with_assets(
            &mote::compile(&source).unwrap(),
            &FileAssetResolver::new(".").unwrap(),
            None,
        )
        .unwrap();
        let slides = mote::pptx::emit_with_assets(
            &mote::compile(&slide(&source)).unwrap(),
            &FileAssetResolver::new(".").unwrap(),
            None,
        )
        .unwrap();
        assert!(word.warnings.is_empty(), "{:?}", word.warnings);
        assert!(slides.warnings.is_empty(), "{:?}", slides.warnings);
        [
            chart_part(&word.bytes, "word/charts/chart1.xml"),
            chart_part(&slides.bytes, "ppt/charts/chart1.xml"),
        ]
    };
    // The defaults are those of the chart style Office gives a new chart: labels of
    // 9 points from the chart space, a title of 14 and an axis title of 10.
    for arguments in ["", ",text={size=pt(9)}"] {
        for xml in parts(arguments) {
            assert_eq!(xml.matches(" sz=").count(), 3, "{xml}");
            assert!(xml.contains(r#"<a:defRPr sz="900">"#), "{xml}");
            assert_eq!(
                xml.matches(r#"<a:rPr lang="en-US" sz="1400"><a:solidFill>"#)
                    .count(),
                1,
                "{xml}"
            );
            assert_eq!(
                xml.matches(r#"<a:rPr lang="en-US" sz="1000"><a:solidFill>"#)
                    .count(),
                1,
                "{xml}"
            );
        }
    }
    for xml in parts(",text={size=pt(14)}") {
        // The two axes carry the axis label colour; the legend and the data
        // labels state the size alone.
        assert_eq!(
            xml.matches(r#"<a:defRPr sz="1400"><a:solidFill>"#).count(),
            2
        );
        assert!(
            xml.contains(r#"<c:overlay val="0"/><c:txPr><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr sz="1400"/></a:pPr><a:endParaRPr lang="en-US"/></a:p></c:txPr></c:legend>"#),
            "{xml}"
        );
        assert!(
            xml.contains(r#"<c:dLbls><c:txPr><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr sz="1400"/></a:pPr>"#),
            "{xml}"
        );
        assert_eq!(xml.matches(r#"<a:defRPr sz="1400""#).count(), 4, "{xml}");
        // The chart space default and the title runs are as they are without the argument.
        assert!(xml.contains(r#"<a:defRPr sz="900">"#), "{xml}");
        assert_eq!(
            xml.matches(r#"<a:rPr lang="en-US" sz="1400"><a:solidFill>"#)
                .count(),
            1
        );
        assert_eq!(
            xml.matches(r#"<a:rPr lang="en-US" sz="1000"><a:solidFill>"#)
                .count(),
            1
        );
    }
    for xml in parts(",text={size=pt(14)},title={text='Totals',size=pt(20)}") {
        assert_eq!(xml.matches(r#"<a:rPr lang="en-US" sz="2000">"#).count(), 1);
        assert_eq!(xml.matches(r#" sz="1400""#).count(), 4, "{xml}");
    }
    let axis_titles = ",axis={category={title={text='Regions',size=pt(12),bold=true,italic=false,color='#334155',font='Georgia'}},value={title={text='Units',size='11pt'}}}";
    for xml in parts(axis_titles) {
        assert!(
            xml.contains(r#"<a:rPr lang="en-US" sz="1200" b="1" i="0"><a:solidFill><a:srgbClr val="334155"/></a:solidFill><a:latin typeface="Georgia"/></a:rPr><a:t>Regions</a:t>"#),
            "{xml}"
        );
        assert!(
            xml.contains(r#"<a:rPr lang="en-US" sz="1100"><a:solidFill>"#),
            "{xml}"
        );
        assert!(xml.contains("<a:t>Units</a:t>"), "{xml}");
    }
    for title in [
        "{text='Units',size=-1}",
        "{text='Units',bold=1}",
        "{text='Units',color='red'}",
        "{text=true}",
    ] {
        for axis in ["category", "value"] {
            let source = format!("#chart({DATA},axis={{{axis}={{title={title}}}}})");
            assert!(mote::compile(&source).is_err(), "{source}");
        }
        let source = format!("#chart({DATA},title={title})");
        assert!(mote::compile(&source).is_err(), "{source}");
    }
}

#[test]
fn a_stated_value_range_is_normalized_and_written_to_both_editable_packages() {
    let chart = |kind: &str, range: &str| {
        format!(
            "#chart(kind='{kind}',axis={{value={{title='Units',{range}}}}},data={{categories={{'A','B'}},values={{50,120}}}})"
        )
    };
    let document = mote::compile(&chart("column", "minimum='0',maximum=200")).unwrap();
    let model: Value = serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
    let value_axis = &find_member(&model, "axis").unwrap()["value"];
    assert_eq!(value_axis["title"], "Units");
    assert_eq!(value_axis["minimum"].as_f64(), Some(0.0));
    assert_eq!(value_axis["maximum"].as_f64(), Some(200.0));

    for range in [
        "minimum='low'",
        "maximum=true",
        "maximum='Inf'",
        "minimum=5,maximum=5",
        "minimum=6,maximum=5",
    ] {
        assert!(mote::compile(&chart("column", range)).is_err(), "{range}");
    }

    for (kind, range, bounds) in [
        (
            "column",
            "minimum=0,maximum=200",
            r#"<c:max val="200"/><c:min val="0"/>"#,
        ),
        ("bar", "minimum=-2.5", r#"<c:min val="-2.5"/>"#),
        ("line", "maximum=1000", r#"<c:max val="1000"/>"#),
        ("stacked_area", "name='Units'", ""),
        ("percent_stacked_column", "minimum=0,maximum=200", ""),
        ("pie", "maximum=5", ""),
    ] {
        let source = chart(kind, range);
        let slide = format!(
            "#slide[#box(position={{x=pt(10),y=pt(10),width=pt(300),height=pt(200)}})[{source}]]"
        );
        for xml in [
            chart_part(
                &mote::docx::emit(&mote::compile(&source).unwrap()).unwrap(),
                "word/charts/chart1.xml",
            ),
            chart_part(
                &mote::pptx::emit(&mote::compile(&slide).unwrap()).unwrap(),
                "ppt/charts/chart1.xml",
            ),
        ] {
            if kind != "pie" {
                let value_axis = &xml[xml.find("<c:valAx>").unwrap()..];
                let scaling =
                    format!(r#"<c:scaling><c:orientation val="minMax"/>{bounds}</c:scaling>"#);
                assert!(value_axis.contains(&scaling), "{kind}: {xml}");
            }
            for bound in ["<c:min ", "<c:max "] {
                assert_eq!(
                    xml.matches(bound).count(),
                    bounds.matches(bound).count(),
                    "{kind}: {xml}"
                );
            }
        }
    }
}

fn find_member<'value>(value: &'value Value, name: &str) -> Option<&'value Value> {
    match value {
        Value::Object(fields) => fields
            .get(name)
            .or_else(|| fields.values().find_map(|nested| find_member(nested, name))),
        Value::Array(values) => values.iter().find_map(|nested| find_member(nested, name)),
        _ => None,
    }
}

#[test]
fn a_stated_value_range_warns_only_where_the_chart_kind_has_no_free_range() {
    let resolver = FileAssetResolver::new(".").unwrap();
    for (kind, inapplicable) in [
        ("column", false),
        ("bar", false),
        ("line", false),
        ("stacked_area", false),
        ("percent_stacked_bar", true),
        ("doughnut", true),
    ] {
        let chart = format!(
            "#chart(kind='{kind}',axis={{value={{minimum=0,maximum=200}}}},data={{categories={{'A','B'}},values={{50,120}}}})"
        );
        let slide = format!(
            "#slide[#box(position={{x=pt(10),y=pt(10),width=pt(300),height=pt(200)}})[{chart}]]"
        );
        let document = mote::compile(&chart).unwrap();
        let deck = mote::compile(&slide).unwrap();
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
            let range: Vec<_> = warnings
                .iter()
                .filter(|warning| warning.contains("axis.value"))
                .collect();
            assert!(
                range
                    .iter()
                    .all(|warning| warning.contains("has no effect on")),
                "{kind} {output}: {warnings:?}"
            );
            // A DOCX package reports no limit of a chart argument.
            let expected = if inapplicable && output != "DOCX" {
                2
            } else {
                0
            };
            assert_eq!(range.len(), expected, "{kind} {output}: {warnings:?}");
            if !inapplicable {
                assert!(
                    !warnings.iter().any(|warning| warning.contains("chart")),
                    "{kind} {output}: {warnings:?}"
                );
            }
        }
    }
}

#[test]
fn a_stated_value_step_is_normalized_and_written_as_the_major_unit() {
    let chart = |kind: &str, value_axis: &str| {
        format!(
            "#chart(kind='{kind}',axis={{value={{{value_axis}}}}},data={{categories={{'A','B'}},values={{50,120}}}})"
        )
    };
    let document = mote::compile(&chart("column", "minimum=0,maximum=150,step='50'")).unwrap();
    let model: Value = serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
    let value_axis = &find_member(&model, "axis").unwrap()["value"];
    assert_eq!(value_axis["step"].as_f64(), Some(50.0));

    for value_axis in [
        "step=0",
        "step=-10",
        "step='wide'",
        "step=true",
        "step='Inf'",
        "minimum=0,maximum=150,step=1",
    ] {
        assert!(
            mote::compile(&chart("column", value_axis)).is_err(),
            "{value_axis}"
        );
    }
    assert!(mote::compile(&chart("column", "minimum=0,maximum=150,step=1.5")).is_ok());

    for (kind, value_axis, major_unit) in [
        (
            "column",
            "minimum=0,maximum=150,step=50",
            r#"<c:majorUnit val="50"/>"#,
        ),
        ("bar", "step=2.5", r#"<c:majorUnit val="2.5"/>"#),
        ("line", "maximum=150", ""),
        ("percent_stacked_column", "step=50", ""),
        ("pie", "step=50", ""),
    ] {
        let source = chart(kind, value_axis);
        let slide = format!(
            "#slide[#box(position={{x=pt(10),y=pt(10),width=pt(300),height=pt(200)}})[{source}]]"
        );
        let deck = mote::compile(&slide).unwrap();
        for xml in [
            chart_part(
                &mote::docx::emit(&mote::compile(&source).unwrap()).unwrap(),
                "word/charts/chart1.xml",
            ),
            chart_part(&mote::pptx::emit(&deck).unwrap(), "ppt/charts/chart1.xml"),
        ] {
            assert_eq!(
                xml.matches("<c:majorUnit ").count(),
                usize::from(!major_unit.is_empty()),
                "{kind}: {xml}"
            );
            if !major_unit.is_empty() {
                let expected = format!(r#"<c:crossBetween val="between"/>{major_unit}</c:valAx>"#);
                assert!(xml.contains(&expected), "{kind}: {xml}");
            }
        }
        let warnings = mote::pptx::emit_with_warnings(&deck).unwrap().warnings;
        let inapplicable = matches!(kind, "percent_stacked_column" | "pie");
        assert_eq!(
            warnings
                .iter()
                .filter(|warning| warning.contains("axis.value.step has no effect on"))
                .count(),
            usize::from(inapplicable),
            "{kind}: {warnings:?}"
        );
        assert!(
            inapplicable || !warnings.iter().any(|warning| warning.contains("chart")),
            "{kind}: {warnings:?}"
        );
    }
}
