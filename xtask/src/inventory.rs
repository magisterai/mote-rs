//! The specification-derived attribute inventory, checked or refreshed from local sources.
mod schema;
mod text;

use crate::{Outcome, arguments::Arguments, json::Json, repository};
use schema::{Field, Projection, Schemas, Source};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, VecDeque},
    fs,
    path::{Component, Path, PathBuf},
    process::ExitCode,
    rc::Rc,
};

const HELP: &str = "Usage: cargo xtask inventory [--write]

Check the generated attribute inventory in docs/checklists/attributes against
the retained local schemas, scope.json and support.json.
--write refreshes the schema-derived Markdown after a change to either record.
No sources are retrieved from the network.
";

pub(crate) fn run(arguments: Arguments) -> Outcome {
    let mut write = false;
    for argument in arguments {
        match argument.as_str() {
            "--write" => write = true,
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(ExitCode::SUCCESS);
            }
            _ => return Err(crate::arguments::unknown(&argument)),
        }
    }
    let directory = inventory_directory();
    let configuration = Json::read(&directory.join("scope.json"))?;
    let support = Json::read(&directory.join("support.json"))?;
    let schemas = Schemas::load(&configuration)?;
    let inventory = Inventory::build(&configuration, &support, &schemas)?;
    let mut mismatches = Vec::new();
    for (filename, content) in &inventory.documents {
        let path = directory.join(filename);
        if write {
            fs::write(&path, content).map_err(|error| format!("{}: {error}", path.display()))?;
        } else if fs::read_to_string(&path).ok().as_deref() != Some(content.as_str()) {
            mismatches.push(filename.as_str());
        }
    }
    if !mismatches.is_empty() {
        return Err(format!(
            "Inventory differs from its sources or support records: {}",
            mismatches.join(", ")
        ));
    }
    println!(
        "{} property types, {} fields, {} value types; {} inventory files {}",
        inventory.types,
        inventory.fields,
        inventory.value_types,
        inventory.documents.len(),
        if write { "written." } else { "verified." }
    );
    Ok(ExitCode::SUCCESS)
}

fn inventory_directory() -> PathBuf {
    repository().join("docs/checklists/attributes")
}

fn anchor(identifier: &str) -> String {
    identifier.to_lowercase().replace(':', "-")
}

fn value_file(identifier: &str) -> String {
    let prefix = identifier.split(':').next().unwrap_or_default();
    let family = match prefix {
        "w" => "word",
        "a" | "p" | "wp" => "drawing",
        "c" | "cdr" | "cdr14" | "c14" | "c15" | "cs" | "c16" | "c16ac" | "cx" | "c16r3"
        | "c16r2" => "charts",
        "m" => "math",
        "s" | "r" => "shared",
        _ => "extensions",
    };
    format!("values-{family}.md")
}

fn normalize(path: &Path) -> PathBuf {
    let mut normal = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normal.pop();
            }
            component => normal.push(component),
        }
    }
    normal
}

fn relative_path(target: &Path, base: &Path) -> String {
    let (target, base) = (normalize(target), normalize(base));
    let common = target
        .components()
        .zip(base.components())
        .take_while(|(first, second)| first == second)
        .count();
    let parents = base.components().count() - common;
    let mut relative = PathBuf::new();
    for _ in 0..parents {
        relative.push("..");
    }
    for component in target.components().skip(common) {
        relative.push(component);
    }
    match relative.to_string_lossy() {
        text if text.is_empty() => ".".into(),
        text => text.into_owned(),
    }
}

/// A reused XML name can have different meanings in different containers, so
/// only unambiguous clause links are kept; every type still links its exact schema.
fn clause_descriptions(configuration: &Json) -> Result<HashMap<(String, String), String>, String> {
    let root = repository();
    let directory = inventory_directory();
    let mut candidates: HashMap<(String, String), BTreeSet<String>> = HashMap::new();
    let indexes = configuration
        .get("clause_indexes")
        .and_then(Json::as_object)
        .unwrap_or_default();
    for (prefix, paths) in indexes {
        for index in paths.as_array().unwrap_or_default() {
            let index_path = root.join(index.as_str().ok_or("scope.json: invalid clause index")?);
            let content = fs::read_to_string(&index_path)
                .map_err(|error| format!("{}: {error}", index_path.display()))?;
            let parent = index_path.parent().unwrap_or(&root);
            for link in text::clause_links(&content) {
                let (target, fragment) = link.target.split_once('#').unwrap_or((link.target, ""));
                let relative = relative_path(&parent.join(target), &directory);
                candidates
                    .entry((prefix.clone(), link.name.to_owned()))
                    .or_default()
                    .insert(format!(
                        "{}; [§{}]({relative}#{fragment})",
                        link.description, link.number
                    ));
            }
        }
    }
    Ok(candidates
        .into_iter()
        .filter(|(_, descriptions)| descriptions.len() == 1)
        .filter_map(|(key, descriptions)| Some((key, descriptions.into_iter().next()?)))
        .collect())
}

struct SupportRecord<'a> {
    supported: bool,
    note: &'a str,
    evidence: Vec<&'a str>,
}

fn support_records<'a>(
    support: &'a Json,
    field_identifiers: &BTreeSet<String>,
) -> Result<HashMap<&'a str, SupportRecord<'a>>, String> {
    let root = repository();
    let entries = support
        .as_object()
        .ok_or("support.json: expected an object")?;
    let unknown: Vec<_> = entries
        .iter()
        .map(|(name, _)| name.as_str())
        .filter(|name| !field_identifiers.contains(*name))
        .collect();
    if !unknown.is_empty() {
        return Err(format!(
            "Support entries without schema fields: {}",
            unknown.join(", ")
        ));
    }
    let mut records = HashMap::new();
    for (identifier, record) in entries {
        let status = record
            .get("status")
            .and_then(Json::as_str)
            .unwrap_or_default();
        let note = record
            .get("note")
            .and_then(Json::as_str)
            .unwrap_or_default();
        if !matches!(status, "supported" | "partial") || note.is_empty() {
            return Err(format!("Invalid support record: {identifier}"));
        }
        let evidence: Vec<&str> = record
            .get("evidence")
            .and_then(Json::as_array)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .collect();
        if evidence.is_empty() || evidence.iter().any(|path| !root.join(path).is_file()) {
            return Err(format!("Missing support evidence: {identifier}"));
        }
        records.insert(
            identifier.as_str(),
            SupportRecord {
                supported: status == "supported",
                note,
                evidence,
            },
        );
    }
    Ok(records)
}

fn family_file(family: &Json) -> &str {
    family
        .get("file")
        .and_then(Json::as_str)
        .unwrap_or_default()
}

fn projection(root: Option<&Json>) -> Projection<'_> {
    let get = |name| root.and_then(|root| root.get(name));
    Projection {
        kind: get("kind").and_then(Json::as_str).unwrap_or("complexType"),
        selection: get("fields")
            .and_then(Json::as_array)
            .map(|fields| fields.iter().filter_map(Json::as_str).collect()),
        attributes_only: get("attributes_only")
            .and_then(Json::as_bool)
            .unwrap_or(false),
    }
}

struct Inventory {
    documents: Vec<(String, String)>,
    types: usize,
    fields: usize,
    value_types: usize,
}

struct Links<'a> {
    owners: &'a HashMap<String, String>,
    value_types: &'a BTreeSet<String>,
}

impl Links<'_> {
    fn type_link(&self, identifier: Option<&str>) -> String {
        let Some(identifier) = identifier else {
            return "extension wildcard".into();
        };
        if let Some(owner) = self.owners.get(identifier) {
            format!("[{identifier}]({owner}#{})", anchor(identifier))
        } else if self.value_types.contains(identifier) {
            format!(
                "[{identifier}]({}#{})",
                value_file(identifier),
                anchor(identifier)
            )
        } else {
            format!("`{identifier}`")
        }
    }
}

impl Inventory {
    fn build(configuration: &Json, support: &Json, schemas: &Schemas) -> Result<Self, String> {
        let descriptions = clause_descriptions(configuration)?;
        let families = configuration
            .get("families")
            .and_then(Json::as_array)
            .ok_or("scope.json: missing families")?;
        let mut owners: HashMap<String, String> = HashMap::new();
        let mut projections: HashMap<String, &Json> = HashMap::new();
        let mut pending = VecDeque::new();
        for family in families {
            for root in family
                .get("roots")
                .and_then(Json::as_array)
                .unwrap_or_default()
            {
                let identifier = root
                    .get("type")
                    .and_then(Json::as_str)
                    .ok_or("scope.json: root without type")?;
                if owners.contains_key(identifier) {
                    return Err(format!("Duplicate root: {identifier}"));
                }
                owners.insert(identifier.to_owned(), family_file(family).to_owned());
                projections.insert(identifier.to_owned(), root);
                pending.push_back(identifier.to_owned());
            }
        }
        let mut fields_by_type: Vec<(String, Vec<Field>)> = Vec::new();
        let mut value_types = BTreeSet::new();
        while let Some(identifier) = pending.pop_front() {
            let projection = projection(projections.get(&identifier).copied());
            if schemas.variants(&identifier, projection.kind).is_empty() {
                return Err(format!("Unresolved complex type: {identifier}"));
            }
            let fields = schemas.fields(&identifier, &projection)?;
            for field in &fields {
                let Some(dependency) = field.field_type.as_deref() else {
                    continue;
                };
                if !schemas.variants(dependency, "complexType").is_empty() {
                    if !owners.contains_key(dependency) {
                        owners.insert(dependency.to_owned(), owners[&identifier].clone());
                        pending.push_back(dependency.to_owned());
                    }
                } else if !schemas.variants(dependency, "simpleType").is_empty() {
                    value_types.insert(dependency.to_owned());
                } else if !dependency.starts_with("xsd:") && !dependency.starts_with("xml:") {
                    return Err(format!("Unresolved field type {dependency}"));
                }
            }
            fields_by_type.push((identifier, fields));
        }
        let mut pending: VecDeque<String> = value_types.iter().cloned().collect();
        while let Some(identifier) = pending.pop_front() {
            for dependency in schemas.scalar_dependencies(&identifier)? {
                let known = !schemas.variants(&dependency, "simpleType").is_empty();
                if known && !value_types.contains(&dependency) {
                    value_types.insert(dependency.clone());
                    pending.push_back(dependency);
                } else if !known && !dependency.starts_with("xsd:") {
                    return Err(format!("Unresolved scalar base {dependency}"));
                }
            }
        }
        let field_identifiers: BTreeSet<String> = fields_by_type
            .iter()
            .flat_map(|(identifier, fields)| {
                fields
                    .iter()
                    .map(move |field| format!("{identifier}/{}", field.label))
            })
            .collect();
        let support = support_records(support, &field_identifiers)?;
        let links = Links {
            owners: &owners,
            value_types: &value_types,
        };
        let notes = configuration.get("supplemental_notes");
        let note = |identifier: &str| {
            notes
                .and_then(|notes| notes.get(identifier))
                .and_then(Json::as_str)
        };
        let mut documents = Vec::new();
        for family in families {
            let file = family_file(family);
            let text = |name| family.get(name).and_then(Json::as_str).unwrap_or_default();
            let identifiers: Vec<_> = fields_by_type
                .iter()
                .filter(|(identifier, _)| owners[identifier] == file)
                .collect();
            let mut lines: Vec<String> = [
                &format!("# {}", text("title")),
                "",
                "[Inventory guide](README.md)",
                "",
                text("description"),
                "",
                "Checkmarks concern the stated Mote language/model behaviour only. Partial",
                "mappings stay unchecked. Unannotated fields have no supported Mote mapping;",
                "shared schema carriers do not independently define authoring attributes.",
                "",
                "Nested types and complete value definitions are linked below. XML element",
                "names and `@attributes` identify specification properties, not new Mote syntax.",
                "",
                "## Types",
                "",
            ]
            .map(str::to_owned)
            .into();
            for (identifier, _) in &identifiers {
                lines.push(format!("- [{identifier}](#{})", anchor(identifier)));
            }
            for (identifier, fields) in &identifiers {
                lines.extend([
                    String::new(),
                    format!("<a id=\"{}\"></a>", anchor(identifier)),
                    String::new(),
                    format!("## {identifier}"),
                    String::new(),
                ]);
                let root = projections.get(identifier).copied();
                let projection = projection(root);
                let mut editions: Vec<(&str, &Source)> = Vec::new();
                for definition in schemas.variants(identifier, projection.kind) {
                    let source = &*definition.source;
                    match editions
                        .iter_mut()
                        .find(|(edition, _)| *edition == source.edition)
                    {
                        Some(entry) => entry.1 = source,
                        None => editions.push((&source.edition, source)),
                    }
                }
                let sources: Vec<_> = editions.iter().map(|(_, source)| source.link()).collect();
                lines.push(format!("Sources: {}.", sources.join(", ")));
                if let Some(note) = note(identifier) {
                    lines.extend([String::new(), note.to_owned()]);
                }
                if projection.attributes_only {
                    lines.extend([
                        String::new(),
                        "Container attributes only; child content has its own directive inventories."
                            .into(),
                    ]);
                }
                if let Some(selection) = projection.selection.filter(|fields| !fields.is_empty()) {
                    let selection: Vec<_> =
                        selection.iter().map(|field| format!("`{field}`")).collect();
                    lines.extend([
                        String::new(),
                        format!(
                            "Selected document-level controls: {}.",
                            selection.join(", ")
                        ),
                    ]);
                }
                lines.push(String::new());
                if fields.is_empty() {
                    lines.push("Empty marker type; no attributes or child properties.".into());
                }
                for field in fields {
                    lines.push(field_line(
                        identifier,
                        field,
                        &links,
                        &descriptions,
                        &support,
                    ));
                }
            }
            documents.push((file.to_owned(), lines.join("\n") + "\n"));
        }
        documents.extend(value_documents(schemas, &links, configuration)?);
        Ok(Self {
            documents,
            types: fields_by_type.len(),
            fields: field_identifiers.len(),
            value_types: value_types.len(),
        })
    }
}

fn field_line(
    identifier: &str,
    field: &Field,
    links: &Links,
    descriptions: &HashMap<(String, String), String>,
    support: &HashMap<&str, SupportRecord>,
) -> String {
    let record = support.get(format!("{identifier}/{}", field.label).as_str());
    let checked = if record.is_some_and(|record| record.supported) {
        'x'
    } else {
        ' '
    };
    let editions: BTreeSet<_> = field
        .sources
        .iter()
        .map(|source| source.edition.as_str())
        .collect();
    let editions: Vec<_> = editions.into_iter().collect();
    let constraints: Vec<_> = field
        .constraints
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect();
    let mut note = String::new();
    let prefix = identifier.split(':').next().unwrap_or_default();
    if let Some(description) = descriptions.get(&(prefix.to_owned(), field.label.clone())) {
        note = format!(" {description}.");
    }
    if let Some(record) = record {
        let evidence: Vec<_> = record
            .evidence
            .iter()
            .map(|path| {
                let name = Path::new(path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                format!("[{name}](../../../{path})")
            })
            .collect();
        note += &format!(" {} Evidence: {}.", record.note, evidence.join(", "));
    }
    let constraints = if constraints.is_empty() {
        String::new()
    } else {
        format!("; {}", constraints.join("; "))
    };
    format!(
        "- [{checked}] `{}` — {}; {}{constraints}.{note}",
        field.label,
        links.type_link(field.field_type.as_deref()),
        editions.join(", ")
    )
}

fn value_documents(
    schemas: &Schemas,
    links: &Links,
    configuration: &Json,
) -> Result<Vec<(String, String)>, String> {
    let notes = configuration.get("supplemental_notes");
    let aliases = configuration.get("legacy_value_aliases");
    let mut documents: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for identifier in links.value_types {
        documents.entry(value_file(identifier)).or_default();
    }
    for (filename, lines) in &mut documents {
        let family = filename
            .trim_start_matches("values-")
            .trim_end_matches(".md");
        let mut title = family.chars();
        let title: String = title
            .next()
            .map(|first| first.to_uppercase().chain(title).collect())
            .unwrap_or_default();
        lines.extend(
            [
                &format!("# {title} value definitions"),
                "",
                "[Inventory guide](README.md)",
                "",
                "These are complete schema value domains for the selected properties.",
                "Property rows record Mote support and supported subsets separately.",
                "Identical Strict and Transitional definitions are combined.",
                "See also [prose-defined extensions](prose-extensions.md).",
                "",
                "## Types",
                "",
            ]
            .map(str::to_owned),
        );
        for identifier in links.value_types {
            if value_file(identifier) == *filename {
                lines.push(format!("- [{identifier}](#{})", anchor(identifier)));
            }
        }
        lines.push(String::new());
    }
    for identifier in links.value_types {
        let lines = documents
            .get_mut(&value_file(identifier))
            .expect("value file exists");
        lines.extend([
            format!("<a id=\"{}\"></a>", anchor(identifier)),
            String::new(),
            format!("## {identifier}"),
            String::new(),
        ]);
        if let Some(note) = notes
            .and_then(|notes| notes.get(identifier))
            .and_then(Json::as_str)
        {
            lines.extend([note.to_owned(), String::new()]);
        }
        if let Some(alias) = aliases
            .and_then(|aliases| aliases.get(identifier))
            .and_then(Json::as_str)
        {
            lines.extend([
                "Publisher imports use this legacy name; the corresponding shared".to_owned(),
                format!("value definition is `{alias}` in the retained ECMA schemas."),
                String::new(),
            ]);
        }
        let mut definitions: Vec<(Vec<String>, Vec<Rc<Source>>)> = Vec::new();
        for definition in schemas.variants(identifier, "simpleType") {
            let mut definition_lines = Vec::new();
            for node in definition.element.descendants() {
                if matches!(
                    node.tag.as_str(),
                    "simpleType" | "annotation" | "documentation"
                ) {
                    continue;
                }
                let mut parts = Vec::new();
                for (name, value) in &node.attributes {
                    if matches!(name.as_str(), "base" | "itemType" | "memberTypes") {
                        let mut types = Vec::new();
                        for token in value.split_whitespace() {
                            let resolved = schemas.resolve(
                                token,
                                &definition.namespaces,
                                &definition.source,
                            )?;
                            types.push(links.type_link(Some(&resolved)));
                        }
                        parts.push(format!("{name}: {}", types.join(", ")));
                    } else {
                        parts.push(format!("{name}: `{}`", value.replace('`', "&#96;")));
                    }
                }
                definition_lines.push(format!("- {}: {}", node.tag, parts.join("; ")));
            }
            match definitions
                .iter_mut()
                .find(|(lines, _)| *lines == definition_lines)
            {
                Some((_, sources)) => sources.push(definition.source.clone()),
                None => definitions.push((definition_lines, vec![definition.source.clone()])),
            }
        }
        for (definition_lines, sources) in definitions {
            let sources: Vec<_> = sources.iter().map(|source| source.link()).collect();
            lines.extend([format!("{}:", sources.join(", ")), String::new()]);
            lines.extend(definition_lines);
            lines.push(String::new());
        }
    }
    let index: Vec<String> = [
        "# Schema value definitions",
        "",
        "[Inventory guide](README.md)",
        "",
    ]
    .map(str::to_owned)
    .into_iter()
    .chain(
        documents
            .keys()
            .map(|filename| format!("- [{filename}]({filename})")),
    )
    .collect();
    let mut output: Vec<_> = documents
        .into_iter()
        .map(|(filename, lines)| (filename, lines.join("\n").trim_end().to_owned() + "\n"))
        .collect();
    output.push(("values.md".into(), index.join("\n") + "\n"));
    Ok(output)
}
