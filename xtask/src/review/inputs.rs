//! Snapshot source dependencies so review sources remain runnable after publication.
use super::{ReviewResult, rendering::fingerprint};
use mote::source::{AssetRequest, AssetResolver, FileAssetResolver, ResolvedAsset};
use serde_json::{Value, json};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};

pub(super) struct ReviewInputs {
    files: FileAssetResolver,
    snapshots: RefCell<BTreeMap<PathBuf, Vec<u8>>>,
}

impl ReviewInputs {
    pub fn new(repository: &Path) -> io::Result<Self> {
        Ok(Self {
            files: FileAssetResolver::new(repository)?,
            snapshots: RefCell::default(),
        })
    }

    pub fn collect(
        &self,
        case_root: &Path,
        files: &mut BTreeMap<String, Vec<u8>>,
    ) -> ReviewResult<Value> {
        let mut inputs = Vec::new();
        for (path, bytes) in self.snapshots.borrow().iter() {
            if fs::read(path)? != *bytes {
                return Err(format!("input changed during rendering: {}", path.display()).into());
            }
            let relative = path.strip_prefix(case_root).ok();
            let name = relative.unwrap_or(path).to_string_lossy().into_owned();
            inputs.push(
                json!({"path":name,"sha256":fingerprint(bytes),"bundled":relative.is_some()}),
            );
            if relative.is_some() {
                files.insert(name, bytes.clone());
            }
        }
        Ok(Value::Array(inputs))
    }
}

impl AssetResolver for ReviewInputs {
    fn resolve_asset(&self, request: AssetRequest<'_>) -> io::Result<ResolvedAsset> {
        let asset = self.files.resolve_asset(request)?;
        let path = Path::new(&asset.name).canonicalize()?;
        let mut snapshots = self.snapshots.borrow_mut();
        if let Some(previous) = snapshots.get(&path) {
            if previous != &asset.bytes {
                return Err(io::Error::other(format!(
                    "input changed during rendering: {}",
                    path.display()
                )));
            }
        } else {
            snapshots.insert(path, asset.bytes.clone());
        }
        Ok(asset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::review::publication::StagingDirectory;

    #[test]
    fn dependencies_are_bundled_and_mutations_are_rejected() {
        let directory = StagingDirectory::new(&std::env::temp_dir()).unwrap();
        let root = directory.0.canonicalize().unwrap();
        fs::create_dir(root.join("data")).unwrap();
        fs::write(root.join("data/values.json"), b"[1,2]").unwrap();
        let inputs = ReviewInputs::new(&root).unwrap();
        inputs
            .resolve_asset(AssetRequest {
                reference: "data/values.json".as_ref(),
                source_name: None,
            })
            .unwrap();
        let mut files = BTreeMap::new();
        let manifest = inputs.collect(&root, &mut files).unwrap();
        assert_eq!(files["data/values.json"], b"[1,2]");
        assert_eq!(manifest[0]["bundled"], true);
        fs::write(root.join("data/values.json"), b"[3]").unwrap();
        assert!(inputs.collect(&root, &mut files).is_err());
    }
}
