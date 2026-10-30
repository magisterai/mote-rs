# Charts value definitions

[Inventory guide](README.md)

These are complete schema value domains for the selected properties.
Property rows record Mote support and supported subsets separately.
Identical Strict and Transitional definitions are combined.
See also [prose-defined extensions](prose-extensions.md).

## Types

- [c14:ST_Style](#c14-st_style)
- [c:ST_AxPos](#c-st_axpos)
- [c:ST_AxisUnit](#c-st_axisunit)
- [c:ST_BarDir](#c-st_bardir)
- [c:ST_BarGrouping](#c-st_bargrouping)
- [c:ST_BubbleScale](#c-st_bubblescale)
- [c:ST_BubbleScalePercent](#c-st_bubblescalepercent)
- [c:ST_BubbleScaleUInt](#c-st_bubblescaleuint)
- [c:ST_BuiltInUnit](#c-st_builtinunit)
- [c:ST_CrossBetween](#c-st_crossbetween)
- [c:ST_Crosses](#c-st_crosses)
- [c:ST_DLblPos](#c-st_dlblpos)
- [c:ST_DepthPercent](#c-st_depthpercent)
- [c:ST_DepthPercentUShort](#c-st_depthpercentushort)
- [c:ST_DepthPercentWithSymbol](#c-st_depthpercentwithsymbol)
- [c:ST_DispBlanksAs](#c-st_dispblanksas)
- [c:ST_ErrBarType](#c-st_errbartype)
- [c:ST_ErrDir](#c-st_errdir)
- [c:ST_ErrValType](#c-st_errvaltype)
- [c:ST_FirstSliceAng](#c-st_firstsliceang)
- [c:ST_GapAmount](#c-st_gapamount)
- [c:ST_GapAmountPercent](#c-st_gapamountpercent)
- [c:ST_GapAmountUShort](#c-st_gapamountushort)
- [c:ST_Grouping](#c-st_grouping)
- [c:ST_HPercent](#c-st_hpercent)
- [c:ST_HPercentUShort](#c-st_hpercentushort)
- [c:ST_HPercentWithSymbol](#c-st_hpercentwithsymbol)
- [c:ST_HoleSize](#c-st_holesize)
- [c:ST_HoleSizePercent](#c-st_holesizepercent)
- [c:ST_HoleSizeUByte](#c-st_holesizeubyte)
- [c:ST_LayoutMode](#c-st_layoutmode)
- [c:ST_LayoutTarget](#c-st_layouttarget)
- [c:ST_LblAlgn](#c-st_lblalgn)
- [c:ST_LblOffset](#c-st_lbloffset)
- [c:ST_LblOffsetPercent](#c-st_lbloffsetpercent)
- [c:ST_LblOffsetUShort](#c-st_lbloffsetushort)
- [c:ST_LegendPos](#c-st_legendpos)
- [c:ST_LogBase](#c-st_logbase)
- [c:ST_MarkerSize](#c-st_markersize)
- [c:ST_MarkerStyle](#c-st_markerstyle)
- [c:ST_OfPieType](#c-st_ofpietype)
- [c:ST_Order](#c-st_order)
- [c:ST_Orientation](#c-st_orientation)
- [c:ST_Overlap](#c-st_overlap)
- [c:ST_OverlapByte](#c-st_overlapbyte)
- [c:ST_OverlapPercent](#c-st_overlappercent)
- [c:ST_PageSetupOrientation](#c-st_pagesetuporientation)
- [c:ST_Period](#c-st_period)
- [c:ST_Perspective](#c-st_perspective)
- [c:ST_PictureFormat](#c-st_pictureformat)
- [c:ST_PictureStackUnit](#c-st_picturestackunit)
- [c:ST_RadarStyle](#c-st_radarstyle)
- [c:ST_RotX](#c-st_rotx)
- [c:ST_RotY](#c-st_roty)
- [c:ST_ScatterStyle](#c-st_scatterstyle)
- [c:ST_SecondPieSize](#c-st_secondpiesize)
- [c:ST_SecondPieSizePercent](#c-st_secondpiesizepercent)
- [c:ST_SecondPieSizeUShort](#c-st_secondpiesizeushort)
- [c:ST_Shape](#c-st_shape)
- [c:ST_SizeRepresents](#c-st_sizerepresents)
- [c:ST_Skip](#c-st_skip)
- [c:ST_SplitType](#c-st_splittype)
- [c:ST_Style](#c-st_style)
- [c:ST_Thickness](#c-st_thickness)
- [c:ST_ThicknessPercent](#c-st_thicknesspercent)
- [c:ST_TickLblPos](#c-st_ticklblpos)
- [c:ST_TickMark](#c-st_tickmark)
- [c:ST_TimeUnit](#c-st_timeunit)
- [c:ST_TrendlineType](#c-st_trendlinetype)
- [c:ST_Xstring](#c-st_xstring)
- [cdr:ST_MarkerCoordinate](#cdr-st_markercoordinate)
- [cs:ST_ColorStyleMethod](#cs-st_colorstylemethod)
- [cs:ST_ColorStyleMethodEnum](#cs-st_colorstylemethodenum)
- [cs:ST_MarkerSize](#cs-st_markersize)
- [cs:ST_MarkerStyle](#cs-st_markerstyle)
- [cs:ST_StyleColorEnum](#cs-st_stylecolorenum)
- [cs:ST_StyleColorVal](#cs-st_stylecolorval)
- [cs:ST_StyleEntryModifier](#cs-st_styleentrymodifier)
- [cs:ST_StyleEntryModifierEnum](#cs-st_styleentrymodifierenum)
- [cs:ST_StyleEntryModifierList](#cs-st_styleentrymodifierlist)
- [cs:ST_StyleReferenceModifier](#cs-st_stylereferencemodifier)
- [cs:ST_StyleReferenceModifierEnum](#cs-st_stylereferencemodifierenum)
- [cs:ST_StyleReferenceModifierList](#cs-st_stylereferencemodifierlist)
- [cx:ST_ArtifactType](#cx-st_artifacttype)
- [cx:ST_AxisId](#cx-st_axisid)
- [cx:ST_AxisUnit](#cx-st_axisunit)
- [cx:ST_BindingType](#cx-st_bindingtype)
- [cx:ST_DataId](#cx-st_dataid)
- [cx:ST_DataLabelPos](#cx-st_datalabelpos)
- [cx:ST_DoubleOrAutomatic](#cx-st_doubleorautomatic)
- [cx:ST_EntityType](#cx-st_entitytype)
- [cx:ST_ExtensionDropMode](#cx-st_extensiondropmode)
- [cx:ST_FormulaDirection](#cx-st_formuladirection)
- [cx:ST_GapWidthRatio](#cx-st_gapwidthratio)
- [cx:ST_GeoMappingLevel](#cx-st_geomappinglevel)
- [cx:ST_GeoProjectionType](#cx-st_geoprojectiontype)
- [cx:ST_IntervalClosedSide](#cx-st_intervalclosedside)
- [cx:ST_NumericDimensionType](#cx-st_numericdimensiontype)
- [cx:ST_PageOrientation](#cx-st_pageorientation)
- [cx:ST_ParentLabelLayout](#cx-st_parentlabellayout)
- [cx:ST_PosAlign](#cx-st_posalign)
- [cx:ST_QuartileMethod](#cx-st_quartilemethod)
- [cx:ST_RegionLabelLayout](#cx-st_regionlabellayout)
- [cx:ST_SeriesLayout](#cx-st_serieslayout)
- [cx:ST_SidePos](#cx-st_sidepos)
- [cx:ST_StringDimensionType](#cx-st_stringdimensiontype)
- [cx:ST_TickMarksType](#cx-st_tickmarkstype)
- [cx:ST_ValueAxisUnit](#cx-st_valueaxisunit)
- [cx:ST_ValueColorPositionCount](#cx-st_valuecolorpositioncount)

<a id="c14-st_style"></a>

## c14:ST_Style

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:unsignedByte`
- minInclusive: value: `101`
- maxInclusive: value: `148`

<a id="c-st_axpos"></a>

## c:ST_AxPos

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `b`
- enumeration: value: `l`
- enumeration: value: `r`
- enumeration: value: `t`

<a id="c-st_axisunit"></a>

## c:ST_AxisUnit

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:double`
- minExclusive: value: `0`

<a id="c-st_bardir"></a>

## c:ST_BarDir

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `bar`
- enumeration: value: `col`

<a id="c-st_bargrouping"></a>

## c:ST_BarGrouping

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `percentStacked`
- enumeration: value: `clustered`
- enumeration: value: `standard`
- enumeration: value: `stacked`

<a id="c-st_bubblescale"></a>

## c:ST_BubbleScale

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_BubbleScalePercent](values-charts.md#c-st_bubblescalepercent), [c:ST_BubbleScaleUInt](values-charts.md#c-st_bubblescaleuint)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_BubbleScalePercent](values-charts.md#c-st_bubblescalepercent)

<a id="c-st_bubblescalepercent"></a>

## c:ST_BubbleScalePercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*(([0-9])|([1-9][0-9])|([1-2][0-9][0-9])|300)%`

<a id="c-st_bubblescaleuint"></a>

## c:ST_BubbleScaleUInt

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:unsignedInt`
- minInclusive: value: `0`
- maxInclusive: value: `300`

<a id="c-st_builtinunit"></a>

## c:ST_BuiltInUnit

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `hundreds`
- enumeration: value: `thousands`
- enumeration: value: `tenThousands`
- enumeration: value: `hundredThousands`
- enumeration: value: `millions`
- enumeration: value: `tenMillions`
- enumeration: value: `hundredMillions`
- enumeration: value: `billions`
- enumeration: value: `trillions`

<a id="c-st_crossbetween"></a>

## c:ST_CrossBetween

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `between`
- enumeration: value: `midCat`

<a id="c-st_crosses"></a>

## c:ST_Crosses

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `autoZero`
- enumeration: value: `max`
- enumeration: value: `min`

<a id="c-st_dlblpos"></a>

## c:ST_DLblPos

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `bestFit`
- enumeration: value: `b`
- enumeration: value: `ctr`
- enumeration: value: `inBase`
- enumeration: value: `inEnd`
- enumeration: value: `l`
- enumeration: value: `outEnd`
- enumeration: value: `r`
- enumeration: value: `t`

<a id="c-st_depthpercent"></a>

## c:ST_DepthPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_DepthPercentWithSymbol](values-charts.md#c-st_depthpercentwithsymbol), [c:ST_DepthPercentUShort](values-charts.md#c-st_depthpercentushort)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_DepthPercentWithSymbol](values-charts.md#c-st_depthpercentwithsymbol)

<a id="c-st_depthpercentushort"></a>

## c:ST_DepthPercentUShort

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:unsignedShort`
- minInclusive: value: `20`
- maxInclusive: value: `2000`

<a id="c-st_depthpercentwithsymbol"></a>

## c:ST_DepthPercentWithSymbol

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*(([2-9][0-9])|([1-9][0-9][0-9])|(1[0-9][0-9][0-9])|2000)%`

<a id="c-st_dispblanksas"></a>

## c:ST_DispBlanksAs

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `span`
- enumeration: value: `gap`
- enumeration: value: `zero`

<a id="c-st_errbartype"></a>

## c:ST_ErrBarType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `both`
- enumeration: value: `minus`
- enumeration: value: `plus`

<a id="c-st_errdir"></a>

## c:ST_ErrDir

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `x`
- enumeration: value: `y`

<a id="c-st_errvaltype"></a>

## c:ST_ErrValType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `cust`
- enumeration: value: `fixedVal`
- enumeration: value: `percentage`
- enumeration: value: `stdDev`
- enumeration: value: `stdErr`

<a id="c-st_firstsliceang"></a>

## c:ST_FirstSliceAng

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedShort`
- minInclusive: value: `0`
- maxInclusive: value: `360`

<a id="c-st_gapamount"></a>

## c:ST_GapAmount

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_GapAmountPercent](values-charts.md#c-st_gapamountpercent), [c:ST_GapAmountUShort](values-charts.md#c-st_gapamountushort)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_GapAmountPercent](values-charts.md#c-st_gapamountpercent)

<a id="c-st_gapamountpercent"></a>

## c:ST_GapAmountPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*(([0-9])|([1-9][0-9])|([1-4][0-9][0-9])|500)%`

<a id="c-st_gapamountushort"></a>

## c:ST_GapAmountUShort

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:unsignedShort`
- minInclusive: value: `0`
- maxInclusive: value: `500`

<a id="c-st_grouping"></a>

## c:ST_Grouping

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `percentStacked`
- enumeration: value: `standard`
- enumeration: value: `stacked`

<a id="c-st_hpercent"></a>

## c:ST_HPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_HPercentWithSymbol](values-charts.md#c-st_hpercentwithsymbol), [c:ST_HPercentUShort](values-charts.md#c-st_hpercentushort)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_HPercentWithSymbol](values-charts.md#c-st_hpercentwithsymbol)

<a id="c-st_hpercentushort"></a>

## c:ST_HPercentUShort

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:unsignedShort`
- minInclusive: value: `5`
- maxInclusive: value: `500`

<a id="c-st_hpercentwithsymbol"></a>

## c:ST_HPercentWithSymbol

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*(([5-9])|([1-9][0-9])|([1-4][0-9][0-9])|500)%`

<a id="c-st_holesize"></a>

## c:ST_HoleSize

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_HoleSizePercent](values-charts.md#c-st_holesizepercent), [c:ST_HoleSizeUByte](values-charts.md#c-st_holesizeubyte)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_HoleSizePercent](values-charts.md#c-st_holesizepercent)

<a id="c-st_holesizepercent"></a>

## c:ST_HoleSizePercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*([1-9]|([1-8][0-9])|90)%`

<a id="c-st_holesizeubyte"></a>

## c:ST_HoleSizeUByte

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:unsignedByte`
- minInclusive: value: `1`
- maxInclusive: value: `90`

<a id="c-st_layoutmode"></a>

## c:ST_LayoutMode

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `edge`
- enumeration: value: `factor`

<a id="c-st_layouttarget"></a>

## c:ST_LayoutTarget

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `inner`
- enumeration: value: `outer`

<a id="c-st_lblalgn"></a>

## c:ST_LblAlgn

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `ctr`
- enumeration: value: `l`
- enumeration: value: `r`

<a id="c-st_lbloffset"></a>

## c:ST_LblOffset

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_LblOffsetPercent](values-charts.md#c-st_lbloffsetpercent), [c:ST_LblOffsetUShort](values-charts.md#c-st_lbloffsetushort)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_LblOffsetPercent](values-charts.md#c-st_lbloffsetpercent)

<a id="c-st_lbloffsetpercent"></a>

## c:ST_LblOffsetPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*(([0-9])|([1-9][0-9])|([1-9][0-9][0-9])|1000)%`

<a id="c-st_lbloffsetushort"></a>

## c:ST_LblOffsetUShort

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:unsignedShort`
- minInclusive: value: `0`
- maxInclusive: value: `1000`

<a id="c-st_legendpos"></a>

## c:ST_LegendPos

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `b`
- enumeration: value: `tr`
- enumeration: value: `l`
- enumeration: value: `r`
- enumeration: value: `t`

<a id="c-st_logbase"></a>

## c:ST_LogBase

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:double`
- minInclusive: value: `2`
- maxInclusive: value: `1000`

<a id="c-st_markersize"></a>

## c:ST_MarkerSize

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedByte`
- minInclusive: value: `2`
- maxInclusive: value: `72`

<a id="c-st_markerstyle"></a>

## c:ST_MarkerStyle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `circle`
- enumeration: value: `dash`
- enumeration: value: `diamond`
- enumeration: value: `dot`
- enumeration: value: `none`
- enumeration: value: `picture`
- enumeration: value: `plus`
- enumeration: value: `square`
- enumeration: value: `star`
- enumeration: value: `triangle`
- enumeration: value: `x`
- enumeration: value: `auto`

<a id="c-st_ofpietype"></a>

## c:ST_OfPieType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `pie`
- enumeration: value: `bar`

<a id="c-st_order"></a>

## c:ST_Order

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedByte`
- minInclusive: value: `2`
- maxInclusive: value: `6`

<a id="c-st_orientation"></a>

## c:ST_Orientation

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `maxMin`
- enumeration: value: `minMax`

<a id="c-st_overlap"></a>

## c:ST_Overlap

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_OverlapPercent](values-charts.md#c-st_overlappercent), [c:ST_OverlapByte](values-charts.md#c-st_overlapbyte)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_OverlapPercent](values-charts.md#c-st_overlappercent)

<a id="c-st_overlapbyte"></a>

## c:ST_OverlapByte

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:byte`
- minInclusive: value: `-100`
- maxInclusive: value: `100`

<a id="c-st_overlappercent"></a>

## c:ST_OverlapPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `(-?0*(([0-9])|([1-9][0-9])|100))%`

<a id="c-st_pagesetuporientation"></a>

## c:ST_PageSetupOrientation

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `default`
- enumeration: value: `portrait`
- enumeration: value: `landscape`

<a id="c-st_period"></a>

## c:ST_Period

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedInt`
- minInclusive: value: `2`

<a id="c-st_perspective"></a>

## c:ST_Perspective

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedByte`
- minInclusive: value: `0`
- maxInclusive: value: `240`

<a id="c-st_pictureformat"></a>

## c:ST_PictureFormat

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `stretch`
- enumeration: value: `stack`
- enumeration: value: `stackScale`

<a id="c-st_picturestackunit"></a>

## c:ST_PictureStackUnit

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:double`
- minExclusive: value: `0`

<a id="c-st_radarstyle"></a>

## c:ST_RadarStyle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `standard`
- enumeration: value: `marker`
- enumeration: value: `filled`

<a id="c-st_rotx"></a>

## c:ST_RotX

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:byte`
- minInclusive: value: `-90`
- maxInclusive: value: `90`

<a id="c-st_roty"></a>

## c:ST_RotY

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedShort`
- minInclusive: value: `0`
- maxInclusive: value: `360`

<a id="c-st_scatterstyle"></a>

## c:ST_ScatterStyle

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `line`
- enumeration: value: `lineMarker`
- enumeration: value: `marker`
- enumeration: value: `smooth`
- enumeration: value: `smoothMarker`

<a id="c-st_secondpiesize"></a>

## c:ST_SecondPieSize

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_SecondPieSizePercent](values-charts.md#c-st_secondpiesizepercent), [c:ST_SecondPieSizeUShort](values-charts.md#c-st_secondpiesizeushort)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_SecondPieSizePercent](values-charts.md#c-st_secondpiesizepercent)

<a id="c-st_secondpiesizepercent"></a>

## c:ST_SecondPieSizePercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*(([5-9])|([1-9][0-9])|(1[0-9][0-9])|200)%`

<a id="c-st_secondpiesizeushort"></a>

## c:ST_SecondPieSizeUShort

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- restriction: base: `xsd:unsignedShort`
- minInclusive: value: `5`
- maxInclusive: value: `200`

<a id="c-st_shape"></a>

## c:ST_Shape

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `cone`
- enumeration: value: `coneToMax`
- enumeration: value: `box`
- enumeration: value: `cylinder`
- enumeration: value: `pyramid`
- enumeration: value: `pyramidToMax`

<a id="c-st_sizerepresents"></a>

## c:ST_SizeRepresents

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `area`
- enumeration: value: `w`

<a id="c-st_skip"></a>

## c:ST_Skip

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedInt`
- minInclusive: value: `1`

<a id="c-st_splittype"></a>

## c:ST_SplitType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `auto`
- enumeration: value: `cust`
- enumeration: value: `percent`
- enumeration: value: `pos`
- enumeration: value: `val`

<a id="c-st_style"></a>

## c:ST_Style

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:unsignedByte`
- minInclusive: value: `1`
- maxInclusive: value: `48`

<a id="c-st_thickness"></a>

## c:ST_Thickness

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd):

- union: memberTypes: [c:ST_ThicknessPercent](values-charts.md#c-st_thicknesspercent), `xsd:unsignedInt`

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- union: memberTypes: [c:ST_ThicknessPercent](values-charts.md#c-st_thicknesspercent)

<a id="c-st_thicknesspercent"></a>

## c:ST_ThicknessPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- pattern: value: `([0-9]+)%`

<a id="c-st_ticklblpos"></a>

## c:ST_TickLblPos

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `high`
- enumeration: value: `low`
- enumeration: value: `nextTo`
- enumeration: value: `none`

<a id="c-st_tickmark"></a>

## c:ST_TickMark

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `cross`
- enumeration: value: `in`
- enumeration: value: `none`
- enumeration: value: `out`

<a id="c-st_timeunit"></a>

## c:ST_TimeUnit

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `days`
- enumeration: value: `months`
- enumeration: value: `years`

<a id="c-st_trendlinetype"></a>

## c:ST_TrendlineType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `exp`
- enumeration: value: `linear`
- enumeration: value: `log`
- enumeration: value: `movingAvg`
- enumeration: value: `poly`
- enumeration: value: `power`

<a id="c-st_xstring"></a>

## c:ST_Xstring

Publisher imports use this legacy name; the corresponding shared
value definition is `s:ST_Xstring` in the retained ECMA schemas.

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`

<a id="cdr-st_markercoordinate"></a>

## cdr:ST_MarkerCoordinate

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd):

- restriction: base: `xsd:double`
- minInclusive: value: `0.0`
- maxInclusive: value: `1.0`

<a id="cs-st_colorstylemethod"></a>

## cs:ST_ColorStyleMethod

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: memberTypes: [cs:ST_ColorStyleMethodEnum](values-charts.md#cs-st_colorstylemethodenum), `xsd:string`

<a id="cs-st_colorstylemethodenum"></a>

## cs:ST_ColorStyleMethodEnum

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `cycle`
- enumeration: value: `withinLinear`
- enumeration: value: `acrossLinear`
- enumeration: value: `withinLinearReversed`
- enumeration: value: `acrossLinearReversed`

<a id="cs-st_markersize"></a>

## cs:ST_MarkerSize

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:unsignedByte`
- minInclusive: value: `2`
- maxInclusive: value: `72`

<a id="cs-st_markerstyle"></a>

## cs:ST_MarkerStyle

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `circle`
- enumeration: value: `dash`
- enumeration: value: `diamond`
- enumeration: value: `dot`
- enumeration: value: `plus`
- enumeration: value: `square`
- enumeration: value: `star`
- enumeration: value: `triangle`
- enumeration: value: `x`

<a id="cs-st_stylecolorenum"></a>

## cs:ST_StyleColorEnum

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `auto`

<a id="cs-st_stylecolorval"></a>

## cs:ST_StyleColorVal

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: memberTypes: `xsd:unsignedInt`, [cs:ST_StyleColorEnum](values-charts.md#cs-st_stylecolorenum), `xsd:string`

<a id="cs-st_styleentrymodifier"></a>

## cs:ST_StyleEntryModifier

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: memberTypes: [cs:ST_StyleEntryModifierEnum](values-charts.md#cs-st_styleentrymodifierenum), `xsd:string`

<a id="cs-st_styleentrymodifierenum"></a>

## cs:ST_StyleEntryModifierEnum

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `allowNoFillOverride`
- enumeration: value: `allowNoLineOverride`

<a id="cs-st_styleentrymodifierlist"></a>

## cs:ST_StyleEntryModifierList

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [cs:ST_StyleEntryModifier](values-charts.md#cs-st_styleentrymodifier)

<a id="cs-st_stylereferencemodifier"></a>

## cs:ST_StyleReferenceModifier

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: memberTypes: [cs:ST_StyleReferenceModifierEnum](values-charts.md#cs-st_stylereferencemodifierenum), `xsd:string`

<a id="cs-st_stylereferencemodifierenum"></a>

## cs:ST_StyleReferenceModifierEnum

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `ignoreCSTransforms`

<a id="cs-st_stylereferencemodifierlist"></a>

## cs:ST_StyleReferenceModifierList

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [cs:ST_StyleReferenceModifier](values-charts.md#cs-st_stylereferencemodifier)

<a id="cx-st_artifacttype"></a>

## cx:ST_ArtifactType

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `html`

<a id="cx-st_axisid"></a>

## cx:ST_AxisId

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:unsignedInt`

<a id="cx-st_axisunit"></a>

## cx:ST_AxisUnit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `hundreds`
- enumeration: value: `thousands`
- enumeration: value: `tenThousands`
- enumeration: value: `hundredThousands`
- enumeration: value: `millions`
- enumeration: value: `tenMillions`
- enumeration: value: `hundredMillions`
- enumeration: value: `billions`
- enumeration: value: `trillions`
- enumeration: value: `percentage`

<a id="cx-st_bindingtype"></a>

## cx:ST_BindingType

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: memberTypes: [cx:ST_NumericDimensionType](values-charts.md#cx-st_numericdimensiontype), [cx:ST_StringDimensionType](values-charts.md#cx-st_stringdimensiontype)

<a id="cx-st_dataid"></a>

## cx:ST_DataId

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:unsignedInt`

<a id="cx-st_datalabelpos"></a>

## cx:ST_DataLabelPos

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `bestFit`
- enumeration: value: `b`
- enumeration: value: `ctr`
- enumeration: value: `inBase`
- enumeration: value: `inEnd`
- enumeration: value: `l`
- enumeration: value: `outEnd`
- enumeration: value: `r`
- enumeration: value: `t`

<a id="cx-st_doubleorautomatic"></a>

## cx:ST_DoubleOrAutomatic

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: memberTypes: `xsd:double`
- restriction: base: `xsd:string`
- enumeration: value: `auto`

<a id="cx-st_entitytype"></a>

## cx:ST_EntityType

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `Address`
- enumeration: value: `AdminDistrict`
- enumeration: value: `AdminDistrict2`
- enumeration: value: `AdminDistrict3`
- enumeration: value: `Continent`
- enumeration: value: `CountryRegion`
- enumeration: value: `Locality`
- enumeration: value: `Ocean`
- enumeration: value: `Planet`
- enumeration: value: `PostalCode`
- enumeration: value: `Region`
- enumeration: value: `Unsupported`

<a id="cx-st_extensiondropmode"></a>

## cx:ST_ExtensionDropMode

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `never`
- enumeration: value: `onModelChange`
- enumeration: value: `onDataChange`
- enumeration: value: `always`

<a id="cx-st_formuladirection"></a>

## cx:ST_FormulaDirection

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `col`
- enumeration: value: `row`

<a id="cx-st_gapwidthratio"></a>

## cx:ST_GapWidthRatio

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: 
- restriction: base: `xsd:double`
- minInclusive: value: `0`
- restriction: base: `xsd:string`
- enumeration: value: `auto`

<a id="cx-st_geomappinglevel"></a>

## cx:ST_GeoMappingLevel

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `dataOnly`
- enumeration: value: `postalCode`
- enumeration: value: `county`
- enumeration: value: `state`
- enumeration: value: `countryRegion`
- enumeration: value: `countryRegionList`
- enumeration: value: `world`

<a id="cx-st_geoprojectiontype"></a>

## cx:ST_GeoProjectionType

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `mercator`
- enumeration: value: `miller`
- enumeration: value: `robinson`
- enumeration: value: `albers`

<a id="cx-st_intervalclosedside"></a>

## cx:ST_IntervalClosedSide

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `l`
- enumeration: value: `r`

<a id="cx-st_numericdimensiontype"></a>

## cx:ST_NumericDimensionType

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `val`
- enumeration: value: `x`
- enumeration: value: `y`
- enumeration: value: `size`
- enumeration: value: `colorVal`

<a id="cx-st_pageorientation"></a>

## cx:ST_PageOrientation

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `default`
- enumeration: value: `portrait`
- enumeration: value: `landscape`

<a id="cx-st_parentlabellayout"></a>

## cx:ST_ParentLabelLayout

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `banner`
- enumeration: value: `overlapping`

<a id="cx-st_posalign"></a>

## cx:ST_PosAlign

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `min`
- enumeration: value: `ctr`
- enumeration: value: `max`

<a id="cx-st_quartilemethod"></a>

## cx:ST_QuartileMethod

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `inclusive`
- enumeration: value: `exclusive`

<a id="cx-st_regionlabellayout"></a>

## cx:ST_RegionLabelLayout

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `bestFitOnly`
- enumeration: value: `showAll`

<a id="cx-st_serieslayout"></a>

## cx:ST_SeriesLayout

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `boxWhisker`
- enumeration: value: `clusteredColumn`
- enumeration: value: `funnel`
- enumeration: value: `paretoLine`
- enumeration: value: `regionMap`
- enumeration: value: `sunburst`
- enumeration: value: `treemap`
- enumeration: value: `waterfall`

<a id="cx-st_sidepos"></a>

## cx:ST_SidePos

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `l`
- enumeration: value: `t`
- enumeration: value: `r`
- enumeration: value: `b`

<a id="cx-st_stringdimensiontype"></a>

## cx:ST_StringDimensionType

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `cat`
- enumeration: value: `colorStr`
- enumeration: value: `entityId`

<a id="cx-st_tickmarkstype"></a>

## cx:ST_TickMarksType

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `in`
- enumeration: value: `out`
- enumeration: value: `cross`
- enumeration: value: `none`

<a id="cx-st_valueaxisunit"></a>

## cx:ST_ValueAxisUnit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- union: 
- restriction: base: `xsd:double`
- minExclusive: value: `0`
- restriction: base: `xsd:string`
- enumeration: value: `auto`

<a id="cx-st_valuecolorpositioncount"></a>

## cx:ST_ValueColorPositionCount

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:int`
- minInclusive: value: `2`
- maxInclusive: value: `3`
