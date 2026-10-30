#!/usr/bin/env python3
"""Check or refresh the specification-derived attribute inventory using local sources."""

import argparse
import io
import json
import os
import re
from collections import deque
from pathlib import Path
from xml.etree import ElementTree

SCHEMA_NAMESPACE = 'http://www.w3.org/2001/XMLSchema'
SCHEMA_TAG = '{' + SCHEMA_NAMESPACE + '}'
ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / 'docs/checklists/attributes'
CONFIGURATION = json.loads((INVENTORY / 'scope.json').read_text())
SUPPORT = json.loads((INVENTORY / 'support.json').read_text())


def schema_fragments(document_name):
    """Read only XML schema declarations from the retained publisher text."""
    directory = ROOT / 'docs/reference/reading/ooxml/microsoft' / document_name
    blocks = []
    for path in sorted((directory / 'chunks').glob('*.md')):
        blocks.extend(re.findall(r'```text\n(.*?)\n```', path.read_text(), re.S))
    lines = []
    for line in '\n'.join(blocks).splitlines():
        if re.match(r'^\s*\d+ / \d+\s*$', line):
            continue
        if line.startswith(('[MS-', 'Copyright ©', 'Release:',
                            'Word Extensions to the Office', 'Office Drawing',
                            'PowerPoint (.pptx)')):
            continue
        lines.append(line)
    for fragment in re.findall(r'<(?:xsd|xs):schema\b.*?</(?:xsd|xs):schema>',
                               '\n'.join(lines), re.S):
        # Schema definitions use attributes for names, values, and restrictions.
        # Remove surrounding prose/footers, never XML attribute values.
        yield '\n'.join(re.findall(r'<[^>]+>', fragment))


class SchemaCollection:
    def __init__(self):
        self.definitions = {}
        self.aliases = {SCHEMA_NAMESPACE: 'xsd',
                        'http://www.w3.org/XML/1998/namespace': 'xml'}
        for source in CONFIGURATION['schemas']:
            self.aliases[source['namespace']] = source['prefix']
        for source in CONFIGURATION['schemas']:
            if 'path' in source:
                self.add_schema((ROOT / source['path']).read_text(), source)
            else:
                matches = [fragment for fragment in schema_fragments(source['document'])
                           if ElementTree.fromstring(fragment).get('targetNamespace')
                           == source['namespace']]
                if len(matches) != 1:
                    raise ValueError(f"Expected one schema for {source['namespace']}")
                self.add_schema(matches[0], source)
            for name, path in source.get('supplemental_elements', {}).items():
                declarations = re.findall(r'<xsd:element\s+name="' + re.escape(name)
                                          + r'"[^>]*/>', (ROOT / path).read_text())
                if len(declarations) != 1:
                    raise ValueError(f'Expected one supplemental declaration for {name}')
                wrapper = (f'<xsd:schema xmlns:xsd="{SCHEMA_NAMESPACE}" '
                           f'xmlns="{source["namespace"]}" '
                           f'targetNamespace="{source["namespace"]}">'
                           + declarations[0] + '</xsd:schema>')
                self.add_schema(wrapper, {**source, 'citation_path': path})

    def add_schema(self, text, source):
        # iterparse supplies (event, (prefix, URI)); collect the namespace pairs.
        namespaces = dict(pair for _, pair in ElementTree.iterparse(
            io.StringIO(text), events=['start-ns']))
        document = ElementTree.fromstring(text)
        assert document.get('targetNamespace') == source['namespace']
        for definition in document:
            if definition.get('name'):
                key = (source['prefix'], definition.tag.removeprefix(SCHEMA_TAG),
                       definition.get('name'))
                self.definitions.setdefault(key, []).append((definition, namespaces, source))

    def resolve(self, name, namespaces, source):
        name = name.strip()
        if ':' in name:
            prefix, local_name = name.split(':', 1)
            if prefix == 'xml':
                return name
            namespace = namespaces.get(prefix)
            if namespace not in self.aliases:
                raise ValueError(f'Unregistered namespace {prefix}: {namespace}')
            return self.aliases[namespace] + ':' + local_name
        namespace = namespaces.get('', source['namespace'])
        return self.aliases[namespace] + ':' + name

    def variants(self, identifier, kind='complexType'):
        if kind == 'simpleType':
            identifier = CONFIGURATION.get('legacy_value_aliases', {}).get(identifier, identifier)
        prefix, local_name = identifier.split(':', 1)
        return self.definitions.get((prefix, kind, local_name), [])

    def imported_variants(self, name, kind, namespaces, source):
        """An extension imports one namespace, not every retained edition of it."""
        name = name.strip()
        identifier = self.resolve(name, namespaces, source)
        namespace = (namespaces.get(name.split(':', 1)[0]) if ':' in name
                     else namespaces.get('', source['namespace']))
        variants = self.variants(identifier, kind)
        matching = [entry for entry in variants if entry[2]['namespace'] == namespace]
        edition = [entry for entry in matching if entry[2]['edition'] == source['edition']]
        return edition or matching

    def fields(self, identifier, selection=None, attributes_only=False, kind='complexType'):
        combined = {}
        for definition, namespaces, source in self.variants(identifier, kind):
            fields = []

            def visit(node, current_namespaces, current_source, seen):
                tag = node.tag.removeprefix(SCHEMA_TAG)
                if tag in ('element', 'attribute', 'any', 'anyAttribute'):
                    if attributes_only and tag not in ('attribute', 'anyAttribute'):
                        return
                    field_name = node.get('name') or node.get('ref') or '*'
                    if selection is not None and field_name not in selection:
                        return
                    field_type = node.get('type')
                    if node.get('ref'):
                        resolved_reference = self.resolve(node.get('ref'), current_namespaces,
                                                          current_source)
                        field_name = resolved_reference
                        declarations = self.variants(resolved_reference, tag)
                        if declarations:
                            matches = self.imported_variants(
                                node.get('ref'), tag, current_namespaces, current_source)
                            if len(matches) != 1:
                                raise ValueError(f'Ambiguous declaration {resolved_reference}')
                            declaration, reference_namespaces, reference_source = matches[0]
                            field_type = declaration.get('type')
                            if field_type:
                                field_type = self.resolve(field_type, reference_namespaces,
                                                          reference_source)
                        elif resolved_reference == 'xml:space':
                            field_type = 'xml:space'
                        else:
                            raise ValueError(f'Unresolved declaration {resolved_reference}')
                    elif field_type:
                        field_type = self.resolve(field_type, current_namespaces, current_source)
                    elif tag in ('element', 'attribute'):
                        raise ValueError(f'Anonymous or missing type: {identifier}/{field_name}')
                    label = ('@' if tag in ('attribute', 'anyAttribute') else '') + field_name
                    constraints = {key: value for key, value in node.attrib.items()
                                   if key not in ('name', 'ref', 'type')}
                    fields.append((label, field_type, constraints))
                    return
                if tag in ('group', 'attributeGroup') and node.get('ref'):
                    reference = self.resolve(node.get('ref'), current_namespaces, current_source)
                    matches = self.imported_variants(
                        node.get('ref'), tag, current_namespaces, current_source)
                    if not matches:
                        raise ValueError(f'Unresolved group {reference}')
                    if reference in seen:
                        raise ValueError(f'Recursive schema group {reference}')
                    for group, group_namespaces, group_source in matches:
                        visit(group, group_namespaces, group_source, seen | {reference})
                    return
                if tag in ('extension', 'restriction') and node.get('base'):
                    base = self.resolve(node.get('base'), current_namespaces, current_source)
                    variants = self.variants(base)
                    matches = self.imported_variants(
                        node.get('base'), 'complexType', current_namespaces, current_source)
                    if variants and not matches:
                        raise ValueError(f'Unresolved base edition for {base}')
                    if not variants and not (base.startswith('xsd:')
                                             or self.variants(base, 'simpleType')):
                        raise ValueError(f'Unresolved base {base}')
                    if base in seen:
                        raise ValueError(f'Recursive schema base {base}')
                    for base_definition, base_namespaces, base_source in matches:
                        visit(base_definition, base_namespaces, base_source, seen | {base})
                for child in node:
                    visit(child, current_namespaces, current_source, seen)

            visit(definition, namespaces, source, {identifier})
            for label, field_type, constraints in fields:
                key = (label, field_type, json.dumps(constraints, sort_keys=True))
                combined.setdefault(key, {'label': label, 'type': field_type,
                                          'constraints': constraints, 'sources': []})['sources'].append(source)
        return list(combined.values())

    def scalar_dependencies(self, identifier):
        dependencies = set()
        for definition, namespaces, source in self.variants(identifier, 'simpleType'):
            for node in definition.iter():
                for attribute in ('base', 'itemType', 'memberTypes'):
                    for name in node.get(attribute, '').split():
                        dependencies.add(self.resolve(name, namespaces, source))
        return dependencies


def anchor(identifier):
    return identifier.lower().replace(':', '-')


def value_file(identifier):
    prefix = identifier.split(':', 1)[0]
    family = ('word' if prefix == 'w' else 'drawing' if prefix in ('a', 'p', 'wp')
              else 'charts' if prefix in ('c', 'cdr', 'cdr14', 'c14', 'c15', 'cs',
                                         'c16', 'c16ac', 'cx', 'c16r3', 'c16r2')
              else 'shared' if prefix in ('s', 'r') else 'extensions')
    return f'values-{family}.md'


def clause_descriptions():
    descriptions = {}
    for prefix, indexes in CONFIGURATION.get('clause_indexes', {}).items():
        for index in indexes:
            index_path = ROOT / index
            for number, name, description, target in re.findall(
                    r'\[([\d.]+) ([\w]+) \(([^)]+)\)\]\(([^)]+)\)', index_path.read_text()):
                target_path, _, fragment = target.partition('#')
                relative = os.path.relpath((index_path.parent / target_path).resolve(), INVENTORY)
                descriptions.setdefault((prefix, name), set()).add(
                    f'{description}; [§{number}]({relative}#{fragment})')
    # A reused XML name can have different meanings in different containers.
    # Keep only unambiguous clause links; every type still links its exact schema.
    return {key: next(iter(candidates)) for key, candidates in descriptions.items()
            if len(candidates) == 1}


def source_link(source):
    path = source.get('citation_path') or source.get('path') or f"docs/reference/reading/ooxml/microsoft/{source.get('document')}/README.md"
    # Files live in docs/checklists/attributes: two parents reach docs.
    relative = '../../' + path.removeprefix('docs/') if path.startswith('docs/') else '../../../' + path
    return f"[{source['edition']}]({relative})"


def build_documents(collection):
    descriptions = clause_descriptions()
    owners = {}
    projections = {}
    pending = deque()
    for family in CONFIGURATION['families']:
        for root in family['roots']:
            identifier = root['type']
            if identifier in owners:
                raise ValueError(f'Duplicate root: {identifier}')
            owners[identifier] = family['file']
            projections[identifier] = root
            pending.append(identifier)
    fields_by_type = {}
    scalar_types = set()
    while pending:
        identifier = pending.popleft()
        projection = projections.get(identifier, {})
        if not collection.variants(identifier, projection.get('kind', 'complexType')):
            raise ValueError(f'Unresolved complex type: {identifier}')
        fields = collection.fields(identifier, projection.get('fields'),
                                   projection.get('attributes_only', False),
                                   projection.get('kind', 'complexType'))
        fields_by_type[identifier] = fields
        for field in fields:
            dependency = field['type']
            if not dependency:
                continue
            if collection.variants(dependency):
                if dependency not in owners:
                    owners[dependency] = owners[identifier]
                    pending.append(dependency)
            elif collection.variants(dependency, 'simpleType'):
                scalar_types.add(dependency)
            elif not dependency.startswith(('xsd:', 'xml:')):
                raise ValueError(f'Unresolved field type {dependency}')
    pending = deque(scalar_types)
    while pending:
        identifier = pending.popleft()
        for dependency in collection.scalar_dependencies(identifier):
            if collection.variants(dependency, 'simpleType') and dependency not in scalar_types:
                scalar_types.add(dependency)
                pending.append(dependency)
            elif not collection.variants(dependency, 'simpleType') and not dependency.startswith('xsd:'):
                raise ValueError(f'Unresolved scalar base {dependency}')
    documents = {}
    field_identifiers = {identifier + '/' + field['label']
                         for identifier, fields in fields_by_type.items() for field in fields}
    if set(SUPPORT) - field_identifiers:
        raise ValueError(f'Support entries without schema fields: {set(SUPPORT) - field_identifiers}')
    for identifier, support in SUPPORT.items():
        if support['status'] not in ('supported', 'partial') or not support['note']:
            raise ValueError(f'Invalid support record: {identifier}')
        if not support['evidence'] or any(not (ROOT / path).is_file() for path in support['evidence']):
            raise ValueError(f'Missing support evidence: {identifier}')

    def type_link(identifier):
        if not identifier:
            return 'extension wildcard'
        if identifier in owners:
            return f'[{identifier}]({owners[identifier]}#{anchor(identifier)})'
        if identifier in scalar_types:
            return f'[{identifier}]({value_file(identifier)}#{anchor(identifier)})'
        return f'`{identifier}`'

    for family in CONFIGURATION['families']:
        identifiers = [identifier for identifier in fields_by_type if owners[identifier] == family['file']]
        lines = [f"# {family['title']}", '', '[Inventory guide](README.md)', '', family['description'], '',
                 'Checkmarks concern the stated Mote language/model behaviour only. Partial',
                 'mappings stay unchecked. Unannotated fields have no supported Mote mapping;',
                 'shared schema carriers do not independently define authoring attributes.', '',
                 'Nested types and complete value definitions are linked below. XML element',
                 'names and `@attributes` identify specification properties, not new Mote syntax.', '',
                 '## Types', '']
        for identifier in identifiers:
            lines.append(f'- [{identifier}](#{anchor(identifier)})')
        for identifier in identifiers:
            lines.extend(['', f'<a id="{anchor(identifier)}"></a>', '', f'## {identifier}', ''])
            projection = projections.get(identifier, {})
            sources = {source['edition']: source for _, _, source in collection.variants(
                identifier, projection.get('kind', 'complexType'))}
            lines.append('Sources: ' + ', '.join(source_link(source) for source in sources.values()) + '.')
            if identifier in CONFIGURATION.get('supplemental_notes', {}):
                lines.extend(['', CONFIGURATION['supplemental_notes'][identifier]])
            if projection.get('attributes_only'):
                lines.extend(['', 'Container attributes only; child content has its own directive inventories.'])
            if projection.get('fields'):
                lines.extend(['', 'Selected document-level controls: ' + ', '.join(
                    f'`{field}`' for field in projection['fields']) + '.'])
            fields = fields_by_type[identifier]
            lines.append('')
            if not fields:
                lines.append('Empty marker type; no attributes or child properties.')
            for field in fields:
                field_identifier = identifier + '/' + field['label']
                support = SUPPORT.get(field_identifier)
                checked = 'x' if support and support['status'] == 'supported' else ' '
                editions = ', '.join(sorted({source['edition'] for source in field['sources']}))
                constraints = '; '.join(f'{key}={value}' for key, value in field['constraints'].items())
                note = ''
                description = descriptions.get((identifier.split(':', 1)[0], field['label']))
                if description:
                    note = ' ' + description + '.'
                if support:
                    evidence = ', '.join(f'[{Path(path).name}](../../../{path})' for path in support['evidence'])
                    note += f" {support['note']} Evidence: {evidence}."
                lines.append(f"- [{checked}] `{field['label']}` — {type_link(field['type'])}; {editions}"
                             + (f'; {constraints}' if constraints else '') + '.' + note)
        documents[family['file']] = '\n'.join(lines) + '\n'
    value_documents = {}
    for filename in sorted({value_file(identifier) for identifier in scalar_types}):
        value_documents[filename] = ['# ' + filename.removeprefix('values-').removesuffix('.md').title()
                                    + ' value definitions', '', '[Inventory guide](README.md)', '',
                                    'These are complete schema value domains for the selected properties.',
                                    'Property rows record Mote support and supported subsets separately.',
                                    'Identical Strict and Transitional definitions are combined.',
                                    'See also [prose-defined extensions](prose-extensions.md).', '',
                                    '## Types', '']
        for identifier in sorted(scalar_types):
            if value_file(identifier) == filename:
                value_documents[filename].append(f'- [{identifier}](#{anchor(identifier)})')
        value_documents[filename].append('')
    for identifier in sorted(scalar_types):
        lines = value_documents[value_file(identifier)]
        lines.extend([f'<a id="{anchor(identifier)}"></a>', '', f'## {identifier}', ''])
        if identifier in CONFIGURATION.get('supplemental_notes', {}):
            lines.extend([CONFIGURATION['supplemental_notes'][identifier], ''])
        if identifier in CONFIGURATION.get('legacy_value_aliases', {}):
            lines.extend(['Publisher imports use this legacy name; the corresponding shared',
                          'value definition is `' + CONFIGURATION['legacy_value_aliases'][identifier]
                          + '` in the retained ECMA schemas.', ''])
        definitions = {}
        for definition, namespaces, source in collection.variants(identifier, 'simpleType'):
            definition_lines = []
            for node in definition.iter():
                tag = node.tag.removeprefix(SCHEMA_TAG)
                if tag in ('simpleType', 'annotation', 'documentation'):
                    continue
                parts = []
                for name, value in node.attrib.items():
                    if name in ('base', 'itemType', 'memberTypes'):
                        parts.append(name + ': ' + ', '.join(type_link(collection.resolve(
                            token, namespaces, source)) for token in value.split()))
                    else:
                        parts.append(f'{name}: `{value.replace("`", "&#96;")}`')
                definition_lines.append(f'- {tag}: ' + '; '.join(parts))
            definitions.setdefault(tuple(definition_lines), []).append(source)
        for definition_lines, sources in definitions.items():
            lines.extend([', '.join(source_link(source) for source in sources) + ':', ''])
            lines.extend(definition_lines)
            lines.append('')
    for filename, lines in value_documents.items():
        documents[filename] = '\n'.join(lines).rstrip() + '\n'
    documents['values.md'] = '\n'.join(['# Schema value definitions', '',
        '[Inventory guide](README.md)', '', *[f'- [{filename}]({filename})'
        for filename in value_documents]]) + '\n'
    return documents, len(fields_by_type), len(field_identifiers), len(scalar_types)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true', help='refresh schema-derived Markdown')
    arguments = parser.parse_args()
    collection = SchemaCollection()
    documents, type_count, field_count, scalar_count = build_documents(collection)
    mismatches = []
    for filename, content in documents.items():
        path = INVENTORY / filename
        if arguments.write:
            path.write_text(content)
        elif not path.exists() or path.read_text() != content:
            mismatches.append(filename)
    if mismatches:
        raise SystemExit('Inventory differs from its sources or support records: ' + ', '.join(mismatches))
    print(f'{type_count} property types, {field_count} fields, {scalar_count} value types; '
          f'{len(documents)} inventory files ' + ('written.' if arguments.write else 'verified.'))


if __name__ == '__main__':
    main()
