# Properties and values defined in prose

[Inventory guide](README.md)

These entries supplement the schema inventories with definitions from the
retained MS-DOCX specification. All remain unsupported. They introduce no Mote
attribute names. This page is maintained by reviewing the linked clauses;
schema regeneration cannot discover these additional meanings on its own.

<a id="numbering"></a>

## Custom numbering formats

- [ ] Word `numFmt/@format`: custom number-format syntax and fallback to `@val`.
  The attribute is inventoried under [w:CT_NumFmt](word-paragraphs.md#w-ct_numfmt).
  Its string type alone does not describe the formatting behaviour. See
  [ECMA-376 §17.9.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0725-0730-0136.md#source-page-728-part-1)
  and its attribute definition on the following page.
- [ ] MS-DOCX predefined sequence-string aliases for the standard numbering
  formats. The complete correspondence table occupies
  [pages 21–22](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0019-0022-0008.md#source-page-21-part-1)
  and [page 23](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0023-0025-0009.md#source-page-23-part-1).
  These strings select formats from the existing
  [number-format enumeration](values-word.md#w-st_numberformat); they do not
  enlarge Mote's supported format set.

MS-DOCX §2.4 additionally defines these nine sequence strings. The source clauses
specify the character sets, rollover rules, and padding behaviour; the strings
below retain the publisher's code-point notation, including its spelling.

- [ ] `U+0041, U+00C7, U+011C, ...`: uppercase Turkish.
  [Page 23](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0023-0025-0009.md#source-page-23-part-1).
- [ ] `U+0061, U+00E7, U+011D, ...`: lowercase Turkish.
  [Page 24](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0023-0025-0009.md#source-page-24-part-1).
- [ ] `U+0410, U+0419, U+041A, ...`: uppercase Bulgarian.
  [Pages 24–25](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0023-0025-0009.md#source-page-24-part-1).
- [ ] `U+0430, U+0439, U+043A, ...`: lowercase Bulgarian.
  [Page 25](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0023-0025-0009.md#source-page-25-part-1).
- [ ] `U+0391, U+0392, U+0393, ...`: uppercase Greek.
  [Page 25](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0023-0025-0009.md#source-page-25-part-1)
  and [continuation](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0026-0029-0010.md#source-page-26-part-1).
- [ ] `U+03B1, U+03B2, U+03B3, ...`: lowercase Greek.
  [Page 26](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0026-0029-0010.md#source-page-26-part-1).
- [ ] `001, 002, 003, ...`: three-position decimal padding.
  [Pages 26–27](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0026-0029-0010.md#source-page-26-part-1).
- [ ] `0001, 0002, 0003, ...`: four-position decimal padding.
  [Page 27](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0026-0029-0010.md#source-page-27-part-1).
- [ ] `00001, 00002, 00003, ...`: five-position decimal padding.
  [Page 27](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0026-0029-0010.md#source-page-27-part-1).

<a id="calendar"></a>

## Calendar values

- [ ] `umalqura`: Um Al-Qura lunar calendar, extending
  [s:ST_CalendarType](values-shared.md#s-st_calendartype), including date controls.
  MS-DOCX requires an `AlternateContent` fallback to `hijri` for compatibility.
  [MS-DOCX §2.2.7](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0015-0018-0007.md#source-page-16-part-1).

<a id="compatibility"></a>

## Related document context

MS-DOCX §2.3 defines these nine `compatSetting` names. They influence text, table,
or style behaviour at document level and are not direct paragraph/span attributes.
Each uses `@name`, `@uri=http://schemas.microsoft.com/office/word`, and `@val`.
The value is `ST_OnOff` except for `compatibilityMode`.

- [ ] `overrideTableStyleFontSizeAndJustification`: style precedence for font size
  and paragraph alignment in tables.
  [§2.3.1](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0015-0018-0007.md#source-page-17-part-1).
- [ ] `doNotFlipMirrorIndents`: inside/outside paragraph indent treatment.
  [§2.3.2](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0015-0018-0007.md#source-page-17-part-1).
- [ ] `enableOpenTypeFeatures`: document-level typography feature selection.
  [§2.3.3](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0015-0018-0007.md#source-page-18-part-1).
- [ ] `differentiateMultirowTableHeaders`: conditional styling of multiple header rows.
  [§2.3.4](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0015-0018-0007.md#source-page-18-part-1).
- [ ] `compatibilityMode`: document feature set, with values `11`, `12`, `14`, `15`
  represented as `ST_UnsignedDecimalNumber`.
  [§2.3.5](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0019-0022-0008.md#source-page-19-part-1).
- [ ] `allowTextAfterFloatingTableBreak`: text flow beside a table spanning pages.
  [§2.3.6](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0019-0022-0008.md#source-page-19-part-1).
- [ ] `allowHyphenationAtTrackBottom`: hyphenation at a page or column boundary.
  [§2.3.7](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0019-0022-0008.md#source-page-20-part-1).
- [ ] `useWord2013TrackBottomHyphenation`: alternate boundary-hyphenation behaviour.
  [§2.3.8](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0019-0022-0008.md#source-page-20-part-1).
- [ ] `usePre2018iOSMacLayout`: platform font-metric compatibility.
  [§2.3.9](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0019-0022-0008.md#source-page-21-part-1).
