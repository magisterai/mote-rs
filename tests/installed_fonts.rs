use mote::{
    measurement::{FontSource, FontVariant, TextMeasurer},
    render,
    source::{FileAssetResolver, ResolvedAsset},
};
use std::{cell::Cell, path::Path};
const REGULAR: &[u8] = include_bytes!("fixtures/measurements/fonts/SourceSans3-Regular.ttf");

#[test]
fn exact_font_resolver_feeds_the_same_bytes_to_layout_pdf_and_png() {
    let resolver = FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let installed = mote::compile("#paragraph(font=\"Exact\",size=pt(12))[AV To café]").unwrap();
    let calls = Cell::new(0);
    let fonts = |family: &str, variant: FontVariant| {
        assert_eq!(family, "Exact");
        assert_eq!(variant, FontVariant::Regular);
        calls.set(calls.get() + 1);
        Ok(Some(ResolvedAsset {
            name: "font://exact/regular".into(),
            bytes: REGULAR.to_vec(),
        }))
    };
    let plan = render::plan_with_fonts(&installed, &resolver, None, Some(&fonts))
        .unwrap()
        .output;
    assert_eq!(calls.get(), 1);
    assert!(
        plan.pages[0].lines[0]
            .line
            .fragments
            .iter()
            .all(|fragment| fragment.font.source == FontSource::Installed)
    );
    let declared=mote::compile("#font(name=\"Exact\",files={regular=\"tests/fixtures/measurements/fonts/SourceSans3-Regular.ttf\"})\n#paragraph(font=\"Exact\",size=pt(12))[AV To café]").unwrap();
    let declared = render::plan_with_fonts(&declared, &resolver, None, None)
        .unwrap()
        .output;
    assert_eq!(
        render::paint_pdf(&plan).unwrap(),
        render::paint_pdf(&declared).unwrap()
    );
    assert_eq!(
        render::paint_png(&plan, &Default::default())
            .unwrap()
            .output[0]
            .bytes,
        render::paint_png(&declared, &Default::default())
            .unwrap()
            .output[0]
            .bytes
    );
}
#[test]
fn declared_fonts_take_precedence_and_discovery_is_opt_in_for_measurement() {
    let document=mote::compile("#font(name=\"Exact\",files={regular=\"tests/fixtures/measurements/fonts/SourceSans3-Regular.ttf\"})").unwrap();
    let assets = FileAssetResolver::new(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let fonts = |_: &str, _: FontVariant| -> Result<Option<ResolvedAsset>, String> {
        panic!("declared file must win")
    };
    let mut measurer = TextMeasurer::new(&document, &assets, None).with_font_resolver(&fonts);
    assert_eq!(
        measurer.resolve("Exact", FontVariant::Regular).source,
        FontSource::Declared
    );
    let mut portable = TextMeasurer::new(&document, &assets, None);
    assert_eq!(
        portable.resolve("Arial", FontVariant::Regular).source,
        FontSource::Core
    );
}
#[test]
fn unavailable_variants_and_conflicting_assets_keep_explicit_substitution_reasons() {
    let document = mote::compile("").unwrap();
    let assets = FileAssetResolver::from_current_directory().unwrap();
    let fonts = |family: &str, variant: FontVariant| {
        if variant != FontVariant::Regular {
            return Ok(None);
        }
        let bytes = if family == "first" {
            REGULAR
        } else {
            include_bytes!("fixtures/measurements/fonts/SourceSans3-Regular.otf")
        };
        Ok(Some(ResolvedAsset {
            name: "font://same-name".into(),
            bytes: bytes.to_vec(),
        }))
    };
    let mut measurer = TextMeasurer::new(&document, &assets, None).with_font_resolver(&fonts);
    assert_eq!(
        measurer.resolve("first", FontVariant::Regular).source,
        FontSource::Installed
    );
    assert!(
        measurer
            .resolve("second", FontVariant::Regular)
            .substitution
            .unwrap()
            .contains("conflicting bytes")
    );
    assert_eq!(
        measurer.resolve("first", FontVariant::Bold).source,
        FontSource::Core
    );
}
