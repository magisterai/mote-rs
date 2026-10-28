# Spans

[Specification index](README.md) · [Formatting attributes](formatting-attributes.md)

A **span** is an inline group with optional text formatting. It introduces no
bold or italic intent by itself. `span` takes exactly one inline body, which may
be empty, and no positional arguments. It can nest inside paragraphs, headings,
other spans, strong text, emphasis, and evaluated code. Block children are invalid.
At document level it belongs to an implicit paragraph. It adds no spaces or text
of its own and follows the existing flow and preserved-whitespace rules.

## Attributes

The shared values and `properties` precedence from the preceding chapter apply.
Outer `style` and `font` arguments require strings. Values supplied through
`properties` bypass those outer checks and remain unchanged; size, colors, and
booleans still normalize after merging. Extra attributes remain in the model
without implying formatting support.

| Attribute | Meaning and value |
| --- | --- |
| `style` | Character-style reference string. |
| `font` | Typeface name or font alias string. |
| `size` | Non-negative length; unit-bearing strings normalize to typed lengths. |
| `color` | Text color; normalize a hexadecimal string or typed color. |
| `highlight` | Background color behind text; the same color rules. |
| `bold`, `italic` | Boolean requests for the corresponding text appearance. |
| `underline` | Boolean request for an underline. |
| `small_caps` | Boolean request to display lowercase letters as small capitals. |
| `strike`, `double_strike` | Boolean requests for single and double strikethrough. |
| `properties` | Attribute object whose entries override outer arguments. |

Boolean strings normalize under the shared rules. Omission is unspecified;
explicit false requests that formatting be disabled. Font size and colors
normalize on `span`; this differs from the authored-value retention on `strong`,
`emphasis`, and `code`. Each directive retains its documented value contract.

```mote
#paragraph[Amount: #span(size = "12pt", color = "#369", bold = true)[42].]
```

The paragraph contains “Amount: 42.” Its span records a 12-point size, the color
`#336699`, and explicit bold formatting. No formatting is copied into the text
node or surrounding content during evaluation.

```mote
#span[]
```

This creates an empty span inside an implicit paragraph, with no attributes.

## Nesting and precedence

Evaluation retains each span's own attributes and child structure. Resolving
formatting later starts with inherited text properties, applies a named style,
then applies direct span attributes. An inner explicit false overrides an
inherited true; an omitted value inherits. A child's overrides do not affect its
siblings. The next chapter defines named declarations and this resolution step.

```mote
#span(bold = true)[First #span(bold = false, italic = true)[second] third]
```

The inner span requests italic text with bold disabled. “First” and “third” keep
the outer bold request. The evaluated model retains two spans rather than
flattening them into separately styled text nodes.

## Invalid uses

A span requires a content body:

```mote
#span(color = "#369")
```

Its body cannot contain blocks:

```mote
#span[#paragraph[Block]]
```

Its underline control is a boolean; named underline styles belong to a separate
directive:

```mote
#span(underline = "wave")[Text]
```

Output support is tracked separately in the [span checklist](../checklists/span.md).
