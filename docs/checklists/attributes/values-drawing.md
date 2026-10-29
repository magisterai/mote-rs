# Drawing value definitions

[Inventory guide](README.md)

These are complete schema value domains for the selected properties.
Property rows record Mote support and supported subsets separately.
Identical Strict and Transitional definitions are combined.
See also [prose-defined extensions](prose-extensions.md).

## Types

- [a:ST_AdjAngle](#a-st_adjangle)
- [a:ST_AdjCoordinate](#a-st_adjcoordinate)
- [a:ST_Angle](#a-st_angle)
- [a:ST_BevelPresetType](#a-st_bevelpresettype)
- [a:ST_BlackWhiteMode](#a-st_blackwhitemode)
- [a:ST_BlendMode](#a-st_blendmode)
- [a:ST_BlipCompression](#a-st_blipcompression)
- [a:ST_CompoundLine](#a-st_compoundline)
- [a:ST_Coordinate](#a-st_coordinate)
- [a:ST_Coordinate32](#a-st_coordinate32)
- [a:ST_Coordinate32Unqualified](#a-st_coordinate32unqualified)
- [a:ST_CoordinateUnqualified](#a-st_coordinateunqualified)
- [a:ST_DrawingElementId](#a-st_drawingelementid)
- [a:ST_EffectContainerType](#a-st_effectcontainertype)
- [a:ST_FOVAngle](#a-st_fovangle)
- [a:ST_FixedAngle](#a-st_fixedangle)
- [a:ST_FixedPercentage](#a-st_fixedpercentage)
- [a:ST_FixedPercentageDecimal](#a-st_fixedpercentagedecimal)
- [a:ST_GeomGuideFormula](#a-st_geomguideformula)
- [a:ST_GeomGuideName](#a-st_geomguidename)
- [a:ST_LightRigDirection](#a-st_lightrigdirection)
- [a:ST_LightRigType](#a-st_lightrigtype)
- [a:ST_LineCap](#a-st_linecap)
- [a:ST_LineEndLength](#a-st_lineendlength)
- [a:ST_LineEndType](#a-st_lineendtype)
- [a:ST_LineEndWidth](#a-st_lineendwidth)
- [a:ST_LineWidth](#a-st_linewidth)
- [a:ST_PathFillMode](#a-st_pathfillmode)
- [a:ST_PathShadeType](#a-st_pathshadetype)
- [a:ST_PenAlignment](#a-st_penalignment)
- [a:ST_Percentage](#a-st_percentage)
- [a:ST_PercentageDecimal](#a-st_percentagedecimal)
- [a:ST_PitchFamily](#a-st_pitchfamily)
- [a:ST_PositiveCoordinate](#a-st_positivecoordinate)
- [a:ST_PositiveCoordinate32](#a-st_positivecoordinate32)
- [a:ST_PositiveFixedAngle](#a-st_positivefixedangle)
- [a:ST_PositiveFixedPercentage](#a-st_positivefixedpercentage)
- [a:ST_PositiveFixedPercentageDecimal](#a-st_positivefixedpercentagedecimal)
- [a:ST_PositivePercentage](#a-st_positivepercentage)
- [a:ST_PositivePercentageDecimal](#a-st_positivepercentagedecimal)
- [a:ST_PresetCameraType](#a-st_presetcameratype)
- [a:ST_PresetColorVal](#a-st_presetcolorval)
- [a:ST_PresetLineDashVal](#a-st_presetlinedashval)
- [a:ST_PresetMaterialType](#a-st_presetmaterialtype)
- [a:ST_PresetPatternVal](#a-st_presetpatternval)
- [a:ST_PresetShadowVal](#a-st_presetshadowval)
- [a:ST_RectAlignment](#a-st_rectalignment)
- [a:ST_SchemeColorVal](#a-st_schemecolorval)
- [a:ST_ShapeType](#a-st_shapetype)
- [a:ST_SystemColorVal](#a-st_systemcolorval)
- [a:ST_TextAlignType](#a-st_textaligntype)
- [a:ST_TextAnchoringType](#a-st_textanchoringtype)
- [a:ST_TextAutonumberScheme](#a-st_textautonumberscheme)
- [a:ST_TextBulletSizePercent](#a-st_textbulletsizepercent)
- [a:ST_TextBulletStartAtNum](#a-st_textbulletstartatnum)
- [a:ST_TextCapsType](#a-st_textcapstype)
- [a:ST_TextColumnCount](#a-st_textcolumncount)
- [a:ST_TextFontAlignType](#a-st_textfontaligntype)
- [a:ST_TextFontScalePercent](#a-st_textfontscalepercent)
- [a:ST_TextFontScalePercentOrPercentString](#a-st_textfontscalepercentorpercentstring)
- [a:ST_TextFontSize](#a-st_textfontsize)
- [a:ST_TextHorzOverflowType](#a-st_texthorzoverflowtype)
- [a:ST_TextIndent](#a-st_textindent)
- [a:ST_TextIndentLevelType](#a-st_textindentleveltype)
- [a:ST_TextMargin](#a-st_textmargin)
- [a:ST_TextNonNegativePoint](#a-st_textnonnegativepoint)
- [a:ST_TextPoint](#a-st_textpoint)
- [a:ST_TextPointUnqualified](#a-st_textpointunqualified)
- [a:ST_TextShapeType](#a-st_textshapetype)
- [a:ST_TextSpacingPercent](#a-st_textspacingpercent)
- [a:ST_TextSpacingPercentOrPercentString](#a-st_textspacingpercentorpercentstring)
- [a:ST_TextSpacingPoint](#a-st_textspacingpoint)
- [a:ST_TextStrikeType](#a-st_textstriketype)
- [a:ST_TextTabAlignType](#a-st_texttabaligntype)
- [a:ST_TextTypeface](#a-st_texttypeface)
- [a:ST_TextUnderlineType](#a-st_textunderlinetype)
- [a:ST_TextVertOverflowType](#a-st_textvertoverflowtype)
- [a:ST_TextVerticalType](#a-st_textverticaltype)
- [a:ST_TextWrappingType](#a-st_textwrappingtype)
- [a:ST_TileFlipMode](#a-st_tileflipmode)
- [wp:ST_AlignH](#wp-st_alignh)
- [wp:ST_AlignV](#wp-st_alignv)
- [wp:ST_PositionOffset](#wp-st_positionoffset)
- [wp:ST_RelFromH](#wp-st_relfromh)
- [wp:ST_RelFromV](#wp-st_relfromv)
- [wp:ST_WrapDistance](#wp-st_wrapdistance)
- [wp:ST_WrapText](#wp-st_wraptext)

<a id="a-st_adjangle"></a>

## a:ST_AdjAngle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [a:ST_Angle](values-drawing.md#a-st_angle), [a:ST_GeomGuideName](values-drawing.md#a-st_geomguidename)

<a id="a-st_adjcoordinate"></a>

## a:ST_AdjCoordinate

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [a:ST_Coordinate](values-drawing.md#a-st_coordinate), [a:ST_GeomGuideName](values-drawing.md#a-st_geomguidename)

<a id="a-st_angle"></a>

## a:ST_Angle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`

<a id="a-st_bevelpresettype"></a>

## a:ST_BevelPresetType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `relaxedInset`
- enumeration: value: `circle`
- enumeration: value: `slope`
- enumeration: value: `cross`
- enumeration: value: `angle`
- enumeration: value: `softRound`
- enumeration: value: `convex`
- enumeration: value: `coolSlant`
- enumeration: value: `divot`
- enumeration: value: `riblet`
- enumeration: value: `hardEdge`
- enumeration: value: `artDeco`

<a id="a-st_blackwhitemode"></a>

## a:ST_BlackWhiteMode

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `clr`
- enumeration: value: `auto`
- enumeration: value: `gray`
- enumeration: value: `ltGray`
- enumeration: value: `invGray`
- enumeration: value: `grayWhite`
- enumeration: value: `blackGray`
- enumeration: value: `blackWhite`
- enumeration: value: `black`
- enumeration: value: `white`
- enumeration: value: `hidden`

<a id="a-st_blendmode"></a>

## a:ST_BlendMode

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `over`
- enumeration: value: `mult`
- enumeration: value: `screen`
- enumeration: value: `darken`
- enumeration: value: `lighten`

<a id="a-st_blipcompression"></a>

## a:ST_BlipCompression

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `email`
- enumeration: value: `screen`
- enumeration: value: `print`
- enumeration: value: `hqprint`
- enumeration: value: `none`

<a id="a-st_compoundline"></a>

## a:ST_CompoundLine

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `sng`
- enumeration: value: `dbl`
- enumeration: value: `thickThin`
- enumeration: value: `thinThick`
- enumeration: value: `tri`

<a id="a-st_coordinate"></a>

## a:ST_Coordinate

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [a:ST_CoordinateUnqualified](values-drawing.md#a-st_coordinateunqualified), [s:ST_UniversalMeasure](values-shared.md#s-st_universalmeasure)

<a id="a-st_coordinate32"></a>

## a:ST_Coordinate32

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [a:ST_Coordinate32Unqualified](values-drawing.md#a-st_coordinate32unqualified), [s:ST_UniversalMeasure](values-shared.md#s-st_universalmeasure)

<a id="a-st_coordinate32unqualified"></a>

## a:ST_Coordinate32Unqualified

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`

<a id="a-st_coordinateunqualified"></a>

## a:ST_CoordinateUnqualified

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:long`
- minInclusive: value: `-27273042329600`
- maxInclusive: value: `27273042316900`

<a id="a-st_drawingelementid"></a>

## a:ST_DrawingElementId

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:unsignedInt`

<a id="a-st_effectcontainertype"></a>

## a:ST_EffectContainerType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `sib`
- enumeration: value: `tree`

<a id="a-st_fovangle"></a>

## a:ST_FOVAngle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: [a:ST_Angle](values-drawing.md#a-st_angle)
- minInclusive: value: `0`
- maxInclusive: value: `10800000`

<a id="a-st_fixedangle"></a>

## a:ST_FixedAngle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: [a:ST_Angle](values-drawing.md#a-st_angle)
- minExclusive: value: `-5400000`
- maxExclusive: value: `5400000`

<a id="a-st_fixedpercentage"></a>

## a:ST_FixedPercentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- union: memberTypes: [a:ST_FixedPercentageDecimal](values-drawing.md#a-st_fixedpercentagedecimal), [s:ST_FixedPercentage](values-shared.md#s-st_fixedpercentage)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [s:ST_FixedPercentage](values-shared.md#s-st_fixedpercentage)

<a id="a-st_fixedpercentagedecimal"></a>

## a:ST_FixedPercentageDecimal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- restriction: base: [a:ST_PercentageDecimal](values-drawing.md#a-st_percentagedecimal)
- minInclusive: value: `-100000`
- maxInclusive: value: `100000`

<a id="a-st_geomguideformula"></a>

## a:ST_GeomGuideFormula

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:string`

<a id="a-st_geomguidename"></a>

## a:ST_GeomGuideName

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`

<a id="a-st_lightrigdirection"></a>

## a:ST_LightRigDirection

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `tl`
- enumeration: value: `t`
- enumeration: value: `tr`
- enumeration: value: `l`
- enumeration: value: `r`
- enumeration: value: `bl`
- enumeration: value: `b`
- enumeration: value: `br`

<a id="a-st_lightrigtype"></a>

## a:ST_LightRigType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `legacyFlat1`
- enumeration: value: `legacyFlat2`
- enumeration: value: `legacyFlat3`
- enumeration: value: `legacyFlat4`
- enumeration: value: `legacyNormal1`
- enumeration: value: `legacyNormal2`
- enumeration: value: `legacyNormal3`
- enumeration: value: `legacyNormal4`
- enumeration: value: `legacyHarsh1`
- enumeration: value: `legacyHarsh2`
- enumeration: value: `legacyHarsh3`
- enumeration: value: `legacyHarsh4`
- enumeration: value: `threePt`
- enumeration: value: `balanced`
- enumeration: value: `soft`
- enumeration: value: `harsh`
- enumeration: value: `flood`
- enumeration: value: `contrasting`
- enumeration: value: `morning`
- enumeration: value: `sunrise`
- enumeration: value: `sunset`
- enumeration: value: `chilly`
- enumeration: value: `freezing`
- enumeration: value: `flat`
- enumeration: value: `twoPt`
- enumeration: value: `glow`
- enumeration: value: `brightRoom`

<a id="a-st_linecap"></a>

## a:ST_LineCap

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `rnd`
- enumeration: value: `sq`
- enumeration: value: `flat`

<a id="a-st_lineendlength"></a>

## a:ST_LineEndLength

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `sm`
- enumeration: value: `med`
- enumeration: value: `lg`

<a id="a-st_lineendtype"></a>

## a:ST_LineEndType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `none`
- enumeration: value: `triangle`
- enumeration: value: `stealth`
- enumeration: value: `diamond`
- enumeration: value: `oval`
- enumeration: value: `arrow`

<a id="a-st_lineendwidth"></a>

## a:ST_LineEndWidth

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `sm`
- enumeration: value: `med`
- enumeration: value: `lg`

<a id="a-st_linewidth"></a>

## a:ST_LineWidth

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: [a:ST_Coordinate32Unqualified](values-drawing.md#a-st_coordinate32unqualified)
- minInclusive: value: `0`
- maxInclusive: value: `20116800`

<a id="a-st_pathfillmode"></a>

## a:ST_PathFillMode

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `none`
- enumeration: value: `norm`
- enumeration: value: `lighten`
- enumeration: value: `lightenLess`
- enumeration: value: `darken`
- enumeration: value: `darkenLess`

<a id="a-st_pathshadetype"></a>

## a:ST_PathShadeType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `shape`
- enumeration: value: `circle`
- enumeration: value: `rect`

<a id="a-st_penalignment"></a>

## a:ST_PenAlignment

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `ctr`
- enumeration: value: `in`

<a id="a-st_percentage"></a>

## a:ST_Percentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- union: memberTypes: [a:ST_PercentageDecimal](values-drawing.md#a-st_percentagedecimal), [s:ST_Percentage](values-shared.md#s-st_percentage)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [s:ST_Percentage](values-shared.md#s-st_percentage)

<a id="a-st_percentagedecimal"></a>

## a:ST_PercentageDecimal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- restriction: base: `xsd:int`

<a id="a-st_pitchfamily"></a>

## a:ST_PitchFamily

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:byte`
- enumeration: value: `00`
- enumeration: value: `01`
- enumeration: value: `02`
- enumeration: value: `16`
- enumeration: value: `17`
- enumeration: value: `18`
- enumeration: value: `32`
- enumeration: value: `33`
- enumeration: value: `34`
- enumeration: value: `48`
- enumeration: value: `49`
- enumeration: value: `50`
- enumeration: value: `64`
- enumeration: value: `65`
- enumeration: value: `66`
- enumeration: value: `80`
- enumeration: value: `81`
- enumeration: value: `82`

<a id="a-st_positivecoordinate"></a>

## a:ST_PositiveCoordinate

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:long`
- minInclusive: value: `0`
- maxInclusive: value: `27273042316900`

<a id="a-st_positivecoordinate32"></a>

## a:ST_PositiveCoordinate32

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: [a:ST_Coordinate32Unqualified](values-drawing.md#a-st_coordinate32unqualified)
- minInclusive: value: `0`

<a id="a-st_positivefixedangle"></a>

## a:ST_PositiveFixedAngle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: [a:ST_Angle](values-drawing.md#a-st_angle)
- minInclusive: value: `0`
- maxExclusive: value: `21600000`

<a id="a-st_positivefixedpercentage"></a>

## a:ST_PositiveFixedPercentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- union: memberTypes: [a:ST_PositiveFixedPercentageDecimal](values-drawing.md#a-st_positivefixedpercentagedecimal), [s:ST_PositiveFixedPercentage](values-shared.md#s-st_positivefixedpercentage)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [s:ST_PositiveFixedPercentage](values-shared.md#s-st_positivefixedpercentage)

<a id="a-st_positivefixedpercentagedecimal"></a>

## a:ST_PositiveFixedPercentageDecimal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- restriction: base: [a:ST_PercentageDecimal](values-drawing.md#a-st_percentagedecimal)
- minInclusive: value: `0`
- maxInclusive: value: `100000`

<a id="a-st_positivepercentage"></a>

## a:ST_PositivePercentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- union: memberTypes: [a:ST_PositivePercentageDecimal](values-drawing.md#a-st_positivepercentagedecimal), [s:ST_PositivePercentage](values-shared.md#s-st_positivepercentage)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [s:ST_PositivePercentage](values-shared.md#s-st_positivepercentage)

<a id="a-st_positivepercentagedecimal"></a>

## a:ST_PositivePercentageDecimal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- restriction: base: [a:ST_PercentageDecimal](values-drawing.md#a-st_percentagedecimal)
- minInclusive: value: `0`

<a id="a-st_presetcameratype"></a>

## a:ST_PresetCameraType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `legacyObliqueTopLeft`
- enumeration: value: `legacyObliqueTop`
- enumeration: value: `legacyObliqueTopRight`
- enumeration: value: `legacyObliqueLeft`
- enumeration: value: `legacyObliqueFront`
- enumeration: value: `legacyObliqueRight`
- enumeration: value: `legacyObliqueBottomLeft`
- enumeration: value: `legacyObliqueBottom`
- enumeration: value: `legacyObliqueBottomRight`
- enumeration: value: `legacyPerspectiveTopLeft`
- enumeration: value: `legacyPerspectiveTop`
- enumeration: value: `legacyPerspectiveTopRight`
- enumeration: value: `legacyPerspectiveLeft`
- enumeration: value: `legacyPerspectiveFront`
- enumeration: value: `legacyPerspectiveRight`
- enumeration: value: `legacyPerspectiveBottomLeft`
- enumeration: value: `legacyPerspectiveBottom`
- enumeration: value: `legacyPerspectiveBottomRight`
- enumeration: value: `orthographicFront`
- enumeration: value: `isometricTopUp`
- enumeration: value: `isometricTopDown`
- enumeration: value: `isometricBottomUp`
- enumeration: value: `isometricBottomDown`
- enumeration: value: `isometricLeftUp`
- enumeration: value: `isometricLeftDown`
- enumeration: value: `isometricRightUp`
- enumeration: value: `isometricRightDown`
- enumeration: value: `isometricOffAxis1Left`
- enumeration: value: `isometricOffAxis1Right`
- enumeration: value: `isometricOffAxis1Top`
- enumeration: value: `isometricOffAxis2Left`
- enumeration: value: `isometricOffAxis2Right`
- enumeration: value: `isometricOffAxis2Top`
- enumeration: value: `isometricOffAxis3Left`
- enumeration: value: `isometricOffAxis3Right`
- enumeration: value: `isometricOffAxis3Bottom`
- enumeration: value: `isometricOffAxis4Left`
- enumeration: value: `isometricOffAxis4Right`
- enumeration: value: `isometricOffAxis4Bottom`
- enumeration: value: `obliqueTopLeft`
- enumeration: value: `obliqueTop`
- enumeration: value: `obliqueTopRight`
- enumeration: value: `obliqueLeft`
- enumeration: value: `obliqueRight`
- enumeration: value: `obliqueBottomLeft`
- enumeration: value: `obliqueBottom`
- enumeration: value: `obliqueBottomRight`
- enumeration: value: `perspectiveFront`
- enumeration: value: `perspectiveLeft`
- enumeration: value: `perspectiveRight`
- enumeration: value: `perspectiveAbove`
- enumeration: value: `perspectiveBelow`
- enumeration: value: `perspectiveAboveLeftFacing`
- enumeration: value: `perspectiveAboveRightFacing`
- enumeration: value: `perspectiveContrastingLeftFacing`
- enumeration: value: `perspectiveContrastingRightFacing`
- enumeration: value: `perspectiveHeroicLeftFacing`
- enumeration: value: `perspectiveHeroicRightFacing`
- enumeration: value: `perspectiveHeroicExtremeLeftFacing`
- enumeration: value: `perspectiveHeroicExtremeRightFacing`
- enumeration: value: `perspectiveRelaxed`
- enumeration: value: `perspectiveRelaxedModerately`

<a id="a-st_presetcolorval"></a>

## a:ST_PresetColorVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `aliceBlue`
- enumeration: value: `antiqueWhite`
- enumeration: value: `aqua`
- enumeration: value: `aquamarine`
- enumeration: value: `azure`
- enumeration: value: `beige`
- enumeration: value: `bisque`
- enumeration: value: `black`
- enumeration: value: `blanchedAlmond`
- enumeration: value: `blue`
- enumeration: value: `blueViolet`
- enumeration: value: `brown`
- enumeration: value: `burlyWood`
- enumeration: value: `cadetBlue`
- enumeration: value: `chartreuse`
- enumeration: value: `chocolate`
- enumeration: value: `coral`
- enumeration: value: `cornflowerBlue`
- enumeration: value: `cornsilk`
- enumeration: value: `crimson`
- enumeration: value: `cyan`
- enumeration: value: `darkBlue`
- enumeration: value: `darkCyan`
- enumeration: value: `darkGoldenrod`
- enumeration: value: `darkGray`
- enumeration: value: `darkGrey`
- enumeration: value: `darkGreen`
- enumeration: value: `darkKhaki`
- enumeration: value: `darkMagenta`
- enumeration: value: `darkOliveGreen`
- enumeration: value: `darkOrange`
- enumeration: value: `darkOrchid`
- enumeration: value: `darkRed`
- enumeration: value: `darkSalmon`
- enumeration: value: `darkSeaGreen`
- enumeration: value: `darkSlateBlue`
- enumeration: value: `darkSlateGray`
- enumeration: value: `darkSlateGrey`
- enumeration: value: `darkTurquoise`
- enumeration: value: `darkViolet`
- enumeration: value: `dkBlue`
- enumeration: value: `dkCyan`
- enumeration: value: `dkGoldenrod`
- enumeration: value: `dkGray`
- enumeration: value: `dkGrey`
- enumeration: value: `dkGreen`
- enumeration: value: `dkKhaki`
- enumeration: value: `dkMagenta`
- enumeration: value: `dkOliveGreen`
- enumeration: value: `dkOrange`
- enumeration: value: `dkOrchid`
- enumeration: value: `dkRed`
- enumeration: value: `dkSalmon`
- enumeration: value: `dkSeaGreen`
- enumeration: value: `dkSlateBlue`
- enumeration: value: `dkSlateGray`
- enumeration: value: `dkSlateGrey`
- enumeration: value: `dkTurquoise`
- enumeration: value: `dkViolet`
- enumeration: value: `deepPink`
- enumeration: value: `deepSkyBlue`
- enumeration: value: `dimGray`
- enumeration: value: `dimGrey`
- enumeration: value: `dodgerBlue`
- enumeration: value: `firebrick`
- enumeration: value: `floralWhite`
- enumeration: value: `forestGreen`
- enumeration: value: `fuchsia`
- enumeration: value: `gainsboro`
- enumeration: value: `ghostWhite`
- enumeration: value: `gold`
- enumeration: value: `goldenrod`
- enumeration: value: `gray`
- enumeration: value: `grey`
- enumeration: value: `green`
- enumeration: value: `greenYellow`
- enumeration: value: `honeydew`
- enumeration: value: `hotPink`
- enumeration: value: `indianRed`
- enumeration: value: `indigo`
- enumeration: value: `ivory`
- enumeration: value: `khaki`
- enumeration: value: `lavender`
- enumeration: value: `lavenderBlush`
- enumeration: value: `lawnGreen`
- enumeration: value: `lemonChiffon`
- enumeration: value: `lightBlue`
- enumeration: value: `lightCoral`
- enumeration: value: `lightCyan`
- enumeration: value: `lightGoldenrodYellow`
- enumeration: value: `lightGray`
- enumeration: value: `lightGrey`
- enumeration: value: `lightGreen`
- enumeration: value: `lightPink`
- enumeration: value: `lightSalmon`
- enumeration: value: `lightSeaGreen`
- enumeration: value: `lightSkyBlue`
- enumeration: value: `lightSlateGray`
- enumeration: value: `lightSlateGrey`
- enumeration: value: `lightSteelBlue`
- enumeration: value: `lightYellow`
- enumeration: value: `ltBlue`
- enumeration: value: `ltCoral`
- enumeration: value: `ltCyan`
- enumeration: value: `ltGoldenrodYellow`
- enumeration: value: `ltGray`
- enumeration: value: `ltGrey`
- enumeration: value: `ltGreen`
- enumeration: value: `ltPink`
- enumeration: value: `ltSalmon`
- enumeration: value: `ltSeaGreen`
- enumeration: value: `ltSkyBlue`
- enumeration: value: `ltSlateGray`
- enumeration: value: `ltSlateGrey`
- enumeration: value: `ltSteelBlue`
- enumeration: value: `ltYellow`
- enumeration: value: `lime`
- enumeration: value: `limeGreen`
- enumeration: value: `linen`
- enumeration: value: `magenta`
- enumeration: value: `maroon`
- enumeration: value: `medAquamarine`
- enumeration: value: `medBlue`
- enumeration: value: `medOrchid`
- enumeration: value: `medPurple`
- enumeration: value: `medSeaGreen`
- enumeration: value: `medSlateBlue`
- enumeration: value: `medSpringGreen`
- enumeration: value: `medTurquoise`
- enumeration: value: `medVioletRed`
- enumeration: value: `mediumAquamarine`
- enumeration: value: `mediumBlue`
- enumeration: value: `mediumOrchid`
- enumeration: value: `mediumPurple`
- enumeration: value: `mediumSeaGreen`
- enumeration: value: `mediumSlateBlue`
- enumeration: value: `mediumSpringGreen`
- enumeration: value: `mediumTurquoise`
- enumeration: value: `mediumVioletRed`
- enumeration: value: `midnightBlue`
- enumeration: value: `mintCream`
- enumeration: value: `mistyRose`
- enumeration: value: `moccasin`
- enumeration: value: `navajoWhite`
- enumeration: value: `navy`
- enumeration: value: `oldLace`
- enumeration: value: `olive`
- enumeration: value: `oliveDrab`
- enumeration: value: `orange`
- enumeration: value: `orangeRed`
- enumeration: value: `orchid`
- enumeration: value: `paleGoldenrod`
- enumeration: value: `paleGreen`
- enumeration: value: `paleTurquoise`
- enumeration: value: `paleVioletRed`
- enumeration: value: `papayaWhip`
- enumeration: value: `peachPuff`
- enumeration: value: `peru`
- enumeration: value: `pink`
- enumeration: value: `plum`
- enumeration: value: `powderBlue`
- enumeration: value: `purple`
- enumeration: value: `red`
- enumeration: value: `rosyBrown`
- enumeration: value: `royalBlue`
- enumeration: value: `saddleBrown`
- enumeration: value: `salmon`
- enumeration: value: `sandyBrown`
- enumeration: value: `seaGreen`
- enumeration: value: `seaShell`
- enumeration: value: `sienna`
- enumeration: value: `silver`
- enumeration: value: `skyBlue`
- enumeration: value: `slateBlue`
- enumeration: value: `slateGray`
- enumeration: value: `slateGrey`
- enumeration: value: `snow`
- enumeration: value: `springGreen`
- enumeration: value: `steelBlue`
- enumeration: value: `tan`
- enumeration: value: `teal`
- enumeration: value: `thistle`
- enumeration: value: `tomato`
- enumeration: value: `turquoise`
- enumeration: value: `violet`
- enumeration: value: `wheat`
- enumeration: value: `white`
- enumeration: value: `whiteSmoke`
- enumeration: value: `yellow`
- enumeration: value: `yellowGreen`

<a id="a-st_presetlinedashval"></a>

## a:ST_PresetLineDashVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `solid`
- enumeration: value: `dot`
- enumeration: value: `dash`
- enumeration: value: `lgDash`
- enumeration: value: `dashDot`
- enumeration: value: `lgDashDot`
- enumeration: value: `lgDashDotDot`
- enumeration: value: `sysDash`
- enumeration: value: `sysDot`
- enumeration: value: `sysDashDot`
- enumeration: value: `sysDashDotDot`

<a id="a-st_presetmaterialtype"></a>

## a:ST_PresetMaterialType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `legacyMatte`
- enumeration: value: `legacyPlastic`
- enumeration: value: `legacyMetal`
- enumeration: value: `legacyWireframe`
- enumeration: value: `matte`
- enumeration: value: `plastic`
- enumeration: value: `metal`
- enumeration: value: `warmMatte`
- enumeration: value: `translucentPowder`
- enumeration: value: `powder`
- enumeration: value: `dkEdge`
- enumeration: value: `softEdge`
- enumeration: value: `clear`
- enumeration: value: `flat`
- enumeration: value: `softmetal`

<a id="a-st_presetpatternval"></a>

## a:ST_PresetPatternVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `pct5`
- enumeration: value: `pct10`
- enumeration: value: `pct20`
- enumeration: value: `pct25`
- enumeration: value: `pct30`
- enumeration: value: `pct40`
- enumeration: value: `pct50`
- enumeration: value: `pct60`
- enumeration: value: `pct70`
- enumeration: value: `pct75`
- enumeration: value: `pct80`
- enumeration: value: `pct90`
- enumeration: value: `horz`
- enumeration: value: `vert`
- enumeration: value: `ltHorz`
- enumeration: value: `ltVert`
- enumeration: value: `dkHorz`
- enumeration: value: `dkVert`
- enumeration: value: `narHorz`
- enumeration: value: `narVert`
- enumeration: value: `dashHorz`
- enumeration: value: `dashVert`
- enumeration: value: `cross`
- enumeration: value: `dnDiag`
- enumeration: value: `upDiag`
- enumeration: value: `ltDnDiag`
- enumeration: value: `ltUpDiag`
- enumeration: value: `dkDnDiag`
- enumeration: value: `dkUpDiag`
- enumeration: value: `wdDnDiag`
- enumeration: value: `wdUpDiag`
- enumeration: value: `dashDnDiag`
- enumeration: value: `dashUpDiag`
- enumeration: value: `diagCross`
- enumeration: value: `smCheck`
- enumeration: value: `lgCheck`
- enumeration: value: `smGrid`
- enumeration: value: `lgGrid`
- enumeration: value: `dotGrid`
- enumeration: value: `smConfetti`
- enumeration: value: `lgConfetti`
- enumeration: value: `horzBrick`
- enumeration: value: `diagBrick`
- enumeration: value: `solidDmnd`
- enumeration: value: `openDmnd`
- enumeration: value: `dotDmnd`
- enumeration: value: `plaid`
- enumeration: value: `sphere`
- enumeration: value: `weave`
- enumeration: value: `divot`
- enumeration: value: `shingle`
- enumeration: value: `wave`
- enumeration: value: `trellis`
- enumeration: value: `zigZag`

<a id="a-st_presetshadowval"></a>

## a:ST_PresetShadowVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `shdw1`
- enumeration: value: `shdw2`
- enumeration: value: `shdw3`
- enumeration: value: `shdw4`
- enumeration: value: `shdw5`
- enumeration: value: `shdw6`
- enumeration: value: `shdw7`
- enumeration: value: `shdw8`
- enumeration: value: `shdw9`
- enumeration: value: `shdw10`
- enumeration: value: `shdw11`
- enumeration: value: `shdw12`
- enumeration: value: `shdw13`
- enumeration: value: `shdw14`
- enumeration: value: `shdw15`
- enumeration: value: `shdw16`
- enumeration: value: `shdw17`
- enumeration: value: `shdw18`
- enumeration: value: `shdw19`
- enumeration: value: `shdw20`

<a id="a-st_rectalignment"></a>

## a:ST_RectAlignment

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `tl`
- enumeration: value: `t`
- enumeration: value: `tr`
- enumeration: value: `l`
- enumeration: value: `ctr`
- enumeration: value: `r`
- enumeration: value: `bl`
- enumeration: value: `b`
- enumeration: value: `br`

<a id="a-st_schemecolorval"></a>

## a:ST_SchemeColorVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `bg1`
- enumeration: value: `tx1`
- enumeration: value: `bg2`
- enumeration: value: `tx2`
- enumeration: value: `accent1`
- enumeration: value: `accent2`
- enumeration: value: `accent3`
- enumeration: value: `accent4`
- enumeration: value: `accent5`
- enumeration: value: `accent6`
- enumeration: value: `hlink`
- enumeration: value: `folHlink`
- enumeration: value: `phClr`
- enumeration: value: `dk1`
- enumeration: value: `lt1`
- enumeration: value: `dk2`
- enumeration: value: `lt2`

<a id="a-st_shapetype"></a>

## a:ST_ShapeType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `line`
- enumeration: value: `lineInv`
- enumeration: value: `triangle`
- enumeration: value: `rtTriangle`
- enumeration: value: `rect`
- enumeration: value: `diamond`
- enumeration: value: `parallelogram`
- enumeration: value: `trapezoid`
- enumeration: value: `nonIsoscelesTrapezoid`
- enumeration: value: `pentagon`
- enumeration: value: `hexagon`
- enumeration: value: `heptagon`
- enumeration: value: `octagon`
- enumeration: value: `decagon`
- enumeration: value: `dodecagon`
- enumeration: value: `star4`
- enumeration: value: `star5`
- enumeration: value: `star6`
- enumeration: value: `star7`
- enumeration: value: `star8`
- enumeration: value: `star10`
- enumeration: value: `star12`
- enumeration: value: `star16`
- enumeration: value: `star24`
- enumeration: value: `star32`
- enumeration: value: `roundRect`
- enumeration: value: `round1Rect`
- enumeration: value: `round2SameRect`
- enumeration: value: `round2DiagRect`
- enumeration: value: `snipRoundRect`
- enumeration: value: `snip1Rect`
- enumeration: value: `snip2SameRect`
- enumeration: value: `snip2DiagRect`
- enumeration: value: `plaque`
- enumeration: value: `ellipse`
- enumeration: value: `teardrop`
- enumeration: value: `homePlate`
- enumeration: value: `chevron`
- enumeration: value: `pieWedge`
- enumeration: value: `pie`
- enumeration: value: `blockArc`
- enumeration: value: `donut`
- enumeration: value: `noSmoking`
- enumeration: value: `rightArrow`
- enumeration: value: `leftArrow`
- enumeration: value: `upArrow`
- enumeration: value: `downArrow`
- enumeration: value: `stripedRightArrow`
- enumeration: value: `notchedRightArrow`
- enumeration: value: `bentUpArrow`
- enumeration: value: `leftRightArrow`
- enumeration: value: `upDownArrow`
- enumeration: value: `leftUpArrow`
- enumeration: value: `leftRightUpArrow`
- enumeration: value: `quadArrow`
- enumeration: value: `leftArrowCallout`
- enumeration: value: `rightArrowCallout`
- enumeration: value: `upArrowCallout`
- enumeration: value: `downArrowCallout`
- enumeration: value: `leftRightArrowCallout`
- enumeration: value: `upDownArrowCallout`
- enumeration: value: `quadArrowCallout`
- enumeration: value: `bentArrow`
- enumeration: value: `uturnArrow`
- enumeration: value: `circularArrow`
- enumeration: value: `leftCircularArrow`
- enumeration: value: `leftRightCircularArrow`
- enumeration: value: `curvedRightArrow`
- enumeration: value: `curvedLeftArrow`
- enumeration: value: `curvedUpArrow`
- enumeration: value: `curvedDownArrow`
- enumeration: value: `swooshArrow`
- enumeration: value: `cube`
- enumeration: value: `can`
- enumeration: value: `lightningBolt`
- enumeration: value: `heart`
- enumeration: value: `sun`
- enumeration: value: `moon`
- enumeration: value: `smileyFace`
- enumeration: value: `irregularSeal1`
- enumeration: value: `irregularSeal2`
- enumeration: value: `foldedCorner`
- enumeration: value: `bevel`
- enumeration: value: `frame`
- enumeration: value: `halfFrame`
- enumeration: value: `corner`
- enumeration: value: `diagStripe`
- enumeration: value: `chord`
- enumeration: value: `arc`
- enumeration: value: `leftBracket`
- enumeration: value: `rightBracket`
- enumeration: value: `leftBrace`
- enumeration: value: `rightBrace`
- enumeration: value: `bracketPair`
- enumeration: value: `bracePair`
- enumeration: value: `straightConnector1`
- enumeration: value: `bentConnector2`
- enumeration: value: `bentConnector3`
- enumeration: value: `bentConnector4`
- enumeration: value: `bentConnector5`
- enumeration: value: `curvedConnector2`
- enumeration: value: `curvedConnector3`
- enumeration: value: `curvedConnector4`
- enumeration: value: `curvedConnector5`
- enumeration: value: `callout1`
- enumeration: value: `callout2`
- enumeration: value: `callout3`
- enumeration: value: `accentCallout1`
- enumeration: value: `accentCallout2`
- enumeration: value: `accentCallout3`
- enumeration: value: `borderCallout1`
- enumeration: value: `borderCallout2`
- enumeration: value: `borderCallout3`
- enumeration: value: `accentBorderCallout1`
- enumeration: value: `accentBorderCallout2`
- enumeration: value: `accentBorderCallout3`
- enumeration: value: `wedgeRectCallout`
- enumeration: value: `wedgeRoundRectCallout`
- enumeration: value: `wedgeEllipseCallout`
- enumeration: value: `cloudCallout`
- enumeration: value: `cloud`
- enumeration: value: `ribbon`
- enumeration: value: `ribbon2`
- enumeration: value: `ellipseRibbon`
- enumeration: value: `ellipseRibbon2`
- enumeration: value: `leftRightRibbon`
- enumeration: value: `verticalScroll`
- enumeration: value: `horizontalScroll`
- enumeration: value: `wave`
- enumeration: value: `doubleWave`
- enumeration: value: `plus`
- enumeration: value: `flowChartProcess`
- enumeration: value: `flowChartDecision`
- enumeration: value: `flowChartInputOutput`
- enumeration: value: `flowChartPredefinedProcess`
- enumeration: value: `flowChartInternalStorage`
- enumeration: value: `flowChartDocument`
- enumeration: value: `flowChartMultidocument`
- enumeration: value: `flowChartTerminator`
- enumeration: value: `flowChartPreparation`
- enumeration: value: `flowChartManualInput`
- enumeration: value: `flowChartManualOperation`
- enumeration: value: `flowChartConnector`
- enumeration: value: `flowChartPunchedCard`
- enumeration: value: `flowChartPunchedTape`
- enumeration: value: `flowChartSummingJunction`
- enumeration: value: `flowChartOr`
- enumeration: value: `flowChartCollate`
- enumeration: value: `flowChartSort`
- enumeration: value: `flowChartExtract`
- enumeration: value: `flowChartMerge`
- enumeration: value: `flowChartOfflineStorage`
- enumeration: value: `flowChartOnlineStorage`
- enumeration: value: `flowChartMagneticTape`
- enumeration: value: `flowChartMagneticDisk`
- enumeration: value: `flowChartMagneticDrum`
- enumeration: value: `flowChartDisplay`
- enumeration: value: `flowChartDelay`
- enumeration: value: `flowChartAlternateProcess`
- enumeration: value: `flowChartOffpageConnector`
- enumeration: value: `actionButtonBlank`
- enumeration: value: `actionButtonHome`
- enumeration: value: `actionButtonHelp`
- enumeration: value: `actionButtonInformation`
- enumeration: value: `actionButtonForwardNext`
- enumeration: value: `actionButtonBackPrevious`
- enumeration: value: `actionButtonEnd`
- enumeration: value: `actionButtonBeginning`
- enumeration: value: `actionButtonReturn`
- enumeration: value: `actionButtonDocument`
- enumeration: value: `actionButtonSound`
- enumeration: value: `actionButtonMovie`
- enumeration: value: `gear6`
- enumeration: value: `gear9`
- enumeration: value: `funnel`
- enumeration: value: `mathPlus`
- enumeration: value: `mathMinus`
- enumeration: value: `mathMultiply`
- enumeration: value: `mathDivide`
- enumeration: value: `mathEqual`
- enumeration: value: `mathNotEqual`
- enumeration: value: `cornerTabs`
- enumeration: value: `squareTabs`
- enumeration: value: `plaqueTabs`
- enumeration: value: `chartX`
- enumeration: value: `chartStar`
- enumeration: value: `chartPlus`

<a id="a-st_systemcolorval"></a>

## a:ST_SystemColorVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `scrollBar`
- enumeration: value: `background`
- enumeration: value: `activeCaption`
- enumeration: value: `inactiveCaption`
- enumeration: value: `menu`
- enumeration: value: `window`
- enumeration: value: `windowFrame`
- enumeration: value: `menuText`
- enumeration: value: `windowText`
- enumeration: value: `captionText`
- enumeration: value: `activeBorder`
- enumeration: value: `inactiveBorder`
- enumeration: value: `appWorkspace`
- enumeration: value: `highlight`
- enumeration: value: `highlightText`
- enumeration: value: `btnFace`
- enumeration: value: `btnShadow`
- enumeration: value: `grayText`
- enumeration: value: `btnText`
- enumeration: value: `inactiveCaptionText`
- enumeration: value: `btnHighlight`
- enumeration: value: `3dDkShadow`
- enumeration: value: `3dLight`
- enumeration: value: `infoText`
- enumeration: value: `infoBk`
- enumeration: value: `hotLight`
- enumeration: value: `gradientActiveCaption`
- enumeration: value: `gradientInactiveCaption`
- enumeration: value: `menuHighlight`
- enumeration: value: `menuBar`

<a id="a-st_textaligntype"></a>

## a:ST_TextAlignType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `l`
- enumeration: value: `ctr`
- enumeration: value: `r`
- enumeration: value: `just`
- enumeration: value: `justLow`
- enumeration: value: `dist`
- enumeration: value: `thaiDist`

<a id="a-st_textanchoringtype"></a>

## a:ST_TextAnchoringType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `t`
- enumeration: value: `ctr`
- enumeration: value: `b`
- enumeration: value: `just`
- enumeration: value: `dist`

<a id="a-st_textautonumberscheme"></a>

## a:ST_TextAutonumberScheme

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `alphaLcParenBoth`
- enumeration: value: `alphaUcParenBoth`
- enumeration: value: `alphaLcParenR`
- enumeration: value: `alphaUcParenR`
- enumeration: value: `alphaLcPeriod`
- enumeration: value: `alphaUcPeriod`
- enumeration: value: `arabicParenBoth`
- enumeration: value: `arabicParenR`
- enumeration: value: `arabicPeriod`
- enumeration: value: `arabicPlain`
- enumeration: value: `romanLcParenBoth`
- enumeration: value: `romanUcParenBoth`
- enumeration: value: `romanLcParenR`
- enumeration: value: `romanUcParenR`
- enumeration: value: `romanLcPeriod`
- enumeration: value: `romanUcPeriod`
- enumeration: value: `circleNumDbPlain`
- enumeration: value: `circleNumWdBlackPlain`
- enumeration: value: `circleNumWdWhitePlain`
- enumeration: value: `arabicDbPeriod`
- enumeration: value: `arabicDbPlain`
- enumeration: value: `ea1ChsPeriod`
- enumeration: value: `ea1ChsPlain`
- enumeration: value: `ea1ChtPeriod`
- enumeration: value: `ea1ChtPlain`
- enumeration: value: `ea1JpnChsDbPeriod`
- enumeration: value: `ea1JpnKorPlain`
- enumeration: value: `ea1JpnKorPeriod`
- enumeration: value: `arabic1Minus`
- enumeration: value: `arabic2Minus`
- enumeration: value: `hebrew2Minus`
- enumeration: value: `thaiAlphaPeriod`
- enumeration: value: `thaiAlphaParenR`
- enumeration: value: `thaiAlphaParenBoth`
- enumeration: value: `thaiNumPeriod`
- enumeration: value: `thaiNumParenR`
- enumeration: value: `thaiNumParenBoth`
- enumeration: value: `hindiAlphaPeriod`
- enumeration: value: `hindiNumPeriod`
- enumeration: value: `hindiNumParenR`
- enumeration: value: `hindiAlpha1Period`

<a id="a-st_textbulletsizepercent"></a>

## a:ST_TextBulletSizePercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*((2[5-9])|([3-9][0-9])|([1-3][0-9][0-9])|400)%`

<a id="a-st_textbulletstartatnum"></a>

## a:ST_TextBulletStartAtNum

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`
- minInclusive: value: `1`
- maxInclusive: value: `32767`

<a id="a-st_textcapstype"></a>

## a:ST_TextCapsType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `none`
- enumeration: value: `small`
- enumeration: value: `all`

<a id="a-st_textcolumncount"></a>

## a:ST_TextColumnCount

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`
- minInclusive: value: `1`
- maxInclusive: value: `16`

<a id="a-st_textfontaligntype"></a>

## a:ST_TextFontAlignType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `auto`
- enumeration: value: `t`
- enumeration: value: `ctr`
- enumeration: value: `base`
- enumeration: value: `b`

<a id="a-st_textfontscalepercent"></a>

## a:ST_TextFontScalePercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- restriction: base: [a:ST_PercentageDecimal](values-drawing.md#a-st_percentagedecimal)
- minInclusive: value: `1000`
- maxInclusive: value: `100000`

<a id="a-st_textfontscalepercentorpercentstring"></a>

## a:ST_TextFontScalePercentOrPercentString

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- union: memberTypes: [a:ST_TextFontScalePercent](values-drawing.md#a-st_textfontscalepercent), [s:ST_Percentage](values-shared.md#s-st_percentage)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [s:ST_Percentage](values-shared.md#s-st_percentage)

<a id="a-st_textfontsize"></a>

## a:ST_TextFontSize

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`
- minInclusive: value: `100`
- maxInclusive: value: `400000`

<a id="a-st_texthorzoverflowtype"></a>

## a:ST_TextHorzOverflowType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `overflow`
- enumeration: value: `clip`

<a id="a-st_textindent"></a>

## a:ST_TextIndent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: [a:ST_Coordinate32Unqualified](values-drawing.md#a-st_coordinate32unqualified)
- minInclusive: value: `-51206400`
- maxInclusive: value: `51206400`

<a id="a-st_textindentleveltype"></a>

## a:ST_TextIndentLevelType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`
- minInclusive: value: `0`
- maxInclusive: value: `8`

<a id="a-st_textmargin"></a>

## a:ST_TextMargin

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: [a:ST_Coordinate32Unqualified](values-drawing.md#a-st_coordinate32unqualified)
- minInclusive: value: `0`
- maxInclusive: value: `51206400`

<a id="a-st_textnonnegativepoint"></a>

## a:ST_TextNonNegativePoint

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`
- minInclusive: value: `0`
- maxInclusive: value: `400000`

<a id="a-st_textpoint"></a>

## a:ST_TextPoint

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [a:ST_TextPointUnqualified](values-drawing.md#a-st_textpointunqualified), [s:ST_UniversalMeasure](values-shared.md#s-st_universalmeasure)

<a id="a-st_textpointunqualified"></a>

## a:ST_TextPointUnqualified

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`
- minInclusive: value: `-400000`
- maxInclusive: value: `400000`

<a id="a-st_textshapetype"></a>

## a:ST_TextShapeType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `textNoShape`
- enumeration: value: `textPlain`
- enumeration: value: `textStop`
- enumeration: value: `textTriangle`
- enumeration: value: `textTriangleInverted`
- enumeration: value: `textChevron`
- enumeration: value: `textChevronInverted`
- enumeration: value: `textRingInside`
- enumeration: value: `textRingOutside`
- enumeration: value: `textArchUp`
- enumeration: value: `textArchDown`
- enumeration: value: `textCircle`
- enumeration: value: `textButton`
- enumeration: value: `textArchUpPour`
- enumeration: value: `textArchDownPour`
- enumeration: value: `textCirclePour`
- enumeration: value: `textButtonPour`
- enumeration: value: `textCurveUp`
- enumeration: value: `textCurveDown`
- enumeration: value: `textCanUp`
- enumeration: value: `textCanDown`
- enumeration: value: `textWave1`
- enumeration: value: `textWave2`
- enumeration: value: `textDoubleWave1`
- enumeration: value: `textWave4`
- enumeration: value: `textInflate`
- enumeration: value: `textDeflate`
- enumeration: value: `textInflateBottom`
- enumeration: value: `textDeflateBottom`
- enumeration: value: `textInflateTop`
- enumeration: value: `textDeflateTop`
- enumeration: value: `textDeflateInflate`
- enumeration: value: `textDeflateInflateDeflate`
- enumeration: value: `textFadeRight`
- enumeration: value: `textFadeLeft`
- enumeration: value: `textFadeUp`
- enumeration: value: `textFadeDown`
- enumeration: value: `textSlantUp`
- enumeration: value: `textSlantDown`
- enumeration: value: `textCascadeUp`
- enumeration: value: `textCascadeDown`

<a id="a-st_textspacingpercent"></a>

## a:ST_TextSpacingPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- restriction: base: [a:ST_PercentageDecimal](values-drawing.md#a-st_percentagedecimal)
- minInclusive: value: `0`
- maxInclusive: value: `13200000`

<a id="a-st_textspacingpercentorpercentstring"></a>

## a:ST_TextSpacingPercentOrPercentString

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd):

- union: memberTypes: [a:ST_TextSpacingPercent](values-drawing.md#a-st_textspacingpercent), [s:ST_Percentage](values-shared.md#s-st_percentage)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- union: memberTypes: [s:ST_Percentage](values-shared.md#s-st_percentage)

<a id="a-st_textspacingpoint"></a>

## a:ST_TextSpacingPoint

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:int`
- minInclusive: value: `0`
- maxInclusive: value: `158400`

<a id="a-st_textstriketype"></a>

## a:ST_TextStrikeType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `noStrike`
- enumeration: value: `sngStrike`
- enumeration: value: `dblStrike`

<a id="a-st_texttabaligntype"></a>

## a:ST_TextTabAlignType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `l`
- enumeration: value: `ctr`
- enumeration: value: `r`
- enumeration: value: `dec`

<a id="a-st_texttypeface"></a>

## a:ST_TextTypeface

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:string`

<a id="a-st_textunderlinetype"></a>

## a:ST_TextUnderlineType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `none`
- enumeration: value: `words`
- enumeration: value: `sng`
- enumeration: value: `dbl`
- enumeration: value: `heavy`
- enumeration: value: `dotted`
- enumeration: value: `dottedHeavy`
- enumeration: value: `dash`
- enumeration: value: `dashHeavy`
- enumeration: value: `dashLong`
- enumeration: value: `dashLongHeavy`
- enumeration: value: `dotDash`
- enumeration: value: `dotDashHeavy`
- enumeration: value: `dotDotDash`
- enumeration: value: `dotDotDashHeavy`
- enumeration: value: `wavy`
- enumeration: value: `wavyHeavy`
- enumeration: value: `wavyDbl`

<a id="a-st_textvertoverflowtype"></a>

## a:ST_TextVertOverflowType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `overflow`
- enumeration: value: `ellipsis`
- enumeration: value: `clip`

<a id="a-st_textverticaltype"></a>

## a:ST_TextVerticalType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `horz`
- enumeration: value: `vert`
- enumeration: value: `vert270`
- enumeration: value: `wordArtVert`
- enumeration: value: `eaVert`
- enumeration: value: `mongolianVert`
- enumeration: value: `wordArtVertRtl`

<a id="a-st_textwrappingtype"></a>

## a:ST_TextWrappingType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `none`
- enumeration: value: `square`

<a id="a-st_tileflipmode"></a>

## a:ST_TileFlipMode

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `none`
- enumeration: value: `x`
- enumeration: value: `y`
- enumeration: value: `xy`

<a id="wp-st_alignh"></a>

## wp:ST_AlignH

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `left`
- enumeration: value: `right`
- enumeration: value: `center`
- enumeration: value: `inside`
- enumeration: value: `outside`

<a id="wp-st_alignv"></a>

## wp:ST_AlignV

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `top`
- enumeration: value: `bottom`
- enumeration: value: `center`
- enumeration: value: `inside`
- enumeration: value: `outside`

<a id="wp-st_positionoffset"></a>

## wp:ST_PositionOffset

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd):

- restriction: base: `xsd:int`

<a id="wp-st_relfromh"></a>

## wp:ST_RelFromH

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `margin`
- enumeration: value: `page`
- enumeration: value: `column`
- enumeration: value: `character`
- enumeration: value: `leftMargin`
- enumeration: value: `rightMargin`
- enumeration: value: `insideMargin`
- enumeration: value: `outsideMargin`

<a id="wp-st_relfromv"></a>

## wp:ST_RelFromV

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `margin`
- enumeration: value: `page`
- enumeration: value: `paragraph`
- enumeration: value: `line`
- enumeration: value: `topMargin`
- enumeration: value: `bottomMargin`
- enumeration: value: `insideMargin`
- enumeration: value: `outsideMargin`

<a id="wp-st_wrapdistance"></a>

## wp:ST_WrapDistance

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd):

- restriction: base: `xsd:unsignedInt`

<a id="wp-st_wraptext"></a>

## wp:ST_WrapText

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-wordprocessingDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-wordprocessingDrawing.xsd):

- restriction: base: `xsd:token`
- enumeration: value: `bothSides`
- enumeration: value: `left`
- enumeration: value: `right`
- enumeration: value: `largest`
