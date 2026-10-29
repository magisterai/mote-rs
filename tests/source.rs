use std::{
    cell::Cell,
    ffi::{OsStr, OsString},
    fs, io,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

use mote::source::{
    AssetRequest, AssetResolver, FileAssetResolver, ResolvedAsset, load_source, resolve_asset,
};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        static NEXT_DIRECTORY_IDENTIFIER: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "mote-source-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY_IDENTIFIER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn filesystem_resolves_root_and_source_relative_names() {
    let directory = TestDirectory::new();
    fs::create_dir_all(directory.0.join("reports/chapters")).unwrap();
    fs::write(directory.0.join("reports/main.mote"), "Root source.").unwrap();
    fs::write(directory.0.join("reports/shared.mote"), "Résumé.").unwrap();
    let resolver = FileAssetResolver::new(&directory.0).unwrap();

    let root = load_source(
        &resolver,
        AssetRequest {
            reference: OsStr::new("reports/./main.mote"),
            source_name: None,
        },
    )
    .unwrap();
    assert_eq!(root.name, directory.0.join("reports/main.mote"));
    assert_eq!(root.text, "Root source.");

    for source_name in [
        OsString::from("reports/chapters/entry.mote"),
        directory
            .0
            .join("reports/chapters/entry.mote")
            .into_os_string(),
    ] {
        let source = load_source(
            &resolver,
            AssetRequest {
                reference: OsStr::new("../shared.mote"),
                source_name: Some(&source_name),
            },
        )
        .unwrap();
        assert_eq!(source.name, directory.0.join("reports/shared.mote"));
        assert_eq!(source.text, "Résumé.");
        assert!(mote::compile(&source.text).is_ok());
    }
}

#[test]
fn filesystem_absolute_references_override_the_referring_source() {
    let directory = TestDirectory::new();
    let path = directory.0.join("source.mote");
    fs::write(&path, "Absolute input.").unwrap();
    let resolver = FileAssetResolver::new(directory.0.join("unused")).unwrap();
    let reference = directory.0.join("absent/../source.mote");
    let source = load_source(
        &resolver,
        AssetRequest {
            reference: reference.as_os_str(),
            source_name: Some(OsStr::new("asset://bundle/entry.mote")),
        },
    )
    .unwrap();
    assert_eq!(source.name, path);
    assert_eq!(source.text, "Absolute input.");
}

#[test]
fn current_and_relative_working_directories_anchor_root_requests() {
    let expected_path = std::env::current_dir().unwrap().join("Cargo.toml");
    for resolver in [
        FileAssetResolver::from_current_directory().unwrap(),
        FileAssetResolver::new(".").unwrap(),
    ] {
        let asset = resolve_asset(
            &resolver,
            AssetRequest {
                reference: OsStr::new("Cargo.toml"),
                source_name: Some(OsStr::new("")),
            },
        )
        .unwrap();
        assert_eq!(asset.name, expected_path);
        assert_eq!(asset.bytes, fs::read(&expected_path).unwrap());
    }
}

#[test]
fn custom_resolvers_preserve_authoritative_names_and_requests() {
    let calls = Cell::new(0);
    let resolver = |request: AssetRequest<'_>| {
        calls.set(calls.get() + 1);
        match request.reference.to_str().unwrap() {
            "entry.mote" => {
                assert!(request.source_name.is_none());
                Ok(ResolvedAsset {
                    name: "asset://bundle/reports/main.mote".into(),
                    bytes: b"#paragraph[Root.]".to_vec(),
                })
            }
            "../shared.mote" => {
                assert_eq!(
                    request.source_name,
                    Some(OsStr::new("asset://bundle/reports/main.mote"))
                );
                Ok(ResolvedAsset {
                    name: "asset://library/revision-3/shared.mote".into(),
                    bytes: b"#paragraph[Shared.]".to_vec(),
                })
            }
            _ => panic!("unexpected request: {request:?}"),
        }
    };
    let root = load_source(
        &resolver,
        AssetRequest {
            reference: OsStr::new("entry.mote"),
            source_name: None,
        },
    )
    .unwrap();
    let child = load_source(
        &resolver,
        AssetRequest {
            reference: OsStr::new("../shared.mote"),
            source_name: Some(&root.name),
        },
    )
    .unwrap();
    assert_eq!(calls.get(), 2);
    assert_eq!(child.name, "asset://library/revision-3/shared.mote");
    assert_eq!(
        mote::compile(&child.text).unwrap(),
        mote::compile("#paragraph[Shared.]").unwrap()
    );
}

#[test]
fn omitted_custom_names_are_derived_without_filesystem_access() {
    let resolver = |_: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: OsString::new(),
            bytes: Vec::new(),
        })
    };
    for (reference, source_name, expected) in [
        ("entry.mote", None, PathBuf::from("entry.mote")),
        ("./entry.mote", None, PathBuf::from("entry.mote")),
        ("../entry.mote", None, PathBuf::from("../entry.mote")),
        (
            "../shared.mote",
            Some("bundle/reports/main.mote"),
            PathBuf::from("bundle/shared.mote"),
        ),
        (
            "../shared.mote",
            Some("asset://bundle/reports/main.mote"),
            PathBuf::from("asset://bundle/shared.mote"),
        ),
        (
            "asset://library/shared.mote",
            Some("bundle/main.mote"),
            PathBuf::from("asset://library/shared.mote"),
        ),
    ] {
        let source = load_source(
            &resolver,
            AssetRequest {
                reference: OsStr::new(reference),
                source_name: source_name.map(OsStr::new),
            },
        )
        .unwrap();
        assert_eq!(source.name, expected.as_os_str(), "{reference:?}");
        assert!(source.text.is_empty());
        assert!(mote::compile(&source.text).is_ok());
    }
}

#[test]
fn uri_relative_names_preserve_encoded_segments_queries_and_fragments() {
    let resolver = |_: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: OsString::new(),
            bytes: Vec::new(),
        })
    };
    for (reference, expected) in [
        ("../%2e%2e/shared.mote", "asset://bundle/%2e%2e/shared.mote"),
        ("?revision=2", "asset://bundle/reports/main.mote?revision=2"),
        ("#section", "asset://bundle/reports/main.mote#section"),
        ("part one.mote", "asset://bundle/reports/part%20one.mote"),
        ("café.mote", "asset://bundle/reports/caf%C3%A9.mote"),
        ("../../../../root.mote", "asset://bundle/root.mote"),
        (
            "../%252e%252e/shared.mote",
            "asset://bundle/%252e%252e/shared.mote",
        ),
        ("./", "asset://bundle/reports/"),
        ("../", "asset://bundle/"),
        (
            "nested//leaf.mote",
            "asset://bundle/reports/nested//leaf.mote",
        ),
        ("nested/..", "asset://bundle/reports/"),
    ] {
        let asset = resolve_asset(
            &resolver,
            AssetRequest {
                reference: OsStr::new(reference),
                source_name: Some(OsStr::new("asset://bundle/reports/main.mote")),
            },
        )
        .unwrap();
        assert_eq!(asset.name, expected, "{reference:?}");
    }
}

#[test]
fn opaque_source_names_require_an_explicit_result_for_relative_references() {
    let request = AssetRequest {
        reference: OsStr::new("child.mote"),
        source_name: Some(OsStr::new("data:text/plain,source")),
    };
    let unnamed = |_: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: OsString::new(),
            bytes: Vec::new(),
        })
    };
    let error = resolve_asset(&unnamed, request).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("resolver-supplied name"));

    let named = |_: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "asset://generated/child.mote".into(),
            bytes: Vec::new(),
        })
    };
    assert_eq!(
        resolve_asset(&named, request).unwrap().name,
        "asset://generated/child.mote"
    );
}

#[test]
fn custom_errors_remain_authoritative_even_when_a_local_file_exists() {
    let directory = TestDirectory::new();
    let path = directory.0.join("available.mote");
    fs::write(&path, "This must not be read.").unwrap();
    let calls = Cell::new(0);
    let resolver = |request: AssetRequest<'_>| {
        calls.set(calls.get() + 1);
        assert_eq!(request.reference, path.as_os_str());
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "access denied",
        ))
    };
    let error = load_source(
        &resolver,
        AssetRequest {
            reference: path.as_os_str(),
            source_name: None,
        },
    )
    .unwrap_err();
    assert_eq!(calls.get(), 1);
    assert_eq!(error.name, path);
    assert_eq!(error.cause.kind(), io::ErrorKind::PermissionDenied);
    assert_eq!(error.cause.to_string(), "access denied");
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn loading_failures_keep_names_and_invalid_text_never_reaches_compilation() {
    let directory = TestDirectory::new();
    fs::write(directory.0.join("invalid.mote"), [0xff]).unwrap();
    let resolver = FileAssetResolver::new(&directory.0).unwrap();
    let missing = load_source(
        &resolver,
        AssetRequest {
            reference: OsStr::new("missing.mote"),
            source_name: None,
        },
    )
    .unwrap_err();
    assert_eq!(missing.name, "missing.mote");
    assert_eq!(missing.cause.kind(), io::ErrorKind::NotFound);

    let invalid = load_source(
        &resolver,
        AssetRequest {
            reference: OsStr::new("invalid.mote"),
            source_name: None,
        },
    )
    .unwrap_err();
    assert_eq!(invalid.name, directory.0.join("invalid.mote"));
    assert_eq!(invalid.cause.kind(), io::ErrorKind::InvalidData);

    let custom = |_: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "asset://generated/invalid.mote".into(),
            bytes: vec![0xff],
        })
    };
    let invalid = load_source(
        &custom,
        AssetRequest {
            reference: OsStr::new("entry.mote"),
            source_name: None,
        },
    )
    .unwrap_err();
    assert_eq!(invalid.name, "asset://generated/invalid.mote");
    assert_eq!(invalid.cause.kind(), io::ErrorKind::InvalidData);
}

#[test]
fn filesystem_rejects_external_names_and_virtual_source_bases() {
    let directory = TestDirectory::new();
    let resolver = FileAssetResolver::new(&directory.0).unwrap();
    for (reference, source_name) in [
        ("https://example.invalid/main.mote", None),
        ("asset://bundle/main.mote", None),
        ("file:///main.mote", None),
        ("data:text/plain,hello", None),
        ("shared.mote", Some("asset://bundle/main.mote")),
    ] {
        let error = resolver
            .resolve_asset(AssetRequest {
                reference: OsStr::new(reference),
                source_name: source_name.map(OsStr::new),
            })
            .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::Unsupported, "{reference}");
    }
    for reference in [
        r"C:\mote-missing-input\main.mote",
        "D:/mote-missing-input/main.mote",
    ] {
        let error = resolver
            .resolve_asset(AssetRequest {
                reference: OsStr::new(reference),
                source_name: None,
            })
            .unwrap_err();
        assert_ne!(error.kind(), io::ErrorKind::Unsupported, "{reference}");
    }
}

// Some Unix filesystems require Unicode filenames. Exercise native byte names
// on Linux and verify byte-preserving custom resolution on every Unix platform.
#[cfg(target_os = "linux")]
#[test]
fn filesystem_names_preserve_non_utf8_bytes() {
    use std::os::unix::ffi::OsStringExt;

    let directory = TestDirectory::new();
    let filename = OsString::from_vec(b"source-\xff.mote".to_vec());
    let path = directory.0.join(&filename);
    fs::write(&path, "#paragraph[Unicode: café.]").unwrap();
    let resolver = FileAssetResolver::new(&directory.0).unwrap();
    let source = load_source(
        &resolver,
        AssetRequest {
            reference: &filename,
            source_name: None,
        },
    )
    .unwrap();
    assert_eq!(source.name, path);
    assert!(mote::compile(&source.text).is_ok());
}

#[cfg(unix)]
#[test]
fn custom_names_preserve_non_utf8_bytes_without_lossy_conversion() {
    use std::os::unix::ffi::OsStringExt;

    let filename = OsString::from_vec(b"source-\xff.mote".to_vec());
    let resolver = |request: AssetRequest<'_>| {
        assert_eq!(request.reference, filename);
        Ok(ResolvedAsset {
            name: OsString::new(),
            bytes: b"#paragraph[Named input.]".to_vec(),
        })
    };
    let source = load_source(
        &resolver,
        AssetRequest {
            reference: &filename,
            source_name: Some(OsStr::new("bundle/main.mote")),
        },
    )
    .unwrap();
    assert_eq!(source.name, PathBuf::from("bundle").join(filename));
    assert!(mote::compile(&source.text).is_ok());
}
