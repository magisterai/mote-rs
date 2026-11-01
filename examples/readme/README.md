# README examples

The [main README](../../README.md) introduces these examples in order. Each has
an editable Word and PowerPoint download in
[docs/examples/readme](../../docs/examples/readme/README.md). The sample business
content and revenue figures are illustrative.

| Example | Source | Composition |
| --- | --- | --- |
| Pure Markdown | [plain.md](plain.md) | The exact file is passed to both output formats, without any added declarations. |
| Defaults | [defaults.mote](defaults.mote) + [plain.md](plain.md) | The defaults are prepended in memory; the original Markdown remains unchanged. |
| Directives | [directives.mote](directives.mote) | Imports the defaults and adds a styled introduction and an inline accent. |
| Presentation | [presentation.mote](presentation.mote) | A named slide layout with a theme and speaker notes. The [Word companion](presentation-document.mote) puts the same text into document flow. |
| JSON chart | [chart.mote](chart.mote) + [revenue.json](revenue.json) | Static JSON loading creates an editable chart without scripting. |
| Lua calculation | [lua.mote](lua.mote) + [revenue.json](revenue.json) | A function sums monthly values and converts thousands to dollars. This example alone requires `--lua`. |

The styled document examples use compact pages for the README. Their PowerPoint
versions use automatic layout and the separate
[slide defaults](slide-defaults.mote) for a wide canvas and larger text. These
are format-specific defaults applied to the same content. The explicitly
positioned presentation uses its own slide setup instead.

## Keep Markdown unchanged

From the repository root, after building Mote:

```sh
mkdir -p target/readme
cd examples/readme
cat defaults.mote plain.md | ../../target/release/mote --format docx -o ../../target/readme/defaults.docx -
```

The compiler reads the combined stream. Neither input file is edited, and there
is no dedicated defaults CLI flag. Running in the source directory keeps imports
and resource paths relative to these examples.

## Regenerate the downloads and previews

From the repository root, with Rust, Cargo and Python 3 installed:

```sh
cargo build --release
python3 tools/render_readme_examples.py
```

The generator uses Mote for all twelve Office packages and twelve native PNG
previews. It validates package XML, requires one page or slide per preview, and
publishes the set after generation succeeds. Pass `--mote PATH` to select another
Mote binary. Inspect every native image before committing an updated set.

Document page settings, Markdown heading markers and repeated document table
headers can report that they have no presentation effect. The generator preserves
these diagnostics and rejects unexpected warnings. Slide canvas dimensions come
from the presentation setup.

The full sources include the setup omitted from README excerpts.
[fonts.mote](fonts.mote) declares Source Sans 3 using the existing
[font fixtures](../../tests/fixtures/measurements/fonts/) and their
[license](../../tests/fixtures/measurements/fonts/LICENSE.md). The `Aptos` alias
supplies the native chart's default font outlines. Styled native previews use
these declared files; Office applications need Source Sans 3 installed to use
the same typeface. The pure Markdown example retains the engine's default fonts.

Images live separately in [docs/images/readme](../../docs/images/readme/).
They are native Mote renders, not Office captures. Word and PowerPoint visual
review remains pending. Keep temporary artifacts under `target/readme/`.
