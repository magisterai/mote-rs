#[allow(dead_code)]
#[path = "docx/support.rs"]
mod xml_support;
use mote::{
    layout::PageOperation,
    model::{Content, ElementKind},
    render::{self, PngOptions},
    source::{AssetRequest, FileAssetResolver, ResolvedAsset},
};
use std::{
    cell::Cell,
    ffi::OsStr,
    fs,
    io::{self, Cursor, Read},
    path::PathBuf,
    sync::Arc,
};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/images")
}
fn normalize_numbers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        serde_json::Value::Object(fields) => fields.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}
fn pixels(bytes: &[u8]) -> (u32, u32, Vec<u8>) {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA);
    let mut reader = decoder.read_info().unwrap();
    let mut buffer = vec![0; reader.output_buffer_size().unwrap()];
    let output = reader.next_frame(&mut buffer).unwrap();
    buffer.truncate(output.buffer_size());
    assert_eq!(output.color_type, png::ColorType::Rgba);
    (output.width, output.height, buffer)
}

#[test]
fn image_models_packages_and_native_pages_match_captures() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "geometry",
        "transforms",
        "placement",
        "flow",
        "contexts",
        "color-spaces",
        "subframe",
    ] {
        let source_name = format!("{case}.mote");
        let source = fs::read_to_string(fixtures().join(&source_name)).unwrap();
        let document = mote::compile(&source).unwrap();
        let original = document.clone();
        let capture = fixtures().join("captured").join(case);
        let mut actual: serde_json::Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}: model");
        let generated =
            mote::docx::emit_with_assets(&document, &resolver, Some(OsStr::new(&source_name)))
                .unwrap();
        let mut archive = zip::ZipArchive::new(Cursor::new(generated.bytes)).unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("capture.json")).unwrap()).unwrap();
        assert_eq!(archive.len(), manifest["parts"].as_array().unwrap().len());
        for name in manifest["parts"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            let mut actual = Vec::new();
            archive
                .by_name(name)
                .unwrap()
                .read_to_end(&mut actual)
                .unwrap();
            let expected = fs::read(capture.join("package").join(name)).unwrap();
            if name.ends_with(".xml") || name.ends_with(".rels") {
                xml_support::compare_xml(
                    std::str::from_utf8(&actual).unwrap(),
                    std::str::from_utf8(&expected).unwrap(),
                    &format!("{case}/{name}"),
                );
            } else {
                assert_eq!(actual, expected, "{case}/{name}: original media");
            }
        }
        let plan =
            render::plan_with_fonts(&document, &resolver, Some(OsStr::new(&source_name)), None)
                .unwrap();
        assert_eq!(
            plan.output.pages.len() as u64,
            manifest["pages"].as_u64().unwrap()
        );
        let expected_images: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("images.json")).unwrap()).unwrap();
        let actual_images: Vec<_> = plan
            .output
            .pages
            .iter()
            .enumerate()
            .flat_map(|(page_index, page)| {
                page.operations
                    .iter()
                    .filter_map(move |operation| match operation {
                        PageOperation::Image(image) => Some((page_index + 1, image)),
                        _ => None,
                    })
            })
            .collect();
        assert_eq!(
            actual_images.len(),
            expected_images.as_array().unwrap().len()
        );
        for ((page_number, actual), expected) in actual_images
            .iter()
            .zip(expected_images.as_array().unwrap())
        {
            assert_eq!(*page_number as u64, expected["page"].as_u64().unwrap());
            for (field, coordinates) in [("bounds", actual.bounds), ("clip", actual.clip)] {
                for (coordinate, expected) in
                    coordinates.iter().zip(expected[field].as_array().unwrap())
                {
                    assert!(
                        (coordinate - expected.as_f64().unwrap()).abs() < 0.011,
                        "{case}: {field}"
                    );
                }
            }
            let expected_pixels =
                pixels(&fs::read(capture.join(expected["pixels"].as_str().unwrap())).unwrap());
            assert_eq!(
                (actual.image.raster.width, actual.image.raster.height),
                (expected_pixels.0, expected_pixels.1)
            );
            let tolerance = expected["pixel_tolerance"].as_u64().unwrap() as u16;
            for (actual, expected) in actual
                .image
                .raster
                .pixels
                .as_chunks::<4>()
                .0
                .iter()
                .zip(expected_pixels.2.as_chunks::<4>().0)
            {
                assert!(actual[3].abs_diff(expected[3]) <= 1, "{case}: alpha");
                for channel in 0..3 {
                    let actual = u16::from(actual[channel]) * u16::from(actual[3]) / 255;
                    let expected = u16::from(expected[channel]) * u16::from(expected[3]) / 255;
                    assert!(
                        actual.abs_diff(expected) <= tolerance,
                        "{case}: transformed pixel {actual} versus {expected}"
                    );
                }
            }
        }
        let painted = render::paint_png(&plan.output, &PngOptions::default()).unwrap();
        for page in painted.output {
            let expected =
                pixels(&fs::read(capture.join(format!("reference-{}.png", page.number))).unwrap());
            let actual = pixels(&page.bytes);
            assert_eq!((actual.0, actual.1), (expected.0, expected.1));
            let mut difference_total = 0_u64;
            let mut foreground_channels = 0_u64;
            let mut maximum_difference = 0;
            for (actual, expected) in actual
                .2
                .as_chunks::<4>()
                .0
                .iter()
                .zip(expected.2.as_chunks::<4>().0)
            {
                if actual[..3] != [255; 3] || expected[..3] != [255; 3] {
                    foreground_channels += 3;
                    for channel in 0..3 {
                        let difference = actual[channel].abs_diff(expected[channel]);
                        maximum_difference = maximum_difference.max(difference);
                        difference_total += u64::from(difference);
                    }
                }
                assert_eq!(actual[3], 255);
            }
            let mean = difference_total as f64 / foreground_channels.max(1) as f64;
            assert!(
                maximum_difference <= 48 && mean < if case == "contexts" { 10.0 } else { 2.0 },
                "{case}/{}: maximum {maximum_difference}, foreground mean {mean}",
                page.number
            );
        }
        let pdf = render::paint_pdf(&plan.output).unwrap();
        let serialized = String::from_utf8_lossy(&pdf);
        if case == "color-spaces" {
            assert!(serialized.contains("/DeviceGray"));
            assert!(serialized.contains("/DeviceCMYK"));
            assert_eq!(serialized.matches("/DCTDecode").count(), 2);
        }
        if case == "geometry" {
            assert!(serialized.contains("/SMask"));
            assert!(serialized.contains("/URI (https://example.com/image)"));
            assert_eq!(serialized.matches("/DCTDecode").count(), 1);
        }
        assert_eq!(document, original);
    }
}

#[test]
fn image_assets_are_source_relative_cached_and_frozen_before_painting() {
    let calls = Cell::new(0);
    let available = Cell::new(true);
    let resolver = |request: AssetRequest<'_>| {
        assert!(available.get());
        calls.set(calls.get() + 1);
        assert_eq!(request.source_name, Some(OsStr::new("memory:report")));
        assert_eq!(request.reference, OsStr::new("picture"));
        Ok(ResolvedAsset {
            name: "memory:picture".into(),
            bytes: include_bytes!("fixtures/images/assets/pattern.png").to_vec(),
        })
    };
    let document = mote::compile("#image(\"picture\",width=pt(40))\n#image(\"picture\",width=pt(40))\n#image(\"picture\",width=pt(50),rotation=30)").unwrap();
    let planned = render::plan_with_fonts(
        &document,
        &resolver,
        Some(OsStr::new("memory:report")),
        None,
    )
    .unwrap();
    assert_eq!(calls.get(), 1);
    let images: Vec<_> = planned
        .output
        .pages
        .iter()
        .flat_map(|page| &page.operations)
        .filter_map(|operation| match operation {
            PageOperation::Image(image) => Some(image),
            _ => None,
        })
        .collect();
    assert_eq!(images.len(), 3);
    assert!(Arc::ptr_eq(&images[0].image, &images[1].image));
    available.set(false);
    assert!(!render::paint_pdf(&planned.output).unwrap().is_empty());
    for resolution in [72, 96, 144] {
        let painted = render::paint_png(
            &planned.output,
            &PngOptions {
                dots_per_inch: resolution,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            painted.output[0].width,
            (planned.output.geometry.width * f64::from(resolution) / 72.0).ceil() as u32
        );
    }
    assert_eq!(calls.get(), 1);
}

#[test]
fn unavailable_or_invalid_images_have_visible_placeholders_and_diagnostics() {
    let resolver = |request: AssetRequest<'_>| match request.reference.to_str().unwrap() {
        "missing" => Err(io::Error::new(io::ErrorKind::NotFound, "missing fixture")),
        "external" => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "external fixture",
        )),
        _ => Ok(ResolvedAsset {
            name: request.reference.to_owned(),
            bytes: b"not an image".to_vec(),
        }),
    };
    let document = mote::compile(
        "#image(\"missing\",alt=\"Missing diagram\")\n#image(\"external\")\n#image(\"invalid\")",
    )
    .unwrap();
    let planned = render::plan_with_fonts(&document, &resolver, None, None).unwrap();
    let text: String = planned
        .output
        .pages
        .iter()
        .flat_map(|page| page.text_lines())
        .flat_map(|line| &line.fragments)
        .map(|fragment| fragment.text.as_str())
        .collect();
    assert!(text.contains("[missing image: missing]"));
    assert!(text.contains("[unsupported image: external]"));
    assert!(text.contains("[unsupported image: invalid]"));
    assert!(
        planned
            .warnings
            .iter()
            .filter(|warning| warning.contains("placeholder"))
            .count()
            >= 3
    );
    let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    let parts = xml_support::unpack(generated.bytes);
    assert!(parts["word/document.xml"].contains("[Image: Missing diagram]"));
    assert_eq!(generated.warnings.len(), 3);
    assert!(
        generated
            .warnings
            .iter()
            .all(|warning| warning.contains("placeholder"))
    );
}

#[test]
fn image_arguments_and_placement_rules_are_validated() {
    for source in [
        "#image()",
        "#image(7)",
        "#image(\"\")",
        "#image(\"a\",\"b\")",
        "#image(\"a\",src=\"b\")",
        "#image(\"a\")[body]",
        "#image(\"a\",width=pt(-1))",
        "#image(\"a\",alignment=\"top\")",
        "#image(\"a\",flip_horizontal=\"perhaps\")",
        "#image(\"a\",border={style=\"wavy\"})",
        "#image(\"a\",placement={relative_height=-1})",
        "#image(\"a\",placement={horizontal={relative_to=\"line\"}})",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let document =
        mote::compile("#image(\"primary\",properties={src=\"override\",custom=\"retained\"})")
            .unwrap();
    let Content::Element(image) = &document.content[0] else {
        panic!()
    };
    assert_eq!(image.kind, ElementKind::Image);
    assert_eq!(image.attributes["src"].to_text(), "primary");
    assert_eq!(image.attributes["custom"].to_text(), "retained");
}

#[test]
fn retained_image_metadata_does_not_claim_floating_layout() {
    let document = mote::compile(r#"#image("assets/pattern.png", name="Object", description="Description", title="Title", style="Metadata", placement={type="floating",float="top",wrap="square",anchor="page",x=pt(10),y=pt(12),margin={top=pt(2)},allow_overlap=true,behind_text=false,layout_in_cell=true,locked=false,relative_height=3,horizontal={offset=pt(2),relative_to="page",alignment="inside"},vertical={offset=pt(3),relative_to="paragraph",alignment="bottom"}},border={style="none"})"#).unwrap();
    let Content::Element(image) = &document.content[0] else {
        panic!()
    };
    let mote::model::Value::Object(placement) = &image.attributes["placement"] else {
        panic!()
    };
    assert_eq!(placement["relative_height"].to_text(), "3");
    assert_eq!(placement["behind_text"], mote::model::Value::Boolean(false));
    assert_eq!(image.attributes["description"].to_text(), "Description");
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    let plan = render::plan_with_fonts(&document, &resolver, None, None).unwrap();
    assert!(
        plan.warnings
            .iter()
            .any(|warning| warning.contains("placement.type"))
    );
    assert!(
        plan.warnings
            .iter()
            .any(|warning| warning.contains("style"))
    );
    assert_eq!(
        plan.output.pages[0]
            .operations
            .iter()
            .filter(|operation| matches!(operation, PageOperation::Image(_)))
            .count(),
        1
    );
    assert!(
        !plan.output.pages[0]
            .operations
            .iter()
            .any(|operation| matches!(operation, PageOperation::OutlineRectangle { .. }))
    );
}

#[test]
fn image_painters_reject_corrupt_pixel_buffers_in_a_modified_plan() {
    let document = mote::compile("#image(\"assets/pattern.png\")").unwrap();
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    let mut plan = render::plan_with_fonts(&document, &resolver, None, None)
        .unwrap()
        .output;
    for (width, height) in [(2, 2), (u32::MAX, u32::MAX), (0, 1)] {
        let PageOperation::Image(image) = &mut plan.pages[0].operations[0] else {
            panic!()
        };
        let raster = &mut Arc::make_mut(&mut image.image).raster;
        raster.width = width;
        raster.height = height;
        raster.pixels = Arc::from([0_u8]);
        assert!(render::paint_pdf(&plan).is_err());
        assert!(render::paint_png(&plan, &PngOptions::default()).is_err());
    }
}
