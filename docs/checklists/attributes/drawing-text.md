# DrawingML and PresentationML text properties

[Inventory guide](README.md)

Used by presentation text, paragraph/run formatting, bullet controls, and text defaults. Includes text-body context, all nested fill/effect/geometry properties, and slide-master text styles; these context properties are not automatically direct Mote attributes.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [a:CT_TextBody](#a-ct_textbody)
- [p:CT_SlideMasterTextStyles](#p-ct_slidemastertextstyles)
- [a:CT_TextBodyProperties](#a-ct_textbodyproperties)
- [a:CT_TextListStyle](#a-ct_textliststyle)
- [a:CT_TextParagraph](#a-ct_textparagraph)
- [p:CT_ExtensionList](#p-ct_extensionlist)
- [a:CT_PresetTextShape](#a-ct_presettextshape)
- [a:CT_TextNoAutofit](#a-ct_textnoautofit)
- [a:CT_TextNormalAutofit](#a-ct_textnormalautofit)
- [a:CT_TextShapeAutofit](#a-ct_textshapeautofit)
- [a:CT_Scene3D](#a-ct_scene3d)
- [a:CT_Shape3D](#a-ct_shape3d)
- [a:CT_FlatText](#a-ct_flattext)
- [a:CT_TextParagraphProperties](#a-ct_textparagraphproperties)
- [a:CT_RegularTextRun](#a-ct_regulartextrun)
- [a:CT_TextLineBreak](#a-ct_textlinebreak)
- [a:CT_TextField](#a-ct_textfield)
- [a:CT_TextCharacterProperties](#a-ct_textcharacterproperties)
- [p:CT_Extension](#p-ct_extension)
- [a:CT_GeomGuideList](#a-ct_geomguidelist)
- [a:CT_Camera](#a-ct_camera)
- [a:CT_Backdrop](#a-ct_backdrop)
- [a:CT_TextSpacing](#a-ct_textspacing)
- [a:CT_TextBulletColorFollowText](#a-ct_textbulletcolorfollowtext)
- [a:CT_TextBulletSizeFollowText](#a-ct_textbulletsizefollowtext)
- [a:CT_TextBulletSizePercent](#a-ct_textbulletsizepercent)
- [a:CT_TextBulletSizePoint](#a-ct_textbulletsizepoint)
- [a:CT_TextBulletTypefaceFollowText](#a-ct_textbullettypefacefollowtext)
- [a:CT_TextNoBullet](#a-ct_textnobullet)
- [a:CT_TextAutonumberBullet](#a-ct_textautonumberbullet)
- [a:CT_TextCharBullet](#a-ct_textcharbullet)
- [a:CT_TextBlipBullet](#a-ct_textblipbullet)
- [a:CT_TextTabStopList](#a-ct_texttabstoplist)
- [a:CT_TextUnderlineLineFollowText](#a-ct_textunderlinelinefollowtext)
- [a:CT_TextUnderlineFillFollowText](#a-ct_textunderlinefillfollowtext)
- [a:CT_TextUnderlineFillGroupWrapper](#a-ct_textunderlinefillgroupwrapper)
- [a:CT_Boolean](#a-ct_boolean)
- [a:CT_GeomGuide](#a-ct_geomguide)
- [a:CT_Point3D](#a-ct_point3d)
- [a:CT_Vector3D](#a-ct_vector3d)
- [a:CT_TextSpacingPercent](#a-ct_textspacingpercent)
- [a:CT_TextSpacingPoint](#a-ct_textspacingpoint)
- [a:CT_TextTabStop](#a-ct_texttabstop)

<a id="a-ct_textbody"></a>

## a:CT_TextBody

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `bodyPr` — [a:CT_TextBodyProperties](drawing-text.md#a-ct_textbodyproperties); Strict, Transitional; minOccurs=1; maxOccurs=1. Body Properties; [§21.1.2.1.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3196-3200-0593.md#source-page-3197-part-1).
- [ ] `lstStyle` — [a:CT_TextListStyle](drawing-text.md#a-ct_textliststyle); Strict, Transitional; minOccurs=0; maxOccurs=1. Text List Styles; [§21.1.2.4.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3279-part-1).
- [ ] `p` — [a:CT_TextParagraph](drawing-text.md#a-ct_textparagraph); Strict, Transitional; minOccurs=1; maxOccurs=unbounded. Text Paragraphs; [§21.1.2.2.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3223-3228-0598.md#source-page-3225-part-1).

<a id="p-ct_slidemastertextstyles"></a>

## p:CT_SlideMasterTextStyles

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `titleStyle` — [a:CT_TextListStyle](drawing-text.md#a-ct_textliststyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `bodyStyle` — [a:CT_TextListStyle](drawing-text.md#a-ct_textliststyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `otherStyle` — [a:CT_TextListStyle](drawing-text.md#a-ct_textliststyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_textbodyproperties"></a>

## a:CT_TextBodyProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `prstTxWarp` — [a:CT_PresetTextShape](drawing-text.md#a-ct_presettextshape); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `noAutofit` — [a:CT_TextNoAutofit](drawing-text.md#a-ct_textnoautofit); Strict, Transitional. No AutoFit; [§21.1.2.1.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3201-3205-0594.md#source-page-3205-part-1).
- [ ] `normAutofit` — [a:CT_TextNormalAutofit](drawing-text.md#a-ct_textnormalautofit); Strict, Transitional. Normal AutoFit; [§21.1.2.1.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3206-3212-0595.md#source-page-3206-part-1).
- [ ] `spAutoFit` — [a:CT_TextShapeAutofit](drawing-text.md#a-ct_textshapeautofit); Strict, Transitional. Shape AutoFit; [§21.1.2.1.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3206-3212-0595.md#source-page-3207-part-1).
- [ ] `scene3d` — [a:CT_Scene3D](drawing-text.md#a-ct_scene3d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `sp3d` — [a:CT_Shape3D](drawing-text.md#a-ct_shape3d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `flatTx` — [a:CT_FlatText](drawing-text.md#a-ct_flattext); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@rot` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=optional.
- [ ] `@spcFirstLastPara` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@vertOverflow` — [a:ST_TextVertOverflowType](values-drawing.md#a-st_textvertoverflowtype); Strict, Transitional; use=optional.
- [ ] `@horzOverflow` — [a:ST_TextHorzOverflowType](values-drawing.md#a-st_texthorzoverflowtype); Strict, Transitional; use=optional.
- [ ] `@vert` — [a:ST_TextVerticalType](values-drawing.md#a-st_textverticaltype); Strict, Transitional; use=optional.
- [ ] `@wrap` — [a:ST_TextWrappingType](values-drawing.md#a-st_textwrappingtype); Strict, Transitional; use=optional.
- [ ] `@lIns` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional.
- [ ] `@tIns` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional.
- [ ] `@rIns` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional.
- [ ] `@bIns` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional.
- [ ] `@numCol` — [a:ST_TextColumnCount](values-drawing.md#a-st_textcolumncount); Strict, Transitional; use=optional.
- [ ] `@spcCol` — [a:ST_PositiveCoordinate32](values-drawing.md#a-st_positivecoordinate32); Strict, Transitional; use=optional.
- [ ] `@rtlCol` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@fromWordArt` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@anchor` — [a:ST_TextAnchoringType](values-drawing.md#a-st_textanchoringtype); Strict, Transitional; use=optional.
- [ ] `@anchorCtr` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@forceAA` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@upright` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@compatLnSpc` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="a-ct_textliststyle"></a>

## a:CT_TextListStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `defPPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Default Paragraph Style; [§21.1.2.2.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3206-3212-0595.md#source-page-3210-part-1).
- [ ] `lvl1pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 1 Text Style; [§21.1.2.4.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3279-part-1).
- [ ] `lvl2pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 2 Text Style; [§21.1.2.4.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3286-3291-0609.md#source-page-3287-part-1).
- [ ] `lvl3pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 3 Text Style; [§21.1.2.4.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3292-3297-0610.md#source-page-3295-part-1).
- [ ] `lvl4pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 4 Text Style; [§21.1.2.4.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3298-3303-0611.md#source-page-3303-part-1).
- [ ] `lvl5pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 5 Text Style; [§21.1.2.4.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3310-3315-0613.md#source-page-3311-part-1).
- [ ] `lvl6pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 6 Text Style; [§21.1.2.4.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3316-3321-0614.md#source-page-3319-part-1).
- [ ] `lvl7pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 7 Text Style; [§21.1.2.4.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3322-3327-0615.md#source-page-3327-part-1).
- [ ] `lvl8pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 8 Text Style; [§21.1.2.4.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3334-3339-0617.md#source-page-3335-part-1).
- [ ] `lvl9pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. List Level 9 Text Style; [§21.1.2.4.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3340-3345-0618.md#source-page-3343-part-1).
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_textparagraph"></a>

## a:CT_TextParagraph

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Paragraph Properties; [§21.1.2.2.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3223-3228-0598.md#source-page-3225-part-1).
- [ ] `r` — [a:CT_RegularTextRun](drawing-text.md#a-ct_regulartextrun); Strict, Transitional. Text Run; [§21.1.2.3.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3255-3259-0604.md#source-page-3257-part-1).
- [ ] `br` — [a:CT_TextLineBreak](drawing-text.md#a-ct_textlinebreak); Strict, Transitional. Text Line Break; [§21.1.2.2.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3206-3212-0595.md#source-page-3209-part-1).
- [ ] `fld` — [a:CT_TextField](drawing-text.md#a-ct_textfield); Strict, Transitional. Text Field; [§21.1.2.2.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3218-3222-0597.md#source-page-3222-part-1).
- [ ] `endParaRPr` — [a:CT_TextCharacterProperties](drawing-text.md#a-ct_textcharacterproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. End Paragraph Run Properties; [§21.1.2.2.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3213-3217-0596.md#source-page-3217-part-1).

<a id="p-ct_extensionlist"></a>

## p:CT_ExtensionList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `ext` — [p:CT_Extension](drawing-text.md#p-ct_extension); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_presettextshape"></a>

## a:CT_PresetTextShape

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `avLst` — [a:CT_GeomGuideList](drawing-text.md#a-ct_geomguidelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@prst` — [a:ST_TextShapeType](values-drawing.md#a-st_textshapetype); Strict, Transitional; use=required.

<a id="a-ct_textnoautofit"></a>

## a:CT_TextNoAutofit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_textnormalautofit"></a>

## a:CT_TextNormalAutofit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@fontScale` — [a:ST_TextFontScalePercentOrPercentString](values-drawing.md#a-st_textfontscalepercentorpercentstring); Strict, Transitional; use=optional; default=100%.
- [ ] `@lnSpcReduction` — [a:ST_TextSpacingPercentOrPercentString](values-drawing.md#a-st_textspacingpercentorpercentstring); Strict, Transitional; use=optional; default=0%.

<a id="a-ct_textshapeautofit"></a>

## a:CT_TextShapeAutofit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_scene3d"></a>

## a:CT_Scene3D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `camera` — [a:CT_Camera](drawing-text.md#a-ct_camera); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lightRig` — [a:CT_LightRig](tables.md#a-ct_lightrig); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `backdrop` — [a:CT_Backdrop](drawing-text.md#a-ct_backdrop); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_shape3d"></a>

## a:CT_Shape3D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `bevelT` — [a:CT_Bevel](tables.md#a-ct_bevel); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `bevelB` — [a:CT_Bevel](tables.md#a-ct_bevel); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extrusionClr` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `contourClr` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@z` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@extrusionH` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@contourW` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@prstMaterial` — [a:ST_PresetMaterialType](values-drawing.md#a-st_presetmaterialtype); Strict, Transitional; use=optional; default=warmMatte.

<a id="a-ct_flattext"></a>

## a:CT_FlatText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@z` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=optional; default=0.

<a id="a-ct_textparagraphproperties"></a>

## a:CT_TextParagraphProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `lnSpc` — [a:CT_TextSpacing](drawing-text.md#a-ct_textspacing); Strict, Transitional; minOccurs=0; maxOccurs=1. Line Spacing; [§21.1.2.2.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3223-3228-0598.md#source-page-3224-part-1). Partial: `spacing.line` represents physical and multiple rules; DrawingML conversion and limits are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `spcBef` — [a:CT_TextSpacing](drawing-text.md#a-ct_textspacing); Strict, Transitional; minOccurs=0; maxOccurs=1. Space Before; [§21.1.2.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3235-3241-0600.md#source-page-3236-part-1). Partial: Physical `spacing.before` exists; percentage before-spacing is missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `spcAft` — [a:CT_TextSpacing](drawing-text.md#a-ct_textspacing); Strict, Transitional; minOccurs=0; maxOccurs=1. Space After; [§21.1.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3235-3241-0600.md#source-page-3236-part-1). Partial: Physical `spacing.after` exists; percentage after-spacing is missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `buClrTx` — [a:CT_TextBulletColorFollowText](drawing-text.md#a-ct_textbulletcolorfollowtext); Strict, Transitional; minOccurs=1; maxOccurs=1. Follow Text; [§21.1.2.4.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3273-part-1).
- [ ] `buClr` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1. Color Specified; [§21.1.2.4.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3272-part-1).
- [ ] `buSzTx` — [a:CT_TextBulletSizeFollowText](drawing-text.md#a-ct_textbulletsizefollowtext); Strict, Transitional. Bullet Size Follows Text; [§21.1.2.4.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3279-part-1).
- [ ] `buSzPct` — [a:CT_TextBulletSizePercent](drawing-text.md#a-ct_textbulletsizepercent); Strict, Transitional. Bullet Size Percentage; [§21.1.2.4.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3277-part-1).
- [ ] `buSzPts` — [a:CT_TextBulletSizePoint](drawing-text.md#a-ct_textbulletsizepoint); Strict, Transitional. Bullet Size Points; [§21.1.2.4.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3278-part-1).
- [ ] `buFontTx` — [a:CT_TextBulletTypefaceFollowText](drawing-text.md#a-ct_textbullettypefacefollowtext); Strict, Transitional. Follow text; [§21.1.2.4.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3276-part-1).
- [ ] `buFont` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional. Specified; [§21.1.2.4.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3274-part-1).
- [ ] `buNone` — [a:CT_TextNoBullet](drawing-text.md#a-ct_textnobullet); Strict, Transitional. No Bullet; [§21.1.2.4.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3276-part-1).
- [ ] `buAutoNum` — [a:CT_TextAutonumberBullet](drawing-text.md#a-ct_textautonumberbullet); Strict, Transitional. Auto-Numbered Bullet; [§21.1.2.4.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3268-part-1). Partial: Scoped numbering formats and positive starts exist; additional autonumber schemes are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `buChar` — [a:CT_TextCharBullet](drawing-text.md#a-ct_textcharbullet); Strict, Transitional. Character Bullet; [§21.1.2.4.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3271-part-1). Partial: Presentation `list.marker` records string metadata; bullet font/size/color controls are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `buBlip` — [a:CT_TextBlipBullet](drawing-text.md#a-ct_textblipbullet); Strict, Transitional. Picture Bullet; [§21.1.2.4.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3269-part-1).
- [ ] `tabLst` — [a:CT_TextTabStopList](drawing-text.md#a-ct_texttabstoplist); Strict, Transitional; minOccurs=0; maxOccurs=1. Tab List; [§21.1.2.2.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3235-3241-0600.md#source-page-3240-part-1). Partial: Paragraph tab objects are retained; full DrawingML tab semantics are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `defRPr` — [a:CT_TextCharacterProperties](drawing-text.md#a-ct_textcharacterproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Default Text Run Properties; [§21.1.2.3.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3242-3245-0601.md#source-page-3244-part-1). Partial: Run-style resolution exists; independently authored presentation default-run property containers are not defined. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@marL` — [a:ST_TextMargin](values-drawing.md#a-st_textmargin); Strict, Transitional; use=optional. Partial: `indent.left` exists; presentation paragraph geometry is not resolved. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@marR` — [a:ST_TextMargin](values-drawing.md#a-st_textmargin); Strict, Transitional; use=optional. Partial: `indent.right` exists on paragraphs; presentation paragraph geometry is not resolved. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@lvl` — [a:ST_TextIndentLevelType](values-drawing.md#a-st_textindentleveltype); Strict, Transitional; use=optional. Partial: Positive list/item levels exist; DrawingML zero-based levels and bounds are not resolved. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@indent` — [a:ST_TextIndent](values-drawing.md#a-st_textindent); Strict, Transitional; use=optional. Partial: First-line/hanging indentation intent exists; signed DrawingML indentation is not resolved. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@algn` — [a:ST_TextAlignType](values-drawing.md#a-st_textaligntype); Strict, Transitional; use=optional. Partial: `alignment` supports left/center/right/justify; remaining DrawingML alignment values are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@defTabSz` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional.
- [ ] `@rtl` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@eaLnBrk` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@fontAlgn` — [a:ST_TextFontAlignType](values-drawing.md#a-st_textfontaligntype); Strict, Transitional; use=optional.
- [ ] `@latinLnBrk` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@hangingPunct` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="a-ct_regulartextrun"></a>

## a:CT_RegularTextRun

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `rPr` — [a:CT_TextCharacterProperties](drawing-text.md#a-ct_textcharacterproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Run Properties; [§21.1.2.3.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3255-3259-0604.md#source-page-3257-part-1).
- [ ] `t` — `xsd:string`; Strict, Transitional; minOccurs=1; maxOccurs=1. Text String; [§21.1.2.3.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3260-3264-0605.md#source-page-3264-part-1).

<a id="a-ct_textlinebreak"></a>

## a:CT_TextLineBreak

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `rPr` — [a:CT_TextCharacterProperties](drawing-text.md#a-ct_textcharacterproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Run Properties; [§21.1.2.3.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3255-3259-0604.md#source-page-3257-part-1).

<a id="a-ct_textfield"></a>

## a:CT_TextField

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `rPr` — [a:CT_TextCharacterProperties](drawing-text.md#a-ct_textcharacterproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Run Properties; [§21.1.2.3.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3255-3259-0604.md#source-page-3257-part-1).
- [ ] `pPr` — [a:CT_TextParagraphProperties](drawing-text.md#a-ct_textparagraphproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Paragraph Properties; [§21.1.2.2.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3223-3228-0598.md#source-page-3225-part-1).
- [ ] `t` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1. Text String; [§21.1.2.3.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3260-3264-0605.md#source-page-3264-part-1).
- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional; use=required.
- [ ] `@type` — `xsd:string`; Strict, Transitional; use=optional.

<a id="a-ct_textcharacterproperties"></a>

## a:CT_TextCharacterProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `ln` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1. Partial: Hexadecimal `color` exists; theme/system/preset colors and color transforms are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectLst` — [a:CT_EffectList](tables.md#a-ct_effectlist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectDag` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `highlight` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=0; maxOccurs=1. Highlight Color; [§21.1.2.3.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3250-3254-0603.md#source-page-3251-part-1). Partial: Hexadecimal `highlight` exists; other color models and transformations are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `uLnTx` — [a:CT_TextUnderlineLineFollowText](drawing-text.md#a-ct_textunderlinelinefollowtext); Strict, Transitional. Underline Follows Text; [§21.1.2.3.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3267-part-1).
- [ ] `uLn` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Underline Stroke; [§21.1.2.3.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3266-part-1).
- [ ] `uFillTx` — [a:CT_TextUnderlineFillFollowText](drawing-text.md#a-ct_textunderlinefillfollowtext); Strict, Transitional. Underline Fill Properties Follow Text; [§21.1.2.3.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3265-part-1).
- [ ] `uFill` — [a:CT_TextUnderlineFillGroupWrapper](drawing-text.md#a-ct_textunderlinefillgroupwrapper); Strict, Transitional. Underline Fill; [§21.1.2.3.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3265-3272-0606.md#source-page-3265-part-1). Partial: `#underline.color` maps a solid underline fill; other fill kinds are not mapped.
- [ ] `latin` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=0; maxOccurs=1. Latin Font; [§21.1.2.3.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3255-3259-0604.md#source-page-3255-part-1). Partial: Font aliases resolve Latin typefaces; theme selection and font classification fields are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `ea` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=0; maxOccurs=1. East Asian Font; [§21.1.2.3.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3246-3249-0602.md#source-page-3249-part-1). Partial: Font aliases resolve East Asian typefaces; theme selection and font classification fields are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `cs` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=0; maxOccurs=1. Complex Script Font; [§21.1.2.3.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3242-3245-0601.md#source-page-3242-part-1). Partial: Font aliases resolve complex-script typefaces; theme selection and font classification fields are missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `sym` — [a:CT_TextFont](fonts.md#a-ct_textfont); Strict, Transitional; minOccurs=0; maxOccurs=1. Symbol Font; [§21.1.2.3.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3260-3264-0605.md#source-page-3262-part-1).
- [ ] `hlinkClick` — [a:CT_Hyperlink](links.md#a-ct_hyperlink); Strict, Transitional; minOccurs=0; maxOccurs=1. Click Hyperlink; [§21.1.2.3.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3250-3254-0603.md#source-page-3251-part-1).
- [ ] `hlinkMouseOver` — [a:CT_Hyperlink](links.md#a-ct_hyperlink); Strict, Transitional; minOccurs=0; maxOccurs=1. Mouse-Over Hyperlink; [§21.1.2.3.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3250-3254-0603.md#source-page-3253-part-1).
- [ ] `rtl` — [a:CT_Boolean](drawing-text.md#a-ct_boolean); Strict, Transitional; minOccurs=0. Right to Left Run; [§21.1.2.2.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3229-3234-0599.md#source-page-3233-part-1).
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@kumimoji` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@lang` — [s:ST_Lang](values-shared.md#s-st_lang); Strict, Transitional; use=optional.
- [ ] `@altLang` — [s:ST_Lang](values-shared.md#s-st_lang); Strict, Transitional; use=optional.
- [ ] `@sz` — [a:ST_TextFontSize](values-drawing.md#a-st_textfontsize); Strict, Transitional; use=optional. Partial: `size` records physical text size; DrawingML unit conversion and allowed bounds are not enforced. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `@b` — `xsd:boolean`; Strict, Transitional; use=optional. Model support: `span.bold` and run-style declarations retain an explicit boolean. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [x] `@i` — `xsd:boolean`; Strict, Transitional; use=optional. Model support: `span.italic` and run-style declarations retain an explicit boolean. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@u` — [a:ST_TextUnderlineType](values-drawing.md#a-st_textunderlinetype); Strict, Transitional; use=optional. Partial: spans retain boolean underline; `#underline` adds single, double, dotted, dashed, wave and none. Other DrawingML styles remain unsupported. See [semantic formatting](../../specification/semantic-formatting.md). Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@strike` — [a:ST_TextStrikeType](values-drawing.md#a-st_textstriketype); Strict, Transitional; use=optional. Partial: `strike` and `double_strike` exist as separate booleans; exclusive DrawingML strike selection is not resolved. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@kern` — [a:ST_TextNonNegativePoint](values-drawing.md#a-st_textnonnegativepoint); Strict, Transitional; use=optional.
- [ ] `@cap` — [a:ST_TextCapsType](values-drawing.md#a-st_textcapstype); Strict, Transitional; use=optional; default=none. Partial: `small_caps` exists; all-capital text selection is missing. Evidence: [spans.md](../../../docs/specification/spans.md), [spans.rs](../../../tests/spans.rs), [formatting.rs](../../../tests/formatting.rs).
- [ ] `@spc` — [a:ST_TextPoint](values-drawing.md#a-st_textpoint); Strict, Transitional; use=optional.
- [ ] `@normalizeH` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@baseline` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional. Partial: subscript and superscript directives map fixed baseline profiles; arbitrary percentage offsets are not authored.
- [ ] `@noProof` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@dirty` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@err` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@smtClean` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@smtId` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=0.
- [ ] `@bmk` — `xsd:string`; Strict, Transitional; use=optional.

<a id="p-ct_extension"></a>

## p:CT_Extension

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `*` — extension wildcard; Strict, Transitional; processContents=lax; minOccurs=0; maxOccurs=unbounded.
- [ ] `@uri` — `xsd:token`; Strict, Transitional; use=required.

<a id="a-ct_geomguidelist"></a>

## a:CT_GeomGuideList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `gd` — [a:CT_GeomGuide](drawing-text.md#a-ct_geomguide); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_camera"></a>

## a:CT_Camera

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `rot` — [a:CT_SphereCoords](tables.md#a-ct_spherecoords); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@prst` — [a:ST_PresetCameraType](values-drawing.md#a-st_presetcameratype); Strict, Transitional; use=required.
- [ ] `@fov` — [a:ST_FOVAngle](values-drawing.md#a-st_fovangle); Strict, Transitional; use=optional.
- [ ] `@zoom` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); Strict, Transitional; use=optional; default=100%.

<a id="a-ct_backdrop"></a>

## a:CT_Backdrop

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `anchor` — [a:CT_Point3D](drawing-text.md#a-ct_point3d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `norm` — [a:CT_Vector3D](drawing-text.md#a-ct_vector3d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `up` — [a:CT_Vector3D](drawing-text.md#a-ct_vector3d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_textspacing"></a>

## a:CT_TextSpacing

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `spcPct` — [a:CT_TextSpacingPercent](drawing-text.md#a-ct_textspacingpercent); Strict, Transitional. Spacing Percent; [§21.1.2.2.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3235-3241-0600.md#source-page-3237-part-1). Partial: Line-spacing multiples exist; percentage before/after paragraph spacing is missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `spcPts` — [a:CT_TextSpacingPoint](drawing-text.md#a-ct_textspacingpoint); Strict, Transitional. Spacing Points; [§21.1.2.2.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3235-3241-0600.md#source-page-3238-part-1). Partial: Physical paragraph spacing exists; DrawingML encoding and exact range constraints are not implemented. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).

<a id="a-ct_textbulletcolorfollowtext"></a>

## a:CT_TextBulletColorFollowText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_textbulletsizefollowtext"></a>

## a:CT_TextBulletSizeFollowText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_textbulletsizepercent"></a>

## a:CT_TextBulletSizePercent

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_TextBulletSizePercent](values-drawing.md#a-st_textbulletsizepercent); Strict, Transitional; use=required.

<a id="a-ct_textbulletsizepoint"></a>

## a:CT_TextBulletSizePoint

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_TextFontSize](values-drawing.md#a-st_textfontsize); Strict, Transitional; use=required.

<a id="a-ct_textbullettypefacefollowtext"></a>

## a:CT_TextBulletTypefaceFollowText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_textnobullet"></a>

## a:CT_TextNoBullet

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_textautonumberbullet"></a>

## a:CT_TextAutonumberBullet

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@type` — [a:ST_TextAutonumberScheme](values-drawing.md#a-st_textautonumberscheme); Strict, Transitional; use=required. Partial: Only the documented decimal/alpha/Roman list formats have Mote counterparts; other schemes are unsupported. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@startAt` — [a:ST_TextBulletStartAtNum](values-drawing.md#a-st_textbulletstartatnum); Strict, Transitional; use=optional; default=1. Partial: Positive `start` values are normalized; DrawingML-specific upper bounds and numbering application are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).

<a id="a-ct_textcharbullet"></a>

## a:CT_TextCharBullet

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@char` — `xsd:string`; Strict, Transitional; use=required. Partial: `list.marker` is retained as presentation marker text; marker rendering is absent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).

<a id="a-ct_textblipbullet"></a>

## a:CT_TextBlipBullet

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `blip` — [a:CT_Blip](images.md#a-ct_blip); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_texttabstoplist"></a>

## a:CT_TextTabStopList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tab` — [a:CT_TextTabStop](drawing-text.md#a-ct_texttabstop); Strict, Transitional; minOccurs=0; maxOccurs=32. Tab Stop; [§21.1.2.2.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3235-3241-0600.md#source-page-3239-part-1).

<a id="a-ct_textunderlinelinefollowtext"></a>

## a:CT_TextUnderlineLineFollowText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_textunderlinefillfollowtext"></a>

## a:CT_TextUnderlineFillFollowText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_textunderlinefillgroupwrapper"></a>

## a:CT_TextUnderlineFillGroupWrapper

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_boolean"></a>

## a:CT_Boolean

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; default=0.

<a id="a-ct_geomguide"></a>

## a:CT_GeomGuide

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@name` — [a:ST_GeomGuideName](values-drawing.md#a-st_geomguidename); Strict, Transitional; use=required.
- [ ] `@fmla` — [a:ST_GeomGuideFormula](values-drawing.md#a-st_geomguideformula); Strict, Transitional; use=required.

<a id="a-ct_point3d"></a>

## a:CT_Point3D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@x` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@y` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@z` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.

<a id="a-ct_vector3d"></a>

## a:CT_Vector3D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@dx` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@dy` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@dz` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.

<a id="a-ct_textspacingpercent"></a>

## a:CT_TextSpacingPercent

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_TextSpacingPercentOrPercentString](values-drawing.md#a-st_textspacingpercentorpercentstring); Strict, Transitional; use=required.

<a id="a-ct_textspacingpoint"></a>

## a:CT_TextSpacingPoint

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_TextSpacingPoint](values-drawing.md#a-st_textspacingpoint); Strict, Transitional; use=required.

<a id="a-ct_texttabstop"></a>

## a:CT_TextTabStop

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@pos` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional.
- [ ] `@algn` — [a:ST_TextTabAlignType](values-drawing.md#a-st_texttabaligntype); Strict, Transitional; use=optional.
