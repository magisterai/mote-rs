use mote::source::{AssetRequest, AssetResolver, CachingAssetResolver, ResolvedAsset};
use std::{cell::Cell, ffi::OsStr, io};

#[test]
fn requests_are_snapshotted_by_reference_and_referring_source() {
    let calls = Cell::new(0);
    let resolver = |request: AssetRequest<'_>| {
        calls.set(calls.get() + 1);
        Ok(ResolvedAsset {
            name: Default::default(),
            bytes: request
                .source_name
                .unwrap()
                .to_string_lossy()
                .as_bytes()
                .to_vec(),
        })
    };
    let cached = CachingAssetResolver::new(&resolver);
    let first = AssetRequest {
        reference: OsStr::new("font.ttf"),
        source_name: Some(OsStr::new("memory://one/main.mote")),
    };
    let second = AssetRequest {
        reference: first.reference,
        source_name: Some(OsStr::new("memory://two/main.mote")),
    };
    let mut original = cached.resolve_asset(first).unwrap();
    let bytes = original.bytes.clone();
    original.bytes.clear();
    assert_eq!(cached.resolve_asset(first).unwrap().bytes, bytes);
    assert_eq!(
        cached.resolve_asset(first).unwrap().name,
        "memory://one/font.ttf"
    );
    assert_ne!(cached.resolve_asset(second).unwrap().bytes, bytes);
    assert_eq!(calls.get(), 2);
}

#[test]
fn failed_requests_are_not_retried_and_keep_their_error_kind() {
    let calls = Cell::new(0);
    let resolver = |_: AssetRequest<'_>| {
        calls.set(calls.get() + 1);
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"))
    };
    let cached = CachingAssetResolver::new(&resolver);
    let request = AssetRequest {
        reference: OsStr::new("asset"),
        source_name: None,
    };
    for _ in 0..3 {
        let failure = cached.resolve_asset(request).unwrap_err();
        assert_eq!(failure.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(failure.to_string(), "denied");
    }
    assert_eq!(calls.get(), 1);
}

#[test]
fn identical_named_bytes_are_shared_but_conflicting_bytes_are_rejected() {
    let resolver = |request: AssetRequest<'_>| {
        Ok(ResolvedAsset {
            name: "shared".into(),
            bytes: if request.reference == "changed" {
                vec![2]
            } else {
                vec![1]
            },
        })
    };
    let cached = CachingAssetResolver::new(&resolver);
    for reference in ["first", "same"] {
        assert_eq!(
            cached
                .resolve_asset(AssetRequest {
                    reference: OsStr::new(reference),
                    source_name: None
                })
                .unwrap()
                .bytes,
            [1]
        );
    }
    let request = AssetRequest {
        reference: OsStr::new("changed"),
        source_name: None,
    };
    let failure = cached.resolve_asset(request).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::InvalidData);
    assert!(failure.to_string().contains("conflicting bytes"));
}
