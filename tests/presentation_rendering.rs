#[allow(dead_code)]
#[path = "support/native_capture.rs"]
mod capture;
#[path = "support/presentation_capture.rs"]
mod presentation_capture;
use mote::{render, source::FileAssetResolver};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

#[test]
fn text_box_shadows_preserve_native_layout_and_report_the_output_limit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentation-text");
    let source = fs::read_to_string(root.join("shadows.mote")).unwrap();
    let document = mote::compile_with_options(
        &source,
        mote::evaluation::EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
    .unwrap();
    let resolver = FileAssetResolver::unconfined(&root).unwrap();
    let shadowed = render::plan(&document, &resolver, None).unwrap();
    assert!(
        shadowed
            .warnings
            .iter()
            .any(|warning| warning.contains("omit text-box shadows"))
    );
    assert!(
        shadowed
            .warnings
            .iter()
            .any(|warning| warning.contains("shadow.enabled"))
    );

    let mut without_shadows = document.clone();
    for content in &mut without_shadows.content {
        if let mote::model::Content::Element(slide) = content {
            for content in &mut slide.children {
                if let mote::model::Content::Element(element) = content {
                    element.attributes.remove("shadow");
                }
            }
        }
    }
    let plain = render::plan(&without_shadows, &resolver, None).unwrap();
    assert_eq!(
        presentation_capture::projection(&shadowed.output),
        presentation_capture::projection(&plain.output)
    );
    assert!(
        !plain
            .warnings
            .iter()
            .any(|warning| warning.contains("shadow"))
    );
}

#[test]
fn invisible_text_box_shadows_do_not_report_a_native_omission() {
    let document = mote::compile(
        "#slide[#box(position={width=pt(200),height=pt(100)},shadow={opacity=0})[Text]]",
    )
    .unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let rendered =
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    assert!(
        !rendered
            .warnings
            .iter()
            .any(|warning| warning.contains("shadow"))
    );
}

#[test]
fn native_slide_geometry_and_independent_rasters_match_captured_output() {
    compare_captured_slides(
        "presentation-text",
        &[
            ("basics", 48, 6.0, 0),
            ("edges", 48, 9.0, 0),
            ("fitting", 64, 2.0, 0),
            ("font-assets", 64, 6.0, 0),
            ("formatting", 48, 0.2, 0),
            ("lua", 40, 5.0, 0),
            ("shadows", 64, 1.5, 0),
            ("styles", 48, 6.0, 0),
            ("theme", 48, 0.1, 0),
        ],
    );
}

#[test]
fn picture_geometry_and_native_rasters_match_captured_output() {
    compare_captured_slides(
        "presentation-images",
        &[
            ("placement", 0, 0.0, 0),
            ("formats", 3, 0.03, 0),
            ("transforms", 64, 0.06, 0),
            ("missing", 0, 0.0, 0),
        ],
    );
}
fn compare_captured_slides(directory: &str, cases: &[(&str, u8, f64, usize)]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(directory);
    let resolver = FileAssetResolver::unconfined(&root).unwrap();
    for &(case, maximum_limit, mean_limit, edge_limit) in cases {
        let path = root.join(format!("{case}.mote"));
        let source = fs::read_to_string(&path).unwrap();
        let folder = root.join("captured").join(case);
        let read =
            |name| serde_json::from_slice::<Value>(&fs::read(folder.join(name)).unwrap()).unwrap();
        let manifest = read("capture.json");
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            manifest["source_sha256"]
        );
        let document = mote::compile_with_options(
            &source,
            mote::evaluation::EvaluationOptions {
                enable_lua: true,
                source_name: Some(path.as_os_str()),
                asset_resolver: Some(&resolver),
                ..Default::default()
            },
        )
        .unwrap();
        let original = document.clone();
        let plan = render::plan_with_layout(
            &document,
            &resolver,
            Some(path.as_os_str()),
            render::LayoutMode::Presentation,
        )
        .unwrap();
        assert_eq!(document, original);
        let geometry = presentation_capture::projection(&plan.output);
        if capture::refreshing() {
            if capture::mismatch(&geometry, &read("geometry.json"), case).is_some() {
                capture::replace_json_recorded(&folder, "capture.json", "geometry.json", &geometry);
            }
        } else {
            capture::compare(&geometry, &read("geometry.json"), case);
        }
        let manifest = read("capture.json");
        assert_eq!(
            format!(
                "{:x}",
                Sha256::digest(fs::read(folder.join("geometry.json")).unwrap())
            ),
            manifest["geometry_sha256"]
        );
        assert!(
            render::paint_pdf(&plan.output)
                .unwrap()
                .starts_with(b"%PDF-")
        );
        let pages = render::paint_png(&plan.output, &Default::default())
            .unwrap()
            .output;
        assert_eq!(
            pages.len(),
            manifest["native_png_files"].as_object().unwrap().len()
        );
        for page in pages {
            let name = format!("reference-{}.png", page.number);
            let expected = fs::read(folder.join(&name)).unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(&expected)),
                manifest["native_png_files"][&name]
            );
            let (width, height, expected) = capture::pixels(&expected);
            let (actual_width, actual_height, actual) = capture::pixels(&page.bytes);
            if capture::refreshing() && (width, height) != (actual_width, actual_height) {
                capture::replace_recorded(&folder, "capture.json", &name, &page.bytes);
                continue;
            }
            assert_eq!((width, height), (actual_width, actual_height));
            let mut maximum = 0u8;
            let mut sum = 0u64;
            let mut count = 0u64;
            let mut edge_pixels = 0usize;
            for (actual, expected) in actual
                .as_chunks::<4>()
                .0
                .iter()
                .zip(expected.as_chunks::<4>().0)
            {
                if actual[..3] == [255; 3] && expected[..3] == [255; 3] {
                    continue;
                }
                let differences: Vec<_> = (0..3)
                    .map(|channel| actual[channel].abs_diff(expected[channel]))
                    .collect();
                edge_pixels += usize::from(differences.iter().any(|value| *value > 64));
                for difference in differences {
                    maximum = maximum.max(difference);
                    sum += u64::from(difference);
                    count += 1;
                }
            }
            let mean = sum as f64 / count.max(1) as f64;
            if capture::refreshing() {
                if maximum > maximum_limit || mean > mean_limit || edge_pixels > edge_limit {
                    capture::replace_recorded(&folder, "capture.json", &name, &page.bytes);
                }
                continue;
            }
            assert!(maximum <= maximum_limit, "{case}: maximum {maximum}");
            assert!(mean <= mean_limit, "{case}: mean {mean}");
            assert!(
                edge_pixels <= edge_limit,
                "{case}: {edge_pixels} edge pixels"
            );
        }
    }
}

#[test]
fn native_layout_selection_preserves_document_mode_and_explicit_slide_boundaries() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let report = mote::compile("#heading(level=1)[Report]\n\nParagraph.").unwrap();
    let document = render::plan(&report, &resolver, None).unwrap();
    assert_eq!(
        (
            document.output.geometry.width,
            document.output.geometry.height
        ),
        (612.0, 792.0)
    );
    let forced =
        render::plan_with_layout(&report, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    assert_eq!(forced.output.pages.len(), 1);
    assert_eq!(forced.output.geometry.width, 960.0);
    let slides = mote::compile("#presentation(size='standard')#slide[]#slide[]").unwrap();
    let deck = render::plan(&slides, &resolver, None).unwrap();
    assert_eq!(deck.output.pages.len(), 2);
    assert_eq!(
        (deck.output.geometry.width, deck.output.geometry.height),
        (720.0, 540.0)
    );
    assert!(
        render::plan_with_layout(&slides, &resolver, None, render::LayoutMode::Document).is_err()
    );
    assert!(
        render::plan_with_layout(
            &mote::compile("#presentation()").unwrap(),
            &resolver,
            None,
            render::LayoutMode::Presentation
        )
        .is_ok()
    );
}

#[test]
fn presentation_output_reports_unsupported_retained_properties_and_structures() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let document = mote::compile("#slide[#box(position={width=inch(3),height=inch(2),extra=1},rotation=30,text={language='fr'})[#paragraph(keep_with_next=true)[Text]]]").unwrap();
    for warnings in [
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap()
            .warnings,
        mote::pptx::emit_with_warnings(&document).unwrap().warnings,
    ] {
        for field in [
            "position.extra",
            "rotation",
            "text.language",
            "keep_with_next",
        ] {
            assert!(
                warnings.iter().any(|warning| warning.contains(field)),
                "{field}: {warnings:?}"
            );
        }
    }
}

#[test]
fn markdown_heading_syntax_on_a_slide_is_not_an_unsupported_property() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let markdown =
        mote::compile("#slide[\n# Title\n\nUnderlined\n==========\n\nBody text.\n]").unwrap();
    let custom = mote::compile("#slide[#heading(level=1, marker=\"custom\")[Title]]").unwrap();
    for (document, warned) in [(markdown, false), (custom, true)] {
        for warnings in [
            render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
                .unwrap()
                .warnings,
            mote::pptx::emit_with_warnings(&document).unwrap().warnings,
        ] {
            assert_eq!(
                warnings.iter().any(|warning| warning.contains("marker")),
                warned,
                "{warnings:?}"
            );
        }
    }
}

#[test]
fn slide_layout_does_not_load_document_header_assets() {
    let document =
        mote::compile("#header[#image(src='unused.png')]#presentation(size='wide')#slide[]")
            .unwrap();
    let resolver =
        |_: mote::source::AssetRequest<'_>| -> std::io::Result<mote::source::ResolvedAsset> {
            panic!("presentation requested an unused document asset");
        };
    for mode in [
        render::LayoutMode::Automatic,
        render::LayoutMode::Presentation,
    ] {
        render::plan_with_layout_and_fonts(&document, &resolver, None, mode, None).unwrap();
    }
    mote::pptx::emit_with_fonts(&document, &resolver, None, None).unwrap();
}

#[test]
fn slide_pictures_cache_source_relative_assets_and_freeze_them_before_painting() {
    use mote::{
        layout::PageOperation,
        source::{AssetRequest, ResolvedAsset},
    };
    use std::{cell::Cell, ffi::OsStr, sync::Arc};
    let calls = Cell::new(0);
    let available = Cell::new(true);
    let resolver = |request: AssetRequest<'_>| {
        assert!(available.get());
        calls.set(calls.get() + 1);
        assert_eq!(request.source_name, Some(OsStr::new("memory:slides")));
        assert_eq!(request.reference, OsStr::new("picture"));
        Ok(ResolvedAsset {
            name: "memory:picture".into(),
            bytes: include_bytes!("fixtures/images/assets/pattern.png").to_vec(),
        })
    };
    let document = mote::compile("#presentation(width=pt(200),height=pt(200))#slide[        #box(position={x=pt(10),y=pt(10),width=pt(60),height=pt(40)})[#image('picture',width=pt(400),height=pt(300),alignment='right',spacing={before=pt(50)})]        #box(position={x=pt(100),y=pt(10),width=pt(60),height=pt(40)})[#image('picture',rotation=30)]]").unwrap();
    let source_name = Some(OsStr::new("memory:slides"));
    mote::pptx::emit_with_fonts(&document, &resolver, source_name, None).unwrap();
    assert_eq!(calls.get(), 1);
    let planned = render::plan_with_fonts(&document, &resolver, source_name, None).unwrap();
    assert_eq!(calls.get(), 2);
    let pictures: Vec<_> = planned.output.pages[0]
        .operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::Image(image) => Some(image),
            _ => None,
        })
        .collect();
    assert_eq!(pictures.len(), 2);
    assert_eq!(pictures[0].bounds, [10.0, 10.0, 60.0, 40.0]);
    assert!(Arc::ptr_eq(&pictures[0].image, &pictures[1].image));
    available.set(false);
    render::paint_pdf(&planned.output).unwrap();
    for dots_per_inch in [72, 144] {
        render::paint_png(
            &planned.output,
            &render::PngOptions {
                dots_per_inch,
                ..Default::default()
            },
        )
        .unwrap();
    }
    assert_eq!(calls.get(), 2);
}

#[test]
fn missing_slide_pictures_have_truthful_diagnostics_and_rotated_pdf_links_follow_pictures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentation-images");
    let resolver = FileAssetResolver::unconfined(&root).unwrap();
    for case in ["missing", "transforms"] {
        let path = root.join(format!("{case}.mote"));
        let document = mote::compile(&fs::read_to_string(&path).unwrap()).unwrap();
        let plan = render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
        if case == "missing" {
            assert_eq!(
                plan.warnings
                    .iter()
                    .filter(|warning| warning.contains("omitted picture"))
                    .count(),
                2
            );
            assert!(
                !plan
                    .warnings
                    .iter()
                    .any(|warning| warning.contains("placeholder"))
            );
        } else {
            assert!(plan.warnings.is_empty(), "{:?}", plan.warnings);
            let bytes = render::paint_pdf(&plan.output).unwrap();
            let serialized = String::from_utf8_lossy(&bytes);
            assert!(serialized.contains("/Rect [8.04 165.36 151.96 294.64]"));
            assert!(serialized.contains("/URI (https://example.com/image?a=1&b=2)"));
        }
    }
}

#[test]
fn table_geometry_and_native_rasters_match_captured_output() {
    compare_captured_slides(
        "presentation-tables",
        &[
            ("flow", 40, 0.28, 0),
            ("geometry", 48, 0.16, 0),
            ("merges", 40, 0.12, 0),
            ("formatting", 48, 0.07, 0),
            ("borders", 48, 0.15, 0),
        ],
    );
}

#[test]
fn drawing_geometry_and_native_rasters_match_captured_output() {
    compare_captured_slides(
        "presentation-shapes",
        &[
            ("presets", 96, 0.3, 140),
            ("transforms", 88, 0.16, 4),
            ("fitting", 48, 0.35, 0),
            ("connectors", 64, 1.0, 0),
            ("connector-caps", 64, 1.0, 0),
        ],
    );
}

#[test]
fn explicit_flat_connector_caps_can_be_painted_natively() {
    let document =
        mote::compile("#slide[#line(from={x=pt(10),y=pt(10)},to={x=pt(100),y=pt(40)},cap='flat')]")
            .unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let plan =
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    assert!(!plan.output.pages[0].operations.is_empty());
    assert!(
        render::paint_pdf(&plan.output)
            .unwrap()
            .starts_with(b"%PDF-")
    );
    assert_eq!(
        render::paint_png(&plan.output, &Default::default())
            .unwrap()
            .output
            .len(),
        1
    );
}

#[test]
fn slide_chart_geometry_and_native_rasters_match_captured_output() {
    compare_captured_slides(
        "presentation-charts",
        &[
            ("bars", 48, 0.1, 0),
            ("curves", 48, 0.12, 0),
            ("pie", 40, 0.05, 0),
            ("theme", 48, 0.18, 0),
            ("labels", 40, 0.06, 0),
        ],
    );
}

#[test]
fn background_geometry_and_native_rasters_match_captured_output() {
    compare_captured_slides(
        "presentation-backgrounds",
        &[
            ("fitting", 32, 0.04, 0),
            ("aspect", 32, 0.05, 0),
            ("inheritance", 40, 0.31, 0),
            ("missing", 40, 0.07, 0),
        ],
    );
}

#[test]
fn background_assets_share_the_picture_cache_and_overrides_avoid_unused_sources() {
    use mote::{
        layout::PageOperation,
        source::{AssetRequest, ResolvedAsset},
    };
    use std::{cell::Cell, ffi::OsStr, sync::Arc};
    let calls = Cell::new(0);
    let available = Cell::new(true);
    let resolver = |request: AssetRequest<'_>| {
        assert!(available.get());
        calls.set(calls.get() + 1);
        assert_eq!(request.source_name, Some(OsStr::new("memory:slides")));
        assert_eq!(request.reference, OsStr::new("picture"));
        Ok(ResolvedAsset {
            name: "memory:picture".into(),
            bytes: include_bytes!("fixtures/images/assets/pattern.png").to_vec(),
        })
    };
    let document = mote::compile("#presentation(width=pt(200),height=pt(200),background={src='unused'})#slide(background={src='picture',fit='contain'})[#box(position={x=pt(10),y=pt(10),width=pt(60),height=pt(40)})[#image('picture')]]#slide(background='#ffffff')[]").unwrap();
    let source_name = Some(OsStr::new("memory:slides"));
    mote::pptx::emit_with_fonts(&document, &resolver, source_name, None).unwrap();
    assert_eq!(calls.get(), 1);
    let plan = render::plan_with_fonts(&document, &resolver, source_name, None).unwrap();
    assert_eq!(calls.get(), 2);
    let pictures: Vec<_> = plan.output.pages[0]
        .operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::Image(image) => Some(image),
            _ => None,
        })
        .collect();
    assert_eq!(pictures.len(), 2);
    assert!(Arc::ptr_eq(&pictures[0].image, &pictures[1].image));
    assert!(
        !plan.output.pages[1]
            .operations
            .iter()
            .any(|operation| matches!(operation, PageOperation::Image(_)))
    );
    available.set(false);
    render::paint_pdf(&plan.output).unwrap();
    render::paint_png(&plan.output, &Default::default()).unwrap();
    assert_eq!(calls.get(), 2);
}

#[test]
fn footer_fields_match_captured_native_geometry_and_rasters() {
    compare_captured_slides(
        "presentation-fields",
        &[
            ("inheritance", 40, 6.9, 0),
            ("notes", 40, 6.6, 0),
            ("geometry", 40, 6.3, 0),
            ("lua", 48, 7.6, 0),
            ("mixed", 44, 0.16, 0),
        ],
    );
}

#[test]
fn speaker_notes_do_not_change_native_slide_content() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentation-fields");
    let resolver = FileAssetResolver::unconfined(&root).unwrap();
    let path = root.join("notes.mote");
    let mut document = mote::compile(&fs::read_to_string(&path).unwrap()).unwrap();
    let with_notes = render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
    for content in &mut document.content {
        if let mote::model::Content::Element(element) = content {
            element.attributes.remove("notes");
        }
    }
    let without_notes = render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
    assert_eq!(
        presentation_capture::projection(&with_notes.output),
        presentation_capture::projection(&without_notes.output)
    );
}

#[test]
fn automatic_slide_geometry_and_rasters_match_captures() {
    compare_captured_slides(
        "presentation-automatic",
        &[
            ("standard", 48, 6.2, 0),
            ("media", 48, 0.16, 0),
            ("fitting", 64, 11.0, 0),
        ],
    );
}

#[test]
fn markdown_decks_match_captured_geometry_and_rasters() {
    compare_captured_slides(
        "presentation-markdown",
        &[
            ("headings", 64, 9.0, 0),
            ("separators", 64, 9.0, 0),
            ("mixed", 64, 9.0, 0),
        ],
    );
}

#[test]
fn markdown_boundaries_do_not_change_document_mode_or_split_nested_headings() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let source = "# First\n\nBody.\n\n# Second\n\nBody.";
    let document = mote::compile(source).unwrap();
    assert_eq!(
        render::plan(&document, &resolver, None)
            .unwrap()
            .output
            .pages
            .len(),
        1
    );
    assert_eq!(
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap()
            .output
            .pages
            .len(),
        2
    );
    for source in [
        "---\n\n---\n\n# Title\n\n---",
        "#slide[#heading(level=1)[One]#heading(level=1)[Two]]",
    ] {
        let document = mote::compile(source).unwrap();
        assert_eq!(
            render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
                .unwrap()
                .output
                .pages
                .len(),
            1
        );
    }
    let document = mote::compile("# First\n\n---\n\n# Second").unwrap();
    let planned =
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Document)
            .unwrap()
            .output;
    assert_eq!(planned.pages.len(), 1);
    assert!(
        planned.pages[0]
            .operations
            .iter()
            .any(|operation| matches!(operation, mote::layout::PageOperation::Rule(rule) if rule.role == mote::layout::RuleRole::Divider))
    );
}

#[test]
fn builtin_layouts_match_captured_native_geometry_and_rasters() {
    compare_captured_slides(
        "presentation-layouts",
        &[
            ("wide", 50, 7.5, 0),
            ("standard", 50, 7.5, 0),
            ("custom", 48, 8.3, 0),
        ],
    );
}

#[test]
fn custom_layouts_match_captured_native_geometry_and_rasters() {
    compare_captured_slides(
        "presentation-custom",
        &[
            ("inheritance", 48, 0.1, 0),
            ("backgrounds", 48, 0.1, 0),
            ("replacement", 48, 0.1, 0),
            ("lua-json", 40, 0.04, 0),
        ],
    );
}

fn slide_xml(bytes: Vec<u8>) -> String {
    use std::io::{Cursor, Read};
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("ppt/slides/slide1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}
fn object_frames(xml: &str, object_name: &str) -> Vec<[f64; 4]> {
    let tree = roxmltree::Document::parse(xml).unwrap();
    tree.descendants()
        .filter(|node| node.tag_name().name() == object_name)
        .map(|node| {
            let transform = node
                .descendants()
                .find(|child| child.tag_name().name() == "xfrm")
                .unwrap();
            let offset = transform
                .children()
                .find(|child| child.tag_name().name() == "off")
                .unwrap();
            let extent = transform
                .children()
                .find(|child| child.tag_name().name() == "ext")
                .unwrap();
            [
                offset.attribute("x").unwrap(),
                offset.attribute("y").unwrap(),
                extent.attribute("cx").unwrap(),
                extent.attribute("cy").unwrap(),
            ]
            .map(|value| value.parse::<f64>().unwrap() / 12700.0)
        })
        .collect()
}

#[test]
fn automatic_charts_use_a_readable_grid_or_report_insufficient_space() {
    let resolver = FileAssetResolver::new(".").unwrap();
    let chart = "#chart(kind='column',title='Values',legend=false,data={categories={'A','B'},values={1,2}})";
    for (width, height, count, columns) in [(640, 480, 4, 2), (720, 720, 2, 1)] {
        let document = mote::compile(&format!(
            "#presentation(width=pt({width}),height=pt({height}))#slide[{}]",
            chart.repeat(count)
        ))
        .unwrap();
        let plan = render::plan(&document, &resolver, None).unwrap();
        assert_eq!(plan.output.pages.len(), 1);
        let xml = slide_xml(mote::pptx::emit(&document).unwrap());
        let frames = object_frames(&xml, "graphicFrame");
        assert_eq!(frames.len(), count);
        for (index, frame) in frames.iter().enumerate() {
            assert!(frame[2] >= 120.0 && frame[3] >= 120.0);
            if index >= columns {
                assert!(frame[1] > frames[index - columns][1]);
            }
            if index % columns > 0 {
                assert!(frame[0] > frames[index - 1][0]);
                assert_eq!(frame[1], frames[index - 1][1]);
            }
        }
        let titles: Vec<_> = plan.output.pages[0]
            .lines
            .iter()
            .filter(|line| {
                line.line
                    .fragments
                    .iter()
                    .map(|fragment| fragment.text.as_str())
                    .collect::<String>()
                    == "Values"
            })
            .collect();
        assert_eq!(titles.len(), count);
        for (title, frame) in titles.iter().zip(frames) {
            assert!(title.top >= frame[1] && title.top + title.line.height <= frame[1] + frame[3]);
            assert!(title.line.fragments[0].x >= frame[0]);
        }
    }
    let document = mote::compile(&format!(
        "#presentation(width=pt(200),height=pt(160))#slide[{chart}]"
    ))
    .unwrap();
    assert!(
        render::plan(&document, &resolver, None)
            .unwrap_err()
            .to_string()
            .contains("automatic slide content does not fit")
    );
    assert!(
        mote::pptx::emit(&document)
            .unwrap_err()
            .to_string()
            .contains("automatic slide content does not fit")
    );
}

#[test]
fn cropped_contain_and_cover_preserve_the_cropped_aspect_ratio() {
    use mote::layout::PageOperation;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentation-images");
    let path = root.join("transforms.mote");
    let resolver = FileAssetResolver::unconfined(&root).unwrap();
    let document = mote::compile(&fs::read_to_string(&path).unwrap()).unwrap();
    let plan = render::plan(&document, &resolver, Some(path.as_os_str())).unwrap();
    let pictures: Vec<_> = plan.output.pages[0]
        .operations
        .iter()
        .filter_map(|operation| {
            if let PageOperation::Image(image) = operation {
                Some(image)
            } else {
                None
            }
        })
        .collect();
    let editable =
        mote::pptx::emit_with_assets(&document, &resolver, Some(path.as_os_str())).unwrap();
    let frames = object_frames(&slide_xml(editable.bytes), "pic");
    assert_eq!(pictures.len(), frames.len());
    for (actual, expected) in pictures[1].bounds.into_iter().zip(frames[1]) {
        assert!((actual - expected).abs() <= 1.0 / 12700.0);
    }
    let contain_ratio = pictures[1].bounds[2] / pictures[1].bounds[3];
    let cover_ratio = pictures[2].bounds[2] / pictures[2].bounds[3];
    assert!((contain_ratio - cover_ratio).abs() < 1e-8);
    assert!(pictures[2].bounds[2] > frames[2][2] || pictures[2].bounds[3] > frames[2][3]);
    for (actual, expected) in pictures[2].clip.into_iter().zip(frames[2]) {
        assert!((actual - expected).abs() <= 1.0 / 12700.0);
    }
}

#[test]
fn image_opacity_fades_pictures_in_editable_and_native_output() {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let slide = |arguments: &str| {
        mote::compile(&format!(
            "#presentation(width=pt(96),height=pt(64))#slide[#box(position={{width=pt(96),height=pt(64)}},fit='stretch')[#image('pattern.png'{arguments})]]"
        ))
        .unwrap()
    };
    let painted = |arguments: &str| {
        let plan = render::plan_with_layout(
            &slide(arguments),
            &resolver,
            None,
            render::LayoutMode::Presentation,
        )
        .unwrap();
        assert_eq!(plan.warnings, Vec::<String>::new(), "{arguments}");
        assert!(
            render::paint_pdf(&plan.output)
                .unwrap()
                .starts_with(b"%PDF-")
        );
        let pages = render::paint_png(&plan.output, &Default::default())
            .unwrap()
            .output;
        capture::pixels(&pages[0].bytes).2
    };
    let opaque = painted("");
    let faded = painted(",opacity=0.4");
    assert_eq!(faded, painted(",opacity='40%'"));
    assert_eq!(faded, painted(",opacity=40"));
    assert_eq!(faded, painted(",opacity=0.8,effects={transparency=50}"));
    assert_ne!(faded, opaque);
    // The slide is white, so a faded pixel lies two fifths of the way from white
    // to the opaque pixel.
    for (faded, opaque) in faded.iter().zip(&opaque) {
        let expected = 255.0 - 0.4 * (255.0 - f64::from(*opaque));
        assert!(
            (f64::from(*faded) - expected).abs() <= 2.0,
            "{faded} {expected}"
        );
    }
    // Two pictures of one source that differ only in opacity are prepared apart.
    let pair = mote::compile(
        "#presentation(width=pt(200),height=pt(64))#slide[\
         #box(position={width=pt(96),height=pt(64)})[#image('pattern.png')]\
         #box(position={x=pt(100),width=pt(96),height=pt(64)})[#image('pattern.png',opacity=0.4)]]",
    )
    .unwrap();
    let plan =
        render::plan_with_layout(&pair, &resolver, None, render::LayoutMode::Presentation).unwrap();
    let pictures: Vec<_> = plan.output.pages[0]
        .operations
        .iter()
        .filter_map(|operation| match operation {
            mote::layout::PageOperation::Image(image) => Some(&image.image),
            _ => None,
        })
        .collect();
    assert_eq!(pictures.len(), 2);
    assert_ne!(pictures[0], pictures[1]);
    for (arguments, amount) in [
        (",opacity=0.4", "40000"),
        (",opacity='40%'", "40000"),
        (",opacity=0.8,effects={transparency=50}", "40000"),
        (",opacity=3,effects={grayscale=true}", "3000"),
    ] {
        let package =
            mote::pptx::emit_with_fonts(&slide(arguments), &resolver, None, None).unwrap();
        assert!(
            slide_xml(package.bytes).contains(&format!("<a:alphaModFix amt=\"{amount}\"/>")),
            "{arguments}"
        );
    }
}

#[test]
fn a_stated_corner_radius_gives_rounded_rectangles_of_any_size_the_same_corner() {
    use mote::layout::PageOperation;
    let resolver = FileAssetResolver::new(".").unwrap();
    let document = mote::compile(
        "#presentation(width=pt(400),height=pt(300))#slide[\
         #shape(position={x=pt(10),y=pt(10),width=pt(300),height=pt(200)},kind='round_rect',corner_radius=pt(6),fill='#336699')\
         #shape(position={x=pt(10),y=pt(220),width=pt(60),height=pt(20)},kind='rounded',corner_radius=pt(6),fill='#336699')\
         #shape(position={x=pt(100),y=pt(220),width=pt(60),height=pt(20)},kind='round_rect',corner_radius=pt(40),fill='#336699')\
         #shape(position={x=pt(200),y=pt(220),width=pt(60),height=pt(24)},kind='round_rect',fill='#336699')]",
    )
    .unwrap();
    let plan =
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    assert_eq!(plan.warnings, Vec::<String>::new());
    // A rounded rectangle begins where its top edge meets the top-right corner.
    let corners: Vec<f64> = plan.output.pages[0]
        .operations
        .iter()
        .filter_map(|operation| match operation {
            PageOperation::Path(path) => {
                let right = path.points.iter().map(|point| point[0]).fold(0.0, f64::max);
                Some(right - path.points[0][0])
            }
            _ => None,
        })
        .collect();
    assert_eq!(corners.len(), 4);
    for (corner, expected) in corners
        .iter()
        .zip([6.0, 6.0, 10.0, 24.0 * 16667.0 / 100000.0])
    {
        assert!((corner - expected).abs() < 1e-9, "{corner} {expected}");
    }
    let xml = slide_xml(mote::pptx::emit(&document).unwrap());
    let adjustments: Vec<&str> = xml
        .split("<a:prstGeom prst=\"roundRect\">")
        .skip(1)
        .map(|rest| rest.split("</a:prstGeom>").next().unwrap())
        .collect();
    assert_eq!(
        adjustments,
        [
            "<a:avLst><a:gd name=\"adj\" fmla=\"val 3000\"/></a:avLst>",
            "<a:avLst><a:gd name=\"adj\" fmla=\"val 30000\"/></a:avLst>",
            "<a:avLst><a:gd name=\"adj\" fmla=\"val 50000\"/></a:avLst>",
            "<a:avLst/>",
        ]
    );

    let misplaced = mote::compile(
        "#slide[#shape(position={width=pt(60),height=pt(20)},kind='ellipse',corner_radius=pt(6))]",
    )
    .unwrap();
    for warnings in [
        render::plan_with_layout(
            &misplaced,
            &resolver,
            None,
            render::LayoutMode::Presentation,
        )
        .unwrap()
        .warnings,
        mote::pptx::emit_with_warnings(&misplaced).unwrap().warnings,
    ] {
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].ends_with(".corner_radius has an effect only on a round_rect shape"));
    }
    assert!(
        mote::compile("#slide[#shape(position={width=pt(60),height=pt(20)},corner_radius=pt(-6))]")
            .is_err()
    );
}

#[test]
fn groups_become_powerpoint_group_shapes_and_leave_native_pictures_unchanged() {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let objects = [
        "#shape(name='Panel',position={x=pt(20),y=pt(30),width=pt(120),height=pt(80)},fill='#dcfce7')[Panel]",
        "#box(position={x=pt(30),y=pt(40),width=pt(48),height=pt(32)})[#image('pattern.png')]",
        "#line(from={x=pt(150),y=pt(100)},to={x=pt(180),y=pt(20)})",
        "#shape(position={x=pt(170),y=pt(60),width=pt(20),height=pt(20)},kind='ellipse',fill='#15803d')",
    ];
    let deck = |body: String| {
        mote::compile(&format!(
            "#presentation(width=pt(200),height=pt(150))#slide[{body}]"
        ))
        .unwrap()
    };
    let grouped = deck(format!(
        "#group(name='Card & frame')[{}{}#group[{}{}]]",
        objects[0], objects[1], objects[2], objects[3]
    ));
    let plan = |document: &mote::model::Document| {
        render::plan_with_layout(document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap()
    };
    let grouped_plan = plan(&grouped);
    assert_eq!(grouped_plan.warnings, Vec::<String>::new());
    let single = deck(objects.concat());
    assert_eq!(
        grouped_plan.output.pages[0].operations,
        plan(&single).output.pages[0].operations
    );

    let package = mote::pptx::emit_with_fonts(&grouped, &resolver, None, None).unwrap();
    assert_eq!(package.warnings, Vec::<String>::new());
    let xml = slide_xml(package.bytes);
    let tree = roxmltree::Document::parse(&xml).unwrap();
    fn named<'tree, 'input>(
        node: roxmltree::Node<'tree, 'input>,
        name: &str,
    ) -> Vec<roxmltree::Node<'tree, 'input>> {
        node.children()
            .filter(|child| child.has_tag_name(name))
            .collect()
    }
    let shape_tree = tree
        .descendants()
        .find(|node| node.has_tag_name("spTree"))
        .unwrap();
    let [outer] = named(shape_tree, "grpSp")[..] else {
        panic!("the slide holds one group shape");
    };
    let frame = |group: roxmltree::Node<'_, '_>| {
        let transform = group
            .descendants()
            .find(|node| node.has_tag_name("xfrm"))
            .unwrap();
        ["off", "ext", "chOff", "chExt"].map(|name| {
            let field = named(transform, name)[0];
            field
                .attributes()
                .map(|attribute| attribute.value().parse::<i64>().unwrap() / 12700)
                .collect::<Vec<_>>()
        })
    };
    let title = |group: roxmltree::Node<'_, '_>| {
        group
            .descendants()
            .find(|node| node.has_tag_name("cNvPr"))
            .unwrap()
            .attribute("name")
            .unwrap()
            .to_owned()
    };
    assert_eq!(title(outer), "Card & frame");
    assert_eq!(
        frame(outer),
        [vec![20, 20], vec![170, 90], vec![20, 20], vec![170, 90]]
    );
    assert_eq!(named(outer, "sp").len(), 1);
    assert_eq!(named(outer, "pic").len(), 1);
    let [inner] = named(outer, "grpSp")[..] else {
        panic!("the group holds one group shape");
    };
    assert!(title(inner).starts_with("Group "));
    assert_eq!(
        frame(inner),
        [vec![150, 20], vec![40, 80], vec![150, 20], vec![40, 80]]
    );
    assert_eq!(named(inner, "cxnSp").len(), 1);
    assert_eq!(named(inner, "sp").len(), 1);
    let identifiers: std::collections::BTreeSet<_> = tree
        .descendants()
        .filter(|node| node.has_tag_name("cNvPr"))
        .map(|node| node.attribute("id").unwrap())
        .collect();
    assert_eq!(identifiers.len(), 7);

    let empty =
        mote::compile("#slide[#group[#group[]]#shape(position={width=pt(4),height=pt(4)})]")
            .unwrap();
    let package = mote::pptx::emit_with_warnings(&empty).unwrap();
    assert!(!slide_xml(package.bytes).contains("<p:grpSp>"));
    for warnings in [package.warnings, plan(&empty).warnings] {
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        for warning in warnings {
            assert!(warning.ends_with(".group is empty and is omitted"));
        }
    }
}

#[test]
fn a_box_of_several_pieces_becomes_a_group_stacked_down_its_frame() {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let deck = |body: &str| {
        mote::compile(&format!(
            "#presentation(width=pt(400),height=pt(300))#slide[{body}]"
        ))
        .unwrap()
    };
    let plan = |document: &mote::model::Document| {
        render::plan_with_layout(document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap()
    };
    let pieces = [
        "#paragraph[Caption]",
        "#table[#row[#cell[A]#cell[B]]]",
        "#image('pattern.png')",
        "#paragraph[Note]",
    ];
    let stacked = deck(&format!(
        "#box(name='Card',position={{x=pt(20),y=pt(10),width=pt(240),height=pt(260)}},fill='#dcfce7',text={{size=pt(14)}})[{}]",
        pieces.concat()
    ));
    let package = mote::pptx::emit_with_fonts(&stacked, &resolver, None, None).unwrap();
    assert_eq!(package.warnings, Vec::<String>::new());
    let xml = slide_xml(package.bytes);
    assert_eq!(xml.matches("<p:grpSp>").count(), 1);
    assert!(xml.contains("name=\"Card\""));
    let frame = [20.0, 10.0, 240.0, 260.0];
    assert_eq!(object_frames(&xml, "grpSp"), [frame]);
    let [background, caption, note] = object_frames(&xml, "sp")[..] else {
        panic!("the group holds a background and two text boxes");
    };
    let [table] = object_frames(&xml, "graphicFrame")[..] else {
        panic!("the group holds one table");
    };
    let [picture] = object_frames(&xml, "pic")[..] else {
        panic!("the group holds one picture");
    };
    assert_eq!(background, frame);
    let gap = 7.2;
    let close = |first: f64, second: f64| (first - second).abs() < 0.01;
    assert_eq!(caption[..3], [20.0, 10.0, 240.0]);
    assert!(caption[3] > 14.0 && caption[3] < 28.0);
    assert_eq!(table[0], 20.0);
    assert!(close(table[1], caption[1] + caption[3] + gap));
    // The picture is 96 by 64 pixels, and an equal share of the frame is less
    // than the height its proportion gives at the width of the frame.
    assert!(close(picture[1], table[1] + table[3] + gap));
    assert!(close(picture[3], table[3]));
    assert!(close(picture[2], picture[3] * 1.5));
    assert!(close(picture[0] + picture[2] / 2.0, 140.0));
    assert!(close(note[1], picture[1] + picture[3] + gap));
    assert!(close(note[1] + note[3], 270.0));

    let position = |[x, y, width, height]: [f64; 4]| {
        format!("position={{x=pt({x}),y=pt({y}),width=pt({width}),height=pt({height})}}")
    };
    let explicit = deck(&format!(
        "#group(name='Card')[\
         #shape(kind='rect',{},fill='#dcfce7')\
         #box({},text={{size=pt(14)}})[{}]\
         #box({})[{}]\
         #box({},fit='contain')[{}]\
         #box({},text={{size=pt(14)}})[{}]]",
        position(frame),
        position(caption),
        pieces[0],
        position(table),
        pieces[1],
        position([20.0, picture[1], 240.0, picture[3]]),
        pieces[2],
        position(note),
        pieces[3],
    ));
    let stacked_plan = plan(&stacked);
    assert_eq!(stacked_plan.warnings, Vec::<String>::new());
    assert_eq!(
        stacked_plan.output.pages[0].operations,
        plan(&explicit).output.pages[0].operations
    );

    let crowded = deck(
        "#box(name='Tag',position={width=pt(240),height=pt(20)},text={fit='shrink'})[\
         #paragraph[Caption]#table[#row[#cell[A]]]]",
    );
    assert_eq!(
        plan(&crowded).warnings,
        [
            "box \"Tag\" exceeds its frame",
            "box \"Tag\" ignores text.fit"
        ]
    );

    let error = mote::compile(
        "#slide(layout='title_and_content')[#box(slot='title')[\
         #paragraph[Title]#table[#row[#cell[A]]]]]",
    )
    .unwrap_err();
    assert_eq!(
        error.message,
        "layout slot \"title\" requires text content, got mixed content"
    );
}

#[test]
fn effects_are_drawn_as_they_stand_at_a_time_and_written_as_powerpoint_timing() {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let deck = |objects: &str, time| {
        mote::compile_with_options(
            &format!("#presentation(width=pt(200),height=pt(150))#slide(duration=4,advance_after=6)[{objects}]"),
            mote::evaluation::EvaluationOptions {
                time,
                ..Default::default()
            },
        )
        .unwrap()
    };
    let operations = |document: &mote::model::Document| {
        let planned =
            render::plan_with_layout(document, &resolver, None, render::LayoutMode::Presentation)
                .unwrap();
        assert_eq!(planned.warnings, Vec::<String>::new());
        planned.output.pages[0].operations.clone()
    };
    let mover = |position: &str, animate: &str| {
        format!(
            "#shape(name='Mover',position={{{position},width=pt(40),height=pt(20)}},fill='#2563eb'{animate})[Go]"
        )
    };
    let moves = ",animate={{effect='move',by={x=pt(100)},start=1,duration=2},\
                 {effect='move',by={y=pt(30)},start=3,duration=1,easing='ease_in'}}";
    let card = |arguments: &str| {
        format!(
            "#group(name='Card'{arguments})[\
             #shape(position={{x=pt(20),y=pt(60),width=pt(60),height=pt(60)}},fill='#ff0000')\
             #shape(position={{x=pt(50),y=pt(80),width=pt(60),height=pt(60)}},fill='#0000ff')]"
        )
    };
    let fades = ",position={x=pt(10)},animate={{effect='fade_in',duration=2},\
                 {effect='fade_out',start=3,duration=1}}";
    let spinner = |animate: &str| {
        format!(
            "#shape(name='Spinner',position={{x=pt(140),y=pt(80),width=pt(40),height=pt(20)}},fill='#d97706'{animate})"
        )
    };
    let turns = ",animate={{effect='rotate',by=90,duration=1},{effect='scale',to=2,duration=1}}";
    let rule = |animate: &str| {
        format!(
            "#line(name='Rule',from={{x=pt(10),y=pt(145)}},to={{x=pt(190),y=pt(145)}}{animate})"
        )
    };
    let late = |animate: &str| {
        format!(
            "#box(name='Late',position={{x=pt(120),y=pt(10),width=pt(60),height=pt(20)}}{animate})[Late]"
        )
    };
    let turned = card(",rotation=15");
    let animated = [
        mover("x=pt(20),y=pt(10)", moves),
        card(fades),
        spinner(turns),
        rule(",animate={effect='disappear',start=1}"),
        late(",animate={effect='appear',start=2}"),
    ]
    .concat();

    // A slide rendered without a time is the slide as authored.
    let authored = [
        mover("x=pt(20),y=pt(10)", ""),
        card(",position={x=pt(10)}"),
        spinner(""),
        rule(""),
        late(""),
    ]
    .concat();
    assert_eq!(
        operations(&deck(&animated, None)),
        operations(&deck(&authored, None))
    );

    // An object out of view is not drawn, and a displaced one is drawn in its place.
    let layer = |operations: &[mote::layout::PageOperation]| {
        operations
            .iter()
            .filter_map(|operation| match operation {
                mote::layout::PageOperation::PushLayer { matrix, opacity } => Some((
                    matrix.map(|value| (value * 1000.0).round() / 1000.0),
                    *opacity,
                )),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let start = operations(&deck(&animated, Some(0.0)));
    assert_eq!(layer(&start), []);
    assert_eq!(
        start,
        operations(&deck(
            &[mover("x=pt(20),y=pt(10)", ""), spinner(""), rule("")].concat(),
            None
        ))
    );
    let without_layers = |operations: Vec<mote::layout::PageOperation>| {
        operations
            .into_iter()
            .filter(|operation| {
                !matches!(
                    operation,
                    mote::layout::PageOperation::PushLayer { .. }
                        | mote::layout::PageOperation::PopLayer
                )
            })
            .collect::<Vec<_>>()
    };
    let moved = operations(&deck(&animated, Some(3.0)));
    assert_eq!(layer(&moved), [([0.0, 2.0, -2.0, 0.0, 340.0, -230.0], 1.0)]);
    assert_eq!(
        without_layers(moved),
        operations(&deck(
            &[
                mover("x=pt(120),y=pt(10)", ""),
                card(",position={x=pt(10)}"),
                spinner(""),
                late(""),
            ]
            .concat(),
            None
        ))
    );
    // An effect that gathers speed through its first half has covered a third
    // of its way at its middle.
    let late_operations = operations(&deck(&animated, Some(3.5)));
    assert_eq!(
        layer(&late_operations),
        [
            ([1.0, 0.0, 0.0, 1.0, 0.0, 0.0], 0.5),
            ([0.0, 2.0, -2.0, 0.0, 340.0, -230.0], 1.0)
        ]
    );
    assert!(late_operations.iter().any(|operation| matches!(
        operation,
        mote::layout::PageOperation::Path(path) if path.points[0] == [120.0, 20.0]
    )));

    // A group fades as one object: the lower shape does not show through the upper.
    let pixel = |document: &mote::model::Document, [x, y]: [usize; 2]| {
        let planned =
            render::plan_with_layout(document, &resolver, None, render::LayoutMode::Presentation)
                .unwrap();
        render::paint_pdf(&planned.output).unwrap();
        let page = &render::paint_png(&planned.output, &render::PngOptions::default())
            .unwrap()
            .output[0];
        let mut reader = png::Decoder::new(std::io::Cursor::new(&page.bytes))
            .read_info()
            .unwrap();
        let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
        let frame = reader.next_frame(&mut pixels).unwrap();
        let channels = frame.color_type.samples();
        let offset = (y * frame.width as usize + x) * channels;
        [pixels[offset], pixels[offset + 1], pixels[offset + 2]]
    };
    let faint = deck(&animated, Some(1.0));
    for (point, color) in [([40, 70], [255, 128, 128]), ([75, 100], [128, 128, 255])] {
        let actual = pixel(&faint, point);
        for (actual, expected) in actual.into_iter().zip(color) {
            assert!(actual.abs_diff(expected) <= 2, "{point:?}: {actual}");
        }
    }
    assert_eq!(pixel(&deck(&animated, Some(2.0)), [75, 100]), [0, 0, 255]);

    let package =
        mote::pptx::emit_with_fonts(&deck(&animated, None), &resolver, None, None).unwrap();
    assert_eq!(package.warnings, Vec::<String>::new());
    let xml = slide_xml(package.bytes);
    let tree = roxmltree::Document::parse(&xml).unwrap();
    let identifier = |name: &str| {
        tree.descendants()
            .find(|node| node.has_tag_name("cNvPr") && node.attribute("name") == Some(name))
            .unwrap()
            .attribute("id")
            .unwrap()
            .to_owned()
    };
    let effects: Vec<_> = tree
        .descendants()
        .filter(|node| node.has_tag_name("cTn") && node.attribute("presetID").is_some())
        .map(|node| {
            let find = |name: &str| node.descendants().find(|child| child.has_tag_name(name));
            (
                find("spTgt").unwrap().attribute("spid").unwrap().to_owned(),
                node.attribute("presetClass").unwrap(),
                node.attribute("presetID").unwrap(),
                find("cond").unwrap().attribute("delay").unwrap(),
                node.attribute("nodeType").unwrap(),
                node.attribute("accel"),
                find("animMotion").and_then(|motion| motion.attribute("path")),
            )
        })
        .collect();
    let [mover, card, spinner, rule, late] =
        ["Mover", "Card", "Spinner", "Rule", "Late"].map(identifier);
    assert_eq!(
        effects,
        [
            (card.clone(), "entr", "10", "0", "afterEffect", None, None),
            (spinner.clone(), "emph", "8", "0", "withEffect", None, None),
            (spinner.clone(), "emph", "6", "0", "withEffect", None, None),
            (
                mover.clone(),
                "path",
                "0",
                "1000",
                "withEffect",
                None,
                Some("M 0 0 L 0.5 0")
            ),
            (rule, "exit", "1", "1000", "withEffect", None, None),
            (late.clone(), "entr", "1", "2000", "withEffect", None, None),
            (
                mover.clone(),
                "path",
                "0",
                "3000",
                "withEffect",
                Some("50000"),
                Some("M 0.5 0 L 0.5 0.2")
            ),
            (card, "exit", "10", "3000", "withEffect", None, None),
        ]
    );
    assert!(xml.contains(r#"<p:animRot by="5400000">"#));
    assert!(xml.contains(r#"<p:by x="200000" y="200000"/>"#));
    // A shape with a text body has one build for each of its effects.
    let builds: Vec<_> = tree
        .descendants()
        .filter(|node| node.has_tag_name("bldP"))
        .map(|node| {
            (
                node.attribute("spid").unwrap().to_owned(),
                node.attribute("grpId").unwrap(),
            )
        })
        .collect();
    assert_eq!(
        builds,
        [
            (spinner.clone(), "0"),
            (spinner, "1"),
            (mover.clone(), "0"),
            (late, "0"),
            (mover, "1")
        ]
    );
    assert!(xml.contains(r#"<p:transition advTm="6000"/><p:timing>"#));
    assert_eq!(object_frames(&xml, "grpSp")[0], [30.0, 60.0, 90.0, 80.0]);

    let still = mote::pptx::emit_with_warnings(&deck(&authored, None)).unwrap();
    assert!(!slide_xml(still.bytes).contains("<p:timing>"));
    // A duration alone leaves the advance of a slide to the speaker.
    let timed =
        mote::compile("#slide(duration=4)[#shape(position={width=pt(4),height=pt(4)})]").unwrap();
    let package = mote::pptx::emit_with_warnings(&timed).unwrap();
    assert_eq!(package.warnings, Vec::<String>::new());
    assert!(!slide_xml(package.bytes).contains("<p:transition"));
    let turned = deck(&turned, None);
    let xml = slide_xml(mote::pptx::emit_with_warnings(&turned).unwrap().bytes);
    assert!(xml.contains(r#"<a:xfrm rot="900000">"#), "{xml}");
    assert_eq!(
        layer(&operations(&turned)),
        [([0.966, 0.259, -0.259, 0.966, 28.097, -13.416], 1.0)]
    );
}

#[test]
fn a_box_governs_the_fit_of_its_image_and_the_text_of_its_paragraphs_and_table() {
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let plan = |source: &str| {
        let document = mote::compile(&format!(
            "#presentation(width=pt(400),height=pt(300))\n{source}"
        ))
        .unwrap();
        let planned =
            render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
                .unwrap();
        let package = mote::pptx::emit_with_fonts(&document, &resolver, None, None).unwrap();
        (planned, package.warnings, slide_xml(package.bytes))
    };

    // An image keeps its proportion in a box that states no fit.
    let picture = |fit: &str| {
        let (planned, warnings, xml) = plan(&format!(
            "#slide[#box(position={{x=pt(20),y=pt(20),width=pt(300),height=pt(100)}}{fit})[#image('pattern.png')]]"
        ));
        assert_eq!(planned.warnings, Vec::<String>::new());
        assert_eq!(warnings, Vec::<String>::new());
        (
            planned.output.pages[0].operations.clone(),
            object_frames(&xml, "pic"),
        )
    };
    let (default_operations, default_frames) = picture("");
    assert_eq!(
        (default_operations, default_frames.clone()),
        picture(",fit='contain'")
    );
    let [frame] = default_frames[..] else {
        panic!("the slide holds one picture");
    };
    assert!(frame[2] < 300.0 && frame[3] == 100.0, "{frame:?}");
    assert_eq!(picture(",fit='stretch'").1, [[20.0, 20.0, 300.0, 100.0]]);

    let (planned, warnings, _) = plan(
        "#slide[#box(position={width=pt(300),height=pt(100)})[#image('pattern.png',width=pt(40))]]",
    );
    for warnings in [planned.warnings, warnings] {
        assert_eq!(
            warnings,
            [
                "content[0].slide[0].box[0].image.width has no effect on a slide: the box sets the size of its image"
            ]
        );
    }

    // The text settings of a box override the Normal style, which governs
    // what the box leaves unstated, and they reach a table within the box.
    let (planned, warnings, xml) = plan(
        "#style \"Normal\" (run={size=pt(14),color='#aa0000'})\n\
         #slide[\n\
         #box(position={x=pt(10),y=pt(10),width=pt(380),height=pt(60)},text={size=pt(30)})[Sized by the box]\n\
         #box(position={x=pt(10),y=pt(80),width=pt(380),height=pt(60)})[Sized by the style]\n\
         #box(position={x=pt(10),y=pt(150),width=pt(380),height=pt(120)},text={size=pt(22),color='#0000aa'})[\n\
         | Rocket | Year |\n| --- | --- |\n| Saturn V | 1967 |\n]\n]",
    );
    assert_eq!(planned.warnings, Vec::<String>::new());
    assert_eq!(warnings, Vec::<String>::new());
    let tree = roxmltree::Document::parse(&xml).unwrap();
    let run = |text: &str| {
        let run = tree
            .descendants()
            .find(|node| {
                node.has_tag_name("r") && node.descendants().any(|child| child.text() == Some(text))
            })
            .unwrap();
        let properties = run
            .children()
            .find(|node| node.has_tag_name("rPr"))
            .unwrap();
        (
            properties.attribute("sz").unwrap().to_owned(),
            properties
                .descendants()
                .find(|node| node.has_tag_name("srgbClr"))
                .map(|node| node.attribute("val").unwrap().to_owned()),
        )
    };
    assert_eq!(
        run("Sized by the box"),
        ("3000".into(), Some("AA0000".into()))
    );
    assert_eq!(
        run("Sized by the style"),
        ("1400".into(), Some("AA0000".into()))
    );
    assert_eq!(run("Saturn V"), ("2200".into(), Some("0000AA".into())));
    assert_eq!(run("Rocket").0, "2200");
    let sizes: Vec<_> = planned.output.pages[0]
        .lines
        .iter()
        .flat_map(|line| &line.line.fragments)
        .map(|fragment| fragment.style.measurement.size_points)
        .collect();
    for expected in [30.0, 14.0, 22.0] {
        assert!(sizes.contains(&expected), "{expected}: {sizes:?}");
    }
}

#[test]
fn the_footer_fields_take_the_field_text_of_the_deck_and_of_the_slide() {
    let document = mote::compile(
        "#presentation(width=pt(400),height=pt(300),footer='Foot',slide_number=true,\
         field_text={size=pt(14),color='#f59e0b',bold=true})\
         #slide[#shape(position={width=pt(4),height=pt(4)})]\
         #slide(field_text={color='#38bdf8',italic=true,font='Georgia'})[\
         #shape(position={width=pt(4),height=pt(4)})]",
    )
    .unwrap();
    let package = mote::pptx::emit_with_warnings(&document).unwrap();
    assert_eq!(package.warnings, Vec::<String>::new());
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(package.bytes)).unwrap();
    let mut slide = |number: usize| {
        use std::io::Read;
        let mut xml = String::new();
        archive
            .by_name(&format!("ppt/slides/slide{number}.xml"))
            .unwrap()
            .read_to_string(&mut xml)
            .unwrap();
        xml
    };
    assert_eq!(
        slide(1)
            .matches(r#"<a:rPr lang="en-US" sz="1400" b="1"><a:solidFill><a:srgbClr val="F59E0B"/></a:solidFill></a:rPr>"#)
            .count(),
        2
    );
    assert_eq!(
        slide(2)
            .matches(r#"<a:rPr lang="en-US" sz="1400" b="1" i="1"><a:solidFill><a:srgbClr val="38BDF8"/></a:solidFill><a:latin typeface="Georgia"/></a:rPr>"#)
            .count(),
        2
    );
    let resolver = FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let planned =
        render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
            .unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    for (page, color) in planned.output.pages.iter().zip(["F59E0B", "38BDF8"]) {
        let fields: Vec<_> = page
            .lines
            .iter()
            .flat_map(|line| &line.line.fragments)
            .map(|fragment| {
                (
                    fragment.style.measurement.size_points,
                    fragment.style.color.as_str(),
                )
            })
            .collect();
        assert_eq!(fields, [(14.0, color), (14.0, color)]);
    }
}

#[test]
fn unknown_members_of_effects_displacements_and_field_text_warn_and_still_render() {
    let document = mote::compile(
        "#presentation(footer='F',field_text={size=pt(12),weight=700})\
         #slide(field_text={shade='dark'})[\
         #shape(position={x=pt(10),y=pt(10),width=pt(40),height=pt(20)},fill='#2563eb',animate={\
         {effect='fade_in',to=2},\
         {effect='move',by={x=pt(10),z=pt(10)},from='left'},\
         {effect='appear',duration=1}})\
         #group(position={x=pt(10),depth=pt(10)})[\
         #shape(position={x=pt(100),y=pt(10),width=pt(40),height=pt(20)})]]",
    )
    .unwrap();
    let package = mote::pptx::emit_with_warnings(&document).unwrap();
    let expected = [
        "content[0].slide.field_text.shade",
        "content[0].slide[0].shape.animate[0].to",
        "content[0].slide[0].shape.animate[1].by.z",
        "content[0].slide[0].shape.animate[1].from",
        "content[0].slide[0].shape.animate[2].duration",
        "content[0].slide[1].group.position.depth",
        "presentation.field_text.weight",
    ]
    .map(|path| format!("{path} is retained without a presentation output effect"));
    assert_eq!(package.warnings, expected);
    let xml = slide_xml(package.bytes);
    assert_eq!(xml.matches("presetID=").count(), 3);
    assert!(xml.contains(r#"sz="1200""#));
}
