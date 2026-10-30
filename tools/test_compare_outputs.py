"""Regression checks for the structural comparison boundary, using only stdlib."""

from decimal import Decimal
import json
import io
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch
import warnings
import xml.etree.ElementTree as ElementTree
import zipfile

import compare_outputs


class ComparisonTests(unittest.TestCase):
    def test_numeric_spelling_matches_but_boolean_types_do_not(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'model.json'
            path.write_text('{"value": 2.00}')
            first = compare_outputs.read_model(path)
            path.write_text('{"value": 2}')
            self.assertIsNone(compare_outputs.model_difference(first, compare_outputs.read_model(path)))
            self.assertIsNotNone(compare_outputs.model_difference({'value': True}, {'value': Decimal(1)}))
            path.write_text('{"value": NaN}')
            with self.assertRaises(ValueError):
                compare_outputs.read_model(path)

    def test_xml_prefixes_and_indentation_do_not_hide_changed_geometry(self):
        first = ElementTree.fromstring('<w:p xmlns:w="urn:word"><w:r width="20"/></w:p>')
        second = ElementTree.fromstring('<a:p xmlns:a="urn:word">\n  <a:r width="20"/>\n</a:p>')
        self.assertEqual(compare_outputs.canonical_xml(first), compare_outputs.canonical_xml(second))
        second[0].set('width', '21')
        self.assertNotEqual(compare_outputs.canonical_xml(first), compare_outputs.canonical_xml(second))

    def test_preserved_whitespace_order_and_tail_text_remain_significant(self):
        for first, second in [
            ('<p xml:space="preserve"> </p>', '<p xml:space="preserve">  </p>'),
            ('<p><first/><second/></p>', '<p><second/><first/></p>'),
            ('<p><run/>tail</p>', '<p><run/>changed</p>'),
        ]:
            self.assertNotEqual(compare_outputs.canonical_xml(ElementTree.fromstring(first)),
                                compare_outputs.canonical_xml(ElementTree.fromstring(second)))

    def test_timestamp_normalization_does_not_ignore_other_metadata(self):
        first = ElementTree.fromstring('<core xmlns:d="http://purl.org/dc/terms/"><d:created>one</d:created><title>Title</title></core>')
        second = ElementTree.fromstring('<core xmlns:d="http://purl.org/dc/terms/"><d:created>two</d:created><title>Title</title></core>')
        self.assertEqual(compare_outputs.canonical_xml(first), compare_outputs.canonical_xml(second))
        second[1].text = 'Changed'
        self.assertNotEqual(compare_outputs.canonical_xml(first), compare_outputs.canonical_xml(second))

    def test_only_text_split_by_an_advisory_marker_is_joined(self):
        namespace = compare_outputs.WORD_NAMESPACE
        def canonical(body):
            return compare_outputs.canonical_xml(ElementTree.fromstring(f'<w:r xmlns:w="{namespace}">{body}</w:r>'))
        whole = canonical('<w:t xml:space="preserve">one two</w:t>')
        split = canonical('<w:t xml:space="preserve">one </w:t><w:lastRenderedPageBreak/><w:t xml:space="preserve">two</w:t>')
        self.assertEqual(whole, split)
        for separator in ['', '<w:br/>', '<w:tab/>']:
            self.assertNotEqual(whole, canonical('<w:t xml:space="preserve">one </w:t>' + separator + '<w:t xml:space="preserve">two</w:t>'))
        self.assertNotEqual(whole, canonical('<w:t>one </w:t><w:lastRenderedPageBreak/><w:t xml:space="preserve">two</w:t>'))

    def test_packages_count_advisory_breaks_and_compare_binary_parts(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'test.docx'
            with zipfile.ZipFile(path, 'w') as archive:
                archive.writestr('word/document.xml', f'<w:document xmlns:w="{compare_outputs.WORD_NAMESPACE}"><w:lastRenderedPageBreak/><w:br w:type="page"/></w:document>')
                archive.writestr('word/media/image.png', b'pixels')
            parts, counts = compare_outputs.read_package(path)
            self.assertEqual(counts, {'word/document.xml': 1})
            self.assertEqual(parts['word/media/image.png'][0], 'bytes')
            self.assertIn('br', str(parts['word/document.xml']))
            with zipfile.ZipFile(path, 'a') as archive, warnings.catch_warnings():
                warnings.simplefilter('ignore', UserWarning)
                archive.writestr('word/document.xml', '<changed/>')
            with self.assertRaisesRegex(ValueError, 'duplicate package part'):
                compare_outputs.read_package(path)

    def test_embedded_workbooks_compare_contents_not_archive_encoding(self):
        def package(value, reverse=False, duplicate=False):
            workbook = io.BytesIO()
            entries = [('xl/worksheets/sheet1.xml', '<sheet><cell>' + value + '</cell></sheet>'),
                       ('xl/styles.xml', '<styles/>')]
            if reverse:
                entries.reverse()
            with zipfile.ZipFile(workbook, 'w', compression=(zipfile.ZIP_DEFLATED if reverse else zipfile.ZIP_STORED)) as archive:
                for name, contents in entries:
                    archive.writestr(name, contents)
                if duplicate:
                    with warnings.catch_warnings():
                        warnings.simplefilter('ignore', UserWarning)
                        archive.writestr('xl/styles.xml', '<other/>')
            document = io.BytesIO()
            with zipfile.ZipFile(document, 'w') as archive:
                archive.writestr('word/embeddings/chart1.xlsx', workbook.getvalue())
            document.seek(0)
            return document
        original = compare_outputs.read_package(package('4'))
        self.assertEqual(original, compare_outputs.read_package(package('4', reverse=True)))
        self.assertNotEqual(original, compare_outputs.read_package(package('5')))
        with self.assertRaisesRegex(ValueError, 'duplicate package part'):
            compare_outputs.read_package(package('4', duplicate=True))
        with self.assertRaisesRegex(ValueError, 'nesting'):
            compare_outputs.read_package(package('4'), depth=5)

    def test_failed_emitters_and_timeouts_cannot_be_reported_as_matches(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'case.mote'
            source.write_text('Text')
            executables = {'reference': Path(sys.executable), 'candidate': Path(sys.executable)}
            result = compare_outputs.compare_case(source, root / 'failure', executables, 5)
            self.assertEqual(result['status'], 'error')
            self.assertEqual(result['error']['executable'], 'reference')
            import subprocess
            with patch('compare_outputs.subprocess.run', side_effect=subprocess.TimeoutExpired('emitter', 0.1)):
                result = compare_outputs.compare_case(source, root / 'timeout', executables, 0.1)
            self.assertEqual(result['status'], 'error')
            self.assertIn('timed out', result['error']['message'])

    def test_command_writes_a_traceable_report_and_refuses_existing_destinations(self):
        import contextlib
        import io
        import subprocess
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            sources = root / 'sources'
            sources.mkdir()
            source = sources / 'case.mote'
            source.write_text('Text')
            executable = Path(sys.executable)
            def produce_output(command, **options):
                self.assertEqual(Path(command[-1]), source)
                output = Path(command[4])
                if command[2] == 'json':
                    output.write_text('{"content": []}')
                else:
                    with zipfile.ZipFile(output, 'w') as archive:
                        archive.writestr('word/document.xml', '<document/>')
                return subprocess.CompletedProcess(command, 0, '', '')
            arguments = ['--reference', str(executable), '--candidate', str(executable),
                         '--sources', str(sources), '--output', str(root / 'report')]
            with patch('compare_outputs.subprocess.run', side_effect=produce_output), contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(compare_outputs.main(arguments), 0)
            report = json.loads((root / 'report/report.json').read_text())
            self.assertEqual(report['summary'], {'cases': 1, 'matched': 1, 'different': 0, 'error': 0})
            self.assertEqual(report['word_review_status'], 'unreviewed')
            self.assertEqual(report['cases'][0]['source_sha256'], compare_outputs.fingerprint(source))
            self.assertEqual((root / 'report/00000-case/source.mote').read_text(), 'Text')
            with self.assertRaises(SystemExit), contextlib.redirect_stderr(io.StringIO()):
                compare_outputs.main(arguments)

    def test_changed_sources_are_errors_even_when_the_emitter_succeeds(self):
        import subprocess
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'case.mote'
            source.write_text('Before')
            def change_source(command, **options):
                Path(command[4]).write_text('{}')
                source.write_text('After')
                return subprocess.CompletedProcess(command, 0, '', '')
            with patch('compare_outputs.subprocess.run', side_effect=change_source):
                result = compare_outputs.compare_case(source, root / 'result',
                    {'reference': Path(sys.executable), 'candidate': Path(sys.executable)}, 5)
            self.assertEqual(result['status'], 'error')
            self.assertIn('source changed', result['error']['message'])


if __name__ == '__main__':
    unittest.main()
