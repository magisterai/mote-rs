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
  Text pieces separated only by such a marker within one run are joined when
  their text attributes agree. Authored `w:br`, tabs, formatting changes and real
  text-node boundaries remain compared.
- Embedded `.xlsx` workbooks compare recursively by package part, using these
  same XML and binary rules. ZIP compression and entry order are immaterial;
  workbook cells, formulas, styles and relationships remain significant.
- Other binary parts compare by SHA-256, and duplicate ZIP entries are errors,
  including inside embedded workbooks. Nesting beyond four levels is rejected.

Promote a confirmed mismatch into a focused frozen fixture and regression test.
Do not change fixture text or geometry to conceal a rendering difference. Keep
intentional support boundaries explicit rather than labelling all differences as
passes. The permanent fixture comparison tests also require each advisory-marker
omission to be inventoried in its capture metadata.

Run the comparison tool's regression checks with:

```sh
python3 -m unittest discover -s tools -p 'test_compare_outputs.py'
```

## Font measurement comparison

`compare_measurements.py` validates the self-contained
[measurement fixtures](../tests/fixtures/measurements/README.md) and compares
candidate measurements against the frozen expectations. It uses only Python's
standard library and does not run a renderer or locate installed fonts.

```sh
python3 tools/compare_measurements.py --fixture tests/fixtures/measurements
python3 tools/compare_measurements.py \
  --fixture tests/fixtures/measurements \
  --actual target/measurements/candidate.json \
  --output target/measurements/review-001
```

The first command checks integrity only. The second requires independently
produced result JSON in the fixture's documented format and writes a comparison
report to a new directory. Font, request, expectation and license fingerprints
must match. Cases and categorical fields compare exactly; metric tolerance is
fixed at one billionth of a point. Missing cases, mismatches and invalid inputs
return nonzero. The command never updates the baseline or claims Word review.

Run all comparison tool regression checks with:

```sh
python3 -m unittest discover -s tools -p 'test_compare_*.py'
```

## Native layout captures

Verify the seven frozen layout collections, including file fingerprints and exact
case coverage, without running a renderer:

```sh
python3 tools/verify_layout_fixtures.py
```

This checks 1,542 cases across lines, styles, paragraph text, page geometry,
paragraph flow, document text and inline decoration. It rejects changed bytes,
duplicate JSON fields/case identities, non-finite numbers and missing cases.
`fixture_verified` confirms capture integrity, not candidate output agreement.
`cargo test` compares the Rust implementation with these frozen expectations.
The separate font-measurement command checks its 960 cases and bundled font data.

Emit independent document-text candidate JSON from source requests:

```sh
cargo run --example plan_document_text -- \
  tests/fixtures/document-text/requests.json target/document-text-candidate.json
```

Requests contain `schema_version: 1` and uniquely identified `cases`, each with
`id`, `source`, and an optional `source_name` for asset resolution. The exporter
never reads expected results. Successful cases contain page text geometry;
compilation errors, unsupported layout and invalid geometry remain explicit
separate outcomes. This projection does not serialize PDF/PNG or claim that all
painted properties are represented. The fixture regression test compares this
same export format with captured expectations.

Run all tool checks with `python3 -m unittest discover -s tools -p 'test_*.py'`.
