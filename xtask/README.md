# Repository tools

The repository tasks are a Rust crate in this directory, run as
`cargo xtask COMMAND`; `cargo xtask --help` lists the commands. The crate
depends on the Mote library and on no library that Mote does not already use.
`cargo test --workspace` runs its tests with those of the library.

## Rendering test cases

`cargo review` builds the Rust review generator and renders the
[case catalogue](../tests/cases/README.md) to `target/review/`. The `concise/` tree
groups numbered cases under `word/` and `powerpoint/`; each case contains its
source, editable package, native PDF and one native PNG per page. The `complete/`
tree has the same files plus evaluated models and manifests with hashes, warnings and review status. Use
`cargo review --case NAME` to select a case, `--group NAME` to select a topic,
`--list` to list cases, or
`--output PATH` to choose the review root. A focused refresh leaves other cases
unchanged and refuses to replace an open Office file. No Office application is opened.
The generator calls the library directly. Its source is
[src/review.rs](src/review.rs); `cargo review` is the alias of
`cargo xtask review [OPTIONS]`.

## README examples

`cargo xtask readme` regenerates the seven README examples as editable DOCX and
PPTX downloads, PDF downloads of the report and deck, and native PNG previews.
The task calls the Mote library directly, so no separately built executable is
required. The outputs are saved under `readme/downloads/` and
`readme/images/`. The [example guide](../readme/examples/README.md) explains
the composition of the sources and their review.

## Package validation

`cargo xtask validate PACKAGE...` validates every XML part of the named DOCX and
PPTX packages, and of their embedded workbooks, against the retained ECMA-376
transitional schemas with `xmllint`. Markup compatibility is applied first, as
Office applies it. Word and PowerPoint may silently omit a part that violates
its schema, so a package can render natively yet lose a chart in Office; the
validator reports such parts before review. The README task runs it on every
package it publishes.

The retained schemas import the XML namespace without a location, and `xmllint`
then fails to compile them and reports no validity errors at all. The validator
therefore loads each schema through a wrapper, written under
`target/xtask/schemas/`, that supplies [xml.xsd](schemas/xml.xsd), and treats a
schema that fails to compile as an error rather than a pass.

## Test captures

`cargo xtask refresh-captures` regenerates every captured DOCX and PPTX under
`tests/fixtures` after an intended output change and replaces each stored part
whose canonical XML differs. Formatting whitespace, generation timestamps and
advisory page-break markers never cause a replacement. Each source is rendered as
its test renders it: from its own directory, and with Lua where the group's test
harness enables it. Content-addressed part files that no manifest names any
longer are removed. `--group NAME` limits the refresh to one fixture group, and
`--check` reports the parts that would change without writing them. Every
replaced part is listed, so that the refresh can be reviewed before it is
committed. The tests of the task require every manifest to be rewritten without
change and every stored part file to be referenced.

Native geometry and raster captures are refreshed by their own test harnesses.
After an intended layout change, `MOTE_REFRESH_CAPTURES=1 cargo test --test NAME`
writes the current output in place of each capture that fails its tolerance, and
updates the recorded digests, in the harnesses that support refresh: the section,
note and table captures, the authored-content and combined rendering-audit
cases, and the document-text, paragraph-text and text-decoration collections. A
refreshed JSON capture keeps its stored text and changes only the values that
moved. A refresh is made only for a change that Office evidence supports.

## Attribute inventory

`cargo xtask inventory` checks the generated support inventories against
retained local schemas. Use `--write` after changing `scope.json` or
`support.json`. The task does not retrieve sources from the network.

## Output comparison

`cargo xtask compare-outputs` runs two explicitly selected Mote executables over the same
source cases and compares their JSON models and every DOCX package part. Build the
candidate first so the comparison uses the intended code:

```sh
cargo build
cargo xtask compare-outputs \
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

Run the comparison task's regression checks with
`cargo test -p xtask compare_outputs`.

## Font measurement comparison

`cargo xtask compare-measurements` validates the self-contained
[measurement fixtures](../tests/fixtures/measurements/README.md) and compares
candidate measurements against the frozen expectations. It does not run a
renderer or locate installed fonts.

```sh
cargo xtask compare-measurements --fixture tests/fixtures/measurements
cargo xtask compare-measurements \
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

Run the measurement task's regression checks with
`cargo test -p xtask compare_measurements`. Metric differences are computed in
exact decimal arithmetic, so a tolerance of one billionth of a point is neither
widened nor narrowed by binary rounding.

## Native layout captures

Verify the nine frozen layout collections, including file fingerprints and exact
case coverage, without running a renderer:

```sh
cargo xtask verify-layout
```

This checks 1,599 cases across lines, styles, paragraph text, page geometry,
paragraph flow, document text, inline decoration, native blocks and native
columns. It rejects changed bytes,
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

Run all task checks with `cargo test -p xtask`.
