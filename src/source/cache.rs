use super::{AssetRequest, AssetResolver, ResolvedAsset, resolve_asset};
use std::{collections::BTreeMap, ffi::OsString, io, sync::Mutex};

type RequestKey = (Option<OsString>, OsString);

#[derive(Clone)]
struct CachedFailure {
    kind: io::ErrorKind,
    message: String,
}

#[derive(Default)]
struct Assets {
    requests: BTreeMap<RequestKey, Result<ResolvedAsset, CachedFailure>>,
    names: BTreeMap<OsString, Vec<u8>>,
}

/// Snapshots requests and enforces one immutable byte sequence per logical name.
/// Create one wrapper per compilation; errors are cached as well as successes.
pub struct CachingAssetResolver<'resolver> {
    resolver: &'resolver dyn AssetResolver,
    assets: Mutex<Assets>,
}

impl<'resolver> CachingAssetResolver<'resolver> {
    pub fn new(resolver: &'resolver dyn AssetResolver) -> Self {
        Self {
            resolver,
            assets: Mutex::new(Assets::default()),
        }
    }
}

impl AssetResolver for CachingAssetResolver<'_> {
    fn resolve_asset(&self, request: AssetRequest<'_>) -> io::Result<ResolvedAsset> {
        let key = (
            request.source_name.map(ToOwned::to_owned),
            request.reference.to_owned(),
        );
        let mut assets = self
            .assets
            .lock()
            .map_err(|_| io::Error::other("asset cache lock was poisoned"))?;
        if let Some(result) = assets.requests.get(&key) {
            return result
                .clone()
                .map_err(|failure| io::Error::new(failure.kind, failure.message));
        }
        let result = resolve_asset(self.resolver, request).and_then(|asset| {
            if let Some(bytes) = assets.names.get(&asset.name) {
                if bytes != &asset.bytes {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "asset resolver returned conflicting bytes for logical name {:?}",
                            asset.name
                        ),
                    ));
                }
            } else {
                assets.names.insert(asset.name.clone(), asset.bytes.clone());
            }
            Ok(asset)
        });
        assets.requests.insert(
            key,
            result
                .as_ref()
                .map(Clone::clone)
                .map_err(|failure| CachedFailure {
                    kind: failure.kind(),
                    message: failure.to_string(),
                }),
        );
        result
    }
}
