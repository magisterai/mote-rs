# Plain-paragraph DOCX reference

[Fixture guide](../../README.md)

This fixture records the input `#paragraph[Hello, world.]`, its evaluated model,
and captured package data for comparison during DOCX development. Its Word review
status is **unreviewed**; no rendered images or measured geometry are recorded.

## Files

- [source.mote](source.mote): the complete input, ending with one newline.
- [model.json](model.json): the complete evaluated model.
- [word/document.xml](package/word/document.xml): paragraph content and section
  geometry.
- [word/styles.xml](package/word/styles.xml): paragraph and run defaults.
- [word/settings.xml](package/word/settings.xml): compatibility and editing settings.
- The remaining files under `package/`: content types, relationships, font table,
  empty numbering, and metadata, retaining the package's paths.

[capture.json](capture.json) records source and archive SHA-256 fingerprints,
generation time, the complete ten-entry inventory, and review status. The capture
completed with no warnings. XML is pretty-printed, retaining namespaces,
attributes, element order, and text values. Core timestamps remain as captured;
the ZIP container is omitted. Its fingerprint identifies this capture rather
than an expected byte-for-byte match for future output.

## Captured behaviour

The model has one paragraph and one text child with no authored attributes.
Root and implicit-section content are two views of that same paragraph. Empty
page, margin, and column objects contain no output defaults.

The package uses Transitional WordprocessingML. Its body has one `w:p`, containing
one `w:r` and `w:t xml:space="preserve"` with the exact text `Hello, world.`.
There are no direct paragraph/run properties or extra paragraphs. A terminal
`w:sectPr` follows the paragraph as a direct child of `w:body`.

| Captured property | Value |
| --- | --- |
| Default paragraph style | `Normal`, with no formatting beyond document defaults. |
| Typeface and size | Helvetica, 11 points; both size fields are `22` half-points. |
| Paragraph spacing | 8 points after (`160` twips); 1.15 line spacing (`276`, `auto`). |
| Page size | Letter, portrait: `12240` × `15840` twips. |
| Margins | `1440` twips (1 inch) on every side; zero gutter. |
| Header/footer distances | `720` twips (0.5 inch); no header/footer content. |
| Settings | 100 percent zoom, `720`-twip default tabs, `doNotExpandShiftReturn`, compatibility mode `15`. |

The font table also names Aptos, Aptos Display, and Courier New; their presence
does not select them for this text. Numbering is empty. Metadata names `Mote` as
creator, last modifier, and application, with an empty title and equal UTC
creation/modification times. The captured `AppVersion` value is `16.0000`; it
does not identify a Word review. The complete values remain readable in the XML.

## Comparison notes

The [JSON test](../../../json.rs) evaluates `source.mote` and compares the complete
result with `model.json`, including the absence of output defaults and evaluation
warnings. XML files are reference data; the Rust test suite does not generate
DOCX yet.

For package comparisons, resolve XML names by namespace URI and local name, and
relationships by type, resolved target, and target mode. Preserve text, units,
formatting values, section geometry, and meaningful element order. Do not hide
differences in these values through normalization.

ZIP ordering/compression/timestamps, XML attribute order, empty-element spelling,
and indentation outside text values can differ. Relationship identifiers and
namespace prefixes can differ when their references remain consistent, including
QName and prefix-list values such as `mc:Ignorable`. Content-type declarations
can be reordered while preserving their mappings. Core timestamps describe each
generation and should not be copied from this capture.

The package can be reconstructed by zipping the contents of `package/` with its
relative paths at the archive root. Repacking reference XML does not implement
generation. Actual generated output still needs structural comparison and review
in real Microsoft Word. This short paragraph alone cannot establish multiline
spacing or pagination conformance. Record review evidence for the exact generated
package before changing its review status.
