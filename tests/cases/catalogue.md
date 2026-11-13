# Rendering case catalogue

Each case has an editable source and an expected native page count. Run a group
with `cargo review --group NAME`, or select an identifier with `--case NAME`.
Images and data live in `assets/` and `data/`; imported presets live in `defaults/`.

Page counts are regression expectations, not certification of Office layout.
Cases may deliberately exercise unsupported values; inspect their warning manifests.

## Fundamentals

| Identifier | Layout | Pages | Source |
| --- | --- | ---: | --- |
| `word-plain-markdown-basics` | Document | 1 | [plain markdown basics](word-01-plain-markdown-basics.mote) |
| `word-plain-markdown-longform` | Document | 3 | [plain markdown longform](word-02-plain-markdown-longform.mote) |
| `word-nested-styles-layout` | Document | 1 | [nested styles layout](word-03-nested-styles-layout.mote) |
| `word-expanded-markdown` | Document | 1 | [expanded markdown](word-04-expanded-markdown.mote) |
| `word-full-page-markdown-report` | Document | 2 | [full page markdown report](word-05-full-page-markdown-report.mote) |
| `word-full-page-layout-review` | Document · explicit font files | 5 | [full page layout review](word-06-full-page-layout-review.mote) |
| `word-default-style` | Document | 1 | [default style](word-62-default-style.mote) |
| `powerpoint-plain-markdown-deck` | Presentation | 2 | [plain markdown deck](powerpoint-01-plain-markdown-deck.mote) |
| `powerpoint-explicit-boxes` | Presentation | 1 | [explicit boxes](powerpoint-02-explicit-boxes.mote) |
| `powerpoint-slide-features-links` | Presentation | 2 | [slide features links](powerpoint-03-slide-features-links.mote) |
| `powerpoint-standard-auto-layout` | Presentation | 2 | [standard auto layout](powerpoint-14-standard-auto-layout.mote) |
| `powerpoint-markdown-core` | Presentation | 3 | [markdown core](powerpoint-23-markdown-core.mote) |
| `powerpoint-default-style` | Presentation | 3 | [default style](powerpoint-27-default-style.mote) |

## Text

| Identifier | Layout | Pages | Source |
| --- | --- | ---: | --- |
| `word-warning-authority` | Document | 1 | [warning authority](word-07-warning-authority.mote) |
| `word-lists` | Document | 3 | [lists](word-08-lists.mote) |
| `word-definition-lists` | Document | 1 | [definition lists](word-09-definition-lists.mote) |
| `word-rich-numbering` | Document | 1 | [rich numbering](word-10-rich-numbering.mote) |
| `word-wrapped-nested-list-boundary` | Document | 2 | [wrapped nested list boundary](word-11-wrapped-nested-list-boundary.mote) |
| `word-unbreakable-tokens` | Document | 1 | [unbreakable tokens](word-12-unbreakable-tokens.mote) |
| `word-mixed-size-inline-runs` | Document | 2 | [mixed size inline runs](word-13-mixed-size-inline-runs.mote) |
| `word-glyph-coverage` | Document | 1 | [glyph coverage](word-14-glyph-coverage.mote) |
| `word-cjk-line-breaking` | Document · explicit font files | 1 | [cjk line breaking](word-15-cjk-line-breaking.mote) |
| `word-justified-text` | Document · explicit font files | 1 | [justified text](word-16-justified-text.mote) |
| `word-inline-code-font` | Document · explicit font files | 1 | [inline code font](word-17-inline-code-font.mote) |
| `word-preserve-space` | Document | 1 | [preserve space](word-18-preserve-space.mote) |
| `word-keep-with-next` | Document | 4 | [keep with next](word-19-keep-with-next.mote) |
| `word-keep-together` | Document | 4 | [keep together](word-20-keep-together.mote) |
| `word-page-break-before` | Document | 4 | [page break before](word-21-page-break-before.mote) |
| `word-widow-control` | Document | 4 | [widow control](word-22-widow-control.mote) |
| `powerpoint-text-box-styling` | Presentation | 1 | [text box styling](powerpoint-04-text-box-styling.mote) |
| `powerpoint-alignment-grid` | Presentation | 2 | [alignment grid](powerpoint-05-alignment-grid.mote) |
| `powerpoint-text-autofit` | Presentation | 2 | [text autofit](powerpoint-08-text-autofit.mote) |
| `powerpoint-international-text` | Presentation · explicit font files | 2 | [international text](powerpoint-20-international-text.mote) |

## Tables

| Identifier | Layout | Pages | Source |
| --- | --- | ---: | --- |
| `word-longform-tables` | Document | 4 | [longform tables](word-46-longform-tables.mote) |
| `word-table-feature-overview` | Document | 1 | [table feature overview](word-47-table-feature-overview.mote) |
| `word-full-page-tables-and-media` | Document | 2 | [full page tables and media](word-48-full-page-tables-and-media.mote) |
| `word-spanning-table-components` | Document | 2 | [spanning table components](word-49-spanning-table-components.mote) |
| `word-table-column-widths` | Document | 1 | [table column widths](word-50-table-column-widths.mote) |
| `word-table-cell-block-image` | Document | 1 | [table cell block image](word-51-table-cell-block-image.mote) |
| `word-recursive-table-cell-content` | Document | 1 | [recursive table cell content](word-52-recursive-table-cell-content.mote) |
| `word-table-multi-page` | Document | 3 | [table multi page](word-53-table-multi-page.mote) |
| `word-table-row-splitting` | Document | 3 | [table row splitting](word-54-table-row-splitting.mote) |
| `word-table-row-splitting-mixed-cells` | Document | 2 | [table row splitting mixed cells](word-55-table-row-splitting-mixed-cells.mote) |
| `word-table-autofit-vs-fixed` | Document | 1 | [table autofit vs fixed](word-56-table-autofit-vs-fixed.mote) |
| `word-table-cross-page-spans` | Document | 3 | [table cross page spans](word-57-table-cross-page-spans.mote) |
| `word-nested-table-content` | Document | 1 | [nested table content](word-58-nested-table-content.mote) |
| `word-nested-table-row-splitting` | Document | 2 | [nested table row splitting](word-59-nested-table-row-splitting.mote) |
| `word-table-style-effective-runs` | Document | 1 | [table style effective runs](word-60-table-style-effective-runs.mote) |
| `word-table-borders` | Document | 1 | [table borders](word-61-table-borders.mote) |
| `powerpoint-table-fidelity` | Presentation | 2 | [table fidelity](powerpoint-06-table-fidelity.mote) |
| `powerpoint-table-cell-flow` | Presentation | 2 | [table cell flow](powerpoint-17-table-cell-flow.mote) |
| `powerpoint-table-borders` | Presentation | 3 | [table borders](powerpoint-26-table-borders.mote) |
| `slide-table-alignment` | Presentation | 2 | [table alignment](slide-table-alignment.mote) |
| `slide-table-spacing` | Presentation | 2 | [table spacing](slide-table-spacing.mote) |
| `slide-table-margins` | Presentation | 1 | [table margins](slide-table-margins.mote) |
| `markdown-table-document` | Document | 1 | [markdown table alignment](markdown-table-alignment.mote) |
| `markdown-table-slides` | Presentation | 1 | [markdown table alignment](markdown-table-alignment.mote) |

## Flow

| Identifier | Layout | Pages | Source |
| --- | --- | ---: | --- |
| `word-sections-headers-tables` | Document | 2 | [sections headers tables](word-23-sections-headers-tables.mote) |
| `word-headers-footers` | Document | 3 | [headers footers](word-24-headers-footers.mote) |
| `word-multiline-headers-footers` | Document | 2 | [multiline headers footers](word-25-multiline-headers-footers.mote) |
| `word-columns-longform` | Document | 3 | [columns longform](word-26-columns-longform.mote) |
| `word-columns-layout` | Document · explicit font files | 3 | [columns layout](word-27-columns-layout.mote) |
| `word-natural-column-balancing` | Document | 1 | [natural column balancing](word-28-natural-column-balancing.mote) |
| `word-section-transition` | Document | 3 | [section transition](word-29-section-transition.mote) |
| `word-media-references-notes` | Document | 1 | [media references notes](word-30-media-references-notes.mote) |
| `word-reference-parity` | Document | 2 | [reference parity](word-31-reference-parity.mote) |
| `word-forward-page-reference-reflow` | Document | 10 | [forward page reference reflow](word-32-forward-page-reference-reflow.mote) |
| `word-annotation-story-placement` | Document | 1 | [annotation story placement](word-33-annotation-story-placement.mote) |
| `word-notes-one-page` | Document | 1 | [notes one page](word-34-notes-one-page.mote) |
| `word-notes-two-page` | Document | 2 | [notes two page](word-35-notes-two-page.mote) |
| `word-footnote-page-pressure` | Document | 2 | [footnote page pressure](word-36-footnote-page-pressure.mote) |
| `word-endnote-page-pressure` | Document | 2 | [endnote page pressure](word-37-endnote-page-pressure.mote) |

## Media

| Identifier | Layout | Pages | Source |
| --- | --- | ---: | --- |
| `word-layout-images` | Document | 2 | [layout images](word-38-layout-images.mote) |
| `word-image-transforms` | Document | 1 | [image transforms](word-39-image-transforms.mote) |
| `word-image-horizontal-positioning` | Document | 1 | [image horizontal positioning](word-40-image-horizontal-positioning.mote) |
| `word-anchored-image-wrap` | Document | 1 | [anchored image wrap](word-41-anchored-image-wrap.mote) |
| `word-editable-drawingml-shapes` | Document | 1 | [editable drawingml shapes](word-42-editable-drawingml-shapes.mote) |
| `word-chart-properties` | Document | 2 | [chart properties](word-43-chart-properties.mote) |
| `word-chart-defaults` | Document | 2 | [chart defaults](word-44-chart-defaults.mote) |
| `word-chart-lua-json` | Document · Lua | 1 | [chart lua json](word-45-chart-lua-json.mote) |
| `word-drawings-charts-flow` | Document | 2 | [drawings and charts in flow](word-64-drawings-charts-flow.mote) |
| `word-display-equations` | Document | 2 | [display equations](word-65-display-equations.mote) |
| `word-inline-equations-flow` | Document | 2 | [inline equations in flow](word-66-inline-equations-flow.mote) |
| `word-equation-page-breaks` | Document | 2 | [equation page breaks](word-67-equation-page-breaks.mote) |
| `word-stacked-fractions` | Document | 3 | [stacked fractions](word-68-stacked-fractions.mote) |
| `word-equation-columns` | Document | 2 | [equation columns](word-69-equation-columns.mote) |
| `powerpoint-shapes-lines` | Presentation | 2 | [shapes lines](powerpoint-07-shapes-lines.mote) |
| `powerpoint-theme-text-tables-charts` | Presentation | 1 | [theme text tables charts](powerpoint-09-theme-text-tables-charts.mote) |
| `powerpoint-chart-properties` | Presentation | 3 | [chart properties](powerpoint-10-chart-properties.mote) |
| `powerpoint-chart-exhaustive` | Presentation | 10 | [chart exhaustive](powerpoint-11-chart-exhaustive.mote) |
| `powerpoint-chart-defaults` | Presentation | 2 | [chart defaults](powerpoint-12-chart-defaults.mote) |
| `powerpoint-chart-lua-json` | Presentation · Lua | 1 | [chart lua json](powerpoint-13-chart-lua-json.mote) |
| `powerpoint-image-transforms` | Presentation | 2 | [image transforms](powerpoint-15-image-transforms.mote) |
| `powerpoint-background-images` | Presentation | 3 | [background images](powerpoint-16-background-images.mote) |
| `powerpoint-math-layout` | Presentation | 2 | [math layout](powerpoint-18-math-layout.mote) |
| `powerpoint-shape-text-bounds` | Presentation | 2 | [shape text bounds](powerpoint-19-shape-text-bounds.mote) |
| `powerpoint-named-layouts` | Presentation | 9 | [named layouts](powerpoint-21-named-layouts.mote) |
| `powerpoint-named-layouts-lua-json` | Presentation · Lua | 1 | [named layouts lua json](powerpoint-22-named-layouts-lua-json.mote) |
| `powerpoint-media-charts` | Presentation | 1 | [media charts](powerpoint-24-media-charts.mote) |
| `powerpoint-slide-edges` | Presentation | 2 | [slide edges](powerpoint-25-slide-edges.mote) |
| `powerpoint-groups` | Presentation | 1 | [groups](powerpoint-32-groups.mote) |
