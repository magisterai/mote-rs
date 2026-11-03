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

# Each sample names its outputs and its expected native page count. Pure Markdown
# alone is rendered in both layouts, to show one source serving both formats.
SAMPLES = {
    "plain": {"formats": ("docx", "pptx"), "previews": {"document": 1, "presentation": 1}},
    "defaults": {"formats": ("docx",), "previews": {"document": 1}},
    "directives": {"formats": ("docx",), "previews": {"document": 1}},
    "chart": {"formats": ("docx",), "previews": {"document": 1}},
    "lua": {"formats": ("docx",), "previews": {"document": 1}},
    "report": {"formats": ("docx", "pdf"), "previews": {"document": 1}},
    "deck": {"formats": ("pptx", "pdf"), "previews": {"presentation": 4}},
}


def source_for(sample):
    if sample == "plain":
        return (SOURCES / "plain.md").read_text()
    if sample == "defaults":
        return (SOURCES / "defaults.mote").read_text() + "\n" + (SOURCES / "plain.md").read_text()
    return (SOURCES / (sample + ".mote")).read_text()


def expected_warning(sample, warning):
    # Pure Markdown deliberately uses the engine's fallback font outlines, and
    # Markdown heading markers and repeated table headers have no slide appearance.
    if sample == "plain":
        return (
            "fallback" in warning or "substituted" in warning
            or warning.endswith((
                "heading.marker is retained without a presentation output effect",
                "row.repeat_header is retained without a presentation output effect",
            ))
        )
    return False


def generate(executable, sample, output_format, source, destination, layout=None):
    arguments = [str(executable), "--format", output_format, "-o", str(destination)]
    if sample == "lua":
        arguments.append("--lua")
    if output_format == "png":
        arguments.extend(["--dpi", "144"])
    if layout:
        arguments.extend(["--layout", layout])
    result = subprocess.run(
        arguments + ["-"], input=source, text=True, capture_output=True, cwd=SOURCES
    )
    if result.returncode:
        raise RuntimeError(f"{sample} {output_format}: {result.stderr.strip()}")
    warnings = result.stderr.strip().splitlines()
    unexpected = [warning for warning in warnings if not expected_warning(sample, warning)]
    if unexpected:
        raise RuntimeError("\n".join(unexpected))
    return warnings


def verify_package(path, slides_expected):
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
            if len(slides) != slides_expected:
                raise RuntimeError(f"Expected {slides_expected} slides in {path}, found {len(slides)}")


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
        for sample, outputs in SAMPLES.items():
            source = source_for(sample)
            previews = outputs["previews"]
            for output_format in outputs["formats"]:
                layout = next(iter(previews)) if output_format == "pdf" else None
                package = staging / (sample + "." + output_format)
                warnings.update(generate(executable, sample, output_format, source, package, layout))
                if output_format != "pdf":
                    verify_package(package, previews.get("presentation", 1))
                publications.append((package, downloads / package.name))
            for layout, page_count in previews.items():
                primary = layout == next(iter(previews))
                suffix = "" if primary else "-slides"
                prefix = staging / (sample + suffix)
                warnings.update(generate(executable, sample, "png", source, prefix, layout))
                pages = sorted(staging.glob(prefix.name + "-[0-9]*.png"))
                if len(pages) != page_count:
                    raise RuntimeError(f"Expected {page_count} previews for {prefix.name}, found {len(pages)}")
                publications.extend((page, images / page.name) for page in pages)
            print(f"{sample}: generated {', '.join(outputs['formats']).upper()} and native previews")
        for source, destination in publications:
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
    for warning in sorted(warnings):
        print(warning)
    print("Inspect all native previews; Office visual review is separate.")


if __name__ == "__main__":
    main()
