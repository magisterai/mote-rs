#!/usr/bin/env python3
"""Verify a frozen font-measurement fixture and compare candidate measurements."""

import argparse
from decimal import Decimal, DecimalException
import hashlib
import json
from pathlib import Path
import re
import sys

POINT_FIELDS = ('layout_width', 'paint_width', 'ascent', 'descent', 'height')
RESULT_FIELDS = {'id', 'font_id', 'variant', 'metric_source', 'prepared_text',
                 'missing_codepoints', *POINT_FIELDS}
VARIANTS = {'regular', 'bold', 'italic', 'bold_italic'}
TOLERANCE = Decimal('0.000000001')


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_json(path):
    def object_fields(pairs):
        result = {}
        for name, value in pairs:
            if name in result:
                raise ValueError(f'duplicate JSON field {name!r}')
            result[name] = value
        return result

    def reject_constant(value):
        raise ValueError(f'non-finite JSON value {value}')

    return json.loads(path.read_text(encoding='utf-8'), parse_float=Decimal,
                      object_pairs_hook=object_fields, parse_constant=reject_constant)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def fields(value, expected, location):
    require(isinstance(value, dict) and value.keys() == expected,
            f'{location}: unexpected or missing fields')


def text(value):
    return isinstance(value, str) and not any(0xD800 <= ord(character) <= 0xDFFF
                                             for character in value)


def number(value):
    return type(value) is int or isinstance(value, Decimal) and value.is_finite()


def identifier(value):
    return isinstance(value, str) and re.fullmatch(r'[a-z][a-z0-9_]*', value)


def local_file(root, relative):
    require(isinstance(relative, str) and relative, 'empty fixture path')
    relative_path = Path(relative)
    require(not relative_path.is_absolute(), 'absolute fixture path')
    path = (root / relative_path).resolve()
    require(path.is_relative_to(root.resolve()), 'fixture path leaves its directory')
    require(path.is_file(), f'missing fixture file: {relative}')
    return path


def verify_digest(path, digest):
    require(isinstance(digest, str) and re.fullmatch('[0-9a-f]{64}', digest),
            f'invalid fingerprint for {path.name}')
    require(fingerprint(path) == digest, f'fingerprint mismatch: {path.name}')


def index_records(records, location):
    require(isinstance(records, list), f'{location}: expected array')
    indexed = {}
    for record in records:
        require(isinstance(record, dict) and identifier(record.get('id')),
                f'{location}: invalid identifier')
        name = record['id']
        require(name not in indexed, f'{location}: duplicate identifier {name}')
        indexed[name] = record
    return indexed


def read_results(path, requests_digest):
    output = read_json(path)
    fields(output, {'schema_version', 'requests_sha256', 'measurements'}, 'results')
    require(type(output['schema_version']) is int and output['schema_version'] == 1,
            'unsupported result schema version')
    require(output['requests_sha256'] == requests_digest, 'result request fingerprint mismatch')
    results = index_records(output['measurements'], 'measurements')
    for name, result in results.items():
        fields(result, RESULT_FIELDS, name)
        require(identifier(result['font_id']), f'{name}: invalid font identifier')
        require(result['variant'] in VARIANTS, f'{name}: invalid font variant')
        require(result['metric_source'] in ('core', 'declared'), f'{name}: invalid metric source')
        require(text(result['prepared_text']), f'{name}: invalid prepared text')
        for field in POINT_FIELDS:
            require(number(result[field]), f'{name}: {field} must be finite numeric points')
        missing = result['missing_codepoints']
        require(isinstance(missing, list) and all(type(value) is int and
                0 <= value <= 0x10FFFF and not 0xD800 <= value <= 0xDFFF
                for value in missing), f'{name}: invalid missing codepoints')
        require(len(missing) == len(set(missing)), f'{name}: duplicate missing codepoints')
    return results


def load_fixture(root):
    root = root.resolve()
    capture = read_json(root / 'capture.json')
    fields(capture, {'schema_version', 'requests', 'requests_sha256', 'expected',
                     'expected_sha256', 'case_count', 'coordinate_unit',
                     'absolute_tolerance_points', 'captured_at_utc',
                     'word_review_status', 'license', 'license_sha256'}, 'capture')
    require(type(capture['schema_version']) is int and capture['schema_version'] == 1,
            'unsupported capture schema version')
    require(capture['coordinate_unit'] == 'point', 'unsupported coordinate unit')
    require(capture['absolute_tolerance_points'] == TOLERANCE,
            'unexpected comparison tolerance')
    require(capture['word_review_status'] == 'unreviewed',
            'measurement capture cannot establish Word review')
    requests_path = local_file(root, capture['requests'])
    expected_path = local_file(root, capture['expected'])
    verify_digest(requests_path, capture['requests_sha256'])
    verify_digest(expected_path, capture['expected_sha256'])
    verify_digest(local_file(root, capture['license']), capture['license_sha256'])
    requests = read_json(requests_path)
    fields(requests, {'schema_version', 'coordinate_unit', 'fonts', 'cases'}, 'requests')
    require(type(requests['schema_version']) is int and requests['schema_version'] == 1,
            'unsupported request schema version')
    require(requests['coordinate_unit'] == 'point', 'request coordinate unit mismatch')
    fonts = index_records(requests['fonts'], 'fonts')
    for name, font in fonts.items():
        expected_fields = {'id', 'family', 'variant', 'format'}
        if font.get('format') != 'core':
            expected_fields |= {'path', 'face_index', 'sha256', 'size_bytes', 'source_url'}
        fields(font, expected_fields, name)
        require(font['format'] in ('core', 'truetype', 'cff'), f'{name}: invalid font format')
        require(text(font['family']) and font['family'], f'{name}: empty font family')
        require(font['variant'] in VARIANTS, f'{name}: invalid font variant')
        if font['format'] == 'core':
            require(font['family'] in ('Helvetica', 'Times', 'Courier'), f'{name}: invalid core family')
        else:
            require(type(font['face_index']) is int and font['face_index'] == 0,
                    f'{name}: only standalone font fixtures are defined')
            path = local_file(root, font['path'])
            verify_digest(path, font['sha256'])
            require(type(font['size_bytes']) is int and path.stat().st_size == font['size_bytes'],
                    f'{name}: font length mismatch')
            require(isinstance(font['source_url'], str) and font['source_url'].startswith('https://'),
                    f'{name}: missing font source URL')
    cases = index_records(requests['cases'], 'cases')
    require(cases and type(capture['case_count']) is int and len(cases) == capture['case_count'],
            'capture case count mismatch')
    for name, request in cases.items():
        fields(request, {'id', 'font_id', 'size_points', 'layout_kerning', 'text'}, name)
        require(request['font_id'] in fonts, f'{name}: unknown requested font')
        require(number(request['size_points']) and request['size_points'] > 0,
                f'{name}: font size must be finite and positive')
        require(type(request['layout_kerning']) is bool, f'{name}: kerning must be boolean')
        require(text(request['text']), f'{name}: invalid source text')
    expected = read_results(expected_path, capture['requests_sha256'])
    require(expected.keys() == cases.keys(), 'expected results do not cover the requested cases')
    for name, result in expected.items():
        request = cases[name]
        font = fonts[request['font_id']]
        require(result['font_id'] == request['font_id'] and result['variant'] == font['variant'],
                f'{name}: captured font identity mismatch')
        require(result['metric_source'] == ('core' if font['format'] == 'core' else 'declared'),
                f'{name}: captured metric source mismatch')
    return capture, requests, expected


def compare(expected, actual):
    differences = []
    for name in sorted(expected.keys() | actual.keys()):
        if name not in actual or name not in expected:
            differences.append({'case': name, 'field': 'id',
                                'reason': 'missing result' if name not in actual else 'unexpected result'})
            continue
        for field in sorted(RESULT_FIELDS - {'id'}):
            first, second = expected[name][field], actual[name][field]
            equal = abs(Decimal(first) - Decimal(second)) <= TOLERANCE if field in POINT_FIELDS else first == second
            if not equal:
                differences.append({'case': name, 'field': field,
                                    'expected': first, 'actual': second})
    return differences


def main(arguments=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--fixture', type=Path, required=True)
    parser.add_argument('--actual', type=Path, help='candidate result JSON; omit to verify fixture integrity only')
    parser.add_argument('--output', type=Path, help='new directory for report.json')
    options = parser.parse_args(arguments)
    if options.output and options.output.exists():
        parser.error('output directory already exists; choose a new path')
    report = {'word_review_status': 'unreviewed', 'status': 'error'}
    try:
        capture, requests, expected = load_fixture(options.fixture)
        report.update(cases=len(expected), fonts=len(requests['fonts']),
                      requests_sha256=capture['requests_sha256'],
                      expected_sha256=capture['expected_sha256'],
                      absolute_tolerance_points=str(TOLERANCE), status='fixture_verified')
        if options.actual:
            before = fingerprint(options.actual)
            actual = read_results(options.actual, capture['requests_sha256'])
            differences = compare(expected, actual)
            require(fingerprint(options.actual) == before, 'candidate results changed during comparison')
            # Revalidate assets as well as requests/results after comparison.
            after_capture, _, _ = load_fixture(options.fixture)
            require(after_capture == capture, 'fixture changed during comparison')
            report.update(status='different' if differences else 'matched',
                          actual_sha256=before, differences=differences)
    except (OSError, ValueError, TypeError, KeyError, DecimalException) as failure:
        report.update(status='error', error=str(failure))
    if options.output:
        options.output.mkdir(parents=True)
        (options.output / 'report.json').write_text(json.dumps(report, indent=2, default=str) + '\n')
    print(json.dumps(report, default=str))
    return 0 if report['status'] in ('fixture_verified', 'matched') else 1


if __name__ == '__main__':
    sys.exit(main())
