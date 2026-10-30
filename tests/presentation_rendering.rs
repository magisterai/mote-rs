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
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/presentation-text");
    let resolver = FileAssetResolver::new(&root).unwrap();
    for (case, maximum_limit, mean_limit, edge_limit) in [
        ("basics", 48, 6.0, 0),
        ("edges", 48, 9.0, 0),
        ("fitting", 64, 2.0, 0),
        ("font-assets", 64, 6.0, 0),
        ("formatting", 48, 0.2, 0),
        ("lua", 40, 5.0, 0),
        ("styles", 48, 6.0, 0),
        ("theme", 48, 0.1, 0),
    ] {
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
        "#slide[#box(position={width=inch(3),height=inch(2)})[#image(src='missing.png')]]",
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
fn unsupported_slide_images_are_rejected_without_loading_document_assets() {
    let document = mote::compile(
        "#presentation(size='wide')#slide[#box(position={width=inch(3),height=inch(2)})[#image(src='unused.png')]]",
    )
    .unwrap();
    let requests = std::cell::RefCell::new(Vec::new());
    let resolver = |request: mote::source::AssetRequest<'_>| {
        requests.borrow_mut().push(request.reference.to_owned());
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "unused asset",
        ))
    };
    for mode in [
        render::LayoutMode::Automatic,
        render::LayoutMode::Presentation,
    ] {
        let error =
            render::plan_with_layout_and_fonts(&document, &resolver, None, mode, None).unwrap_err();
        assert!(error.to_string().contains("do not yet support image"));
    }
    assert!(
        requests.borrow().is_empty(),
        "unexpected requests: {requests:?}"
    );
}
