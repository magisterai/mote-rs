# Section and header/footer properties

[Inventory guide](README.md)

Complete section property, header/footer reference and page numbering families. Story content inherits the paragraph, run, table and image inventories. First and odd/even variants remain visible even where no authoring mapping exists.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_SectPr](#w-ct_sectpr)
- [w:CT_HdrFtrRef](#w-ct_hdrftrref)
- [w:CT_HdrFtr](#w-ct_hdrftr)
- [w:CT_FtnProps](#w-ct_ftnprops)
- [w:CT_EdnProps](#w-ct_ednprops)
- [w:CT_SectType](#w-ct_secttype)
- [w:CT_PageSz](#w-ct_pagesz)
- [w:CT_PageMar](#w-ct_pagemar)
- [w:CT_PaperSource](#w-ct_papersource)
- [w:CT_PageBorders](#w-ct_pageborders)
- [w:CT_LineNumber](#w-ct_linenumber)
- [w:CT_PageNumber](#w-ct_pagenumber)
- [w:CT_Columns](#w-ct_columns)
- [w:CT_DocGrid](#w-ct_docgrid)
- [w:CT_Rel](#w-ct_rel)
- [w:CT_SectPrChange](#w-ct_sectprchange)
- [w:CT_FtnPos](#w-ct_ftnpos)
- [w:CT_NumFmt](#w-ct_numfmt)
- [w:CT_NumRestart](#w-ct_numrestart)
- [w:CT_EdnPos](#w-ct_ednpos)
- [w:CT_TopPageBorder](#w-ct_toppageborder)
- [w:CT_PageBorder](#w-ct_pageborder)
- [w:CT_BottomPageBorder](#w-ct_bottompageborder)
- [w:CT_Column](#w-ct_column)
- [w:CT_SectPrBase](#w-ct_sectprbase)

<a id="w-ct_sectpr"></a>

## w:CT_SectPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `headerReference` — [w:CT_HdrFtrRef](sections-and-stories.md#w-ct_hdrftrref); Strict, Transitional; minOccurs=0. Default story declaration, inheritance and explicit empty resets; separate first/even variants have no authoring mapping. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `footerReference` — [w:CT_HdrFtrRef](sections-and-stories.md#w-ct_hdrftrref); Strict, Transitional; minOccurs=0. Default story declaration, inheritance and explicit empty resets; separate first/even variants have no authoring mapping. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `footnotePr` — [w:CT_FtnProps](sections-and-stories.md#w-ct_ftnprops); Strict, Transitional; minOccurs=0.
- [ ] `endnotePr` — [w:CT_EdnProps](sections-and-stories.md#w-ct_ednprops); Strict, Transitional; minOccurs=0.
- [x] `type` — [w:CT_SectType](sections-and-stories.md#w-ct_secttype); Strict, Transitional; minOccurs=0. Section Type; [§17.6.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0618-0622-0117.md#source-page-621-part-1). properties.start supports every section-start value; native parity and next-column behavior remain limited. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `pgSz` — [w:CT_PageSz](sections-and-stories.md#w-ct_pagesz); Strict, Transitional; minOccurs=0. Page Size; [§17.6.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0594-0598-0112.md#source-page-595-part-1). Named paper sizes, explicit dimensions and orientation are supported; printer paper codes are unsupported. Implicit and explicit sections are supported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs), [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `pgMar` — [w:CT_PageMar](sections-and-stories.md#w-ct_pagemar); Strict, Transitional; minOccurs=0. Page Margins; [§17.6.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0585-0589-0110.md#source-page-589-part-1). All seven margin fields are supported for non-negative lengths; negative top/bottom margins are unsupported. Implicit and explicit sections are supported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs), [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `paperSrc` — [w:CT_PaperSource](sections-and-stories.md#w-ct_papersource); Strict, Transitional; minOccurs=0. Paper Source Information; [§17.6.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0585-0589-0110.md#source-page-585-part-1).
- [ ] `pgBorders` — [w:CT_PageBorders](sections-and-stories.md#w-ct_pageborders); Strict, Transitional; minOccurs=0. Page Borders; [§17.6.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0585-0589-0110.md#source-page-586-part-1).
- [ ] `lnNumType` — [w:CT_LineNumber](sections-and-stories.md#w-ct_linenumber); Strict, Transitional; minOccurs=0. Line Numbering Settings; [§17.6.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0580-0584-0109.md#source-page-583-part-1).
- [ ] `pgNumType` — [w:CT_PageNumber](sections-and-stories.md#w-ct_pagenumber); Strict, Transitional; minOccurs=0. Page Numbering Settings; [§17.6.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0590-0593-0111.md#source-page-593-part-1). page_numbering restart/start/format; chapter numbering properties are not mapped. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `cols` — [w:CT_Columns](sections-and-stories.md#w-ct_columns); Strict, Transitional; minOccurs=0. Column Definitions; [§17.6.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0568-0574-0107.md#source-page-568-part-1). Implicit and explicit sections: equal-width columns with count, gap and separator. Individual column widths remain unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs), [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `formProt` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Only Allow Editing of Form Fields; [§17.6.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0575-0579-0108.md#source-page-575-part-1).
- [x] `vAlign` — [w:CT_VerticalJc](tables.md#w-ct_verticaljc); Strict, Transitional; minOccurs=0. properties.vertical_alignment: top, center, bottom, justified; both normalizes to justified. Native vertical placement remains top-aligned with a warning. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `noEndnote` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `titlePg` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0.
- [ ] `bidi` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `rtlGutter` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Gutter on Right Side of Page; [§17.6.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0603-0607-0114.md#source-page-605-part-1).
- [ ] `docGrid` — [w:CT_DocGrid](sections-and-stories.md#w-ct_docgrid); Strict, Transitional; minOccurs=0. Document Grid; [§17.6.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0568-0574-0107.md#source-page-570-part-1).
- [ ] `printerSettings` — [w:CT_Rel](sections-and-stories.md#w-ct_rel); Strict, Transitional; minOccurs=0. Reference to Printer Settings Data; [§17.6.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0594-0598-0112.md#source-page-597-part-1).
- [ ] `sectPrChange` — [w:CT_SectPrChange](sections-and-stories.md#w-ct_sectprchange); Strict, Transitional; minOccurs=0.
- [ ] `@rsidRPr` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidDel` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidR` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidSect` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.

<a id="w-ct_hdrftrref"></a>

## w:CT_HdrFtrRef

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
- [ ] `@type` — [w:ST_HdrFtr](values-word.md#w-st_hdrftr); Strict, Transitional; use=required. Default story only. First and even types have no Mote authoring mapping. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).

<a id="w-ct_hdrftr"></a>

## w:CT_HdrFtr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

Empty marker type; no attributes or child properties.

<a id="w-ct_ftnprops"></a>

## w:CT_FtnProps

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `pos` — [w:CT_FtnPos](sections-and-stories.md#w-ct_ftnpos); Strict, Transitional; minOccurs=0.
- [ ] `numFmt` — [w:CT_NumFmt](sections-and-stories.md#w-ct_numfmt); Strict, Transitional; minOccurs=0. Numbering Format; [§17.9.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0725-0730-0136.md#source-page-728-part-1).
- [ ] `numStart` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `numRestart` — [w:CT_NumRestart](sections-and-stories.md#w-ct_numrestart); Strict, Transitional; minOccurs=0.

<a id="w-ct_ednprops"></a>

## w:CT_EdnProps

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `pos` — [w:CT_EdnPos](sections-and-stories.md#w-ct_ednpos); Strict, Transitional; minOccurs=0.
- [ ] `numFmt` — [w:CT_NumFmt](sections-and-stories.md#w-ct_numfmt); Strict, Transitional; minOccurs=0. Numbering Format; [§17.9.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0725-0730-0136.md#source-page-728-part-1).
- [ ] `numStart` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `numRestart` — [w:CT_NumRestart](sections-and-stories.md#w-ct_numrestart); Strict, Transitional; minOccurs=0.

<a id="w-ct_secttype"></a>

## w:CT_SectType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `@val` — [w:ST_SectionMark](values-word.md#w-st_sectionmark); Strict, Transitional. properties.start: continuous, next_page, next_column, even_page, odd_page. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).

<a id="w-ct_pagesz"></a>

## w:CT_PageSz

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `@w` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional. Document page.width: normalized geometry and single-section DOCX output; omitted orientation uses portrait dimensions. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@h` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional. Document page.height: normalized geometry and single-section DOCX output; omitted orientation uses portrait dimensions. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@orient` — [w:ST_PageOrientation](values-word.md#w-st_pageorientation); Strict, Transitional; use=optional. Document page.orientation: normalized geometry and single-section DOCX output; omitted orientation uses portrait dimensions. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [ ] `@code` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.

<a id="w-ct_pagemar"></a>

## w:CT_PageMar

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@top` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=required. Document margin.top: non-negative lengths and single-section DOCX output. Negative signed margins are unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@right` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=required. Document margin.right: non-negative lengths and single-section DOCX output. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [ ] `@bottom` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=required. Document margin.bottom: non-negative lengths and single-section DOCX output. Negative signed margins are unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@left` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=required. Document margin.left: non-negative lengths and single-section DOCX output. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@header` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=required. Document margin.header: non-negative lengths and single-section DOCX output. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@footer` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=required. Document margin.footer: non-negative lengths and single-section DOCX output. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@gutter` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=required. Document margin.gutter: non-negative lengths and single-section DOCX output. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).

<a id="w-ct_papersource"></a>

## w:CT_PaperSource

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@first` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional.
- [ ] `@other` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional.

<a id="w-ct_pageborders"></a>

## w:CT_PageBorders

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_TopPageBorder](sections-and-stories.md#w-ct_toppageborder); Strict, Transitional; minOccurs=0.
- [ ] `left` — [w:CT_PageBorder](sections-and-stories.md#w-ct_pageborder); Strict, Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_BottomPageBorder](sections-and-stories.md#w-ct_bottompageborder); Strict, Transitional; minOccurs=0.
- [ ] `right` — [w:CT_PageBorder](sections-and-stories.md#w-ct_pageborder); Strict, Transitional; minOccurs=0.
- [ ] `@zOrder` — [w:ST_PageBorderZOrder](values-word.md#w-st_pageborderzorder); Strict, Transitional; use=optional; default=front.
- [ ] `@display` — [w:ST_PageBorderDisplay](values-word.md#w-st_pageborderdisplay); Strict, Transitional; use=optional.
- [ ] `@offsetFrom` — [w:ST_PageBorderOffset](values-word.md#w-st_pageborderoffset); Strict, Transitional; use=optional; default=text.

<a id="w-ct_linenumber"></a>

## w:CT_LineNumber

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@countBy` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@start` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional; default=1.
- [ ] `@distance` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional.
- [ ] `@restart` — [w:ST_LineNumberRestart](values-word.md#w-st_linenumberrestart); Strict, Transitional; use=optional; default=newPage.

<a id="w-ct_pagenumber"></a>

## w:CT_PageNumber

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@fmt` — [w:ST_NumberFormat](values-word.md#w-st_numberformat); Strict, Transitional; use=optional; default=decimal. page_numbering.format retains a format string. DOCX maps decimal, alphabetic and Roman forms; native page fields are decimal. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [x] `@start` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional. page_numbering.start accepts a positive integer; restart without a start selects 1. Evidence: [sections-and-stories.md](../../../docs/specification/sections-and-stories.md), [sections.rs](../../../tests/sections.rs), [section_rendering.rs](../../../tests/section_rendering.rs).
- [ ] `@chapStyle` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@chapSep` — [w:ST_ChapterSep](values-word.md#w-st_chaptersep); Strict, Transitional; use=optional; default=hyphen.

<a id="w-ct_columns"></a>

## w:CT_Columns

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `col` — [w:CT_Column](sections-and-stories.md#w-ct_column); Strict, Transitional; maxOccurs=45. Single Column Definition; [§17.6.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0563-0567-0106.md#source-page-567-part-1).
- [ ] `@equalWidth` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional. columns.equal_width retains explicit booleans; DOCX supports equal-width columns and rejects false. Individual widths remain unsupported. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@space` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional; default=720. Document columns.gap: validated model and equal-width DOCX columns when count is present. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@num` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional; default=1. Document columns.count: validated model and equal-width DOCX columns when count is present. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).
- [x] `@sep` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional. Document columns.separator: validated model and equal-width DOCX columns when count is present. Evidence: [document-configuration.md](../../../docs/specification/document-configuration.md), [configuration.rs](../../../tests/configuration.rs), [docx_configuration.rs](../../../tests/docx_configuration.rs).

<a id="w-ct_docgrid"></a>

## w:CT_DocGrid

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@type` — [w:ST_DocGrid](values-word.md#w-st_docgrid); Strict, Transitional.
- [ ] `@linePitch` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional.
- [ ] `@charSpace` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Transitional.
- [ ] `@charSpace` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict; use=optional; default=0.

<a id="w-ct_rel"></a>

## w:CT_Rel

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="w-ct_sectprchange"></a>

## w:CT_SectPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `sectPr` — [w:CT_SectPrBase](sections-and-stories.md#w-ct_sectprbase); Strict, Transitional; minOccurs=0.

<a id="w-ct_ftnpos"></a>

## w:CT_FtnPos

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_FtnPos](values-word.md#w-st_ftnpos); Strict, Transitional; use=required.

<a id="w-ct_numfmt"></a>

## w:CT_NumFmt

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

See also the [custom-format semantics and additional sequence strings](prose-extensions.md#numbering) defined in specification prose.

- [ ] `@val` — [w:ST_NumberFormat](values-word.md#w-st_numberformat); Strict, Transitional; use=required. Partial: List and heading formats cover a small documented subset; the full enumeration is retained in the linked value definition. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@format` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.

<a id="w-ct_numrestart"></a>

## w:CT_NumRestart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_RestartNumber](values-word.md#w-st_restartnumber); Strict, Transitional; use=required.

<a id="w-ct_ednpos"></a>

## w:CT_EdnPos

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_EdnPos](values-word.md#w-st_ednpos); Strict, Transitional; use=required.

<a id="w-ct_toppageborder"></a>

## w:CT_TopPageBorder

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Border](values-word.md#w-st_border); Strict, Transitional; use=required.
- [ ] `@color` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=optional; default=auto.
- [ ] `@themeColor` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@sz` — [w:ST_EighthPointMeasure](values-word.md#w-st_eighthpointmeasure); Strict, Transitional; use=optional.
- [ ] `@space` — [w:ST_PointMeasure](values-word.md#w-st_pointmeasure); Strict, Transitional; use=optional; default=0.
- [ ] `@shadow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@frame` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.
- [ ] `@r:topLeft` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.
- [ ] `@r:topRight` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.

<a id="w-ct_pageborder"></a>

## w:CT_PageBorder

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Border](values-word.md#w-st_border); Strict, Transitional; use=required.
- [ ] `@color` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=optional; default=auto.
- [ ] `@themeColor` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@sz` — [w:ST_EighthPointMeasure](values-word.md#w-st_eighthpointmeasure); Strict, Transitional; use=optional.
- [ ] `@space` — [w:ST_PointMeasure](values-word.md#w-st_pointmeasure); Strict, Transitional; use=optional; default=0.
- [ ] `@shadow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@frame` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.

<a id="w-ct_bottompageborder"></a>

## w:CT_BottomPageBorder

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Border](values-word.md#w-st_border); Strict, Transitional; use=required.
- [ ] `@color` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=optional; default=auto.
- [ ] `@themeColor` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@sz` — [w:ST_EighthPointMeasure](values-word.md#w-st_eighthpointmeasure); Strict, Transitional; use=optional.
- [ ] `@space` — [w:ST_PointMeasure](values-word.md#w-st_pointmeasure); Strict, Transitional; use=optional; default=0.
- [ ] `@shadow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@frame` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.
- [ ] `@r:bottomLeft` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.
- [ ] `@r:bottomRight` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.

<a id="w-ct_column"></a>

## w:CT_Column

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@w` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional.
- [ ] `@space` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional; default=0.

<a id="w-ct_sectprbase"></a>

## w:CT_SectPrBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `footnotePr` — [w:CT_FtnProps](sections-and-stories.md#w-ct_ftnprops); Strict, Transitional; minOccurs=0.
- [ ] `endnotePr` — [w:CT_EdnProps](sections-and-stories.md#w-ct_ednprops); Strict, Transitional; minOccurs=0.
- [ ] `type` — [w:CT_SectType](sections-and-stories.md#w-ct_secttype); Strict, Transitional; minOccurs=0. Section Type; [§17.6.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0618-0622-0117.md#source-page-621-part-1).
- [ ] `pgSz` — [w:CT_PageSz](sections-and-stories.md#w-ct_pagesz); Strict, Transitional; minOccurs=0. Page Size; [§17.6.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0594-0598-0112.md#source-page-595-part-1).
- [ ] `pgMar` — [w:CT_PageMar](sections-and-stories.md#w-ct_pagemar); Strict, Transitional; minOccurs=0. Page Margins; [§17.6.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0585-0589-0110.md#source-page-589-part-1).
- [ ] `paperSrc` — [w:CT_PaperSource](sections-and-stories.md#w-ct_papersource); Strict, Transitional; minOccurs=0. Paper Source Information; [§17.6.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0585-0589-0110.md#source-page-585-part-1).
- [ ] `pgBorders` — [w:CT_PageBorders](sections-and-stories.md#w-ct_pageborders); Strict, Transitional; minOccurs=0. Page Borders; [§17.6.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0585-0589-0110.md#source-page-586-part-1).
- [ ] `lnNumType` — [w:CT_LineNumber](sections-and-stories.md#w-ct_linenumber); Strict, Transitional; minOccurs=0. Line Numbering Settings; [§17.6.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0580-0584-0109.md#source-page-583-part-1).
- [ ] `pgNumType` — [w:CT_PageNumber](sections-and-stories.md#w-ct_pagenumber); Strict, Transitional; minOccurs=0. Page Numbering Settings; [§17.6.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0590-0593-0111.md#source-page-593-part-1).
- [ ] `cols` — [w:CT_Columns](sections-and-stories.md#w-ct_columns); Strict, Transitional; minOccurs=0. Column Definitions; [§17.6.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0568-0574-0107.md#source-page-568-part-1).
- [ ] `formProt` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Only Allow Editing of Form Fields; [§17.6.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0575-0579-0108.md#source-page-575-part-1).
- [ ] `vAlign` — [w:CT_VerticalJc](tables.md#w-ct_verticaljc); Strict, Transitional; minOccurs=0.
- [ ] `noEndnote` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `titlePg` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0.
- [ ] `bidi` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `rtlGutter` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Gutter on Right Side of Page; [§17.6.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0603-0607-0114.md#source-page-605-part-1).
- [ ] `docGrid` — [w:CT_DocGrid](sections-and-stories.md#w-ct_docgrid); Strict, Transitional; minOccurs=0. Document Grid; [§17.6.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0568-0574-0107.md#source-page-570-part-1).
- [ ] `printerSettings` — [w:CT_Rel](sections-and-stories.md#w-ct_rel); Strict, Transitional; minOccurs=0. Reference to Printer Settings Data; [§17.6.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0594-0598-0112.md#source-page-597-part-1).
- [ ] `@rsidRPr` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidDel` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidR` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidSect` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
