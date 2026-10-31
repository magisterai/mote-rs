# Rendering coverage

[Documentation](README.md) · [Implementation](implementation.md) ·
[Attribute inventories](checklists/attributes/README.md)

The directive checklists define the accepted attributes, nested fields and value
rules. This map connects those contracts to output tests. A retained property is
not necessarily painted. The linked support notes distinguish editable mappings,
native geometry and unsupported values; checkmarks do not certify Office layout.

The combined [rendering audit](../tests/fixtures/rendering-audit/README.md) tests
interactions across these families. Its sources stay fixed when a discrepancy is
found. Focused tests remain responsible for individual attributes and edge values.
Microsoft Word and PowerPoint visual review remains unreviewed.

## Document flow and text

| Contracts and attributes | Output evidence | Current boundary |
| --- | --- | --- |
| [Document](checklists/document.md), [section](checklists/section.md): page size, orientation, margins, column count/gap/separator, section starts | [Configuration](../tests/docx_configuration.rs), [sections](../tests/section_rendering.rs), audit `document-sections` | Equal-width columns, supported continuous transitions and section snapshots. Unequal column geometry is not implemented. |
| [Paragraph](checklists/paragraph.md), [heading](checklists/heading.md), [blockquote](checklists/blockquote.md): alignment, spacing, indentation, keeps, widow control, fill, borders, outline and numbering | [Paragraph flow](../tests/paragraph_flow.rs), [native blocks](../tests/native_blocks.rs), audit `document-typography` and `document-pagination` | DOCX maps the documented paragraph properties. Native borders use a bottom rule; outline metadata is not painted. Quote child blocks flatten. |
| [Span](checklists/span.md), [strong](checklists/strong.md), [emphasis](checklists/emphasis.md), semantic formatting: fonts, sizes, colors, emphasis, underline, strike, highlight, small capitals and scripts | [Spans](../tests/spans.rs), [semantic formatting](../tests/semantic_formatting.rs), [decorations](../tests/text_decorations.rs), audit typography/text/notes cases | Direct wrapper formatting has output-specific limits. Native wave/none underlines retain a solid line and warn. Script metrics differ between documents and slides. |
| [Style](checklists/style.md), [font](checklists/font.md): inheritance, run/paragraph/table defaults, script mappings, font files and variants | [Styles](../tests/layout_styles.rs), [measurement](../tests/measurement.rs), [font resolution](../tests/font_resolution.rs), [installed fonts](../tests/installed_fonts.rs) | Static font measurement and PDF embedding are supported. Complex script shaping and variable-font interpolation remain outside the current scope. Document line-height calibration does not apply to drawing text. |
| [List](checklists/list.md), [ordered list](checklists/ordered-list.md), [item](checklists/item.md): numbering identity, start/restart, format, level, indentation, spacing and tasks | [Editable lists](../tests/docx_lists.rs), [native blocks](../tests/native_blocks.rs), audit `slide-text` and `slide-automatic` | Document item child blocks flatten; nested lists keep their markers. Slide lists use presentation paragraph and marker metrics. |
| [Tab](checklists/tab.md), [line break](checklists/line-break.md), [break](checklists/break.md): inline controls, custom tab stops, page and column boundaries | [Tabs](../tests/tabs.rs), [breaks](../tests/breaks.rs), [line layout](../tests/line_layout.rs), audit `document-typography` | DOCX supports authored tab stops; native text uses its existing fixed tab policy. Explicit breaks and natural pagination have separate tests. |
| [Code](checklists/code.md), [raw](checklists/raw.md), [definition lists](checklists/definition-list.md), [divider](checklists/divider.md): literal content, whitespace, term/definition flow and rule styling | [Block content](../tests/block_content.rs), audit `document-typography`, `slide-text` | Literal language labels do not enable syntax highlighting. Definition paragraphs flatten without added separators in native flow. |
| [Header](checklists/header.md), [footer](checklists/footer.md), [page number](checklists/page-number.md): variants, inheritance, height, distance, current/total fields | [Sections](../tests/section_rendering.rs), audit `document-pagination`, `document-sections`, `document-table-flow` | Running-story content is restricted and has its own diagnostics. Page-count fields resolve after pagination. |
| [Footnote](checklists/footnote.md), [endnote](checklists/endnote.md): identifiers, note bodies and semantic wrappers | [Notes](../tests/notes.rs), [note rendering](../tests/note_rendering.rs), audit `document-formatted-notes` | Native placement supports stable notes in one single-column section; other contexts retain fallback content and warn. Footnotes are not split. Note text uses document kerning. |
| [Link](checklists/link.md), [label](checklists/label.md), [bookmark](checklists/bookmark.md), [reference](checklists/ref.md): destinations, reference text/page, titles and links | [References](../tests/reference_rendering.rs), [links](../tests/links.rs), audit typography/pagination/layout cases | Unsupported anchor placements warn. Links and reference text are tested independently of Office field refresh. |

## Tables and drawings

| Contracts and attributes | Output evidence | Current boundary |
| --- | --- | --- |
| [Table](checklists/table.md), [row](checklists/row.md), [cell](checklists/cell.md): width/alignment, styles, margins, fills, merges, borders, header repetition and splitting | [Tables](../tests/tables.rs), [table rendering](../tests/table_rendering.rs), audit `document-table-flow`, `document-table-edges`, `slide-table` | Document and slide table engines have distinct height, direction and no-wrap rules. Native document indent, cell spacing, layout and column style regions remain limited as described in their checklists. |
| [Image](checklists/image.md): asset resolution, dimensions, alignment, crop, rotation, flips, effects, border and link | [Images](../tests/images.rs), [slide rendering](../tests/presentation_rendering.rs), audit `document-objects`, `slide-media-edges` | Floating wrap exclusion is not implemented. Native document image borders are solid/opaque; slide picture borders support patterns and opacity. Cropped fitting has format-specific behavior. |
| [Figure](checklists/figure.md), [caption](checklists/caption.md): alignment, position, labels, sequences and numbering | [Captions](../tests/caption_rendering.rs), audit `document-objects` | Native flow honors caption position; editable ordering follows authored content. Nested caption counters and flattened content have documented limits. |
| [Shape](checklists/shape.md), [line](checklists/line.md): preset geometry, transforms, fill/opacity, stroke, shadows, connectors, arrows and text bounds | [Shape geometry](../tests/shape_geometry.rs), [shape text](../tests/shape_text.rs), [slide rendering](../tests/presentation_rendering.rs), audit `slide-text`, `slide-media-edges` | Preset-specific text bounds and fitting are measured. Unsupported presets, effects and marker combinations report their limits. Exact canvas-edge clipping remains visible in the fixtures. |
| [Chart](checklists/chart.md): kind, data, size, colors, legend, titles, axes, labels, gap/overlap, chart/plot styling | [Charts](../tests/charts.rs), [chart rendering](../tests/chart_rendering.rs), audit `document-objects`, `slide-positioned-charts` | Editable charts retain workbooks. Native painting supports the documented twelve kinds. Zero-valued bars paint no area but retain labels. Signed percentage stacks normalize by total magnitude. Labels fit their allocated frames and legends reserve measured rows; very dense charts and radial-label collisions remain limited. |

## Presentation composition and evaluated content

| Contracts and attributes | Output evidence | Current boundary |
| --- | --- | --- |
| [Presentation](checklists/presentation.md), [slide](checklists/slide.md): canvas, theme, backgrounds, footer fields and slide boundaries | [Presentations](../tests/presentations.rs), [PPTX](../tests/pptx.rs), [slide rendering](../tests/presentation_rendering.rs), audit automatic/media cases | Native slides are painted independently from PPTX. Bare Markdown requires explicit presentation layout selection for PDF/PNG. Speaker notes remain package content. |
| [Box](checklists/box.md): position, size, text defaults, margins, alignment, clip/shrink/resize, image fit, fill and border | [Slide rendering](../tests/presentation_rendering.rs), audit `slide-text`, `slide-table`, `slide-media-edges`, `slide-positioned-charts` | Boxes contain supported text or a single image/table/chart object. Loose charts receive separate automatic frames. The equal-height automatic layout can still leave dense multi-chart slides too cramped. |
| [Layout](checklists/layout.md): built-ins, inheritance, typed slots, order, overrides, removal and required content | [Named layouts](../tests/presentation_layouts.rs), audit `slide-layouts` | Slot resolution precedes shared editable/native layout. Local overrides do not mutate the declaration or sibling slides. |
| [Scoped defaults](checklists/set.md), [imports](checklists/import.md), bindings, functions, control flow, JSON and CSV | [Defaults](../tests/scoped_defaults.rs), [imports](../tests/imports.rs), [functions](../tests/function_captures.rs), [control flow](../tests/control_rendering.rs), [CSV](../tests/csv.rs) | These produce the same model consumed by the renderers. Source-relative resources, evaluation scope and constructor defaults have separate regression captures; they do not grant additional output properties. |

Forms, broader metadata and additional OOXML mappings are outside this rendering
pass. Unsupported properties remain visible in the standard inventories instead
of being counted as completed rendering support.
