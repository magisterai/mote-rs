use std::{
    fs, io,
    path::{Path, PathBuf},
};

use super::{AssetRequest, AssetResolver, ResolvedAsset, names};

/// Reads local files relative to a referring source or a captured working directory.
/// URI references require a custom resolver; this resolver never fetches them.
#[derive(Debug, Clone)]
pub struct FileAssetResolver {
    working_directory: PathBuf,
}

impl FileAssetResolver {
    /// Captures an absolute working directory without resolving symlinks.
    /// A relative directory is anchored to the current process directory now.
    pub fn new(working_directory: impl AsRef<Path>) -> io::Result<Self> {
        let working_directory = working_directory.as_ref();
        let absolute_directory = if working_directory.is_absolute() {
            working_directory.to_owned()
        } else {
            std::env::current_dir()?.join(working_directory)
        };
        Ok(Self {
            working_directory: names::normalize_path(&absolute_directory),
        })
    }

    /// Uses the current process directory as the root lookup directory.
    pub fn from_current_directory() -> io::Result<Self> {
        Self::new(std::env::current_dir()?)
    }
}

impl AssetResolver for FileAssetResolver {
    fn resolve_asset(&self, request: AssetRequest<'_>) -> io::Result<ResolvedAsset> {
        let reference = Path::new(request.reference);
        if names::is_external_reference(request.reference)
            || (!reference.is_absolute()
                && request
                    .source_name
                    .is_some_and(names::is_external_reference))
        {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "external asset references require a custom resolver",
            ));
        }
        let path = if reference.is_absolute() {
            reference.to_owned()
        } else {
            let base = match request.source_name.filter(|name| !name.is_empty()) {
                Some(source_name) => {
                    let source_path = self.working_directory.join(source_name);
                    source_path.parent().unwrap_or(&source_path).to_owned()
                }
                None => self.working_directory.clone(),
            };
            base.join(reference)
        };
        let path = names::normalize_path(&path);
        let bytes = fs::read(&path)?;
        Ok(ResolvedAsset {
            name: path.into_os_string(),
            bytes,
        })
    }
}
