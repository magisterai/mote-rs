# Item control properties

[Inventory guide](README.md)

Used by task-item checkbox metadata. Includes legacy form fields and structured-document-tag properties, alternative control kinds, and their nested fields.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_SdtPr](#w-ct_sdtpr)
- [w:CT_SdtEndPr](#w-ct_sdtendpr)
- [w:CT_FFData](#w-ct_ffdata)
- [w:CT_Lock](#w-ct_lock)
- [w:CT_Placeholder](#w-ct_placeholder)
- [w:CT_DataBinding](#w-ct_databinding)
- [w:CT_UnsignedDecimalNumber](#w-ct_unsigneddecimalnumber)
- [w:CT_Empty](#w-ct_empty)
- [w:CT_SdtComboBox](#w-ct_sdtcombobox)
- [w:CT_SdtDate](#w-ct_sdtdate)
- [w:CT_SdtDocPart](#w-ct_sdtdocpart)
- [w:CT_SdtDropDownList](#w-ct_sdtdropdownlist)
- [w:CT_SdtText](#w-ct_sdttext)
- [w:CT_FFName](#w-ct_ffname)
- [w:CT_MacroName](#w-ct_macroname)
- [w:CT_FFHelpText](#w-ct_ffhelptext)
- [w:CT_FFStatusText](#w-ct_ffstatustext)
- [w:CT_FFCheckBox](#w-ct_ffcheckbox)
- [w:CT_FFDDList](#w-ct_ffddlist)
- [w:CT_FFTextInput](#w-ct_fftextinput)
- [w:CT_SdtListItem](#w-ct_sdtlistitem)
- [w:CT_Lang](#w-ct_lang)
- [w:CT_SdtDateMappingType](#w-ct_sdtdatemappingtype)
- [w:CT_CalendarType](#w-ct_calendartype)
- [w:CT_FFTextType](#w-ct_fftexttype)

<a id="w-ct_sdtpr"></a>

## w:CT_SdtPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0.
- [x] `alias` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. `checkbox.title` supplies the alias of a task content control; absent/empty titles omit the alias. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_controls.rs](../../../tests/docx_controls.rs).
- [x] `tag` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. `checkbox.name` supplies the tag of a task content control; absent/empty names omit the tag. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_controls.rs](../../../tests/docx_controls.rs).
- [ ] `id` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `lock` — [w:CT_Lock](controls.md#w-ct_lock); Strict, Transitional; minOccurs=0.
- [ ] `placeholder` — [w:CT_Placeholder](controls.md#w-ct_placeholder); Strict, Transitional; minOccurs=0.
- [ ] `temporary` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `showingPlcHdr` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `dataBinding` — [w:CT_DataBinding](controls.md#w-ct_databinding); Strict, Transitional; minOccurs=0.
- [ ] `label` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `tabIndex` — [w:CT_UnsignedDecimalNumber](controls.md#w-ct_unsigneddecimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `equation` — [w:CT_Empty](controls.md#w-ct_empty); Strict, Transitional.
- [ ] `comboBox` — [w:CT_SdtComboBox](controls.md#w-ct_sdtcombobox); Strict, Transitional.
- [ ] `date` — [w:CT_SdtDate](controls.md#w-ct_sdtdate); Strict, Transitional.
- [ ] `docPartObj` — [w:CT_SdtDocPart](controls.md#w-ct_sdtdocpart); Strict, Transitional.
- [ ] `docPartList` — [w:CT_SdtDocPart](controls.md#w-ct_sdtdocpart); Strict, Transitional.
- [ ] `dropDownList` — [w:CT_SdtDropDownList](controls.md#w-ct_sdtdropdownlist); Strict, Transitional.
- [ ] `picture` — [w:CT_Empty](controls.md#w-ct_empty); Strict, Transitional.
- [ ] `richText` — [w:CT_Empty](controls.md#w-ct_empty); Strict, Transitional.
- [ ] `text` — [w:CT_SdtText](controls.md#w-ct_sdttext); Strict, Transitional.
- [ ] `citation` — [w:CT_Empty](controls.md#w-ct_empty); Strict, Transitional.
- [ ] `group` — [w:CT_Empty](controls.md#w-ct_empty); Strict, Transitional.
- [ ] `bibliography` — [w:CT_Empty](controls.md#w-ct_empty); Strict, Transitional.

<a id="w-ct_sdtendpr"></a>

## w:CT_SdtEndPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0.

<a id="w-ct_ffdata"></a>

## w:CT_FFData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `name` — [w:CT_FFName](controls.md#w-ct_ffname); Strict, Transitional. `checkbox.name` supplies a legacy task-field name; omission uses a unique generated name. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_controls.rs](../../../tests/docx_controls.rs).
- [ ] `label` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `tabIndex` — [w:CT_UnsignedDecimalNumber](controls.md#w-ct_unsigneddecimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `enabled` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional.
- [ ] `calcOnExit` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional.
- [ ] `entryMacro` — [w:CT_MacroName](controls.md#w-ct_macroname); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `exitMacro` — [w:CT_MacroName](controls.md#w-ct_macroname); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `helpText` — [w:CT_FFHelpText](controls.md#w-ct_ffhelptext); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `statusText` — [w:CT_FFStatusText](controls.md#w-ct_ffstatustext); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `checkBox` — [w:CT_FFCheckBox](controls.md#w-ct_ffcheckbox); Strict, Transitional. Partial: `checkbox.kind=legacy_form_field` emits task fields with automatic size and matching default/checked state; independent size/default-state controls are not exposed. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_controls.rs](../../../tests/docx_controls.rs).
- [ ] `ddList` — [w:CT_FFDDList](controls.md#w-ct_ffddlist); Strict, Transitional.
- [ ] `textInput` — [w:CT_FFTextInput](controls.md#w-ct_fftextinput); Strict, Transitional.

<a id="w-ct_lock"></a>

## w:CT_Lock

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Lock](values-word.md#w-st_lock); Strict, Transitional.

<a id="w-ct_placeholder"></a>

## w:CT_Placeholder

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `docPart` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional.

<a id="w-ct_databinding"></a>

## w:CT_DataBinding

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@prefixMappings` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional.
- [ ] `@xpath` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@storeItemID` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.

<a id="w-ct_unsigneddecimalnumber"></a>

## w:CT_UnsignedDecimalNumber

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_UnsignedDecimalNumber](values-shared.md#s-st_unsigneddecimalnumber); Strict, Transitional; use=required.

<a id="w-ct_empty"></a>

## w:CT_Empty

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Empty marker type; no attributes or child properties.

<a id="w-ct_sdtcombobox"></a>

## w:CT_SdtComboBox

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `listItem` — [w:CT_SdtListItem](controls.md#w-ct_sdtlistitem); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `@lastValue` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional; default=.

<a id="w-ct_sdtdate"></a>

## w:CT_SdtDate

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `dateFormat` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0.
- [ ] `lid` — [w:CT_Lang](controls.md#w-ct_lang); Strict, Transitional; minOccurs=0. Language ID for Phonetic Guide; [§17.3.3.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0345-0350-0070.md#source-page-346-part-1).
- [ ] `storeMappedDataAs` — [w:CT_SdtDateMappingType](controls.md#w-ct_sdtdatemappingtype); Strict, Transitional; minOccurs=0.
- [ ] `calendar` — [w:CT_CalendarType](controls.md#w-ct_calendartype); Strict, Transitional; minOccurs=0.
- [ ] `@fullDate` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.

<a id="w-ct_sdtdocpart"></a>

## w:CT_SdtDocPart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `docPartGallery` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0.
- [ ] `docPartCategory` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0.
- [ ] `docPartUnique` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.

<a id="w-ct_sdtdropdownlist"></a>

## w:CT_SdtDropDownList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `listItem` — [w:CT_SdtListItem](controls.md#w-ct_sdtlistitem); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `@lastValue` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional; default=.

<a id="w-ct_sdttext"></a>

## w:CT_SdtText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@multiLine` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.

<a id="w-ct_ffname"></a>

## w:CT_FFName

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_FFName](values-word.md#w-st_ffname); Strict, Transitional.

<a id="w-ct_macroname"></a>

## w:CT_MacroName

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_MacroName](values-word.md#w-st_macroname); Strict, Transitional; use=required.

<a id="w-ct_ffhelptext"></a>

## w:CT_FFHelpText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@type` — [w:ST_InfoTextType](values-word.md#w-st_infotexttype); Strict, Transitional.
- [ ] `@val` — [w:ST_FFHelpTextVal](values-word.md#w-st_ffhelptextval); Strict, Transitional.

<a id="w-ct_ffstatustext"></a>

## w:CT_FFStatusText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@type` — [w:ST_InfoTextType](values-word.md#w-st_infotexttype); Strict, Transitional.
- [ ] `@val` — [w:ST_FFStatusTextVal](values-word.md#w-st_ffstatustextval); Strict, Transitional.

<a id="w-ct_ffcheckbox"></a>

## w:CT_FFCheckBox

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `size` — [w:CT_HpsMeasure](word-runs.md#w-ct_hpsmeasure); Strict, Transitional.
- [ ] `sizeAuto` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional.
- [ ] `default` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [x] `checked` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. `item.checked` supplies legacy task-field state, overriding nested checkbox state; absence of item state leaves ordinary list numbering. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_controls.rs](../../../tests/docx_controls.rs).

<a id="w-ct_ffddlist"></a>

## w:CT_FFDDList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `result` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `default` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `listEntry` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="w-ct_fftextinput"></a>

## w:CT_FFTextInput

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `type` — [w:CT_FFTextType](controls.md#w-ct_fftexttype); Strict, Transitional; minOccurs=0. Section Type; [§17.6.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0618-0622-0117.md#source-page-621-part-1).
- [ ] `default` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0.
- [ ] `maxLength` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `format` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0.

<a id="w-ct_sdtlistitem"></a>

## w:CT_SdtListItem

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@displayText` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional.
- [ ] `@value` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional.

<a id="w-ct_lang"></a>

## w:CT_Lang

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_Lang](values-shared.md#s-st_lang); Strict, Transitional; use=required.

<a id="w-ct_sdtdatemappingtype"></a>

## w:CT_SdtDateMappingType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_SdtDateMappingType](values-word.md#w-st_sdtdatemappingtype); Strict, Transitional.

<a id="w-ct_calendartype"></a>

## w:CT_CalendarType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_CalendarType](values-shared.md#s-st_calendartype); Strict, Transitional.

<a id="w-ct_fftexttype"></a>

## w:CT_FFTextType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_FFTextType](values-word.md#w-st_fftexttype); Strict, Transitional; use=required.
