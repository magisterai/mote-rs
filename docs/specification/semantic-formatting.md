# Semantic text formatting

[Specification index](README.md) · [Spans](spans.md)

These directives add inline formatting to the content and value rules already
defined for spans. Each takes exactly one inline body, which may be empty, and
no positional arguments. They do not introduce spaces or paragraph boundaries.
Nested directives inherit surrounding formatting; changes remain local to their
own children.

## Directives and attributes

| Directive | Formatting request | Additional attributes |
| --- | --- | --- |
| `underline` | Underline the body. | `style`: `single`, `double`, `dotted`, `dashed`, `wave`, or `none`; omission requests a single line. |
| `strike` | Strike through the body. | `kind`: `single` or `double`; omission requests a single line. |
| `highlight` | Color the background behind the body. | `fill`: hexadecimal or typed color. Omission supplies no new fill. |
| `small_caps` | Display lowercase characters as small capitals. | None. |
| `subscript` | Lower the body's baseline. | None. |
| `superscript` | Raise the body's baseline. | None. |

All six accept `style`, `font`, `size`, `color`, and `properties`. For `underline`,
`style` selects the underline pattern, rather than naming a character style.
For the other directives it is a character-style reference. Outer `style` and
`font` require strings; `size` accepts a non-negative length; `color` and `fill`
normalize hexadecimal strings and typed colors. `strike.kind` requires a string.
The shared `properties` precedence and extra-property retention rules apply.

Underline `color` requests both text and underline color where the output supports
that distinction. Script text retains its semantic baseline request in the model;
its effective size depends on document or presentation layout. Direct size values
and nested script requests follow the receiving output's supported rules.

The [style-resolution API](styles-and-fonts.md) retains these requests without
choosing output sizes or baseline offsets. Its run map carries `underline_style`
and `underline_color` separately from the underline flag, and boolean
`small_caps`, `subscript`, `superscript`, `strike` and `double_strike` requests.
Underline patterns do not trigger character-style lookup in that API.

## Output boundaries

Word output supports underline patterns, color, font and size. The remaining
semantic wrappers emit their requested effect but reject direct style, font,
size and color attributes that have no Word mapping. Use surrounding spans for
that formatting. Highlight `fill` and strike `kind` do have Word mappings.

Native text uses reduced script sizes and baseline offsets; presentation text
uses its own script profile while editable slides retain the authored run size.
Native underline `wave` and `none` currently retain a solid underline; those
requests require a diagnostic. Editable Office outputs retain their requested
patterns. Model acceptance alone does not demonstrate matching Office rendering.

## Examples

```mote
#paragraph[
  #underline(style="double")[Underlined], #strike(kind="double")[removed],
  #highlight(fill="#fff2cc")[highlighted], and #small_caps[Small capitals].
  Water is H#subscript[2]O; a square is x#superscript[2].
]
```
