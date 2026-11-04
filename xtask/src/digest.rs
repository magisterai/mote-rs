use sha2::{Digest, Sha256};
use std::{fs, path::Path};

pub(crate) fn fingerprint(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(digest(&bytes))
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
