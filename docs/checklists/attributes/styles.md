# Word style properties

[Inventory guide](README.md)

Used by style declarations: paragraph, character, numbering, table, and conditional-region properties. Includes table/row/cell property records even though applying table styles remains unsupported.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_Styles](#w-ct_styles)
- [w:CT_DocDefaults](#w-ct_docdefaults)
- [w:CT_LatentStyles](#w-ct_latentstyles)
- [w:CT_Style](#w-ct_style)
- [w:CT_RPrDefault](#w-ct_rprdefault)
- [w:CT_PPrDefault](#w-ct_pprdefault)
- [w:CT_LsdException](#w-ct_lsdexception)
- [w:CT_PPrGeneral](#w-ct_pprgeneral)
- [w:CT_TblPrBase](#w-ct_tblprbase)
- [w:CT_TblStylePr](#w-ct_tblstylepr)

<a id="w-ct_styles"></a>

## w:CT_Styles

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `docDefaults` — [w:CT_DocDefaults](styles.md#w-ct_docdefaults); Strict, Transitional; minOccurs=0. Document Default Paragraph and Run Properties; [§17.7.5.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0654-0658-0124.md#source-page-657-part-1). Document font, size and language defaults are emitted; arbitrary default run/paragraph properties are unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [docx_defaults.rs](../../../tests/docx_defaults.rs).
- [ ] `latentStyles` — [w:CT_LatentStyles](styles.md#w-ct_latentstyles); Strict, Transitional; minOccurs=0; maxOccurs=1. Latent Style Information; [§17.7.4.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0628-0634-0119.md#source-page-634-part-1).
- [ ] `style` — [w:CT_Style](styles.md#w-ct_style); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Style Definition; [§17.7.4.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0649-0653-0123.md#source-page-650-part-1).

<a id="w-ct_docdefaults"></a>

## w:CT_DocDefaults

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `rPrDefault` — [w:CT_RPrDefault](styles.md#w-ct_rprdefault); Strict, Transitional; minOccurs=0. Default Run Properties; [§17.7.5.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0659-0665-0125.md#source-page-660-part-1). Document font, size and language defaults are emitted; other run defaults are unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [docx_defaults.rs](../../../tests/docx_defaults.rs).
- [ ] `pPrDefault` — [w:CT_PPrDefault](styles.md#w-ct_pprdefault); Strict, Transitional; minOccurs=0. Default Paragraph Properties; [§17.7.5.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0659-0665-0125.md#source-page-659-part-1).

<a id="w-ct_latentstyles"></a>

## w:CT_LatentStyles

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `lsdException` — [w:CT_LsdException](styles.md#w-ct_lsdexception); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Latent Style Exception; [§17.7.4.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0639-0643-0121.md#source-page-640-part-1).
- [ ] `@defLockedState` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@defUIPriority` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional.
- [ ] `@defSemiHidden` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@defUnhideWhenUsed` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@defQFormat` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@count` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional.

<a id="w-ct_style"></a>

## w:CT_Style

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `name` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: Mote uses a declaration name as identity; a separately authored display name is missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `aliases` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Alternate Style Names; [§17.7.4.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0628-0634-0119.md#source-page-628-part-1).
- [x] `basedOn` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Parent Style ID; [§17.7.4.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0628-0634-0119.md#source-page-631-part-1). Model support: `style.based_on` is recorded; only `based_on` participates in explicit inheritance resolution. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `next` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Style For Next Paragraph; [§17.7.4.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0644-0648-0122.md#source-page-644-part-1). Model support: `style.next` is recorded; only `based_on` participates in explicit inheritance resolution. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `link` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Linked Style Reference; [§17.7.4.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0635-0638-0120.md#source-page-638-part-1). Model support: `style.link` is recorded; only `based_on` participates in explicit inheritance resolution. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `autoRedefine` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Merge User Formatting Into Style Definition; [§17.7.4.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0628-0634-0119.md#source-page-629-part-1).
- [ ] `hidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `uiPriority` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Optional User Interface Sorting Order; [§17.7.4.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0654-0658-0124.md#source-page-655-part-1).
- [ ] `semiHidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Hide Style From Main User Interface; [§17.7.4.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0649-0653-0123.md#source-page-649-part-1).
- [ ] `unhideWhenUsed` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Remove Semi-Hidden Property When Style Is Used; [§17.7.4.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0654-0658-0124.md#source-page-656-part-1).
- [ ] `qFormat` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Primary Style; [§17.7.4.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0644-0648-0122.md#source-page-647-part-1).
- [ ] `locked` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Style Cannot Be Applied; [§17.7.4.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0639-0643-0121.md#source-page-639-part-1).
- [ ] `personal` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. E-Mail Message Text Style; [§17.7.4.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0644-0648-0122.md#source-page-646-part-1).
- [ ] `personalCompose` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. E-Mail Message Composition Style; [§17.7.4.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0644-0648-0122.md#source-page-646-part-1).
- [ ] `personalReply` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. E-Mail Message Reply Style; [§17.7.4.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0644-0648-0122.md#source-page-647-part-1).
- [ ] `rsid` — [w:CT_LongHexNumber](numbering.md#w-ct_longhexnumber); Strict, Transitional; minOccurs=0. Revision Identifier for Style Definition; [§17.7.4.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0644-0648-0122.md#source-page-648-part-1).
- [ ] `pPr` — [w:CT_PPrGeneral](styles.md#w-ct_pprgeneral); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.paragraph` supports the documented subset; additional paragraph fields are inventoried separately. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.run` supports the documented subset; additional run fields are inventoried separately. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblPr` — [w:CT_TblPrBase](styles.md#w-ct_tblprbase); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` normalizes a subset of declaration metadata; table application remains unsupported. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `trPr` — [w:CT_TrPr](tables.md#w-ct_trpr); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcPr` — [w:CT_TcPr](tables.md#w-ct_tcpr); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: Nested table-cell declaration metadata exists; cell-style application remains unsupported. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblStylePr` — [w:CT_TblStylePr](styles.md#w-ct_tblstylepr); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Style Conditional Table Formatting Properties; [§17.7.6.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-668-part-1). Partial: Edge and banding region metadata exists; applying conditional table regions remains unsupported. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@type` — [w:ST_StyleType](values-word.md#w-st_styletype); Strict, Transitional; use=optional. Partial: Paragraph, character, and table declarations are supported; numbering-style declarations are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `@styleId` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional. `style.name` maps to stable, unique Word identifiers with normalized reference lookup; relationship targets are checked. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs), [docx.rs](../../../tests/docx.rs), [docx_packages.rs](../../../tests/docx_packages.rs).
- [ ] `@default` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@customStyle` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.

<a id="w-ct_rprdefault"></a>

## w:CT_RPrDefault

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0. Document font, size and language defaults only. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [docx_defaults.rs](../../../tests/docx_defaults.rs).

<a id="w-ct_pprdefault"></a>

## w:CT_PPrDefault

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `pPr` — [w:CT_PPrGeneral](styles.md#w-ct_pprgeneral); Strict, Transitional; minOccurs=0.

<a id="w-ct_lsdexception"></a>

## w:CT_LsdException

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@name` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@locked` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@uiPriority` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional.
- [ ] `@semiHidden` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@unhideWhenUsed` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@qFormat` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.

<a id="w-ct_pprgeneral"></a>

## w:CT_PPrGeneral

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `pStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Model support: `style` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `keepNext` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Keep Paragraph With Next Paragraph; [§17.3.1.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0233-0237-0049.md#source-page-236-part-1). Model support: `keep_with_next` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `keepLines` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Keep All Lines On One Page; [§17.3.1.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0233-0237-0049.md#source-page-235-part-1). Model support: `keep_together` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `pageBreakBefore` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Start Paragraph on Next Page; [§17.3.1.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-247-part-1). Model support: `page_break_before` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `framePr` — [w:CT_FramePr](word-paragraphs.md#w-ct_framepr); Strict, Transitional; minOccurs=0. Text Frame Properties; [§17.3.1.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0216-0220-0045.md#source-page-218-part-1).
- [x] `widowControl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow First/Last Line to Display on a Separate Page; [§17.3.1.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-271-part-1). Model support: `widow_control` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `numPr` — [w:CT_NumPr](word-paragraphs.md#w-ct_numpr); Strict, Transitional; minOccurs=0. Numbering Definition Instance Reference; [§17.3.1.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-243-part-1). Partial: List and heading numbering emits level/instance references and start overrides; numbering revision metadata is not represented. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs), [docx_packages.rs](../../../tests/docx_packages.rs).
- [ ] `suppressLineNumbers` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Suppress Line Numbers for Paragraph; [§17.3.1.35](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-262-part-1).
- [ ] `pBdr` — [w:CT_PBdr](word-paragraphs.md#w-ct_pbdr); Strict, Transitional; minOccurs=0. Paragraph Borders; [§17.3.1.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0249-0254-0052.md#source-page-249-part-1). Partial: `border` retains side objects as metadata; complete border values and style semantics are not implemented. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0. Partial: `fill` covers hexadecimal background color; patterns, automatic colors, and theme-based shading are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `tabs` — [w:CT_Tabs](word-paragraphs.md#w-ct_tabs); Strict, Transitional; minOccurs=0. Set of Custom Tab Stops; [§17.3.1.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-265-part-1). Partial: `tabs` retains position/alignment/leader metadata; complete stop semantics and schema value validation are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `suppressAutoHyphens` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Suppress Hyphenation for Paragraph; [§17.3.1.34](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-261-part-1).
- [ ] `kinsoku` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Use East Asian Typography Rules for First and Last Character per Line; [§17.3.1.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0238-0242-0050.md#source-page-238-part-1).
- [ ] `wordWrap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow Line Breaking At Character Level; [§17.3.1.45](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-272-part-1).
- [ ] `overflowPunct` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow Punctuation to Extend Past Text Extents; [§17.3.1.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-244-part-1).
- [ ] `topLinePunct` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Compress Punctuation at Start of a Line; [§17.3.1.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-271-part-1).
- [ ] `autoSpaceDE` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Spacing of Latin and East Asian Text; [§17.3.1.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-205-part-1).
- [ ] `autoSpaceDN` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Spacing of East Asian Text and Numbers; [§17.3.1.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-206-part-1).
- [ ] `bidi` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `adjustRightInd` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Right Indent When Using Document Grid; [§17.3.1.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-205-part-1).
- [ ] `snapToGrid` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `spacing` — [w:CT_Spacing](word-paragraphs.md#w-ct_spacing); Strict, Transitional; minOccurs=0. Partial: `spacing` and flat aliases cover physical before/after spacing and three line rules; line-count and automatic before/after controls are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `ind` — [w:CT_Ind](word-paragraphs.md#w-ct_ind); Strict, Transitional; minOccurs=0. Paragraph Indentation; [§17.3.1.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0229-0232-0048.md#source-page-229-part-1). Partial: `indent` covers non-negative physical left/right/first-line/hanging values; signed, character-count, and logical start/end controls are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `contextualSpacing` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Ignore Spacing Above and Below When Using Identical Styles; [§17.3.1.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0216-0220-0045.md#source-page-216-part-1).
- [ ] `mirrorIndents` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Use Left/Right Indents as Inside/Outside Indents; [§17.3.1.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0238-0242-0050.md#source-page-242-part-1).
- [ ] `suppressOverlap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Prevent Text Frames From Overlapping; [§17.3.1.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-263-part-1).
- [ ] `jc` — [w:CT_Jc](word-paragraphs.md#w-ct_jc); Strict, Transitional; minOccurs=0. Partial: `alignment` supports left, center, right, and justify; other Word alignment values have no defined Mote mapping. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0.
- [ ] `textAlignment` — [w:CT_TextAlignment](word-paragraphs.md#w-ct_textalignment); Strict, Transitional; minOccurs=0. Vertical Character Alignment on Line; [§17.3.1.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-265-part-1).
- [ ] `textboxTightWrap` — [w:CT_TextboxTightWrap](word-paragraphs.md#w-ct_textboxtightwrap); Strict, Transitional; minOccurs=0. Allow Surrounding Paragraphs to Tight Wrap to Text Box Contents; [§17.3.1.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-267-part-1).
- [x] `outlineLvl` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Associated Outline Level; [§17.3.1.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-243-part-1). Model support: `outline_level` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `divId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `pPrChange` — [w:CT_PPrChange](word-paragraphs.md#w-ct_pprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_tblprbase"></a>

## w:CT_TblPrBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `tblStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Referenced Table Style; [§17.4.62](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0459-0465-0089.md#source-page-464-part-1). Named table styles and conditional regions are emitted in editable DOCX. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblpPr` — [w:CT_TblPPr](tables.md#w-ct_tblppr); Strict, Transitional; minOccurs=0; maxOccurs=1. Floating Table Positioning; [§17.4.57](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0448-0453-0087.md#source-page-452-part-1).
- [ ] `tblOverlap` — [w:CT_TblOverlap](tables.md#w-ct_tbloverlap); Strict, Transitional; minOccurs=0; maxOccurs=1. Floating Table Allows Other Tables to Overlap; [§17.4.56](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0448-0453-0087.md#source-page-450-part-1).
- [ ] `bidiVisual` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Visually Right to Left Table; [§17.4.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0378-0383-0076.md#source-page-383-part-1).
- [ ] `tblStyleRowBandSize` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0; maxOccurs=1. Number of Rows in Row Band; [§17.7.6.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-670-part-1).
- [ ] `tblStyleColBandSize` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0; maxOccurs=1. Number of Columns in Column Band; [§17.7.6.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-667-part-1).
- [ ] `tblW` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical table width is emitted; percentage width is not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `jc` — [w:CT_JcTable](tables.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1. Four Mote alignment values map to left/center/right placement; justify maps to left. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCellSpacing` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical cell spacing is emitted; other width types are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblInd` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical indentation is emitted; other width types are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblBorders` — [w:CT_TblBorders](tables.md#w-ct_tblborders); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical outside/inside edges, widths and colors are emitted; themed border controls are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [x] `tblLayout` — [w:CT_TblLayoutType](tables.md#w-ct_tbllayouttype); Strict, Transitional; minOccurs=0; maxOccurs=1. Table layout accepts auto/autofit/fixed; fixed emits a fixed DOCX grid. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCellMar` — [w:CT_TblCellMar](tables.md#w-ct_tblcellmar); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical cell margins and shorthands are emitted; logical edges are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblLook` — [w:CT_TblLook](tables.md#w-ct_tbllook); Strict, Transitional; minOccurs=0; maxOccurs=1. Declared conditional regions select table look flags; independent look-bit authorship is not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCaption` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Caption; [§17.4.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0428-0434-0084.md#source-page-434-part-1).
- [ ] `tblDescription` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Description; [§17.4.46](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0435-0440-0085.md#source-page-440-part-1).

<a id="w-ct_tblstylepr"></a>

## w:CT_TblStylePr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `pPr` — [w:CT_PPrGeneral](styles.md#w-ct_pprgeneral); Strict, Transitional; minOccurs=0.
- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0.
- [ ] `tblPr` — [w:CT_TblPrBase](styles.md#w-ct_tblprbase); Strict, Transitional; minOccurs=0.
- [ ] `trPr` — [w:CT_TrPr](tables.md#w-ct_trpr); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcPr` — [w:CT_TcPr](tables.md#w-ct_tcpr); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@type` — [w:ST_TblStyleOverrideType](values-word.md#w-st_tblstyleoverridetype); Strict, Transitional; use=required.
