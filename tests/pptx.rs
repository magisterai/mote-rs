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
fn positioned_slides_match_captured_models_and_complete_editable_packages() {
    compare_captured_packages(
        "presentation-text",
        &[
            "basics",
            "edges",
            "empty",
            "formatting",
            "fitting",
            "font-assets",
            "lua",
            "styles",
            "theme",
        ],
    );
}

#[test]
fn picture_slides_match_captured_models_and_complete_editable_packages() {
    compare_captured_packages(
        "presentation-images",
        &["placement", "transforms", "formats", "missing"],
    );
}

fn compare_captured_packages(directory: &str, cases: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(directory);
    let resolver = FileAssetResolver::new(&root).unwrap();
    for case in cases {
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
            mote::pptx::emit_with_assets(&document, &resolver, Some(path.as_os_str())).unwrap();
        compare_package(&package.bytes, &manifest["parts"], &root, case);
        assert_eq!(document, original);
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
            if name.ends_with(".xml") || name.ends_with(".rels") {
                xml_support::compare_xml(
                    std::str::from_utf8(&actual).unwrap(),
                    std::str::from_utf8(&expected).unwrap(),
                    &context,
                );
            } else {
                assert_eq!(actual, expected, "{context}: embedded bytes");
            }
        }
    }
}

#[test]
fn pending_slide_features_fail_explicitly() {
    let document = mote::compile(
        r##"#slide[#box(position={width=inch(2),height=inch(1)},shadow={color="#000"})[Text]]"##,
    )
    .unwrap();
    assert!(mote::pptx::emit(&document).is_err());
}
#[test]
fn invalid_physical_slide_dimensions_cannot_create_a_package() {
    for source in [
        r##"#presentation(width=pt(0),height=pt(10))"##,
        r##"#presentation(width=pt(1000000000000000000000000000000),height=pt(10))"##,
    ] {
        let document = mote::compile(source).unwrap();
        assert!(mote::pptx::emit(&document).is_err(), "{source}");
    }
}

#[test]
fn authored_template_tokens_remain_literal_in_theme_metadata() {
    let document = mote::compile(
        r##"#presentation(theme={name="{{heading_font}} & <theme>",fonts={heading="{{body_font}}",body="{{color_0}}"}})#slide[]"##,
    )
    .unwrap();
    let bytes = mote::pptx::emit(&document).unwrap();
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut source = String::new();
    archive
        .by_name("ppt/theme/theme1.xml")
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();
    let theme = roxmltree::Document::parse(&source).unwrap();
    assert_eq!(
        theme.root_element().attribute("name"),
        Some("{{heading_font}} & <theme>")
    );
    for (kind, expected) in [("majorFont", "{{body_font}}"), ("minorFont", "{{color_0}}")] {
        let font = theme
            .descendants()
            .find(|node| {
                node.has_tag_name((
                    "http://schemas.openxmlformats.org/drawingml/2006/main",
                    kind,
                ))
            })
            .unwrap();
        let latin = font
            .children()
            .find(|node| node.tag_name().name() == "latin")
            .unwrap();
        assert_eq!(latin.attribute("typeface"), Some(expected));
    }
}

#[test]
fn positioned_tables_match_captured_editable_packages() {
    compare_captured_packages(
        "presentation-tables",
        &["geometry", "merges", "formatting", "flow", "borders"],
    );
}

#[test]
fn positioned_drawings_match_captured_editable_packages() {
    compare_captured_packages(
        "presentation-shapes",
        &[
            "presets",
            "transforms",
            "fitting",
            "connectors",
            "connector-caps",
        ],
    );
}

#[test]
fn positioned_charts_match_captured_packages_and_workbooks() {
    compare_captured_packages(
        "presentation-charts",
        &["bars", "curves", "pie", "theme", "labels"],
    );
}

#[test]
fn slide_backgrounds_match_captured_editable_packages() {
    compare_captured_packages(
        "presentation-backgrounds",
        &["fitting", "aspect", "inheritance", "missing"],
    );
}

#[test]
fn footer_fields_and_speaker_notes_match_captured_editable_packages() {
    compare_captured_packages(
        "presentation-fields",
        &["inheritance", "notes", "geometry", "lua", "mixed"],
    );
}

#[test]
fn automatic_dates_keep_editable_fields_and_paint_the_current_calendar_date() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentation-fields");
    let resolver = FileAssetResolver::new(&root).unwrap();
    let path = root.join("automatic.mote");
    let source = concat!(
        "#font(name='body',latin='Source Sans 3',files={regular='../measurements/fonts/SourceSans3-Regular.ttf'})",
        "#presentation(width=pt(600),height=pt(400),date=true,slide_number=true)",
        "#slide[]#slide(date=false)[]#slide[]"
    );
    let document = mote::compile(source).unwrap();
    let calendar_date = || {
        let today = time::OffsetDateTime::now_local()
            .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
            .date();
        format!("{} {}, {}", today.month(), today.day(), today.year())
    };
    let before = calendar_date();
    let package =
        mote::pptx::emit_with_assets(&document, &resolver, Some(path.as_os_str())).unwrap();
    let plan = mote::render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
    let after = calendar_date();
    let mut archive = zip::ZipArchive::new(Cursor::new(package.bytes)).unwrap();
    let mut identifiers = std::collections::BTreeSet::new();
    for number in 1..=3 {
        let mut source = String::new();
        archive
            .by_name(&format!("ppt/slides/slide{number}.xml"))
            .unwrap()
            .read_to_string(&mut source)
            .unwrap();
        let slide = roxmltree::Document::parse(&source).unwrap();
        let fields: Vec<_> = slide
            .descendants()
            .filter(|node| {
                node.has_tag_name((
                    "http://schemas.openxmlformats.org/drawingml/2006/main",
                    "fld",
                ))
            })
            .collect();
        assert_eq!(fields.len(), if number == 2 { 1 } else { 2 });
        for field in fields {
            assert!(identifiers.insert(field.attribute("id").unwrap().to_owned()));
            let value = field
                .children()
                .find(|node| node.tag_name().name() == "t")
                .unwrap()
                .text()
                .unwrap();
            if field.attribute("type") == Some("datetime4") {
                assert!(value == before || value == after);
            } else {
                assert_eq!(field.attribute("type"), Some("slidenum"));
                assert_eq!(value, number.to_string());
            }
        }
        let visible = plan.output.pages[number - 1]
            .text_lines()
            .flat_map(|line| line.fragments.iter().map(|fragment| fragment.text.as_str()))
            .collect::<String>();
        if number == 2 {
            assert_eq!(visible, "2");
        } else {
            assert!(
                visible.contains(&before) || visible.contains(&after),
                "{visible}"
            );
        }
    }
    assert!(
        mote::render::paint_pdf(&plan.output)
            .unwrap()
            .starts_with(b"%PDF-")
    );
    assert_eq!(
        mote::render::paint_png(&plan.output, &Default::default())
            .unwrap()
            .output
            .len(),
        3
    );
}

#[test]
fn automatic_slides_match_captured_models_and_packages() {
    compare_captured_packages("presentation-automatic", &["standard", "media", "fitting"]);
}

#[test]
fn markdown_decks_match_captured_models_and_packages() {
    compare_captured_packages(
        "presentation-markdown",
        &["headings", "separators", "mixed"],
    );
}

#[test]
fn builtin_layouts_match_captured_models_and_editable_packages() {
    compare_captured_packages("presentation-layouts", &["wide", "standard", "custom"]);
}
