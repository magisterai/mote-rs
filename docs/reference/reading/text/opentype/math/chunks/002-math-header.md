# MATH Header

Source: [Mathematical typesetting table](https://learn.microsoft.com/en-us/typography/opentype/spec/math) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/math).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-overview.md) · [Next](003-continuation-3.md)

<a id="header"></a>

<a id="math-header"></a>

### MATH Header

The MATH table begins with a header that consists of a version number for the table (majorVersion/minorVersion), which is currently set to 1.0, and specified offsets to the following tables that store information on positioning of math formula elements:

- MathConstants table stores font parameters to be used in typesetting of each supported mathematical function, such as a fraction or a radical.
- MathGlyphInfo table supplies positioning information that is defined on a per-glyph basis.
- MathVariants table contains information to be used in constructing glyph variants of different height or width, either by finding a pre-defined glyph with desired measurements in the font, or by assembling the required shape from pieces found in the glyph set.

<a id="mathTable"></a>

*MATH Header*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the MATH table, = 1. |
| uint16 | minorVersion | Minor version of the MATH table, = 0. |
| Offset16 | mathConstantsOffset | Offset to MathConstants table, from the beginning of MATH table. |
| Offset16 | mathGlyphInfoOffset | Offset to MathGlyphInfo table, from the beginning of MATH table. |
| Offset16 | mathVariantsOffset | Offset to MathVariants table, from the beginning of MATH table. |

<a id="mathConstants"></a>

<a id="mathconstants-table"></a>

### MathConstants table

The MathConstants table defines a number of constants required to properly position elements of mathematical formulas. These constants belong to several groups of semantically-related values, such as values for positioning of accents, positioning of superscripts and subscripts, and positioning of elements of fractions. The table also contains general-use constants that may affect all parts of the formula, such as axis height and math leading. Note that most of the constants deal with aspects of vertical positioning.

In most cases, values in the MathConstants table are assumed to be positive. For example, for descenders and shift-down values a positive constant signifies movement in a downwards direction. Most values in the MathConstants table are represented by a MathValueRecord, which allows the font designer to supply device corrections to those values when necessary.

For values that pertain to layout interactions between a base and dependent elements (e.g., superscripts or limits), the specific value used is taken from the font associated with the base, and the size of the value is relative to the size of the base.

The following naming convention are used for fields in the MathConstants table:

- **Height** — Specifies a distance from the main baseline.
- **Kern** — Represents a fixed amount of empty space to be introduced.
- **Gap** — Represents an amount of empty space that may need to be increased to meet certain criteria.
- **Drop and Rise** — Specifies the relationship between measurements of two elements to be positioned relative to each other (but not necessarily in a stack-like manner) that must meet certain criteria. For a Drop, one of the positioned elements has to be moved down to satisfy those criteria; for a Rise, the movement is upwards.
- **Shift** — Defines a vertical shift applied to an element sitting on a baseline. Note that the value is an amount of *adjustment* to the position of an element, not the resulting distance from the baseline or other reference line.
- **Dist** — Defines a distance between baselines of two elements.

The descriptions for several fields refer to *default rule thickness*. Layout engines control how rules are drawn and how their thickness is set. It is recommended that rules have the same thickness as a minus sign, low line, or a similar font value such as [OS/2.yStrikeoutSize](../../os2/chunks/006-ystrikeoutsize.md#ystrikeoutsize). For fields that are described in reference to default rule thickness, one of these should be assumed.

<a id="mathConstantsTable"></a>

*MathConstants table*

