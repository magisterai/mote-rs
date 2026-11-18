//! The Mote sources that every build carries, importable by name from any source.
use crate::source::SourceFile;

/// The prefix of the logical name of a library source, which no file can bear.
const PREFIX: &str = "mote:";

/// The library: the path that an import names, and the source it gives.
const SOURCES: &[(&str, &str)] = &[
    (
        "components/gantt.mote",
        include_str!("library/components/gantt.mote"),
    ),
    (
        "components/timeline.mote",
        include_str!("library/components/timeline.mote"),
    ),
    #[cfg(test)]
    (
        "components/sample.mote",
        "#import(\"shared\")\n#let(sample = upper(shared))",
    ),
    #[cfg(test)]
    ("components/shared.mote", "#let(shared = \"carried\")"),
];

pub(crate) fn source(path: &str) -> Option<SourceFile> {
    SOURCES
        .iter()
        .find(|(name, _)| *name == path)
        .map(|(name, text)| SourceFile {
            name: format!("{PREFIX}{name}").into(),
            text: (*text).to_owned(),
        })
}

/// Returns the directory of a library source within the library, with its
/// trailing separator, or nothing for a source from elsewhere.
pub(crate) fn directory(name: &std::ffi::OsStr) -> Option<&str> {
    let path = name.to_str()?.strip_prefix(PREFIX)?;
    Some(path.rfind('/').map_or("", |end| &path[..=end]))
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_library_source_is_imported_by_name_and_imports_its_neighbour() {
        for path in ["components/sample", "components/sample.mote"] {
            let document =
                crate::compile(&format!("#import({path:?})\n#paragraph[#sample]")).unwrap();
            assert_eq!(document.content[0].to_text(), "CARRIED", "{path}");
        }
        let error = crate::compile("#import(\"components/absent\")").unwrap_err();
        assert!(
            error.message.contains("asset resolver"),
            "{}",
            error.message
        );
    }
}
