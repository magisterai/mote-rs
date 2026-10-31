#[path = "support/language_capture.rs"]
mod language_capture;
use mote::{
    evaluation::EvaluationOptions,
    model::{Content, Element},
    source::{AssetRequest, ResolvedAsset},
};
use std::cell::RefCell;
fn element(content: &Content) -> &Element {
    let Content::Element(element) = content else {
        panic!("element")
    };
    element
}

#[test]
fn imported_presets_match_captured_models_packages_and_rasters() {
    language_capture::compare("imports", &["document", "slides"]);
}

#[test]
fn imports_export_locals_once_and_deferred_data_uses_its_definition_source() {
    let requests = RefCell::new(Vec::new());
    let resolver = |request: AssetRequest<'_>| {
        requests.borrow_mut().push((
            request.source_name.map(ToOwned::to_owned),
            request.reference.to_owned(),
        ));
        let reference = request.reference.to_str().unwrap();
        let (name, source) = match reference {
            "preset.mote" | "alias.mote" => (
                "library/preset.mote",
                "#let(label=\"Library\")\n#set paragraph(alignment=\"right\")\n#function data() return json(\"data.json\").label end\nDiscarded.",
            ),
            "data.json"
                if request.source_name == Some(std::ffi::OsStr::new("library/preset.mote")) =>
            {
                ("library/data.json", "{\"label\":\"Correct file\"}")
            }
            _ => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, reference)),
        };
        Ok(ResolvedAsset {
            name: name.into(),
            bytes: source.as_bytes().to_vec(),
        })
    };
    let document = mote::compile_with_options("Before.\n\n#import(\"preset.mote\")\n#let(label=\"Caller\")\n#import(\"alias.mote\")\n#import(\"preset.mote\")\n#paragraph[#label / #data()]", EvaluationOptions { asset_resolver: Some(&resolver), source_name: Some(std::ffi::OsStr::new("main.mote")), enable_lua: true, ..Default::default() }).unwrap();
    assert_eq!(
        document.imports,
        [std::ffi::OsString::from("library/preset.mote")]
    );
    assert_eq!(document.content.len(), 2);
    assert!(
        !element(&document.content[0])
            .attributes
            .contains_key("alignment")
    );
    assert_eq!(
        element(&document.content[1]).attributes["alignment"].to_text(),
        "right"
    );
    assert_eq!(document.content[1].to_text(), "Caller / Correct file");
    assert_eq!(
        requests
            .borrow()
            .iter()
            .filter(|(_, path)| path == "preset.mote")
            .count(),
        1
    );
}

#[test]
fn imports_detect_cycles_and_report_the_imported_source_for_failures() {
    for (imported, expected, line) in [
        ("\n\n#paragraph(alignment=\"wrong\")[bad]", "alignment", 3),
        ("#import(\"preset.mote\")", "cycle", 1),
    ] {
        let resolver = |_request: AssetRequest<'_>| {
            Ok(ResolvedAsset {
                name: "library/preset.mote".into(),
                bytes: imported.as_bytes().to_vec(),
            })
        };
        let error =
            mote::compile_with_assets("#import(\"preset.mote\")", &resolver, None).unwrap_err();
        assert!(error.message.contains(expected), "{error}");
        assert_eq!(error.source.as_ref().unwrap().name, "library/preset.mote");
        assert_eq!(error.location("caller text").0, line);
    }
    for source in [
        "#import()",
        "#import(12)",
        "#import(\"x\",unexpected=true)",
        "#import(\"x\")[body]",
    ] {
        assert!(mote::compile(source).is_err());
    }
}

#[test]
fn nested_lua_imports_restore_host_callbacks_and_keep_function_diagnostics() {
    let resolver = |request: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: request.reference.to_owned(),
            bytes: b"#let(label=upper(\"nested\"))\n#function failure() return 1 + nil end"
                .to_vec(),
        })
    };
    let options = EvaluationOptions {
        asset_resolver: Some(&resolver),
        enable_lua: true,
        ..Default::default()
    };
    let document = mote::compile_with_options(
        "#let(loaded=import(\"preset.mote\"))\n#paragraph[#label / #upper(\"after\")]",
        options,
    )
    .unwrap();
    assert_eq!(document.content[0].to_text(), "NESTED / AFTER");
    let error =
        mote::compile_with_options("#import(\"preset.mote\")\n#failure()", options).unwrap_err();
    assert_eq!(error.source.as_ref().unwrap().name, "preset.mote");
    assert_eq!(error.location("caller").0, 2);
}

#[test]
fn import_defaults_end_with_the_branch_and_repeated_imports_do_not_reexport() {
    let resolver = |_request: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "preset.mote".into(),
            bytes: b"#set paragraph(alignment=\"right\")\n#let(hidden=1)".to_vec(),
        })
    };
    let document = mote::compile_with_assets(
        "#if(true)[#import(\"preset.mote\")\nInside.]\n\n#import(\"preset.mote\")\nOutside.",
        &resolver,
        None,
    )
    .unwrap();
    assert_eq!(
        element(&document.content[0]).attributes["alignment"].to_text(),
        "right"
    );
    assert!(
        !element(&document.content[1])
            .attributes
            .contains_key("alignment")
    );
    let error = mote::compile_with_assets(
        "#if(true)[#import(\"preset.mote\")]\n#import(\"preset.mote\")\n#hidden",
        &resolver,
        None,
    )
    .unwrap_err();
    assert!(error.message.contains("unknown binding"));
}
