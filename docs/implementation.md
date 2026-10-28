# Implementation

The Rust crate is at an initial language-core stage. The
[specification](specification/README.md) describes the language; this page records
what this crate currently implements.

The [directive checklists](checklists/README.md) track attribute-level work.
Checked items have a specification, implementation, and tests. Partial model
support is noted beside unchecked items; output verification is tracked separately.

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
`heading`, `strong`, and `emphasis` bodies accept inline content and reject nested
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
[Formatting attributes](specification/formatting-attributes.md): paragraph/heading geometry and pagination requests, borders,
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
stored without declaration lookup or effective-style resolution. Pagination,
numbering, indentation, and checkbox values record intent without rendering it.

Length helpers are `pt`, `in`, `inch`, `cm`, `mm`, and `twip`; `color` constructs
normalized typed hexadecimal colors. `upper` remains ASCII-only and awaits its
detailed specification. Values retain their documented types and physical units.

Alignment values must match their defined spelling exactly. A heading rejects a
level supplied both directly positionally and by name. Omitted attributes remain
absent from the model, and an explicit empty body still creates its element.

Document configuration, bindings, JSON loading, style/font declarations and
resolution, defaults, Lua, layout, and Office/PDF/PNG output remain deferred.
Model support does not demonstrate rendered behaviour or Office conformance.

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
hexadecimal strings. This deterministic inspection format contains no computed layout
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
