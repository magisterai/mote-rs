# Continuation 3

Source: [Glyph data](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-composite-glyph-description.md)

The SCALED\_COMPONENT\_OFFSET and UNSCALED\_COMPONENT\_OFFSET flags are used to determine how x and y offset values are to be interpreted when the component glyph is scaled. If the SCALED\_COMPONENT\_OFFSET flag is set, then the x and y offset values are deemed to be in the component glyph’s coordinate system, and the scale transformation is applied to both values. If the UNSCALED\_COMPONENT\_OFFSET flag is set, then the x and y offset values are deemed to be in the current glyph’s coordinate system, and the scale transformation is not applied to either value. If neither flag is set, then the rasterizer may apply a default behavior. On Microsoft and Apple platforms, the default behavior is the same as when the UNSCALED\_COMPONENT\_OFFSET flag is set; this behavior is recommended for all rasterizer implementations. If a font has both flags set, this is invalid; the rasterizer should use its default behavior for this case.
