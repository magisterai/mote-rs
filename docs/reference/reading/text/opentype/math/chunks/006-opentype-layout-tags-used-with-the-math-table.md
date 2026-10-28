# OpenType Layout tags used with the MATH table

Source: [Mathematical typesetting table](https://learn.microsoft.com/en-us/typography/opentype/spec/math) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/math).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-math-variants.md)

<a id="mathTags"></a>

<a id="opentype-layout-tags-used-with-the-math-table"></a>

## OpenType Layout tags used with the MATH table

The following OpenType Layout tags can be used by a math layout engine to access a particular set of glyph variants. For detailed descriptions of the feature tags, see the [Feature Tags](https://learn.microsoft.com/en-us/typography/opentype/spec/featuretags) section of the OpenType Layout Tag Registry.

<a id="mathTagsTbl"></a>

*OpenType Layout tags for math processing*

| Tag | Description |
| --- | --- |
| 'math' | Script tag to be used for features in math layout. The only language system supported with this tag is the default language system. |
| ['ssty'](https://learn.microsoft.com/en-us/typography/opentype/spec/features_pt#ssty) | Math Script-style Alternates    This feature provides glyph variants adjusted to be more suitable for use in subscripts and superscripts.    These script style forms should not be scaled or moved in the font; scaling and moving them is done by the math layout engine. Instead, the 'ssty' feature should provide glyph forms that result in shapes that look good as superscripts and subscripts when scaled and positioned by the math engine. When designing the script forms, the font developer may assume that the scriptPercentScaleDown and scriptScriptPercentScaleDown values in the MathConstants table will be scaling factors applied to the size of the alternate glyphs by the math engine.    This feature can have a parameter indicating the script level: 1 for simple subscripts and superscripts, 2 for second level subscripts and superscripts (that is, scripts on scripts), and so on. (Currently, only the first two alternates are used).    For glyphs that are not covered by this feature, the original glyph is used in subscripts and superscripts.    Recommended format: Alternate Substitution table (Single Substitution if there are no second level forms). There should be no context. |
| ['flac'](https://learn.microsoft.com/en-us/typography/opentype/spec/features_fj#flac) | Flattened Accent Forms    This feature provides flattened forms of accents to be used over high-rise bases such as capitals.    This feature should only change the shape of the accent and should not move it in the vertical or horizontal direction. Moving of the accents is done by the math layout engine.    Accents are flattened by the math engine if their base is higher than the flattenedAccentBaseHeight value in the MathConstants table.    Recommended format: Single Substitution table. There should be no context. |
| ['dtls'](https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#dtls) | Dotless Forms    This feature provides dotless forms for Math Alphanumeric characters, such as U+1D422 MATHEMATICAL BOLD SMALL I, U+1D423 MATHEMATICAL BOLD SMALL J, U+1D456 U+MATHEMATICAL ITALIC SMALL I, U+1D457 MATHEMATICAL ITALIC SMALL J, and so on.    The dotless forms are to be used as base forms for placing mathematical accents over them.    Recommended format: Single Substitution table. There should be no context. |
