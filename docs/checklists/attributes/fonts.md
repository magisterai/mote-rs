# Font declaration and selection properties

[Inventory guide](README.md)

Used by font declarations and text font references. Includes script mappings, theme collections, font metadata, embedding variants, and document-level embedding switches.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_FontsList](#w-ct_fontslist)
- [w:CT_Fonts](#w-ct_fonts)
- [a:CT_FontScheme](#a-ct_fontscheme)
- [a:CT_TextFont](#a-ct_textfont)
- [p:CT_EmbeddedFontList](#p-ct_embeddedfontlist)
- [w:CT_Settings](#w-ct_settings)
- [w:CT_Font](#w-ct_font)
- [a:CT_FontCollection](#a-ct_fontcollection)
- [a:CT_OfficeArtExtensionList](#a-ct_officeartextensionlist)
- [p:CT_EmbeddedFontListEntry](#p-ct_embeddedfontlistentry)
- [w:CT_Panose](#w-ct_panose)
- [w:CT_Charset](#w-ct_charset)
- [w:CT_FontFamily](#w-ct_fontfamily)
- [w:CT_Pitch](#w-ct_pitch)
- [w:CT_FontSig](#w-ct_fontsig)
- [w:CT_FontRel](#w-ct_fontrel)
- [a:CT_SupplementalFont](#a-ct_supplementalfont)
- [a:CT_OfficeArtExtension](#a-ct_officeartextension)
- [p:CT_EmbeddedFontDataId](#p-ct_embeddedfontdataid)

<a id="w-ct_fontslist"></a>

## w:CT_FontsList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `font` — [w:CT_Font](fonts.md#w-ct_font); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Properties for a Single Font; [§17.8.3.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0689-0693-0130.md#source-page-693-part-1).

<a id="w-ct_fonts"></a>

## w:CT_Fonts

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@hint` — [w:ST_Hint](values-word.md#w-st_hint); Strict, Transitional.
- [ ] `@ascii` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional. Partial: `font.latin` supplies a shared Latin typeface; separate ASCII and high-ANSI choices are not exposed. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@hAnsi` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional. Partial: `font.latin` supplies a shared Latin typeface; separate ASCII and high-ANSI choices are not exposed. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `@eastAsia` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional. Model support: `font.east_asian` records and resolves a script-specific typeface. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `@cs` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional. Model support: `font.complex_script` records and resolves a script-specific typeface. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@asciiTheme` — [w:ST_Theme](values-word.md#w-st_theme); Strict, Transitional.
- [ ] `@hAnsiTheme` — [w:ST_Theme](values-word.md#w-st_theme); Strict, Transitional.
- [ ] `@eastAsiaTheme` — [w:ST_Theme](values-word.md#w-st_theme); Strict, Transitional.
- [ ] `@cstheme` — [w:ST_Theme](values-word.md#w-st_theme); Strict, Transitional.

<a id="a-ct_fontscheme"></a>

## a:CT_FontScheme

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `majorFont` — [a:CT_FontCollection](fonts.md#a-ct_fontcollection); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `minorFont` — [a:CT_FontCollection](fonts.md#a-ct_fontcollection); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=required.

<a id="a-ct_textfont"></a>

## a:CT_TextFont

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@typeface` — [a:ST_TextTypeface](values-drawing.md#a-st_texttypeface); Strict, Transitional; use=required. Partial: Direct typeface names and font aliases resolve; theme-token interpretation is missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@panose` — [s:ST_Panose](values-shared.md#s-st_panose); Strict, Transitional; use=optional.
- [ ] `@pitchFamily` — [a:ST_PitchFamily](values-drawing.md#a-st_pitchfamily); Strict, Transitional; use=optional; default=0.
- [ ] `@charset` — `xsd:byte`; Strict, Transitional; use=optional; default=1.

<a id="p-ct_embeddedfontlist"></a>

## p:CT_EmbeddedFontList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `embeddedFont` — [p:CT_EmbeddedFontListEntry](fonts.md#p-ct_embeddedfontlistentry); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="w-ct_settings"></a>

## w:CT_Settings

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Selected document-level controls: `embedTrueTypeFonts`, `embedSystemFonts`, `saveSubsetFonts`.

- [ ] `embedTrueTypeFonts` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Embed TrueType Fonts; [§17.8.3.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0689-0693-0130.md#source-page-692-part-1).
- [ ] `embedSystemFonts` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Embed Common System Fonts; [§17.8.3.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0689-0693-0130.md#source-page-692-part-1).
- [ ] `saveSubsetFonts` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Subset Fonts When Embedding; [§17.8.3.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0694-0699-0131.md#source-page-697-part-1).

<a id="w-ct_font"></a>

## w:CT_Font

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `altName` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Alternate Names for Font; [§17.8.3.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0680-0684-0128.md#source-page-681-part-1).
- [ ] `panose1` — [w:CT_Panose](fonts.md#w-ct_panose); Strict, Transitional; minOccurs=0; maxOccurs=1. Panose-1 Typeface Classification Number; [§17.8.3.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0694-0699-0131.md#source-page-695-part-1).
- [ ] `charset` — [w:CT_Charset](fonts.md#w-ct_charset); Strict, Transitional; minOccurs=0; maxOccurs=1. Character Set Supported By Font; [§17.8.3.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0680-0684-0128.md#source-page-683-part-1).
- [ ] `family` — [w:CT_FontFamily](fonts.md#w-ct_fontfamily); Strict, Transitional; minOccurs=0; maxOccurs=1. Font Family; [§17.8.3.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0689-0693-0130.md#source-page-693-part-1). Partial: `font.family` is retained metadata only; font-table interpretation is missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `notTrueType` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Not a TrueType outline Font; [§17.8.3.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0694-0699-0131.md#source-page-695-part-1).
- [ ] `pitch` — [w:CT_Pitch](fonts.md#w-ct_pitch); Strict, Transitional; minOccurs=0; maxOccurs=1. Font Pitch; [§17.8.3.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0694-0699-0131.md#source-page-696-part-1). Partial: `font.pitch` is retained metadata only; font-table interpretation is missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `sig` — [w:CT_FontSig](fonts.md#w-ct_fontsig); Strict, Transitional; minOccurs=0; maxOccurs=1. Supported Unicode Subranges and Code Pages; [§17.8.3.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0694-0699-0131.md#source-page-697-part-1).
- [ ] `embedRegular` — [w:CT_FontRel](fonts.md#w-ct_fontrel); Strict, Transitional; minOccurs=0; maxOccurs=1. Regular Font Style Embedding; [§17.8.3.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0689-0693-0130.md#source-page-690-part-1).
- [ ] `embedBold` — [w:CT_FontRel](fonts.md#w-ct_fontrel); Strict, Transitional; minOccurs=0; maxOccurs=1. Bold Style Font Style Embedding; [§17.8.3.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0680-0684-0128.md#source-page-683-part-1).
- [ ] `embedItalic` — [w:CT_FontRel](fonts.md#w-ct_fontrel); Strict, Transitional; minOccurs=0; maxOccurs=1. Italic Font Style Embedding; [§17.8.3.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0685-0688-0129.md#source-page-688-part-1).
- [ ] `embedBoldItalic` — [w:CT_FontRel](fonts.md#w-ct_fontrel); Strict, Transitional; minOccurs=0; maxOccurs=1. Bold Italic Font Style Embedding; [§17.8.3.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0685-0688-0129.md#source-page-686-part-1).
- [ ] `@name` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required. Partial: Font declarations distinguish a Mote alias from script typeface names; no separate Word font-table entry API exists. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).

<a id="a-ct_fontcollection"></a>

## a:CT_FontCollection

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `latin` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=1; maxOccurs=1. Latin Font; [§21.1.2.3.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3255-3259-0604.md#source-page-3255-part-1).
- [ ] `ea` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=1; maxOccurs=1. East Asian Font; [§21.1.2.3.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3246-3249-0602.md#source-page-3249-part-1).
- [ ] `cs` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=1; maxOccurs=1. Complex Script Font; [§21.1.2.3.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3242-3245-0601.md#source-page-3242-part-1).
- [ ] `font` — [a:CT_SupplementalFont](fonts.md#a-ct_supplementalfont); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_officeartextensionlist"></a>

## a:CT_OfficeArtExtensionList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `ext` — [a:CT_OfficeArtExtension](fonts.md#a-ct_officeartextension); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_embeddedfontlistentry"></a>

## p:CT_EmbeddedFontListEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `font` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `regular` — [p:CT_EmbeddedFontDataId](fonts.md#p-ct_embeddedfontdataid); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `bold` — [p:CT_EmbeddedFontDataId](fonts.md#p-ct_embeddedfontdataid); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `italic` — [p:CT_EmbeddedFontDataId](fonts.md#p-ct_embeddedfontdataid); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `boldItalic` — [p:CT_EmbeddedFontDataId](fonts.md#p-ct_embeddedfontdataid); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="w-ct_panose"></a>

## w:CT_Panose

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_Panose](values-shared.md#s-st_panose); Strict, Transitional; use=required.

<a id="w-ct_charset"></a>

## w:CT_Charset

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Transitional; use=optional.
- [ ] `@characterSet` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional; default=ISO-8859-1.

<a id="w-ct_fontfamily"></a>

## w:CT_FontFamily

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_FontFamily](values-word.md#w-st_fontfamily); Strict, Transitional; use=required.

<a id="w-ct_pitch"></a>

## w:CT_Pitch

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Pitch](values-word.md#w-st_pitch); Strict, Transitional; use=required.

<a id="w-ct_fontsig"></a>

## w:CT_FontSig

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@usb0` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=required.
- [ ] `@usb1` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=required.
- [ ] `@usb2` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=required.
- [ ] `@usb3` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=required.
- [ ] `@csb0` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=required.
- [ ] `@csb1` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=required.

<a id="w-ct_fontrel"></a>

## w:CT_FontRel

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
- [ ] `@fontKey` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional.
- [ ] `@subsetted` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.

<a id="a-ct_supplementalfont"></a>

## a:CT_SupplementalFont

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@script` — `xsd:string`; Strict, Transitional; use=required.
- [ ] `@typeface` — [a:ST_TextTypeface](values-drawing.md#a-st_texttypeface); Strict, Transitional; use=required.

<a id="a-ct_officeartextension"></a>

## a:CT_OfficeArtExtension

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `*` — extension wildcard; Strict, Transitional; processContents=lax; minOccurs=0; maxOccurs=unbounded.
- [ ] `@uri` — `xsd:token`; Strict, Transitional; use=required.

<a id="p-ct_embeddedfontdataid"></a>

## p:CT_EmbeddedFontDataId

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
