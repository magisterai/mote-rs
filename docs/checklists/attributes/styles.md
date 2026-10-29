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
- [w:CT_TrPr](#w-ct_trpr)
- [w:CT_TcPr](#w-ct_tcpr)
- [w:CT_TblStylePr](#w-ct_tblstylepr)
- [w:CT_TblPPr](#w-ct_tblppr)
- [w:CT_TblOverlap](#w-ct_tbloverlap)
- [w:CT_TblWidth](#w-ct_tblwidth)
- [w:CT_JcTable](#w-ct_jctable)
- [w:CT_TblBorders](#w-ct_tblborders)
- [w:CT_TblLayoutType](#w-ct_tbllayouttype)
- [w:CT_TblCellMar](#w-ct_tblcellmar)
- [w:CT_TblLook](#w-ct_tbllook)
- [w:CT_Height](#w-ct_height)
- [w:CT_TrPrChange](#w-ct_trprchange)
- [w:CT_HMerge](#w-ct_hmerge)
- [w:CT_VMerge](#w-ct_vmerge)
- [w:CT_TcBorders](#w-ct_tcborders)
- [w:CT_TcMar](#w-ct_tcmar)
- [w:CT_Headers](#w-ct_headers)
- [w:CT_CellMergeTrackChange](#w-ct_cellmergetrackchange)
- [w:CT_TcPrChange](#w-ct_tcprchange)
- [w:CT_TrPrBase](#w-ct_trprbase)
- [w:CT_TcPrInner](#w-ct_tcprinner)

<a id="w-ct_styles"></a>

## w:CT_Styles

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `docDefaults` — [w:CT_DocDefaults](styles.md#w-ct_docdefaults); Strict, Transitional; minOccurs=0. Document Default Paragraph and Run Properties; [§17.7.5.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0654-0658-0124.md#source-page-657-part-1).
- [ ] `latentStyles` — [w:CT_LatentStyles](styles.md#w-ct_latentstyles); Strict, Transitional; minOccurs=0; maxOccurs=1. Latent Style Information; [§17.7.4.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0628-0634-0119.md#source-page-634-part-1).
- [ ] `style` — [w:CT_Style](styles.md#w-ct_style); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Style Definition; [§17.7.4.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0649-0653-0123.md#source-page-650-part-1).

<a id="w-ct_docdefaults"></a>

## w:CT_DocDefaults

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `rPrDefault` — [w:CT_RPrDefault](styles.md#w-ct_rprdefault); Strict, Transitional; minOccurs=0. Default Run Properties; [§17.7.5.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0659-0665-0125.md#source-page-660-part-1).
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
- [ ] `trPr` — [w:CT_TrPr](styles.md#w-ct_trpr); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcPr` — [w:CT_TcPr](styles.md#w-ct_tcpr); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: Nested table-cell declaration metadata exists; cell-style application remains unsupported. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblStylePr` — [w:CT_TblStylePr](styles.md#w-ct_tblstylepr); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Style Conditional Table Formatting Properties; [§17.7.6.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-668-part-1). Partial: Edge and banding region metadata exists; applying conditional table regions remains unsupported. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@type` — [w:ST_StyleType](values-word.md#w-st_styletype); Strict, Transitional; use=optional. Partial: Paragraph, character, and table declarations are supported; numbering-style declarations are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `@styleId` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional. `style.name` maps to stable, unique Word identifiers with normalized reference lookup; relationship targets are checked. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs), [docx.rs](../../../tests/docx.rs), [docx_packages.rs](../../../tests/docx_packages.rs).
- [ ] `@default` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@customStyle` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.

<a id="w-ct_rprdefault"></a>

## w:CT_RPrDefault

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0.

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

- [ ] `tblStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Referenced Table Style; [§17.4.62](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0459-0465-0089.md#source-page-464-part-1).
- [ ] `tblpPr` — [w:CT_TblPPr](styles.md#w-ct_tblppr); Strict, Transitional; minOccurs=0; maxOccurs=1. Floating Table Positioning; [§17.4.57](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0448-0453-0087.md#source-page-452-part-1).
- [ ] `tblOverlap` — [w:CT_TblOverlap](styles.md#w-ct_tbloverlap); Strict, Transitional; minOccurs=0; maxOccurs=1. Floating Table Allows Other Tables to Overlap; [§17.4.56](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0448-0453-0087.md#source-page-450-part-1).
- [ ] `bidiVisual` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Visually Right to Left Table; [§17.4.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0378-0383-0076.md#source-page-383-part-1).
- [ ] `tblStyleRowBandSize` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0; maxOccurs=1. Number of Rows in Row Band; [§17.7.6.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-670-part-1).
- [ ] `tblStyleColBandSize` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0; maxOccurs=1. Number of Columns in Column Band; [§17.7.6.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-667-part-1).
- [ ] `tblW` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` has related normalized declaration fields; complete OOXML value coverage and table-style application are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `jc` — [w:CT_JcTable](styles.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` has related normalized declaration fields; complete OOXML value coverage and table-style application are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblCellSpacing` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` has related normalized declaration fields; complete OOXML value coverage and table-style application are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblInd` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` has related normalized declaration fields; complete OOXML value coverage and table-style application are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblBorders` — [w:CT_TblBorders](styles.md#w-ct_tblborders); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblLayout` — [w:CT_TblLayoutType](styles.md#w-ct_tbllayouttype); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` has related normalized declaration fields; complete OOXML value coverage and table-style application are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblCellMar` — [w:CT_TblCellMar](styles.md#w-ct_tblcellmar); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` has related normalized declaration fields; complete OOXML value coverage and table-style application are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblLook` — [w:CT_TblLook](styles.md#w-ct_tbllook); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: `style.table` has related normalized declaration fields; complete OOXML value coverage and table-style application are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tblCaption` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Caption; [§17.4.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0428-0434-0084.md#source-page-434-part-1).
- [ ] `tblDescription` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Description; [§17.4.46](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0435-0440-0085.md#source-page-440-part-1).

<a id="w-ct_trpr"></a>

## w:CT_TrPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `divId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `gridBefore` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Before First Cell; [§17.4.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-406-part-1).
- [ ] `gridAfter` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns After Last Cell; [§17.4.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-405-part-1).
- [ ] `wBefore` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width Before Table Row; [§17.4.86](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-492-part-1).
- [ ] `wAfter` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width After Table Row; [§17.4.85](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-491-part-1).
- [ ] `cantSplit` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Table Row Cannot Break Across Pages; [§17.4.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0384-0390-0077.md#source-page-387-part-1).
- [ ] `trHeight` — [w:CT_Height](styles.md#w-ct_height); Strict, Transitional; minOccurs=0. Table Row Height; [§17.4.80](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0480-0485-0092.md#source-page-484-part-1).
- [ ] `tblHeader` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Repeat Table Row on Every New Page; [§17.4.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0441-0447-0086.md#source-page-443-part-1).
- [ ] `tblCellSpacing` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `jc` — [w:CT_JcTable](styles.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `hidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `ins` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `del` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `trPrChange` — [w:CT_TrPrChange](styles.md#w-ct_trprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_tcpr"></a>

## w:CT_TcPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcW` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Table Cell Width; [§17.4.71](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0473-0479-0091.md#source-page-475-part-1).
- [ ] `gridSpan` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Spanned by Current Table Cell; [§17.4.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-409-part-1).
- [ ] `hMerge` — [w:CT_HMerge](styles.md#w-ct_hmerge); Transitional; minOccurs=0.
- [ ] `vMerge` — [w:CT_VMerge](styles.md#w-ct_vmerge); Strict, Transitional; minOccurs=0. Vertically Merged Cell; [§17.4.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-489-part-1).
- [ ] `tcBorders` — [w:CT_TcBorders](styles.md#w-ct_tcborders); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Cell Borders; [§17.4.66](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-469-part-1).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0. Partial: Related table-cell style values are normalized and retained; full value coverage and application to cells are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `noWrap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Don't Wrap Cell Content; [§17.4.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0421-0427-0083.md#source-page-423-part-1). Partial: Related table-cell style values are normalized and retained; full value coverage and application to cells are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tcMar` — [w:CT_TcMar](styles.md#w-ct_tcmar); Strict, Transitional; minOccurs=0; maxOccurs=1. Single Table Cell Margins; [§17.4.68](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1). Partial: Related table-cell style values are normalized and retained; full value coverage and application to cells are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0; maxOccurs=1. Partial: Related table-cell style values are normalized and retained; full value coverage and application to cells are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `tcFitText` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Fit Text Within Cell; [§17.4.67](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1).
- [ ] `vAlign` — [w:CT_VerticalJc](word-paragraphs.md#w-ct_verticaljc); Strict, Transitional; minOccurs=0. Partial: Related table-cell style values are normalized and retained; full value coverage and application to cells are missing. Evidence: [styles-and-fonts.md](../../../docs/specification/styles-and-fonts.md), [declarations.rs](../../../tests/declarations.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `hideMark` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Ignore End Of Cell Marker In Row Height Calculation; [§17.4.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0415-0420-0082.md#source-page-415-part-1).
- [ ] `headers` — [w:CT_Headers](styles.md#w-ct_headers); Strict, Transitional; minOccurs=0. Header Cells Associated With Table Cell; [§17.4.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-413-part-1).
- [ ] `cellIns` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellDel` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellMerge` — [w:CT_CellMergeTrackChange](styles.md#w-ct_cellmergetrackchange); Strict, Transitional; minOccurs=0.
- [ ] `tcPrChange` — [w:CT_TcPrChange](styles.md#w-ct_tcprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_tblstylepr"></a>

## w:CT_TblStylePr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `pPr` — [w:CT_PPrGeneral](styles.md#w-ct_pprgeneral); Strict, Transitional; minOccurs=0.
- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0.
- [ ] `tblPr` — [w:CT_TblPrBase](styles.md#w-ct_tblprbase); Strict, Transitional; minOccurs=0.
- [ ] `trPr` — [w:CT_TrPr](styles.md#w-ct_trpr); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcPr` — [w:CT_TcPr](styles.md#w-ct_tcpr); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@type` — [w:ST_TblStyleOverrideType](values-word.md#w-st_tblstyleoverridetype); Strict, Transitional; use=required.

<a id="w-ct_tblppr"></a>

## w:CT_TblPPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@leftFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@rightFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@topFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@bottomFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@vertAnchor` — [w:ST_VAnchor](values-word.md#w-st_vanchor); Strict, Transitional.
- [ ] `@horzAnchor` — [w:ST_HAnchor](values-word.md#w-st_hanchor); Strict, Transitional.
- [ ] `@tblpXSpec` — [s:ST_XAlign](values-shared.md#s-st_xalign); Strict, Transitional.
- [ ] `@tblpX` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional.
- [ ] `@tblpYSpec` — [s:ST_YAlign](values-shared.md#s-st_yalign); Strict, Transitional.
- [ ] `@tblpY` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional.

<a id="w-ct_tbloverlap"></a>

## w:CT_TblOverlap

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TblOverlap](values-word.md#w-st_tbloverlap); Strict, Transitional; use=required.

<a id="w-ct_tblwidth"></a>

## w:CT_TblWidth

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@w` — [w:ST_MeasurementOrPercent](values-word.md#w-st_measurementorpercent); Strict, Transitional.
- [ ] `@type` — [w:ST_TblWidth](values-word.md#w-st_tblwidth); Strict, Transitional.

<a id="w-ct_jctable"></a>

## w:CT_JcTable

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_JcTable](values-word.md#w-st_jctable); Strict, Transitional; use=required.

<a id="w-ct_tblborders"></a>

## w:CT_TblBorders

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `start` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `left` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `end` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `right` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `insideH` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `insideV` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.

<a id="w-ct_tbllayouttype"></a>

## w:CT_TblLayoutType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@type` — [w:ST_TblLayoutType](values-word.md#w-st_tbllayouttype); Strict, Transitional.

<a id="w-ct_tblcellmar"></a>

## w:CT_TblCellMar

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `start` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `left` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `end` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `right` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Transitional; minOccurs=0.

<a id="w-ct_tbllook"></a>

## w:CT_TblLook

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@firstRow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastRow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@firstColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@noHBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@noVBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@val` — [w:ST_ShortHexNumber](values-word.md#w-st_shorthexnumber); Transitional.

<a id="w-ct_height"></a>

## w:CT_Height

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@hRule` — [w:ST_HeightRule](values-word.md#w-st_heightrule); Strict, Transitional.

<a id="w-ct_trprchange"></a>

## w:CT_TrPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `trPr` — [w:CT_TrPrBase](styles.md#w-ct_trprbase); Strict, Transitional; minOccurs=1.

<a id="w-ct_hmerge"></a>

## w:CT_HMerge

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd).

- [ ] `@val` — [w:ST_Merge](values-word.md#w-st_merge); Transitional.

<a id="w-ct_vmerge"></a>

## w:CT_VMerge

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Merge](values-word.md#w-st_merge); Strict, Transitional.

<a id="w-ct_tcborders"></a>

## w:CT_TcBorders

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `start` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `left` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `end` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `right` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `insideH` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `insideV` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `tl2br` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0. Table Cell Top Left to Bottom Right Diagonal Border; [§17.4.73](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0473-0479-0091.md#source-page-478-part-1).
- [ ] `tr2bl` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0. Table Cell Top Right to Bottom Left Diagonal Border; [§17.4.79](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0480-0485-0092.md#source-page-483-part-1).

<a id="w-ct_tcmar"></a>

## w:CT_TcMar

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `start` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `left` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `end` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `right` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Transitional; minOccurs=0.

<a id="w-ct_headers"></a>

## w:CT_Headers

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `header` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional. Header Cell Reference; [§17.4.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-411-part-1).

<a id="w-ct_cellmergetrackchange"></a>

## w:CT_CellMergeTrackChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `@vMerge` — [w:ST_AnnotationVMerge](values-word.md#w-st_annotationvmerge); Strict, Transitional; use=optional.
- [ ] `@vMergeOrig` — [w:ST_AnnotationVMerge](values-word.md#w-st_annotationvmerge); Strict, Transitional; use=optional.

<a id="w-ct_tcprchange"></a>

## w:CT_TcPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `tcPr` — [w:CT_TcPrInner](styles.md#w-ct_tcprinner); Strict, Transitional; minOccurs=1.

<a id="w-ct_trprbase"></a>

## w:CT_TrPrBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `divId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `gridBefore` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Before First Cell; [§17.4.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-406-part-1).
- [ ] `gridAfter` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns After Last Cell; [§17.4.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-405-part-1).
- [ ] `wBefore` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width Before Table Row; [§17.4.86](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-492-part-1).
- [ ] `wAfter` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width After Table Row; [§17.4.85](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-491-part-1).
- [ ] `cantSplit` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Table Row Cannot Break Across Pages; [§17.4.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0384-0390-0077.md#source-page-387-part-1).
- [ ] `trHeight` — [w:CT_Height](styles.md#w-ct_height); Strict, Transitional; minOccurs=0. Table Row Height; [§17.4.80](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0480-0485-0092.md#source-page-484-part-1).
- [ ] `tblHeader` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Repeat Table Row on Every New Page; [§17.4.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0441-0447-0086.md#source-page-443-part-1).
- [ ] `tblCellSpacing` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `jc` — [w:CT_JcTable](styles.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `hidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.

<a id="w-ct_tcprinner"></a>

## w:CT_TcPrInner

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcW` — [w:CT_TblWidth](styles.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Table Cell Width; [§17.4.71](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0473-0479-0091.md#source-page-475-part-1).
- [ ] `gridSpan` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Spanned by Current Table Cell; [§17.4.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-409-part-1).
- [ ] `hMerge` — [w:CT_HMerge](styles.md#w-ct_hmerge); Transitional; minOccurs=0.
- [ ] `vMerge` — [w:CT_VMerge](styles.md#w-ct_vmerge); Strict, Transitional; minOccurs=0. Vertically Merged Cell; [§17.4.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-489-part-1).
- [ ] `tcBorders` — [w:CT_TcBorders](styles.md#w-ct_tcborders); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Cell Borders; [§17.4.66](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-469-part-1).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0.
- [ ] `noWrap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Don't Wrap Cell Content; [§17.4.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0421-0427-0083.md#source-page-423-part-1).
- [ ] `tcMar` — [w:CT_TcMar](styles.md#w-ct_tcmar); Strict, Transitional; minOccurs=0; maxOccurs=1. Single Table Cell Margins; [§17.4.68](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1).
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcFitText` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Fit Text Within Cell; [§17.4.67](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1).
- [ ] `vAlign` — [w:CT_VerticalJc](word-paragraphs.md#w-ct_verticaljc); Strict, Transitional; minOccurs=0.
- [ ] `hideMark` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Ignore End Of Cell Marker In Row Height Calculation; [§17.4.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0415-0420-0082.md#source-page-415-part-1).
- [ ] `headers` — [w:CT_Headers](styles.md#w-ct_headers); Strict, Transitional; minOccurs=0. Header Cells Associated With Table Cell; [§17.4.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-413-part-1).
- [ ] `cellIns` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellDel` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellMerge` — [w:CT_CellMergeTrackChange](styles.md#w-ct_cellmergetrackchange); Strict, Transitional; minOccurs=0.
