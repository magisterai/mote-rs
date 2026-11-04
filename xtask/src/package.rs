//! Office packages as ordered part lists, read without trusting archive encoding.
use std::io::{Cursor, Read};

pub(crate) struct Part {
    pub name: String,
    pub bytes: Vec<u8>,
}

pub(crate) fn parts(bytes: &[u8]) -> Result<Vec<Part>, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| format!("invalid archive: {error}"))?;
    // The archive reader merges entries of the same name, so duplicates are
    // detected on the central directory before any part is read.
    if archive.has_duplicate_names() {
        return Err("duplicate package part".into());
    }
    let mut parts = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(|error| error.to_string())?;
        let name = entry
            .name()
            .map_err(|error| error.to_string())?
            .into_owned();
        let mut bytes = Vec::new();
        entry
            .read_to_end(&mut bytes)
            .map_err(|error| format!("{name}: {error}"))?;
        parts.push(Part { name, bytes });
    }
    Ok(parts)
}
