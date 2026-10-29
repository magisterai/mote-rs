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
| [evaluation](../src/evaluation.rs) | Content flow and directive bodies, with separate expression, attribute, declaration, and value-normalization modules. |
| [model](../src/model.rs) | Evaluated content, values, and declaration registries. |
| [formatting](../src/formatting.rs) | Style inheritance and font-alias resolution. |
| [docx](../src/docx.rs) | Editable Word output, with separate content, properties, styles, fonts, XML and package modules. |
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
backtick spans retain literal text. Explicit `line_break` calls retain an inline
element, while Markdown hard breaks are line feeds in text.

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

Document configuration, bindings, JSON loading, defaults, Lua, layout, and
Office/PDF/PNG output remain deferred.
Model support does not demonstrate rendered behaviour or Office conformance.

The [plain-paragraph reference fixture](../tests/fixtures/docx/plain-paragraph/README.md)
contains source, expected JSON, and a captured DOCX package with comparison notes.
Its source/model agreement is tested. DOCX generation and package comparison are
not implemented; this fixture remains unreviewed in Microsoft Word.

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

Implicit output styles, document/list/container defaults, font-file lookup,
glyph coverage, measured line height, and output-specific formatting remain
future work. The resolver does not lay out or render documents.

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

JSON contains a root `content` array and empty `page`, `margin`, and `columns`
objects until document configuration is implemented. Nonempty content also
appears in one implicit `sections` entry, alongside empty section geometry,
numbering, and header/footer property objects. Empty content omits `sections`.
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
writer supports paragraphs with the captured document defaults, Unicode text,
XML escaping, tabs, empty paragraphs, and hard line breaks. Ordinary whitespace
at paragraph and explicit line edges is trimmed; interior whitespace is retained.
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
declared. Named character styles clear only the inherited direct fields they
supply, preserving other enclosing formatting. Font aliases map the three script
groups without inventing missing mappings. The font table names physical
script-specific typefaces. `word_line_height` remains metadata for later layout.

Unknown nested properties, missing style references, cycles and unsupported
output values fail explicitly. Declared style sizes must round to a positive
half-point value. Table styles, headings, lists, quotes, font files and embedding
remain deferred. These are output limits; their language/model support is
unchanged. Authored exact line heights are preserved without estimating clipping.

Font measurement is not required for DOCX emission; Word performs layout and
pagination. Production dependencies add ZIP/DEFLATE and UTC metadata support;
XML parsing for comparisons is a test-only dependency. Measurement and shaping
libraries remain deferred. The resolved production graph has no duplicate crate
versions.

[DOCX tests](../tests/docx.rs) compare evaluated models and every captured package
part across [six focused fixtures](../tests/fixtures/README.md). JSON numbers are
compared numerically; XML names use namespace URIs. Generation timestamps and
formatting whitespace outside text elements are ignored. Captured advisory
pagination markers are counted separately and omitted; actual break settings
remain compared. Real Word rendering remains **unreviewed**. Structural agreement
does not establish layout conformance.
