use mote::{
    layout::DocumentTextPlan,
    render::{self, PngOptions},
    source::FileAssetResolver,
};
use std::{fs, io::Cursor, path::Path};

fn fixture(name: &str) -> DocumentTextPlan {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = root.join(format!("tests/fixtures/native-output/{name}.mote"));
    let document = mote::compile(&fs::read_to_string(&source).unwrap()).unwrap();
    let resolver = FileAssetResolver::new(root).unwrap();
    render::plan(&document, &resolver, Some(source.as_os_str()))
        .unwrap()
        .output
}
fn decode(bytes: &[u8]) -> (png::OutputInfo, Vec<u8>) {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA);
    let mut reader = decoder.read_info().unwrap();
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut pixels).unwrap();
    pixels.truncate(info.buffer_size());
    (info, pixels)
}
#[test]
fn core_and_declared_fonts_generate_deterministic_pdf_and_png_pages() {
    for name in ["core", "truetype", "cff"] {
        let plan = fixture(name);
        let first = render::paint_pdf(&plan).unwrap();
        assert_eq!(first, render::paint_pdf(&plan).unwrap());
        assert!(first.starts_with(b"%PDF-1.7"));
        assert!(first.ends_with(b"%%EOF"));
        let pages = render::paint_png(&plan, &PngOptions::default())
            .unwrap()
            .output;
        assert_eq!(pages.len(), if name == "core" { 2 } else { 1 });
        let repeated = render::paint_png(&plan, &PngOptions::default())
            .unwrap()
            .output;
        for (page, repeated) in pages.iter().zip(repeated) {
            assert_eq!(page.bytes, repeated.bytes);
            let (info, pixels) = decode(&page.bytes);
            assert_eq!(info.width, 360);
            assert_eq!(info.height, if name == "core" { 300 } else { 240 });
            assert_eq!(info.color_type, png::ColorType::Rgba);
            assert!(
                pixels
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .all(|pixel| pixel[3] == 255)
            );
            let foreground = pixels
                .as_chunks::<4>()
                .0
                .iter()
                .filter(|pixel| pixel[..3] != [255, 255, 255])
                .count();
            assert!(foreground > 100, "{name} page is blank");
        }
    }
}
#[test]
fn resolution_scales_pixels_without_changing_pagination() {
    let plan = fixture("core");
    let output = render::paint_png(
        &plan,
        &PngOptions {
            dots_per_inch: 144,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(output.output.len(), 2);
    for page in output.output {
        assert_eq!((page.width, page.height), (720, 600));
        let reader = png::Decoder::new(Cursor::new(page.bytes))
            .read_info()
            .unwrap();
        let density = reader.info().pixel_dims.unwrap();
        assert_eq!(density.unit, png::Unit::Meter);
        assert_eq!(density.xppu, 5669);
    }
}
#[test]
fn invalid_geometry_and_excessive_raster_allocations_are_rejected() {
    let mut plan = fixture("core");
    for resolution in [0, 9601] {
        assert!(
            render::paint_png(
                &plan,
                &PngOptions {
                    dots_per_inch: resolution,
                    ..Default::default()
                }
            )
            .is_err()
        );
    }
    assert!(
        render::paint_png(
            &plan,
            &PngOptions {
                maximum_page_pixels: 10,
                ..Default::default()
            }
        )
        .is_err()
    );
    plan.pages[0].lines[0].line.fragments[0].x = f64::NAN;
    assert!(render::paint_pdf(&plan).is_err());
    assert!(render::paint_png(&plan, &PngOptions::default()).is_err());
}
#[test]
fn font_substitution_and_missing_glyphs_are_reported_before_painting() {
    let document=mote::compile("#font(name=\"missing\",files={regular=\"missing.ttf\"})\n#paragraph(font=\"missing\")[a☃☃]").unwrap();
    let resolver = FileAssetResolver::from_current_directory().unwrap();
    let output = render::emit_pdf(&document, &resolver, None).unwrap();
    assert!(
        output
            .warnings
            .iter()
            .any(|warning| warning.contains("could not be read"))
    );
    assert_eq!(
        output
            .warnings
            .iter()
            .filter(|warning| warning.contains("U+2603"))
            .count(),
        1
    );
}

#[test]
fn declared_font_rasters_match_frozen_coverage_with_explicit_edge_tolerance() {
    for name in ["truetype", "cff"] {
        let plan = fixture(name);
        let output = render::paint_png(&plan, &PngOptions::default()).unwrap();
        let expected = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "tests/fixtures/native-output/expected/{name}-1.png"
        )))
        .unwrap();
        let (expected_info, expected) = decode(&expected);
        let (actual_info, actual) = decode(&output.output[0].bytes);
        assert_eq!(
            (actual_info.width, actual_info.height),
            (expected_info.width, expected_info.height)
        );
        let mut total = 0_u64;
        let mut foreground_channels = 0_u64;
        let mut maximum = 0;
        for (expected, actual) in expected
            .as_chunks::<4>()
            .0
            .iter()
            .zip(actual.as_chunks::<4>().0.iter())
        {
            if expected[..3] != [255, 255, 255] || actual[..3] != [255, 255, 255] {
                for channel in 0..3 {
                    let difference = expected[channel].abs_diff(actual[channel]);
                    maximum = maximum.max(difference);
                    total += u64::from(difference);
                    foreground_channels += 1;
                }
            }
        }
        let mean = total as f64 / foreground_channels as f64;
        assert!(
            maximum <= 48 && mean <= 8.0,
            "{name}: maximum={maximum}, foreground mean={mean}"
        );
    }
}
