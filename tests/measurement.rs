use mote::measurement::{CoreFont, FontVariant, MeasurementError, OpenTypeFont};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};

const REGULAR: &[u8] = include_bytes!("fixtures/measurements/fonts/SourceSans3-Regular.ttf");
const COMPACT: &[u8] = include_bytes!("fixtures/measurements/fonts/SourceSans3-Regular.otf");

#[test]
fn all_captured_measurements_match_native_metrics() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/measurements");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let fonts: BTreeMap<_, _> = requests["fonts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|font| (font["id"].as_str().unwrap(), font))
        .collect();
    let expected: BTreeMap<_, _> = expected["measurements"]
        .as_array()
        .unwrap()
        .iter()
        .map(|result| (result["id"].as_str().unwrap(), result))
        .collect();
    assert_eq!(requests["cases"].as_array().unwrap().len(), 960);
    for request in requests["cases"].as_array().unwrap() {
        let descriptor = fonts[request["font_id"].as_str().unwrap()];
        let variant = match descriptor["variant"].as_str().unwrap() {
            "regular" => FontVariant::Regular,
            "bold" => FontVariant::Bold,
            "italic" => FontVariant::Italic,
            "bold_italic" => FontVariant::BoldItalic,
            value => panic!("unexpected variant {value}"),
        };
        let text = request["text"].as_str().unwrap();
        let size = request["size_points"].as_f64().unwrap();
        let measured = if descriptor["format"] == "core" {
            let font = match descriptor["family"].as_str().unwrap() {
                "Helvetica" => CoreFont::Helvetica,
                "Times" => CoreFont::Times,
                "Courier" => CoreFont::Courier,
                value => panic!("unexpected family {value}"),
            };
            font.measure(text, size, variant).unwrap()
        } else {
            let bytes = fs::read(root.join(descriptor["path"].as_str().unwrap())).unwrap();
            OpenTypeFont::parse(&bytes)
                .unwrap()
                .measure(text, size, request["layout_kerning"].as_bool().unwrap())
                .unwrap()
        };
        let identifier = request["id"].as_str().unwrap();
        let expected = expected[identifier];
        assert_eq!(
            measured.prepared_text,
            expected["prepared_text"].as_str().unwrap(),
            "{identifier}"
        );
        let missing: Vec<_> = measured
            .missing_codepoints
            .iter()
            .map(|character| u64::from(*character as u32))
            .collect();
        assert_eq!(
            missing,
            expected["missing_codepoints"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_u64().unwrap())
                .collect::<Vec<_>>(),
            "{identifier}"
        );
        for (field, actual) in [
            ("layout_width", measured.layout_width),
            ("paint_width", measured.paint_width),
            ("ascent", measured.extents.ascent),
            ("descent", measured.extents.descent),
            ("height", measured.extents.height),
        ] {
            assert!(
                (expected[field].as_f64().unwrap() - actual).abs() <= 1e-9,
                "{identifier}: {field}: expected {}, actual {actual}",
                expected[field]
            );
        }
    }
}

#[test]
fn size_validation_and_omitted_size_are_consistent() {
    let face = OpenTypeFont::parse(REGULAR).unwrap();
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1e100] {
        assert_eq!(
            face.measure("A", invalid, true),
            Err(MeasurementError::InvalidSize)
        );
        assert_eq!(
            CoreFont::Times.measure("A", invalid, FontVariant::Regular),
            Err(MeasurementError::InvalidSize)
        );
    }
    for default in [0.0, -1.0] {
        assert_eq!(
            face.measure("AV", default, true),
            face.measure("AV", 11.0, true)
        );
        assert_eq!(
            CoreFont::Times.measure("AV", default, FontVariant::Regular),
            CoreFont::Times.measure("AV", 11.0, FontVariant::Regular)
        );
    }
}

#[test]
fn kerning_scale_and_paint_width_are_distinct_from_layout_advance() {
    let truetype = OpenTypeFont::parse(REGULAR).unwrap();
    let compact = OpenTypeFont::parse(COMPACT).unwrap();
    for size in [9.5, 12.0, 24.0] {
        let first = truetype.measure("AV To Wa", size, false).unwrap();
        let second = compact.measure("AV To Wa", size, false).unwrap();
        assert_eq!(first.layout_width, second.layout_width);
        assert_eq!(first.layout_width - first.paint_width, 1.5);
        assert_ne!(first.paint_width, second.paint_width);
        assert_eq!(
            truetype
                .measure("AV To Wa", size, true)
                .unwrap()
                .layout_width,
            first.paint_width
        );
    }
}

#[test]
fn missing_characters_are_replaced_and_reported_once_in_source_order() {
    let face = OpenTypeFont::parse(REGULAR).unwrap();
    let measured = face.measure("字☃字☃", 12.0, true).unwrap();
    assert_eq!(measured.prepared_text, "????");
    assert_eq!(measured.missing_codepoints, ['字', '☃']);
    let core = CoreFont::Helvetica
        .measure("\n\t\0\u{7f}€字", 12.0, FontVariant::Regular)
        .unwrap();
    assert_eq!(core.prepared_text, "????€?");
    assert_eq!(core.missing_codepoints, ['\n', '\t', '\0', '\u{7f}', '字']);
}

#[test]
fn malformed_fonts_and_collection_containers_are_explicit_errors() {
    for bytes in [b"".as_slice(), b"not a font", &REGULAR[..100]] {
        assert!(matches!(
            OpenTypeFont::parse(bytes),
            Err(MeasurementError::InvalidFont)
        ));
    }
    assert!(matches!(
        OpenTypeFont::parse(b"ttcf"),
        Err(MeasurementError::UnsupportedFont(_))
    ));
    for length in (0..REGULAR.len()).step_by(997) {
        let _ = OpenTypeFont::parse(&REGULAR[..length]);
    }
}
