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

## Development

Use a Rust toolchain supporting edition 2024:

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```
