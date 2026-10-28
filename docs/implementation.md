# Implementation

The Rust crate is at an initial language-core stage. The
[specification](specification/README.md) describes the language; this page records
what this crate currently implements.

## Parser

`mote::syntax::parse` returns a syntax tree with UTF-8 byte ranges or a diagnostic.
It handles text, hash headings, `**strong**`, content escapes, paragraph breaks,
directive calls with nested bodies, and core argument expressions. Expressions
include strings, finite decimal numbers, booleans, `nil`, nonempty property
objects, paths, and helper calls. Names currently use ASCII identifiers.

Calls and paths are parsed without checking whether they are defined. Syntax
success alone does not mean that a document can be evaluated or rendered.

The parser is deliberately incomplete: additional Markdown forms, collections,
control flow, and Lua are deferred. Unsupported recognized Markdown forms return
diagnostics; this is not a general Markdown parser. Recursive nesting is limited
to protect the parser from excessive input depth.

## Evaluation

`mote::compile` parses and evaluates a source string into a document model.
Markdown headings and strong spans share semantics with `heading` and `strong`
directives. Bare inline content becomes paragraphs. Explicit `paragraph`,
`heading`, and `strong` bodies accept inline content and reject nested blocks.

The initial evaluator supports heading levels, paragraph alignment, the
`keep_with_next`, `keep_together`, `page_break_before`, and `widow_control`
booleans, and `spacing.before` and `spacing.after`. Length helpers are `pt`, `in`,
`inch`, `cm`, `mm`, and `twip`. `upper` currently accepts ASCII text only. Values
retain their types and physical units. Other attributes and helpers report that
they are unsupported.

Alignment values must match their defined spelling exactly. A heading rejects a
level supplied both positionally and by name. Omitted attributes remain absent
from the model, and an explicit empty body still creates its element.

The pagination booleans and `upper` have initial implementation coverage but
await their detailed specification chapters. Introductory examples use only the
defined basic content and length helpers.

Document configuration, bindings, JSON loading, styles, defaults, Lua, layout,
and Office/PDF/PNG output remain deferred. The model records formatting intent;
it does not demonstrate rendered behaviour or Office conformance.

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

JSON contains a root `content` array. Elements have `kind`, `category`, and
`children`, plus `attributes` when present. Text nodes have `kind = "text"` and
`text`. Objects retain named fields; lengths carry `value`, `unit`, and a readable
`text` representation. This deterministic inspection format contains no layout
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
