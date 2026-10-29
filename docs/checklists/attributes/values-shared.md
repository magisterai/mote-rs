# Shared value definitions

[Inventory guide](README.md)

These are complete schema value domains for the selected properties.
Property rows record Mote support and supported subsets separately.
Identical Strict and Transitional definitions are combined.
See also [prose-defined extensions](prose-extensions.md).

## Types

- [r:ST_RelationshipId](#r-st_relationshipid)
- [s:ST_CalendarType](#s-st_calendartype)
- [s:ST_FixedPercentage](#s-st_fixedpercentage)
- [s:ST_Guid](#s-st_guid)
- [s:ST_HexColorRGB](#s-st_hexcolorrgb)
- [s:ST_Lang](#s-st_lang)
- [s:ST_OnOff](#s-st_onoff)
- [s:ST_OnOff1](#s-st_onoff1)
- [s:ST_Panose](#s-st_panose)
- [s:ST_Percentage](#s-st_percentage)
- [s:ST_PositiveFixedPercentage](#s-st_positivefixedpercentage)
- [s:ST_PositivePercentage](#s-st_positivepercentage)
- [s:ST_PositiveUniversalMeasure](#s-st_positiveuniversalmeasure)
- [s:ST_String](#s-st_string)
- [s:ST_TwipsMeasure](#s-st_twipsmeasure)
- [s:ST_UniversalMeasure](#s-st_universalmeasure)
- [s:ST_UnsignedDecimalNumber](#s-st_unsigneddecimalnumber)
- [s:ST_VerticalAlignRun](#s-st_verticalalignrun)
- [s:ST_XAlign](#s-st_xalign)
- [s:ST_YAlign](#s-st_yalign)

<a id="r-st_relationshipid"></a>

## r:ST_RelationshipId

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-relationshipReference.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-relationshipReference.xsd):

- restriction: base: `xsd:string`

<a id="s-st_calendartype"></a>

## s:ST_CalendarType

MS-DOCX adds the [umalqura calendar value](prose-extensions.md#calendar) outside this schema enumeration.

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `gregorian`
- enumeration: value: `gregorianUs`
- enumeration: value: `gregorianMeFrench`
- enumeration: value: `gregorianArabic`
- enumeration: value: `hijri`
- enumeration: value: `hebrew`
- enumeration: value: `taiwan`
- enumeration: value: `japan`
- enumeration: value: `thai`
- enumeration: value: `korea`
- enumeration: value: `saka`
- enumeration: value: `gregorianXlitEnglish`
- enumeration: value: `gregorianXlitFrench`
- enumeration: value: `none`

<a id="s-st_fixedpercentage"></a>

## s:ST_FixedPercentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: [s:ST_Percentage](values-shared.md#s-st_percentage)
- pattern: value: `-?((100)|([0-9][0-9]?))(\.[0-9][0-9]?)?%`

<a id="s-st_guid"></a>

## s:ST_Guid

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:token`
- pattern: value: `\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\}`

<a id="s-st_hexcolorrgb"></a>

## s:ST_HexColorRGB

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:hexBinary`
- length: value: `3`; fixed: `true`

<a id="s-st_lang"></a>

## s:ST_Lang

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`

<a id="s-st_onoff"></a>

## s:ST_OnOff

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd):

- union: memberTypes: `xsd:boolean`, [s:ST_OnOff1](values-shared.md#s-st_onoff1)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- union: memberTypes: `xsd:boolean`

<a id="s-st_onoff1"></a>

## s:ST_OnOff1

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `on`
- enumeration: value: `off`

<a id="s-st_panose"></a>

## s:ST_Panose

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:hexBinary`
- length: value: `10`

<a id="s-st_percentage"></a>

## s:ST_Percentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`
- pattern: value: `-?[0-9]+(\.[0-9]+)?%`

<a id="s-st_positivefixedpercentage"></a>

## s:ST_PositiveFixedPercentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: [s:ST_Percentage](values-shared.md#s-st_percentage)
- pattern: value: `((100)|([0-9][0-9]?))(\.[0-9][0-9]?)?%`

<a id="s-st_positivepercentage"></a>

## s:ST_PositivePercentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: [s:ST_Percentage](values-shared.md#s-st_percentage)
- pattern: value: `[0-9]+(\.[0-9]+)?%`

<a id="s-st_positiveuniversalmeasure"></a>

## s:ST_PositiveUniversalMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: [s:ST_UniversalMeasure](values-shared.md#s-st_universalmeasure)
- pattern: value: `[0-9]+(\.[0-9]+)?(mm|cm|in|pt|pc|pi)`

<a id="s-st_string"></a>

## s:ST_String

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`

<a id="s-st_twipsmeasure"></a>

## s:ST_TwipsMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- union: memberTypes: [s:ST_UnsignedDecimalNumber](values-shared.md#s-st_unsigneddecimalnumber), [s:ST_PositiveUniversalMeasure](values-shared.md#s-st_positiveuniversalmeasure)

<a id="s-st_universalmeasure"></a>

## s:ST_UniversalMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`
- pattern: value: `-?[0-9]+(\.[0-9]+)?(mm|cm|in|pt|pc|pi)`

<a id="s-st_unsigneddecimalnumber"></a>

## s:ST_UnsignedDecimalNumber

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:unsignedLong`

<a id="s-st_verticalalignrun"></a>

## s:ST_VerticalAlignRun

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `baseline`
- enumeration: value: `superscript`
- enumeration: value: `subscript`

<a id="s-st_xalign"></a>

## s:ST_XAlign

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `left`
- enumeration: value: `center`
- enumeration: value: `right`
- enumeration: value: `inside`
- enumeration: value: `outside`

<a id="s-st_yalign"></a>

## s:ST_YAlign

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `inline`
- enumeration: value: `top`
- enumeration: value: `center`
- enumeration: value: `bottom`
- enumeration: value: `inside`
- enumeration: value: `outside`
