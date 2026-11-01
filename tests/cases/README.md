# Rendering test cases

These small, editable sources expose specific layout rules. The
[table tests](../table_alignment.rs) assert text positions and editable package
properties directly. Generated images are review artifacts, not test expectations.

| Case | What to inspect |
| --- | --- |
| [Markdown table alignment](markdown-table-alignment.mote) | Left edges, centers and right edges line up by column in both document and slide layouts. The source uses only Markdown. |
| [Slide table alignment](slide-table-alignment.mote) | Top, middle and bottom rows; left, center and right columns; explicit paragraph overrides; a styled Markdown table. Cell alignment overrides the table style, and paragraph alignment overrides the cell. |
| [Slide table spacing](slide-table-spacing.mote) | Two paragraphs in each cell. Style spacing contributes to the vertically aligned text stack. The second slide removes the inner gap with explicit zero values. |
| [Slide table margins](slide-table-margins.mote) | Style and default margins, direct cell overrides, and explicit zero. `all` applies first, then `horizontal`/`vertical`, then individual edges. |

Render all five layout cases without opening an Office application:

```sh
cargo review
cargo test --test table_alignment
```

`cargo review` builds and runs the Rust review generator against the current
library. It requires no Python or separately built Mote executable.

```sh
cargo review --list
cargo review --case slide-table-alignment
cargo review --output /path/to/review
```

`--case` can repeat. The full command is
`cargo run --example render_review -- [OPTIONS]`; `cargo review` is its repository
alias. Generator regression tests run with `cargo test --example render_review`.

Source cases stay in `tests/cases/`; generated review artifacts live in
`target/review/`, alongside other disposable build output. The review tree
separates applications and numbers cases within each application:

```text
target/review/
  concise/
    word/
      01_markdown_table_alignment/
    powerpoint/
      01_markdown_table_alignment/
      02_table_alignment/
      03_table_spacing/
      04_table_margins/
  complete/
    word/                         # same case folders, with diagnostics
    powerpoint/
```

Each concise folder contains `<case>.mote`, `<case>.docx` or `<case>.pptx`,
native `<case>.pdf`, and native `<case>-1.png`, `<case>-2.png`, and so on at
144 DPI. The complete folder contains byte-identical copies plus
`<case>-manifest.txt`, recording source/output hashes, warnings and review status.
Only the selected case is refreshed; unrelated cases remain unchanged. The
renderer refuses to replace a case with an Office owner file present.

The concise tree contains native outputs only. Future Office captures belong in
the complete tree with explicit `<case>-word.*` or `<case>-powerpoint.*` names;
they must never replace native PDF/PNG files. No Office references have been
captured for these cases.

Slide cases use Arial; the Markdown case deliberately retains ordinary defaults.

Inspect every page for alignment, spacing, clipping and overflow. Generated
packages remain unreviewed in Microsoft Word or PowerPoint until inspected in
the corresponding application. Keep the source fixed after finding a mismatch;
correct the layout behavior rather than changing the case to conceal it.
