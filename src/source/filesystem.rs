use std::{
    ffi::OsStr,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use super::{AssetRequest, AssetResolver, ResolvedAsset, names};

/// Reads local files relative to a referring source or a captured root
/// directory. URI references require a custom resolver; this resolver never
/// fetches them.
///
/// A resolver made by [`FileAssetResolver::new`] is confined: the root
/// directory bounds what a source can read. A reference is refused if its
/// path, after symbolic links are resolved, lies outside the root; an absolute
/// reference, a `..` segment, and a symbolic link are therefore accepted only
/// where they stay within the root. A confined resolver reads regular files
/// alone, so that a device or a named pipe can neither block a compilation nor
/// supply unbounded input, and refuses a file larger than its size limit.
///
/// A resolver made by [`FileAssetResolver::unconfined`] reads whatever path the
/// process may read. The constructor is meant for a trusted local tool, such as
/// the command line, whose user chooses the source.
#[derive(Debug, Clone)]
pub struct FileAssetResolver {
    working_directory: PathBuf,
    confined: bool,
    maximum_size: Option<u64>,
}

impl FileAssetResolver {
    /// The size limit of a confined resolver, in bytes. No image, font, or
    /// data file of a document approaches 64 MiB, and a file of that size is
    /// held in memory once without strain.
    pub const DEFAULT_MAXIMUM_SIZE: u64 = 64 * 1024 * 1024;

    /// Makes a confined resolver whose root is the given directory. A relative
    /// directory is anchored to the current process directory now.
    pub fn new(root: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            working_directory: absolute_directory(root.as_ref())?,
            confined: true,
            maximum_size: Some(Self::DEFAULT_MAXIMUM_SIZE),
        })
    }

    /// Makes a resolver that reads any path the process may read, of any kind
    /// and size. A relative reference is resolved against the given directory;
    /// an absolute reference and a `..` segment are followed wherever they
    /// lead. A caller that compiles untrusted source uses
    /// [`FileAssetResolver::new`] instead.
    pub fn unconfined(working_directory: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            working_directory: absolute_directory(working_directory.as_ref())?,
            confined: false,
            maximum_size: None,
        })
    }

    /// Makes a confined resolver whose root is the current process directory.
    pub fn from_current_directory() -> io::Result<Self> {
        Self::new(std::env::current_dir()?)
    }

    /// Replaces the size limit. None removes it.
    pub fn with_maximum_size(mut self, maximum_size: Option<u64>) -> Self {
        self.maximum_size = maximum_size;
        self
    }

    /// The path to read for a reference that a confined resolver accepts: the
    /// lexical path with every symbolic link resolved, inside the root.
    fn confined_path(&self, path: &Path, reference: &OsStr) -> io::Result<PathBuf> {
        let outside_root = || {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("asset reference {reference:?} resolves outside the asset root"),
            )
        };
        let root = fs::canonicalize(&self.working_directory).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("the asset root cannot be resolved: {error}"),
            )
        })?;
        // A path that lies outside the root as written is refused before the
        // filesystem is consulted, so that a source cannot learn from the
        // error whether a file outside the root exists.
        if !path.starts_with(&self.working_directory) && !path.starts_with(&root) {
            return Err(outside_root());
        }
        let resolved = fs::canonicalize(path)?;
        if !resolved.starts_with(&root) {
            return Err(outside_root());
        }
        Ok(resolved)
    }

    fn read_confined(&self, path: &Path, reference: &OsStr) -> io::Result<Vec<u8>> {
        let not_regular = || {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("asset reference {reference:?} does not name a regular file"),
            )
        };
        let resolved = self.confined_path(path, reference)?;
        // The kind is examined before the file is opened: to open a named pipe
        // would block until a writer appears.
        if !fs::metadata(&resolved)?.is_file() {
            return Err(not_regular());
        }
        let file = fs::File::open(&resolved)?;
        if !file.metadata()?.is_file() {
            return Err(not_regular());
        }
        self.read_within_limit(file, reference)
    }

    fn read_within_limit(&self, mut file: fs::File, reference: &OsStr) -> io::Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let Some(maximum_size) = self.maximum_size else {
            file.read_to_end(&mut bytes)?;
            return Ok(bytes);
        };
        let too_large = || {
            io::Error::new(
                io::ErrorKind::FileTooLarge,
                format!(
                    "asset reference {reference:?} names a file larger than the limit of {maximum_size} bytes"
                ),
            )
        };
        if file.metadata()?.len() > maximum_size {
            return Err(too_large());
        }
        // A file may grow after its size is read, and a file that is not
        // regular reports no size; one byte beyond the limit shows either.
        file.take(maximum_size.saturating_add(1))
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > maximum_size {
            return Err(too_large());
        }
        Ok(bytes)
    }
}

fn absolute_directory(directory: &Path) -> io::Result<PathBuf> {
    let absolute = if directory.is_absolute() {
        directory.to_owned()
    } else {
        std::env::current_dir()?.join(directory)
    };
    Ok(names::normalize_path(&absolute))
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
        let bytes = if self.confined {
            self.read_confined(&path, request.reference)?
        } else if self.maximum_size.is_some() {
            self.read_within_limit(fs::File::open(&path)?, request.reference)?
        } else {
            fs::read(&path)?
        };
        Ok(ResolvedAsset {
            name: path.into_os_string(),
            bytes,
        })
    }
}
