use mote::{
    measurement::{CoreFont, FontSource, FontVariant, MeasurementStyle, TextMeasurer},
    source::{AssetRequest, FileAssetResolver, ResolvedAsset},
};
use std::{cell::Cell, ffi::OsStr, io, path::Path};

const FONT: &[u8] = include_bytes!("fixtures/measurements/fonts/SourceSans3-Regular.ttf");

#[test]
fn declared_variants_load_relative_to_the_source_and_match_captured_metrics() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_name = root.join("tests/fixtures/measurements/requests.json");
    let document = mote::compile(
        r#"#font(name = "body", latin = "Source Sans 3", files = {
        regular = "fonts/SourceSans3-Regular.ttf", bold = "fonts/SourceSans3-Bold.ttf",
        italic = "fonts/SourceSans3-It.ttf", bold_italic = "fonts/SourceSans3-BoldIt.ttf"
    })"#,
    )
    .unwrap();
    let resolver = FileAssetResolver::new(root).unwrap();
    let mut measurer = TextMeasurer::new(&document, &resolver, Some(source_name.as_os_str()));
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/measurements/expected.json")).unwrap();
    for variant in [
        FontVariant::Regular,
        FontVariant::Bold,
        FontVariant::Italic,
        FontVariant::BoldItalic,
    ] {
        let measured = measurer
            .measure(
                &MeasurementStyle {
                    font: "body".into(),
                    size_points: 12.0,
                    variant,
                },
                "AV To Wa",
                true,
            )
            .unwrap();
        assert_eq!(measured.font.source, FontSource::Declared);
        assert_eq!(measured.font.family, "body");
        assert_eq!(measured.font.variant, variant);
        assert!(measured.font.substitution.is_none());
        assert!(
            measured
                .font
                .asset_name
                .unwrap()
                .to_string_lossy()
                .contains("fonts/SourceSans3-")
        );
        assert!(measured.font.bytes.unwrap().len() > 100_000);
        let identifier = format!("source_sans_3_{}_ttf_12_kerning_kerned", variant.as_text());
        let capture = expected["measurements"]
            .as_array()
            .unwrap()
            .iter()
            .find(|record| record["id"] == identifier)
            .unwrap();
        assert_eq!(
            measured.measurement.layout_width,
            capture["layout_width"].as_f64().unwrap()
        );
    }
}

#[test]
fn custom_font_sources_are_cached_and_exact_bytes_are_retained_for_painting() {
    let document = mote::compile(r#"#font(name="body", files={regular="body.ttf"})"#).unwrap();
    let calls = Cell::new(0);
    let resolver = |request: AssetRequest<'_>| {
        assert_eq!(request.reference, "body.ttf");
        assert_eq!(
            request.source_name,
            Some(OsStr::new("memory://report/main.mote"))
        );
        calls.set(calls.get() + 1);
        Ok(ResolvedAsset {
            name: "memory://fonts/body".into(),
            bytes: FONT.to_vec(),
        })
    };
    let mut measurer = TextMeasurer::new(
        &document,
        &resolver,
        Some(OsStr::new("memory://report/main.mote")),
    );
    let style = MeasurementStyle {
        font: "body".into(),
        ..Default::default()
    };
    let first = measurer.measure(&style, "Hello", true).unwrap();
    let second = measurer.measure(&style, "AV", false).unwrap();
    assert_eq!(calls.get(), 1);
    assert_eq!(first.font.bytes.as_deref(), Some(FONT));
    assert!(std::sync::Arc::ptr_eq(
        first.font.bytes.as_ref().unwrap(),
        second.font.bytes.as_ref().unwrap()
    ));
}

#[test]
fn missing_variants_do_not_synthesize_faces_or_use_the_latin_mapping() {
    let document =
        mote::compile(r#"#font(name="body", latin="Courier", files={regular="body.ttf"})"#)
            .unwrap();
    let resolver = |_: AssetRequest<'_>| -> io::Result<ResolvedAsset> {
        panic!("missing variant must not request an asset")
    };
    let mut measurer = TextMeasurer::new(&document, &resolver, None);
    let selected = measurer.resolve("body", FontVariant::Bold);
    assert_eq!(selected.family, "Helvetica");
    assert_eq!(selected.variant, FontVariant::Bold);
    assert_eq!(
        selected.substitution.as_deref(),
        Some("missing bold font file")
    );
    assert!(selected.bytes.is_none());
}

#[test]
fn failed_custom_resolution_is_authoritative_and_reports_fallback() {
    let document = mote::compile(r#"#font(name="body", files={regular="tests/fixtures/measurements/fonts/SourceSans3-Regular.ttf"})"#).unwrap();
    let calls = Cell::new(0);
    let resolver = |_: AssetRequest<'_>| {
        calls.set(calls.get() + 1);
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "asset unavailable",
        ))
    };
    let mut measurer = TextMeasurer::new(&document, &resolver, None);
    let style = MeasurementStyle {
        font: "body".into(),
        ..Default::default()
    };
    let first = measurer.measure(&style, "Hello", true).unwrap();
    let second = measurer.measure(&style, "Hello", true).unwrap();
    assert_eq!(calls.get(), 1);
    assert_eq!(first, second);
    assert_eq!(first.font.source, FontSource::Core);
    assert!(
        first
            .font
            .substitution
            .unwrap()
            .contains("could not be read")
    );
    assert_eq!(
        first.measurement,
        CoreFont::Helvetica
            .measure("Hello", 11.0, FontVariant::Regular)
            .unwrap()
    );
}

#[test]
fn aliases_are_exact_and_core_substitutions_remain_visible() {
    let document =
        mote::compile(r#"#font(name="body", latin="Times") #font(name="other", latin="body")"#)
            .unwrap();
    let resolver = |_: AssetRequest<'_>| -> io::Result<ResolvedAsset> {
        panic!("named fonts must not search the filesystem")
    };
    let mut measurer = TextMeasurer::new(&document, &resolver, None);
    assert_eq!(
        measurer.resolve("body", FontVariant::Regular).family,
        "Times"
    );
    assert!(
        measurer
            .resolve("body", FontVariant::Regular)
            .substitution
            .is_none()
    );
    assert!(
        measurer
            .resolve("Body", FontVariant::Regular)
            .substitution
            .is_some()
    );
    assert_eq!(
        measurer.resolve("other", FontVariant::Regular).family,
        "Helvetica"
    );
    for (name, family) in [
        ("mono", "Courier"),
        ("Arial", "Helvetica"),
        ("Georgia", "Times"),
        ("sans-serif", "Helvetica"),
        ("sans serif", "Times"),
    ] {
        assert_eq!(
            measurer.resolve(name, FontVariant::BoldItalic).family,
            family,
            "{name}"
        );
    }
    assert!(
        measurer
            .resolve("Arial", FontVariant::Regular)
            .substitution
            .unwrap()
            .contains("unavailable")
    );
}

#[test]
fn conflicting_resolver_names_never_replace_selected_font_bytes() {
    let document = mote::compile(
        r#"#font(name="first", files={regular="a"}) #font(name="second", files={regular="b"})"#,
    )
    .unwrap();
    let resolver = |request: AssetRequest<'_>| {
        let mut bytes = FONT.to_vec();
        if request.reference == "b" {
            bytes.push(0);
        }
        Ok(ResolvedAsset {
            name: "same-name".into(),
            bytes,
        })
    };
    let mut measurer = TextMeasurer::new(&document, &resolver, None);
    let first = measurer.resolve("first", FontVariant::Regular);
    let second = measurer.resolve("second", FontVariant::Regular);
    assert_eq!(first.bytes.as_deref(), Some(FONT));
    assert_eq!(second.source, FontSource::Core);
    assert!(second.substitution.is_some());
    assert_eq!(measurer.resolve("first", FontVariant::Regular), first);
}

#[test]
fn embedding_permissions_are_checked_before_selecting_declared_font_data() {
    let document = mote::compile(r#"#font(name="body", files={regular="body.ttf"})"#).unwrap();
    for (permissions, allowed) in [
        (0, true),
        (8, true),
        (2, false),
        (4, false),
        (0x100, false),
        (0x200, false),
        (0x8000, false),
        (12, false),
    ] {
        let mut bytes = FONT.to_vec();
        let count = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
        let directory = (0..usize::from(count))
            .map(|index| 12 + index * 16)
            .find(|offset| &bytes[*offset..*offset + 4] == b"OS/2")
            .unwrap();
        let offset =
            u32::from_be_bytes(bytes[directory + 8..directory + 12].try_into().unwrap()) as usize;
        bytes[offset + 8..offset + 10].copy_from_slice(&u16::to_be_bytes(permissions));
        let resolver = |_: AssetRequest<'_>| {
            Ok(ResolvedAsset {
                name: "font".into(),
                bytes: bytes.clone(),
            })
        };
        let mut measurer = TextMeasurer::new(&document, &resolver, None);
        let selection = measurer.resolve("body", FontVariant::Regular);
        assert_eq!(
            selection.bytes.is_some(),
            allowed,
            "permissions {permissions:#06x}"
        );
        assert_eq!(selection.substitution.is_none(), allowed);
    }
}

#[test]
fn ambiguous_and_overlapping_permission_tables_use_visible_fallbacks() {
    let table_count = usize::from(u16::from_be_bytes([FONT[4], FONT[5]]));
    let permission_record = (0..table_count)
        .map(|index| 12 + index * 16)
        .find(|offset| &FONT[*offset..*offset + 4] == b"OS/2")
        .unwrap();
    let mut duplicate = FONT.to_vec();
    let other_record = (0..table_count)
        .map(|index| 12 + index * 16)
        .find(|offset| *offset != permission_record)
        .unwrap();
    duplicate[other_record..other_record + 16]
        .copy_from_slice(&FONT[permission_record..permission_record + 16]);
    let mut overlapping = FONT.to_vec();
    overlapping[permission_record + 8..permission_record + 12]
        .copy_from_slice(&12_u32.to_be_bytes());
    let mut truncated = FONT.to_vec();
    truncated[permission_record + 12..permission_record + 16].copy_from_slice(&9_u32.to_be_bytes());
    for (bytes, reason) in [
        (duplicate, "duplicate OS/2"),
        (overlapping, "invalid OS/2"),
        (truncated, "invalid OS/2"),
    ] {
        let document = mote::compile(r#"#font(name="body",files={regular="body.ttf"})"#).unwrap();
        let resolver = |_: AssetRequest<'_>| {
            Ok(ResolvedAsset {
                name: "memory://font".into(),
                bytes: bytes.clone(),
            })
        };
        let mut measurer = TextMeasurer::new(&document, &resolver, None);
        let selected = measurer.resolve("body", FontVariant::Regular);
        assert_eq!(selected.source, FontSource::Core);
        assert!(selected.bytes.is_none());
        assert!(selected.substitution.unwrap().contains(reason));
    }
}
