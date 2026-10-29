import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from verify_layout_fixtures import verify_fixture


class LayoutFixtureIntegrityTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.directory = self.root / "lines"
        self.directory.mkdir()
        document = {"schema_version": 1, "cases": [{"id": "example"}]}
        for name in ["requests.json", "expected.json"]:
            (self.directory / name).write_text(json.dumps(document))
        self.capture = {"schema_version": 1, "case_count": 1, "units": "points", "absolute_tolerance": 1e-9, "files": {}}
        self.refresh()

    def refresh(self):
        for name in ["requests.json", "expected.json"]:
            data = (self.directory / name).read_bytes()
            self.capture["files"][name] = {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}
        (self.directory / "capture.json").write_text(json.dumps(self.capture))

    def test_unchanged_capture_verifies(self):
        self.assertEqual(verify_fixture(self.directory, self.root), 1)

    def test_changed_bytes_fail_before_comparison(self):
        with (self.directory / "expected.json").open("a") as output:
            output.write(" ")
        with self.assertRaisesRegex(ValueError, "fingerprint"):
            verify_fixture(self.directory, self.root)

    def test_duplicate_fields_are_rejected_even_with_matching_fingerprint(self):
        (self.directory / "expected.json").write_text('{"schema_version":1,"schema_version":1,"cases":[{"id":"example"}]}')
        self.refresh()
        with self.assertRaisesRegex(ValueError, "duplicate JSON"):
            verify_fixture(self.directory, self.root)

    def test_case_identity_must_cover_the_request(self):
        (self.directory / "expected.json").write_text(json.dumps({"schema_version": 1, "cases": [{"id": "different"}]}))
        self.refresh()
        with self.assertRaisesRegex(ValueError, "coverage"):
            verify_fixture(self.directory, self.root)


if __name__ == "__main__":
    unittest.main()
