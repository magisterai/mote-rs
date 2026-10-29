"""Regression checks for measurement fixtures and candidate comparisons."""

import contextlib
import copy
from decimal import Decimal
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import compare_measurements


class MeasurementComparisonTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        (self.root / 'font.ttf').write_bytes(b'\0\1\0\0fixture')
        (self.root / 'LICENSE').write_text('Fixture license')
        self.requests = {
            'schema_version': 1, 'coordinate_unit': 'point',
            'fonts': [{'id': 'face', 'family': 'Fixture', 'variant': 'regular',
                       'format': 'truetype', 'face_index': 0, 'path': 'font.ttf',
                       'sha256': compare_measurements.fingerprint(self.root / 'font.ttf'),
                       'size_bytes': 11, 'source_url': 'https://example.com/font.ttf'}],
            'cases': [{'id': name, 'font_id': 'face', 'size_points': 12,
                       'layout_kerning': True, 'text': 'AV☃字'} for name in ['first', 'second']],
        }
        self.write('requests.json', self.requests)
        self.expected = {
            'schema_version': 1,
            'requests_sha256': compare_measurements.fingerprint(self.root / 'requests.json'),
            'measurements': [
                {'id': name, 'font_id': 'face', 'variant': 'regular',
                 'metric_source': 'declared', 'prepared_text': 'AV??',
                 'missing_codepoints': [9731, 23383], 'layout_width': 28,
                 'paint_width': 28, 'ascent': 9, 'descent': 3, 'height': 13}
                for name in ['first', 'second']],
        }
        self.write('expected.json', self.expected)
        self.capture = {
            'schema_version': 1, 'requests': 'requests.json',
            'requests_sha256': self.expected['requests_sha256'],
            'expected': 'expected.json',
            'expected_sha256': compare_measurements.fingerprint(self.root / 'expected.json'),
            'case_count': 2, 'coordinate_unit': 'point', 'absolute_tolerance_points': 1e-9,
            'captured_at_utc': '2026-01-01T00:00:00+00:00', 'word_review_status': 'unreviewed',
            'license': 'LICENSE', 'license_sha256': compare_measurements.fingerprint(self.root / 'LICENSE'),
        }
        self.write('capture.json', self.capture)
        self.actual = self.root / 'actual.json'
        self.write('actual.json', self.expected)

    def write(self, name, value):
        (self.root / name).write_text(json.dumps(value, ensure_ascii=False) + '\n')

    def results(self):
        return compare_measurements.read_results(self.actual, self.expected['requests_sha256'])

    def run_command(self, *arguments):
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            status = compare_measurements.main(['--fixture', str(self.root), *arguments])
        return status, json.loads(output.getvalue())

    def test_checked_in_capture_is_complete_and_self_contained(self):
        fixture = Path(__file__).resolve().parents[1] / 'tests/fixtures/measurements'
        capture, requests, results = compare_measurements.load_fixture(fixture)
        self.assertEqual(capture['case_count'], 960)
        self.assertEqual(len(results), 960)
        self.assertEqual(len(requests['fonts']), 20)
        self.assertEqual(sum(font['format'] != 'core' for font in requests['fonts']), 8)

    def test_result_order_and_numeric_spelling_do_not_change_measurement(self):
        expected = self.results()
        candidate = copy.deepcopy(self.expected)
        candidate['measurements'].reverse()
        candidate['measurements'][0]['layout_width'] = 28.0
        self.write('actual.json', candidate)
        self.assertEqual(compare_measurements.compare(expected, self.results()), [])

    def test_point_tolerance_is_tiny_and_applies_only_to_metrics(self):
        expected = self.results()
        candidate = copy.deepcopy(expected)
        candidate['first']['layout_width'] = Decimal(28) + compare_measurements.TOLERANCE
        self.assertEqual(compare_measurements.compare(expected, candidate), [])
        candidate['first']['layout_width'] += Decimal('0.000000000001')
        differences = compare_measurements.compare(expected, candidate)
        self.assertEqual(differences[0]['field'], 'layout_width')
        candidate = copy.deepcopy(expected)
        candidate['first']['missing_codepoints'][0] += 1
        self.assertEqual(compare_measurements.compare(expected, candidate)[0]['field'], 'missing_codepoints')

    def test_font_text_and_missing_glyph_order_are_exact(self):
        expected = self.results()
        for field, value in [('font_id', 'another_face'), ('variant', 'bold'),
                             ('metric_source', 'core'), ('prepared_text', 'AV?'),
                             ('missing_codepoints', [23383, 9731])]:
            with self.subTest(field=field):
                candidate = copy.deepcopy(expected)
                candidate['first'][field] = value
                self.assertEqual(compare_measurements.compare(expected, candidate)[0]['field'], field)

    def test_missing_extra_and_duplicate_cases_cannot_pass(self):
        expected = self.results()
        candidate = copy.deepcopy(expected)
        del candidate['second']
        candidate['extra'] = candidate['first']
        differences = compare_measurements.compare(expected, candidate)
        self.assertEqual({difference['reason'] for difference in differences},
                         {'missing result', 'unexpected result'})
        self.expected['measurements'].append(self.expected['measurements'][0])
        self.write('actual.json', self.expected)
        with self.assertRaisesRegex(ValueError, 'duplicate identifier'):
            self.results()

    def test_changed_requests_expected_fonts_and_license_fail_integrity(self):
        for name in ['requests.json', 'expected.json', 'font.ttf', 'LICENSE']:
            with self.subTest(name=name):
                path = self.root / name
                original = path.read_bytes()
                path.write_bytes(original + b' ')
                try:
                    with self.assertRaisesRegex(ValueError, 'fingerprint mismatch'):
                        compare_measurements.load_fixture(self.root)
                finally:
                    path.write_bytes(original)

    def test_nonfinite_boolean_and_ambiguous_json_are_rejected(self):
        for value in [float('nan'), float('inf'), True, '28']:
            with self.subTest(value=value):
                candidate = copy.deepcopy(self.expected)
                candidate['measurements'][0]['paint_width'] = value
                self.write('actual.json', candidate)
                with self.assertRaises(ValueError):
                    self.results()
        self.actual.write_text('{"schema_version":1,"schema_version":1}')
        with self.assertRaisesRegex(ValueError, 'duplicate JSON field'):
            self.results()
        candidate = copy.deepcopy(self.expected)
        candidate['schema_version'] = True
        self.write('actual.json', candidate)
        with self.assertRaisesRegex(ValueError, 'schema version'):
            self.results()

    def test_unknown_fields_and_changed_request_identity_are_rejected(self):
        candidate = copy.deepcopy(self.expected)
        candidate['measurements'][0]['ignored_field'] = 1
        self.write('actual.json', candidate)
        with self.assertRaisesRegex(ValueError, 'fields'):
            self.results()
        candidate = copy.deepcopy(self.expected)
        candidate['requests_sha256'] = '0' * 64
        self.write('actual.json', candidate)
        with self.assertRaisesRegex(ValueError, 'request fingerprint mismatch'):
            self.results()

    def test_unrepresentable_metric_arithmetic_reports_an_error(self):
        contents = self.actual.read_text().replace('"paint_width": 28', '"paint_width": 1e999999999')
        self.actual.write_text(contents)
        status, report = self.run_command('--actual', str(self.actual))
        self.assertEqual(status, 1)
        self.assertEqual(report['status'], 'error')

    def test_fixture_cannot_reference_absolute_or_escaping_paths(self):
        for relative in [str(self.root / 'font.ttf'), '../font.ttf']:
            with self.subTest(relative=relative), self.assertRaises(ValueError):
                compare_measurements.local_file(self.root, relative)
        with tempfile.TemporaryDirectory() as directory:
            outside = Path(directory) / 'outside.ttf'
            outside.write_bytes(b'outside')
            (self.root / 'linked.ttf').symlink_to(outside)
            with self.assertRaisesRegex(ValueError, 'leaves its directory'):
                compare_measurements.local_file(self.root, 'linked.ttf')

    def test_fixture_verification_does_not_claim_candidate_or_word_conformance(self):
        status, report = self.run_command()
        self.assertEqual(status, 0)
        self.assertEqual(report['status'], 'fixture_verified')
        self.assertEqual(report['word_review_status'], 'unreviewed')
        status, report = self.run_command('--actual', str(self.actual), '--output', str(self.root / 'review'))
        self.assertEqual(status, 0)
        self.assertEqual(report['status'], 'matched')
        self.assertEqual(report['actual_sha256'], compare_measurements.fingerprint(self.actual))
        with self.assertRaises(SystemExit), contextlib.redirect_stderr(io.StringIO()):
            self.run_command('--output', str(self.root / 'review'))

    def test_mismatch_is_failure_and_is_written_to_the_report(self):
        self.expected['measurements'][0]['height'] = 14
        self.write('actual.json', self.expected)
        status, report = self.run_command('--actual', str(self.actual), '--output', str(self.root / 'review'))
        self.assertEqual(status, 1)
        self.assertEqual(report['status'], 'different')
        self.assertEqual(report['differences'][0]['field'], 'height')
        saved = json.loads((self.root / 'review/report.json').read_text())
        self.assertEqual(saved, report)

    def test_candidate_and_fixture_changes_during_comparison_are_errors(self):
        for name in ['actual.json', 'font.ttf']:
            with self.subTest(name=name):
                path = self.root / name
                original = path.read_bytes()
                def change_input(expected, actual):
                    path.write_bytes(original + b' ')
                    return []
                try:
                    with patch('compare_measurements.compare', side_effect=change_input):
                        status, report = self.run_command('--actual', str(self.actual))
                    self.assertEqual(status, 1)
                    self.assertEqual(report['status'], 'error')
                finally:
                    path.write_bytes(original)


if __name__ == '__main__':
    unittest.main()
