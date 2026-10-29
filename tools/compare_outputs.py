#!/usr/bin/env python3
"""Compare evaluated JSON and DOCX packages from two explicit Mote executables."""

import argparse
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import xml.etree.ElementTree as ElementTree
import zipfile

WORD_NAMESPACE = 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'
DATE_NAMESPACE = 'http://purl.org/dc/terms/'
XML_NAMESPACE = 'http://www.w3.org/XML/1998/namespace'
ADVISORY_BREAK = '{' + WORD_NAMESPACE + '}lastRenderedPageBreak'
TIMESTAMPS = {'{' + DATE_NAMESPACE + '}' + name for name in ('created', 'modified')}


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_model(path):
    def reject_constant(value):
        raise ValueError(f'non-finite JSON number: {value}')

    return json.loads(path.read_text(), parse_int=Decimal, parse_float=Decimal,
                      parse_constant=reject_constant)


def model_difference(reference, candidate, location='$'):
    """Return the first changed field; booleans never compare equal to numbers."""
    if type(reference) is not type(candidate):
        return location + ': value type'
    if isinstance(reference, dict):
        if reference.keys() != candidate.keys():
            return location + ': object fields'
        for name in sorted(reference):
            difference = model_difference(reference[name], candidate[name],
                                          location + '.' + name)
            if difference:
                return difference
    elif isinstance(reference, list):
        if len(reference) != len(candidate):
            return location + ': array length'
        for index, (first, second) in enumerate(zip(reference, candidate)):
            difference = model_difference(first, second, f'{location}[{index}]')
            if difference:
                return difference
    elif reference != candidate:
        return location + ': value'
    return None


def canonical_xml(element, preserve_space=False):
    preserve_space = element.attrib.get('{' + XML_NAMESPACE + '}space',
                                       'preserve' if preserve_space else 'default') == 'preserve'
    text = element.text or ''
    if element.tag in TIMESTAMPS:
        text = '<generation timestamp>'
    elif not preserve_space and not text.strip() and element.tag != '{' + WORD_NAMESPACE + '}t':
        text = ''
    children = []
    for child in element:
        if child.tag != ADVISORY_BREAK:
            children.append(canonical_xml(child, preserve_space))
        tail = child.tail or ''
        if tail.strip() or (preserve_space and tail):
            children.append(('#tail', tail))
    return (element.tag, tuple(sorted(element.attrib.items())), text, tuple(children))


def read_package(path):
    parts = {}
    advisory_counts = {}
    with zipfile.ZipFile(path) as archive:
        for entry in archive.infolist():
            if entry.filename in parts:
                raise ValueError(f'duplicate package part: {entry.filename}')
            data = archive.read(entry)
            if entry.filename.endswith(('.xml', '.rels')):
                element = ElementTree.fromstring(data)
                parts[entry.filename] = ('xml', canonical_xml(element))
                count = sum(node.tag == ADVISORY_BREAK for node in element.iter())
                if count:
                    advisory_counts[entry.filename] = count
            else:
                parts[entry.filename] = ('bytes', hashlib.sha256(data).hexdigest())
    return parts, advisory_counts


def compare_case(source, destination, executables, timeout):
    destination.mkdir()
    snapshot = destination / 'source.mote'
    snapshot.write_bytes(source.read_bytes())
    result = {'source': str(source), 'source_sha256': fingerprint(snapshot),
              'artifacts': str(destination), 'status': 'error', 'diagnostics': {}}
    try:
        for label, executable in executables.items():
            result['diagnostics'][label] = {}
            for output_format in ('json', 'docx'):
                output = destination / f'{label}.{output_format}'
                # Keep source-relative resolution anchored to the original input.
                command = [str(executable), '-format', output_format, '-o', str(output), str(source)]
                completed = subprocess.run(command, capture_output=True, text=True,
                                           timeout=timeout, check=False)
                if completed.stderr:
                    result['diagnostics'][label][output_format] = completed.stderr.strip()
                if completed.returncode:
                    result['error'] = {'executable': label, 'format': output_format,
                                       'exit_code': completed.returncode}
                    return result
                if not output.is_file():
                    raise ValueError(f'{label} did not create {output_format} output')
                if fingerprint(source) != result['source_sha256']:
                    raise ValueError('source changed during comparison')
        differences = []
        model = model_difference(read_model(destination / 'reference.json'),
                                 read_model(destination / 'candidate.json'))
        if model:
            differences.append({'part': 'model.json', 'location': model})
        reference, reference_advisory = read_package(destination / 'reference.docx')
        candidate, candidate_advisory = read_package(destination / 'candidate.docx')
        for name in sorted(reference.keys() | candidate.keys()):
            if reference.get(name) != candidate.get(name):
                differences.append({'part': name})
        result['ignored_advisory_markers'] = {'reference': reference_advisory,
                                             'candidate': candidate_advisory}
        result['differences'] = differences
        result['status'] = 'different' if differences else 'matched'
    except (OSError, ValueError, ElementTree.ParseError, zipfile.BadZipFile,
            subprocess.TimeoutExpired) as error:
        result['error'] = {'message': str(error)}
    return result


def main(arguments=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference', required=True, type=Path)
    parser.add_argument('--candidate', required=True, type=Path)
    parser.add_argument('--sources', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path,
                        help='new directory for source snapshots, packages, and report.json')
    parser.add_argument('--recursive', action='store_true')
    parser.add_argument('--workers', type=int, default=4)
    parser.add_argument('--timeout', type=float, default=30,
                        help='deadline in seconds for each emitter process')
    options = parser.parse_args(arguments)
    if options.workers < 1 or options.timeout <= 0:
        parser.error('workers and timeout must be positive')
    executables = {name: getattr(options, name).resolve()
                   for name in ('reference', 'candidate')}
    for name, executable in executables.items():
        if not executable.is_file():
            parser.error(f'{name} executable is not a file: {executable}')
    executable_fingerprints = {name: fingerprint(path) for name, path in executables.items()}
    source_root = options.sources.absolute()
    sources = sorted(source_root.rglob('*.mote') if options.recursive
                     else source_root.glob('*.mote'))
    if not sources:
        parser.error('no .mote sources found')
    destination = options.output.resolve()
    if destination.exists():
        parser.error('output directory already exists; choose a new path')
    destination.mkdir(parents=True)
    jobs = [(source, destination / f'{index:05d}-{source.stem}')
            for index, source in enumerate(sources)]
    with ThreadPoolExecutor(max_workers=options.workers) as executor:
        results = list(executor.map(lambda job: compare_case(*job, executables, options.timeout), jobs))
    counts = {status: sum(result['status'] == status for result in results)
              for status in ('matched', 'different', 'error')}
    integrity_errors = [f'{name} executable changed during comparison'
                        for name, path in executables.items()
                        if fingerprint(path) != executable_fingerprints[name]]
    report = {
        'created_at_utc': datetime.now(timezone.utc).isoformat(),
        'executables': {name: {'path': str(path), 'sha256': executable_fingerprints[name]}
                        for name, path in executables.items()},
        'normalization': ['JSON numbers compare numerically; booleans retain their type.',
                          'XML names compare by namespace URI, with attribute order ignored.',
                          'Generation created/modified timestamps are ignored.',
                          'Formatting whitespace is ignored outside text and xml:space="preserve".',
                          'Advisory lastRenderedPageBreak elements are counted and ignored.',
                          'All other element order, values, text, and package parts are compared.'],
        'word_review_status': 'unreviewed',
        'input_integrity_errors': integrity_errors,
        'summary': {'cases': len(results), **counts}, 'cases': results,
    }
    (destination / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'report': str(destination / 'report.json'), **report['summary']}))
    return 1 if counts['different'] or counts['error'] or integrity_errors else 0


if __name__ == '__main__':
    sys.exit(main())
