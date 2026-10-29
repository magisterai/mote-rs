# Drawing shapes and connectors

[Inventory guide](README.md)

Complete DrawingML shape and connector property families, nonvisual metadata, geometry, fills, strokes, effects, text bodies and Word shape hosting. Textbox content belongs to the existing Word content inventories; textbox identifiers remain inventoried here. Unsupported preset kinds and property values remain visible without introducing authoring names.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [p:CT_Shape](#p-ct_shape)
- [p:CT_Connector](#p-ct_connector)
- [wps:CT_WordprocessingShape](#wps-ct_wordprocessingshape)
- [wps:CT_TextboxInfo](#wps-ct_textboxinfo)
- [p:CT_ShapeNonVisual](#p-ct_shapenonvisual)
- [a:CT_ShapeStyle](#a-ct_shapestyle)
- [p:CT_ExtensionListModify](#p-ct_extensionlistmodify)
- [p:CT_ConnectorNonVisual](#p-ct_connectornonvisual)
- [a:CT_NonVisualDrawingShapeProps](#a-ct_nonvisualdrawingshapeprops)
- [a:CT_NonVisualConnectorProperties](#a-ct_nonvisualconnectorproperties)
- [wps:CT_LinkedTextboxInformation](#wps-ct_linkedtextboxinformation)
- [p:CT_ApplicationNonVisualDrawingProps](#p-ct_applicationnonvisualdrawingprops)
- [a:CT_StyleMatrixReference](#a-ct_stylematrixreference)
- [a:CT_FontReference](#a-ct_fontreference)
- [a:CT_ShapeLocking](#a-ct_shapelocking)
- [a:CT_ConnectorLocking](#a-ct_connectorlocking)
- [a:CT_Connection](#a-ct_connection)
- [p:CT_Placeholder](#p-ct_placeholder)
- [a:CT_AudioCD](#a-ct_audiocd)
- [a:CT_AudioFile](#a-ct_audiofile)
- [a:CT_VideoFile](#a-ct_videofile)
- [a:CT_QuickTimeFile](#a-ct_quicktimefile)
- [p:CT_CustomerDataList](#p-ct_customerdatalist)
- [a:CT_AudioCDTime](#a-ct_audiocdtime)
- [p:CT_CustomerData](#p-ct_customerdata)
- [p:CT_TagsData](#p-ct_tagsdata)

<a id="p-ct_shape"></a>

## p:CT_Shape

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `nvSpPr` — [p:CT_ShapeNonVisual](shapes-and-lines.md#p-ct_shapenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `txBody` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@useBgFill` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_connector"></a>

## p:CT_Connector

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `nvCxnSpPr` — [p:CT_ConnectorNonVisual](shapes-and-lines.md#p-ct_connectornonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="wps-ct_wordprocessingshape"></a>

## wps:CT_WordprocessingShape

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `cNvSpPr` — [a:CT_NonVisualDrawingShapeProps](shapes-and-lines.md#a-ct_nonvisualdrawingshapeprops); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `cNvCnPr` — [a:CT_NonVisualConnectorProperties](shapes-and-lines.md#a-ct_nonvisualconnectorproperties); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txbx` — [wps:CT_TextboxInfo](shapes-and-lines.md#wps-ct_textboxinfo); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `linkedTxbx` — [wps:CT_LinkedTextboxInformation](shapes-and-lines.md#wps-ct_linkedtextboxinformation); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `bodyPr` — [a:CT_TextBodyProperties](drawing-text.md#a-ct_textbodyproperties); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `@normalEastAsianFlow` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.

<a id="wps-ct_textboxinfo"></a>

## wps:CT_TextboxInfo

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

The container hosts WordprocessingML textbox content and extension payloads. Its identifiers are inventoried here; paragraph/run/table content follows their existing inventories. Linked text boxes remain unsupported.

Container attributes only; child content has its own directive inventories.

- [ ] `@id` — `xsd:unsignedShort`; MS-ODRAWXML; use=optional; default=0.

<a id="p-ct_shapenonvisual"></a>

## p:CT_ShapeNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvSpPr` — [a:CT_NonVisualDrawingShapeProps](shapes-and-lines.md#a-ct_nonvisualdrawingshapeprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `nvPr` — [p:CT_ApplicationNonVisualDrawingProps](shapes-and-lines.md#p-ct_applicationnonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_shapestyle"></a>

## a:CT_ShapeStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `lnRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fillRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fontRef` — [a:CT_FontReference](shapes-and-lines.md#a-ct_fontreference); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="p-ct_extensionlistmodify"></a>

## p:CT_ExtensionListModify

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `ext` — [p:CT_Extension](drawing-text.md#p-ct_extension); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `@mod` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_connectornonvisual"></a>

## p:CT_ConnectorNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvCxnSpPr` — [a:CT_NonVisualConnectorProperties](shapes-and-lines.md#a-ct_nonvisualconnectorproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `nvPr` — [p:CT_ApplicationNonVisualDrawingProps](shapes-and-lines.md#p-ct_applicationnonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_nonvisualdrawingshapeprops"></a>

## a:CT_NonVisualDrawingShapeProps

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `spLocks` — [a:CT_ShapeLocking](shapes-and-lines.md#a-ct_shapelocking); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@txBox` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_nonvisualconnectorproperties"></a>

## a:CT_NonVisualConnectorProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `cxnSpLocks` — [a:CT_ConnectorLocking](shapes-and-lines.md#a-ct_connectorlocking); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `stCxn` — [a:CT_Connection](shapes-and-lines.md#a-ct_connection); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `endCxn` — [a:CT_Connection](shapes-and-lines.md#a-ct_connection); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="wps-ct_linkedtextboxinformation"></a>

## wps:CT_LinkedTextboxInformation

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@id` — `xsd:unsignedShort`; MS-ODRAWXML; use=required.
- [ ] `@seq` — `xsd:unsignedShort`; MS-ODRAWXML; use=required.

<a id="p-ct_applicationnonvisualdrawingprops"></a>

## p:CT_ApplicationNonVisualDrawingProps

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `ph` — [p:CT_Placeholder](shapes-and-lines.md#p-ct_placeholder); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `audioCd` — [a:CT_AudioCD](shapes-and-lines.md#a-ct_audiocd); Strict, Transitional.
- [ ] `wavAudioFile` — [a:CT_EmbeddedWAVAudioFile](links.md#a-ct_embeddedwavaudiofile); Strict, Transitional.
- [ ] `audioFile` — [a:CT_AudioFile](shapes-and-lines.md#a-ct_audiofile); Strict, Transitional.
- [ ] `videoFile` — [a:CT_VideoFile](shapes-and-lines.md#a-ct_videofile); Strict, Transitional.
- [ ] `quickTimeFile` — [a:CT_QuickTimeFile](shapes-and-lines.md#a-ct_quicktimefile); Strict, Transitional.
- [ ] `custDataLst` — [p:CT_CustomerDataList](shapes-and-lines.md#p-ct_customerdatalist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@isPhoto` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@userDrawn` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_stylematrixreference"></a>

## a:CT_StyleMatrixReference

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@idx` — [a:ST_StyleMatrixColumnIndex](values-drawing.md#a-st_stylematrixcolumnindex); Strict, Transitional; use=required.

<a id="a-ct_fontreference"></a>

## a:CT_FontReference

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@idx` — [a:ST_FontCollectionIndex](values-drawing.md#a-st_fontcollectionindex); Strict, Transitional; use=required.

<a id="a-ct_shapelocking"></a>

## a:CT_ShapeLocking

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@noGrp` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noSelect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noRot` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeAspect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noMove` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noResize` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noEditPoints` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noAdjustHandles` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeArrowheads` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeShapeType` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noTextEdit` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_connectorlocking"></a>

## a:CT_ConnectorLocking

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@noGrp` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noSelect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noRot` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeAspect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noMove` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noResize` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noEditPoints` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noAdjustHandles` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeArrowheads` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeShapeType` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_connection"></a>

## a:CT_Connection

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@id` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); Strict, Transitional; use=required.
- [ ] `@idx` — `xsd:unsignedInt`; Strict, Transitional; use=required.

<a id="p-ct_placeholder"></a>

## p:CT_Placeholder

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@type` — [p:ST_PlaceholderType](values-drawing.md#p-st_placeholdertype); Strict, Transitional; use=optional; default=obj.
- [ ] `@orient` — [p:ST_Direction](values-drawing.md#p-st_direction); Strict, Transitional; use=optional; default=horz.
- [ ] `@sz` — [p:ST_PlaceholderSize](values-drawing.md#p-st_placeholdersize); Strict, Transitional; use=optional; default=full.
- [ ] `@idx` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=0.
- [ ] `@hasCustomPrompt` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_audiocd"></a>

## a:CT_AudioCD

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `st` — [a:CT_AudioCDTime](shapes-and-lines.md#a-ct_audiocdtime); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `end` — [a:CT_AudioCDTime](shapes-and-lines.md#a-ct_audiocdtime); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_audiofile"></a>

## a:CT_AudioFile

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
- [ ] `@contentType` — `xsd:string`; Strict, Transitional; use=optional.

<a id="a-ct_videofile"></a>

## a:CT_VideoFile

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
- [ ] `@contentType` — `xsd:string`; Strict, Transitional; use=optional.

<a id="a-ct_quicktimefile"></a>

## a:CT_QuickTimeFile

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_customerdatalist"></a>

## p:CT_CustomerDataList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `custData` — [p:CT_CustomerData](shapes-and-lines.md#p-ct_customerdata); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `tags` — [p:CT_TagsData](shapes-and-lines.md#p-ct_tagsdata); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_audiocdtime"></a>

## a:CT_AudioCDTime

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@track` — `xsd:unsignedByte`; Strict, Transitional; use=required.
- [ ] `@time` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=0.

<a id="p-ct_customerdata"></a>

## p:CT_CustomerData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_tagsdata"></a>

## p:CT_TagsData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
