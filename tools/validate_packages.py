#!/usr/bin/env python3
"""Validate the XML parts of DOCX, PPTX and embedded XLSX packages against ECMA-376."""

import argparse
from collections import Counter
import io
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ElementTree
import zipfile


REPOSITORY = Path(__file__).resolve().parents[1]
SCHEMAS = REPOSITORY / "docs/reference/ooxml/ecma-376/part-4/xsd-transitional"
SCHEMA_FOR_NAMESPACE = {
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main": "wml.xsd",
    "http://schemas.openxmlformats.org/presentationml/2006/main": "pml.xsd",
    "http://schemas.openxmlformats.org/spreadsheetml/2006/main": "sml.xsd",
    "http://schemas.openxmlformats.org/drawingml/2006/main": "dml-main.xsd",
    "http://schemas.openxmlformats.org/drawingml/2006/chart": "dml-chart.xsd",
}
COMPATIBILITY = "http://schemas.openxmlformats.org/markup-compatibility/2006"


def namespace(name):
    return name[1:].split("}")[0] if name.startswith("{") else ""


def ignorable_namespaces(data):
    namespaces = set()
    for prefixes in re.findall(rb'Ignorable="([^"]*)"', data):
        for prefix in prefixes.split():
            declaration = re.search(rb"xmlns:" + re.escape(prefix) + rb'="([^"]+)"', data)
            if declaration:
                namespaces.add(declaration.group(1).decode())
    return namespaces


def apply_compatibility(data):
    """Office reads ignorable markup as absent and alternate content as its fallback,
    so the schemas apply only after the same preprocessing."""
    ignorable = ignorable_namespaces(data) | {COMPATIBILITY}
    root = ElementTree.fromstring(data)

    def visit(element):
        for attribute in list(element.attrib):
            if namespace(attribute) in ignorable:
                del element.attrib[attribute]
        children = []
        for child in list(element):
            if child.tag == f"{{{COMPATIBILITY}}}AlternateContent":
                fallback = child.find(f"{{{COMPATIBILITY}}}Fallback")
                children.extend(fallback if fallback is not None else [])
            elif namespace(child.tag) not in ignorable:
                children.append(child)
        for child in list(element):
            element.remove(child)
        for child in children:
            visit(child)
            element.append(child)

    visit(root)
    return root


def validate_part(data):
    root = apply_compatibility(data)
    schema = SCHEMA_FOR_NAMESPACE.get(namespace(root.tag))
    if schema is None:
        return []
    with tempfile.NamedTemporaryFile(suffix=".xml") as part:
        part.write(ElementTree.tostring(root))
        part.flush()
        result = subprocess.run(
            ["xmllint", "--noout", "--schema", str(SCHEMAS / schema), part.name],
            capture_output=True,
            text=True,
        )
    return [
        re.sub(r"^.*?: element ", "element ", line)
        for line in result.stderr.splitlines()
        if "validity error" in line
    ]


def validate_package(data, origin):
    failures = []
    with zipfile.ZipFile(io.BytesIO(data)) as package:
        for name in package.namelist():
            content = package.read(name)
            if name.endswith(".xml"):
                failures.extend((f"{origin}:{name}", error) for error in validate_part(content))
            elif name.endswith(".xlsx"):
                failures.extend(validate_package(content, f"{origin}:{name}"))
    return failures


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packages", nargs="+", type=Path)
    arguments = parser.parse_args()
    failures = []
    for path in arguments.packages:
        failures.extend(validate_package(path.read_bytes(), str(path)))
    for location, error in failures:
        print(f"{location}: {error}")
    distinct = Counter(re.sub(r"Expected is .*", "", error) for _, error in failures)
    print(f"{len(arguments.packages)} packages, {len(failures)} schema errors, {len(distinct)} distinct")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
