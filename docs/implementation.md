# Implementation

The Rust crate implements the language core, editable DOCX and PPTX, and native
PDF/PNG output for the documented document and slide subsets. The
[specification](specification/README.md) describes the language; this page records
what this crate currently implements.

The [directive checklists](checklists/README.md) track attribute-level work.
Checked items have a specification, implementation, and tests. Partial model
support is noted beside unchecked items; output verification is tracked separately.

## Code organization

The language core stays in one crate. Modules divide responsibilities within it;
new capabilities should justify a separate crate through a substantial boundary.

| Module | Responsibility |
| --- | --- |
| [diagnostics](../src/diagnostics.rs) | Shared source ranges and diagnostic locations. |
| [source](../src/source.rs) | Asset resolver interface and UTF-8 source loading, with filesystem lookup and logical-name handling in separate modules. |
| [syntax](../src/syntax.rs) | Syntax tree and parser entry points, with separate markup and expression parsers. |
| [evaluation](../src/evaluation.rs) | Content flow and directive bodies, with separate expression, attribute, declaration, configuration, and value-normalization modules. |
| [model](../src/model.rs) | Evaluated content, values, geometry, and declaration registries. |
| [formatting](../src/formatting.rs) | Style inheritance and font-alias resolution. |
| [references](../src/references.rs) | Document anchor metadata, caption associations and reference text selection. |
| [captions](../src/captions.rs) | Shared caption sequences and child alignment defaults, with counter formatting in `numbering`. |
| [measurement](../src/measurement.rs) | Core and static OpenType font metrics, with separate kerning and font-table modules. |
| [media](../src/media.rs) | Cached raster assets, bounded PNG/JPEG/GIF decoding and shared image transforms. |
| [layout](../src/layout.rs) | Shared native line and block geometry, paragraph flow, numbering, columns, and ordered painting operations. |
| [render](../src/render.rs) | Independent PDF and PNG painters, font embedding, rasterization and native support diagnostics. |
| [docx](../src/docx.rs) | Editable Word output, with separate document flow, list/numbering, properties, styles, fonts, XML and package modules. |
| [drawings](../src/drawings.rs) | Shared drawing text, shape/connector XML and physical unit conversion. |
| [charts](../src/charts.rs) | Resolved chart data and appearance, editable chart XML and embedded workbooks. |
| [presentation](../src/presentation.rs) | Deck dimensions, segmentation, automatic frames, named layout resolution, themes, backgrounds, slide metadata and shared object semantics. |
| [pptx](../src/pptx.rs) | Editable slide packages, masters, themes, positioned objects, chart workbooks, footer fields and speaker notes. |
| [json](../src/json.rs) | Deterministic model inspection. |
| [main](../src/main.rs) | Command-line arguments, input/output, and diagnostic reporting. |

Parsing does not depend on evaluation. The model shares diagnostic types without
depending on the parser; formatting and JSON inspection operate on the model.
Named-argument evaluation retains each value with its source expression so type
checks and conversion diagnostics keep the correct location.

## Source loading

`source::load_source` loads a single named input through an `AssetResolver` and
returns a `SourceFile` containing its logical name and validated UTF-8 text.
Callers pass that text to `mote::compile`; `compile(source)` remains independent
of filesystem access. Loading does not add output defaults or source metadata
to the document model.

An `AssetRequest` contains the authored reference and an optional referring source
name. A `ResolvedAsset` owns its bytes and its stable logical name. Custom
resolvers can supply memory-backed or generated inputs and implement the trait
directly or use a closure. Their returned names are preserved exactly, and their
errors are returned without filesystem fallback. If the returned name is empty,
`resolve_asset` derives a source-relative logical path or hierarchical URI without
accessing the filesystem. URI resolution preserves encoded path segments, queries,
and fragments; it removes only literal `.` and `..` segments. The `iri-string`
dependency validates URI components and escapes Unicode names; it does not fetch
resources. A relative reference from an opaque URI requires a resolver-supplied name.

`FileAssetResolver` captures an absolute working directory when constructed.
Root relative references use that directory. Nested references use the directory
of their referring source, with a relative source name anchored to the captured
working directory. Absolute references retain their own location. Names are
normalized lexically, including `.` and `..`; symlinks are not canonicalized.
The filesystem resolver rejects URI references and relative references whose
source name is a URI. Such references require a custom resolver.

References and names use native OS strings to avoid losing filesystem filename
bytes; source contents must be UTF-8. Loading failures retain the requested name,
and decoding failures retain the resolved name. The CLI uses this loader for
files and reports resolved names on parser/evaluator errors and warnings.
Standard input remains labelled `<stdin>`.

`CachingAssetResolver` snapshots each source/reference request, including failures,
and rejects conflicting bytes for one logical name. The font measurer owns a
wrapper for its lifetime and retains selected font bytes for future painters.
Images also cache resolved assets and transformed rasters for the duration of planning.
The JSON helper shares source-relative resolution and compilation snapshots; imports
remain deferred. Tests in
[asset_cache.rs](../tests/asset_cache.rs), [source.rs](../tests/source.rs) and
[cli.rs](../tests/cli.rs) cover custom sources, relative names, failure propagation,
encoding, and CLI diagnostics.

## Document configuration

`document` declarations retain authored arguments and merge normalized page,
margin and column fields. They produce no visible node, including when evaluated inside a
body. JSON preserves both the last authored argument and the merged geometry,
which also appears on the implicit section. DOCX maps named paper sizes, custom
dimensions, orientation, all seven margin distances and equal-width columns to a
resolved section snapshots. Columns support count, gap and separator; an omitted
count emits
no column property. Unequal-width columns are retained in JSON but rejected by
DOCX until individual column geometry is implemented.
Unitless geometry is measured in twips; conversion rounds to the nearest integer.
DOCX rejects unsupported configuration fields; native output reports retained
fields without a rendering effect. Explicit sections and running stories are
described below.
Document font, size and language defaults map to Word default run properties;
explicit styles and direct attributes keep their existing precedence. Script-only
default font aliases receive the physical Latin fallback. Default sizes use the
existing half-point conversion; invalid or zero results fail explicitly.
[Configuration tests](../tests/configuration.rs) and
[output captures](../tests/docx_configuration.rs) cover this boundary.

Top-level `break` elements support page, column and block line breaks. A pending
column break prefixes the next paragraph, heading or quote where possible;
consecutive/trailing breaks and breaks before lists remain separate paragraphs.
Heading implicit before-spacing is suppressed at column starts, preserving
explicit values. Section boundaries have their own geometry and start semantics.
[Break tests](../tests/breaks.rs) compare models and frozen package parts.

## Parser

`mote::syntax::parse` returns a syntax tree with UTF-8 byte ranges or a diagnostic.
It handles text, hash headings, strong and emphasis spans, literal backtick code,
content escapes, paragraph and hard line breaks, Markdown list items with indented
continuations, quote lines, directive calls with nested bodies, and core argument
expressions. Expressions include strings, finite decimal numbers, booleans,
`nil`, nonempty property objects, ordered arrays (including empty arrays), paths,
and helper calls. Mixed named/unnamed collections are rejected. Names currently
use ASCII identifiers. The syntax tree retains source text for exact preserved
whitespace evaluation.

Calls and paths are parsed without checking whether they are defined. Syntax
success alone does not mean that a document can be evaluated or rendered.

The parser is deliberately incomplete: task lists, fenced code blocks, additional
Markdown forms remain deferred. Lua-enabled argument parsing retains expression source. Unsupported
recognized Markdown forms return diagnostics; this is not a general Markdown
parser. Recursive nesting is limited
to protect the parser from excessive input depth.

## Evaluation

`mote::compile` parses and evaluates a source string into a document model.
Markdown headings, strong spans, and emphasis share semantics with their
directives. Bare inline content becomes paragraphs. Explicit `paragraph`,
`heading`, `span`, `strong`, and `emphasis` bodies accept inline content and reject nested
blocks. Code supports literal scalar arguments and evaluated inline bodies;
backtick spans retain literal text. Explicit `line_break` and `tab` calls retain inline
elements, while Markdown hard breaks are line feeds in text.

Lists support explicit `list`, `ordered_list`, and `item` calls, a positive
`start` attribute on list containers, and string item markers. Explicit item
bodies and `blockquote` bodies normalize ordinary text into paragraphs. List
bodies accept only items and separating whitespace. Markdown items retain their
markers and direct content, including indented continuations and nested lists.
Adjacent items are grouped by kind during block-content normalization. Source
indentation determines Markdown parentage and is not a physical length.
Direct Markdown items in an explicit list body are limited to flat items; nested
content uses an explicit item body or a nested list container.
Each Markdown quote line creates a separate quote block.

The evaluator implements the attribute contracts in
[Formatting attributes](specification/formatting-attributes.md) and
[Spans](specification/spans.md): paragraph/heading geometry and pagination requests, borders,
tab stops, heading numbering, inline font/style references, list layout and
numbering metadata, item checkbox metadata, and quote formatting. Extra attributes
and nested fields are retained without implying formatting support. Declared
outer argument types are checked before merging `properties`; its entries take
precedence, then designated fields normalize. Border/tab contents, strong/emphasis
attributes, and most quote geometry remain unvalidated authored metadata.

`code(preserve_space = true)` preserves whitespace through nested inline bodies
after removing syntactic indentation before directives and block markers, and
normalizes CRLF/CR to LF. Tabs remain literal in the model. Language labels
are metadata; syntax highlighting is not implemented. Style/font references are
stored separately from declarations and resolved formatting. Pagination,
numbering, indentation, and checkbox values record intent without rendering it.

Length helpers are `pt`, `in`, `inch`, `cm`, `mm`, and `twip`; `color` constructs
normalized typed hexadecimal colors. `upper` remains ASCII-only and awaits its
detailed specification. Values retain their documented types and physical units.

Alignment values must match their defined spelling exactly. A heading rejects a
level supplied both directly positionally and by name. Omitted attributes remain
absent from the model, and an explicit empty body still creates its element.

`font` and `style` register document-wide declarations without creating content
nodes. Later declarations replace earlier entries with the same exact name.
Fonts record script-specific typefaces and line-height metadata. Styles support
paragraph/character/table types, relationships, flat properties, and paragraph/run/
table property objects. Table styles apply inherited cell and text defaults,
with conditional-region output limits described below. Numeric
font/style names produce source-located warnings in `Document.warnings`.

Bindings preserve typed values and lexical child scopes for explicit directive
bodies. Static expressions support dotted object paths and zero-based array
segments. Markdown formatting shares its surrounding scope. Six frozen binding
cases compare complete models and DOCX packages; see
[the captures](../tests/fixtures/bindings/README.md).

`compile_with_assets` evaluates source with a selected `AssetResolver` and an
optional logical source name. The `json` helper uses that resolver and snapshots
requests for the compilation. Plain `compile` remains independent of external
resources and reports an explicit resolver requirement for JSON. The CLI selects
a filesystem resolver and shares its snapshot through source loading, evaluation,
and output generation. [JSON tests](../tests/json_data.rs) cover custom resolver
errors, logical identities, repeated requests, source-relative files, stdin,
and complete model/package captures. No dependency is added for data loading.

Conditions and iteration select or repeat ordinary content in lexical child
scopes. Branches preserve surrounding flow; block loops normalize each iteration.
Arrays preserve order and object iteration sorts keys. Seven frozen cases compare
models, complete DOCX packages, native geometry and PNGs. Ten native PDF pages
match the captured raster at 144 DPI; this is not Office visual review. The
[control-flow chapter](specification/control-flow.md) records the generated-section
content duplication limit.

Opt-in Lua expressions use bundled Lua 5.1 through `mlua`, with default features
disabled and only `lua51`/`vendored` enabled. Explicit conversions retain Mote's
value types; no generic serialization layer is added. The runtime opens the base
library with the documented restricted globals, shares one deadline, and routes
JSON through the selected resolver. The normal and build dependency graph was
reviewed; asynchronous runtimes, macros and alternate VM features are disabled.
Lexical functions capture definition-time values, validate named/positional calls,
and convert typed results back to content. The `document` namespace constructs
registered directives through the same validation as authored calls. The
[function](../tests/fixtures/functions/README.md) and
[constructor captures](../tests/fixtures/constructors/README.md) cover models,
packages and native output.
Model support does not demonstrate rendered behaviour or Office conformance.

The [plain-paragraph reference fixture](../tests/fixtures/docx/plain-paragraph/README.md)
contains source, expected JSON, and a captured DOCX package with comparison notes.
Its source/model agreement and generated package parts are tested. The fixture
remains unreviewed in Microsoft Word.

## Formatting resolution

`formatting::StyleResolver` provides explicit named-style, paragraph, inline, and
font-alias resolution. It reads the completed declaration registry, allowing
forward references, and follows `based_on` from base to derived. Paragraph
properties merge recursively with spacing-alias precedence; explicit false and
zero remain meaningful. Missing styles and cycles report resolution issues while
retaining discovered properties. Direct formatting follows named-style values.

Resolution returns new maps without modifying authored attributes. Callers pass
a parent's resolved run properties into each inline child; siblings remain
independent. Font resolution returns declared script mappings or a direct
typeface name without looking up installed fonts. See
[Styles and fonts](specification/styles-and-fonts.md) for the exact contracts.

The DOCX writer supplies the supported implicit output styles separately.
List/container defaults, font-file lookup, glyph coverage and measured
line height remain future work. The resolver does not lay out or render documents.

## JSON inspection

Run the command-line tool with a source file, or omit the file to read standard
input:

```sh
cargo run -- --format json report.mote
printf '%s\n' '#heading(1)[Report]' | cargo run -- --format json -
```

The tool writes the complete evaluated model to standard output. Errors go to
standard error with a source name, line, and column, and return a nonzero exit
status. It accepts one source file. `--help` describes the available options.

JSON contains a root `content` array, configured `page` and `margin` objects, and
a `columns` object. Ordinary nonempty content also appears in one implicit
`sections` entry. Explicit section snapshots retain local geometry, numbering,
header/footer properties and content; root content mirrors the section bodies.
Empty documents omit `sections`, while explicit empty sections remain represented.
Elements have `kind` and `category`; empty `children` and `attributes` are omitted.
Text nodes have `kind = "text"` and omit `text` only when empty. Adjacent fragments
remain distinct, including helper results, escaped characters, and line breaks.
Markdown headings retain `marker = "#"` or `marker = "setext"` alongside their numeric level. Objects retain named fields; lengths carry `value`, `unit`, and a readable
`text` representation. Arrays retain order; typed colors serialize as normalized
hexadecimal strings. Documents with declarations also contain `fonts` and
`styles` arrays sorted by name. Nonempty authored properties and structural
fields are exposed as defined in [Styles and fonts](specification/styles-and-fonts.md).
Empty registries are omitted. Conversion warnings go to standard error and stay
out of JSON. This deterministic inspection format contains no computed layout
geometry and is not yet a stable interchange schema. Nonempty presentation
configuration and explicit slide/box content are retained in the same model.
Custom layout declarations remain in the library model but are currently omitted
from JSON inspection.

## Development

Use a Rust toolchain supporting edition 2024:

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

The conformance test reads the fenced Mote examples directly from the
specification and checks their declared outcomes in
[tests/fixtures/specification.json](../tests/fixtures/specification.json).
Every successful example has an expected document model. Intentional invalid
examples must fail for the documented rule; unsupported implementation is not an
accepted substitute for a defined example. See the
[fixture guide](../tests/fixtures/README.md) when adding or changing examples.

## DOCX output

`docx::emit(&document)` returns a complete Word package without modifying the
model. `mote --format docx -o report.docx input.mote` writes it to a file. The
writer supports paragraphs and headings with captured defaults, Unicode text,
XML escaping, tabs, empty paragraphs, and hard line breaks. Ordinary whitespace
at paragraph and explicit line edges is trimmed, including ordinary tab elements;
interior whitespace is retained. Explicit tabs use Word tab elements and paragraph
tab stops; preserved-code tabs survive edge trimming. Inline control metadata is
validated before trimming so discarded whitespace cannot hide unsupported fields.
Unsupported content, attributes, and declarations return explicit errors before
the CLI opens its output file. JSON remains the default output format.

The package uses ZIP with only DEFLATE enabled, XML generated from the model,
and UTC creation/modification metadata. Paragraph formatting includes alignment,
spacing, indentation, pagination flags, outline levels 0–9, shading, borders and
tab stops. Inline output includes strong/emphasis intent, span formatting, and
code. Nested formatting remains local to each child. Preserved code tabs expand
to four spaces. Direct strong/emphasis attributes and direct code font, size and
color have no DOCX mapping and fail explicitly.

Paragraph and character styles preserve declarations, inheritance, next/link
references, exact/normalized lookup and unique Word identifiers. A declared
Normal style supplies implicit paragraph formatting; Code has a fallback unless
declared. Named character styles clear only the inherited direct fields their declaration
chain supplies, preserving other enclosing formatting. Implicit built-in parent
formatting does not count as a declared property. A direct Code reference retains
its documented fallback font and size. Font aliases map the three script
groups without inventing missing mappings. The font table names physical
script-specific typefaces. `word_line_height` remains metadata for later layout.

Unknown nested properties, missing style references, cycles and unsupported
output values fail explicitly. Declared style sizes must round to a positive
half-point value. Table styles are emitted with conditional regions; font
embedding in DOCX remains deferred. These are output limits; their language/model support is
unchanged. Authored exact line heights are preserved without estimating clipping.

Headings use built-in styles for levels one through six, with declared overrides
and direct paragraph properties. Higher levels need a declared style; levels
above nine map to nine. At the start of the document only implicit before-spacing
is suppressed. Explicit empty style references differ from omitted references.
Heading numbering emits hierarchical Word labels, formats, separators, sequence
identity and restart/continuation. Disabled numbering emits no definition.
Label/prefix metadata does not create text. Heading restart at one creates a new
instance without a start override; list restart at one emits that override.

Bullet and ordered lists emit reusable nine-level numbering definitions. Geometry and
sequence identity determine reuse. List/item indentation, tight spacing, explicit
after-spacing, nested lists and empty items are supported. Item left indentation
is absolute. Item paragraphs flatten into one paragraph and nested lists follow
its text; independent paragraph formatting remains deferred.
Ordered lists preserve all five numeric formats, explicit starts and starts from
source markers. Named instances continue by default; unnamed instances restart
by default. Explicit restart replaces subsequent continuation, while a later start
on a continued sequence is ignored. No new dependency is needed for list output.

Task items use editable Word 2010 checkbox controls or legacy form fields.
Authored item checked state wins over nested state; metadata alone leaves ordinary
list numbering. Tasks do not consume numbered-list positions. Their geometry is
written directly, and control identifiers remain unique across nested lists and
both control kinds. Markdown task syntax and a standalone checkbox directive are
still deferred; this increment uses the existing item attributes.

Block quotes emit one paragraph with the fallback BlockQuote style or a declared
style, plus direct borders, shading and geometry. Nested blocks supply inline text
without separate paragraphs or numbered markers. Nested block formatting remains
unsupported and fails explicitly. Following paragraphs retain their own formatting.

Font measurement is not required for DOCX emission; Word performs layout and
pagination. Production dependencies add ZIP/DEFLATE and UTC metadata support;
XML parsing for comparisons is a test-only dependency. Font-table parsing uses
`ttf-parser` with only `std` and `opentype-layout`; it adds no transitive crates.
Shaping remains deferred. The resolved production graph has no duplicate crate
versions.

[DOCX tests](../tests/docx.rs) compare evaluated models and every captured package
part across [focused fixtures](../tests/fixtures/README.md). JSON numbers are
compared numerically; XML names use namespace URIs. Generation timestamps and
formatting whitespace outside text elements are ignored. Captured advisory
pagination markers are counted separately and omitted; actual break settings
remain compared. Real Word rendering remains **unreviewed**. Structural agreement
does not establish layout conformance.

Package integrity tests also resolve every style relationship, numbering instance,
and internal package target across all captured cases. The writer emits referenced
built-in style declarations even when they are used only as dependencies, and
normalized style names resolve to the actual declared identifier. This prevents
dangling references; reference captures with omitted declarations are not treated
as structurally identical output.

The [output comparison command](../tools/README.md#output-comparison) can rerun
JSON and DOCX comparisons against an explicitly selected reference executable.
It retains source snapshots, both outputs and diagnostics, with input fingerprints
and a machine-readable report. It reports known differences and unsupported cases
separately from matches, and never claims a Word visual review.

## Native layout foundation

The [font measurement fixtures](../tests/fixtures/measurements/README.md) retain
960 frozen cases with bundled, licensed font files. They record text preparation,
layout and painting advances, font extents, variants and rounding behavior.
The [measurement comparison tool](../tools/README.md#font-measurement-comparison)
checks fixture integrity offline and compares independently generated result JSON
without modifying the expectations. Its metric tolerance is too small to hide a
different 1/64-point rounding policy.

The measurement module implements core Helvetica, Times and Courier variants,
plus borrowed static TrueType and OpenType/CFF faces. It prepares missing glyphs,
measures layout and painting advances separately, and retains natural font extents.
Its explicit 26.6 rounding and pair-positioning policy match all 960 captured
cases. Supported pair positioning uses the default Latin/default script's `kern`
features, first matching pair adjustment, or the first legacy horizontal format-0
subtable. This is character measurement, not script shaping.

The parser has a maintenance tradeoff: [ttf-parser](https://github.com/harfbuzz/ttf-parser)
prioritizes correctness fixes and directs new projects toward Fontations. It is
used here behind a small internal boundary for the measured static-font subset,
with no parser types exposed in the public interface. Both parser experiments
matched the captured raw advances and extents; the selected feature set keeps
the dependency graph compact. Broader shaping/variable-font work should reassess
that choice rather than extending the current pair policy into a shaping engine.

[Measurement tests](../tests/measurement.rs) exercise the complete capture and
invalid sizes, missing glyphs and malformed fonts. The
[candidate measurement example](../examples/measure_fonts.rs) emits result JSON
for the offline comparison command without reading the expected measurements.
`TextMeasurer` connects declared variant files to the source asset resolver and
caches exact bytes, missing variants, invalid files and fallback decisions.
Fallback families and substitution reasons remain visible in each result.
It checks embedding eligibility before selecting native font data. Optional
installed-face resolution supplies exact bytes through the same validation path;
font embedding belongs to the PDF painter. DOCX accepts `files` while continuing
to reference physical typeface names.
The line planner wraps styled runs at Unicode break opportunities and uses
extended grapheme clusters for emergency breaks. It handles preserved whitespace,
small caps, hard breaks, code, tabs, four alignments, mixed font sizes and three
line-spacing rules. Its [342 line captures](../tests/fixtures/lines/README.md)
compare every break and fragment position, including boundaries across runs.
Four focused Unicode table libraries supply line breaks, grapheme boundaries,
script properties and letter categories; each adds no runtime dependencies.
Their Unicode data versions differ, so coverage remains explicitly fixture-scoped.

`LayoutStyleResolver` connects document defaults, implicit Normal/heading/Code
styles, inheritance and direct attributes to native run and paragraph properties.
Inline resolution merges adjacent equal runs before measuring so a style boundary
that changes no formatting does not disrupt kerning. The
[91 style captures](../tests/fixtures/layout-styles/README.md) cover the source-to-style
mapping. Paragraph-level planning retains pagination requests; the document planner
resolves them and adds paragraph decoration. Authored custom tab stops remain
unsupported. Native unitless font sizes currently resolve as twips, while DOCX uses points; explicit point lengths avoid this recorded
output discrepancy. It remains a compatibility observation awaiting review.

`plan_paragraph_text` joins source style resolution, inline merging, indentation
frames and line measurement. Its [273 source-to-line captures](../tests/fixtures/paragraph-text/README.md)
check the complete path at three widths, including hanging indents, minimum frame
width and zero-size fallback. Lines remain relative to their paragraph; this API
does not claim numbering, tab-stop or border placement.

`PageGeometry` resolves physical pages and equal-width columns against
[23 geometry captures](../tests/fixtures/page-geometry/README.md).
`plan_paragraph_flow` places measured lines through supplied page areas, with
leading spacing, keep-together and widow/orphan decisions verified against
[672 flow captures](../tests/fixtures/paragraph-flow/README.md). The planner returns
explicit transitions and placements and rejects invalid or non-progressing areas.
It does not independently resolve text or paint output.

`plan_document_text` assembles paragraphs, numbered headings, quotes, nested
bullet/ordered/task lists, and explicit breaks. It collapses adjacent spacing,
applies keep-with-next lookahead, and flows through equal-width columns. Ordered
painting operations retain fill bridges, paragraph rules, marker slots, text,
and column separators. Block preparation, numbering state, pagination and
paragraph painting live in separate modules.

The [document text captures](../tests/fixtures/document-text/README.md) cover 47
successful source plans and four expected invalid-height rejections.
[38 block captures](../tests/fixtures/native-blocks/README.md) and
[19 column captures](../tests/fixtures/native-columns/README.md) additionally verify
complete ordered paint plans. Top-level list items reserve a following line of
space when it fits; task items do not consume ordered numbers. Numbered headings
advance to default half-inch tab stops. Quotes and list-item child paragraphs
flatten to inline text, while nested lists retain their markers.

Text lines also carry highlight rectangles and underline/strike strokes.
[90 decoration captures](../tests/fixtures/text-decorations/README.md) verify their
position, extent, color, thickness, merging and operation order. A shared translation
moves fragments and decorations together into paragraph and page coordinates.
No painter remeasurement is needed for these operations.

Script shaping remains outside the current native output scope. Continuous
section flow and section-boundary column balancing are implemented. Numeric
agreement remains
separate from visual review in Word.

### Native text painters

The `render` module consumes positioned document text without recomputing wrapping
or pagination. `emit_pdf` writes searchable PDF with standard core fonts or subsets
of declared static TrueType/CFF faces; Unicode mappings and actual text preserve
text extraction. `emit_png` rasterizes outlines directly and encodes one image per
page, with a 72-DPI default and an explicit per-page pixel limit. Both paint shared
highlight and decoration geometry and report font substitutions and missing glyphs.

PDF serialization uses `pdf-writer` and `subsetter` with variable-font features
disabled. PNG uses `tiny-skia` and `png`; compression reuses `flate2`. No native
runtime or document-conversion subprocess is required. Raster core fonts select
platform outlines where available, then licensed bundled fallbacks. Declared fonts
are the portable choice for image comparisons.

See [native output captures](../tests/fixtures/native-output/README.md) for validation
and raster tolerances. Both painters cover the current shared document plan,
including lists, quotes, numbered headings, fills, borders and columns. Native
border painting uses `all.color` or `bottom.color` for a 0.35-point bottom rule per
line; other border controls and custom tab stops produce support warnings.
Native property warnings distinguish retained metadata from rendered behavior.
These checks do not establish Microsoft Word or PowerPoint visual conformance.

The command line accepts `--format pdf -o report.pdf` and
`--format png -o report.png --dpi 144` (the single-dash forms are also accepted).
PNG canvas dimensions round up to cover fractional page sizes. PNG writes
`report-1.png`, `report-2.png`, and so on; an output directory uses
`page` as the prefix. `--dpi` is valid only for PNG. Font paths resolve relative
to the source file. Generation errors leave existing output untouched, and source
paths cannot be used as output destinations, including generated PNG page names.

### Installed-face discovery

`FontResolver` supplies exact named font variants independently of source asset
lookup. `SystemFontResolver` implements the narrow macOS catalogue for Aptos,
Aptos Display, Arial and Courier New, checking Word resources before PowerPoint
resources and system copies. Aptos Display uses the sorted Office cloud font
cache. Font family and variant metadata must match; restricted embedding,
collections and unsupported outlines are rejected before use. Proprietary fonts
are read from the host and are not distributed with the crate.

`TextMeasurer::new` remains independent of installed fonts. Its optional resolver
is enabled with `with_font_resolver`. `render::plan` enables system discovery;
`render::plan_with_fonts` accepts a custom resolver or `None` for portable fixture
measurement. Explicit font declarations take precedence. Each session retains
immutable font bytes and rejects conflicting bytes under one logical name, so
measurement and painters cannot accidentally use different faces. The
[installed-font tests](../tests/installed_fonts.rs) exercise this through both PDF
and PNG without requiring proprietary fonts on the test machine.

## External links

Inline `link` content and Markdown links share evaluation and retain their external
URL, title, tooltip and target metadata. DOCX uses content-order relationships and
the Hyperlink character style; PDF annotations use each positioned fragment's
advance and full line height. PNG paints the same text and decorations. Metadata
without an output effect remains documented separately. Fragment-only Markdown
links lower to internal references, described below.

[Link tests](../tests/links.rs) compare three independently captured models and
complete DOCX packages, validate argument/body rules, and check wrapped PDF link
areas. Native PDF pixels and extracted destinations are checked independently;
real Word rendering remains unreviewed.

## Images

`docx::emit_with_assets` accepts an asset resolver and referring source name,
returning package bytes and warnings. `docx::emit` uses the current directory.
Native planning freezes decoded/transformed pixels before either painter runs.
PNG decoding reuses the existing library; JPEG and first-frame GIF decoding use
narrow dependencies without unrelated format support. Input and transformed
images have a 64-million-pixel allocation limit. Untransformed JPEG PDF resources
preserve compressed source data and their grayscale, RGB or CMYK color space.

[Image captures](../tests/fixtures/images/README.md) cover model/package equality,
image geometry, transforms, links, fallbacks and separate native raster output.

## Tables

The table, row and cell model supports Markdown pipe tables and explicit
structure. A shared logical grid records row/column spans; DOCX modules separately
handle property emission, width hints, nested content and conditional styles.
Physical borders and cell defaults retain their authored precedence.

Native table modules separately resolve column widths, cell content, shared
physical edges, safe row fragments and placement. They use the existing text
measurement and line layout, and return the same ordered page operations to PDF
and PNG. Repeated headers, row/column spans, nested tables, images, lists and links
participate in that layout. Speculative width measurement clones numbering state
so it cannot consume a real list or heading number. Cell clipping applies to both
painters; the PNG painter reuses one mask through nested clips.

Conditional row styles supply missing cell defaults. First/last-column regions,
column banding, table indent/cell spacing/layout selection, row height/rules and
cell text direction/no-wrap are retained for editable DOCX but have no native
effect and produce warnings. Cell shading covers the physical cell; paragraph
bands and paragraph borders inside a cell are omitted, while run highlights paint.
Oversized unsplittable rows warn about clipping; oversized merged groups without
a safe continuation warn and fall back to individual rows.

See [table captures](../tests/fixtures/tables/README.md) for frozen model/package,
geometry and native raster evidence, including the blocked repeated-header case.
Microsoft Word visual review remains unreviewed.

## Sections and running stories

Explicit sections inherit document geometry and retain local page, margin, column,
start, numbering and vertical-alignment properties. Header/footer declarations
provide defaults or local overrides, including empty resets. Inherited stories
retain declaration identity so anchor metadata counts each declaration once.
DOCX writes section properties and independently related header/footer parts.

Native layout owns geometry and story selection per physical page. Continuous
sections share a page where their page size permits it, and intermediate multi-column
sections balance their final area. Current/total page fields and page references
use bounded pagination passes; authored models remain unchanged. Running text uses
full-width header/footer bands, and overflow removes complete trailing lines with
a warning. Native fields currently use decimal text, section vertical alignment
is ignored, and parity/next-column section starts advance to a page. Story media
and tables use explicit text fallbacks.

[Section captures](../tests/fixtures/sections-stories/README.md) cover 14 cases and
41 pages, including mixed page sizes, section numbering restarts, column balancing,
story inheritance, resets and overflow.

## Anchors, references and captions

Labels are unique across retained body and story declarations. References require
a declared label and support forward targets, title/number/page/text selection,
explicit fallback text and styled Markdown children. Replaced stories that were
never used contribute no labels or references. Authored bookmarks occupy a separate
Word namespace. Stable label names use SHA-256 suffixes; the focused `sha2`
dependency supplies this hashing without default features.

DOCX emits bookmark ranges, internal hyperlinks, editable REF/PAGEREF fields and
caption SEQ fields. Native PDF emits internal GoTo actions at the placed body
destination; PNG shares its text, styling and geometry. Page references use the
displayed section page number. Unplaced targets retain visible text and warn
without creating fabricated PDF links.

Figures group existing blocks with one optional caption. Caption numbering shares
counter formatting with headings and lists, supports independent sequences and
restarts, and supplies inferred label title/number metadata. Native layout orders
captions by position; DOCX retains authored order. Direct figure alignment applies
to compatible children; `placement.align` is a DOCX-only fallback. Figure style
and floating placement remain metadata. Native cells and running stories omit
caption numbering even though those captions contribute to reference metadata;
the resulting number mismatch is a documented, warned limitation.

[Reference captures](../tests/fixtures/references/README.md) cover seven cases and
13 pages. [Caption captures](../tests/fixtures/captions/README.md) cover ten cases
and 12 pages. Tests compare complete models/packages, native geometry and native
PNGs. PDF rasters matched at 144 DPI with zero changed pixels, and link rectangles,
destination heights and target pages were independently compared. All captured
native PDF pages were visually inspected. Microsoft Word review remains unreviewed.


## Notes

Footnotes and endnotes retain block bodies behind inline references. Markdown note
definitions are collected before evaluation, evaluated lazily, cached, and checked
for recursive use. DOCX writes independent note stories, automatic superscript
reference markers, separator definitions, and per-story relationships. Nested notes
and annotations in running stories retain visible text with warnings.

Native note selection and measurement live in `layout::notes`, with separate flow
and painting modules. Stable single-paragraph notes in one single-column section
participate in body pagination. Footnote reservations reduce the available space
for following text, images, and tables. Endnotes enter document flow and use distinct
continuation separators. Note selection is recreated for each pagination pass;
authored models remain unchanged. Unsupported contexts use visible, warned fallbacks.
Oversized notes remain a warned limit. Endnote continuation may overlap footer text
unless an explicit footer height is reserved.

[Note captures](../tests/fixtures/notes/README.md) cover eleven cases and 19 pages,
with full model/package comparisons, geometry, PNGs, and a recorded PDF raster
exception for negative off-page coordinates. Word review remains unreviewed.

## Drawings and charts

Positioned shapes emit editable Word DrawingML with text, preset geometry,
transforms, fills, borders and shadows. Native shape paths and fitted text use
shared measurement and the page painters. Line declarations retain connector
geometry and markers in the model; document output does not paint them.
See [shape captures](../tests/fixtures/shape-text/README.md) for output limits.

Charts resolve data, defaults and appearance once. DOCX writes editable chart
parts with embedded workbooks for twelve kinds. Native document charts cover
column/bar, stacked and percentage variants, line, area, stacked/percentage area,
pie and doughnut. The shared geometry includes titles, legends, axes, grids and
supported labels. Chart labels, signed percentage stacks and mismatched data
lengths retain the limits recorded in the [chart checklist](checklists/chart.md).
Native table-cell charts remain unsupported. Positioned presentation charts use
the same geometry with deck theme colors and fonts.

## Presentations

`pptx::emit_with_assets` generates editable slides with positioned text, pictures,
tables, shapes, connectors and charts. Deck defaults and slide overrides resolve
once in `presentation`; editable serializers and native layout consume that state.
Shape XML, chart workbooks, image resources, font measurement and native painters
reuse the document components. Text fitting computes shrink ratios and resized
heights before serializing editable boxes. Source-relative font assets and image
resources use the existing resolver and cache.

Background images support contain, cover and stretch beneath foreground objects;
a slide override replaces the entire deck background. Footer text, fixed or
automatic dates and slide numbers occupy fixed footer frames and can be suppressed
independently per slide. Automatic dates use the local calendar once per output,
with a warning and UTC fallback if the local time zone is unavailable. PPTX keeps
editable date/number fields and plain speaker notes; native slides paint the field
values and omit notes. Notes use their own parts, relationships and master.

The CLI accepts `--format pptx`. PDF/PNG selects slide layout for nonempty explicit
presentation configuration, or with `--layout presentation`. `--layout document`
selects document flow explicitly; ordinary Markdown remains document content.
Presentation mode segments top-level headings/dividers and explicit slides,
then resolves automatic content frames or built-in/custom named layouts. Slots
materialize in layout order, followed by ordinary positioned objects. Layout
inheritance resolves iteratively, with recursive property overrides, typed and
required slots, removals and layout-level metadata. The authored model remains
unchanged. `#slide[]` creates a blank slide. Text-box shadows remain unsupported;
retained properties without an effect warn after inherited properties resolve.

All eight built-ins adapt to slide dimensions. Custom declarations preserve their
order, allow forward inheritance and replace repeated names in place. JSON values
can declare slots; Lua can declare derived layouts and provide content. Empty Lua
box round-trips currently prevent Lua slot declarations; the language chapter
records this boundary. Source-relative images and fonts use the same resolver for
inherited layouts and ordinary slides.

Fixed captures compare complete models and canonical packages, including embedded
chart workbooks, plus ordered native geometry and independent PNG output. The
[text](../tests/fixtures/presentation-text/README.md),
[picture](../tests/fixtures/presentation-images/README.md),
[table](../tests/fixtures/presentation-tables/README.md),
[drawing](../tests/fixtures/presentation-shapes/README.md),
[chart](../tests/fixtures/presentation-charts/README.md),
[background](../tests/fixtures/presentation-backgrounds/README.md) and
[field/notes](../tests/fixtures/presentation-fields/README.md),
[automatic layout](../tests/fixtures/presentation-automatic/README.md),
[Markdown decks](../tests/fixtures/presentation-markdown/README.md),
[built-in layouts](../tests/fixtures/presentation-layouts/README.md) and
[custom layouts](../tests/fixtures/presentation-custom/README.md) capture guides record
coverage, raster bounds and remaining output differences. The field cases also
combine all supported positioned object kinds in one slide. These checks do not
establish PowerPoint conformance; the new cases remain unreviewed in Office.
