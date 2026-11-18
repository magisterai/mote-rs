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
    let resolver = FileAssetResolver::new(&directory.0).unwrap();
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

/// A directory tree with a root for a confined resolver and a file beside the
/// root that no source compiled within the root may read.
struct ConfinedTree {
    directory: TestDirectory,
}

impl ConfinedTree {
    fn new() -> Self {
        let directory = TestDirectory::new();
        fs::create_dir_all(directory.0.join("root/reports")).unwrap();
        fs::write(directory.0.join("root/inside.txt"), "inside").unwrap();
        fs::write(directory.0.join("root/reports/main.mote"), "Source.").unwrap();
        fs::write(directory.0.join("outside.txt"), "outside").unwrap();
        Self { directory }
    }

    fn root(&self) -> PathBuf {
        self.directory.0.join("root")
    }

    fn outside(&self) -> PathBuf {
        self.directory.0.join("outside.txt")
    }
}

fn resolve_from(
    resolver: &FileAssetResolver,
    reference: impl AsRef<OsStr>,
    source_name: Option<&str>,
) -> io::Result<ResolvedAsset> {
    resolver.resolve_asset(AssetRequest {
        reference: reference.as_ref(),
        source_name: source_name.map(OsStr::new),
    })
}

fn assert_refused_as_outside_the_root(result: io::Result<ResolvedAsset>) {
    let error = result.unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    assert!(
        error.to_string().contains("outside the asset root"),
        "{error}"
    );
}

#[test]
fn a_confined_resolver_refuses_a_path_that_leaves_its_root() {
    let tree = ConfinedTree::new();
    let resolver = FileAssetResolver::new(tree.root()).unwrap();
    assert_eq!(
        resolve_from(&resolver, "inside.txt", None).unwrap().bytes,
        b"inside"
    );
    assert_eq!(
        resolve_from(&resolver, "../inside.txt", Some("reports/main.mote"))
            .unwrap()
            .bytes,
        b"inside"
    );
    for (reference, source_name) in [
        ("../outside.txt", None),
        ("reports/../../outside.txt", None),
        ("../../outside.txt", Some("reports/main.mote")),
        ("outside.txt", Some("../main.mote")),
        // A file that does not exist is refused alike, so that the error
        // discloses nothing of what lies outside the root.
        ("../absent.txt", None),
    ] {
        assert_refused_as_outside_the_root(resolve_from(&resolver, reference, source_name));
    }
}

#[test]
fn a_confined_resolver_accepts_an_absolute_path_only_within_its_root() {
    let tree = ConfinedTree::new();
    let resolver = FileAssetResolver::new(tree.root()).unwrap();
    assert_eq!(
        resolve_from(&resolver, tree.root().join("inside.txt"), None)
            .unwrap()
            .bytes,
        b"inside"
    );
    assert_refused_as_outside_the_root(resolve_from(&resolver, tree.outside(), None));
    assert_refused_as_outside_the_root(resolve_from(
        &resolver,
        tree.outside(),
        Some("reports/main.mote"),
    ));
    let absolute_source = tree.outside().with_file_name("main.mote");
    assert_refused_as_outside_the_root(resolver.resolve_asset(AssetRequest {
        reference: OsStr::new("outside.txt"),
        source_name: Some(absolute_source.as_os_str()),
    }));
}

#[cfg(unix)]
#[test]
fn a_confined_resolver_refuses_a_symbolic_link_that_leaves_its_root() {
    use std::os::unix::fs::symlink;

    let tree = ConfinedTree::new();
    symlink(tree.outside(), tree.root().join("escaping-file.txt")).unwrap();
    symlink(&tree.directory.0, tree.root().join("escaping-directory")).unwrap();
    symlink(
        tree.root().join("inside.txt"),
        tree.root().join("alias.txt"),
    )
    .unwrap();
    let resolver = FileAssetResolver::new(tree.root()).unwrap();
    assert_refused_as_outside_the_root(resolve_from(&resolver, "escaping-file.txt", None));
    assert_refused_as_outside_the_root(resolve_from(
        &resolver,
        "escaping-directory/outside.txt",
        None,
    ));
    assert_eq!(
        resolve_from(&resolver, "alias.txt", None).unwrap().bytes,
        b"inside"
    );

    // A root that is itself reached through a symbolic link confines as the
    // directory it names.
    symlink(tree.root(), tree.directory.0.join("linked-root")).unwrap();
    let linked = FileAssetResolver::new(tree.directory.0.join("linked-root")).unwrap();
    assert_eq!(
        resolve_from(&linked, "inside.txt", None).unwrap().bytes,
        b"inside"
    );
    assert_refused_as_outside_the_root(resolve_from(&linked, "escaping-file.txt", None));
}

#[test]
fn a_confined_resolver_refuses_a_file_larger_than_its_limit() {
    let tree = ConfinedTree::new();
    fs::write(tree.root().join("eight.bin"), [0; 8]).unwrap();
    fs::write(tree.root().join("nine.bin"), [0; 9]).unwrap();
    let resolver = FileAssetResolver::new(tree.root())
        .unwrap()
        .with_maximum_size(Some(8));
    assert_eq!(
        resolve_from(&resolver, "eight.bin", None).unwrap().bytes,
        [0; 8]
    );
    let error = resolve_from(&resolver, "nine.bin", None).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::FileTooLarge);
    assert!(
        error
            .to_string()
            .contains("larger than the limit of 8 bytes"),
        "{error}"
    );
    assert_eq!(FileAssetResolver::DEFAULT_MAXIMUM_SIZE, 64 * 1024 * 1024);
    let unlimited = FileAssetResolver::new(tree.root())
        .unwrap()
        .with_maximum_size(None);
    assert_eq!(
        resolve_from(&unlimited, "nine.bin", None).unwrap().bytes,
        [0; 9]
    );
}

#[test]
fn a_confined_resolver_reads_regular_files_alone() {
    let tree = ConfinedTree::new();
    let resolver = FileAssetResolver::new(tree.root()).unwrap();
    let error = resolve_from(&resolver, "reports", None).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("regular file"), "{error}");
}

#[cfg(unix)]
#[test]
fn a_confined_resolver_refuses_a_named_pipe_without_blocking() {
    let tree = ConfinedTree::new();
    let pipe = tree.root().join("pipe");
    let created = std::process::Command::new("mkfifo")
        .arg(&pipe)
        .status()
        .unwrap();
    assert!(created.success());
    let resolver = FileAssetResolver::new(tree.root()).unwrap();
    // To open a pipe without a writer blocks. The read runs on a second
    // thread, so that a resolver which opens the pipe fails the test and does
    // not hang it.
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = sender.send(resolve_from(&resolver, "pipe", None));
    });
    let error = receiver
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("the resolver blocked on a named pipe")
        .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("regular file"), "{error}");
}

#[test]
fn an_unconfined_resolver_reads_wherever_a_reference_leads() {
    let tree = ConfinedTree::new();
    fs::write(tree.root().join("nine.bin"), [0; 9]).unwrap();
    let resolver = FileAssetResolver::unconfined(tree.root()).unwrap();
    for (reference, source_name) in [
        (OsString::from("../outside.txt"), None),
        (
            OsString::from("../../outside.txt"),
            Some("reports/main.mote"),
        ),
        (tree.outside().into_os_string(), None),
    ] {
        assert_eq!(
            resolve_from(&resolver, &reference, source_name)
                .unwrap()
                .bytes,
            b"outside"
        );
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(tree.outside(), tree.root().join("escaping-file.txt")).unwrap();
        assert_eq!(
            resolve_from(&resolver, "escaping-file.txt", None)
                .unwrap()
                .bytes,
            b"outside"
        );
    }
    assert_eq!(
        resolve_from(&resolver, "nine.bin", None).unwrap().bytes,
        [0; 9]
    );
    let limited = resolver.with_maximum_size(Some(8));
    assert_eq!(
        resolve_from(&limited, "nine.bin", None).unwrap_err().kind(),
        io::ErrorKind::FileTooLarge
    );
}

#[test]
fn a_source_compiled_with_a_confined_resolver_cannot_read_beside_the_root() {
    let tree = ConfinedTree::new();
    fs::write(tree.directory.0.join("secret.json"), "{\"key\": 1}").unwrap();
    fs::write(tree.root().join("data.json"), "{\"key\": 2}").unwrap();
    let resolver = FileAssetResolver::new(tree.root()).unwrap();
    let compile = |source: &str| {
        mote::compile_with_assets(source, &resolver, Some(OsStr::new("reports/main.mote")))
    };
    assert!(compile("#let(data = json(\"../data.json\"))\n#data.key").is_ok());
    let secret = tree.directory.0.join("secret.json");
    for source in [
        "#let(data = json(\"../../secret.json\"))".to_owned(),
        format!("#let(data = json({:?}))", secret.to_str().unwrap()),
        "#import(\"../../outside.txt\")".to_owned(),
    ] {
        let diagnostic = compile(&source).unwrap_err();
        assert!(
            diagnostic.message.contains("outside the asset root"),
            "{}",
            diagnostic.message
        );
    }
    let document = compile(&format!("![Secret]({})", secret.to_str().unwrap())).unwrap();
    let package =
        mote::docx::emit_with_assets(&document, &resolver, Some(OsStr::new("reports/main.mote")))
            .unwrap();
    assert!(
        package
            .warnings
            .iter()
            .any(|warning| warning.contains("outside the asset root")),
        "{:?}",
        package.warnings
    );
}
