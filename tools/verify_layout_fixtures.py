#!/usr/bin/env python3
"""Verify frozen native layout captures without generating new expectations."""
import argparse
import hashlib
import json
import math
from pathlib import Path

COLLECTIONS = (
    "lines", "layout-styles", "paragraph-text", "paragraph-flow",
    "page-geometry", "document-text", "text-decorations",
)


def unique_object(pairs):
    result = {}
    for name, value in pairs:
        if name in result:
            raise ValueError(f"duplicate JSON field: {name}")
        result[name] = value
    return result


def parse_json(text):
    def invalid_constant(value):
        raise ValueError(f"non-finite JSON constant: {value}")
    def finite_float(value):
        number = float(value)
        if not math.isfinite(number):
            raise ValueError("non-finite JSON number")
        return number
    return json.loads(text, object_pairs_hook=unique_object,
                      parse_constant=invalid_constant, parse_float=finite_float)


def read_json(path):
    return parse_json(path.read_text(encoding="utf-8"))


def case_ids(document):
    if document.get("schema_version") != 1 or type(document.get("schema_version")) is not int:
        raise ValueError("unsupported capture schema")
    cases = document.get("cases")
    if not isinstance(cases, list):
        raise ValueError("cases must be an array")
    if any(not isinstance(case, dict) for case in cases):
        raise ValueError("cases must contain objects")
    identities = [case.get("id") for case in cases]
    if any(not isinstance(identity, str) or not identity for identity in identities):
        raise ValueError("case identities must be nonempty strings")
    if len(set(identities)) != len(identities):
        raise ValueError("duplicate case identity")
    return identities


def verify_fixture(directory, fixture_root):
    capture = read_json(directory / "capture.json")
    if type(capture.get("schema_version")) is not int or capture["schema_version"] != 1:
        raise ValueError("unsupported capture metadata schema")
    if type(capture.get("case_count")) is not int or capture["case_count"] <= 0:
        raise ValueError("case count must be a positive integer")
    if capture.get("units") != "points" or capture.get("absolute_tolerance") != 1e-9:
        raise ValueError("unexpected measurement units or tolerance")
    verified = {}
    for name, identity in capture["files"].items():
        path = (directory / name).resolve()
        path.relative_to(fixture_root.resolve())
        data = path.read_bytes()
        if len(data) != identity["bytes"] or hashlib.sha256(data).hexdigest() != identity["sha256"]:
            raise ValueError(f"fingerprint differs: {name}")
        verified[name] = parse_json(data.decode("utf-8"))
    request_names = [name for name in verified if name.endswith("requests.json")]
    if len(request_names) != 1 or "expected.json" not in verified:
        raise ValueError("one request document and expected.json are required")
    requests = case_ids(verified[request_names[0]])
    expected = case_ids(verified["expected.json"])
    widths = capture.get("widths_points")
    if widths is not None:
        if not isinstance(widths, list) or not widths or any(type(width) not in (int, float) or width <= 0 for width in widths):
            raise ValueError("widths must be positive numbers")
        if len(set(widths)) != len(widths):
            raise ValueError("duplicate width")
        requests = [f"{identity}-{width:g}" for identity in requests for width in widths]
    if set(requests) != set(expected) or len(expected) != capture["case_count"]:
        raise ValueError("captured case coverage differs from the requests")
    if "rejected_plans" in capture:
        rejected = sum(case.get("invalid_line_height") is True for case in verified["expected.json"]["cases"])
        if rejected != capture["rejected_plans"] or len(expected) - rejected != capture["successful_plans"]:
            raise ValueError("successful and rejected plan counts differ")
    return len(expected)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixtures", type=Path, default=Path(__file__).resolve().parents[1] / "tests/fixtures")
    arguments = parser.parse_args()
    total = 0
    for name in COLLECTIONS:
        try:
            count = verify_fixture(arguments.fixtures / name, arguments.fixtures)
        except (OSError, ValueError, KeyError, TypeError) as error:
            parser.exit(1, f"{name}: {error}\n")
        total += count
    print(json.dumps({"status": "fixture_verified", "collections": len(COLLECTIONS), "cases": total}))


if __name__ == "__main__":
    main()
