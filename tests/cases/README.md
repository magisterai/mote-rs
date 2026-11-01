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

Run the focused assertions with:

```sh
cargo test --test table_alignment
```

Slide cases use Arial; the Markdown case deliberately retains ordinary defaults.
Keep the source fixed after finding a mismatch; correct the layout behavior
rather than changing the case to conceal it.
