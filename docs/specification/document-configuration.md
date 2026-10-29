# Document configuration

[Specification index](README.md) · [Checklist](../checklists/document.md)

## A declaration without content

`document` sets document-wide properties. It accepts named arguments and no body,
returns no visible content, and does not split the content into sections. Put it
near the start so the page setup is easy to find. It also takes effect when called
later or inside an evaluated body, as with font and style declarations.

`page` and `margin` are property objects. Omitted values remain omitted in the
model. Repeated declarations merge their fields: a later field replaces the same
field, while other earlier fields survive. This does not change earlier text.

## Page size and orientation

`page.size` names a paper size. The supported names are `letter` (also
`us_letter`), `legal` (`us_legal`), `tabloid` (`ledger`), `a3`, `a4`, and `a5`.
Size names are case-insensitive when selecting paper. The original value remains
in the model. An omitted or unrecognized size uses Letter dimensions unless
explicit dimensions are supplied.

`page.width` and `page.height` accept the non-negative length forms already
defined in [Formatting attributes](formatting-attributes.md). Either overrides
the corresponding dimension of the named size. `page.orientation` is exactly
`"portrait"` or `"landscape"`. Landscape swaps the dimensions when height exceeds
width; portrait does not swap explicit dimensions.

```mote
#document(page = { size = "a4", orientation = "landscape" })

#paragraph[A landscape page.]
```

## Margins

`margin.top`, `right`, `bottom`, and `left` give the body margins.
`margin.header` and `margin.footer` give distances from the page edges to the
header and footer areas; they do not create header or footer content.
`margin.gutter` reserves a binding margin. Each accepts a non-negative length,
including zero. For page and margin geometry, unitless values mean twips.
Use length helpers when a physical unit is intended.

```mote
#document(page = { width = inch(6), height = inch(9) }, margin = { top = inch(1), bottom = inch(1), left = cm(2), right = cm(2) })
#document(margin = { left = cm(3), gutter = pt(6) })

#paragraph[The later declaration changes the left margin and adds a gutter.]
```

The model retains authored arguments under `document`. A repeated argument
replaces its previous authored value there. The normalized `page` and `margin`
objects contain merged geometry; length strings normalize there, while the raw
argument remains unchanged. The implicit section uses that merged geometry.

Extra page fields and document arguments remain metadata, without an implied
formatting effect. Extra margin fields must still be valid lengths. A declaration's
`properties` argument is retained as an ordinary argument, not flattened.
Arguments named `default_*` or `language` are additionally retained in `defaults`;
their effective formatting is not yet supported. DOCX rejects unsupported
properties instead of silently dropping them.

## Invalid uses

A body is invalid because `document` configures the document rather than creating
content:

```mote
#document()[Text]
```

Flat geometry aliases are invalid; use the nested property objects:

```mote
#document(page_size = "a4")
```

Unknown orientation values are invalid:

```mote
#document(page = { orientation = "sideways" })
```

Negative margins are invalid:

```mote
#document(margin = { top = -1 })
```

Columns, explicit sections, document defaults, and header/footer content will be
defined separately. These declarations establish geometry, not pagination or a
claim of visual conformance.
