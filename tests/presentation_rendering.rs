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
    let resolver = FileAssetResolver::new(&root).unwrap();
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
        capture::compare(
            &presentation_capture::projection(&plan.output),
            &read("geometry.json"),
            case,
        );
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
    assert!(
        render::plan_with_layout(&report, &resolver, None, render::LayoutMode::Presentation)
            .is_err()
    );
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
        .is_err()
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
    for source in [
        "#slide(notes='Speaker')[]",
        "#slide[Loose content]",
        "#slide[#box(position={width=inch(3),height=inch(2)})[#chart(data={values={1}})]]",
    ] {
        let document = mote::compile(source).unwrap();
        assert!(
            render::plan_with_layout(&document, &resolver, None, render::LayoutMode::Presentation)
                .is_err(),
            "{source}"
        );
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
    let resolver = FileAssetResolver::new(&root).unwrap();
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
