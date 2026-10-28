# Directive checklists

[Documentation](../README.md) · [Specification](../specification/README.md)

These checklists record the planned scope and current completion of each
directive. They begin with the ten directives already introduced. Add a page for
each future directive before writing its specification or implementation.

## Reading status

- A checked item has a language definition, a working Rust implementation, and
  tests for the behaviour stated on that item.
- An unchecked item remains work to do. Notes identify model support that already
  exists but still needs specification or tests.
- A check applies only to the stated behaviour. Recording a length or marker does
  not establish its layout, appearance, or output support.
- Output tasks remain unchecked until their own mappings and verification exist.
  No current directive has Rust Office, PDF, or PNG output.

All current directive attribute tasks are specified in
[Formatting attributes](../specification/formatting-attributes.md), implemented,
and covered by [attribute tests](../../tests/attributes.rs). Effective style
resolution and output tasks remain open. Future pending descriptions are planning
notes; define their concepts before using them in examples.

## Directives

| Directive | Checklist | Current language coverage |
| --- | --- | --- |
| `paragraph` | [Paragraph](paragraph.md) | Body and full scoped paragraph attributes. |
| `heading` | [Heading](heading.md) | Body, level, geometry, and numbering attributes. |
| `strong` | [Strong](strong.md) | Bold intent, inline forms, font/style overrides. |
| `emphasis` | [Emphasis](emphasis.md) | Italic intent, inline forms, font/style overrides. |
| `code` | [Code](code.md) | Inline forms, formatting, metadata, preserved whitespace. |
| `line_break` | [Line break](line-break.md) | Explicit inline breaks and Markdown hard breaks. |
| `list` | [List](list.md) | Item container, nesting, scoped list attributes. |
| `ordered_list` | [Ordered list](ordered-list.md) | Item container, nesting, numbering/layout attributes. |
| `item` | [Item](item.md) | Block body, markers, layout and task metadata. |
| `blockquote` | [Block quote](blockquote.md) | Block body, quote lines, scoped formatting attributes. |

## Scope and output boundaries

The attribute inventory is a bounded implementation goal. It includes common
document controls and explicitly identified native or presentation controls.
An attribute's presence does not imply that every output supports it. Metadata
has its own stated purpose and must not be advertised as a rendering feature.

Do not add arbitrary OOXML properties, character spacing, kerning, new numbering
formats, or new marker-layout controls to this scope. A property accepted into a
generic object without an implemented effect is not a formatting target.

Dependencies such as styles, fonts, colors, arrays, and presentation text objects
must receive definitions before examples use them. The checklist for a dependent
directive can exist before those definitions; its tasks remain unchecked.

## Shared attribute work

- [x] Retain extra element properties without implying a formatting effect;
  validate declared outer argument types before merging `properties`, then
  normalize only the receiving directive's designated fields. Preserve extra
  nested fields; spacing and indent maps normalize every length entry.
- [x] JSON inspection omits empty optional fields and includes document geometry
  objects and implicit section content.
- [x] Array expressions for ordered tab stops, including empty arrays; reject
  mixed named and positional collection entries.
- [x] Typed `color` helper and hexadecimal color values.
- [x] Preserve code-body whitespace through nested inline content and normalize
  its line endings without changing ordinary flowing text.
- [x] `properties`: an attribute object flattened into the receiving element's
  attributes. Its entries override same-named outer arguments after signature
  type checks. Designated fields normalize; extra fields remain metadata. This
  does not establish arbitrary OOXML output mappings.
- [x] Shared value rules for colors, font names and aliases, named styles, and
  boolean normalization. Preserve the distinction between omitted and explicit
  values.
- [x] Length validation and model storage, including unitless values and retained
  flat/nested spacing fields; physical resolution remains layout work.
- [ ] Style resolution, inheritance, and direct-attribute precedence. Keep a
  reference to a style distinct from resolving its effective properties.

The current attribute implementation work covers evaluation, validation, and
model storage. Style/font references can be complete at that layer while style
declarations, effective formatting, layout, and output tasks remain separate.

## Checklist-first workflow

1. Create a page named for the directive and add it to the index above. Use
   sections for content and arguments, attributes, dependencies or limits, output,
   and evidence.
2. List each attribute, its nested fields, supported value forms, and any
   output-specific or metadata-only limits. Do not infer support from a permissive
   parser or a retained property alone.
3. Select a small, coherent set of unchecked tasks. Define its concepts and
   invalid uses before adding examples, then implement and test those rules.
4. Check only completed tasks and link the specification, implementation, and
   tests. Keep partial work unchecked with an explanation. Update the
   [implementation overview](../implementation.md) when coverage changes.
5. Verify output separately for each supported format and layout mode. Use real
   Word and PowerPoint for their respective visual conformance reviews; native
   PDF and PNG require independent native output checks.

The existing [specification example fixtures](../../tests/fixtures/README.md)
provide model expectations and intentional diagnostics. They establish language
behaviour, not visual conformance.
