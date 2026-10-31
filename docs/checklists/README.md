# Directive checklists

[Documentation](../README.md) · [Specification](../specification/README.md)

These checklists record support for each directive and its attributes, including
content directives and document-wide declarations. Add a page for each future
directive before its specification or implementation. Check supported behaviour
and leave unsupported behaviour unchecked, explaining partial support beside the
item. Output-specific limits belong with the relevant attributes; generation and
review procedures belong with implementation tests and fixtures.

The [specification attribute inventories](attributes/README.md) cover the full
relevant property families, nested fields, and allowed values from the retained
OOXML and related specifications. Each directive links to its applicable families.
Unsupported standard properties remain visible and unchecked; partial mappings
name the supported Mote subset. These inventories extend beyond the current
implementation scope.

## Reading status

- A checked item has a language definition, a working Rust implementation, and
  tests for the behaviour stated on that item.
- An unchecked item remains work to do. Notes identify model support that already
  exists but still needs specification or tests.
- A check applies only to the stated behaviour. Recording a length or marker does
  not establish its layout, appearance, or output support.
- DOCX supports paragraphs, inline formatting, styles and font mappings as
  described in the individual output notes. Native PDF/PNG covers the current text
  and list features, including columns; retained properties without a painting
  effect produce diagnostics. Output support and
  visual review status remain separate from the language/model support recorded
  here.

Content attribute contracts are specified in
[Formatting attributes](../specification/formatting-attributes.md) and
[Spans](../specification/spans.md). [Styles and fonts](../specification/styles-and-fonts.md)
defines declarations and explicit formatting resolution. Each checklist links its
implementation and tests. Table-style application and broader native document
content remain open. Native measurement and text geometry are covered in the
[implementation overview](../implementation.md); those numeric captures do not
establish painted output support. Future pending descriptions are planning notes; define
their concepts before using them in examples.

## Directives

| Directive | Checklist | Current language coverage |
| --- | --- | --- |
| `csv` | [CSV data](csv.md) | Specified and tested; see the checklist for output limits. |
| `import` | [Imported declarations](import.md) | Specified and tested; see the checklist for output limits. |
| `set` | [Scoped defaults](set.md) | Specified and tested; see the checklist for output limits. |
| `raw` | [Literal block content](raw.md) | Specified and tested; see the checklist for output limits. |
| `divider` | [Horizontal dividers](divider.md) | Specified and tested; see the checklist for output limits. |
| `definition_list` | [Definition lists](definition-list.md) | Specified and tested; see the checklist for output limits. |
| `term` | [Definition terms](term.md) | Specified and tested; see the checklist for output limits. |
| `definition` | [Definitions](definition.md) | Specified and tested; see the checklist for output limits. |
| `underline` | [Underline](underline.md) | Specified and tested; see the checklist for output limits. |
| `strike` | [Strikethrough](strike.md) | Specified and tested; see the checklist for output limits. |
| `highlight` | [Highlighted text](highlight.md) | Specified and tested; see the checklist for output limits. |
| `small_caps` | [Small capitals](small-caps.md) | Specified and tested; see the checklist for output limits. |
| `subscript` | [Subscript](subscript.md) | Specified and tested; see the checklist for output limits. |
| `superscript` | [Superscript](superscript.md) | Specified and tested; see the checklist for output limits. |
| `function` | [Functions and scripting](function.md) | Lexical functions, content bodies, opt-in Lua, typed values and document constructors. |
| `if` | [Conditional content](if.md) | Truthiness, selected branches, child scopes and surrounding flow. |
| `for` | [Iteration](for.md) | Ordered array/object iteration, local bindings and generated content. |
| `json` | [JSON data](json.md) | Typed JSON values, source-relative resolution and compilation snapshots. |
| `let` | [Bindings](let.md) | Typed values, lexical scopes, content insertion and property paths. |
| `footnote` | [Footnotes](footnote.md) | Inline references, block bodies, identifiers, and Markdown definitions. |
| `endnote` | [Endnotes](endnote.md) | Inline references, block bodies, and identifiers. |
| `section` | [Sections](section.md) | Geometry, boundaries, page numbering, and local running stories. |
| `header` | [Headers](header.md) | Inherited and local header declarations, sizing, and content. |
| `footer` | [Footers](footer.md) | Inherited and local footer declarations, sizing, and content. |
| `page_number` | [Page numbers](page-number.md) | Current and total page fields with formatting metadata. |
| `bookmark` | [Bookmarks](bookmark.md) | Named content ranges and native destinations. |
| `label` | [Labels](label.md) | Invisible named targets with title and number metadata. |
| `ref` | [Cross-references](ref.md) | Forward references, display modes, fallbacks, and internal links. |
| `caption` | [Captions](caption.md) | Caption text, placement, numbering sequences, and references. |
| `figure` | [Figures](figure.md) | Grouped content, alignment, and caption association. |
| `table` | [Table](table.md) | Row structure, sizing, defaults, borders and styles. |
| `row` | [Row](row.md) | Cell structure, height, repeating headers and splitting. |
| `cell` | [Cell](cell.md) | Content, spans, margins, alignment and borders. |
| `paragraph` | [Paragraph](paragraph.md) | Body and full scoped paragraph attributes. |
| `heading` | [Heading](heading.md) | Body, level, geometry, and numbering attributes. |
| `strong` | [Strong](strong.md) | Bold intent, inline forms, font/style overrides. |
| `emphasis` | [Emphasis](emphasis.md) | Italic intent, inline forms, font/style overrides. |
| `code` | [Code](code.md) | Inline forms, formatting, metadata, preserved whitespace. |
| `tab` | [Tab](tab.md) | Inline tabs and paragraph tab-stop interaction. |
| `line_break` | [Line break](line-break.md) | Explicit inline breaks and Markdown hard breaks. |
| `list` | [List](list.md) | Item container, nesting, scoped list attributes. |
| `ordered_list` | [Ordered list](ordered-list.md) | Item container, nesting, numbering/layout attributes. |
| `item` | [Item](item.md) | Block body, markers, layout and task metadata. |
| `blockquote` | [Block quote](blockquote.md) | Block body, quote lines, scoped formatting attributes. |
| `image` | [Image](image.md) | Block media and placement. |
| `link` | [Link](link.md) | External inline destinations and metadata. |
| `presentation` | [Presentation](presentation.md) | Deck properties, normalized dimensions, overrides and Lua declarations. |
| `layout` | [Reusable presentation layouts](layout.md) | Layout names, inheritance, typed slots and defaults. |
| `slide` | [Slide](slide.md) | Explicit bodies, positioned/loose content rules, appearance and metadata. |
| `box` | [Box](box.md) | Positioned content, text/appearance properties and child validation. |
| `chart` | [Chart](chart.md) | Typed data and appearance; editable document charts and native chart geometry. |
| `shape` | [Shape](shape.md) | Positioned shape geometry, appearance and text. |
| `line` | [Line](line.md) | Positioned line endpoints, strokes and arrow markers. |
| `span` | [Span](span.md) | General inline formatting and explicit resolution. |
| `break` | [Break](break.md) | Block page, column and line breaks; section types deferred. |
| `document` | [Document configuration](document.md) | Document-wide page and margin geometry. |
| `font` | [Font declaration](font.md) | Named script mappings and alias resolution. |
| `style` | [Style declaration](style.md) | Style declarations, paragraph/run inheritance, and table-property storage. |

## Scope and output boundaries

Inventory all applicable specification properties, including character spacing,
kerning, additional numbering formats, marker controls, and unsupported values.
The inventory records capabilities and gaps; it does not set implementation goals.
An attribute's presence does not imply that every output supports it. Metadata
has its own stated purpose and must not be advertised as a rendering feature.

Implementation remains limited to the established Mote compatibility baseline.
Confirm that a capability belongs to that approved feature set before implementing
it. An unchecked standard property is not permission to extend the language.
A property accepted into a generic object without an implemented effect is not
supported formatting.

Dependencies such as styles, fonts, colors, arrays, and presentation text objects
must receive definitions before examples use them. The checklist for a dependent
directive can exist before those definitions; its tasks remain unchecked.

## Shared attribute work

- [x] Retain extra element properties without implying a formatting effect;
  validate declared outer argument types before merging `properties`, then
  normalize only the receiving directive's designated fields. Preserve extra
  nested fields; spacing and indent maps normalize every length entry.
- [x] JSON inspection uses sorted declaration arrays, omits empty optional
  fields, and includes document geometry objects and implicit section content.
- [x] Numeric declaration names produce source-located warnings, available to
  library callers and printed separately from JSON by the CLI.
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
- [x] Explicit style resolution, inheritance, and direct-attribute precedence.
  Keep authored references separate from resolved properties; report missing
  styles and cycles. See the [style checklist](style.md).

The current attribute implementation work covers evaluation, validation, and
model storage. Declaration and explicit resolution tasks now have their own
checklists; their completion does not establish layout or output conformance.
DOCX has implicit Normal, heading, Code and BlockQuote styles, plus document
font, size and language defaults. Broader list/container defaults remain deferred.

## Checklist-first workflow

1. Create a page named for the directive and add it to the index above. Use
   sections for content and arguments, attributes, support limits, and evidence.
2. Inventory every relevant standard property, nested field, and value domain;
   link shared families and map existing Mote attributes. Leave missing or partial
   support unchecked, with output-specific or metadata-only limits stated.
3. Select a small, coherent set of attributes or behaviours within the approved
   implementation scope. Define
   the concepts and invalid uses before adding examples, then implement and test
   those rules.
4. Check only supported behaviour and link the specification, implementation, and
   tests. Keep partial work unchecked with an explanation. Update the
   [implementation overview](../implementation.md) when coverage changes.
5. Verify output separately for each supported format and layout mode. Use real
   Word and PowerPoint for their respective visual conformance reviews; native
   PDF and PNG require independent native output checks.

The existing [specification example fixtures](../../tests/fixtures/README.md)
provide model expectations and intentional diagnostics. They establish language
behaviour, not visual conformance.
