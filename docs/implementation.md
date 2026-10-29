# Implementation

The Rust crate implements an initial language core and a progressive DOCX writer. The
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
| [docx](../src/docx.rs) | Editable Word output, with separate document flow, list/numbering, properties, styles, fonts, XML and package modules. |
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

This implements the source-loading boundary only. Imports, data-loading helpers,
image/font consumers, per-compilation caching, and consistency checks for repeated
logical names remain deferred. Resolvers must supply stable bytes for a logical
name during a compilation. Tests in [source.rs](../tests/source.rs) and
[cli.rs](../tests/cli.rs) cover custom sources, relative names, failure propagation,
encoding, and CLI diagnostics.

## Document configuration

`document` declarations retain authored arguments and merge normalized page,
margin and column fields. They produce no visible node, including when evaluated inside a
body. JSON preserves both the last authored argument and the merged geometry,
which also appears on the implicit section. DOCX maps named paper sizes, custom
dimensions, orientation, all seven margin distances and equal-width columns to a
single section. Columns support count, gap and separator; an omitted count emits
no column property. Unequal-width columns are retained in JSON but rejected by
DOCX until individual column geometry is implemented.
Unitless geometry is measured in twips; conversion rounds to the nearest integer.
Unsupported configuration fields fail explicitly at emission. Explicit sections and header/footer content remain deferred.
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
explicit values. Nested breaks and section-related types remain deferred.
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
Markdown forms, control flow, and Lua are deferred. Unsupported
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
table property objects. Table declaration normalization is available; applying
table styles and resolving conditional regions await the table model. Numeric
font/style names produce source-located warnings in `Document.warnings`.

Explicit sections, bindings, JSON loading, Lua, native layout,
PowerPoint/PDF/PNG output remain deferred.
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
a `columns` object. Nonempty content also appears in one implicit
`sections` entry with the same geometry, plus empty section, numbering, and
header/footer property objects. Empty content omits `sections`.
Elements have `kind` and `category`; empty `children` and `attributes` are omitted.
Text nodes have `kind = "text"` and omit `text` only when empty. Adjacent fragments
remain distinct, including helper results, escaped characters, and line breaks.
Markdown headings retain `marker = "#"` alongside their numeric level. Objects retain named fields; lengths carry `value`, `unit`, and a readable
`text` representation. Arrays retain order; typed colors serialize as normalized
hexadecimal strings. Documents with declarations also contain `fonts` and
`styles` arrays sorted by name. Nonempty authored properties and structural
fields are exposed as defined in [Styles and fonts](specification/styles-and-fonts.md).
Empty registries are omitted. Conversion warnings go to standard error and stay
out of JSON. This deterministic inspection format contains no computed layout
geometry and is not yet a stable interchange schema. JSON output does not enable
the deferred `json(...)` data-loading helper.

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
half-point value. Table styles, font files and embedding
remain deferred. These are output limits; their language/model support is
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
XML parsing for comparisons is a test-only dependency. Measurement and shaping
libraries remain deferred. The resolved production graph has no duplicate crate
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

This is a comparison foundation. Native font measurement, shaping, paragraph
layout, pagination, PDF and PNG generation remain unimplemented. No measurement
dependency or new language support is introduced by these fixtures. Agreement
with numeric measurements will be a separate check from visual review in Word.
