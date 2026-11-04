//! The retained XML schemas, indexed by namespace prefix, declaration kind and name.
use super::text;
use crate::{json::Json, repository};
use std::{
    collections::{BTreeSet, HashMap},
    fs,
    path::Path,
    rc::Rc,
};

const SCHEMA_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema";

/// An owned element whose tag omits the XML Schema namespace and keeps any other as `{uri}`.
pub(super) struct Element {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Element>,
}

impl Element {
    fn from_node(node: roxmltree::Node) -> Self {
        let name = |namespace: Option<&str>, local: &str| match namespace {
            Some(SCHEMA_NAMESPACE) | None => local.to_owned(),
            Some(namespace) => format!("{{{namespace}}}{local}"),
        };
        let tag = node.tag_name();
        Self {
            tag: name(tag.namespace(), tag.name()),
            attributes: node
                .attributes()
                .map(|attribute| {
                    let name = match attribute.namespace() {
                        Some(namespace) => format!("{{{namespace}}}{}", attribute.name()),
                        None => attribute.name().to_owned(),
                    };
                    (name, attribute.value().to_owned())
                })
                .collect(),
            children: node
                .children()
                .filter(roxmltree::Node::is_element)
                .map(Self::from_node)
                .collect(),
        }
    }
    pub fn get(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(attribute, _)| attribute == name)
            .map(|(_, value)| value.as_str())
    }
    /// The element and all its descendants in document order.
    pub fn descendants(&self) -> Vec<&Element> {
        let mut elements = vec![self];
        for child in &self.children {
            elements.extend(child.descendants());
        }
        elements
    }
}

#[derive(Clone)]
pub(super) struct Source {
    pub prefix: String,
    pub namespace: String,
    pub edition: String,
    pub path: Option<String>,
    pub document: Option<String>,
    pub citation_path: Option<String>,
}

impl Source {
    pub fn link(&self) -> String {
        let path = self
            .citation_path
            .clone()
            .or_else(|| self.path.clone())
            .unwrap_or_else(|| {
                format!(
                    "docs/reference/reading/ooxml/microsoft/{}/README.md",
                    self.document.as_deref().unwrap_or("None")
                )
            });
        let relative = match path.strip_prefix("docs/") {
            Some(path) => format!("../../{path}"),
            None => format!("../../../{path}"),
        };
        format!("[{}]({relative})", self.edition)
    }
}

/// The namespace declarations of one schema document; later declarations replace earlier ones.
pub(super) type Namespaces = HashMap<String, String>;

#[derive(Clone)]
pub(super) struct Definition {
    pub element: Rc<Element>,
    pub namespaces: Rc<Namespaces>,
    pub source: Rc<Source>,
}

pub(super) struct Field {
    pub label: String,
    pub field_type: Option<String>,
    pub constraints: Vec<(String, String)>,
    pub sources: Vec<Rc<Source>>,
}

/// A field label, its resolved type and its remaining schema attributes.
type Declared = (String, Option<String>, Vec<(String, String)>);

pub(super) struct Projection<'a> {
    pub kind: &'a str,
    pub selection: Option<Vec<&'a str>>,
    pub attributes_only: bool,
}

pub(super) struct Schemas {
    definitions: HashMap<(String, String, String), Vec<Definition>>,
    aliases: HashMap<String, String>,
    legacy_aliases: HashMap<String, String>,
}

fn field<'a>(value: &'a Json, name: &str) -> Result<&'a str, String> {
    value
        .get(name)
        .and_then(Json::as_str)
        .ok_or_else(|| format!("scope.json: missing {name}"))
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))
}

fn parse(text: &str) -> Result<roxmltree::Document<'_>, String> {
    roxmltree::Document::parse(text).map_err(|error| error.to_string())
}

/// Only the schema declarations within the retained publisher text, without prose or footers.
fn publisher_fragments(document: &str) -> Result<Vec<String>, String> {
    let directory = repository()
        .join("docs/reference/reading/ooxml/microsoft")
        .join(document)
        .join("chunks");
    let mut paths: Vec<_> = fs::read_dir(&directory)
        .map_err(|error| format!("{}: {error}", directory.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
        .collect();
    paths.sort();
    let mut blocks = Vec::new();
    for path in paths {
        blocks.extend(
            text::text_blocks(&read(&path)?)
                .into_iter()
                .map(str::to_owned),
        );
    }
    let joined = blocks.join("\n");
    let lines: Vec<&str> = text::lines(&joined)
        .into_iter()
        .filter(|line| {
            !text::is_page_number(line)
                && ![
                    "[MS-",
                    "Copyright ©",
                    "Release:",
                    "Word Extensions to the Office",
                    "Office Drawing",
                    "PowerPoint (.pptx)",
                ]
                .iter()
                .any(|prefix| line.starts_with(prefix))
        })
        .collect();
    let text = lines.join("\n");
    Ok(text::schema_fragments(&text)
        .into_iter()
        .map(|fragment| text::tags(fragment).join("\n"))
        .collect())
}

impl Schemas {
    pub fn load(configuration: &Json) -> Result<Self, String> {
        let mut schemas = Self {
            definitions: HashMap::new(),
            aliases: HashMap::from([
                (SCHEMA_NAMESPACE.to_owned(), "xsd".to_owned()),
                (
                    "http://www.w3.org/XML/1998/namespace".to_owned(),
                    "xml".to_owned(),
                ),
            ]),
            legacy_aliases: configuration
                .get("legacy_value_aliases")
                .and_then(Json::as_object)
                .unwrap_or_default()
                .iter()
                .filter_map(|(name, value)| Some((name.clone(), value.as_str()?.to_owned())))
                .collect(),
        };
        let sources = configuration
            .get("schemas")
            .and_then(Json::as_array)
            .ok_or("scope.json: missing schemas")?;
        for source in sources {
            schemas.aliases.insert(
                field(source, "namespace")?.to_owned(),
                field(source, "prefix")?.to_owned(),
            );
        }
        let root = repository();
        for entry in sources {
            let source = Source {
                prefix: field(entry, "prefix")?.to_owned(),
                namespace: field(entry, "namespace")?.to_owned(),
                edition: field(entry, "edition")?.to_owned(),
                path: entry.get("path").and_then(Json::as_str).map(str::to_owned),
                document: entry
                    .get("document")
                    .and_then(Json::as_str)
                    .map(str::to_owned),
                citation_path: None,
            };
            if let Some(path) = &source.path {
                let text = read(&root.join(path))?;
                schemas.add(&text, Rc::new(source.clone()))?;
            } else {
                let mut matches = Vec::new();
                for fragment in publisher_fragments(source.document.as_deref().unwrap_or(""))? {
                    if parse(&fragment)?
                        .root_element()
                        .attribute("targetNamespace")
                        == Some(&source.namespace)
                    {
                        matches.push(fragment);
                    }
                }
                if matches.len() != 1 {
                    return Err(format!("Expected one schema for {}", source.namespace));
                }
                schemas.add(&matches[0], Rc::new(source.clone()))?;
            }
            let supplemental = entry
                .get("supplemental_elements")
                .and_then(Json::as_object)
                .unwrap_or_default();
            for (name, path) in supplemental {
                let path = path
                    .as_str()
                    .ok_or("scope.json: invalid supplemental path")?;
                let text = read(&root.join(path))?;
                let declarations = text::element_declarations(&text, name);
                if declarations.len() != 1 {
                    return Err(format!("Expected one supplemental declaration for {name}"));
                }
                let wrapper = format!(
                    "<xsd:schema xmlns:xsd=\"{SCHEMA_NAMESPACE}\" xmlns=\"{0}\" targetNamespace=\"{0}\">{1}</xsd:schema>",
                    source.namespace, declarations[0]
                );
                schemas.add(
                    &wrapper,
                    Rc::new(Source {
                        citation_path: Some(path.to_owned()),
                        ..source.clone()
                    }),
                )?;
            }
        }
        Ok(schemas)
    }

    fn add(&mut self, text: &str, source: Rc<Source>) -> Result<(), String> {
        let document = parse(text)?;
        let mut namespaces = Namespaces::new();
        for node in document.descendants().filter(roxmltree::Node::is_element) {
            let inherited: Vec<_> = node
                .parent_element()
                .map(|parent| {
                    parent
                        .namespaces()
                        .map(|namespace| (namespace.name(), namespace.uri()))
                        .collect()
                })
                .unwrap_or_default();
            for namespace in node.namespaces() {
                let pair = (namespace.name(), namespace.uri());
                if namespace.name() != Some("xml") && !inherited.contains(&pair) {
                    namespaces.insert(
                        namespace.name().unwrap_or_default().to_owned(),
                        namespace.uri().to_owned(),
                    );
                }
            }
        }
        let root = document.root_element();
        if root.attribute("targetNamespace") != Some(&source.namespace) {
            return Err(format!(
                "schema namespace differs from {}",
                source.namespace
            ));
        }
        let namespaces = Rc::new(namespaces);
        for definition in Element::from_node(root).children {
            if let Some(name) = definition.get("name") {
                let key = (
                    source.prefix.clone(),
                    definition.tag.clone(),
                    name.to_owned(),
                );
                self.definitions.entry(key).or_default().push(Definition {
                    element: Rc::new(definition),
                    namespaces: namespaces.clone(),
                    source: source.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn resolve(
        &self,
        name: &str,
        namespaces: &Namespaces,
        source: &Source,
    ) -> Result<String, String> {
        let name = name.trim();
        if let Some((prefix, local)) = name.split_once(':') {
            if prefix == "xml" {
                return Ok(name.to_owned());
            }
            let namespace = namespaces.get(prefix);
            let alias = namespace.and_then(|namespace| self.aliases.get(namespace));
            return alias
                .map(|alias| format!("{alias}:{local}"))
                .ok_or_else(|| {
                    format!(
                        "Unregistered namespace {prefix}: {}",
                        namespace.map_or("None", String::as_str)
                    )
                });
        }
        let namespace = namespaces.get("").unwrap_or(&source.namespace);
        let alias = self
            .aliases
            .get(namespace)
            .ok_or_else(|| format!("Unregistered namespace {namespace}"))?;
        Ok(format!("{alias}:{name}"))
    }

    pub fn variants(&self, identifier: &str, kind: &str) -> &[Definition] {
        let identifier = match kind {
            "simpleType" => self
                .legacy_aliases
                .get(identifier)
                .map_or(identifier, String::as_str),
            _ => identifier,
        };
        let (prefix, local) = identifier.split_once(':').unwrap_or((identifier, ""));
        self.definitions
            .get(&(prefix.to_owned(), kind.to_owned(), local.to_owned()))
            .map_or(&[], Vec::as_slice)
    }

    /// An extension imports one namespace, not every retained edition of it.
    fn imported_variants(
        &self,
        name: &str,
        kind: &str,
        namespaces: &Namespaces,
        source: &Source,
    ) -> Result<Vec<Definition>, String> {
        let name = name.trim();
        let identifier = self.resolve(name, namespaces, source)?;
        let namespace = match name.split_once(':') {
            Some((prefix, _)) => namespaces.get(prefix),
            None => Some(namespaces.get("").unwrap_or(&source.namespace)),
        };
        let matching: Vec<_> = self
            .variants(&identifier, kind)
            .iter()
            .filter(|entry| Some(&entry.source.namespace) == namespace)
            .cloned()
            .collect();
        let edition: Vec<_> = matching
            .iter()
            .filter(|entry| entry.source.edition == source.edition)
            .cloned()
            .collect();
        Ok(if edition.is_empty() {
            matching
        } else {
            edition
        })
    }

    pub fn fields(&self, identifier: &str, projection: &Projection) -> Result<Vec<Field>, String> {
        let mut combined: Vec<(Vec<(String, String)>, Field)> = Vec::new();
        for definition in self.variants(identifier, projection.kind) {
            let mut fields = Vec::new();
            let seen = BTreeSet::from([identifier.to_owned()]);
            self.visit(
                identifier,
                &definition.element,
                definition,
                &seen,
                projection,
                &mut fields,
            )?;
            for (label, field_type, constraints) in fields {
                let mut key = constraints.clone();
                key.sort();
                let existing = combined.iter_mut().find(|(sorted, field)| {
                    field.label == label && field.field_type == field_type && *sorted == key
                });
                match existing {
                    Some((_, field)) => field.sources.push(definition.source.clone()),
                    None => combined.push((
                        key,
                        Field {
                            label,
                            field_type,
                            constraints,
                            sources: vec![definition.source.clone()],
                        },
                    )),
                }
            }
        }
        Ok(combined.into_iter().map(|(_, field)| field).collect())
    }

    fn visit(
        &self,
        identifier: &str,
        node: &Element,
        context: &Definition,
        seen: &BTreeSet<String>,
        projection: &Projection,
        fields: &mut Vec<Declared>,
    ) -> Result<(), String> {
        let (namespaces, source) = (&*context.namespaces, &*context.source);
        let tag = node.tag.as_str();
        if matches!(tag, "element" | "attribute" | "any" | "anyAttribute") {
            let is_attribute = matches!(tag, "attribute" | "anyAttribute");
            if projection.attributes_only && !is_attribute {
                return Ok(());
            }
            let mut field_name = node
                .get("name")
                .or_else(|| node.get("ref"))
                .unwrap_or("*")
                .to_owned();
            if let Some(selection) = &projection.selection
                && !selection.contains(&field_name.as_str())
            {
                return Ok(());
            }
            let mut field_type = node.get("type").map(str::to_owned);
            if let Some(reference) = node.get("ref") {
                let resolved = self.resolve(reference, namespaces, source)?;
                field_name = resolved.clone();
                if !self.variants(&resolved, tag).is_empty() {
                    let matches = self.imported_variants(reference, tag, namespaces, source)?;
                    let [declaration] = matches.as_slice() else {
                        return Err(format!("Ambiguous declaration {resolved}"));
                    };
                    field_type = declaration
                        .element
                        .get("type")
                        .map(|name| {
                            self.resolve(name, &declaration.namespaces, &declaration.source)
                        })
                        .transpose()?;
                } else if resolved == "xml:space" {
                    field_type = Some("xml:space".into());
                } else {
                    return Err(format!("Unresolved declaration {resolved}"));
                }
            } else if let Some(name) = &field_type {
                field_type = Some(self.resolve(name, namespaces, source)?);
            } else if matches!(tag, "element" | "attribute") {
                return Err(format!(
                    "Anonymous or missing type: {identifier}/{field_name}"
                ));
            }
            let label = format!("{}{field_name}", if is_attribute { "@" } else { "" });
            let constraints = node
                .attributes
                .iter()
                .filter(|(name, _)| !matches!(name.as_str(), "name" | "ref" | "type"))
                .cloned()
                .collect();
            fields.push((label, field_type, constraints));
            return Ok(());
        }
        if matches!(tag, "group" | "attributeGroup")
            && let Some(reference) = node.get("ref")
        {
            let resolved = self.resolve(reference, namespaces, source)?;
            let matches = self.imported_variants(reference, tag, namespaces, source)?;
            if matches.is_empty() {
                return Err(format!("Unresolved group {resolved}"));
            }
            if seen.contains(&resolved) {
                return Err(format!("Recursive schema group {resolved}"));
            }
            let mut seen = seen.clone();
            seen.insert(resolved);
            for group in &matches {
                self.visit(identifier, &group.element, group, &seen, projection, fields)?;
            }
            return Ok(());
        }
        if matches!(tag, "extension" | "restriction")
            && let Some(base_name) = node.get("base")
        {
            let base = self.resolve(base_name, namespaces, source)?;
            let variants = self.variants(&base, "complexType");
            let matches = self.imported_variants(base_name, "complexType", namespaces, source)?;
            if !variants.is_empty() && matches.is_empty() {
                return Err(format!("Unresolved base edition for {base}"));
            }
            if variants.is_empty()
                && !(base.starts_with("xsd:") || !self.variants(&base, "simpleType").is_empty())
            {
                return Err(format!("Unresolved base {base}"));
            }
            if seen.contains(&base) {
                return Err(format!("Recursive schema base {base}"));
            }
            let mut seen = seen.clone();
            seen.insert(base);
            for definition in &matches {
                self.visit(
                    identifier,
                    &definition.element,
                    definition,
                    &seen,
                    projection,
                    fields,
                )?;
            }
        }
        for child in &node.children {
            self.visit(identifier, child, context, seen, projection, fields)?;
        }
        Ok(())
    }

    pub fn scalar_dependencies(&self, identifier: &str) -> Result<BTreeSet<String>, String> {
        let mut dependencies = BTreeSet::new();
        for definition in self.variants(identifier, "simpleType") {
            for node in definition.element.descendants() {
                for attribute in ["base", "itemType", "memberTypes"] {
                    for name in node.get(attribute).unwrap_or_default().split_whitespace() {
                        dependencies.insert(self.resolve(
                            name,
                            &definition.namespaces,
                            &definition.source,
                        )?);
                    }
                }
            }
        }
        Ok(dependencies)
    }
}
