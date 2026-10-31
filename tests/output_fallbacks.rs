#[path = "support/language_capture.rs"]
mod language_capture;

#[test]
fn accepted_fallbacks_preserve_complete_packages_and_native_pages() {
    language_capture::compare(
        "output-fallbacks",
        &[
            "whitespace",
            "whitespace-values",
            "objects",
            "constructed-line",
        ],
    );
}

#[test]
fn omitted_document_objects_report_their_visible_behavior() {
    let document = mote::compile(include_str!("fixtures/output-fallbacks/objects.mote")).unwrap();
    let editable = mote::docx::emit_with_assets(
        &document,
        &mote::source::FileAssetResolver::from_current_directory().unwrap(),
        None,
    )
    .unwrap();
    assert!(
        editable
            .warnings
            .iter()
            .any(|warning| warning.contains("omits standalone line"))
    );
    let resolver = mote::source::FileAssetResolver::from_current_directory().unwrap();
    let native = mote::render::plan(&document, &resolver, None).unwrap();
    assert!(
        native
            .warnings
            .iter()
            .any(|warning| warning.contains("[line] fallback"))
    );
    assert!(
        native
            .warnings
            .iter()
            .any(|warning| warning.contains("chart geometry"))
    );
}
