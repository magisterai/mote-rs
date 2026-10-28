# Fonts and text layout

[Reference guide](../README.md)

Separate Unicode text boundaries and direction from font lookup, shaping, metrics, and painting. Read only the relevant algorithm or font table.

| Task | Open next |
| --- | --- |
| Font container and table directory | [OpenType font file](../reading/text/opentype/otff/README.md) |
| Unicode-to-glyph mapping | [cmap](../reading/text/opentype/cmap/README.md) |
| Horizontal metrics | [hmtx](../reading/text/opentype/hmtx/README.md) |
| Windows metrics and embedding flags | [OS/2](../reading/text/opentype/os2/README.md) |
| TrueType outlines | [glyf](../reading/text/opentype/glyf/README.md) |
| Compact Font Format | [CFF](../reading/text/fonts/adobe-5176-cff/README.md) |
| CFF outline instructions | [Type 2 Charstrings](../reading/text/fonts/adobe-5177-type2/README.md) |
| Glyph substitution | [GSUB](../reading/text/opentype/gsub/README.md) |
| Glyph positioning | [GPOS](../reading/text/opentype/gpos/README.md) |
| Line breaking | [UAX #14](../reading/text/unicode/uax-14/README.md) |
| Grapheme and word boundaries | [UAX #29](../reading/text/unicode/uax-29/README.md) |
| Bidirectional text | [UAX #9](../reading/text/unicode/uax-09/README.md) |
| Normalization | [UAX #15](../reading/text/unicode/uax-15/README.md) |
| East Asian width | [UAX #11](../reading/text/unicode/uax-11/README.md) |

The Unicode annexes use a single pinned release, 17.0.0. The [catalogue](../catalogue.md) contains the remaining font tables, including vertical metrics. Inspect the retained table markup or local page image when merged cells or numerical layout are important.
