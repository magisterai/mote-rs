# Bookmarks, fields and captions

[Inventory guide](README.md)

Bookmark range identifiers, names and column metadata; field instructions, locking and dirty state; caption sequence settings. Inline/block content properties are inventoried in their respective families.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_Bookmark](#w-ct_bookmark)
- [w:CT_MarkupRange](#w-ct_markuprange)
- [w:CT_SimpleField](#w-ct_simplefield)
- [w:CT_FldChar](#w-ct_fldchar)
- [w:CT_Captions](#w-ct_captions)
- [w:CT_TrackChangeNumbering](#w-ct_trackchangenumbering)
- [w:CT_Caption](#w-ct_caption)
- [w:CT_AutoCaptions](#w-ct_autocaptions)
- [w:CT_AutoCaption](#w-ct_autocaption)

<a id="w-ct_bookmark"></a>

## w:CT_Bookmark

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@displacedByCustomXml` — [w:ST_DisplacedByCustomXml](values-word.md#w-st_displacedbycustomxml); Strict, Transitional; use=optional.
- [ ] `@colFirst` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@colLast` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@name` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.

<a id="w-ct_markuprange"></a>

## w:CT_MarkupRange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@displacedByCustomXml` — [w:ST_DisplacedByCustomXml](values-word.md#w-st_displacedbycustomxml); Strict, Transitional; use=optional.

<a id="w-ct_simplefield"></a>

## w:CT_SimpleField

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@instr` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@fldLock` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@dirty` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.

<a id="w-ct_fldchar"></a>

## w:CT_FldChar

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `fldData` — [w:CT_Text](word-runs.md#w-ct_text); Transitional; minOccurs=0; maxOccurs=1.
- [ ] `ffData` — [w:CT_FFData](controls.md#w-ct_ffdata); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `numberingChange` — [w:CT_TrackChangeNumbering](anchors-and-fields.md#w-ct_trackchangenumbering); Transitional; minOccurs=0.
- [ ] `@fldCharType` — [w:ST_FldCharType](values-word.md#w-st_fldchartype); Strict, Transitional; use=required.
- [ ] `@fldLock` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@dirty` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.

<a id="w-ct_captions"></a>

## w:CT_Captions

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `caption` — [w:CT_Caption](anchors-and-fields.md#w-ct_caption); Strict, Transitional; minOccurs=1; maxOccurs=unbounded.
- [ ] `autoCaptions` — [w:CT_AutoCaptions](anchors-and-fields.md#w-ct_autocaptions); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="w-ct_trackchangenumbering"></a>

## w:CT_TrackChangeNumbering

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Transitional; use=optional.
- [ ] `@original` — [s:ST_String](values-shared.md#s-st_string); Transitional; use=optional.

<a id="w-ct_caption"></a>

## w:CT_Caption

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@name` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@pos` — [w:ST_CaptionPos](values-word.md#w-st_captionpos); Strict, Transitional; use=optional.
- [ ] `@chapNum` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@heading` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@noLabel` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@numFmt` — [w:ST_NumberFormat](values-word.md#w-st_numberformat); Strict, Transitional; use=optional.
- [ ] `@sep` — [w:ST_ChapterSep](values-word.md#w-st_chaptersep); Strict, Transitional; use=optional.

<a id="w-ct_autocaptions"></a>

## w:CT_AutoCaptions

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `autoCaption` — [w:CT_AutoCaption](anchors-and-fields.md#w-ct_autocaption); Strict, Transitional; minOccurs=1; maxOccurs=unbounded.

<a id="w-ct_autocaption"></a>

## w:CT_AutoCaption

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@name` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@caption` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
