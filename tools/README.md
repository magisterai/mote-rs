# Repository tools

## Attribute inventory

`python3 tools/attribute_inventory.py` checks the generated support inventories
against retained local schemas. Use `--write` after changing `scope.json` or
`support.json`. The tool does not retrieve sources from the network.

## Output comparison

`compare_outputs.py` runs two explicitly selected Mote executables over the same
source cases and compares their JSON models and every DOCX package part. Build the
candidate first so the comparison uses the intended code:

```sh
cargo build
python3 tools/compare_outputs.py \
  --reference /path/to/reference-emitter \
  --candidate target/debug/mote \
  --sources tests/fixtures/docx \
  --recursive \
  --output target/comparisons/review-001
```

The output directory must be new. It contains source snapshots, both models and
packages, diagnostics, executable/source fingerprints, and `report.json`. Source
resolution stays anchored to the original input path. Changed inputs or executables
invalidate the comparison. The command returns nonzero for differences, emitter
failures, timeouts or input-integrity failures; inspect the report to distinguish
them. It does not modify source cases or approved references.

Defaults are four workers and a 30-second deadline for each emitter process;
`--workers` and `--timeout` override them. This command never opens Word or another
renderer. It cannot establish font, wrapping, pagination or visual conformance.
Every report explicitly records Word review as unreviewed.

Comparison rules are deliberately narrow:

- Numeric JSON spellings compare by value; booleans retain their own type.
- XML names use namespace URIs and attributes ignore order. Element order, values,
  text and preserved whitespace remain significant.
- Created/modified metadata timestamps and XML formatting whitespace are ignored.
- Advisory `lastRenderedPageBreak` markers are counted per package part and ignored.
  Authored `w:br` elements remain compared.
- Binary parts compare by SHA-256, and duplicate ZIP entries are errors.

Promote a confirmed mismatch into a focused frozen fixture and regression test.
Do not change fixture text or geometry to conceal a rendering difference. Keep
intentional support boundaries explicit rather than labelling all differences as
passes. The permanent fixture comparison tests also require each advisory-marker
omission to be inventoried in its capture metadata.

Run the comparison tool's regression checks with:

```sh
python3 -m unittest discover -s tools -p 'test_compare_outputs.py'
```
