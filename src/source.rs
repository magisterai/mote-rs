//! Source loading and asset resolution, independent of parsing and evaluation.

use std::{
    ffi::{OsStr, OsString},
    fmt, io,
};

mod cache;
mod filesystem;
mod names;

pub use cache::CachingAssetResolver;
pub use filesystem::FileAssetResolver;

/// An authored reference and the logical source containing it.
///
/// A root request has no `source_name`. Names need not exist on the filesystem;
/// native OS strings also preserve filesystem names that are not UTF-8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssetRequest<'source> {
    pub reference: &'source OsStr,
    pub source_name: Option<&'source OsStr>,
}

/// Owned input bytes and their stable logical identity.
///
/// A resolver-supplied name is authoritative and is preserved exactly. An empty
/// name asks [`resolve_asset`] to derive one from the request. Resolvers should
/// return the same bytes for a name throughout one compilation; these functions
/// do not cache requests or enforce consistency across separate calls yet.
/// Wrap a resolver in [`CachingAssetResolver`] when a compilation needs snapshots
/// and consistency checks across multiple resource requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAsset {
    pub name: OsString,
    pub bytes: Vec<u8>,
}

/// Supplies owned bytes synchronously, including from memory or generated inputs.
///
/// Errors are authoritative: callers never retry through the filesystem. A
/// resolver that performs blocking work is responsible for its own deadlines.
pub trait AssetResolver {
    fn resolve_asset(&self, request: AssetRequest<'_>) -> io::Result<ResolvedAsset>;
}

impl<Resolver> AssetResolver for Resolver
where
    Resolver: Fn(AssetRequest<'_>) -> io::Result<ResolvedAsset>,
{
    fn resolve_asset(&self, request: AssetRequest<'_>) -> io::Result<ResolvedAsset> {
        self(request)
    }
}

/// Calls exactly the selected resolver and fills in an omitted logical name.
/// No filesystem access is needed to derive a name for a custom result.
/// Relative references from opaque URI bases require a resolver-supplied name.
pub fn resolve_asset(
    resolver: &dyn AssetResolver,
    request: AssetRequest<'_>,
) -> io::Result<ResolvedAsset> {
    let mut asset = resolver.resolve_asset(request)?;
    if asset.name.is_empty() {
        asset.name = names::logical_asset_name(request)?;
    }
    Ok(asset)
}

/// A named UTF-8 input, retained separately from the authored document model.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceFile {
    pub name: OsString,
    pub text: String,
}

/// A loading or decoding failure with the input name available at that stage.
#[derive(Debug)]
pub struct SourceError {
    pub name: OsString,
    pub cause: io::Error,
}

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.name.to_string_lossy(), self.cause)
    }
}

impl std::error::Error for SourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}

/// Loads one source through the selected resolver and validates its UTF-8 text.
/// Pass the returned text and name to [`crate::compile_with_assets`] when source
/// expressions load resources. The name also remains available for diagnostics
/// and source-relative output assets.
pub fn load_source(
    resolver: &dyn AssetResolver,
    request: AssetRequest<'_>,
) -> Result<SourceFile, SourceError> {
    let asset = resolve_asset(resolver, request).map_err(|cause| SourceError {
        name: request.reference.to_owned(),
        cause,
    })?;
    let text = String::from_utf8(asset.bytes).map_err(|cause| SourceError {
        name: asset.name.clone(),
        cause: io::Error::new(io::ErrorKind::InvalidData, cause),
    })?;
    Ok(SourceFile {
        name: asset.name,
        text,
    })
}
