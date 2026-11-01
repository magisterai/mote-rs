#!/usr/bin/env python3
"""Generate the README's editable downloads and native previews with Mote."""

import argparse
from pathlib import Path
import shutil
import subprocess
import tempfile
import xml.etree.ElementTree as ElementTree
import zipfile


REPOSITORY = Path(__file__).resolve().parents[1]
SOURCES = REPOSITORY / "examples/readme"
SAMPLES = ("plain", "defaults", "directives", "presentation", "chart", "lua")


def source_for(sample, presentation):
    if sample == "plain":
        return (SOURCES / "plain.md").read_text()
    if sample == "defaults":
        source = (SOURCES / "defaults.mote").read_text() + "\n"
        source += (SOURCES / "plain.md").read_text()
    else:
        name = "presentation-document" if sample == "presentation" and not presentation else sample
        source = (SOURCES / (name + ".mote")).read_text()
    if presentation and sample != "presentation":
        source += "\n" + (SOURCES / "slide-defaults.mote").read_text()
    return source


def expected_warning(sample, warning):
    if sample == "plain" and ("fallback" in warning or "substituted" in warning):
        return True
    metadata = (
        "heading.marker", "row.repeat_header", "document.default_size",
        "document.margin", "document.page",
    )
    return any(warning.endswith(
        f"{property_name} is retained without a presentation output effect"
    ) for property_name in metadata)


def generate(executable, sample, output_format, source, destination, presentation):
    arguments = [str(executable), "--format", output_format, "-o", str(destination)]
    if sample == "lua":
        arguments.append("--lua")
    if output_format == "png":
        arguments.extend(["--dpi", "144", "--layout", "presentation" if presentation else "document"])
    result = subprocess.run(
        arguments + ["-"], input=source, text=True, capture_output=True, cwd=SOURCES
    )
    if result.returncode:
        raise RuntimeError(f"{sample} {output_format}: {result.stderr.strip()}")
    # Markdown markers and repeating document headers have no slide appearance.
    # Pure Markdown also deliberately uses the engine's fallback font outlines.
    warnings = result.stderr.strip().splitlines()
    unexpected = [warning for warning in warnings if not expected_warning(sample, warning)]
    if unexpected:
        raise RuntimeError("\n".join(unexpected))
    return warnings


def verify_package(path):
    with zipfile.ZipFile(path) as package:
        if package.testzip() is not None:
            raise RuntimeError(f"Invalid archive: {path}")
        for name in package.namelist():
            if name.endswith((".xml", ".rels")):
                ElementTree.fromstring(package.read(name))
        if path.suffix == ".pptx":
            slides = [name for name in package.namelist() if (
                name.startswith("ppt/slides/slide") and name.endswith(".xml")
            )]
            if len(slides) != 1:
                raise RuntimeError(f"Expected one slide in {path}, found {len(slides)}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mote", type=Path, default=REPOSITORY / "target/release/mote")
    arguments = parser.parse_args()
    executable = arguments.mote.resolve()
    if not executable.is_file():
        parser.error("build the mote binary first, or provide --mote PATH")
    temporary_root = REPOSITORY / "target/readme"
    temporary_root.mkdir(parents=True, exist_ok=True)
    downloads = REPOSITORY / "docs/examples/readme"
    images = REPOSITORY / "docs/images/readme"
    publications = []
    warnings = set()
    with tempfile.TemporaryDirectory(prefix="generation-", dir=temporary_root) as temporary:
        staging = Path(temporary)
        for sample in SAMPLES:
            for presentation, output_format in ((False, "docx"), (True, "pptx")):
                source = source_for(sample, presentation)
                package = staging / (sample + "." + output_format)
                warnings.update(generate(executable, sample, output_format, source, package, presentation))
                verify_package(package)
                publications.append((package, downloads / package.name))
                primary = presentation == (sample == "presentation")
                suffix = "" if primary else "-slides" if presentation else "-document"
                prefix = staging / (sample + suffix)
                warnings.update(generate(executable, sample, "png", source, prefix, presentation))
                pages = list(staging.glob(prefix.name + "-[0-9]*.png"))
                if len(pages) != 1:
                    raise RuntimeError(f"Expected one preview for {prefix.name}, found {len(pages)}")
                publications.append((pages[0], images / pages[0].name))
            print(f"{sample}: generated DOCX, PPTX and both native previews")
        for source, destination in publications:
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
    for warning in sorted(warnings):
        print(warning)
    print("Inspect all native previews; Office visual review is separate.")


if __name__ == "__main__":
    main()
