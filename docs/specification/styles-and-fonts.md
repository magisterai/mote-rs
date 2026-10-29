# Styles and fonts

[Specification index](README.md) · [Spans](spans.md)

A **declaration** registers a named resource in the document. It produces no
content node. Font declarations name typefaces; style declarations name reusable
formatting. A **run** is a portion of text sharing character formatting, so a
style's `run` object describes character properties while its `paragraph` object
describes paragraph properties.

Evaluation retains declarations and authored element attributes separately.
**Resolution** looks up declarations and combines their formatting. It does not
lay out text, select installed font files, or render output.

## 1. Font declarations

`font` takes named arguments and no body. Its required `name` is a nonempty
string or a number converted to a string. Whitespace-only names are invalid.
Names retain their spelling and are case-sensitive. Numeric names produce a
warning identifying the conversion. Warnings are available in `Document.warnings`
and printed to standard error by the CLI; they do not enter JSON output.

| Attribute | Meaning |
| --- | --- |
| `latin` | Typeface for Latin text. |
| `east_asian` | Typeface for East Asian text. |
| `complex_script` | Typeface for complex-script text. |
| `word_line_height` | Positive finite number describing Word line-height calibration metadata. |
| `files` | Optional object mapping `regular`, `bold`, `italic`, and `bold_italic` to font asset references for native measurement. |

Typeface values are nonempty strings. All four fields are optional and omitted
fields stay absent. `word_line_height` requires a number, not a numeric string;
it records metadata without measuring or changing line height during evaluation.

```mote
#font(name = "body", latin = "Aptos", east_asian = "Yu Gothic", complex_script = "Arial", word_line_height = 1.25)
#paragraph[#span(font = "body")[Hello]]
```

The document registers `body`, and its span retains the reference `body`.
Resolving that reference returns the declared mappings and metadata. It does not
assert that any of the typefaces is installed. An undeclared font reference is
retained as a direct Latin typeface name. Declared aliases use exact matching and
do not recursively resolve typeface strings as further aliases. Missing script
fields in an alias remain unspecified; they are not silently filled from `latin`.

For native measurement, `files` selects the exact requested bold/italic variant.
For example, `files = { regular = "fonts/body.ttf" }` supplies only the regular
face. References resolve relative to the source through the selected asset
resolver, which may provide filesystem, memory or virtual assets. Variant values
are interpreted as text. Missing or empty variants do not synthesize a face or
fall through to `latin`; they select Helvetica metrics with an explicit
substitution reason. A non-object `files` value remains metadata and does not
select file loading. Without a files object, a declared `latin` mapping supplies
the physical family name.

Native measurement accepts static TrueType and OpenType/CFF files with a `?`
replacement glyph and embedding permissions suitable for editable native output.
Unavailable, invalid or restricted files use an explicitly reported fallback.
Requests are snapshotted for the measurer's lifetime, including failures; two
assets with one logical name must have identical bytes. The selected bytes are
retained for painting without a second file lookup. Installed-font discovery,
font embedding and script shaping remain deferred. DOCX continues to reference
the declared typeface names and ignores `files`; it does not embed these assets.

## 2. Style declarations

`style` also takes named arguments, no body, and a required string or numeric
`name`. Numeric names produce the same conversion warning. Style names retain
whitespace, including whitespace-only identities. An empty identity without a
nonempty parent fails the self-inheritance rule below. It accepts these fields:

| Attribute | Meaning |
| --- | --- |
| `type` | `paragraph`, `character`, or `table`; omission means paragraph. |
| `based_on` | String naming the parent whose formatting is inherited. |
| `next` | String naming the following paragraph style; retained relationship metadata. |
| `link` | String naming a linked style; retained relationship metadata. |
| `font` | Font reference, available as flat run formatting. |
| `paragraph` | Property object containing paragraph formatting. |
| `run` | Property object containing character formatting. |
| `table` | Property object recording table-style declarations; see the storage contract below. |

`next` and `link` do not contribute inherited properties, create content, or
change subsequent source paragraphs. Style types describe intended use; the
resolver does not enforce type restrictions on a reference or a parent chain.
Applying table styles to content and resolving conditional regions will be
defined with the table model. Declaration storage does not provide table layout.

The `paragraph` object accepts the paragraph attributes already defined:
`alignment`, `spacing`, `spacing_before`, `spacing_after`, `line_spacing`,
`indent`, `border`, `fill`, `tabs`, `keep_with_next`, `keep_together`,
`page_break_before`, `widow_control`, and `outline_level`. It also accepts
`background`, a normalized shading color retained for output-specific handling.
Extra fields, including `style` and `properties`, are retained without
normalization. Border and tab contents are also retained as supplied.

The `run` object accepts `style`, `font`, `size`, `color`, `highlight`, `bold`,
`italic`, `underline`, `small_caps`, `strike`, and `double_strike`, with the span
normalization rules for size, colors, and booleans. Nested font/style values and
extra fields remain as supplied. It also accepts `fill` and `background` as
normalized colors.
Its `style` is retained relationship metadata; only `based_on` creates an
inheritance edge. These shading fields remain distinct in the property model;
output mappings and their visual interaction require separate specification.

Flat paragraph and run properties are also accepted. Flat size, spacing lengths,
colors, bold, italic, underline, small caps, and pagination booleans normalize.
Other flat fields, including strike controls, remain as supplied; place strike
controls inside `run` to normalize them.
The additional flat `first_line_indent` and `hanging_indent` accept non-negative
lengths and remain metadata; use `paragraph.indent.first_line` and
`paragraph.indent.hanging` for the shared paragraph property model.
Extra declaration fields are retained. On both `font` and `style`, a field named
`properties` remains an ordinary property; declarations do not flatten it.
Accepted metadata does not imply a formatting effect or output support.

Within one declaration, a field in `paragraph` or `run` overrides its flat
counterpart in that group. An object-valued field replaces that flat field as a
whole at this step. Both authored forms remain stored. Colors, booleans, and
standalone unit-bearing lengths normalize; unitless values retain the shared
value rules. Flat `spacing`/`indent` objects and flat `outline_level` retain their
authored value forms without additional validation. Their counterparts inside `paragraph`
normalize under the paragraph rules.

```mote
#style(name = "Body", font = "Aptos", size = pt(11), paragraph = { spacing = { after = pt(6) } })
#style(name = "Accent", type = "character", run = { color = "#369", bold = true })
#paragraph(style = "Body")[Read #span(style = "Accent")[this].]
```

The paragraph resolves to 11-point Aptos text with six points after it. The span
inherits that font and size, then adds the Accent color and bold request. The
model still contains style references and declarations, without copying resolved
properties into the paragraph's or span's authored attributes.

### Table declaration storage

A table arranges content in rows, columns, and cells. A table-style declaration
can record properties for that whole structure and for named regions. This
increment validates and stores the declaration; table content, application of
conditional styles, and layout remain later work.

The `table` object normalizes `width`, `indent`, and `cell_spacing` as lengths;
`alignment` uses the paragraph alignment values. `layout` accepts `auto`,
`autofit`, or `fixed`. The obsolete `cell_padding` field is invalid; cell padding
intent is recorded as `cell.margin`.

`cell` is a property object for shared cell formatting. It normalizes every
`margin` entry as a length, `fill` as a color, and `no_wrap` as a boolean.
`alignment` uses paragraph values. `vertical_alignment` accepts `top`, `center`,
`middle`, or `bottom`, with `middle` normalized to `center`. `text_direction`
accepts `ltr`, `rtl`, `tb_rl`, or `bt_lr`, representing left-to-right,
right-to-left, top-to-bottom/right-to-left, or bottom-to-top/left-to-right flow.
Extra cell fields remain unchanged.

At table level, `paragraph` and `text` objects use the paragraph and run
normalizers above. `first_row`, `last_row`, `first_column`, and `last_column`
are region objects: each accepts the same normalized formatting fields as
`cell`, plus nested `cell`, `paragraph`, and `text` objects. `banding` is an
object whose `rows` and `columns` booleans request alternating regions; `odd`,
`even`, `odd_column`, and `even_column` use the region-object rules. Extra fields
remain unchanged throughout. These normalized values do not select a region or
paint a table during evaluation.

## 3. Availability and replacement

Declarations register when their calls are evaluated, including inside evaluated
content bodies. They are document-wide, not local to those bodies. A declaration
inside literal backtick content or a positional string is only text. An evaluated
code body still registers declarations even when positional code text replaces
that body's visible result.

A later declaration with exactly the same name replaces the entire earlier
declaration; it does not merge fields. Font and style names have separate
registries. Repeated argument names use the last value, while all supplied
expressions are evaluated. Declaration calls add no whitespace. In block content, one immediately
following whitespace-only text, line-break, or paragraph-break node is skipped.
Inline bodies retain surrounding whitespace under their ordinary content rules.

Resolution uses the completed document registry. References may precede their
declarations, and a replacement also affects earlier references when resolved.

```mote
#span(style = "Accent")[Ready]
#style(name = "Accent", type = "character", bold = true)
#style(name = "Accent", type = "character", italic = true)
```

Only the final Accent declaration remains, so the span resolves italic with no
bold value supplied by that style.

## 4. Inheritance and lookup

Style lookup tries the exact name first. If absent, it lowercases the name and
retains only ASCII letters and digits for a fallback comparison. Thus spaces,
hyphens, underscores, punctuation, and non-ASCII characters are omitted in that
comparison. Exact non-ASCII names still work. An empty normalized name has no
fallback match. If several declaration names match, use the first in lexical
key order. Font aliases do not use this fallback.

Follow `based_on` from the requested style towards its ancestors, then apply the
discovered declarations from base to derived. Run fields replace same-named
fields. Paragraph property objects merge recursively between declarations, so
overriding `spacing.after` preserves an inherited `spacing.before` or `spacing.line`.
An explicit false or zero replaces an inherited value; absence does not erase it.

Flat and nested spacing aliases require special handling across inheritance:
a derived flat spacing field removes the inherited nested spellings of that
field, and a derived nested spacing field removes the inherited flat spelling.
Other inherited spacing fields remain. Within a property map, the nested fields
and their primary names retain the precedence defined in Formatting attributes.

```mote
#style(name = "Base", paragraph = { spacing = { before = pt(2), after = pt(6) }, keep_with_next = true }, run = { bold = true, color = "#369" })
#style(name = "Body", based_on = "Base", paragraph = { spacing = { after = pt(0) }, keep_with_next = false }, run = { bold = false })
#paragraph(style = "Body", spacing_before = pt(4))[Text]
```

Body inherits the color, overrides bold and keep-with-next with false, and changes
after-spacing to zero. The paragraph's direct before-spacing overrides the
inherited two points with four. Resolution preserves length units and property
objects; physical conversion and pagination remain layout work.

A style whose `based_on` exactly equals its own name is invalid after evaluating
the final declaration registry. An earlier self-reference replaced by a later
valid declaration does not cause an error.

A missing ancestor stops traversal. A longer cycle, or a cycle reached through
normalized name matching, stops at the first repeated
declaration identity. Both retain the properties discovered before stopping,
and resolution reports the missing name or repeated identity. A wholly missing
style supplies no properties and reports a missing-style issue; inherited and
direct formatting still apply. These are resolution issues, not compile errors.

## 5. Direct formatting and the inspection API

The public `formatting::StyleResolver` operates on an evaluated document:

- `resolve_style(name)` returns inherited paragraph/run property maps, the
  resolved run font if present, and any resolution issues.
- `resolve_paragraph(attributes)` applies the explicit named style, then direct
  paragraph properties. It uses the same nested merge and spacing-alias rules.
- `resolve_inline(element, inherited)` starts with the supplied parent run map,
  adds the element's bold/italic intent for strong/emphasis, applies its named
  style, then its direct run attributes. An explicit style value can therefore
  override that initial intent. Pass the returned run map to nested inline
  children; resolve each sibling from its parent's map.
- `resolve_font(name)` returns an exact alias's mappings or the direct typeface
  reference described above.

Resolution returns new maps and does not modify the source model. It follows
explicit references only: implicit Normal/Heading/Code selection, document
defaults, list/container defaults, built-in font substitutions, and measured
layout remain later work. No output-format conformance is implied by resolution.

JSON inspection adds `fonts` and `styles` arrays only when declarations exist,
sorted by name. Entries record `name` and nonempty authored `properties`. Font
entries also expose nonempty script mappings. Styles expose their effective
`type`, including the paragraph default, plus nonempty `based_on`, `next`, `link`,
`font`, `paragraph`, `run`, and `table` fields. Empty optional fields are omitted. The CLI continues to show this authored model,
not a resolved or laid-out document.

## 6. Invalid uses

A font requires a name:

```mote
#font(latin = "Aptos")
```

A declaration cannot have a body:

```mote
#style(name = "Body")[Text]
```

The declared type must be in this chapter's scope:

```mote
#style(name = "Body", type = "section")
```

Run formatting uses the same boolean rules as spans:

```mote
#style(name = "Accent", run = { bold = 1 })
```

A style cannot be based directly on itself:

```mote
#style(name = "Body", based_on = "Body")
```

The [style](../checklists/style.md) and [font](../checklists/font.md) checklists
separate these language contracts from later output work.
