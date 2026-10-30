# DrawingML charts

[Inventory guide](README.md)

Complete chart-space, chart-kind, series, caches, labels, axes, legend, layout, style, printing and chart-drawing property families. Every unsupported chart kind and property remains visible. Embedded workbook packaging is an output detail; spreadsheet authoring is outside this directive.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [c:CT_ChartSpace](#c-ct_chartspace)
- [cdr:CT_Drawing](#cdr-ct_drawing)
- [c:CT_Boolean](#c-ct_boolean)
- [c:CT_TextLanguageID](#c-ct_textlanguageid)
- [c:CT_Style](#c-ct_style)
- [a:CT_ColorMapping](#a-ct_colormapping)
- [c:CT_PivotSource](#c-ct_pivotsource)
- [c:CT_Protection](#c-ct_protection)
- [c:CT_Chart](#c-ct_chart)
- [c:CT_ExternalData](#c-ct_externaldata)
- [c:CT_PrintSettings](#c-ct_printsettings)
- [c:CT_RelId](#c-ct_relid)
- [c:CT_ExtensionList](#c-ct_extensionlist)
- [cdr:CT_RelSizeAnchor](#cdr-ct_relsizeanchor)
- [cdr:CT_AbsSizeAnchor](#cdr-ct_abssizeanchor)
- [c:CT_Title](#c-ct_title)
- [c:CT_PivotFmts](#c-ct_pivotfmts)
- [c:CT_View3D](#c-ct_view3d)
- [c:CT_Surface](#c-ct_surface)
- [c:CT_PlotArea](#c-ct_plotarea)
- [c:CT_Legend](#c-ct_legend)
- [c:CT_DispBlanksAs](#c-ct_dispblanksas)
- [c:CT_HeaderFooter](#c-ct_headerfooter)
- [c:CT_PageMargins](#c-ct_pagemargins)
- [c:CT_PageSetup](#c-ct_pagesetup)
- [c:CT_Extension](#c-ct_extension)
- [cdr:CT_Marker](#cdr-ct_marker)
- [cdr:CT_Shape](#cdr-ct_shape)
- [cdr:CT_GroupShape](#cdr-ct_groupshape)
- [cdr:CT_GraphicFrame](#cdr-ct_graphicframe)
- [cdr:CT_Connector](#cdr-ct_connector)
- [cdr:CT_Picture](#cdr-ct_picture)
- [c:CT_PivotFmt](#c-ct_pivotfmt)
- [c:CT_RotX](#c-ct_rotx)
- [c:CT_HPercent](#c-ct_hpercent)
- [c:CT_RotY](#c-ct_roty)
- [c:CT_DepthPercent](#c-ct_depthpercent)
- [c:CT_Perspective](#c-ct_perspective)
- [c:CT_Thickness](#c-ct_thickness)
- [c:CT_PictureOptions](#c-ct_pictureoptions)
- [c:CT_AreaChart](#c-ct_areachart)
- [c:CT_Area3DChart](#c-ct_area3dchart)
- [c:CT_LineChart](#c-ct_linechart)
- [c:CT_Line3DChart](#c-ct_line3dchart)
- [c:CT_StockChart](#c-ct_stockchart)
- [c:CT_RadarChart](#c-ct_radarchart)
- [c:CT_ScatterChart](#c-ct_scatterchart)
- [c:CT_PieChart](#c-ct_piechart)
- [c:CT_Pie3DChart](#c-ct_pie3dchart)
- [c:CT_DoughnutChart](#c-ct_doughnutchart)
- [c:CT_BarChart](#c-ct_barchart)
- [c:CT_Bar3DChart](#c-ct_bar3dchart)
- [c:CT_OfPieChart](#c-ct_ofpiechart)
- [c:CT_SurfaceChart](#c-ct_surfacechart)
- [c:CT_Surface3DChart](#c-ct_surface3dchart)
- [c:CT_BubbleChart](#c-ct_bubblechart)
- [c:CT_ValAx](#c-ct_valax)
- [c:CT_CatAx](#c-ct_catax)
- [c:CT_DateAx](#c-ct_dateax)
- [c:CT_SerAx](#c-ct_serax)
- [c:CT_DTable](#c-ct_dtable)
- [c:CT_LegendPos](#c-ct_legendpos)
- [c:CT_LegendEntry](#c-ct_legendentry)
- [cdr:CT_ShapeNonVisual](#cdr-ct_shapenonvisual)
- [cdr:CT_GroupShapeNonVisual](#cdr-ct_groupshapenonvisual)
- [a:CT_GroupShapeProperties](#a-ct_groupshapeproperties)
- [cdr:CT_GraphicFrameNonVisual](#cdr-ct_graphicframenonvisual)
- [cdr:CT_ConnectorNonVisual](#cdr-ct_connectornonvisual)
- [cdr:CT_PictureNonVisual](#cdr-ct_picturenonvisual)
- [c:CT_PictureFormat](#c-ct_pictureformat)
- [c:CT_PictureStackUnit](#c-ct_picturestackunit)
- [c:CT_Grouping](#c-ct_grouping)
- [c:CT_GapAmount](#c-ct_gapamount)
- [c:CT_UpDownBars](#c-ct_updownbars)
- [c:CT_RadarStyle](#c-ct_radarstyle)
- [c:CT_ScatterStyle](#c-ct_scatterstyle)
- [c:CT_FirstSliceAng](#c-ct_firstsliceang)
- [c:CT_HoleSize](#c-ct_holesize)
- [c:CT_BarDir](#c-ct_bardir)
- [c:CT_BarGrouping](#c-ct_bargrouping)
- [c:CT_Overlap](#c-ct_overlap)
- [c:CT_OfPieType](#c-ct_ofpietype)
- [c:CT_SplitType](#c-ct_splittype)
- [c:CT_CustSplit](#c-ct_custsplit)
- [c:CT_SecondPieSize](#c-ct_secondpiesize)
- [c:CT_BandFmts](#c-ct_bandfmts)
- [c:CT_BubbleScale](#c-ct_bubblescale)
- [c:CT_SizeRepresents](#c-ct_sizerepresents)
- [c:CT_Scaling](#c-ct_scaling)
- [c:CT_AxPos](#c-ct_axpos)
- [c:CT_TickMark](#c-ct_tickmark)
- [c:CT_TickLblPos](#c-ct_ticklblpos)
- [c:CT_Crosses](#c-ct_crosses)
- [c:CT_CrossBetween](#c-ct_crossbetween)
- [c:CT_AxisUnit](#c-ct_axisunit)
- [c:CT_DispUnits](#c-ct_dispunits)
- [c:CT_LblAlgn](#c-ct_lblalgn)
- [c:CT_LblOffset](#c-ct_lbloffset)
- [c:CT_Skip](#c-ct_skip)
- [c:CT_TimeUnit](#c-ct_timeunit)
- [a:CT_NonVisualGroupDrawingShapeProps](#a-ct_nonvisualgroupdrawingshapeprops)
- [a:CT_GroupTransform2D](#a-ct_grouptransform2d)
- [c:CT_UpDownBar](#c-ct_updownbar)
- [c:CT_BandFmt](#c-ct_bandfmt)
- [c:CT_LogBase](#c-ct_logbase)
- [c:CT_Orientation](#c-ct_orientation)
- [c:CT_BuiltInUnit](#c-ct_builtinunit)
- [c:CT_DispUnitsLbl](#c-ct_dispunitslbl)
- [a:CT_GroupLocking](#a-ct_grouplocking)

<a id="c-ct_chartspace"></a>

## c:CT_ChartSpace

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `date1904` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. 1904 Date System; [§21.2.2.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `lang` — [c:CT_TextLanguageID](charts.md#c-ct_textlanguageid); Strict, Transitional; minOccurs=0; maxOccurs=1. Editing Language; [§21.2.2.87](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `roundedCorners` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Rounded Corners; [§21.2.2.159](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3432-part-1).
- [ ] `style` — [c:CT_Style](charts.md#c-ct_style); Strict, Transitional; minOccurs=0; maxOccurs=1. Style; [§21.2.2.202](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3444-part-1).
- [ ] `clrMapOvr` — [a:CT_ColorMapping](charts.md#a-ct_colormapping); Strict, Transitional; minOccurs=0; maxOccurs=1. Color Map Override; [§21.2.2.30](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3389-part-1).
- [ ] `pivotSource` — [c:CT_PivotSource](charts.md#c-ct_pivotsource); Strict, Transitional; minOccurs=0; maxOccurs=1. Pivot Source; [§21.2.2.144](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3428-part-1).
- [ ] `protection` — [c:CT_Protection](charts.md#c-ct_protection); Strict, Transitional; minOccurs=0; maxOccurs=1. Protection; [§21.2.2.149](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3429-part-1).
- [ ] `chart` — [c:CT_Chart](charts.md#c-ct_chart); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `externalData` — [c:CT_ExternalData](charts.md#c-ct_externaldata); Strict, Transitional; minOccurs=0; maxOccurs=1. External Data Relationship; [§21.2.2.63](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).
- [ ] `printSettings` — [c:CT_PrintSettings](charts.md#c-ct_printsettings); Strict, Transitional; minOccurs=0; maxOccurs=1. Print Settings; [§21.2.2.148](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3429-part-1).
- [ ] `userShapes` — [c:CT_RelId](charts.md#c-ct_relid); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="cdr-ct_drawing"></a>

## cdr:CT_Drawing

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `relSizeAnchor` — [cdr:CT_RelSizeAnchor](charts.md#cdr-ct_relsizeanchor); Strict, Transitional.
- [ ] `absSizeAnchor` — [cdr:CT_AbsSizeAnchor](charts.md#cdr-ct_abssizeanchor); Strict, Transitional.

<a id="c-ct_boolean"></a>

## c:CT_Boolean

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="c-ct_textlanguageid"></a>

## c:CT_TextLanguageID

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [s:ST_Lang](values-shared.md#s-st_lang); Strict, Transitional; use=required.

<a id="c-ct_style"></a>

## c:CT_Style

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Style](values-charts.md#c-st_style); Strict, Transitional; use=required.

<a id="a-ct_colormapping"></a>

## a:CT_ColorMapping

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@bg1` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@tx1` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@bg2` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@tx2` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@accent1` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@accent2` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@accent3` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@accent4` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@accent5` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@accent6` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@hlink` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.
- [ ] `@folHlink` — [a:ST_ColorSchemeIndex](values-drawing.md#a-st_colorschemeindex); Strict, Transitional; use=required.

<a id="c-ct_pivotsource"></a>

## c:CT_PivotSource

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `name` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fmtId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Format ID; [§21.2.2.70](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3403-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_protection"></a>

## c:CT_Protection

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `chartObject` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Object; [§21.2.2.28](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3389-part-1).
- [ ] `data` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Cannot Be Changed; [§21.2.2.37](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3393-part-1).
- [ ] `formatting` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Formatting; [§21.2.2.72](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3404-part-1).
- [ ] `selection` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Selection; [§21.2.2.165](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3434-part-1).
- [ ] `userInterface` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. User Interface; [§21.2.2.219](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3448-part-1).

<a id="c-ct_chart"></a>

## c:CT_Chart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `title` — [c:CT_Title](charts.md#c-ct_title); Strict, Transitional; minOccurs=0; maxOccurs=1. Title; [§21.2.2.210](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `autoTitleDeleted` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Auto Title Is Deleted; [§21.2.2.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3383-part-1).
- [ ] `pivotFmts` — [c:CT_PivotFmts](charts.md#c-ct_pivotfmts); Strict, Transitional; minOccurs=0; maxOccurs=1. Pivot Formats; [§21.2.2.143](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3428-part-1).
- [ ] `view3D` — [c:CT_View3D](charts.md#c-ct_view3d); Strict, Transitional; minOccurs=0; maxOccurs=1. View In 3D; [§21.2.2.228](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `floor` — [c:CT_Surface](charts.md#c-ct_surface); Strict, Transitional; minOccurs=0; maxOccurs=1. Floor; [§21.2.2.69](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3403-part-1).
- [ ] `sideWall` — [c:CT_Surface](charts.md#c-ct_surface); Strict, Transitional; minOccurs=0; maxOccurs=1. Side Wall; [§21.2.2.191](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3441-part-1).
- [ ] `backWall` — [c:CT_Surface](charts.md#c-ct_surface); Strict, Transitional; minOccurs=0; maxOccurs=1. Back Wall; [§21.2.2.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3385-part-1).
- [ ] `plotArea` — [c:CT_PlotArea](charts.md#c-ct_plotarea); Strict, Transitional; minOccurs=1; maxOccurs=1. Plot Area; [§21.2.2.145](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3428-part-1).
- [ ] `legend` — [c:CT_Legend](charts.md#c-ct_legend); Strict, Transitional; minOccurs=0; maxOccurs=1. Legend; [§21.2.2.93](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3411-part-1).
- [ ] `plotVisOnly` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Plot Visible Only; [§21.2.2.146](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3428-part-1).
- [ ] `dispBlanksAs` — [c:CT_DispBlanksAs](charts.md#c-ct_dispblanksas); Strict, Transitional; minOccurs=0; maxOccurs=1. Display Blanks As; [§21.2.2.42](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3395-part-1).
- [ ] `showDLblsOverMax` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Data Labels over Maximum; [§21.2.2.180](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3437-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_externaldata"></a>

## c:CT_ExternalData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `autoUpdate` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Update Automatically; [§21.2.2.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="c-ct_printsettings"></a>

## c:CT_PrintSettings

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `headerFooter` — [c:CT_HeaderFooter](charts.md#c-ct_headerfooter); Strict, Transitional; minOccurs=0; maxOccurs=1. Header and Footer; [§21.2.2.79](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3406-part-1).
- [ ] `pageMargins` — [c:CT_PageMargins](charts.md#c-ct_pagemargins); Strict, Transitional; minOccurs=0; maxOccurs=1. Page Margins; [§21.2.2.133](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3422-part-1).
- [ ] `pageSetup` — [c:CT_PageSetup](charts.md#c-ct_pagesetup); Strict, Transitional; minOccurs=0; maxOccurs=1. Page Setup; [§21.2.2.134](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3422-part-1).
- [ ] `legacyDrawingHF` — [c:CT_RelId](charts.md#c-ct_relid); Transitional; minOccurs=0; maxOccurs=1.

<a id="c-ct_relid"></a>

## c:CT_RelId

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="c-ct_extensionlist"></a>

## c:CT_ExtensionList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `ext` — [c:CT_Extension](charts.md#c-ct_extension); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Extension; [§21.2.2.62](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3400-part-1).

<a id="cdr-ct_relsizeanchor"></a>

## cdr:CT_RelSizeAnchor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `from` — [cdr:CT_Marker](charts.md#cdr-ct_marker); Strict, Transitional.
- [ ] `to` — [cdr:CT_Marker](charts.md#cdr-ct_marker); Strict, Transitional.
- [ ] `sp` — [cdr:CT_Shape](charts.md#cdr-ct_shape); Strict, Transitional.
- [ ] `grpSp` — [cdr:CT_GroupShape](charts.md#cdr-ct_groupshape); Strict, Transitional.
- [ ] `graphicFrame` — [cdr:CT_GraphicFrame](charts.md#cdr-ct_graphicframe); Strict, Transitional.
- [ ] `cxnSp` — [cdr:CT_Connector](charts.md#cdr-ct_connector); Strict, Transitional.
- [ ] `pic` — [cdr:CT_Picture](charts.md#cdr-ct_picture); Strict, Transitional.

<a id="cdr-ct_abssizeanchor"></a>

## cdr:CT_AbsSizeAnchor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `from` — [cdr:CT_Marker](charts.md#cdr-ct_marker); Strict, Transitional.
- [ ] `ext` — [a:CT_PositiveSize2D](images.md#a-ct_positivesize2d); Strict, Transitional.
- [ ] `sp` — [cdr:CT_Shape](charts.md#cdr-ct_shape); Strict, Transitional.
- [ ] `grpSp` — [cdr:CT_GroupShape](charts.md#cdr-ct_groupshape); Strict, Transitional.
- [ ] `graphicFrame` — [cdr:CT_GraphicFrame](charts.md#cdr-ct_graphicframe); Strict, Transitional.
- [ ] `cxnSp` — [cdr:CT_Connector](charts.md#cdr-ct_connector); Strict, Transitional.
- [ ] `pic` — [cdr:CT_Picture](charts.md#cdr-ct_picture); Strict, Transitional.

<a id="c-ct_title"></a>

## c:CT_Title

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `tx` — [c:CT_Tx](chart-extensions.md#c-ct_tx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `layout` — [c:CT_Layout](chart-extensions.md#c-ct_layout); Strict, Transitional; minOccurs=0; maxOccurs=1. Layout; [§21.2.2.88](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `overlay` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Overlay; [§21.2.2.132](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3421-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_pivotfmts"></a>

## c:CT_PivotFmts

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `pivotFmt` — [c:CT_PivotFmt](charts.md#c-ct_pivotfmt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Pivot Format; [§21.2.2.142](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3428-part-1).

<a id="c-ct_view3d"></a>

## c:CT_View3D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `rotX` — [c:CT_RotX](charts.md#c-ct_rotx); Strict, Transitional; minOccurs=0; maxOccurs=1. X Rotation; [§21.2.2.157](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3431-part-1).
- [ ] `hPercent` — [c:CT_HPercent](charts.md#c-ct_hpercent); Strict, Transitional; minOccurs=0; maxOccurs=1. Height Percent; [§21.2.2.83](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `rotY` — [c:CT_RotY](charts.md#c-ct_roty); Strict, Transitional; minOccurs=0; maxOccurs=1. Y Rotation; [§21.2.2.158](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3432-part-1).
- [ ] `depthPercent` — [c:CT_DepthPercent](charts.md#c-ct_depthpercent); Strict, Transitional; minOccurs=0; maxOccurs=1. Depth Percent; [§21.2.2.41](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3395-part-1).
- [ ] `rAngAx` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Right Angle Axes; [§21.2.2.155](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3431-part-1).
- [ ] `perspective` — [c:CT_Perspective](charts.md#c-ct_perspective); Strict, Transitional; minOccurs=0; maxOccurs=1. Perspective; [§21.2.2.136](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3426-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_surface"></a>

## c:CT_Surface

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `thickness` — [c:CT_Thickness](charts.md#c-ct_thickness); Strict, Transitional; minOccurs=0; maxOccurs=1. Thickness; [§21.2.2.206](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3445-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `pictureOptions` — [c:CT_PictureOptions](charts.md#c-ct_pictureoptions); Strict, Transitional; minOccurs=0; maxOccurs=1. Picture Options; [§21.2.2.138](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3427-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_plotarea"></a>

## c:CT_PlotArea

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `layout` — [c:CT_Layout](chart-extensions.md#c-ct_layout); Strict, Transitional; minOccurs=0; maxOccurs=1. Layout; [§21.2.2.88](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `areaChart` — [c:CT_AreaChart](charts.md#c-ct_areachart); Strict, Transitional; minOccurs=1; maxOccurs=1. Area Charts; [§21.2.2.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3383-part-1).
- [ ] `area3DChart` — [c:CT_Area3DChart](charts.md#c-ct_area3dchart); Strict, Transitional; minOccurs=1; maxOccurs=1. 3D Area Charts; [§21.2.2.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3382-part-1).
- [ ] `lineChart` — [c:CT_LineChart](charts.md#c-ct_linechart); Strict, Transitional; minOccurs=1; maxOccurs=1. Line Charts; [§21.2.2.97](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `line3DChart` — [c:CT_Line3DChart](charts.md#c-ct_line3dchart); Strict, Transitional; minOccurs=1; maxOccurs=1. 3D Line Charts; [§21.2.2.96](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `stockChart` — [c:CT_StockChart](charts.md#c-ct_stockchart); Strict, Transitional; minOccurs=1; maxOccurs=1. Stock Charts; [§21.2.2.198](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3444-part-1).
- [ ] `radarChart` — [c:CT_RadarChart](charts.md#c-ct_radarchart); Strict, Transitional; minOccurs=1; maxOccurs=1. Radar Charts; [§21.2.2.153](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3430-part-1).
- [ ] `scatterChart` — [c:CT_ScatterChart](charts.md#c-ct_scatterchart); Strict, Transitional; minOccurs=1; maxOccurs=1. Scatter Charts; [§21.2.2.161](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3433-part-1).
- [ ] `pieChart` — [c:CT_PieChart](charts.md#c-ct_piechart); Strict, Transitional; minOccurs=1; maxOccurs=1. Pie Charts; [§21.2.2.141](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3428-part-1).
- [ ] `pie3DChart` — [c:CT_Pie3DChart](charts.md#c-ct_pie3dchart); Strict, Transitional; minOccurs=1; maxOccurs=1. 3D Pie Charts; [§21.2.2.140](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3428-part-1).
- [ ] `doughnutChart` — [c:CT_DoughnutChart](charts.md#c-ct_doughnutchart); Strict, Transitional; minOccurs=1; maxOccurs=1. Doughnut Charts; [§21.2.2.50](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `barChart` — [c:CT_BarChart](charts.md#c-ct_barchart); Strict, Transitional; minOccurs=1; maxOccurs=1. Bar Charts; [§21.2.2.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `bar3DChart` — [c:CT_Bar3DChart](charts.md#c-ct_bar3dchart); Strict, Transitional; minOccurs=1; maxOccurs=1. 3D Bar Charts; [§21.2.2.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `ofPieChart` — [c:CT_OfPieChart](charts.md#c-ct_ofpiechart); Strict, Transitional; minOccurs=1; maxOccurs=1. Pie of Pie or Bar of Pie Charts; [§21.2.2.126](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3419-part-1).
- [ ] `surfaceChart` — [c:CT_SurfaceChart](charts.md#c-ct_surfacechart); Strict, Transitional; minOccurs=1; maxOccurs=1. Surface Charts; [§21.2.2.204](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3445-part-1).
- [ ] `surface3DChart` — [c:CT_Surface3DChart](charts.md#c-ct_surface3dchart); Strict, Transitional; minOccurs=1; maxOccurs=1. 3D Surface Charts; [§21.2.2.203](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3445-part-1).
- [ ] `bubbleChart` — [c:CT_BubbleChart](charts.md#c-ct_bubblechart); Strict, Transitional; minOccurs=1; maxOccurs=1. Bubble Charts; [§21.2.2.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3387-part-1).
- [ ] `valAx` — [c:CT_ValAx](charts.md#c-ct_valax); Strict, Transitional; minOccurs=1; maxOccurs=1. Value Axis; [§21.2.2.226](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3449-part-1).
- [ ] `catAx` — [c:CT_CatAx](charts.md#c-ct_catax); Strict, Transitional; minOccurs=1; maxOccurs=1. Category Axis Data; [§21.2.2.25](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3388-part-1).
- [ ] `dateAx` — [c:CT_DateAx](charts.md#c-ct_dateax); Strict, Transitional; minOccurs=1; maxOccurs=1. Date Axis; [§21.2.2.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `serAx` — [c:CT_SerAx](charts.md#c-ct_serax); Strict, Transitional; minOccurs=1; maxOccurs=1. Series Axis; [§21.2.2.175](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3435-part-1).
- [ ] `dTable` — [c:CT_DTable](charts.md#c-ct_dtable); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Table; [§21.2.2.54](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_legend"></a>

## c:CT_Legend

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `legendPos` — [c:CT_LegendPos](charts.md#c-ct_legendpos); Strict, Transitional; minOccurs=0; maxOccurs=1. Legend Position; [§21.2.2.95](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3411-part-1).
- [ ] `legendEntry` — [c:CT_LegendEntry](charts.md#c-ct_legendentry); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Legend Entry; [§21.2.2.94](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3411-part-1).
- [ ] `layout` — [c:CT_Layout](chart-extensions.md#c-ct_layout); Strict, Transitional; minOccurs=0; maxOccurs=1. Layout; [§21.2.2.88](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `overlay` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Overlay; [§21.2.2.132](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3421-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_dispblanksas"></a>

## c:CT_DispBlanksAs

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_DispBlanksAs](values-charts.md#c-st_dispblanksas); Strict, Transitional; default=zero.

<a id="c-ct_headerfooter"></a>

## c:CT_HeaderFooter

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `oddHeader` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=0; maxOccurs=1. Odd Header; [§21.2.2.125](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3419-part-1).
- [ ] `oddFooter` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=0; maxOccurs=1. Odd Footer; [§21.2.2.124](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `evenHeader` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=0; maxOccurs=1. Even Header; [§21.2.2.60](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3399-part-1).
- [ ] `evenFooter` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=0; maxOccurs=1. Even Footer; [§21.2.2.59](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3399-part-1).
- [ ] `firstHeader` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=0; maxOccurs=1. First Header; [§21.2.2.67](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3402-part-1).
- [ ] `firstFooter` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=0; maxOccurs=1. First Footer; [§21.2.2.66](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3402-part-1).
- [ ] `@alignWithMargins` — `xsd:boolean`; Strict, Transitional; default=true.
- [ ] `@differentOddEven` — `xsd:boolean`; Strict, Transitional; default=false.
- [ ] `@differentFirst` — `xsd:boolean`; Strict, Transitional; default=false.

<a id="c-ct_pagemargins"></a>

## c:CT_PageMargins

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@l` — `xsd:double`; Strict, Transitional; use=required.
- [ ] `@r` — `xsd:double`; Strict, Transitional; use=required.
- [ ] `@t` — `xsd:double`; Strict, Transitional; use=required.
- [ ] `@b` — `xsd:double`; Strict, Transitional; use=required.
- [ ] `@header` — `xsd:double`; Strict, Transitional; use=required.
- [ ] `@footer` — `xsd:double`; Strict, Transitional; use=required.

<a id="c-ct_pagesetup"></a>

## c:CT_PageSetup

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@paperSize` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=1.
- [ ] `@paperHeight` — [s:ST_PositiveUniversalMeasure](values-shared.md#s-st_positiveuniversalmeasure); Strict, Transitional; use=optional.
- [ ] `@paperWidth` — [s:ST_PositiveUniversalMeasure](values-shared.md#s-st_positiveuniversalmeasure); Strict, Transitional; use=optional.
- [ ] `@firstPageNumber` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=1.
- [ ] `@orientation` — [c:ST_PageSetupOrientation](values-charts.md#c-st_pagesetuporientation); Strict, Transitional; use=optional; default=default.
- [ ] `@blackAndWhite` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@draft` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@useFirstPageNumber` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@horizontalDpi` — `xsd:int`; Strict, Transitional; use=optional; default=600.
- [ ] `@verticalDpi` — `xsd:int`; Strict, Transitional; use=optional; default=600.
- [ ] `@copies` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=1.

<a id="c-ct_extension"></a>

## c:CT_Extension

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `*` — extension wildcard; Strict, Transitional; processContents=lax.
- [ ] `@uri` — `xsd:token`; Strict, Transitional.

<a id="cdr-ct_marker"></a>

## cdr:CT_Marker

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `x` — [cdr:ST_MarkerCoordinate](values-charts.md#cdr-st_markercoordinate); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `y` — [cdr:ST_MarkerCoordinate](values-charts.md#cdr-st_markercoordinate); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="cdr-ct_shape"></a>

## cdr:CT_Shape

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `nvSpPr` — [cdr:CT_ShapeNonVisual](charts.md#cdr-ct_shapenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `txBody` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@macro` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@textlink` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@fLocksText` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@fPublished` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="cdr-ct_groupshape"></a>

## cdr:CT_GroupShape

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `nvGrpSpPr` — [cdr:CT_GroupShapeNonVisual](charts.md#cdr-ct_groupshapenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpSpPr` — [a:CT_GroupShapeProperties](charts.md#a-ct_groupshapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sp` — [cdr:CT_Shape](charts.md#cdr-ct_shape); Strict, Transitional.
- [ ] `grpSp` — [cdr:CT_GroupShape](charts.md#cdr-ct_groupshape); Strict, Transitional.
- [ ] `graphicFrame` — [cdr:CT_GraphicFrame](charts.md#cdr-ct_graphicframe); Strict, Transitional.
- [ ] `cxnSp` — [cdr:CT_Connector](charts.md#cdr-ct_connector); Strict, Transitional.
- [ ] `pic` — [cdr:CT_Picture](charts.md#cdr-ct_picture); Strict, Transitional.

<a id="cdr-ct_graphicframe"></a>

## cdr:CT_GraphicFrame

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `nvGraphicFramePr` — [cdr:CT_GraphicFrameNonVisual](charts.md#cdr-ct_graphicframenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `xfrm` — [a:CT_Transform2D](images.md#a-ct_transform2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `a:graphic` — [a:CT_GraphicalObject](images.md#a-ct_graphicalobject); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@macro` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@fPublished` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="cdr-ct_connector"></a>

## cdr:CT_Connector

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `nvCxnSpPr` — [cdr:CT_ConnectorNonVisual](charts.md#cdr-ct_connectornonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@macro` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@fPublished` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="cdr-ct_picture"></a>

## cdr:CT_Picture

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `nvPicPr` — [cdr:CT_PictureNonVisual](charts.md#cdr-ct_picturenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@macro` — `xsd:string`; Strict, Transitional; use=optional; default=.
- [ ] `@fPublished` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="c-ct_pivotfmt"></a>

## c:CT_PivotFmt

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `dLbl` — [c:CT_DLbl](chart-extensions.md#c-ct_dlbl); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Label; [§21.2.2.47](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3396-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_rotx"></a>

## c:CT_RotX

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_RotX](values-charts.md#c-st_rotx); Strict, Transitional; default=0.

<a id="c-ct_hpercent"></a>

## c:CT_HPercent

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_HPercent](values-charts.md#c-st_hpercent); Strict, Transitional; default=100%.

<a id="c-ct_roty"></a>

## c:CT_RotY

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_RotY](values-charts.md#c-st_roty); Strict, Transitional; default=0.

<a id="c-ct_depthpercent"></a>

## c:CT_DepthPercent

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_DepthPercent](values-charts.md#c-st_depthpercent); Strict, Transitional; default=100%.

<a id="c-ct_perspective"></a>

## c:CT_Perspective

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Perspective](values-charts.md#c-st_perspective); Strict, Transitional; default=30.

<a id="c-ct_thickness"></a>

## c:CT_Thickness

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Thickness](values-charts.md#c-st_thickness); Strict, Transitional; use=required.

<a id="c-ct_pictureoptions"></a>

## c:CT_PictureOptions

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `applyToFront` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Apply To Front; [§21.2.2.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3382-part-1).
- [ ] `applyToSides` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Apply To Sides; [§21.2.2.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3382-part-1).
- [ ] `applyToEnd` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Apply to End; [§21.2.2.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3380-3381-0628.md#source-page-3381-part-1).
- [ ] `pictureFormat` — [c:CT_PictureFormat](charts.md#c-ct_pictureformat); Strict, Transitional; minOccurs=0; maxOccurs=1. Picture Format; [§21.2.2.137](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3427-part-1).
- [ ] `pictureStackUnit` — [c:CT_PictureStackUnit](charts.md#c-ct_picturestackunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Picture Stack Unit; [§21.2.2.139](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3427-part-1).

<a id="c-ct_areachart"></a>

## c:CT_AreaChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `grouping` — [c:CT_Grouping](charts.md#c-ct_grouping); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_AreaSer](chart-extensions.md#c-ct_areaser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dropLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Drop Lines; [§21.2.2.53](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=2. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_area3dchart"></a>

## c:CT_Area3DChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `grouping` — [c:CT_Grouping](charts.md#c-ct_grouping); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_AreaSer](chart-extensions.md#c-ct_areaser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dropLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Drop Lines; [§21.2.2.53](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `gapDepth` — [c:CT_GapAmount](charts.md#c-ct_gapamount); Strict, Transitional; minOccurs=0; maxOccurs=1. Gap Depth; [§21.2.2.74](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3405-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=3. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_linechart"></a>

## c:CT_LineChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `grouping` — [c:CT_Grouping](charts.md#c-ct_grouping); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_LineSer](chart-extensions.md#c-ct_lineser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dropLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Drop Lines; [§21.2.2.53](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `hiLowLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. High Low Lines; [§21.2.2.80](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3407-part-1).
- [ ] `upDownBars` — [c:CT_UpDownBars](charts.md#c-ct_updownbars); Strict, Transitional; minOccurs=0; maxOccurs=1. Up/Down Bars; [§21.2.2.218](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3448-part-1).
- [ ] `marker` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `smooth` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Smoothing; [§21.2.2.194](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3442-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=2. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_line3dchart"></a>

## c:CT_Line3DChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `grouping` — [c:CT_Grouping](charts.md#c-ct_grouping); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_LineSer](chart-extensions.md#c-ct_lineser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dropLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Drop Lines; [§21.2.2.53](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `gapDepth` — [c:CT_GapAmount](charts.md#c-ct_gapamount); Strict, Transitional; minOccurs=0; maxOccurs=1. Gap Depth; [§21.2.2.74](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3405-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=3; maxOccurs=3. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_stockchart"></a>

## c:CT_StockChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `ser` — [c:CT_LineSer](chart-extensions.md#c-ct_lineser); Strict, Transitional; minOccurs=3; maxOccurs=4.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dropLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Drop Lines; [§21.2.2.53](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `hiLowLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. High Low Lines; [§21.2.2.80](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3407-part-1).
- [ ] `upDownBars` — [c:CT_UpDownBars](charts.md#c-ct_updownbars); Strict, Transitional; minOccurs=0; maxOccurs=1. Up/Down Bars; [§21.2.2.218](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3448-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=2. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_radarchart"></a>

## c:CT_RadarChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `radarStyle` — [c:CT_RadarStyle](charts.md#c-ct_radarstyle); Strict, Transitional; minOccurs=1; maxOccurs=1. Radar Style; [§21.2.2.154](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3430-part-1).
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_RadarSer](chart-extensions.md#c-ct_radarser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=2. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_scatterchart"></a>

## c:CT_ScatterChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `scatterStyle` — [c:CT_ScatterStyle](charts.md#c-ct_scatterstyle); Strict, Transitional; minOccurs=1; maxOccurs=1. Scatter Style; [§21.2.2.162](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3433-part-1).
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_ScatterSer](chart-extensions.md#c-ct_scatterser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=2. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_piechart"></a>

## c:CT_PieChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_PieSer](chart-extensions.md#c-ct_pieser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `firstSliceAng` — [c:CT_FirstSliceAng](charts.md#c-ct_firstsliceang); Strict, Transitional; minOccurs=0; maxOccurs=1. First Slice Angle; [§21.2.2.68](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3403-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_pie3dchart"></a>

## c:CT_Pie3DChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_PieSer](chart-extensions.md#c-ct_pieser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_doughnutchart"></a>

## c:CT_DoughnutChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_PieSer](chart-extensions.md#c-ct_pieser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `firstSliceAng` — [c:CT_FirstSliceAng](charts.md#c-ct_firstsliceang); Strict, Transitional; minOccurs=0; maxOccurs=1. First Slice Angle; [§21.2.2.68](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3403-part-1).
- [ ] `holeSize` — [c:CT_HoleSize](charts.md#c-ct_holesize); Strict, Transitional; minOccurs=0; maxOccurs=1. Hole Size; [§21.2.2.82](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_barchart"></a>

## c:CT_BarChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `barDir` — [c:CT_BarDir](charts.md#c-ct_bardir); Strict, Transitional; minOccurs=1; maxOccurs=1. Bar Direction; [§21.2.2.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `grouping` — [c:CT_BarGrouping](charts.md#c-ct_bargrouping); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_BarSer](chart-extensions.md#c-ct_barser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `gapWidth` — [c:CT_GapAmount](charts.md#c-ct_gapamount); Strict, Transitional; minOccurs=0; maxOccurs=1. Gap Width; [§21.2.2.75](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3405-part-1).
- [ ] `overlap` — [c:CT_Overlap](charts.md#c-ct_overlap); Strict, Transitional; minOccurs=0; maxOccurs=1. Overlap; [§21.2.2.131](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3421-part-1).
- [ ] `serLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Series Lines; [§21.2.2.176](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3435-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=2. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_bar3dchart"></a>

## c:CT_Bar3DChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `barDir` — [c:CT_BarDir](charts.md#c-ct_bardir); Strict, Transitional; minOccurs=1; maxOccurs=1. Bar Direction; [§21.2.2.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `grouping` — [c:CT_BarGrouping](charts.md#c-ct_bargrouping); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_BarSer](chart-extensions.md#c-ct_barser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `gapWidth` — [c:CT_GapAmount](charts.md#c-ct_gapamount); Strict, Transitional; minOccurs=0; maxOccurs=1. Gap Width; [§21.2.2.75](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3405-part-1).
- [ ] `gapDepth` — [c:CT_GapAmount](charts.md#c-ct_gapamount); Strict, Transitional; minOccurs=0; maxOccurs=1. Gap Depth; [§21.2.2.74](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3405-part-1).
- [ ] `shape` — [c:CT_Shape](chart-extensions.md#c-ct_shape); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape; [§21.2.2.177](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3436-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=3. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_ofpiechart"></a>

## c:CT_OfPieChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `ofPieType` — [c:CT_OfPieType](charts.md#c-ct_ofpietype); Strict, Transitional; minOccurs=1; maxOccurs=1. Pie of Pie or Bar of Pie Type; [§21.2.2.127](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3420-part-1).
- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_PieSer](chart-extensions.md#c-ct_pieser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `gapWidth` — [c:CT_GapAmount](charts.md#c-ct_gapamount); Strict, Transitional; minOccurs=0; maxOccurs=1. Gap Width; [§21.2.2.75](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3405-part-1).
- [ ] `splitType` — [c:CT_SplitType](charts.md#c-ct_splittype); Strict, Transitional; minOccurs=0; maxOccurs=1. Split Type; [§21.2.2.196](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `splitPos` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Split Position; [§21.2.2.195](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `custSplit` — [c:CT_CustSplit](charts.md#c-ct_custsplit); Strict, Transitional; minOccurs=0; maxOccurs=1. Custom Split; [§21.2.2.35](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3393-part-1).
- [ ] `secondPieSize` — [c:CT_SecondPieSize](charts.md#c-ct_secondpiesize); Strict, Transitional; minOccurs=0; maxOccurs=1. Second Pie Size; [§21.2.2.164](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3433-part-1).
- [ ] `serLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Series Lines; [§21.2.2.176](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3435-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_surfacechart"></a>

## c:CT_SurfaceChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `wireframe` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Wireframe; [§21.2.2.230](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3451-part-1).
- [ ] `ser` — [c:CT_SurfaceSer](chart-extensions.md#c-ct_surfaceser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `bandFmts` — [c:CT_BandFmts](charts.md#c-ct_bandfmts); Strict, Transitional; minOccurs=0; maxOccurs=1. Band Formats; [§21.2.2.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3385-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=3. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_surface3dchart"></a>

## c:CT_Surface3DChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `wireframe` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Wireframe; [§21.2.2.230](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3451-part-1).
- [ ] `ser` — [c:CT_SurfaceSer](chart-extensions.md#c-ct_surfaceser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `bandFmts` — [c:CT_BandFmts](charts.md#c-ct_bandfmts); Strict, Transitional; minOccurs=0; maxOccurs=1. Band Formats; [§21.2.2.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3385-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=3; maxOccurs=3. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_bubblechart"></a>

## c:CT_BubbleChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `varyColors` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Vary Colors by Point; [§21.2.2.227](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `ser` — [c:CT_BubbleSer](chart-extensions.md#c-ct_bubbleser); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `bubble3D` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. 3D Bubble; [§21.2.2.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `bubbleScale` — [c:CT_BubbleScale](charts.md#c-ct_bubblescale); Strict, Transitional; minOccurs=0; maxOccurs=1. Bubble Scale; [§21.2.2.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3387-part-1).
- [ ] `showNegBubbles` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Negative Bubbles; [§21.2.2.185](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3439-part-1).
- [ ] `sizeRepresents` — [c:CT_SizeRepresents](charts.md#c-ct_sizerepresents); Strict, Transitional; minOccurs=0; maxOccurs=1. Size Represents; [§21.2.2.193](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3442-part-1).
- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=2; maxOccurs=2. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_valax"></a>

## c:CT_ValAx

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `scaling` — [c:CT_Scaling](charts.md#c-ct_scaling); Strict, Transitional; minOccurs=1; maxOccurs=1. Scaling; [§21.2.2.160](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3432-part-1).
- [ ] `delete` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Delete; [§21.2.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `axPos` — [c:CT_AxPos](charts.md#c-ct_axpos); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis Position; [§21.2.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `majorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Gridlines; [§21.2.2.100](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `minorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Gridlines; [§21.2.2.109](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `title` — [c:CT_Title](charts.md#c-ct_title); Strict, Transitional; minOccurs=0; maxOccurs=1. Title; [§21.2.2.210](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Format; [§21.2.2.121](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `majorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Tick Mark; [§21.2.2.101](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `minorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Tick Mark; [§21.2.2.110](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `tickLblPos` — [c:CT_TickLblPos](charts.md#c-ct_ticklblpos); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Label Position; [§21.2.2.207](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `crossAx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Axis ID; [§21.2.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3391-part-1).
- [ ] `crosses` — [c:CT_Crosses](charts.md#c-ct_crosses); Strict, Transitional; minOccurs=1; maxOccurs=1. Crosses; [§21.2.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `crossesAt` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Value; [§21.2.2.34](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `crossBetween` — [c:CT_CrossBetween](charts.md#c-ct_crossbetween); Strict, Transitional; minOccurs=0; maxOccurs=1. Cross Between; [§21.2.2.32](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3391-part-1).
- [ ] `majorUnit` — [c:CT_AxisUnit](charts.md#c-ct_axisunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Unit; [§21.2.2.103](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `minorUnit` — [c:CT_AxisUnit](charts.md#c-ct_axisunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Unit; [§21.2.2.112](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3416-part-1).
- [ ] `dispUnits` — [c:CT_DispUnits](charts.md#c-ct_dispunits); Strict, Transitional; minOccurs=0; maxOccurs=1. Display Units; [§21.2.2.45](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3396-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_catax"></a>

## c:CT_CatAx

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `scaling` — [c:CT_Scaling](charts.md#c-ct_scaling); Strict, Transitional; minOccurs=1; maxOccurs=1. Scaling; [§21.2.2.160](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3432-part-1).
- [ ] `delete` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Delete; [§21.2.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `axPos` — [c:CT_AxPos](charts.md#c-ct_axpos); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis Position; [§21.2.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `majorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Gridlines; [§21.2.2.100](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `minorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Gridlines; [§21.2.2.109](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `title` — [c:CT_Title](charts.md#c-ct_title); Strict, Transitional; minOccurs=0; maxOccurs=1. Title; [§21.2.2.210](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Format; [§21.2.2.121](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `majorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Tick Mark; [§21.2.2.101](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `minorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Tick Mark; [§21.2.2.110](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `tickLblPos` — [c:CT_TickLblPos](charts.md#c-ct_ticklblpos); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Label Position; [§21.2.2.207](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `crossAx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Axis ID; [§21.2.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3391-part-1).
- [ ] `crosses` — [c:CT_Crosses](charts.md#c-ct_crosses); Strict, Transitional; minOccurs=1; maxOccurs=1. Crosses; [§21.2.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `crossesAt` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Value; [§21.2.2.34](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `auto` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Automatic Category Axis; [§21.2.2.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3383-part-1).
- [ ] `lblAlgn` — [c:CT_LblAlgn](charts.md#c-ct_lblalgn); Strict, Transitional; minOccurs=0; maxOccurs=1. Label Alignment; [§21.2.2.90](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `lblOffset` — [c:CT_LblOffset](charts.md#c-ct_lbloffset); Strict, Transitional; minOccurs=0; maxOccurs=1. Label Offset; [§21.2.2.91](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3411-part-1).
- [ ] `tickLblSkip` — [c:CT_Skip](charts.md#c-ct_skip); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Label Skip; [§21.2.2.208](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `tickMarkSkip` — [c:CT_Skip](charts.md#c-ct_skip); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Mark Skip; [§21.2.2.209](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `noMultiLvlLbl` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. No Multi-level Labels; [§21.2.2.119](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3417-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_dateax"></a>

## c:CT_DateAx

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `scaling` — [c:CT_Scaling](charts.md#c-ct_scaling); Strict, Transitional; minOccurs=1; maxOccurs=1. Scaling; [§21.2.2.160](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3432-part-1).
- [ ] `delete` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Delete; [§21.2.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `axPos` — [c:CT_AxPos](charts.md#c-ct_axpos); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis Position; [§21.2.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `majorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Gridlines; [§21.2.2.100](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `minorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Gridlines; [§21.2.2.109](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `title` — [c:CT_Title](charts.md#c-ct_title); Strict, Transitional; minOccurs=0; maxOccurs=1. Title; [§21.2.2.210](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Format; [§21.2.2.121](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `majorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Tick Mark; [§21.2.2.101](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `minorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Tick Mark; [§21.2.2.110](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `tickLblPos` — [c:CT_TickLblPos](charts.md#c-ct_ticklblpos); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Label Position; [§21.2.2.207](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `crossAx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Axis ID; [§21.2.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3391-part-1).
- [ ] `crosses` — [c:CT_Crosses](charts.md#c-ct_crosses); Strict, Transitional; minOccurs=1; maxOccurs=1. Crosses; [§21.2.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `crossesAt` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Value; [§21.2.2.34](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `auto` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Automatic Category Axis; [§21.2.2.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3383-part-1).
- [ ] `lblOffset` — [c:CT_LblOffset](charts.md#c-ct_lbloffset); Strict, Transitional; minOccurs=0; maxOccurs=1. Label Offset; [§21.2.2.91](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3411-part-1).
- [ ] `baseTimeUnit` — [c:CT_TimeUnit](charts.md#c-ct_timeunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Base Time Unit; [§21.2.2.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `majorUnit` — [c:CT_AxisUnit](charts.md#c-ct_axisunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Unit; [§21.2.2.103](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `majorTimeUnit` — [c:CT_TimeUnit](charts.md#c-ct_timeunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Time Unit; [§21.2.2.102](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `minorUnit` — [c:CT_AxisUnit](charts.md#c-ct_axisunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Unit; [§21.2.2.112](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3416-part-1).
- [ ] `minorTimeUnit` — [c:CT_TimeUnit](charts.md#c-ct_timeunit); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Time Unit; [§21.2.2.111](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_serax"></a>

## c:CT_SerAx

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `axId` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis ID; [§21.2.2.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `scaling` — [c:CT_Scaling](charts.md#c-ct_scaling); Strict, Transitional; minOccurs=1; maxOccurs=1. Scaling; [§21.2.2.160](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3432-part-1).
- [ ] `delete` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Delete; [§21.2.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `axPos` — [c:CT_AxPos](charts.md#c-ct_axpos); Strict, Transitional; minOccurs=1; maxOccurs=1. Axis Position; [§21.2.2.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3384-part-1).
- [ ] `majorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Gridlines; [§21.2.2.100](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `minorGridlines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Gridlines; [§21.2.2.109](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `title` — [c:CT_Title](charts.md#c-ct_title); Strict, Transitional; minOccurs=0; maxOccurs=1. Title; [§21.2.2.210](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Format; [§21.2.2.121](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `majorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Major Tick Mark; [§21.2.2.101](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `minorTickMark` — [c:CT_TickMark](charts.md#c-ct_tickmark); Strict, Transitional; minOccurs=0; maxOccurs=1. Minor Tick Mark; [§21.2.2.110](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `tickLblPos` — [c:CT_TickLblPos](charts.md#c-ct_ticklblpos); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Label Position; [§21.2.2.207](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `crossAx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Axis ID; [§21.2.2.31](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3391-part-1).
- [ ] `crosses` — [c:CT_Crosses](charts.md#c-ct_crosses); Strict, Transitional; minOccurs=1; maxOccurs=1. Crosses; [§21.2.2.33](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `crossesAt` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=1; maxOccurs=1. Crossing Value; [§21.2.2.34](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3392-part-1).
- [ ] `tickLblSkip` — [c:CT_Skip](charts.md#c-ct_skip); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Label Skip; [§21.2.2.208](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `tickMarkSkip` — [c:CT_Skip](charts.md#c-ct_skip); Strict, Transitional; minOccurs=0; maxOccurs=1. Tick Mark Skip; [§21.2.2.209](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_dtable"></a>

## c:CT_DTable

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `showHorzBorder` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Horizontal Border; [§21.2.2.181](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3437-part-1).
- [ ] `showVertBorder` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Vertical Border; [§21.2.2.190](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3441-part-1).
- [ ] `showOutline` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Outline Border; [§21.2.2.186](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3439-part-1).
- [ ] `showKeys` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Legend Keys; [§21.2.2.182](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3438-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_legendpos"></a>

## c:CT_LegendPos

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_LegendPos](values-charts.md#c-st_legendpos); Strict, Transitional; default=r.

<a id="c-ct_legendentry"></a>

## c:CT_LegendEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `delete` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=1; maxOccurs=1. Delete; [§21.2.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="cdr-ct_shapenonvisual"></a>

## cdr:CT_ShapeNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvSpPr` — [a:CT_NonVisualDrawingShapeProps](shapes-and-lines.md#a-ct_nonvisualdrawingshapeprops); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="cdr-ct_groupshapenonvisual"></a>

## cdr:CT_GroupShapeNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvGrpSpPr` — [a:CT_NonVisualGroupDrawingShapeProps](charts.md#a-ct_nonvisualgroupdrawingshapeprops); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_groupshapeproperties"></a>

## a:CT_GroupShapeProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `xfrm` — [a:CT_GroupTransform2D](charts.md#a-ct_grouptransform2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectLst` — [a:CT_EffectList](tables.md#a-ct_effectlist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectDag` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `scene3d` — [a:CT_Scene3D](drawing-text.md#a-ct_scene3d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@bwMode` — [a:ST_BlackWhiteMode](values-drawing.md#a-st_blackwhitemode); Strict, Transitional; use=optional.

<a id="cdr-ct_graphicframenonvisual"></a>

## cdr:CT_GraphicFrameNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvGraphicFramePr` — [a:CT_NonVisualGraphicFrameProperties](images.md#a-ct_nonvisualgraphicframeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="cdr-ct_connectornonvisual"></a>

## cdr:CT_ConnectorNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvCxnSpPr` — [a:CT_NonVisualConnectorProperties](shapes-and-lines.md#a-ct_nonvisualconnectorproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="cdr-ct_picturenonvisual"></a>

## cdr:CT_PictureNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chartDrawing.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chartDrawing.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvPicPr` — [a:CT_NonVisualPictureProperties](images.md#a-ct_nonvisualpictureproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="c-ct_pictureformat"></a>

## c:CT_PictureFormat

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_PictureFormat](values-charts.md#c-st_pictureformat); Strict, Transitional; use=required.

<a id="c-ct_picturestackunit"></a>

## c:CT_PictureStackUnit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_PictureStackUnit](values-charts.md#c-st_picturestackunit); Strict, Transitional; use=required.

<a id="c-ct_grouping"></a>

## c:CT_Grouping

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Grouping](values-charts.md#c-st_grouping); Strict, Transitional; default=standard.

<a id="c-ct_gapamount"></a>

## c:CT_GapAmount

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_GapAmount](values-charts.md#c-st_gapamount); Strict, Transitional; default=150%.

<a id="c-ct_updownbars"></a>

## c:CT_UpDownBars

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `gapWidth` — [c:CT_GapAmount](charts.md#c-ct_gapamount); Strict, Transitional; minOccurs=0; maxOccurs=1. Gap Width; [§21.2.2.75](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3405-part-1).
- [ ] `upBars` — [c:CT_UpDownBar](charts.md#c-ct_updownbar); Strict, Transitional; minOccurs=0; maxOccurs=1. Up Bars; [§21.2.2.217](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `downBars` — [c:CT_UpDownBar](charts.md#c-ct_updownbar); Strict, Transitional; minOccurs=0; maxOccurs=1. Down Bars; [§21.2.2.51](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_radarstyle"></a>

## c:CT_RadarStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_RadarStyle](values-charts.md#c-st_radarstyle); Strict, Transitional; default=standard.

<a id="c-ct_scatterstyle"></a>

## c:CT_ScatterStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_ScatterStyle](values-charts.md#c-st_scatterstyle); Strict, Transitional; default=marker.

<a id="c-ct_firstsliceang"></a>

## c:CT_FirstSliceAng

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_FirstSliceAng](values-charts.md#c-st_firstsliceang); Strict, Transitional; default=0.

<a id="c-ct_holesize"></a>

## c:CT_HoleSize

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_HoleSize](values-charts.md#c-st_holesize); Strict, Transitional; default=10%.

<a id="c-ct_bardir"></a>

## c:CT_BarDir

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_BarDir](values-charts.md#c-st_bardir); Strict, Transitional; default=col.

<a id="c-ct_bargrouping"></a>

## c:CT_BarGrouping

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_BarGrouping](values-charts.md#c-st_bargrouping); Strict, Transitional; default=clustered.

<a id="c-ct_overlap"></a>

## c:CT_Overlap

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Overlap](values-charts.md#c-st_overlap); Strict, Transitional; default=0%.

<a id="c-ct_ofpietype"></a>

## c:CT_OfPieType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_OfPieType](values-charts.md#c-st_ofpietype); Strict, Transitional; default=pie.

<a id="c-ct_splittype"></a>

## c:CT_SplitType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_SplitType](values-charts.md#c-st_splittype); Strict, Transitional; default=auto.

<a id="c-ct_custsplit"></a>

## c:CT_CustSplit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `secondPiePt` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Second Pie Point; [§21.2.2.163](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3433-part-1).

<a id="c-ct_secondpiesize"></a>

## c:CT_SecondPieSize

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_SecondPieSize](values-charts.md#c-st_secondpiesize); Strict, Transitional; default=75%.

<a id="c-ct_bandfmts"></a>

## c:CT_BandFmts

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `bandFmt` — [c:CT_BandFmt](charts.md#c-ct_bandfmt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Band Format; [§21.2.2.13](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3385-part-1).

<a id="c-ct_bubblescale"></a>

## c:CT_BubbleScale

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_BubbleScale](values-charts.md#c-st_bubblescale); Strict, Transitional; default=100%.

<a id="c-ct_sizerepresents"></a>

## c:CT_SizeRepresents

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_SizeRepresents](values-charts.md#c-st_sizerepresents); Strict, Transitional; default=area.

<a id="c-ct_scaling"></a>

## c:CT_Scaling

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `logBase` — [c:CT_LogBase](charts.md#c-ct_logbase); Strict, Transitional; minOccurs=0; maxOccurs=1. Logarithmic Base; [§21.2.2.98](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `orientation` — [c:CT_Orientation](charts.md#c-ct_orientation); Strict, Transitional; minOccurs=0; maxOccurs=1. Axis Orientation; [§21.2.2.130](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3419-3424-0635.md#source-page-3420-part-1).
- [ ] `max` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Maximum; [§21.2.2.107](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3414-part-1).
- [ ] `min` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Minimum; [§21.2.2.108](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3415-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_axpos"></a>

## c:CT_AxPos

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_AxPos](values-charts.md#c-st_axpos); Strict, Transitional; use=required.

<a id="c-ct_tickmark"></a>

## c:CT_TickMark

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_TickMark](values-charts.md#c-st_tickmark); Strict, Transitional; default=cross.

<a id="c-ct_ticklblpos"></a>

## c:CT_TickLblPos

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_TickLblPos](values-charts.md#c-st_ticklblpos); Strict, Transitional; default=nextTo.

<a id="c-ct_crosses"></a>

## c:CT_Crosses

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Crosses](values-charts.md#c-st_crosses); Strict, Transitional; use=required.

<a id="c-ct_crossbetween"></a>

## c:CT_CrossBetween

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_CrossBetween](values-charts.md#c-st_crossbetween); Strict, Transitional; use=required.

<a id="c-ct_axisunit"></a>

## c:CT_AxisUnit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_AxisUnit](values-charts.md#c-st_axisunit); Strict, Transitional; use=required.

<a id="c-ct_dispunits"></a>

## c:CT_DispUnits

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `custUnit` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=1; maxOccurs=1. Custom Display Unit; [§21.2.2.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3393-part-1).
- [ ] `builtInUnit` — [c:CT_BuiltInUnit](charts.md#c-ct_builtinunit); Strict, Transitional; minOccurs=1; maxOccurs=1. Built in Display Unit Value; [§21.2.2.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3387-part-1).
- [ ] `dispUnitsLbl` — [c:CT_DispUnitsLbl](charts.md#c-ct_dispunitslbl); Strict, Transitional; minOccurs=0; maxOccurs=1. Display Units Label; [§21.2.2.46](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3396-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_lblalgn"></a>

## c:CT_LblAlgn

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_LblAlgn](values-charts.md#c-st_lblalgn); Strict, Transitional; use=required.

<a id="c-ct_lbloffset"></a>

## c:CT_LblOffset

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_LblOffset](values-charts.md#c-st_lbloffset); Strict, Transitional; default=100%.

<a id="c-ct_skip"></a>

## c:CT_Skip

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Skip](values-charts.md#c-st_skip); Strict, Transitional; use=required.

<a id="c-ct_timeunit"></a>

## c:CT_TimeUnit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_TimeUnit](values-charts.md#c-st_timeunit); Strict, Transitional; default=days.

<a id="a-ct_nonvisualgroupdrawingshapeprops"></a>

## a:CT_NonVisualGroupDrawingShapeProps

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `grpSpLocks` — [a:CT_GroupLocking](charts.md#a-ct_grouplocking); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_grouptransform2d"></a>

## a:CT_GroupTransform2D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `off` — [a:CT_Point2D](images.md#a-ct_point2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `ext` — [a:CT_PositiveSize2D](images.md#a-ct_positivesize2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `chOff` — [a:CT_Point2D](images.md#a-ct_point2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `chExt` — [a:CT_PositiveSize2D](images.md#a-ct_positivesize2d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@rot` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=optional; default=0.
- [ ] `@flipH` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@flipV` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="c-ct_updownbar"></a>

## c:CT_UpDownBar

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).

<a id="c-ct_bandfmt"></a>

## c:CT_BandFmt

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).

<a id="c-ct_logbase"></a>

## c:CT_LogBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_LogBase](values-charts.md#c-st_logbase); Strict, Transitional; use=required.

<a id="c-ct_orientation"></a>

## c:CT_Orientation

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Orientation](values-charts.md#c-st_orientation); Strict, Transitional; default=minMax.

<a id="c-ct_builtinunit"></a>

## c:CT_BuiltInUnit

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_BuiltInUnit](values-charts.md#c-st_builtinunit); Strict, Transitional; default=thousands.

<a id="c-ct_dispunitslbl"></a>

## c:CT_DispUnitsLbl

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `layout` — [c:CT_Layout](chart-extensions.md#c-ct_layout); Strict, Transitional; minOccurs=0; maxOccurs=1. Layout; [§21.2.2.88](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `tx` — [c:CT_Tx](chart-extensions.md#c-ct_tx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).

<a id="a-ct_grouplocking"></a>

## a:CT_GroupLocking

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@noGrp` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noUngrp` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noSelect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noRot` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noChangeAspect` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noMove` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@noResize` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
