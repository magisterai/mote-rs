# Authoring content captures

These frozen sources exercise Markdown strikethrough and underlined headings,
structured list and quote content, recursive table-cell paragraphs, list spacing
inside styled cells, and numeric segments in long URLs. The slide case checks the
same Markdown lowering in editable and native presentation output.

Captured models, full editable packages, native text geometry and PNG pages keep
acceptance separate from visible formatting. Body list and quote children retain
inline text but do not independently apply paragraph or heading formatting;
nested dividers are omitted with a diagnostic. Cell wrappers
preserve their supported paragraph boundaries. Unsupported semantic-wrapper
formatting produces a warning and keeps the wrapper's semantic effect.

Microsoft Word and PowerPoint visual review remains pending.

The thirteen cases pin source hashes, every package part, text positions to 1e-8
points, and native PNGs. Each PNG has its own measured font-edge tolerance; the
slide capture permits one specifically located edge pixel at `(308, 56)` on the
heading. PDF raster comparisons were pixel-identical for all thirteen cases.
