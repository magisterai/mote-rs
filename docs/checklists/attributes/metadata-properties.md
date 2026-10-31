# Extended and custom document properties

[Inventory guide](README.md)

Every extended property, custom-property field and nested variant value in the retained Strict and Transitional schemas. Retaining metadata in the Mote model does not imply package mappings for these properties. Core properties are inventoried separately in metadata.md.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [ep:CT_Properties](#ep-ct_properties)
- [custom:CT_Properties](#custom-ct_properties)
- [ep:CT_VectorVariant](#ep-ct_vectorvariant)
- [ep:CT_VectorLpstr](#ep-ct_vectorlpstr)
- [ep:CT_DigSigBlob](#ep-ct_digsigblob)
- [custom:CT_Property](#custom-ct_property)
- [vt:CT_Vector](#vt-ct_vector)
- [vt:CT_Array](#vt-ct_array)
- [vt:CT_Empty](#vt-ct_empty)
- [vt:CT_Null](#vt-ct_null)
- [vt:CT_Vstream](#vt-ct_vstream)
- [vt:CT_Variant](#vt-ct_variant)

<a id="ep-ct_properties"></a>

## ep:CT_Properties

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesExtended.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesExtended.xsd).

- [ ] `Template` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Manager` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Company` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Pages` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Words` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Characters` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `PresentationFormat` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Lines` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Paragraphs` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Slides` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Notes` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `TotalTime` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `HiddenSlides` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `MMClips` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `ScaleCrop` — `xsd:boolean`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `HeadingPairs` — [ep:CT_VectorVariant](metadata-properties.md#ep-ct_vectorvariant); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `TitlesOfParts` — [ep:CT_VectorLpstr](metadata-properties.md#ep-ct_vectorlpstr); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `LinksUpToDate` — `xsd:boolean`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `CharactersWithSpaces` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `SharedDoc` — `xsd:boolean`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `HyperlinkBase` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `HLinks` — [ep:CT_VectorVariant](metadata-properties.md#ep-ct_vectorvariant); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `HyperlinksChanged` — `xsd:boolean`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `DigSig` — [ep:CT_DigSigBlob](metadata-properties.md#ep-ct_digsigblob); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `Application` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `AppVersion` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `DocSecurity` — `xsd:int`; Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="custom-ct_properties"></a>

## custom:CT_Properties

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesCustom.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesCustom.xsd).

- [ ] `property` — [custom:CT_Property](metadata-properties.md#custom-ct_property); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="ep-ct_vectorvariant"></a>

## ep:CT_VectorVariant

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesExtended.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesExtended.xsd).

- [ ] `vt:vector` — [vt:CT_Vector](metadata-properties.md#vt-ct_vector); Strict, Transitional.

<a id="ep-ct_vectorlpstr"></a>

## ep:CT_VectorLpstr

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesExtended.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesExtended.xsd).

- [ ] `vt:vector` — [vt:CT_Vector](metadata-properties.md#vt-ct_vector); Strict, Transitional.

<a id="ep-ct_digsigblob"></a>

## ep:CT_DigSigBlob

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesExtended.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesExtended.xsd).

- [ ] `vt:blob` — `xsd:base64Binary`; Strict, Transitional.

<a id="custom-ct_property"></a>

## custom:CT_Property

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesCustom.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesCustom.xsd).

- [ ] `vt:vector` — [vt:CT_Vector](metadata-properties.md#vt-ct_vector); Strict, Transitional.
- [ ] `vt:array` — [vt:CT_Array](metadata-properties.md#vt-ct_array); Strict, Transitional.
- [ ] `vt:blob` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:oblob` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:empty` — [vt:CT_Empty](metadata-properties.md#vt-ct_empty); Strict, Transitional.
- [ ] `vt:null` — [vt:CT_Null](metadata-properties.md#vt-ct_null); Strict, Transitional.
- [ ] `vt:i1` — `xsd:byte`; Strict, Transitional.
- [ ] `vt:i2` — `xsd:short`; Strict, Transitional.
- [ ] `vt:i4` — `xsd:int`; Strict, Transitional.
- [ ] `vt:i8` — `xsd:long`; Strict, Transitional.
- [ ] `vt:int` — `xsd:int`; Strict, Transitional.
- [ ] `vt:ui1` — `xsd:unsignedByte`; Strict, Transitional.
- [ ] `vt:ui2` — `xsd:unsignedShort`; Strict, Transitional.
- [ ] `vt:ui4` — `xsd:unsignedInt`; Strict, Transitional.
- [ ] `vt:ui8` — `xsd:unsignedLong`; Strict, Transitional.
- [ ] `vt:uint` — `xsd:unsignedInt`; Strict, Transitional.
- [ ] `vt:r4` — `xsd:float`; Strict, Transitional.
- [ ] `vt:r8` — `xsd:double`; Strict, Transitional.
- [ ] `vt:decimal` — `xsd:decimal`; Strict, Transitional.
- [ ] `vt:lpstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:lpwstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:bstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:date` — `xsd:dateTime`; Strict, Transitional.
- [ ] `vt:filetime` — `xsd:dateTime`; Strict, Transitional.
- [ ] `vt:bool` — `xsd:boolean`; Strict, Transitional.
- [ ] `vt:cy` — [vt:ST_Cy](values-extensions.md#vt-st_cy); Strict, Transitional.
- [ ] `vt:error` — [vt:ST_Error](values-extensions.md#vt-st_error); Strict, Transitional.
- [ ] `vt:stream` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:ostream` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:storage` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:ostorage` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:vstream` — [vt:CT_Vstream](metadata-properties.md#vt-ct_vstream); Strict, Transitional.
- [ ] `vt:clsid` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional.
- [ ] `@fmtid` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional; use=required.
- [ ] `@pid` — `xsd:int`; Strict, Transitional; use=required.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@linkTarget` — `xsd:string`; Strict, Transitional; use=optional.

<a id="vt-ct_vector"></a>

## vt:CT_Vector

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesVariantTypes.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesVariantTypes.xsd).

- [ ] `vt:variant` — [vt:CT_Variant](metadata-properties.md#vt-ct_variant); Strict, Transitional.
- [ ] `vt:i1` — `xsd:byte`; Strict, Transitional.
- [ ] `vt:i2` — `xsd:short`; Strict, Transitional.
- [ ] `vt:i4` — `xsd:int`; Strict, Transitional.
- [ ] `vt:i8` — `xsd:long`; Strict, Transitional.
- [ ] `vt:ui1` — `xsd:unsignedByte`; Strict, Transitional.
- [ ] `vt:ui2` — `xsd:unsignedShort`; Strict, Transitional.
- [ ] `vt:ui4` — `xsd:unsignedInt`; Strict, Transitional.
- [ ] `vt:ui8` — `xsd:unsignedLong`; Strict, Transitional.
- [ ] `vt:r4` — `xsd:float`; Strict, Transitional.
- [ ] `vt:r8` — `xsd:double`; Strict, Transitional.
- [ ] `vt:lpstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:lpwstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:bstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:date` — `xsd:dateTime`; Strict, Transitional.
- [ ] `vt:filetime` — `xsd:dateTime`; Strict, Transitional.
- [ ] `vt:bool` — `xsd:boolean`; Strict, Transitional.
- [ ] `vt:cy` — [vt:ST_Cy](values-extensions.md#vt-st_cy); Strict, Transitional.
- [ ] `vt:error` — [vt:ST_Error](values-extensions.md#vt-st_error); Strict, Transitional.
- [ ] `vt:clsid` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional.
- [ ] `@baseType` — [vt:ST_VectorBaseType](values-extensions.md#vt-st_vectorbasetype); Strict, Transitional; use=required.
- [ ] `@size` — `xsd:unsignedInt`; Strict, Transitional; use=required.

<a id="vt-ct_array"></a>

## vt:CT_Array

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesVariantTypes.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesVariantTypes.xsd).

- [ ] `vt:variant` — [vt:CT_Variant](metadata-properties.md#vt-ct_variant); Strict, Transitional.
- [ ] `vt:i1` — `xsd:byte`; Strict, Transitional.
- [ ] `vt:i2` — `xsd:short`; Strict, Transitional.
- [ ] `vt:i4` — `xsd:int`; Strict, Transitional.
- [ ] `vt:int` — `xsd:int`; Strict, Transitional.
- [ ] `vt:ui1` — `xsd:unsignedByte`; Strict, Transitional.
- [ ] `vt:ui2` — `xsd:unsignedShort`; Strict, Transitional.
- [ ] `vt:ui4` — `xsd:unsignedInt`; Strict, Transitional.
- [ ] `vt:uint` — `xsd:unsignedInt`; Strict, Transitional.
- [ ] `vt:r4` — `xsd:float`; Strict, Transitional.
- [ ] `vt:r8` — `xsd:double`; Strict, Transitional.
- [ ] `vt:decimal` — `xsd:decimal`; Strict, Transitional.
- [ ] `vt:bstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:date` — `xsd:dateTime`; Strict, Transitional.
- [ ] `vt:bool` — `xsd:boolean`; Strict, Transitional.
- [ ] `vt:error` — [vt:ST_Error](values-extensions.md#vt-st_error); Strict, Transitional.
- [ ] `vt:cy` — [vt:ST_Cy](values-extensions.md#vt-st_cy); Strict, Transitional.
- [ ] `@lBounds` — `xsd:int`; Strict, Transitional; use=required.
- [ ] `@uBounds` — `xsd:int`; Strict, Transitional; use=required.
- [ ] `@baseType` — [vt:ST_ArrayBaseType](values-extensions.md#vt-st_arraybasetype); Strict, Transitional; use=required.

<a id="vt-ct_empty"></a>

## vt:CT_Empty

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesVariantTypes.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesVariantTypes.xsd).

Empty marker type; no attributes or child properties.

<a id="vt-ct_null"></a>

## vt:CT_Null

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesVariantTypes.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesVariantTypes.xsd).

Empty marker type; no attributes or child properties.

<a id="vt-ct_vstream"></a>

## vt:CT_Vstream

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesVariantTypes.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesVariantTypes.xsd).

- [ ] `@version` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional.

<a id="vt-ct_variant"></a>

## vt:CT_Variant

Sources: [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-documentPropertiesVariantTypes.xsd), [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-documentPropertiesVariantTypes.xsd).

- [ ] `vt:variant` — [vt:CT_Variant](metadata-properties.md#vt-ct_variant); Strict, Transitional.
- [ ] `vt:vector` — [vt:CT_Vector](metadata-properties.md#vt-ct_vector); Strict, Transitional.
- [ ] `vt:array` — [vt:CT_Array](metadata-properties.md#vt-ct_array); Strict, Transitional.
- [ ] `vt:blob` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:oblob` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:empty` — [vt:CT_Empty](metadata-properties.md#vt-ct_empty); Strict, Transitional.
- [ ] `vt:null` — [vt:CT_Null](metadata-properties.md#vt-ct_null); Strict, Transitional.
- [ ] `vt:i1` — `xsd:byte`; Strict, Transitional.
- [ ] `vt:i2` — `xsd:short`; Strict, Transitional.
- [ ] `vt:i4` — `xsd:int`; Strict, Transitional.
- [ ] `vt:i8` — `xsd:long`; Strict, Transitional.
- [ ] `vt:int` — `xsd:int`; Strict, Transitional.
- [ ] `vt:ui1` — `xsd:unsignedByte`; Strict, Transitional.
- [ ] `vt:ui2` — `xsd:unsignedShort`; Strict, Transitional.
- [ ] `vt:ui4` — `xsd:unsignedInt`; Strict, Transitional.
- [ ] `vt:ui8` — `xsd:unsignedLong`; Strict, Transitional.
- [ ] `vt:uint` — `xsd:unsignedInt`; Strict, Transitional.
- [ ] `vt:r4` — `xsd:float`; Strict, Transitional.
- [ ] `vt:r8` — `xsd:double`; Strict, Transitional.
- [ ] `vt:decimal` — `xsd:decimal`; Strict, Transitional.
- [ ] `vt:lpstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:lpwstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:bstr` — `xsd:string`; Strict, Transitional.
- [ ] `vt:date` — `xsd:dateTime`; Strict, Transitional.
- [ ] `vt:filetime` — `xsd:dateTime`; Strict, Transitional.
- [ ] `vt:bool` — `xsd:boolean`; Strict, Transitional.
- [ ] `vt:cy` — [vt:ST_Cy](values-extensions.md#vt-st_cy); Strict, Transitional.
- [ ] `vt:error` — [vt:ST_Error](values-extensions.md#vt-st_error); Strict, Transitional.
- [ ] `vt:stream` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:ostream` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:storage` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:ostorage` — `xsd:base64Binary`; Strict, Transitional.
- [ ] `vt:vstream` — [vt:CT_Vstream](metadata-properties.md#vt-ct_vstream); Strict, Transitional.
- [ ] `vt:clsid` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional.
