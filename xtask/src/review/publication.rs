use super::{ReviewCase, ReviewResult, rendering::ReviewArtifacts};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

pub(super) struct StagingDirectory(pub PathBuf);

impl StagingDirectory {
    pub fn new(root: &Path) -> ReviewResult<Self> {
        static NEXT_IDENTIFIER: AtomicUsize = AtomicUsize::new(0);
        fs::create_dir_all(root)?;
        let path = root.join(format!(
            ".render-{}-{}",
            std::process::id(),
            NEXT_IDENTIFIER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path)?;
        Ok(Self(path))
    }
}

impl Drop for StagingDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn destinations(root: &Path, case: &ReviewCase) -> [PathBuf; 2] {
    ["complete", "concise"].map(|tree| {
        root.join(tree)
            .join(case.application.directory())
            .join(&case.name)
    })
}

pub(super) fn assert_case_closed(root: &Path, case: &ReviewCase) -> ReviewResult<()> {
    for destination in destinations(root, case) {
        if !destination.exists() {
            continue;
        }
        for entry in fs::read_dir(destination)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.starts_with("~$") && (name.ends_with(".docx") || name.ends_with(".pptx")) {
                return Err(format!(
                    "Close the Office file before refreshing this case: {}",
                    entry.path().display()
                )
                .into());
            }
        }
    }
    Ok(())
}

pub(super) fn publish(
    root: &Path,
    case: &ReviewCase,
    artifacts: &ReviewArtifacts,
) -> ReviewResult<()> {
    assert_case_closed(root, case)?;
    let staging = StagingDirectory::new(root)?;
    for (name, bytes) in artifacts.native_files.iter().chain(&artifacts.diagnostics) {
        let path = staging.0.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
    }
    let manifest_name = format!("{}-manifest.txt", case.name);
    fs::write(staging.0.join(&manifest_name), &artifacts.manifest)?;
    assert_case_closed(root, case)?;
    for (index, destination) in destinations(root, case).iter().enumerate() {
        fs::create_dir_all(destination)?;
        for name in artifacts.native_files.keys() {
            let path = destination.join(name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(staging.0.join(name), path)?;
        }
        if index == 0 {
            for name in artifacts.diagnostics.keys() {
                fs::copy(staging.0.join(name), destination.join(name))?;
            }
            fs::copy(
                staging.0.join(&manifest_name),
                destination.join(&manifest_name),
            )?;
        }
        for entry in fs::read_dir(destination)? {
            let entry = entry?;
            let filename = entry.file_name();
            let name = filename.to_string_lossy();
            let native_page = name
                .strip_prefix(&format!("{}-", case.name))
                .and_then(|suffix| suffix.strip_suffix(".png"))
                .is_some_and(|number| {
                    !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
                });
            if native_page && !artifacts.native_files.contains_key(name.as_ref()) {
                fs::remove_file(entry.path())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::review::{catalogue, rendering};
    use std::collections::BTreeMap;

    #[test]
    fn rendered_review_preserves_all_pages_in_both_trees_and_manifest_hashes() {
        let root = StagingDirectory::new(&std::env::temp_dir()).unwrap();
        let cases = catalogue::load().unwrap();
        let case = cases
            .iter()
            .find(|case| case.identifier == "slide-table-alignment")
            .unwrap();
        let artifacts = rendering::generate(&crate::repository(), case).unwrap();
        publish(&root.0, case, &artifacts).unwrap();
        let [complete, concise] = destinations(&root.0, case);
        let manifest = String::from_utf8(artifacts.manifest.clone()).unwrap();
        assert!(manifest.contains("native_png_pages=2\n"));
        assert!(manifest.contains("powerpoint_reference_status=skipped\n"));
        assert_eq!(fs::read_dir(&concise).unwrap().count(), 5);
        assert_eq!(fs::read_dir(&complete).unwrap().count(), 7);
        for (name, bytes) in &artifacts.native_files {
            assert_eq!(fs::read(complete.join(name)).unwrap(), *bytes);
            assert_eq!(fs::read(concise.join(name)).unwrap(), *bytes);
            assert!(manifest.contains(&format!(
                "{name}_sha256={}\n",
                rendering::fingerprint(bytes)
            )));
        }
    }

    #[test]
    fn refresh_protects_open_packages_and_removes_only_obsolete_native_pages() {
        let root = StagingDirectory::new(&std::env::temp_dir()).unwrap();
        let cases = catalogue::load().unwrap();
        let case = cases
            .iter()
            .find(|case| case.identifier == "slide-table-alignment")
            .unwrap();
        let [complete, concise] = destinations(&root.0, case);
        fs::create_dir_all(&complete).unwrap();
        fs::create_dir_all(&concise).unwrap();
        let owner = concise.join("~$28_table_alignment.pptx");
        let current = concise.join("28_table_alignment-1.png");
        let obsolete = concise.join("28_table_alignment-2.png");
        let reference = complete.join("28_table_alignment-powerpoint-2.png");
        fs::write(&owner, b"owner").unwrap();
        fs::write(&current, b"current").unwrap();
        fs::write(&obsolete, b"obsolete").unwrap();
        fs::write(&reference, b"reference").unwrap();
        let artifacts = ReviewArtifacts {
            native_files: BTreeMap::from([("28_table_alignment-1.png".into(), b"new".to_vec())]),
            manifest: b"office_review=unreviewed\n".to_vec(),
            diagnostics: BTreeMap::new(),
        };
        assert!(publish(&root.0, case, &artifacts).is_err());
        assert_eq!(fs::read(&current).unwrap(), b"current");
        assert_eq!(fs::read(&owner).unwrap(), b"owner");
        fs::remove_file(owner).unwrap();
        publish(&root.0, case, &artifacts).unwrap();
        assert_eq!(fs::read(current).unwrap(), b"new");
        assert!(!obsolete.exists());
        assert_eq!(fs::read(reference).unwrap(), b"reference");
    }

    #[test]
    fn published_sources_resolve_images_imports_and_lua_data_without_the_repository() {
        let root = StagingDirectory::new(&std::env::temp_dir()).unwrap();
        let cases = catalogue::load().unwrap();
        for identifier in [
            "word-default-style",
            "word-chart-lua-json",
            "powerpoint-image-transforms",
        ] {
            let case = cases
                .iter()
                .find(|case| case.identifier == identifier)
                .unwrap();
            let artifacts = rendering::generate(&crate::repository(), case).unwrap();
            publish(&root.0, case, &artifacts).unwrap();
            let [_, concise] = destinations(&root.0, case);
            let source_path = concise.join(format!("{}.mote", case.name));
            let source = fs::read_to_string(&source_path).unwrap();
            let resolver = mote::source::FileAssetResolver::new(&concise).unwrap();
            let document = mote::compile_with_options(
                &source,
                mote::evaluation::EvaluationOptions {
                    asset_resolver: Some(&resolver),
                    source_name: Some(source_path.as_os_str()),
                    enable_lua: case.enable_lua,
                    ..Default::default()
                },
            )
            .unwrap();
            let plan = mote::render::plan_with_layout(
                &document,
                &resolver,
                Some(source_path.as_os_str()),
                case.application.layout(),
            )
            .unwrap();
            assert_eq!(plan.output.pages.len(), case.page_count, "{identifier}");
            for (name, bytes) in &artifacts.native_files {
                assert_eq!(
                    fs::read(concise.join(name)).unwrap(),
                    *bytes,
                    "{identifier}/{name}"
                );
            }
        }
    }
}
