# Version 3.0

Source: [PostScript information](https://learn.microsoft.com/en-us/typography/opentype/spec/post) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/post).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-overview.md)

<a id="version-30"></a>

## Version 3.0

This version makes it possible to create a font that is not burdened with a large set of glyph names. A version 3.0 'post' table can be used by OpenType fonts with TrueType or CFF (version 1 or 2) data.

This version specifies that no PostScript name information is provided for the glyphs in this font file. The printing behavior of this version on PostScript printers is unspecified, except that it should not result in a fatal or unrecoverable error. Some drivers may print nothing; other drivers may attempt to print using a default naming scheme.

> *Note:* Windows makes use of the italic angle value in the 'post' table but does not actually require any glyph names to be stored as Pascal strings.

<a id="post-table-and-opentype-font-variations"></a>

## 'post' Table and OpenType Font Variations

In a variable font, various font-metric values within the 'post' table may need to be adjusted for different variation instances. Variation data for 'post' entries can be provided in the [metrics variations (MVAR)](https://learn.microsoft.com/en-us/typography/opentype/spec/mvar) table. Different 'post' entries are associated with particular variation data in the MVAR table using value tags, as follows:

| 'post' entry | Tag |
| --- | --- |
| underlinePosition | 'undo' |
| underlineThickness | 'unds' |

> *Note:*
> The italicAngle value is not adjusted by variation data since this corresponds to the 'slnt' variation axis that can be used to define a font’s variation space. Appropriate post.italicAngle values for a variation instance can be derived from the 'slnt' user coordinates that are used to select a particular variation instance. See the discussion of the 'slnt' axis in the [OpenType Design-Variation Axis Tag Registry](https://learn.microsoft.com/en-us/typography/opentype/spec/dvaraxisreg) for details on the relationship between italicAngle and the 'slnt' axis.

For general information on OpenType Font Variations, see the chapter, [OpenType Font Variations Overview](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview).
