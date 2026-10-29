# Picture and image placement properties

[Inventory guide](README.md)

Image containers, nonvisual metadata, source fill, transforms, effects, borders and Word inline/anchor placement. Unsupported properties and values remain inventoried.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [pic:CT_Picture](#pic-ct_picture)
- [wp:CT_Inline](#wp-ct_inline)
- [wp:CT_Anchor](#wp-ct_anchor)
- [pic:CT_PictureNonVisual](#pic-ct_picturenonvisual)
- [a:CT_BlipFillProperties](#a-ct_blipfillproperties)
- [a:CT_ShapeProperties](#a-ct_shapeproperties)
- [a:CT_PositiveSize2D](#a-ct_positivesize2d)
- [wp:CT_EffectExtent](#wp-ct_effectextent)
- [a:CT_NonVisualDrawingProps](#a-ct_nonvisualdrawingprops)
- [a:CT_NonVisualGraphicFrameProperties](#a-ct_nonvisualgraphicframeproperties)
- [a:CT_GraphicalObject](#a-ct_graphicalobject)
- [a:CT_Point2D](#a-ct_point2d)
- [wp:CT_PosH](#wp-ct_posh)
- [wp:CT_PosV](#wp-ct_posv)
- [wp:CT_WrapNone](#wp-ct_wrapnone)
- [wp:CT_WrapSquare](#wp-ct_wrapsquare)
- [wp:CT_WrapTight](#wp-ct_wraptight)
- [wp:CT_WrapThrough](#wp-ct_wrapthrough)
- [wp:CT_WrapTopBottom](#wp-ct_wraptopbottom)
- [a:CT_NonVisualPictureProperties](#a-ct_nonvisualpictureproperties)
- [a:CT_Blip](#a-ct_blip)
- [a:CT_RelativeRect](#a-ct_relativerect)
- [a:CT_TileInfoProperties](#a-ct_tileinfoproperties)
- [a:CT_StretchInfoProperties](#a-ct_stretchinfoproperties)
- [a:CT_Transform2D](#a-ct_transform2d)
- [a:CT_CustomGeometry2D](#a-ct_customgeometry2d)
- [a:CT_PresetGeometry2D](#a-ct_presetgeometry2d)
- [a:CT_GraphicalObjectFrameLocking](#a-ct_graphicalobjectframelocking)
- [a:CT_GraphicalObjectData](#a-ct_graphicalobjectdata)
- [wp:CT_WrapPath](#wp-ct_wrappath)
- [a:CT_PictureLocking](#a-ct_picturelocking)
- [a:CT_AdjustHandleList](#a-ct_adjusthandlelist)
- [a:CT_ConnectionSiteList](#a-ct_connectionsitelist)
- [a:CT_GeomRect](#a-ct_geomrect)
- [a:CT_Path2DList](#a-ct_path2dlist)
- [a:CT_XYAdjustHandle](#a-ct_xyadjusthandle)
- [a:CT_PolarAdjustHandle](#a-ct_polaradjusthandle)
- [a:CT_ConnectionSite](#a-ct_connectionsite)
- [a:CT_Path2D](#a-ct_path2d)
- [a:CT_AdjPoint2D](#a-ct_adjpoint2d)
- [a:CT_Path2DClose](#a-ct_path2dclose)
- [a:CT_Path2DMoveTo](#a-ct_path2dmoveto)
- [a:CT_Path2DLineTo](#a-ct_path2dlineto)
- [a:CT_Path2DArcTo](#a-ct_path2darcto)
- [a:CT_Path2DQuadBezierTo](#a-ct_path2dquadbezierto)
- [a:CT_Path2DCubicBezierTo](#a-ct_path2dcubicbezierto)

<a id="pic-ct_picture"></a>

## pic:CT_Picture

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-picture.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-picture.xsd).

- [ ] `nvPicPr` — [pic:CT_PictureNonVisual](images.md#pic-ct_picturenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="wp-ct_inline"></a>

## wp:CT_Inline

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `extent` — [a:CT_PositiveSize2D](images.md#a-ct_positivesize2d); Strict, Transitional.
- [ ] `effectExtent` — [wp:CT_EffectExtent](images.md#wp-ct_effectextent); Strict, Transitional; minOccurs=0.
- [ ] `docPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvGraphicFramePr` — [a:CT_NonVisualGraphicFrameProperties](images.md#a-ct_nonvisualgraphicframeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `a:graphic` — [a:CT_GraphicalObject](images.md#a-ct_graphicalobject); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@distT` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distB` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distL` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distR` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.

<a id="wp-ct_anchor"></a>

## wp:CT_Anchor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `simplePos` — [a:CT_Point2D](images.md#a-ct_point2d); Strict, Transitional.
- [ ] `positionH` — [wp:CT_PosH](images.md#wp-ct_posh); Strict, Transitional.
- [ ] `positionV` — [wp:CT_PosV](images.md#wp-ct_posv); Strict, Transitional.
- [ ] `extent` — [a:CT_PositiveSize2D](images.md#a-ct_positivesize2d); Strict, Transitional.
- [ ] `effectExtent` — [wp:CT_EffectExtent](images.md#wp-ct_effectextent); Strict, Transitional; minOccurs=0.
- [ ] `wrapNone` — [wp:CT_WrapNone](images.md#wp-ct_wrapnone); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `wrapSquare` — [wp:CT_WrapSquare](images.md#wp-ct_wrapsquare); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `wrapTight` — [wp:CT_WrapTight](images.md#wp-ct_wraptight); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `wrapThrough` — [wp:CT_WrapThrough](images.md#wp-ct_wrapthrough); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `wrapTopAndBottom` — [wp:CT_WrapTopBottom](images.md#wp-ct_wraptopbottom); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `docPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvGraphicFramePr` — [a:CT_NonVisualGraphicFrameProperties](images.md#a-ct_nonvisualgraphicframeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `a:graphic` — [a:CT_GraphicalObject](images.md#a-ct_graphicalobject); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@distT` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distB` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distL` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distR` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@simplePos` — `xsd:boolean`; Strict, Transitional.
- [ ] `@relativeHeight` — `xsd:unsignedInt`; Strict, Transitional; use=required.
- [ ] `@behindDoc` — `xsd:boolean`; Strict, Transitional; use=required.
- [ ] `@locked` — `xsd:boolean`; Strict, Transitional; use=required.
- [ ] `@layoutInCell` — `xsd:boolean`; Strict, Transitional; use=required.
- [ ] `@hidden` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@allowOverlap` — `xsd:boolean`; Strict, Transitional; use=required.

<a id="pic-ct_picturenonvisual"></a>

## pic:CT_PictureNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-picture.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-picture.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvPicPr` — [a:CT_NonVisualPictureProperties](images.md#a-ct_nonvisualpictureproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_blipfillproperties"></a>

## a:CT_BlipFillProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `blip` — [a:CT_Blip](images.md#a-ct_blip); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `srcRect` — [a:CT_RelativeRect](images.md#a-ct_relativerect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tile` — [a:CT_TileInfoProperties](images.md#a-ct_tileinfoproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `stretch` — [a:CT_StretchInfoProperties](images.md#a-ct_stretchinfoproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@dpi` — `xsd:unsignedInt`; Strict, Transitional; use=optional.
- [ ] `@rotWithShape` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="a-ct_shapeproperties"></a>

## a:CT_ShapeProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `xfrm` — [a:CT_Transform2D](images.md#a-ct_transform2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `custGeom` — [a:CT_CustomGeometry2D](images.md#a-ct_customgeometry2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstGeom` — [a:CT_PresetGeometry2D](images.md#a-ct_presetgeometry2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `ln` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `effectLst` — [a:CT_EffectList](tables.md#a-ct_effectlist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectDag` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `scene3d` — [a:CT_Scene3D](drawing-text.md#a-ct_scene3d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `sp3d` — [a:CT_Shape3D](drawing-text.md#a-ct_shape3d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@bwMode` — [a:ST_BlackWhiteMode](values-drawing.md#a-st_blackwhitemode); Strict, Transitional; use=optional.

<a id="a-ct_positivesize2d"></a>

## a:CT_PositiveSize2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@cx` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=required.
- [ ] `@cy` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=required.

<a id="wp-ct_effectextent"></a>

## wp:CT_EffectExtent

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `@l` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@t` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@r` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@b` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.

<a id="a-ct_nonvisualdrawingprops"></a>

## a:CT_NonVisualDrawingProps

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `hlinkClick` — [a:CT_Hyperlink](links.md#a-ct_hyperlink); Strict, Transitional; minOccurs=0; maxOccurs=1. Click Hyperlink; [§21.1.2.3.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3250-3254-0603.md#source-page-3251-part-1). Image link emits an external hyperlink in DOCX and PDF; additional click-action controls are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `hlinkHover` — [a:CT_Hyperlink](links.md#a-ct_hyperlink); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); Strict, Transitional; use=required.
- [x] `@name` — `xsd:string`; Strict, Transitional; use=required. Image name uses explicit name, title, then generated media filename in DOCX. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [x] `@descr` — `xsd:string`; Strict, Transitional; use=optional; default=. Image description uses description, alt, title, then src in DOCX. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `@hidden` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@title` — `xsd:string`; Strict, Transitional; use=optional; default=.

<a id="a-ct_nonvisualgraphicframeproperties"></a>

## a:CT_NonVisualGraphicFrameProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `graphicFrameLocks` — [a:CT_GraphicalObjectFrameLocking](images.md#a-ct_graphicalobjectframelocking); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_graphicalobject"></a>

## a:CT_GraphicalObject

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `graphicData` — [a:CT_GraphicalObjectData](images.md#a-ct_graphicalobjectdata); Strict, Transitional.

<a id="a-ct_point2d"></a>

## a:CT_Point2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@x` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.
- [ ] `@y` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.

<a id="wp-ct_posh"></a>

## wp:CT_PosH

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `align` — [wp:ST_AlignH](values-drawing.md#wp-st_alignh); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `posOffset` — [wp:ST_PositionOffset](values-drawing.md#wp-st_positionoffset); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@relativeFrom` — [wp:ST_RelFromH](values-drawing.md#wp-st_relfromh); Strict, Transitional; use=required.

<a id="wp-ct_posv"></a>

## wp:CT_PosV

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `align` — [wp:ST_AlignV](values-drawing.md#wp-st_alignv); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `posOffset` — [wp:ST_PositionOffset](values-drawing.md#wp-st_positionoffset); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@relativeFrom` — [wp:ST_RelFromV](values-drawing.md#wp-st_relfromv); Strict, Transitional; use=required.

<a id="wp-ct_wrapnone"></a>

## wp:CT_WrapNone

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

Empty marker type; no attributes or child properties.

<a id="wp-ct_wrapsquare"></a>

## wp:CT_WrapSquare

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `effectExtent` — [wp:CT_EffectExtent](images.md#wp-ct_effectextent); Strict, Transitional; minOccurs=0.
- [ ] `@wrapText` — [wp:ST_WrapText](values-drawing.md#wp-st_wraptext); Strict, Transitional; use=required.
- [ ] `@distT` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distB` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distL` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distR` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.

<a id="wp-ct_wraptight"></a>

## wp:CT_WrapTight

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `wrapPolygon` — [wp:CT_WrapPath](images.md#wp-ct_wrappath); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@wrapText` — [wp:ST_WrapText](values-drawing.md#wp-st_wraptext); Strict, Transitional; use=required.
- [ ] `@distL` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distR` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.

<a id="wp-ct_wrapthrough"></a>

## wp:CT_WrapThrough

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `wrapPolygon` — [wp:CT_WrapPath](images.md#wp-ct_wrappath); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@wrapText` — [wp:ST_WrapText](values-drawing.md#wp-st_wraptext); Strict, Transitional; use=required.
- [ ] `@distL` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distR` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.

<a id="wp-ct_wraptopbottom"></a>

## wp:CT_WrapTopBottom

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `effectExtent` — [wp:CT_EffectExtent](images.md#wp-ct_effectextent); Strict, Transitional; minOccurs=0.
- [ ] `@distT` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.
- [ ] `@distB` — [wp:ST_WrapDistance](values-drawing.md#wp-st_wrapdistance); Strict, Transitional; use=optional.

<a id="a-ct_nonvisualpictureproperties"></a>

## a:CT_NonVisualPictureProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `picLocks` — [a:CT_PictureLocking](images.md#a-ct_picturelocking); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@preferRelativeResize` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="a-ct_blip"></a>

## a:CT_Blip

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `alphaBiLevel` — [a:CT_AlphaBiLevelEffect](tables.md#a-ct_alphabileveleffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaCeiling` — [a:CT_AlphaCeilingEffect](tables.md#a-ct_alphaceilingeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaFloor` — [a:CT_AlphaFloorEffect](tables.md#a-ct_alphaflooreffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaInv` — [a:CT_AlphaInverseEffect](tables.md#a-ct_alphainverseeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_AlphaModulateEffect](tables.md#a-ct_alphamodulateeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaModFix` — [a:CT_AlphaModulateFixedEffect](tables.md#a-ct_alphamodulatefixedeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaRepl` — [a:CT_AlphaReplaceEffect](tables.md#a-ct_alphareplaceeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `biLevel` — [a:CT_BiLevelEffect](tables.md#a-ct_bileveleffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blur` — [a:CT_BlurEffect](tables.md#a-ct_blureffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrChange` — [a:CT_ColorChangeEffect](tables.md#a-ct_colorchangeeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrRepl` — [a:CT_ColorReplaceEffect](tables.md#a-ct_colorreplaceeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `duotone` — [a:CT_DuotoneEffect](tables.md#a-ct_duotoneeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fillOverlay` — [a:CT_FillOverlayEffect](tables.md#a-ct_filloverlayeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [x] `grayscl` — [a:CT_GrayscaleEffect](tables.md#a-ct_grayscaleeffect); Strict, Transitional; minOccurs=1; maxOccurs=1. Image effects.grayscale emits the DrawingML effect and paints grayscale in native output. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `hsl` — [a:CT_HSLEffect](tables.md#a-ct_hsleffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_LuminanceEffect](tables.md#a-ct_luminanceeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tint` — [a:CT_TintEffect](tables.md#a-ct_tinteffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [x] `@r:embed` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional; default=. Image src resolves through the asset resolver and embeds original PNG, JPEG or GIF media in DOCX; generated relationship identifiers are not authorable. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional; default=.
- [ ] `@cstate` — [a:ST_BlipCompression](values-drawing.md#a-st_blipcompression); Strict, Transitional; use=optional; default=none.

<a id="a-ct_relativerect"></a>

## a:CT_RelativeRect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@l` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=0%. Image `crop.left` maps percentage values; native crop clamps to 0..100 and fills the authored frame. Other rectangle uses are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `@t` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=0%. Image `crop.top` maps percentage values; native crop clamps to 0..100 and fills the authored frame. Other rectangle uses are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `@r` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=0%. Image `crop.right` maps percentage values; native crop clamps to 0..100 and fills the authored frame. Other rectangle uses are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `@b` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=0%. Image `crop.bottom` maps percentage values; native crop clamps to 0..100 and fills the authored frame. Other rectangle uses are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).

<a id="a-ct_tileinfoproperties"></a>

## a:CT_TileInfoProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@tx` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=optional.
- [ ] `@ty` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=optional.
- [ ] `@sx` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional.
- [ ] `@sy` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional.
- [ ] `@flip` — [a:ST_TileFlipMode](values-drawing.md#a-st_tileflipmode); Strict, Transitional; use=optional; default=none.
- [ ] `@algn` — [a:ST_RectAlignment](values-drawing.md#a-st_rectalignment); Strict, Transitional; use=optional.

<a id="a-ct_stretchinfoproperties"></a>

## a:CT_StretchInfoProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `fillRect` — [a:CT_RelativeRect](images.md#a-ct_relativerect); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_transform2d"></a>

## a:CT_Transform2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `off` — [a:CT_Point2D](images.md#a-ct_point2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `ext` — [a:CT_PositiveSize2D](images.md#a-ct_positivesize2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [x] `@rot` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=optional; default=0. Image `rotation` is normalized, emitted in DrawingML and painted in native output. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [x] `@flipH` — `xsd:boolean`; Strict, Transitional; use=optional; default=false. Image `flip_horizontal` is normalized, emitted in DrawingML and painted in native output. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [x] `@flipV` — `xsd:boolean`; Strict, Transitional; use=optional; default=false. Image `flip_vertical` is normalized, emitted in DrawingML and painted in native output. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).

<a id="a-ct_customgeometry2d"></a>

## a:CT_CustomGeometry2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `avLst` — [a:CT_GeomGuideList](drawing-text.md#a-ct_geomguidelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `gdLst` — [a:CT_GeomGuideList](drawing-text.md#a-ct_geomguidelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `ahLst` — [a:CT_AdjustHandleList](images.md#a-ct_adjusthandlelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `cxnLst` — [a:CT_ConnectionSiteList](images.md#a-ct_connectionsitelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `rect` — [a:CT_GeomRect](images.md#a-ct_geomrect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `pathLst` — [a:CT_Path2DList](images.md#a-ct_path2dlist); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_presetgeometry2d"></a>

## a:CT_PresetGeometry2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `avLst` — [a:CT_GeomGuideList](drawing-text.md#a-ct_geomguidelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@prst` — [a:ST_ShapeType](values-drawing.md#a-st_shapetype); Strict, Transitional; use=required.

<a id="a-ct_graphicalobjectframelocking"></a>

## a:CT_GraphicalObjectFrameLocking

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@noGrp` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noDrilldown` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noSelect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeAspect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noMove` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noResize` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_graphicalobjectdata"></a>

## a:CT_GraphicalObjectData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `*` — extension wildcard; Strict, Transitional; minOccurs=0; maxOccurs=unbounded; processContents=strict.
- [ ] `@uri` — `xsd:token`; Strict, Transitional; use=required.

<a id="wp-ct_wrappath"></a>

## wp:CT_WrapPath

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd).

- [ ] `start` — [a:CT_Point2D](images.md#a-ct_point2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lineTo` — [a:CT_Point2D](images.md#a-ct_point2d); Strict, Transitional; minOccurs=2; maxOccurs=unbounded.
- [ ] `@edited` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="a-ct_picturelocking"></a>

## a:CT_PictureLocking

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
- [ ] `@noCrop` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_adjusthandlelist"></a>

## a:CT_AdjustHandleList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `ahXY` — [a:CT_XYAdjustHandle](images.md#a-ct_xyadjusthandle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `ahPolar` — [a:CT_PolarAdjustHandle](images.md#a-ct_polaradjusthandle); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_connectionsitelist"></a>

## a:CT_ConnectionSiteList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `cxn` — [a:CT_ConnectionSite](images.md#a-ct_connectionsite); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_geomrect"></a>

## a:CT_GeomRect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@l` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.
- [ ] `@t` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.
- [ ] `@r` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.
- [ ] `@b` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.

<a id="a-ct_path2dlist"></a>

## a:CT_Path2DList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `path` — [a:CT_Path2D](images.md#a-ct_path2d); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_xyadjusthandle"></a>

## a:CT_XYAdjustHandle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pos` — [a:CT_AdjPoint2D](images.md#a-ct_adjpoint2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@gdRefX` — [a:ST_GeomGuideName](values-drawing.md#a-st_geomguidename); Strict, Transitional; use=optional.
- [ ] `@minX` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=optional.
- [ ] `@maxX` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=optional.
- [ ] `@gdRefY` — [a:ST_GeomGuideName](values-drawing.md#a-st_geomguidename); Strict, Transitional; use=optional.
- [ ] `@minY` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=optional.
- [ ] `@maxY` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=optional.

<a id="a-ct_polaradjusthandle"></a>

## a:CT_PolarAdjustHandle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pos` — [a:CT_AdjPoint2D](images.md#a-ct_adjpoint2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@gdRefR` — [a:ST_GeomGuideName](values-drawing.md#a-st_geomguidename); Strict, Transitional; use=optional.
- [ ] `@minR` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=optional.
- [ ] `@maxR` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=optional.
- [ ] `@gdRefAng` — [a:ST_GeomGuideName](values-drawing.md#a-st_geomguidename); Strict, Transitional; use=optional.
- [ ] `@minAng` — [a:ST_AdjAngle](values-drawing.md#a-st_adjangle); Strict, Transitional; use=optional.
- [ ] `@maxAng` — [a:ST_AdjAngle](values-drawing.md#a-st_adjangle); Strict, Transitional; use=optional.

<a id="a-ct_connectionsite"></a>

## a:CT_ConnectionSite

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pos` — [a:CT_AdjPoint2D](images.md#a-ct_adjpoint2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@ang` — [a:ST_AdjAngle](values-drawing.md#a-st_adjangle); Strict, Transitional; use=required.

<a id="a-ct_path2d"></a>

## a:CT_Path2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `close` — [a:CT_Path2DClose](images.md#a-ct_path2dclose); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `moveTo` — [a:CT_Path2DMoveTo](images.md#a-ct_path2dmoveto); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lnTo` — [a:CT_Path2DLineTo](images.md#a-ct_path2dlineto); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `arcTo` — [a:CT_Path2DArcTo](images.md#a-ct_path2darcto); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `quadBezTo` — [a:CT_Path2DQuadBezierTo](images.md#a-ct_path2dquadbezierto); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cubicBezTo` — [a:CT_Path2DCubicBezierTo](images.md#a-ct_path2dcubicbezierto); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@w` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@h` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@fill` — [a:ST_PathFillMode](values-drawing.md#a-st_pathfillmode); Strict, Transitional; use=optional; default=norm.
- [ ] `@stroke` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@extrusionOk` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="a-ct_adjpoint2d"></a>

## a:CT_AdjPoint2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@x` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.
- [ ] `@y` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.

<a id="a-ct_path2dclose"></a>

## a:CT_Path2DClose

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_path2dmoveto"></a>

## a:CT_Path2DMoveTo

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pt` — [a:CT_AdjPoint2D](images.md#a-ct_adjpoint2d); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_path2dlineto"></a>

## a:CT_Path2DLineTo

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pt` — [a:CT_AdjPoint2D](images.md#a-ct_adjpoint2d); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_path2darcto"></a>

## a:CT_Path2DArcTo

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@wR` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.
- [ ] `@hR` — [a:ST_AdjCoordinate](values-drawing.md#a-st_adjcoordinate); Strict, Transitional; use=required.
- [ ] `@stAng` — [a:ST_AdjAngle](values-drawing.md#a-st_adjangle); Strict, Transitional; use=required.
- [ ] `@swAng` — [a:ST_AdjAngle](values-drawing.md#a-st_adjangle); Strict, Transitional; use=required.

<a id="a-ct_path2dquadbezierto"></a>

## a:CT_Path2DQuadBezierTo

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pt` — [a:CT_AdjPoint2D](images.md#a-ct_adjpoint2d); Strict, Transitional; minOccurs=2; maxOccurs=2.

<a id="a-ct_path2dcubicbezierto"></a>

## a:CT_Path2DCubicBezierTo

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `pt` — [a:CT_AdjPoint2D](images.md#a-ct_adjpoint2d); Strict, Transitional; minOccurs=3; maxOccurs=3.
