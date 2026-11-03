# Rendering test cases

The [catalogue](catalogue.md) contains 94 editable cases: 63 document cases and
31 presentation cases. They cover ordinary Markdown, typography, tables,
pagination, running content, drawings, charts, layouts and evaluated data.
The [catalogue data](catalogue.json) records the source, evaluation mode, group,
required font files and expected native page count for each case.

Sources, small input assets and assertions are versioned. Generated packages,
PDFs and images are disposable review artifacts, not stored test expectations.
The [catalogue test](../review_catalogue.rs) evaluates every case, checks page
counts and parses every XML part in its editable package. It also checks that
nested table continuations retain each row's text once and omit text outside
the enclosing cell fragment. The focused
[table tests](../table_alignment.rs) separately assert alignment, spacing and
margin geometry.

Render all cases without opening an Office application:

```sh
cargo review
cargo test --test review_catalogue --test table_alignment
```

`cargo review` builds and runs the Rust review generator against the current
library. It requires no Python or separately built Mote executable.

```sh
cargo review --list
cargo review --case slide-table-alignment
cargo review --group fundamentals
cargo review --output /path/to/review
```

`--case` and `--group` can repeat. The groups are `fundamentals`, `text`, `tables`,
`flow` and `media`; specifying both filters takes their intersection. Lua is
explicitly enabled only for the three cases that require it. A failed case is
reported and the remaining selected cases still run. The command exits with an
error if any case fails; `complete/last-run.json` records every selected result.
An older successful output may remain when a refresh fails, so consult that run
report before treating existing files as fresh.

The full command is
`cargo run --example render_review -- [OPTIONS]`; `cargo review` is its repository
alias. Generator regression tests run with `cargo test --example render_review`.

Source cases stay in `tests/cases/`; generated review artifacts live in
`target/review/`, alongside other disposable build output. The review tree
separates applications and numbers cases within each application:

```text
target/review/
  concise/
    word/
      01_plain_markdown_basics/
      ...
      63_markdown_table_alignment/
    powerpoint/
      01_plain_markdown_deck/
      ...
      28_table_alignment/
      29_table_spacing/
      30_table_margins/
      31_markdown_table_alignment/
  complete/
    word/                         # same case folders, with diagnostics
    powerpoint/
```

Each concise folder contains `<case>.mote`, `<case>.docx` or `<case>.pptx`,
native `<case>.pdf`, and native `<case>-1.png`, `<case>-2.png`, and so on at
144 DPI. Used images, data and imported presets are copied into `assets/`,
`data/` and `defaults/` within each case so its source remains runnable there.
The complete folder contains byte-identical copies plus `<case>-model.json` and
`<case>-manifest.txt`, recording input/output hashes, warnings and review status.
Inputs are snapshotted and checked for changes before publication.
Only the selected case is refreshed; unrelated cases remain unchanged. The
renderer refuses to replace a case with an Office owner file present.

The concise tree contains native outputs only. Future Office captures belong in
the complete tree with explicit `<case>-word.*` or `<case>-powerpoint.*` names;
they must never replace native PDF/PNG files. No Office references have been
captured for these cases.

Cases retain their declared fonts; ordinary Markdown retains ordinary defaults.
Some typography cases explicitly load Verdana, Arial Unicode or Andale Mono from
`/System/Library/Fonts/Supplemental/`. These require those installed font files;
the generator and catalogue test fail clearly when a required file is missing.
System fonts are recorded by hash when loaded through the asset resolver and
are not redistributed with the review files.

Inspect every page for alignment, spacing, clipping and overflow. Generated
packages remain unreviewed in Microsoft Word or PowerPoint until inspected in
the corresponding application. Keep the source fixed after finding a mismatch;
correct the layout behavior rather than changing the case to conceal it.
