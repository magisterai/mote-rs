# Formatting attributes

[Specification index](README.md) · [Block quotes](block-quotes.md)

This chapter extends the ten content directives already defined. It specifies
attribute values, validation, and the evaluated model. Layout, effective style
resolution, and Office/PDF/PNG output are separate stages. The
[directive checklists](../checklists/README.md) track those stages separately.

## 1. Shared values

A **style reference** is a string naming a paragraph or character style. A
**font reference** is a string naming a typeface or font alias. Evaluation records
these references without looking up declarations, substituting fonts, or copying
effective properties into children. Omission remains distinct from an explicit
value. Style declarations and inheritance will be defined separately.

A **color** uses `#` followed by three or six hexadecimal digits. The
`color` helper accepts exactly one positional string, without a body or named
arguments. It trims surrounding whitespace, expands three digits to six, and
lowercases the result. `color("#AbC")` is the typed color `#aabbcc`; inserting it
in content produces that text. JSON inspection writes a color as a string.
Named colors and alpha channels are outside this value form.

Boolean attributes accept booleans or the strings `true`, `True`, `TRUE`, `t`,
`T`, `1`, `false`, `False`, `FALSE`, `f`, `F`, and `0`. They normalize to booleans.
Numeric zero/one, whitespace-padded strings, and other spellings are invalid.
Positive and non-negative integer attributes accept integer numbers or integer
strings, normalizing to numbers within the supported signed 64-bit range.

Length-valued attributes accept a typed length, a quoted magnitude plus `pt`,
`in`, `cm`, `mm`, or `twip`, or a unitless number/numeric string. The magnitude
must be finite and non-negative. Unit-bearing strings normalize to typed lengths
where specified below; unitless numbers and strings retain their types. They do
not acquire a unit in the model. Use explicit units when expressing a physical
distance; output-specific interpretation of unitless values is separate.

An **array** is an ordered collection written with unnamed comma-separated
expressions inside braces. A trailing comma is allowed. For example, the value
form `{ pt(12), pt(24) }` contains two lengths. A property object instead has named
entries, as previously defined. Mixing named and unnamed entries is invalid.
Empty braces `{}` mean an empty array, not an empty property object. Arrays and
property objects cannot be inserted directly as text. This chapter uses arrays
for tab stops; it does not introduce scripting or data-loading syntax.

## 2. Attribute objects and precedence

Every directive in this chapter except `line_break` accepts `properties`, a
nonempty property object. Its fields become ordinary attributes of the receiving
element, and `properties` itself is not retained. A field in this object overrides
a same-named outer argument regardless of argument order. Validate the resulting
attributes after this merge. Repeated names within an object or argument list use
the last supplied value; all expressions are still evaluated.

An unknown attribute or nested property is an error, including when supplied
through `properties`. This object is not an arbitrary output-format escape hatch.
A positional heading level takes precedence over a `properties.level` field
before value validation;
direct positional and named `level` arguments still cannot be combined.

The following paragraph records style `Body` and right alignment:

```mote
#paragraph(style = "Body", alignment = "left", properties = { alignment = "right" })[Text]
```

This example names a style reference; it does not declare or resolve a style.

## 3. Inline formatting references

`strong`, `emphasis`, and `code` accept these optional attributes:

| Attribute | Meaning |
| --- | --- |
| `style` | Character-style reference string. |
| `font` | Font reference string. |
| `size` | Non-negative text size, using a length value or unitless number/string. |
| `color` | Hexadecimal string or typed color. |

Values are validated and retained in their authored form. In particular, a
quoted `size` remains a string and a quoted color retains its spelling; the
typed `color` helper performs normalization explicitly. Formatting attributes
remain on the authored element. Evaluation does not copy them into nested nodes.
The strong and emphasis elements retain their respective bold and italic intent.

```mote
#paragraph[#strong(font = "Aptos", size = pt(12), color = color("#AbC"))[Bold] #emphasis(style = "Accent")[Italic]]
```

Direct inline overrides have native and presentation output scope. They do not
establish a direct DOCX attribute mapping on `strong` or `emphasis`, or direct
`font`/`size`/`color` mapping on `code`. Those DOCX cases use inherited formatting
and styles; their output work remains tracked separately.

## 4. Paragraph and heading geometry

Paragraphs and headings accept a `style` reference, the alignment values already
defined, and the following spacing and indentation controls.

`spacing` accepts `before`, `after`, and `line`; `spacing_before`, `spacing_after`,
and `line_spacing` are also accepted as field aliases within it. The flat
attributes `spacing_before`, `spacing_after`, and `line_spacing` accept lengths.
Both flat and nested values remain in the model; evaluation does not collapse
them into one value. Within a later effective property lookup, nested spacing
takes precedence over its flat counterpart, with `before`, `after`, and `line`
preferred over the corresponding aliases inside `spacing`.

`spacing.line` can be a length or an object containing exactly `rule` and `value`:

| Rule | Value | Requested behaviour |
| --- | --- | --- |
| `multiple` | Positive finite number or numeric string | A multiple of normal line height. |
| `exact` | Non-negative length | A fixed line height. |
| `at_least` | Non-negative length | A minimum line height. |

A plain line-spacing length requests a fixed line height. A multiple normalizes
to a number; exact/minimum values follow the length rules. Missing fields,
unknown rules, and a length used as a multiple are errors.

`indent` accepts non-negative `left`, `right`, `first_line`, and `hanging`
lengths. `first_line` requests an extra first-line inset; `hanging` requests the
opposite offset relative to following lines. Evaluation preserves both if both
are authored; resolution belongs to layout. Paragraph and heading spacing and
indentation fields normalize unit-bearing strings to typed lengths.

Both directives record four optional pagination booleans:

| Attribute | Requested behaviour |
| --- | --- |
| `keep_with_next` | Keep this block with the next block. |
| `keep_together` | Keep the block's lines together. |
| `page_break_before` | Begin the block on a new page. |
| `widow_control` | Avoid isolated first or last paragraph lines across pages. |

These requests do not calculate breaks during evaluation. Paragraphs additionally
accept `outline_level`, a non-negative integer, and `fill`, a shading color
normalized to a typed color. Heading hierarchy uses its existing `level` attribute.

```mote
#paragraph(
  spacing = { before = pt(2), after = pt(6), line = { rule = "exact", value = "14pt" } },
  indent = { left = pt(12), first_line = pt(6) }, fill = "#eef",
  keep_with_next = "true", keep_together = false, page_break_before = false,
  widow_control = true, outline_level = 0
)[Text]
```

## 5. Paragraph borders and tab stops

A paragraph `border` object accepts `top`, `right`, `bottom`, `left`, and `all`.
Each side is a border-style string or a property object with these fields:

| Field | Meaning |
| --- | --- |
| `style`, `val` | Border-style spelling; `style` takes precedence over its alias `val`. |
| `size` | Non-negative integer width in eighths of a point. |
| `width` | Non-negative length; takes precedence over `size`; a bare number denotes points for this field. |
| `space` | Non-negative integer border separation in points. |
| `color` | Hexadecimal string or typed color. |

`all` supplies sides that are not authored individually. `single` requests a
single stroke; `none` or `nil` as strings requests no visible border. Evaluation
validates the object shape and value types but retains the authored side objects,
aliases, and values. Output support for individual border-style spellings must be
verified separately; a stored string is not a conformance claim.

`tabs` accepts one tab-stop object, an array of such objects, or `{}` for an empty
array. A tab stop requires a position length: `position` or its alias `pos`.
`alignment` or `align` is an optional string; `leader` is an optional string for
the repeated fill character. The usual alignment values are `left`, `center`,
`right`, and `decimal`; `dot` requests a dotted leader. A single-entry object
such as `{ right = pt(72) }` is alignment/position shorthand. Stops retain their
order and authored fields; no sorting, merging, or tab insertion occurs.

```mote
#paragraph(
  border = { bottom = { style = "single", width = pt(1), color = "#123" } },
  tabs = { { position = inch(1), alignment = "right", leader = "dot" } }
)[Text]
```

The model has one bottom border and an array containing one tab stop. A tab stop
is a position setting; this example does not insert a tab character.

## 6. Heading numbering

A heading's `numbering` object accepts `enabled` and `restart` booleans, an `id`
for sequence identity, a `separator` string, a positive integer `start`, and
`format`. Identifiers and separators also accept numbers, converted to strings.
The formats are `decimal`, `lower_alpha`, `upper_alpha`, `lower_roman`, and
`upper_roman`. There are no figure-label or prefix fields on heading numbering.

`enabled` requests numbering, `start` supplies an initial counter, `restart`
requests a fresh sequence, and `id` identifies shared sequence intent. The
separator joins hierarchical levels. Omitted fields stay absent. Evaluation
neither advances counters nor inserts number text into heading children.

```mote
#heading(2, numbering = { enabled = true, id = "chapters", format = "decimal", separator = ".", start = 1, restart = false })[Scope]
```

## 7. List and item attributes

Both list containers accept `id`, `style`, `start`, `marker`, `format`, `restart`,
`tight`, `level`, `indent`, and `spacing`, plus `properties`. `id` is a string or a
number converted to a string; `style` and `marker` are strings. `start` and `level`
are positive integers. `restart` and `tight` follow the boolean rules.

`format` accepts the five numbering formats above or `bullet`. The container's
kind remains authoritative: storing numeric controls on `list` does not turn it
into `ordered_list`. Unordered numeric controls and an ordered list's custom
`marker` are retained intent, not new output capabilities. A custom unordered
marker and list-level paragraph style have presentation-specific output scope.
Sequence continuation also has output-specific scope.

List `indent` accepts non-negative `left`, `hanging`, and `level` lengths. Here
`indent.level` is additional distance per nesting level; the outer `level` is an
integer nesting level. List spacing accepts the shared spacing fields. Paged
list layout uses `after`; additional fields have presentation scope. `tight`
requests zero after-spacing; an explicit `spacing.after` overrides that request.

An `item` accepts `style`, `marker`, `level`, `tight`, `indent`, `spacing`,
`checked`, `checkbox`, and `properties`. Item geometry overrides list defaults
when layout resolves it; evaluation retains values on each element. A task item
has an authored boolean `checked`: true means checked, false means unchecked,
and omission means no explicit task state.

`checkbox` carries a boolean `checked`, a `kind` of `content_control` or
`legacy_form_field`, and `name`/`title` strings or numbers converted to strings.
The item's `checked` controls its marker; a conflicting nested value remains
metadata and does not win. Nested state alone does not create a task item.
`title` is content-control metadata, without a legacy-form-field mapping.
Native output paints task marks; Word control metadata describes editable Word
objects. No output is produced during evaluation.

```mote
#ordered_list(id = "steps", start = 3, format = "upper_roman", restart = false, tight = true, indent = { left = pt(36), hanging = pt(18), level = pt(18) })[
  #item(checked = false, checkbox = { kind = "content_control", name = "review", title = "Review" }, spacing = { after = pt(2) })[Review]
]
```

This defines explicit task-item attributes only. Markdown task markers remain
outside the implemented Markdown subset.

## 8. Code metadata and whitespace

In addition to the inline attributes above, `code` accepts `language`, a string
label, and `preserve_space`, a boolean. A language label does not enable syntax
highlighting. The argument/body and literal-code rules from the inline chapter
continue to apply.

With `preserve_space = true`, body line endings become line feeds rather than
flowing spaces. Leading/trailing spaces, blank-line indentation, and tabs remain
text, except the syntactic indentation before directives and block markers
defined in the syntax chapter. CRLF and CR become LF, including in positional
strings and nested content.
Preservation propagates through nested inline bodies; an inner false value does
not cancel its enclosing preserved context. Directives and emphasis still
evaluate normally. Without a preserved context, ordinary flow rules apply.

```mote
#code(language = "text", preserve_space = true)[first
  #strong[second
 third]]
```

The code begins with `first` and a line feed; the two spaces before `#strong`
are syntactic indentation. Its strong child contains `second`, a line feed, and
one space before `third`. Tabs remain tabs in
the model; fixed four-space expansion is an output concern. Exact whitespace
preservation and any degraded presentation output need separate verification.

## 9. Quoted-block attributes

`blockquote` accepts `style`, `border`, `fill`, `alignment`, `spacing`, `indent`,
the flat spacing aliases, and `properties`. The shapes and value constraints are
the paragraph forms above. `fill` normalizes to a typed color. Other geometry
values remain in their authored form, including quoted lengths and line-spacing
values. The model does not distribute them into the quote's child paragraphs.

```mote
#blockquote(style = "Quote", fill = "#eef", indent = { left = pt(12) })[Quoted]
```

DOCX quote attributes in this scope are `style`, `border`, and `fill`. The other
controls have native/presentation scope; that does not add DOCX mappings or
presentation border/fill support.

## 10. Invalid uses

Each example below parses but fails evaluation for the stated rule.

An array is not a `properties` object:

```mote
#paragraph(properties = {})[Text]
```

A line-height multiple must be positive:

```mote
#paragraph(spacing = { line = { rule = "multiple", value = 0 } })[Text]
```

A tab-stop position cannot be negative:

```mote
#paragraph(tabs = { { position = pt(-1) } })[Text]
```

A color must be hexadecimal:

```mote
#strong(color = "red")[Text]
```

Unsupported attributes remain errors rather than successful formatting:

```mote
#strong(kerning = pt(1))[Text]
```
