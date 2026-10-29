# Word inline text properties

[Inventory guide](README.md)

Used by spans, strong/emphasis/code content, paragraph marks, and explicit line breaks. Inherited properties and revision metadata are included.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_RPr](#w-ct_rpr)
- [w:CT_ParaRPr](#w-ct_pararpr)
- [w:CT_R](#w-ct_r)
- [w:CT_Text](#w-ct_text)
- [w:CT_Br](#w-ct_br)
- [w:CT_Color](#w-ct_color)
- [w:CT_SignedTwipsMeasure](#w-ct_signedtwipsmeasure)
- [w:CT_TextScale](#w-ct_textscale)
- [w:CT_HpsMeasure](#w-ct_hpsmeasure)
- [w:CT_SignedHpsMeasure](#w-ct_signedhpsmeasure)
- [w:CT_Highlight](#w-ct_highlight)
- [w:CT_Underline](#w-ct_underline)
- [w:CT_TextEffect](#w-ct_texteffect)
- [w:CT_Border](#w-ct_border)
- [w:CT_FitText](#w-ct_fittext)
- [w:CT_VerticalAlignRun](#w-ct_verticalalignrun)
- [w:CT_Em](#w-ct_em)
- [w:CT_Language](#w-ct_language)
- [w:CT_EastAsianLayout](#w-ct_eastasianlayout)
- [w:CT_RPrChange](#w-ct_rprchange)
- [w:CT_TrackChange](#w-ct_trackchange)
- [w:CT_ParaRPrChange](#w-ct_pararprchange)
- [w:CT_RPrOriginal](#w-ct_rproriginal)
- [w:CT_ParaRPrOriginal](#w-ct_pararproriginal)

<a id="w-ct_rpr"></a>

## w:CT_RPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `rStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional. Referenced Character Style; [§17.3.2.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-316-part-1). Model support: `style` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `rFonts` — [w:CT_Fonts](fonts.md#w-ct_fonts); Strict, Transitional. Run Fonts; [§17.3.2.26](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-301-part-1). Partial: Font names/aliases and script mappings exist; theme font references and selection hints are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `b` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Bold; [§17.3.2.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Model support: `bold` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `bCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Bold; [§17.3.2.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Partial: `bold` is shared text intent; an independent complex-script bold switch is missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `i` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Italics; [§17.3.2.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Model support: `italic` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `iCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Italics; [§17.3.2.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Partial: `italic` is shared text intent; an independent complex-script italic switch is missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `caps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display All Characters As Capital Letters; [§17.3.2.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [x] `smallCaps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Small Caps; [§17.3.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-320-part-1). Model support: `small_caps` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `strike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Single Strikethrough; [§17.3.2.37](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Model support: `strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `dstrike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Double Strikethrough; [§17.3.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1). Model support: `double_strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `outline` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display Character Outline; [§17.3.2.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-298-part-1).
- [ ] `shadow` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Shadow; [§17.3.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-319-part-1).
- [ ] `emboss` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Embossing; [§17.3.2.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `imprint` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Imprinting; [§17.3.2.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `noProof` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Do Not Check Spelling or Grammar; [§17.3.2.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).
- [ ] `snapToGrid` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional.
- [ ] `vanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Hidden Text; [§17.3.2.41](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-329-part-1).
- [ ] `webHidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Web Hidden Text; [§17.3.2.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-332-part-1).
- [ ] `color` — [w:CT_Color](word-runs.md#w-ct_color); Strict, Transitional. Run Content Color; [§17.3.2.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-277-part-1). Partial: Hexadecimal `color` exists; automatic color, theme selection, and tint/shade are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `spacing` — [w:CT_SignedTwipsMeasure](word-runs.md#w-ct_signedtwipsmeasure); Strict, Transitional.
- [ ] `w` — [w:CT_TextScale](word-runs.md#w-ct_textscale); Strict, Transitional. Expanded/Compressed Text; [§17.3.2.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-331-part-1).
- [ ] `kern` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Font Kerning; [§17.3.2.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `position` — [w:CT_SignedHpsMeasure](word-runs.md#w-ct_signedhpsmeasure); Strict, Transitional. Vertically Raised or Lowered Text; [§17.3.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-299-part-1).
- [x] `sz` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Non-Complex Script Font Size; [§17.3.2.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Model support: `size` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `szCs` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Complex Script Font Size; [§17.3.2.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-324-part-1). Partial: `size` is shared text intent; an independent complex-script size is missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `highlight` — [w:CT_Highlight](word-runs.md#w-ct_highlight); Strict, Transitional. Text Highlighting; [§17.3.2.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-291-part-1). Partial: Mote stores hexadecimal `highlight`; Word palette selection and mapping are not implemented. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `u` — [w:CT_Underline](word-runs.md#w-ct_underline); Strict, Transitional. Underline; [§17.3.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-325-part-1). Partial: `underline` is boolean; patterned/double/word-only underline and separate colors are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `effect` — [w:CT_TextEffect](word-runs.md#w-ct_texteffect); Strict, Transitional. Animated Text Effect; [§17.3.2.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-287-part-1).
- [ ] `bdr` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional. Text Border; [§17.3.2.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional. Partial: Run-style fill/background metadata exists; full Word shading patterns and theme semantics are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `fitText` — [w:CT_FitText](word-runs.md#w-ct_fittext); Strict, Transitional. Manual Run Width; [§17.3.2.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `vertAlign` — [w:CT_VerticalAlignRun](word-runs.md#w-ct_verticalalignrun); Strict, Transitional. Subscript/Superscript Text; [§17.3.2.42](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-330-part-1).
- [ ] `rtl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Right To Left Text; [§17.3.2.30](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-317-part-1).
- [ ] `cs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Use Complex Script Formatting on Run; [§17.3.2.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-280-part-1).
- [ ] `em` — [w:CT_Em](word-runs.md#w-ct_em); Strict, Transitional. Emphasis Mark; [§17.3.2.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-288-part-1).
- [ ] `lang` — [w:CT_Language](word-runs.md#w-ct_language); Strict, Transitional. Languages for Run Content; [§17.3.2.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-295-part-1). Document-level language default only; direct and style-level language properties remain unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [docx_defaults.rs](../../../tests/docx_defaults.rs).
- [ ] `eastAsianLayout` — [w:CT_EastAsianLayout](word-runs.md#w-ct_eastasianlayout); Strict, Transitional. East Asian Typography Settings; [§17.3.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1).
- [ ] `specVanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Paragraph Mark Is Always Hidden; [§17.3.2.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-322-part-1).
- [ ] `oMath` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Office Open XML Math; [§17.3.2.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).
- [ ] `rPrChange` — [w:CT_RPrChange](word-runs.md#w-ct_rprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_pararpr"></a>

## w:CT_ParaRPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `ins` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `del` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `moveFrom` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `moveTo` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `rStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional. Referenced Character Style; [§17.3.2.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-316-part-1). Partial: `style` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `rFonts` — [w:CT_Fonts](fonts.md#w-ct_fonts); Strict, Transitional. Run Fonts; [§17.3.2.26](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-301-part-1). Partial: Font names/aliases and script mappings exist; theme font references and selection hints are missing. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `b` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Bold; [§17.3.2.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Partial: `bold` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `bCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Bold; [§17.3.2.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Partial: `bold` is shared text intent; an independent complex-script bold switch is missing. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `i` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Italics; [§17.3.2.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Partial: `italic` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `iCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Italics; [§17.3.2.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Partial: `italic` is shared text intent; an independent complex-script italic switch is missing. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `caps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display All Characters As Capital Letters; [§17.3.2.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [ ] `smallCaps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Small Caps; [§17.3.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-320-part-1). Partial: `small_caps` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `strike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Single Strikethrough; [§17.3.2.37](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Partial: `strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `dstrike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Double Strikethrough; [§17.3.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1). Partial: `double_strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `outline` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display Character Outline; [§17.3.2.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-298-part-1).
- [ ] `shadow` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Shadow; [§17.3.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-319-part-1).
- [ ] `emboss` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Embossing; [§17.3.2.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `imprint` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Imprinting; [§17.3.2.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `noProof` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Do Not Check Spelling or Grammar; [§17.3.2.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).
- [ ] `snapToGrid` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional.
- [ ] `vanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Hidden Text; [§17.3.2.41](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-329-part-1).
- [ ] `webHidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Web Hidden Text; [§17.3.2.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-332-part-1).
- [ ] `color` — [w:CT_Color](word-runs.md#w-ct_color); Strict, Transitional. Run Content Color; [§17.3.2.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-277-part-1). Partial: Hexadecimal `color` exists; automatic color, theme selection, and tint/shade are missing. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `spacing` — [w:CT_SignedTwipsMeasure](word-runs.md#w-ct_signedtwipsmeasure); Strict, Transitional.
- [ ] `w` — [w:CT_TextScale](word-runs.md#w-ct_textscale); Strict, Transitional. Expanded/Compressed Text; [§17.3.2.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-331-part-1).
- [ ] `kern` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Font Kerning; [§17.3.2.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `position` — [w:CT_SignedHpsMeasure](word-runs.md#w-ct_signedhpsmeasure); Strict, Transitional. Vertically Raised or Lowered Text; [§17.3.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-299-part-1).
- [ ] `sz` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Non-Complex Script Font Size; [§17.3.2.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Partial: `size` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `szCs` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Complex Script Font Size; [§17.3.2.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-324-part-1). Partial: `size` is shared text intent; an independent complex-script size is missing. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `highlight` — [w:CT_Highlight](word-runs.md#w-ct_highlight); Strict, Transitional. Text Highlighting; [§17.3.2.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-291-part-1). Partial: Mote stores hexadecimal `highlight`; Word palette selection and mapping are not implemented. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `u` — [w:CT_Underline](word-runs.md#w-ct_underline); Strict, Transitional. Underline; [§17.3.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-325-part-1). Partial: `underline` is boolean; patterned/double/word-only underline and separate colors are missing. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `effect` — [w:CT_TextEffect](word-runs.md#w-ct_texteffect); Strict, Transitional. Animated Text Effect; [§17.3.2.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-287-part-1).
- [ ] `bdr` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional. Text Border; [§17.3.2.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional. Partial: Run-style fill/background metadata exists; full Word shading patterns and theme semantics are missing. Paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `fitText` — [w:CT_FitText](word-runs.md#w-ct_fittext); Strict, Transitional. Manual Run Width; [§17.3.2.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `vertAlign` — [w:CT_VerticalAlignRun](word-runs.md#w-ct_verticalalignrun); Strict, Transitional. Subscript/Superscript Text; [§17.3.2.42](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-330-part-1).
- [ ] `rtl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Right To Left Text; [§17.3.2.30](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-317-part-1).
- [ ] `cs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Use Complex Script Formatting on Run; [§17.3.2.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-280-part-1).
- [ ] `em` — [w:CT_Em](word-runs.md#w-ct_em); Strict, Transitional. Emphasis Mark; [§17.3.2.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-288-part-1).
- [ ] `lang` — [w:CT_Language](word-runs.md#w-ct_language); Strict, Transitional. Languages for Run Content; [§17.3.2.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-295-part-1).
- [ ] `eastAsianLayout` — [w:CT_EastAsianLayout](word-runs.md#w-ct_eastasianlayout); Strict, Transitional. East Asian Typography Settings; [§17.3.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1).
- [ ] `specVanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Paragraph Mark Is Always Hidden; [§17.3.2.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-322-part-1).
- [ ] `oMath` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Office Open XML Math; [§17.3.2.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).
- [ ] `rPrChange` — [w:CT_ParaRPrChange](word-runs.md#w-ct_pararprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_r"></a>

## w:CT_R

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@rsidRPr` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidDel` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidR` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.

<a id="w-ct_text"></a>

## w:CT_Text

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@xml:space` — `xml:space`; Strict, Transitional; use=optional. Partial: `code(preserve_space=true)` preserves authored whitespace; there is no general XML-space attribute. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).

<a id="w-ct_br"></a>

## w:CT_Br

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@type` — [w:ST_BrType](values-word.md#w-st_brtype); Strict, Transitional; use=optional. Partial: `line_break` represents an inline text break only; page and column break choices are not available through this directive. Evidence: [inline-text.md](../../../docs/specification/inline-text.md), [content.rs](../../../tests/content.rs).
- [ ] `@clear` — [w:ST_BrClear](values-word.md#w-st_brclear); Strict, Transitional; use=optional.

<a id="w-ct_color"></a>

## w:CT_Color

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=required. Partial: Hexadecimal colors are normalized; the automatic-color alternative is missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@themeColor` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.

<a id="w-ct_signedtwipsmeasure"></a>

## w:CT_SignedTwipsMeasure

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=required.

<a id="w-ct_textscale"></a>

## w:CT_TextScale

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TextScale](values-word.md#w-st_textscale); Strict, Transitional.

<a id="w-ct_hpsmeasure"></a>

## w:CT_HpsMeasure

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_HpsMeasure](values-word.md#w-st_hpsmeasure); Strict, Transitional; use=required.

<a id="w-ct_signedhpsmeasure"></a>

## w:CT_SignedHpsMeasure

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_SignedHpsMeasure](values-word.md#w-st_signedhpsmeasure); Strict, Transitional; use=required.

<a id="w-ct_highlight"></a>

## w:CT_Highlight

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_HighlightColor](values-word.md#w-st_highlightcolor); Strict, Transitional; use=required. Partial: `highlight` stores hexadecimal color, not the Word named-color palette. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).

<a id="w-ct_underline"></a>

## w:CT_Underline

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Underline](values-word.md#w-st_underline); Strict, Transitional; use=optional. Partial: `underline` represents on/off only; the complete style enumeration is not supported. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@color` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=optional; default=auto.
- [ ] `@themeColor` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.

<a id="w-ct_texteffect"></a>

## w:CT_TextEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TextEffect](values-word.md#w-st_texteffect); Strict, Transitional; use=required.

<a id="w-ct_border"></a>

## w:CT_Border

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Border](values-word.md#w-st_border); Strict, Transitional; use=required. Partial: Border side properties are retained; schema-specific interpretation and complete value validation are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@color` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=optional; default=auto. Partial: Border side properties are retained; schema-specific interpretation and complete value validation are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@themeColor` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@sz` — [w:ST_EighthPointMeasure](values-word.md#w-st_eighthpointmeasure); Strict, Transitional; use=optional. Partial: Border side properties are retained; schema-specific interpretation and complete value validation are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@space` — [w:ST_PointMeasure](values-word.md#w-st_pointmeasure); Strict, Transitional; use=optional; default=0. Partial: Border side properties are retained; schema-specific interpretation and complete value validation are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@shadow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@frame` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.

<a id="w-ct_fittext"></a>

## w:CT_FitText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=required.
- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.

<a id="w-ct_verticalalignrun"></a>

## w:CT_VerticalAlignRun

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_VerticalAlignRun](values-shared.md#s-st_verticalalignrun); Strict, Transitional; use=required.

<a id="w-ct_em"></a>

## w:CT_Em

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Em](values-word.md#w-st_em); Strict, Transitional; use=required.

<a id="w-ct_language"></a>

## w:CT_Language

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `@val` — [s:ST_Lang](values-shared.md#s-st_lang); Strict, Transitional; use=optional. Document language defaults emit the authored string as the Latin language tag. Script-specific language attributes remain unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [docx_defaults.rs](../../../tests/docx_defaults.rs).
- [ ] `@eastAsia` — [s:ST_Lang](values-shared.md#s-st_lang); Strict, Transitional; use=optional.
- [ ] `@bidi` — [s:ST_Lang](values-shared.md#s-st_lang); Strict, Transitional; use=optional.

<a id="w-ct_eastasianlayout"></a>

## w:CT_EastAsianLayout

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@combine` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@combineBrackets` — [w:ST_CombineBrackets](values-word.md#w-st_combinebrackets); Strict, Transitional; use=optional.
- [ ] `@vert` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@vertCompress` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.

<a id="w-ct_rprchange"></a>

## w:CT_RPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `rPr` — [w:CT_RPrOriginal](word-runs.md#w-ct_rproriginal); Strict, Transitional; minOccurs=1.

<a id="w-ct_trackchange"></a>

## w:CT_TrackChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.

<a id="w-ct_pararprchange"></a>

## w:CT_ParaRPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `rPr` — [w:CT_ParaRPrOriginal](word-runs.md#w-ct_pararproriginal); Strict, Transitional; minOccurs=1.

<a id="w-ct_rproriginal"></a>

## w:CT_RPrOriginal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `rStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional. Referenced Character Style; [§17.3.2.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-316-part-1). Partial: `style` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `rFonts` — [w:CT_Fonts](fonts.md#w-ct_fonts); Strict, Transitional. Run Fonts; [§17.3.2.26](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-301-part-1). Partial: Font names/aliases and script mappings exist; theme font references and selection hints are missing. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `b` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Bold; [§17.3.2.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Partial: `bold` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `bCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Bold; [§17.3.2.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Partial: `bold` is shared text intent; an independent complex-script bold switch is missing. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `i` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Italics; [§17.3.2.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Partial: `italic` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `iCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Italics; [§17.3.2.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Partial: `italic` is shared text intent; an independent complex-script italic switch is missing. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `caps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display All Characters As Capital Letters; [§17.3.2.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [ ] `smallCaps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Small Caps; [§17.3.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-320-part-1). Partial: `small_caps` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `strike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Single Strikethrough; [§17.3.2.37](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Partial: `strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `dstrike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Double Strikethrough; [§17.3.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1). Partial: `double_strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `outline` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display Character Outline; [§17.3.2.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-298-part-1).
- [ ] `shadow` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Shadow; [§17.3.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-319-part-1).
- [ ] `emboss` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Embossing; [§17.3.2.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `imprint` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Imprinting; [§17.3.2.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `noProof` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Do Not Check Spelling or Grammar; [§17.3.2.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).
- [ ] `snapToGrid` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional.
- [ ] `vanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Hidden Text; [§17.3.2.41](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-329-part-1).
- [ ] `webHidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Web Hidden Text; [§17.3.2.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-332-part-1).
- [ ] `color` — [w:CT_Color](word-runs.md#w-ct_color); Strict, Transitional. Run Content Color; [§17.3.2.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-277-part-1). Partial: Hexadecimal `color` exists; automatic color, theme selection, and tint/shade are missing. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `spacing` — [w:CT_SignedTwipsMeasure](word-runs.md#w-ct_signedtwipsmeasure); Strict, Transitional.
- [ ] `w` — [w:CT_TextScale](word-runs.md#w-ct_textscale); Strict, Transitional. Expanded/Compressed Text; [§17.3.2.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-331-part-1).
- [ ] `kern` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Font Kerning; [§17.3.2.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `position` — [w:CT_SignedHpsMeasure](word-runs.md#w-ct_signedhpsmeasure); Strict, Transitional. Vertically Raised or Lowered Text; [§17.3.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-299-part-1).
- [ ] `sz` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Non-Complex Script Font Size; [§17.3.2.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Partial: `size` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `szCs` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Complex Script Font Size; [§17.3.2.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-324-part-1). Partial: `size` is shared text intent; an independent complex-script size is missing. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `highlight` — [w:CT_Highlight](word-runs.md#w-ct_highlight); Strict, Transitional. Text Highlighting; [§17.3.2.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-291-part-1). Partial: Mote stores hexadecimal `highlight`; Word palette selection and mapping are not implemented. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `u` — [w:CT_Underline](word-runs.md#w-ct_underline); Strict, Transitional. Underline; [§17.3.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-325-part-1). Partial: `underline` is boolean; patterned/double/word-only underline and separate colors are missing. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `effect` — [w:CT_TextEffect](word-runs.md#w-ct_texteffect); Strict, Transitional. Animated Text Effect; [§17.3.2.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-287-part-1).
- [ ] `bdr` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional. Text Border; [§17.3.2.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional. Partial: Run-style fill/background metadata exists; full Word shading patterns and theme semantics are missing. Historical run formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `fitText` — [w:CT_FitText](word-runs.md#w-ct_fittext); Strict, Transitional. Manual Run Width; [§17.3.2.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `vertAlign` — [w:CT_VerticalAlignRun](word-runs.md#w-ct_verticalalignrun); Strict, Transitional. Subscript/Superscript Text; [§17.3.2.42](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-330-part-1).
- [ ] `rtl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Right To Left Text; [§17.3.2.30](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-317-part-1).
- [ ] `cs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Use Complex Script Formatting on Run; [§17.3.2.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-280-part-1).
- [ ] `em` — [w:CT_Em](word-runs.md#w-ct_em); Strict, Transitional. Emphasis Mark; [§17.3.2.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-288-part-1).
- [ ] `lang` — [w:CT_Language](word-runs.md#w-ct_language); Strict, Transitional. Languages for Run Content; [§17.3.2.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-295-part-1).
- [ ] `eastAsianLayout` — [w:CT_EastAsianLayout](word-runs.md#w-ct_eastasianlayout); Strict, Transitional. East Asian Typography Settings; [§17.3.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1).
- [ ] `specVanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Paragraph Mark Is Always Hidden; [§17.3.2.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-322-part-1).
- [ ] `oMath` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Office Open XML Math; [§17.3.2.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).

<a id="w-ct_pararproriginal"></a>

## w:CT_ParaRPrOriginal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `ins` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `del` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `moveFrom` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `moveTo` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `rStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional. Referenced Character Style; [§17.3.2.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-316-part-1). Partial: `style` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `rFonts` — [w:CT_Fonts](fonts.md#w-ct_fonts); Strict, Transitional. Run Fonts; [§17.3.2.26](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-301-part-1). Partial: Font names/aliases and script mappings exist; theme font references and selection hints are missing. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `b` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Bold; [§17.3.2.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Partial: `bold` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `bCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Bold; [§17.3.2.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-274-part-1). Partial: `bold` is shared text intent; an independent complex-script bold switch is missing. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `i` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Italics; [§17.3.2.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Partial: `italic` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `iCs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Complex Script Italics; [§17.3.2.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-292-part-1). Partial: `italic` is shared text intent; an independent complex-script italic switch is missing. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `caps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display All Characters As Capital Letters; [§17.3.2.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [ ] `smallCaps` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Small Caps; [§17.3.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-320-part-1). Partial: `small_caps` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `strike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Single Strikethrough; [§17.3.2.37](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Partial: `strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `dstrike` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Double Strikethrough; [§17.3.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1). Partial: `double_strike` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `outline` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Display Character Outline; [§17.3.2.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-298-part-1).
- [ ] `shadow` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Shadow; [§17.3.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-319-part-1).
- [ ] `emboss` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Embossing; [§17.3.2.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `imprint` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Imprinting; [§17.3.2.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `noProof` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Do Not Check Spelling or Grammar; [§17.3.2.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).
- [ ] `snapToGrid` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional.
- [ ] `vanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Hidden Text; [§17.3.2.41](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-329-part-1).
- [ ] `webHidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Web Hidden Text; [§17.3.2.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-332-part-1).
- [ ] `color` — [w:CT_Color](word-runs.md#w-ct_color); Strict, Transitional. Run Content Color; [§17.3.2.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-277-part-1). Partial: Hexadecimal `color` exists; automatic color, theme selection, and tint/shade are missing. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `spacing` — [w:CT_SignedTwipsMeasure](word-runs.md#w-ct_signedtwipsmeasure); Strict, Transitional.
- [ ] `w` — [w:CT_TextScale](word-runs.md#w-ct_textscale); Strict, Transitional. Expanded/Compressed Text; [§17.3.2.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-331-part-1).
- [ ] `kern` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Font Kerning; [§17.3.2.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-293-part-1).
- [ ] `position` — [w:CT_SignedHpsMeasure](word-runs.md#w-ct_signedhpsmeasure); Strict, Transitional. Vertically Raised or Lowered Text; [§17.3.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-299-part-1).
- [ ] `sz` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Non-Complex Script Font Size; [§17.3.2.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-323-part-1). Partial: `size` is represented on spans and run-style declarations; `strong`/`emphasis` also carry their intrinsic bold/italic intent. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `szCs` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional. Complex Script Font Size; [§17.3.2.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-324-part-1). Partial: `size` is shared text intent; an independent complex-script size is missing. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `highlight` — [w:CT_Highlight](word-runs.md#w-ct_highlight); Strict, Transitional. Text Highlighting; [§17.3.2.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-291-part-1). Partial: Mote stores hexadecimal `highlight`; Word palette selection and mapping are not implemented. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `u` — [w:CT_Underline](word-runs.md#w-ct_underline); Strict, Transitional. Underline; [§17.3.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0323-0327-0066.md#source-page-325-part-1). Partial: `underline` is boolean; patterned/double/word-only underline and separate colors are missing. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `effect` — [w:CT_TextEffect](word-runs.md#w-ct_texteffect); Strict, Transitional. Animated Text Effect; [§17.3.2.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-287-part-1).
- [ ] `bdr` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional. Text Border; [§17.3.2.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-276-part-1).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional. Partial: Run-style fill/background metadata exists; full Word shading patterns and theme semantics are missing. Historical paragraph-mark formatting is not represented separately. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `fitText` — [w:CT_FitText](word-runs.md#w-ct_fittext); Strict, Transitional. Manual Run Width; [§17.3.2.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-289-part-1).
- [ ] `vertAlign` — [w:CT_VerticalAlignRun](word-runs.md#w-ct_verticalalignrun); Strict, Transitional. Subscript/Superscript Text; [§17.3.2.42](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0328-0332-0067.md#source-page-330-part-1).
- [ ] `rtl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Right To Left Text; [§17.3.2.30](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0313-0317-0064.md#source-page-317-part-1).
- [ ] `cs` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Use Complex Script Formatting on Run; [§17.3.2.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0277-0281-0057.md#source-page-280-part-1).
- [ ] `em` — [w:CT_Em](word-runs.md#w-ct_em); Strict, Transitional. Emphasis Mark; [§17.3.2.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0287-0291-0059.md#source-page-288-part-1).
- [ ] `lang` — [w:CT_Language](word-runs.md#w-ct_language); Strict, Transitional. Languages for Run Content; [§17.3.2.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0292-0296-0060.md#source-page-295-part-1).
- [ ] `eastAsianLayout` — [w:CT_EastAsianLayout](word-runs.md#w-ct_eastasianlayout); Strict, Transitional. East Asian Typography Settings; [§17.3.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0282-0286-0058.md#source-page-283-part-1).
- [ ] `specVanish` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Paragraph Mark Is Always Hidden; [§17.3.2.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0318-0322-0065.md#source-page-322-part-1).
- [ ] `oMath` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional. Office Open XML Math; [§17.3.2.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0297-0302-0061.md#source-page-297-part-1).
