use std::{
    ffi::{OsStr, OsString},
    io,
    path::{Component, Path, PathBuf},
};

use super::AssetRequest;
use iri_string::types::{IriReferenceStr, IriStr};

/// URI classification must not mistake an absolute Windows drive path for a scheme.
pub(super) fn is_external_reference(name: &OsStr) -> bool {
    if Path::new(name).is_absolute() || is_windows_drive_path(name) {
        return false;
    }
    name.to_str()
        .is_some_and(|text| IriStr::new(&text.replace(' ', "%20")).is_ok())
}

fn is_windows_drive_path(name: &OsStr) -> bool {
    let Some(text) = name.to_str() else {
        return false;
    };
    let bytes = text.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'/' | b'\\')
}

pub(super) fn logical_asset_name(request: AssetRequest<'_>) -> io::Result<OsString> {
    let reference = Path::new(request.reference);
    if reference.is_absolute() || is_windows_drive_path(request.reference) {
        return Ok(normalize_path(reference).into_os_string());
    }
    if is_external_reference(request.reference) {
        return Ok(request.reference.to_owned());
    }
    let Some(source_name) = request.source_name.filter(|name| !name.is_empty()) else {
        return Ok(normalize_path(reference).into_os_string());
    };
    if is_external_reference(source_name) {
        let reference_text = request.reference.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "URI-relative names must be UTF-8",
            )
        })?;
        let base_text = source_name
            .to_str()
            .expect("classified URI is UTF-8")
            .replace(' ', "%20");
        let reference_text = reference_text.replace(' ', "%20");
        let base = IriStr::new(&base_text)
            .map_err(|cause| io::Error::new(io::ErrorKind::InvalidInput, cause))?;
        let reference = IriReferenceStr::new(&reference_text)
            .map_err(|cause| io::Error::new(io::ErrorKind::InvalidInput, cause))?;
        return resolve_uri_reference(base, reference).map(OsString::from);
    }
    let source_path = Path::new(source_name);
    let base = source_path.parent().unwrap_or(source_path);
    Ok(normalize_path(&base.join(reference)).into_os_string())
}

fn resolve_uri_reference(base: &IriStr, reference: &IriReferenceStr) -> io::Result<String> {
    if base.authority_str().is_none() && !base.path_str().starts_with('/') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "relative references from an opaque URI require a resolver-supplied name",
        ));
    }
    let reference_path = reference.path_str();
    let merged_path = if reference.authority_str().is_some() || reference_path.starts_with('/') {
        reference_path.to_owned()
    } else if reference_path.is_empty() {
        base.path_str().to_owned()
    } else {
        let directory = base
            .path_str()
            .rsplit_once('/')
            .map_or("", |(directory, _)| directory);
        format!("{directory}/{reference_path}")
    };
    let inherits_query = reference.authority_str().is_none()
        && reference_path.is_empty()
        && reference.query_str().is_none();
    let query = if inherits_query {
        base.query_str()
    } else {
        reference.query_str()
    };
    let fragment = if inherits_query {
        reference.fragment_str().or(base.fragment_str())
    } else {
        reference.fragment_str()
    };
    let mut resolved = format!("{}:", base.scheme_str());
    if let Some(authority) = reference.authority_str().or(base.authority_str()) {
        resolved.push_str("//");
        resolved.push_str(authority);
    }
    resolved.push_str(&remove_literal_dot_segments(&merged_path));
    if let Some(query) = query {
        resolved.push('?');
        resolved.push_str(query);
    }
    if let Some(fragment) = fragment {
        resolved.push('#');
        resolved.push_str(fragment);
    }
    IriStr::new(&resolved)
        .map(|name| name.encode_to_uri().to_string())
        .map_err(|cause| io::Error::new(io::ErrorKind::InvalidInput, cause))
}

/// Resolve only literal `.` and `..`. Escaped segments identify different assets;
/// general URI normalization would incorrectly collapse `%2e` and `%2e%2e`.
fn remove_literal_dot_segments(path: &str) -> String {
    let Some(relative_path) = path.strip_prefix('/') else {
        return path.to_owned();
    };
    let mut segments = Vec::new();
    for segment in relative_path.split('/') {
        match segment {
            "." => {}
            ".." => {
                segments.pop();
            }
            _ => segments.push(segment),
        }
    }
    if path.ends_with("/.") || path.ends_with("/..") {
        segments.push("");
    }
    format!("/{}", segments.join("/"))
}

/// Lexical normalization preserves names without requiring a file to exist.
/// It does not resolve symlinks or require filesystem canonicalization.
pub(super) fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(
                    normalized.components().next_back(),
                    Some(Component::Normal(_))
                ) {
                    normalized.pop();
                } else if !normalized.has_root() {
                    normalized.push(component);
                }
            }
            _ => normalized.push(component),
        }
    }
    if normalized.as_os_str().is_empty() {
        normalized.push(".");
    }
    normalized
}
