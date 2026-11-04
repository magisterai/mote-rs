//! Schema validation of package parts against the retained ECMA-376 transitional
//! schemas. Word and PowerPoint may silently omit a part that violates its schema,
//! so a package can render natively yet lose a chart in Office.
use crate::{
    Outcome,
    arguments::{Arguments, unknown},
    package, repository,
};
use roxmltree::{Document, Node};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    sync::OnceLock,
    thread,
};

const HELP: &str = "Usage: cargo xtask validate PACKAGE...

Validate every XML part of the named DOCX and PPTX packages, and of their
embedded workbooks, against the retained ECMA-376 transitional schemas with
xmllint. Markup compatibility is applied first, as Office applies it.
";

const SCHEMA_DIRECTORY: &str = "docs/reference/ooxml/ecma-376/part-4/xsd-transitional";
const SCHEMA_FOR_NAMESPACE: [(&str, &str); 5] = [
    (
        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
        "wml.xsd",
    ),
    (
        "http://schemas.openxmlformats.org/presentationml/2006/main",
        "pml.xsd",
    ),
    (
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        "sml.xsd",
    ),
    (
        "http://schemas.openxmlformats.org/drawingml/2006/main",
        "dml-main.xsd",
    ),
    (
        "http://schemas.openxmlformats.org/drawingml/2006/chart",
        "dml-chart.xsd",
    ),
];
/// The retained schemas import the XML namespace without a location, and xmllint
/// then fails to compile them and reports no validity errors at all. Each schema
/// is therefore loaded through a wrapper that supplies the namespace.
const XML_SCHEMA: &str = "xtask/schemas/xml.xsd";
const WRAPPER_DIRECTORY: &str = "target/xtask/schemas";
const COMPATIBILITY: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
const XML: &str = "http://www.w3.org/XML/1998/namespace";

pub(crate) struct Failure {
    pub location: String,
    pub error: String,
}

pub(crate) fn run(arguments: Arguments) -> Outcome {
    let mut packages = Vec::new();
    for argument in arguments {
        match argument.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(ExitCode::SUCCESS);
            }
            option if option.starts_with('-') => return Err(unknown(option)),
            _ => packages.push(PathBuf::from(argument)),
        }
    }
    if packages.is_empty() {
        return Err("name at least one package; use --help".into());
    }
    let reports = thread::scope(|scope| {
        let workers: Vec<_> = packages
            .iter()
            .map(|path| scope.spawn(move || validate_file(path)))
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().expect("validation does not panic"))
            .collect::<Result<Vec<_>, _>>()
    })?;
    let failures: Vec<_> = reports.into_iter().flatten().collect();
    for failure in &failures {
        println!("{}: {}", failure.location, failure.error);
    }
    let distinct: BTreeSet<_> = failures
        .iter()
        .map(|failure| {
            failure
                .error
                .split("Expected is")
                .next()
                .unwrap_or_default()
        })
        .collect();
    println!(
        "{} packages, {} schema errors, {} distinct",
        packages.len(),
        failures.len(),
        distinct.len()
    );
    Ok(if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn validate_file(path: &Path) -> Result<Vec<Failure>, String> {
    let bytes = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    validate_package(&bytes, &path.display().to_string())
}

pub(crate) fn validate_package(bytes: &[u8], origin: &str) -> Result<Vec<Failure>, String> {
    let mut failures = Vec::new();
    for part in package::parts(bytes).map_err(|error| format!("{origin}: {error}"))? {
        let location = format!("{origin}:{}", part.name);
        if part.name.ends_with(".xml") {
            let errors =
                validate_part(&part.bytes).map_err(|error| format!("{location}: {error}"))?;
            failures.extend(errors.into_iter().map(|error| Failure {
                location: location.clone(),
                error,
            }));
        } else if part.name.ends_with(".xlsx") {
            failures.extend(validate_package(&part.bytes, &location)?);
        }
    }
    Ok(failures)
}

fn validate_part(bytes: &[u8]) -> Result<Vec<String>, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let document = Document::parse(text).map_err(|error| error.to_string())?;
    let root = document.root_element();
    let Some(index) = SCHEMA_FOR_NAMESPACE
        .iter()
        .position(|(namespace, _)| root.tag_name().namespace() == Some(namespace))
    else {
        return Ok(Vec::new());
    };
    let schema = &wrappers()?[index];
    let markup = apply_compatibility(&document);
    let mut xmllint = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(schema)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("xmllint is required for schema validation: {error}"))?;
    xmllint
        .stdin
        .take()
        .expect("standard input is piped")
        .write_all(markup.as_bytes())
        .map_err(|error| error.to_string())?;
    let output = xmllint
        .wait_with_output()
        .map_err(|error| error.to_string())?;
    let diagnostics = String::from_utf8_lossy(&output.stderr);
    if diagnostics.contains("failed to compile") {
        return Err(format!(
            "{} does not compile:\n{diagnostics}",
            schema.display()
        ));
    }
    Ok(diagnostics
        .lines()
        .filter(|line| line.contains("validity error"))
        .map(|line| match line.find(": element ") {
            Some(start) => line[start + 2..].to_owned(),
            None => line.to_owned(),
        })
        .collect())
}

fn wrappers() -> Result<&'static [PathBuf], String> {
    static WRAPPERS: OnceLock<Result<Vec<PathBuf>, String>> = OnceLock::new();
    WRAPPERS
        .get_or_init(|| {
            let repository = repository();
            let directory = repository.join(WRAPPER_DIRECTORY);
            fs::create_dir_all(&directory)
                .map_err(|error| format!("{}: {error}", directory.display()))?;
            SCHEMA_FOR_NAMESPACE
                .iter()
                .map(|(namespace, schema)| {
                    let wrapper = format!(
                        "<xsd:schema xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">\
                         <xsd:import namespace=\"{XML}\" schemaLocation=\"{}\"/>\
                         <xsd:import namespace=\"{namespace}\" schemaLocation=\"{}\"/>\
                         </xsd:schema>\n",
                        escape(&repository.join(XML_SCHEMA).to_string_lossy(), true),
                        escape(
                            &repository
                                .join(SCHEMA_DIRECTORY)
                                .join(schema)
                                .to_string_lossy(),
                            true
                        ),
                    );
                    let path = directory.join(schema);
                    fs::write(&path, wrapper)
                        .map_err(|error| format!("{}: {error}", path.display()))?;
                    Ok(path)
                })
                .collect()
        })
        .as_deref()
        .map_err(Clone::clone)
}

/// Office reads ignorable markup as absent and alternate content as its fallback,
/// so the schemas apply only after the same preprocessing.
fn apply_compatibility<'input>(document: &'input Document<'input>) -> String {
    let mut ignorable = BTreeSet::from([COMPATIBILITY]);
    for element in document.descendants().filter(Node::is_element) {
        if let Some(prefixes) = element.attribute((COMPATIBILITY, "Ignorable")) {
            ignorable.extend(
                prefixes
                    .split_whitespace()
                    .filter_map(|prefix| element.lookup_namespace_uri(Some(prefix))),
            );
        }
    }
    let writer = Writer {
        prefixes: prefixes(document),
        ignorable,
    };
    let mut output = String::new();
    writer.element(document.root_element(), true, &mut output);
    output
}

/// Every namespace receives a distinct non-empty prefix, preferring the authored
/// one, so that qualified names survive the removal of ignorable declarations.
fn prefixes<'input>(document: &'input Document<'input>) -> BTreeMap<&'input str, String> {
    let mut prefixes: BTreeMap<&str, String> = BTreeMap::new();
    for element in document.descendants().filter(Node::is_element) {
        for namespace in element.namespaces() {
            let uri = namespace.uri();
            if uri == XML || prefixes.contains_key(uri) {
                continue;
            }
            let taken = |prefix: &str| prefixes.values().any(|existing| existing == prefix);
            let prefix = match namespace.name() {
                Some(prefix) if !taken(prefix) => prefix.to_owned(),
                _ => (0..)
                    .map(|index| format!("ns{index}"))
                    .find(|prefix| !taken(prefix))
                    .expect("an unused prefix exists"),
            };
            prefixes.insert(uri, prefix);
        }
    }
    prefixes
}

struct Writer<'input> {
    prefixes: BTreeMap<&'input str, String>,
    ignorable: BTreeSet<&'input str>,
}

impl Writer<'_> {
    fn name(&self, namespace: Option<&str>, local: &str) -> String {
        match namespace {
            Some(XML) => format!("xml:{local}"),
            Some(uri) => format!("{}:{local}", self.prefixes[uri]),
            None => local.to_owned(),
        }
    }
    fn element(&self, node: Node, root: bool, output: &mut String) {
        let name = self.name(node.tag_name().namespace(), node.tag_name().name());
        output.push('<');
        output.push_str(&name);
        if root {
            for (uri, prefix) in &self.prefixes {
                output.push_str(&format!(" xmlns:{prefix}=\"{}\"", escape(uri, true)));
            }
        }
        for attribute in node.attributes() {
            if attribute
                .namespace()
                .is_some_and(|namespace| self.ignorable.contains(namespace))
            {
                continue;
            }
            let name = self.name(attribute.namespace(), attribute.name());
            output.push_str(&format!(" {name}=\"{}\"", escape(attribute.value(), true)));
        }
        output.push('>');
        self.children(node, output);
        output.push_str(&format!("</{name}>"));
    }
    fn children(&self, node: Node, output: &mut String) {
        for child in node.children() {
            if child.is_text() {
                output.push_str(&escape(child.text().unwrap_or_default(), false));
            } else if !child.is_element() {
                continue;
            } else if child.tag_name().namespace() == Some(COMPATIBILITY)
                && child.tag_name().name() == "AlternateContent"
            {
                let fallback = child.children().find(|option| {
                    option.tag_name().namespace() == Some(COMPATIBILITY)
                        && option.tag_name().name() == "Fallback"
                });
                if let Some(fallback) = fallback {
                    self.children(fallback, output);
                }
            } else if !child
                .tag_name()
                .namespace()
                .is_some_and(|namespace| self.ignorable.contains(namespace))
            {
                self.element(child, false, output);
            }
        }
    }
}

fn escape(text: &str, attribute: bool) -> String {
    let mut output = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' if attribute => output.push_str("&quot;"),
            '\t' if attribute => output.push_str("&#9;"),
            '\n' if attribute => output.push_str("&#10;"),
            '\r' => output.push_str("&#13;"),
            _ => output.push(character),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{apply_compatibility, validate_part};
    use roxmltree::Document;

    const WORD: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const COMPATIBILITY: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";

    #[test]
    fn ignorable_markup_is_removed_and_alternate_content_becomes_its_fallback() {
        let source = format!(
            r#"<w:document xmlns:w="{WORD}" xmlns:mc="{COMPATIBILITY}" xmlns:w14="urn:w14" mc:Ignorable="w14" w14:paraId="1"><mc:AlternateContent><mc:Choice Requires="w14"><w14:new/></mc:Choice><mc:Fallback><w:body/></mc:Fallback></mc:AlternateContent><w14:extra/></w:document>"#
        );
        let markup = apply_compatibility(&Document::parse(&source).unwrap());
        let document = Document::parse(&markup).unwrap();
        let root = document.root_element();
        assert_eq!(root.attributes().count(), 0);
        let children: Vec<_> = root
            .children()
            .map(|child| child.tag_name().name())
            .collect();
        assert_eq!(children, ["body"]);
        assert_eq!(
            root.first_child().unwrap().tag_name().namespace(),
            Some(WORD)
        );
    }

    #[test]
    fn schema_order_violations_are_reported() {
        let valid = format!(r#"<w:document xmlns:w="{WORD}"><w:body><w:p/></w:body></w:document>"#);
        let invalid = format!(
            r#"<w:document xmlns:w="{WORD}"><w:body><w:p><w:r/><w:pPr/></w:p></w:body></w:document>"#
        );
        assert!(validate_part(valid.as_bytes()).unwrap().is_empty());
        let errors = validate_part(invalid.as_bytes()).unwrap();
        assert!(errors[0].starts_with("element "), "{errors:?}");
    }
}
