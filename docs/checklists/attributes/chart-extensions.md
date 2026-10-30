# Chart extensions

[Inventory guide](README.md)

Complete retained Microsoft chart, chart drawing, chart style and extended-chart declarations. These include modern chart families, formatting, data bindings, filtered-series and label controls. No extension entry implies a new Mote argument or implementation commitment.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [cdr14:contentPart](#cdr14-contentpart)
- [c14:pivotOptions](#c14-pivotoptions)
- [c14:invertSolidFillFmt](#c14-invertsolidfillfmt)
- [c14:style](#c14-style)
- [c15:pivotSource](#c15-pivotsource)
- [c15:numFmt](#c15-numfmt)
- [c15:spPr](#c15-sppr)
- [c15:layout](#c15-layout)
- [c15:fullRef](#c15-fullref)
- [c15:levelRef](#c15-levelref)
- [c15:formulaRef](#c15-formularef)
- [c15:filteredSeriesTitle](#c15-filteredseriestitle)
- [c15:filteredCategoryTitle](#c15-filteredcategorytitle)
- [c15:filteredAreaSeries](#c15-filteredareaseries)
- [c15:filteredBarSeries](#c15-filteredbarseries)
- [c15:filteredBubbleSeries](#c15-filteredbubbleseries)
- [c15:filteredLineSeries](#c15-filteredlineseries)
- [c15:filteredPieSeries](#c15-filteredpieseries)
- [c15:filteredRadarSeries](#c15-filteredradarseries)
- [c15:filteredScatterSeries](#c15-filteredscatterseries)
- [c15:filteredSurfaceSeries](#c15-filteredsurfaceseries)
- [c15:datalabelsRange](#c15-datalabelsrange)
- [c15:categoryFilterExceptions](#c15-categoryfilterexceptions)
- [c15:dlblFieldTable](#c15-dlblfieldtable)
- [c15:xForSave](#c15-xforsave)
- [c15:showDataLabelsRange](#c15-showdatalabelsrange)
- [c15:tx](#c15-tx)
- [c15:showLeaderLines](#c15-showleaderlines)
- [c15:leaderLines](#c15-leaderlines)
- [c15:autoCat](#c15-autocat)
- [cs:colorStyle](#cs-colorstyle)
- [cs:chartStyle](#cs-chartstyle)
- [c16:datapointuniqueidmap](#c16-datapointuniqueidmap)
- [c16:spPr](#c16-sppr)
- [c16:explosion](#c16-explosion)
- [c16:invertIfNegative](#c16-invertifnegative)
- [c16:bubble3D](#c16-bubble3d)
- [c16:marker](#c16-marker)
- [c16:dLbl](#c16-dlbl)
- [c16:categoryFilterExceptions](#c16-categoryfilterexceptions)
- [c16:pivotOptions16](#c16-pivotoptions16)
- [c16ac:multiLvlStrLit](#c16ac-multilvlstrlit)
- [cx:chartSpace](#cx-chartspace)
- [cx:chart](#cx-chart)
- [c16r3:dataDisplayOptions16](#c16r3-datadisplayoptions16)
- [c16r2:formatcode2](#c16r2-formatcode2)
- [c14:CT_BooleanTrue](#c14-ct_booleantrue)
- [c16:CT_NumFilteredLiteralCache](#c16-ct_numfilteredliteralcache)
- [c16:CT_StrFilteredLiteralCache](#c16-ct_strfilteredliteralcache)
- [c16:CT_MultiLvlStrFilteredLiteralCache](#c16-ct_multilvlstrfilteredliteralcache)
- [c16:CT_LiteralDataChart](#c16-ct_literaldatachart)
- [cx:CT_FeatureExtension](#cx-ct_featureextension)
- [cdr14:CT_ContentPart](#cdr14-ct_contentpart)
- [c14:CT_PivotOptions](#c14-ct_pivotoptions)
- [c14:CT_InvertSolidFillFmt](#c14-ct_invertsolidfillfmt)
- [c14:CT_Style](#c14-ct_style)
- [c:CT_NumFmt](#c-ct_numfmt)
- [c:CT_Layout](#c-ct_layout)
- [c15:CT_FullRef](#c15-ct_fullref)
- [c15:CT_LevelRef](#c15-ct_levelref)
- [c15:CT_FormulaRef](#c15-ct_formularef)
- [c15:CT_FilteredSeriesTitle](#c15-ct_filteredseriestitle)
- [c15:CT_FilteredCategoryTitle](#c15-ct_filteredcategorytitle)
- [c15:CT_FilteredAreaSer](#c15-ct_filteredareaser)
- [c15:CT_FilteredBarSer](#c15-ct_filteredbarser)
- [c15:CT_FilteredBubbleSer](#c15-ct_filteredbubbleser)
- [c15:CT_FilteredLineSer](#c15-ct_filteredlineser)
- [c15:CT_FilteredPieSer](#c15-ct_filteredpieser)
- [c15:CT_FilteredRadarSer](#c15-ct_filteredradarser)
- [c15:CT_FilteredScatterSer](#c15-ct_filteredscatterser)
- [c15:CT_FilteredSurfaceSer](#c15-ct_filteredsurfaceser)
- [c15:CT_SeriesDataLabelsRange](#c15-ct_seriesdatalabelsrange)
- [c15:CT_CategoryFilterExceptions](#c15-ct_categoryfilterexceptions)
- [c15:CT_DataLabelFieldTable](#c15-ct_datalabelfieldtable)
- [c:CT_Tx](#c-ct_tx)
- [c:CT_ChartLines](#c-ct_chartlines)
- [cs:CT_ColorStyle](#cs-ct_colorstyle)
- [cs:CT_ChartStyle](#cs-ct_chartstyle)
- [c16:CT_ChartDataPointUniqueIDMap](#c16-ct_chartdatapointuniqueidmap)
- [c:CT_UnsignedInt](#c-ct_unsignedint)
- [c:CT_Marker](#c-ct_marker)
- [c:CT_DLbl](#c-ct_dlbl)
- [c16:CT_CategoryFilterExceptions](#c16-ct_categoryfilterexceptions)
- [c16:CT_PivotOptions16](#c16-ct_pivotoptions16)
- [c:CT_MultiLvlStrData](#c-ct_multilvlstrdata)
- [cx:CT_ChartSpace](#cx-ct_chartspace)
- [cx:CT_RelId](#cx-ct_relid)
- [c16r3:CT_DataDisplayOptions16](#c16r3-ct_datadisplayoptions16)
- [c:CT_NumData](#c-ct_numdata)
- [c:CT_StrData](#c-ct_strdata)
- [cdr14:CT_ContentPartNonVisual](#cdr14-ct_contentpartnonvisual)
- [cdr14:CT_ApplicationNonVisualDrawingProps](#cdr14-ct_applicationnonvisualdrawingprops)
- [c14:CT_BooleanFalse](#c14-ct_booleanfalse)
- [c:CT_ManualLayout](#c-ct_manuallayout)
- [c:CT_SerTx](#c-ct_sertx)
- [c:CT_AxDataSource](#c-ct_axdatasource)
- [c:CT_AreaSer](#c-ct_areaser)
- [c:CT_BarSer](#c-ct_barser)
- [c:CT_BubbleSer](#c-ct_bubbleser)
- [c:CT_LineSer](#c-ct_lineser)
- [c:CT_PieSer](#c-ct_pieser)
- [c:CT_RadarSer](#c-ct_radarser)
- [c:CT_ScatterSer](#c-ct_scatterser)
- [c:CT_SurfaceSer](#c-ct_surfaceser)
- [c15:CT_CategoryFilterException](#c15-ct_categoryfilterexception)
- [c15:CT_DataLabelFieldTableEntry](#c15-ct_datalabelfieldtableentry)
- [c:CT_StrRef](#c-ct_strref)
- [cs:CT_ColorStyleVariation](#cs-ct_colorstylevariation)
- [cs:CT_StyleEntry](#cs-ct_styleentry)
- [cs:CT_MarkerLayout](#cs-ct_markerlayout)
- [c16:CT_ChartDataPointUniqueIDMapEntry](#c16-ct_chartdatapointuniqueidmapentry)
- [c:CT_MarkerStyle](#c-ct_markerstyle)
- [c:CT_MarkerSize](#c-ct_markersize)
- [c:CT_DLblPos](#c-ct_dlblpos)
- [c16:CT_CategoryFilterException](#c16-ct_categoryfilterexception)
- [c16:CT_BooleanFalse](#c16-ct_booleanfalse)
- [c:CT_Lvl](#c-ct_lvl)
- [cx:CT_ChartData](#cx-ct_chartdata)
- [cx:CT_Chart](#cx-ct_chart)
- [cx:CT_Dashboard](#cx-ct_dashboard)
- [cx:CT_FormatOverrides](#cx-ct_formatoverrides)
- [cx:CT_PrintSettings](#cx-ct_printsettings)
- [cx:CT_ExtensionList](#cx-ct_extensionlist)
- [c16r3:CT_BooleanFalse](#c16r3-ct_booleanfalse)
- [c:CT_NumVal](#c-ct_numval)
- [c:CT_StrVal](#c-ct_strval)
- [c:CT_LayoutTarget](#c-ct_layouttarget)
- [c:CT_LayoutMode](#c-ct_layoutmode)
- [c:CT_Double](#c-ct_double)
- [c:CT_MultiLvlStrRef](#c-ct_multilvlstrref)
- [c:CT_NumRef](#c-ct_numref)
- [c:CT_DPt](#c-ct_dpt)
- [c:CT_DLbls](#c-ct_dlbls)
- [c:CT_Trendline](#c-ct_trendline)
- [c:CT_ErrBars](#c-ct_errbars)
- [c:CT_NumDataSource](#c-ct_numdatasource)
- [c:CT_Shape](#c-ct_shape)
- [cs:CT_StyleReference](#cs-ct_stylereference)
- [cs:CT_FontReference](#cs-ct_fontreference)
- [c16:CT_ChartUniqueID](#c16-ct_chartuniqueid)
- [cx:CT_ExternalData](#cx-ct_externaldata)
- [cx:CT_Data](#cx-ct_data)
- [cx:CT_ChartTitle](#cx-ct_charttitle)
- [cx:CT_PlotArea](#cx-ct_plotarea)
- [cx:CT_Legend](#cx-ct_legend)
- [cx:CT_Bindings](#cx-ct_bindings)
- [cx:CT_Artifacts](#cx-ct_artifacts)
- [cx:CT_FormatOverride](#cx-ct_formatoverride)
- [cx:CT_HeaderFooter](#cx-ct_headerfooter)
- [cx:CT_PageMargins](#cx-ct_pagemargins)
- [cx:CT_PageSetup](#cx-ct_pagesetup)
- [cx:CT_Extension](#cx-ct_extension)
- [c:CT_TrendlineType](#c-ct_trendlinetype)
- [c:CT_Order](#c-ct_order)
- [c:CT_Period](#c-ct_period)
- [c:CT_TrendlineLbl](#c-ct_trendlinelbl)
- [c:CT_ErrDir](#c-ct_errdir)
- [c:CT_ErrBarType](#c-ct_errbartype)
- [c:CT_ErrValType](#c-ct_errvaltype)
- [cs:CT_StyleColor](#cs-ct_stylecolor)
- [cx:CT_NumericDimension](#cx-ct_numericdimension)
- [cx:CT_StringDimension](#cx-ct_stringdimension)
- [cx:CT_Text](#cx-ct_text)
- [cx:CT_Offset](#cx-ct_offset)
- [cx:CT_PlotAreaRegion](#cx-ct_plotarearegion)
- [cx:CT_Axis](#cx-ct_axis)
- [cx:CT_Binding](#cx-ct_binding)
- [cx:CT_Artifact](#cx-ct_artifact)
- [cx:CT_Formula](#cx-ct_formula)
- [cx:CT_NumericLevel](#cx-ct_numericlevel)
- [cx:CT_StringLevel](#cx-ct_stringlevel)
- [cx:CT_TextData](#cx-ct_textdata)
- [cx:CT_PlotSurface](#cx-ct_plotsurface)
- [cx:CT_Series](#cx-ct_series)
- [cx:CT_CategoryAxisScaling](#cx-ct_categoryaxisscaling)
- [cx:CT_ValueAxisScaling](#cx-ct_valueaxisscaling)
- [cx:CT_AxisTitle](#cx-ct_axistitle)
- [cx:CT_AxisUnits](#cx-ct_axisunits)
- [cx:CT_Gridlines](#cx-ct_gridlines)
- [cx:CT_TickMarks](#cx-ct_tickmarks)
- [cx:CT_TickLabels](#cx-ct_ticklabels)
- [cx:CT_NumberFormat](#cx-ct_numberformat)
- [cx:CT_NumericValue](#cx-ct_numericvalue)
- [cx:CT_StringValue](#cx-ct_stringvalue)
- [cx:CT_ValueColors](#cx-ct_valuecolors)
- [cx:CT_ValueColorPositions](#cx-ct_valuecolorpositions)
- [cx:CT_DataPoint](#cx-ct_datapoint)
- [cx:CT_DataLabels](#cx-ct_datalabels)
- [cx:CT_DataId](#cx-ct_dataid)
- [cx:CT_SeriesLayoutProperties](#cx-ct_serieslayoutproperties)
- [cx:CT_AxisUnitsLabel](#cx-ct_axisunitslabel)
- [cx:CT_ValueColorEndPosition](#cx-ct_valuecolorendposition)
- [cx:CT_ValueColorMiddlePosition](#cx-ct_valuecolormiddleposition)
- [cx:CT_DataLabelVisibilities](#cx-ct_datalabelvisibilities)
- [cx:CT_DataLabel](#cx-ct_datalabel)
- [cx:CT_DataLabelHidden](#cx-ct_datalabelhidden)
- [cx:CT_ParentLabelLayout](#cx-ct_parentlabellayout)
- [cx:CT_RegionLabelLayout](#cx-ct_regionlabellayout)
- [cx:CT_SeriesElementVisibilities](#cx-ct_serieselementvisibilities)
- [cx:CT_Aggregation](#cx-ct_aggregation)
- [cx:CT_Binning](#cx-ct_binning)
- [cx:CT_Geography](#cx-ct_geography)
- [cx:CT_Statistics](#cx-ct_statistics)
- [cx:CT_Subtotals](#cx-ct_subtotals)
- [cx:CT_ExtremeValueColorPosition](#cx-ct_extremevaluecolorposition)
- [cx:CT_NumberColorPosition](#cx-ct_numbercolorposition)
- [cx:CT_PercentageColorPosition](#cx-ct_percentagecolorposition)
- [cx:CT_GeoCache](#cx-ct_geocache)
- [cx:CT_SubtotalIndex](#cx-ct_subtotalindex)
- [cx:CT_Clear](#cx-ct_clear)
- [cx:CT_GeoLocationQueryResults](#cx-ct_geolocationqueryresults)
- [cx:CT_GeoDataEntityQueryResults](#cx-ct_geodataentityqueryresults)
- [cx:CT_GeoDataPointToEntityQueryResults](#cx-ct_geodatapointtoentityqueryresults)
- [cx:CT_GeoChildEntitiesQueryResults](#cx-ct_geochildentitiesqueryresults)
- [cx:CT_GeoParentEntitiesQueryResults](#cx-ct_geoparententitiesqueryresults)
- [cx:CT_GeoLocationQueryResult](#cx-ct_geolocationqueryresult)
- [cx:CT_GeoDataEntityQueryResult](#cx-ct_geodataentityqueryresult)
- [cx:CT_GeoDataPointToEntityQueryResult](#cx-ct_geodatapointtoentityqueryresult)
- [cx:CT_GeoChildEntitiesQueryResult](#cx-ct_geochildentitiesqueryresult)
- [cx:CT_GeoParentEntitiesQueryResult](#cx-ct_geoparententitiesqueryresult)
- [cx:CT_GeoLocationQuery](#cx-ct_geolocationquery)
- [cx:CT_GeoLocations](#cx-ct_geolocations)
- [cx:CT_GeoDataEntityQuery](#cx-ct_geodataentityquery)
- [cx:CT_GeoData](#cx-ct_geodata)
- [cx:CT_GeoDataPointQuery](#cx-ct_geodatapointquery)
- [cx:CT_GeoDataPointToEntityQuery](#cx-ct_geodatapointtoentityquery)
- [cx:CT_GeoChildEntitiesQuery](#cx-ct_geochildentitiesquery)
- [cx:CT_GeoChildEntities](#cx-ct_geochildentities)
- [cx:CT_GeoParentEntitiesQuery](#cx-ct_geoparententitiesquery)
- [cx:CT_GeoEntity](#cx-ct_geoentity)
- [cx:CT_GeoParentEntity](#cx-ct_geoparententity)
- [cx:CT_GeoLocation](#cx-ct_geolocation)
- [cx:CT_GeoPolygons](#cx-ct_geopolygons)
- [cx:CT_Copyrights](#cx-ct_copyrights)
- [cx:CT_GeoChildTypes](#cx-ct_geochildtypes)
- [cx:CT_GeoHierarchyEntity](#cx-ct_geohierarchyentity)
- [cx:CT_Address](#cx-ct_address)
- [cx:CT_GeoPolygon](#cx-ct_geopolygon)

<a id="cdr14-contentpart"></a>

## cdr14:contentPart

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `contentPart` — [cdr14:CT_ContentPart](chart-extensions.md#cdr14-ct_contentpart); MS-ODRAWXML.

<a id="c14-pivotoptions"></a>

## c14:pivotOptions

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `pivotOptions` — [c14:CT_PivotOptions](chart-extensions.md#c14-ct_pivotoptions); MS-ODRAWXML.

<a id="c14-invertsolidfillfmt"></a>

## c14:invertSolidFillFmt

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `invertSolidFillFmt` — [c14:CT_InvertSolidFillFmt](chart-extensions.md#c14-ct_invertsolidfillfmt); MS-ODRAWXML.

<a id="c14-style"></a>

## c14:style

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `style` — [c14:CT_Style](chart-extensions.md#c14-ct_style); MS-ODRAWXML.

<a id="c15-pivotsource"></a>

## c15:pivotSource

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `pivotSource` — [c:CT_PivotSource](charts.md#c-ct_pivotsource); MS-ODRAWXML.

<a id="c15-numfmt"></a>

## c15:numFmt

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); MS-ODRAWXML.

<a id="c15-sppr"></a>

## c15:spPr

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML.

<a id="c15-layout"></a>

## c15:layout

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `layout` — [c:CT_Layout](chart-extensions.md#c-ct_layout); MS-ODRAWXML.

<a id="c15-fullref"></a>

## c15:fullRef

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `fullRef` — [c15:CT_FullRef](chart-extensions.md#c15-ct_fullref); MS-ODRAWXML.

<a id="c15-levelref"></a>

## c15:levelRef

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `levelRef` — [c15:CT_LevelRef](chart-extensions.md#c15-ct_levelref); MS-ODRAWXML.

<a id="c15-formularef"></a>

## c15:formulaRef

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `formulaRef` — [c15:CT_FormulaRef](chart-extensions.md#c15-ct_formularef); MS-ODRAWXML.

<a id="c15-filteredseriestitle"></a>

## c15:filteredSeriesTitle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredSeriesTitle` — [c15:CT_FilteredSeriesTitle](chart-extensions.md#c15-ct_filteredseriestitle); MS-ODRAWXML.

<a id="c15-filteredcategorytitle"></a>

## c15:filteredCategoryTitle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredCategoryTitle` — [c15:CT_FilteredCategoryTitle](chart-extensions.md#c15-ct_filteredcategorytitle); MS-ODRAWXML.

<a id="c15-filteredareaseries"></a>

## c15:filteredAreaSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredAreaSeries` — [c15:CT_FilteredAreaSer](chart-extensions.md#c15-ct_filteredareaser); MS-ODRAWXML.

<a id="c15-filteredbarseries"></a>

## c15:filteredBarSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredBarSeries` — [c15:CT_FilteredBarSer](chart-extensions.md#c15-ct_filteredbarser); MS-ODRAWXML.

<a id="c15-filteredbubbleseries"></a>

## c15:filteredBubbleSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredBubbleSeries` — [c15:CT_FilteredBubbleSer](chart-extensions.md#c15-ct_filteredbubbleser); MS-ODRAWXML.

<a id="c15-filteredlineseries"></a>

## c15:filteredLineSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredLineSeries` — [c15:CT_FilteredLineSer](chart-extensions.md#c15-ct_filteredlineser); MS-ODRAWXML.

<a id="c15-filteredpieseries"></a>

## c15:filteredPieSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredPieSeries` — [c15:CT_FilteredPieSer](chart-extensions.md#c15-ct_filteredpieser); MS-ODRAWXML.

<a id="c15-filteredradarseries"></a>

## c15:filteredRadarSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredRadarSeries` — [c15:CT_FilteredRadarSer](chart-extensions.md#c15-ct_filteredradarser); MS-ODRAWXML.

<a id="c15-filteredscatterseries"></a>

## c15:filteredScatterSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredScatterSeries` — [c15:CT_FilteredScatterSer](chart-extensions.md#c15-ct_filteredscatterser); MS-ODRAWXML.

<a id="c15-filteredsurfaceseries"></a>

## c15:filteredSurfaceSeries

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `filteredSurfaceSeries` — [c15:CT_FilteredSurfaceSer](chart-extensions.md#c15-ct_filteredsurfaceser); MS-ODRAWXML.

<a id="c15-datalabelsrange"></a>

## c15:datalabelsRange

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `datalabelsRange` — [c15:CT_SeriesDataLabelsRange](chart-extensions.md#c15-ct_seriesdatalabelsrange); MS-ODRAWXML.

<a id="c15-categoryfilterexceptions"></a>

## c15:categoryFilterExceptions

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `categoryFilterExceptions` — [c15:CT_CategoryFilterExceptions](chart-extensions.md#c15-ct_categoryfilterexceptions); MS-ODRAWXML.

<a id="c15-dlblfieldtable"></a>

## c15:dlblFieldTable

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `dlblFieldTable` — [c15:CT_DataLabelFieldTable](chart-extensions.md#c15-ct_datalabelfieldtable); MS-ODRAWXML.

<a id="c15-xforsave"></a>

## c15:xForSave

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `xForSave` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML.

<a id="c15-showdatalabelsrange"></a>

## c15:showDataLabelsRange

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `showDataLabelsRange` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML.

<a id="c15-tx"></a>

## c15:tx

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tx` — [c:CT_Tx](chart-extensions.md#c-ct_tx); MS-ODRAWXML.

<a id="c15-showleaderlines"></a>

## c15:showLeaderLines

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `showLeaderLines` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML.

<a id="c15-leaderlines"></a>

## c15:leaderLines

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `leaderLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); MS-ODRAWXML.

<a id="c15-autocat"></a>

## c15:autoCat

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `autoCat` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML.

<a id="cs-colorstyle"></a>

## cs:colorStyle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `colorStyle` — [cs:CT_ColorStyle](chart-extensions.md#cs-ct_colorstyle); MS-ODRAWXML.

<a id="cs-chartstyle"></a>

## cs:chartStyle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chartStyle` — [cs:CT_ChartStyle](chart-extensions.md#cs-ct_chartstyle); MS-ODRAWXML.

<a id="c16-datapointuniqueidmap"></a>

## c16:datapointuniqueidmap

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `datapointuniqueidmap` — [c16:CT_ChartDataPointUniqueIDMap](chart-extensions.md#c16-ct_chartdatapointuniqueidmap); MS-ODRAWXML.

<a id="c16-sppr"></a>

## c16:spPr

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML.

<a id="c16-explosion"></a>

## c16:explosion

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `explosion` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); MS-ODRAWXML.

<a id="c16-invertifnegative"></a>

## c16:invertIfNegative

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `invertIfNegative` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML.

<a id="c16-bubble3d"></a>

## c16:bubble3D

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `bubble3D` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML.

<a id="c16-marker"></a>

## c16:marker

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); MS-ODRAWXML.

<a id="c16-dlbl"></a>

## c16:dLbl

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `dLbl` — [c:CT_DLbl](chart-extensions.md#c-ct_dlbl); MS-ODRAWXML.

<a id="c16-categoryfilterexceptions"></a>

## c16:categoryFilterExceptions

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `categoryFilterExceptions` — [c16:CT_CategoryFilterExceptions](chart-extensions.md#c16-ct_categoryfilterexceptions); MS-ODRAWXML.

<a id="c16-pivotoptions16"></a>

## c16:pivotOptions16

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `pivotOptions16` — [c16:CT_PivotOptions16](chart-extensions.md#c16-ct_pivotoptions16); MS-ODRAWXML.

<a id="c16ac-multilvlstrlit"></a>

## c16ac:multiLvlStrLit

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `multiLvlStrLit` — [c:CT_MultiLvlStrData](chart-extensions.md#c-ct_multilvlstrdata); MS-ODRAWXML.

<a id="cx-chartspace"></a>

## cx:chartSpace

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chartSpace` — [cx:CT_ChartSpace](chart-extensions.md#cx-ct_chartspace); MS-ODRAWXML.

<a id="cx-chart"></a>

## cx:chart

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chart` — [cx:CT_RelId](chart-extensions.md#cx-ct_relid); MS-ODRAWXML.

<a id="c16r3-datadisplayoptions16"></a>

## c16r3:dataDisplayOptions16

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `dataDisplayOptions16` — [c16r3:CT_DataDisplayOptions16](chart-extensions.md#c16r3-ct_datadisplayoptions16); MS-ODRAWXML.

<a id="c16r2-formatcode2"></a>

## c16r2:formatcode2

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `formatcode2` — [c:ST_Xstring](values-charts.md#c-st_xstring); MS-ODRAWXML.

<a id="c14-ct_booleantrue"></a>

## c14:CT_BooleanTrue

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=true.

<a id="c16-ct_numfilteredliteralcache"></a>

## c16:CT_NumFilteredLiteralCache

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `numCache` — [c:CT_NumData](chart-extensions.md#c-ct_numdata); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c16-ct_strfilteredliteralcache"></a>

## c16:CT_StrFilteredLiteralCache

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `strCache` — [c:CT_StrData](chart-extensions.md#c-ct_strdata); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c16-ct_multilvlstrfilteredliteralcache"></a>

## c16:CT_MultiLvlStrFilteredLiteralCache

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `multiLvlStrCache` — [c:CT_MultiLvlStrData](chart-extensions.md#c-ct_multilvlstrdata); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c16-ct_literaldatachart"></a>

## c16:CT_LiteralDataChart

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:boolean`; MS-ODRAWXML; use=required.

<a id="cx-ct_featureextension"></a>

## cx:CT_FeatureExtension

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; processContents=lax.
- [ ] `@feature` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@drop` — [cx:ST_ExtensionDropMode](values-charts.md#cx-st_extensiondropmode); MS-ODRAWXML; use=optional; default=never.

<a id="cdr14-ct_contentpart"></a>

## cdr14:CT_ContentPart

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `nvContentPartPr` — [cdr14:CT_ContentPartNonVisual](chart-extensions.md#cdr14-ct_contentpartnonvisual); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `nvPr` — [cdr14:CT_ApplicationNonVisualDrawingProps](chart-extensions.md#cdr14-ct_applicationnonvisualdrawingprops); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `xfrm` — [a:CT_Transform2D](images.md#a-ct_transform2d); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-ODRAWXML; use=required.
- [ ] `@bwMode` — [a:ST_BlackWhiteMode](values-drawing.md#a-st_blackwhitemode); MS-ODRAWXML; use=optional; default=auto.

<a id="c14-ct_pivotoptions"></a>

## c14:CT_PivotOptions

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `dropZoneFilter` — [c14:CT_BooleanFalse](chart-extensions.md#c14-ct_booleanfalse); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dropZoneCategories` — [c14:CT_BooleanFalse](chart-extensions.md#c14-ct_booleanfalse); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dropZoneData` — [c14:CT_BooleanFalse](chart-extensions.md#c14-ct_booleanfalse); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dropZoneSeries` — [c14:CT_BooleanFalse](chart-extensions.md#c14-ct_booleanfalse); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dropZonesVisible` — [c14:CT_BooleanFalse](chart-extensions.md#c14-ct_booleanfalse); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="c14-ct_invertsolidfillfmt"></a>

## c14:CT_InvertSolidFillFmt

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c14-ct_style"></a>

## c14:CT_Style

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — [c14:ST_Style](values-charts.md#c14-st_style); MS-ODRAWXML; use=required.

<a id="c-ct_numfmt"></a>

## c:CT_NumFmt

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@formatCode` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; use=required.
- [ ] `@sourceLinked` — `xsd:boolean`; Strict, Transitional.

<a id="c-ct_layout"></a>

## c:CT_Layout

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `manualLayout` — [c:CT_ManualLayout](chart-extensions.md#c-ct_manuallayout); Strict, Transitional; minOccurs=0; maxOccurs=1. Manual Layout; [§21.2.2.104](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3413-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c15-ct_fullref"></a>

## c15:CT_FullRef

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `sqref` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_levelref"></a>

## c15:CT_LevelRef

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `sqref` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_formularef"></a>

## c15:CT_FormulaRef

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `sqref` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredseriestitle"></a>

## c15:CT_FilteredSeriesTitle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredcategorytitle"></a>

## c15:CT_FilteredCategoryTitle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `cat` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredareaser"></a>

## c15:CT_FilteredAreaSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_AreaSer](chart-extensions.md#c-ct_areaser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredbarser"></a>

## c15:CT_FilteredBarSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_BarSer](chart-extensions.md#c-ct_barser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredbubbleser"></a>

## c15:CT_FilteredBubbleSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_BubbleSer](chart-extensions.md#c-ct_bubbleser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredlineser"></a>

## c15:CT_FilteredLineSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_LineSer](chart-extensions.md#c-ct_lineser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredpieser"></a>

## c15:CT_FilteredPieSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_PieSer](chart-extensions.md#c-ct_pieser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredradarser"></a>

## c15:CT_FilteredRadarSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_RadarSer](chart-extensions.md#c-ct_radarser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredscatterser"></a>

## c15:CT_FilteredScatterSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_ScatterSer](chart-extensions.md#c-ct_scatterser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_filteredsurfaceser"></a>

## c15:CT_FilteredSurfaceSer

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ser` — [c:CT_SurfaceSer](chart-extensions.md#c-ct_surfaceser); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c15-ct_seriesdatalabelsrange"></a>

## c15:CT_SeriesDataLabelsRange

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `f` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dlblRangeCache` — [c:CT_StrData](chart-extensions.md#c-ct_strdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="c15-ct_categoryfilterexceptions"></a>

## c15:CT_CategoryFilterExceptions

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `categoryFilterException` — [c15:CT_CategoryFilterException](chart-extensions.md#c15-ct_categoryfilterexception); MS-ODRAWXML; minOccurs=1; maxOccurs=unbounded.

<a id="c15-ct_datalabelfieldtable"></a>

## c15:CT_DataLabelFieldTable

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `dlblFTEntry` — [c15:CT_DataLabelFieldTableEntry](chart-extensions.md#c15-ct_datalabelfieldtableentry); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="c-ct_tx"></a>

## c:CT_Tx

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `strRef` — [c:CT_StrRef](chart-extensions.md#c-ct_strref); Strict, Transitional; minOccurs=1; maxOccurs=1. String Reference; [§21.2.2.201](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3444-part-1).
- [ ] `rich` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=1; maxOccurs=1. Rich Text; [§21.2.2.156](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3431-part-1).

<a id="c-ct_chartlines"></a>

## c:CT_ChartLines

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).

<a id="cs-ct_colorstyle"></a>

## cs:CT_ColorStyle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `variation` — [cs:CT_ColorStyleVariation](chart-extensions.md#cs-ct_colorstylevariation); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@meth` — [cs:ST_ColorStyleMethod](values-charts.md#cs-st_colorstylemethod); MS-ODRAWXML; use=required.
- [ ] `@id` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional.

<a id="cs-ct_chartstyle"></a>

## cs:CT_ChartStyle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `axisTitle` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `categoryAxis` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `chartArea` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dataLabel` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dataLabelCallout` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dataPoint` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dataPoint3D` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dataPointLine` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dataPointMarker` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dataPointMarkerLayout` — [cs:CT_MarkerLayout](chart-extensions.md#cs-ct_markerlayout); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dataPointWireframe` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dataTable` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `downBar` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dropLine` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `errorBar` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `floor` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `gridlineMajor` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `gridlineMinor` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hiLoLine` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `leaderLine` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `legend` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `plotArea` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `plotArea3D` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `seriesAxis` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `seriesLine` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `title` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `trendline` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `trendlineLabel` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `upBar` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `valueAxis` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `wall` — [cs:CT_StyleEntry](chart-extensions.md#cs-ct_styleentry); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@id` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional.

<a id="c16-ct_chartdatapointuniqueidmap"></a>

## c16:CT_ChartDataPointUniqueIDMap

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ptentry` — [c16:CT_ChartDataPointUniqueIDMapEntry](chart-extensions.md#c16-ct_chartdatapointuniqueidmapentry); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="c-ct_unsignedint"></a>

## c:CT_UnsignedInt

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — `xsd:unsignedInt`; Strict, Transitional; use=required.

<a id="c-ct_marker"></a>

## c:CT_Marker

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `symbol` — [c:CT_MarkerStyle](chart-extensions.md#c-ct_markerstyle); Strict, Transitional; minOccurs=0; maxOccurs=1. Symbol; [§21.2.2.205](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3445-part-1).
- [ ] `size` — [c:CT_MarkerSize](chart-extensions.md#c-ct_markersize); Strict, Transitional; minOccurs=0; maxOccurs=1. Size; [§21.2.2.192](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3442-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_dlbl"></a>

## c:CT_DLbl

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `delete` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=1; maxOccurs=1. Delete; [§21.2.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `layout` — [c:CT_Layout](chart-extensions.md#c-ct_layout); Strict, Transitional; minOccurs=0; maxOccurs=1. Layout; [§21.2.2.88](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `tx` — [c:CT_Tx](chart-extensions.md#c-ct_tx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Format; [§21.2.2.121](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `dLblPos` — [c:CT_DLblPos](chart-extensions.md#c-ct_dlblpos); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Label Position; [§21.2.2.48](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `showLegendKey` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Legend Key; [§21.2.2.184](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3438-part-1).
- [ ] `showVal` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Value; [§21.2.2.189](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3441-part-1).
- [ ] `showCatName` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Category Name; [§21.2.2.179](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3436-part-1).
- [ ] `showSerName` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Series Name; [§21.2.2.188](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3440-part-1).
- [ ] `showPercent` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Percent; [§21.2.2.187](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3440-part-1).
- [ ] `showBubbleSize` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Bubble Size; [§21.2.2.178](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3436-part-1).
- [ ] `separator` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1. Separator; [§21.2.2.166](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3434-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c16-ct_categoryfilterexceptions"></a>

## c16:CT_CategoryFilterExceptions

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `categoryFilterException` — [c16:CT_CategoryFilterException](chart-extensions.md#c16-ct_categoryfilterexception); MS-ODRAWXML; minOccurs=1; maxOccurs=unbounded.

<a id="c16-ct_pivotoptions16"></a>

## c16:CT_PivotOptions16

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `showExpandCollapseFieldButtons` — [c16:CT_BooleanFalse](chart-extensions.md#c16-ct_booleanfalse); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="c-ct_multilvlstrdata"></a>

## c:CT_MultiLvlStrData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `ptCount` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=0; maxOccurs=1. Point Count; [§21.2.2.152](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3430-part-1).
- [ ] `lvl` — [c:CT_Lvl](chart-extensions.md#c-ct_lvl); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Level; [§21.2.2.99](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3412-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="cx-ct_chartspace"></a>

## cx:CT_ChartSpace

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chartData` — [cx:CT_ChartData](chart-extensions.md#cx-ct_chartdata); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `chart` — [cx:CT_Chart](chart-extensions.md#cx-ct_chart); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dashboard` — [cx:CT_Dashboard](chart-extensions.md#cx-ct_dashboard); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `clrMapOvr` — [a:CT_ColorMapping](charts.md#a-ct_colormapping); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `fmtOvrs` — [cx:CT_FormatOverrides](chart-extensions.md#cx-ct_formatoverrides); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `printSettings` — [cx:CT_PrintSettings](chart-extensions.md#cx-ct_printsettings); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@version` — `xsd:string`; MS-ODRAWXML; use=optional; default=0.0.
- [ ] `@featureList` — `xsd:string`; MS-ODRAWXML; use=optional; default=.
- [ ] `@fallbackImg` — `xsd:string`; MS-ODRAWXML; use=optional; default=.

<a id="cx-ct_relid"></a>

## cx:CT_RelId

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-ODRAWXML; use=required.

<a id="c16r3-ct_datadisplayoptions16"></a>

## c16r3:CT_DataDisplayOptions16

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `dispNaAsBlank` — [c16r3:CT_BooleanFalse](chart-extensions.md#c16r3-ct_booleanfalse); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="c-ct_numdata"></a>

## c:CT_NumData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `formatCode` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=0; maxOccurs=1. Format Code; [§21.2.2.71](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3404-part-1).
- [ ] `ptCount` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=0; maxOccurs=1. Point Count; [§21.2.2.152](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3430-part-1).
- [ ] `pt` — [c:CT_NumVal](chart-extensions.md#c-ct_numval); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_strdata"></a>

## c:CT_StrData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `ptCount` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=0; maxOccurs=1. Point Count; [§21.2.2.152](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3430-part-1).
- [ ] `pt` — [c:CT_StrVal](chart-extensions.md#c-ct_strval); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="cdr14-ct_contentpartnonvisual"></a>

## cdr14:CT_ContentPartNonVisual

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `cNvContentPartPr` — [a14:CT_NonVisualInkContentPartProperties](presentation-extensions.md#a14-ct_nonvisualinkcontentpartproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cdr14-ct_applicationnonvisualdrawingprops"></a>

## cdr14:CT_ApplicationNonVisualDrawingProps

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@macro` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@fPublished` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.

<a id="c14-ct_booleanfalse"></a>

## c14:CT_BooleanFalse

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.

<a id="c-ct_manuallayout"></a>

## c:CT_ManualLayout

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `layoutTarget` — [c:CT_LayoutTarget](chart-extensions.md#c-ct_layouttarget); Strict, Transitional; minOccurs=0; maxOccurs=1. Layout Target; [§21.2.2.89](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `xMode` — [c:CT_LayoutMode](chart-extensions.md#c-ct_layoutmode); Strict, Transitional; minOccurs=0; maxOccurs=1. Left Mode; [§21.2.2.233](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3452-part-1).
- [ ] `yMode` — [c:CT_LayoutMode](chart-extensions.md#c-ct_layoutmode); Strict, Transitional; minOccurs=0; maxOccurs=1. Top Mode; [§21.2.2.236](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3452-part-1).
- [ ] `wMode` — [c:CT_LayoutMode](chart-extensions.md#c-ct_layoutmode); Strict, Transitional; minOccurs=0; maxOccurs=1. Width Mode; [§21.2.2.231](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3451-part-1).
- [ ] `hMode` — [c:CT_LayoutMode](chart-extensions.md#c-ct_layoutmode); Strict, Transitional; minOccurs=0; maxOccurs=1. Height Mode; [§21.2.2.81](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3407-part-1).
- [ ] `x` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Left; [§21.2.2.232](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3451-part-1).
- [ ] `y` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Top; [§21.2.2.235](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3452-part-1).
- [ ] `w` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Width; [§21.2.2.229](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3450-part-1).
- [ ] `h` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Height; [§21.2.2.78](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3406-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_sertx"></a>

## c:CT_SerTx

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `strRef` — [c:CT_StrRef](chart-extensions.md#c-ct_strref); Strict, Transitional; minOccurs=1; maxOccurs=1. String Reference; [§21.2.2.201](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3444-part-1).
- [ ] `v` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="c-ct_axdatasource"></a>

## c:CT_AxDataSource

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `multiLvlStrRef` — [c:CT_MultiLvlStrRef](chart-extensions.md#c-ct_multilvlstrref); Strict, Transitional; minOccurs=1; maxOccurs=1. Multi Level String Reference; [§21.2.2.115](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3416-part-1).
- [ ] `numRef` — [c:CT_NumRef](chart-extensions.md#c-ct_numref); Strict, Transitional; minOccurs=1; maxOccurs=1. Number Reference; [§21.2.2.123](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `numLit` — [c:CT_NumData](chart-extensions.md#c-ct_numdata); Strict, Transitional; minOccurs=1; maxOccurs=1. Number Literal; [§21.2.2.122](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `strRef` — [c:CT_StrRef](chart-extensions.md#c-ct_strref); Strict, Transitional; minOccurs=1; maxOccurs=1. String Reference; [§21.2.2.201](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3444-part-1).
- [ ] `strLit` — [c:CT_StrData](chart-extensions.md#c-ct_strdata); Strict, Transitional; minOccurs=1; maxOccurs=1. String Literal; [§21.2.2.200](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3444-part-1).

<a id="c-ct_areaser"></a>

## c:CT_AreaSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `pictureOptions` — [c:CT_PictureOptions](charts.md#c-ct_pictureoptions); Strict, Transitional; minOccurs=0; maxOccurs=1. Picture Options; [§21.2.2.138](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3427-part-1).
- [ ] `dPt` — [c:CT_DPt](chart-extensions.md#c-ct_dpt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Point; [§21.2.2.52](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `trendline` — [c:CT_Trendline](chart-extensions.md#c-ct_trendline); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Trendlines; [§21.2.2.211](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `errBars` — [c:CT_ErrBars](chart-extensions.md#c-ct_errbars); Strict, Transitional; minOccurs=0; maxOccurs=2. Error Bars; [§21.2.2.55](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `cat` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Category Axis Data; [§21.2.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3388-part-1).
- [ ] `val` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_barser"></a>

## c:CT_BarSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `invertIfNegative` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Invert if Negative; [§21.2.2.86](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3409-part-1).
- [ ] `pictureOptions` — [c:CT_PictureOptions](charts.md#c-ct_pictureoptions); Strict, Transitional; minOccurs=0; maxOccurs=1. Picture Options; [§21.2.2.138](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3427-part-1).
- [ ] `dPt` — [c:CT_DPt](chart-extensions.md#c-ct_dpt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Point; [§21.2.2.52](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `trendline` — [c:CT_Trendline](chart-extensions.md#c-ct_trendline); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Trendlines; [§21.2.2.211](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `errBars` — [c:CT_ErrBars](chart-extensions.md#c-ct_errbars); Strict, Transitional; minOccurs=0; maxOccurs=1. Error Bars; [§21.2.2.55](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `cat` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Category Axis Data; [§21.2.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3388-part-1).
- [ ] `val` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `shape` — [c:CT_Shape](chart-extensions.md#c-ct_shape); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape; [§21.2.2.177](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3436-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_bubbleser"></a>

## c:CT_BubbleSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `invertIfNegative` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Invert if Negative; [§21.2.2.86](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3409-part-1).
- [ ] `dPt` — [c:CT_DPt](chart-extensions.md#c-ct_dpt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Point; [§21.2.2.52](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `trendline` — [c:CT_Trendline](chart-extensions.md#c-ct_trendline); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Trendlines; [§21.2.2.211](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `errBars` — [c:CT_ErrBars](chart-extensions.md#c-ct_errbars); Strict, Transitional; minOccurs=0; maxOccurs=2. Error Bars; [§21.2.2.55](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `xVal` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. X Values; [§21.2.2.234](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3452-part-1).
- [ ] `yVal` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Y Values; [§21.2.2.237](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3453-part-1).
- [ ] `bubbleSize` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Bubble Size; [§21.2.2.22](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3387-part-1).
- [ ] `bubble3D` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. 3D Bubble; [§21.2.2.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_lineser"></a>

## c:CT_LineSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `dPt` — [c:CT_DPt](chart-extensions.md#c-ct_dpt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Point; [§21.2.2.52](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `trendline` — [c:CT_Trendline](chart-extensions.md#c-ct_trendline); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Trendlines; [§21.2.2.211](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `errBars` — [c:CT_ErrBars](chart-extensions.md#c-ct_errbars); Strict, Transitional; minOccurs=0; maxOccurs=1. Error Bars; [§21.2.2.55](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `cat` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Category Axis Data; [§21.2.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3388-part-1).
- [ ] `val` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `smooth` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Smoothing; [§21.2.2.194](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3442-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_pieser"></a>

## c:CT_PieSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `explosion` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=0; maxOccurs=1. Explosion; [§21.2.2.61](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3400-part-1).
- [ ] `dPt` — [c:CT_DPt](chart-extensions.md#c-ct_dpt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Point; [§21.2.2.52](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `cat` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Category Axis Data; [§21.2.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3388-part-1).
- [ ] `val` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_radarser"></a>

## c:CT_RadarSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `dPt` — [c:CT_DPt](chart-extensions.md#c-ct_dpt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Point; [§21.2.2.52](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `cat` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Category Axis Data; [§21.2.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3388-part-1).
- [ ] `val` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_scatterser"></a>

## c:CT_ScatterSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `dPt` — [c:CT_DPt](chart-extensions.md#c-ct_dpt); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Point; [§21.2.2.52](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `dLbls` — [c:CT_DLbls](chart-extensions.md#c-ct_dlbls); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Labels; [§21.2.2.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `trendline` — [c:CT_Trendline](chart-extensions.md#c-ct_trendline); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Trendlines; [§21.2.2.211](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3446-part-1).
- [ ] `errBars` — [c:CT_ErrBars](chart-extensions.md#c-ct_errbars); Strict, Transitional; minOccurs=0; maxOccurs=2. Error Bars; [§21.2.2.55](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `xVal` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. X Values; [§21.2.2.234](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3452-part-1).
- [ ] `yVal` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Y Values; [§21.2.2.237](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3449-3454-0640.md#source-page-3453-part-1).
- [ ] `smooth` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Smoothing; [§21.2.2.194](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3442-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_surfaceser"></a>

## c:CT_SurfaceSer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `order` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tx` — [c:CT_SerTx](chart-extensions.md#c-ct_sertx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `cat` — [c:CT_AxDataSource](chart-extensions.md#c-ct_axdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Category Axis Data; [§21.2.2.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3388-3393-0630.md#source-page-3388-part-1).
- [ ] `val` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c15-ct_categoryfilterexception"></a>

## c15:CT_CategoryFilterException

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `sqref` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `explosion` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `invertIfNegative` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `bubble3D` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dLbl` — [c:CT_DLbl](chart-extensions.md#c-ct_dlbl); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="c15-ct_datalabelfieldtableentry"></a>

## c15:CT_DataLabelFieldTableEntry

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `txfldGUID` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `f` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `dlblFieldTableCache` — [c:CT_StrData](chart-extensions.md#c-ct_strdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="c-ct_strref"></a>

## c:CT_StrRef

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `f` — `xsd:string`; Strict, Transitional; minOccurs=1; maxOccurs=1. Formula; [§21.2.2.65](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).
- [ ] `strCache` — [c:CT_StrData](chart-extensions.md#c-ct_strdata); Strict, Transitional; minOccurs=0; maxOccurs=1. String Cache; [§21.2.2.199](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3444-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="cs-ct_colorstylevariation"></a>

## cs:CT_ColorStyleVariation

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="cs-ct_styleentry"></a>

## cs:CT_StyleEntry

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `lnRef` — [cs:CT_StyleReference](chart-extensions.md#cs-ct_stylereference); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `lineWidthScale` — `xsd:double`; MS-ODRAWXML; minOccurs=0; maxOccurs=1; default=1.0.
- [ ] `fillRef` — [cs:CT_StyleReference](chart-extensions.md#cs-ct_stylereference); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `effectRef` — [cs:CT_StyleReference](chart-extensions.md#cs-ct_stylereference); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `fontRef` — [cs:CT_FontReference](chart-extensions.md#cs-ct_fontreference); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `defRPr` — [a:CT_TextCharacterProperties](drawing-text.md#a-ct_textcharacterproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `bodyPr` — [a:CT_TextBodyProperties](drawing-text.md#a-ct_textbodyproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@mods` — [cs:ST_StyleEntryModifierList](values-charts.md#cs-st_styleentrymodifierlist); MS-ODRAWXML; use=optional.

<a id="cs-ct_markerlayout"></a>

## cs:CT_MarkerLayout

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@symbol` — [cs:ST_MarkerStyle](values-charts.md#cs-st_markerstyle); MS-ODRAWXML; use=optional.
- [ ] `@size` — [cs:ST_MarkerSize](values-charts.md#cs-st_markersize); MS-ODRAWXML; use=optional.

<a id="c16-ct_chartdatapointuniqueidmapentry"></a>

## c16:CT_ChartDataPointUniqueIDMapEntry

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ptidx` — `xsd:unsignedInt`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `uniqueID` — [c16:CT_ChartUniqueID](chart-extensions.md#c16-ct_chartuniqueid); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="c-ct_markerstyle"></a>

## c:CT_MarkerStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_MarkerStyle](values-charts.md#c-st_markerstyle); Strict, Transitional; use=required.

<a id="c-ct_markersize"></a>

## c:CT_MarkerSize

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_MarkerSize](values-charts.md#c-st_markersize); Strict, Transitional; default=5.

<a id="c-ct_dlblpos"></a>

## c:CT_DLblPos

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_DLblPos](values-charts.md#c-st_dlblpos); Strict, Transitional; use=required.

<a id="c16-ct_categoryfilterexception"></a>

## c16:CT_CategoryFilterException

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `uniqueId` — [c16:CT_ChartUniqueID](chart-extensions.md#c16-ct_chartuniqueid); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `c16:spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `c16:explosion` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `c16:invertIfNegative` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `c16:bubble3D` — [c:CT_Boolean](charts.md#c-ct_boolean); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `c16:marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `c16:dLbl` — [c:CT_DLbl](chart-extensions.md#c-ct_dlbl); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="c16-ct_booleanfalse"></a>

## c16:CT_BooleanFalse

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.

<a id="c-ct_lvl"></a>

## c:CT_Lvl

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `pt` — [c:CT_StrVal](chart-extensions.md#c-ct_strval); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_chartdata"></a>

## cx:CT_ChartData

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `externalData` — [cx:CT_ExternalData](chart-extensions.md#cx-ct_externaldata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `data` — [cx:CT_Data](chart-extensions.md#cx-ct_data); MS-ODRAWXML; minOccurs=1; maxOccurs=unbounded.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_chart"></a>

## cx:CT_Chart

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `title` — [cx:CT_ChartTitle](chart-extensions.md#cx-ct_charttitle); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `plotArea` — [cx:CT_PlotArea](chart-extensions.md#cx-ct_plotarea); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `legend` — [cx:CT_Legend](chart-extensions.md#cx-ct_legend); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_dashboard"></a>

## cx:CT_Dashboard

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `bindings` — [cx:CT_Bindings](chart-extensions.md#cx-ct_bindings); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `artifacts` — [cx:CT_Artifacts](chart-extensions.md#cx-ct_artifacts); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@entrypoint` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_formatoverrides"></a>

## cx:CT_FormatOverrides

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `fmtOvr` — [cx:CT_FormatOverride](chart-extensions.md#cx-ct_formatoverride); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_printsettings"></a>

## cx:CT_PrintSettings

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `headerFooter` — [cx:CT_HeaderFooter](chart-extensions.md#cx-ct_headerfooter); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `pageMargins` — [cx:CT_PageMargins](chart-extensions.md#cx-ct_pagemargins); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `pageSetup` — [cx:CT_PageSetup](chart-extensions.md#cx-ct_pagesetup); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_extensionlist"></a>

## cx:CT_ExtensionList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `ext` — [cx:CT_Extension](chart-extensions.md#cx-ct_extension); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="c16r3-ct_booleanfalse"></a>

## c16r3:CT_BooleanFalse

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.

<a id="c-ct_numval"></a>

## c:CT_NumVal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `v` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@idx` — `xsd:unsignedInt`; Strict, Transitional; use=required.
- [ ] `@formatCode` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; use=optional.

<a id="c-ct_strval"></a>

## c:CT_StrVal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `v` — [s:ST_Xstring](values-shared.md#s-st_xstring); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@idx` — `xsd:unsignedInt`; Strict, Transitional; use=required.

<a id="c-ct_layouttarget"></a>

## c:CT_LayoutTarget

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_LayoutTarget](values-charts.md#c-st_layouttarget); Strict, Transitional; default=outer.

<a id="c-ct_layoutmode"></a>

## c:CT_LayoutMode

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_LayoutMode](values-charts.md#c-st_layoutmode); Strict, Transitional; default=factor.

<a id="c-ct_double"></a>

## c:CT_Double

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — `xsd:double`; Strict, Transitional; use=required.

<a id="c-ct_multilvlstrref"></a>

## c:CT_MultiLvlStrRef

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `f` — `xsd:string`; Strict, Transitional; minOccurs=1; maxOccurs=1. Formula; [§21.2.2.65](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).
- [ ] `multiLvlStrCache` — [c:CT_MultiLvlStrData](chart-extensions.md#c-ct_multilvlstrdata); Strict, Transitional; minOccurs=0; maxOccurs=1. Multi Level String Cache; [§21.2.2.114](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3416-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_numref"></a>

## c:CT_NumRef

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `f` — `xsd:string`; Strict, Transitional; minOccurs=1; maxOccurs=1. Formula; [§21.2.2.65](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).
- [ ] `numCache` — [c:CT_NumData](chart-extensions.md#c-ct_numdata); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Cache; [§21.2.2.120](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_dpt"></a>

## c:CT_DPt

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `idx` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=1; maxOccurs=1. Index; [§21.2.2.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3408-part-1).
- [ ] `invertIfNegative` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Invert if Negative; [§21.2.2.86](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3409-part-1).
- [ ] `marker` — [c:CT_Marker](chart-extensions.md#c-ct_marker); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `bubble3D` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. 3D Bubble; [§21.2.2.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3386-part-1).
- [ ] `explosion` — [c:CT_UnsignedInt](chart-extensions.md#c-ct_unsignedint); Strict, Transitional; minOccurs=0; maxOccurs=1. Explosion; [§21.2.2.61](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3400-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `pictureOptions` — [c:CT_PictureOptions](charts.md#c-ct_pictureoptions); Strict, Transitional; minOccurs=0; maxOccurs=1. Picture Options; [§21.2.2.138](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3427-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_dlbls"></a>

## c:CT_DLbls

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `dLbl` — [c:CT_DLbl](chart-extensions.md#c-ct_dlbl); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Data Label; [§21.2.2.47](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3396-part-1).
- [ ] `delete` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=1; maxOccurs=1. Delete; [§21.2.2.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3394-part-1).
- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Format; [§21.2.2.121](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `dLblPos` — [c:CT_DLblPos](chart-extensions.md#c-ct_dlblpos); Strict, Transitional; minOccurs=0; maxOccurs=1. Data Label Position; [§21.2.2.48](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3397-part-1).
- [ ] `showLegendKey` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Legend Key; [§21.2.2.184](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3438-part-1).
- [ ] `showVal` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Value; [§21.2.2.189](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3441-part-1).
- [ ] `showCatName` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Category Name; [§21.2.2.179](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3436-part-1).
- [ ] `showSerName` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Series Name; [§21.2.2.188](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3440-part-1).
- [ ] `showPercent` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Percent; [§21.2.2.187](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3440-part-1).
- [ ] `showBubbleSize` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Bubble Size; [§21.2.2.178](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3436-part-1).
- [ ] `separator` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1. Separator; [§21.2.2.166](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3431-3436-0637.md#source-page-3434-part-1).
- [ ] `showLeaderLines` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Show Leader Lines; [§21.2.2.183](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3437-3442-0638.md#source-page-3438-part-1).
- [ ] `leaderLines` — [c:CT_ChartLines](chart-extensions.md#c-ct_chartlines); Strict, Transitional; minOccurs=0; maxOccurs=1. Leader Lines; [§21.2.2.92](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3411-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_trendline"></a>

## c:CT_Trendline

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `name` — `xsd:string`; Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `trendlineType` — [c:CT_TrendlineType](chart-extensions.md#c-ct_trendlinetype); Strict, Transitional; minOccurs=1; maxOccurs=1. Trendline Type; [§21.2.2.213](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `order` — [c:CT_Order](chart-extensions.md#c-ct_order); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `period` — [c:CT_Period](chart-extensions.md#c-ct_period); Strict, Transitional; minOccurs=0; maxOccurs=1. Period; [§21.2.2.135](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3426-part-1).
- [ ] `forward` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Forward; [§21.2.2.73](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3404-part-1).
- [ ] `backward` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Backward; [§21.2.2.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3382-3387-0629.md#source-page-3385-part-1).
- [ ] `intercept` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1. Intercept; [§21.2.2.85](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3409-part-1).
- [ ] `dispRSqr` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Display R Squared Value; [§21.2.2.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3396-part-1).
- [ ] `dispEq` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. Display Equation; [§21.2.2.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3395-part-1).
- [ ] `trendlineLbl` — [c:CT_TrendlineLbl](chart-extensions.md#c-ct_trendlinelbl); Strict, Transitional; minOccurs=0; maxOccurs=1. Trendline Label; [§21.2.2.212](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_errbars"></a>

## c:CT_ErrBars

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `errDir` — [c:CT_ErrDir](chart-extensions.md#c-ct_errdir); Strict, Transitional; minOccurs=0; maxOccurs=1. Error Bar Direction; [§21.2.2.57](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `errBarType` — [c:CT_ErrBarType](chart-extensions.md#c-ct_errbartype); Strict, Transitional; minOccurs=1; maxOccurs=1. Error Bar Type; [§21.2.2.56](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `errValType` — [c:CT_ErrValType](chart-extensions.md#c-ct_errvaltype); Strict, Transitional; minOccurs=1; maxOccurs=1. Error Bar Value Type; [§21.2.2.58](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3394-3400-0631.md#source-page-3398-part-1).
- [ ] `noEndCap` — [c:CT_Boolean](charts.md#c-ct_boolean); Strict, Transitional; minOccurs=0; maxOccurs=1. No End Cap; [§21.2.2.118](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3417-part-1).
- [ ] `plus` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Plus; [§21.2.2.147](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3425-3430-0636.md#source-page-3429-part-1).
- [ ] `minus` — [c:CT_NumDataSource](chart-extensions.md#c-ct_numdatasource); Strict, Transitional; minOccurs=0; maxOccurs=1. Minus; [§21.2.2.113](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3416-part-1).
- [ ] `val` — [c:CT_Double](chart-extensions.md#c-ct_double); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_numdatasource"></a>

## c:CT_NumDataSource

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `numRef` — [c:CT_NumRef](chart-extensions.md#c-ct_numref); Strict, Transitional; minOccurs=1; maxOccurs=1. Number Reference; [§21.2.2.123](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `numLit` — [c:CT_NumData](chart-extensions.md#c-ct_numdata); Strict, Transitional; minOccurs=1; maxOccurs=1. Number Literal; [§21.2.2.122](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).

<a id="c-ct_shape"></a>

## c:CT_Shape

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Shape](values-charts.md#c-st_shape); Strict, Transitional; default=box.

<a id="cs-ct_stylereference"></a>

## cs:CT_StyleReference

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `styleClr` — [cs:CT_StyleColor](chart-extensions.md#cs-ct_stylecolor); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@idx` — [a:ST_StyleMatrixColumnIndex](values-drawing.md#a-st_stylematrixcolumnindex); MS-ODRAWXML; use=required.
- [ ] `@mods` — [cs:ST_StyleReferenceModifierList](values-charts.md#cs-st_stylereferencemodifierlist); MS-ODRAWXML; use=optional.

<a id="cs-ct_fontreference"></a>

## cs:CT_FontReference

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `styleClr` — [cs:CT_StyleColor](chart-extensions.md#cs-ct_stylecolor); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@idx` — [a:ST_FontCollectionIndex](values-drawing.md#a-st_fontcollectionindex); MS-ODRAWXML; use=required.
- [ ] `@mods` — [cs:ST_StyleReferenceModifierList](values-charts.md#cs-st_stylereferencemodifierlist); MS-ODRAWXML; use=optional.

<a id="c16-ct_chartuniqueid"></a>

## c16:CT_ChartUniqueID

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_externaldata"></a>

## cx:CT_ExternalData

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-ODRAWXML; use=required.
- [ ] `@autoUpdate` — `xsd:boolean`; MS-ODRAWXML; use=optional.

<a id="cx-ct_data"></a>

## cx:CT_Data

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `numDim` — [cx:CT_NumericDimension](chart-extensions.md#cx-ct_numericdimension); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `strDim` — [cx:CT_StringDimension](chart-extensions.md#cx-ct_stringdimension); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [cx:ST_DataId](values-charts.md#cx-st_dataid); MS-ODRAWXML; use=required.

<a id="cx-ct_charttitle"></a>

## cx:CT_ChartTitle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tx` — [cx:CT_Text](chart-extensions.md#cx-ct_text); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `offset` — [cx:CT_Offset](chart-extensions.md#cx-ct_offset); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@pos` — [cx:ST_SidePos](values-charts.md#cx-st_sidepos); MS-ODRAWXML; use=optional; default=t.
- [ ] `@align` — [cx:ST_PosAlign](values-charts.md#cx-st_posalign); MS-ODRAWXML; use=optional; default=ctr.
- [ ] `@overlay` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=0.

<a id="cx-ct_plotarea"></a>

## cx:CT_PlotArea

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `plotAreaRegion` — [cx:CT_PlotAreaRegion](chart-extensions.md#cx-ct_plotarearegion); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `axis` — [cx:CT_Axis](chart-extensions.md#cx-ct_axis); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_legend"></a>

## cx:CT_Legend

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `offset` — [cx:CT_Offset](chart-extensions.md#cx-ct_offset); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@pos` — [cx:ST_SidePos](values-charts.md#cx-st_sidepos); MS-ODRAWXML; use=optional; default=r.
- [ ] `@align` — [cx:ST_PosAlign](values-charts.md#cx-st_posalign); MS-ODRAWXML; use=optional; default=ctr.
- [ ] `@overlay` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=0.

<a id="cx-ct_bindings"></a>

## cx:CT_Bindings

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `binding` — [cx:CT_Binding](chart-extensions.md#cx-ct_binding); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_artifacts"></a>

## cx:CT_Artifacts

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `artifact` — [cx:CT_Artifact](chart-extensions.md#cx-ct_artifact); MS-ODRAWXML; minOccurs=1; maxOccurs=unbounded.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_formatoverride"></a>

## cx:CT_FormatOverride

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@idx` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.

<a id="cx-ct_headerfooter"></a>

## cx:CT_HeaderFooter

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `oddHeader` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `oddFooter` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `evenHeader` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `evenFooter` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `firstHeader` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `firstFooter` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@alignWithMargins` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=true.
- [ ] `@differentOddEven` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@differentFirst` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.

<a id="cx-ct_pagemargins"></a>

## cx:CT_PageMargins

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@l` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@r` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@t` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@b` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@header` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@footer` — `xsd:double`; MS-ODRAWXML; use=required.

<a id="cx-ct_pagesetup"></a>

## cx:CT_PageSetup

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@paperSize` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional; default=1.
- [ ] `@firstPageNumber` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional; default=1.
- [ ] `@orientation` — [cx:ST_PageOrientation](values-charts.md#cx-st_pageorientation); MS-ODRAWXML; use=optional; default=default.
- [ ] `@blackAndWhite` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@draft` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@useFirstPageNumber` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@horizontalDpi` — `xsd:int`; MS-ODRAWXML; use=optional; default=600.
- [ ] `@verticalDpi` — `xsd:int`; MS-ODRAWXML; use=optional; default=600.
- [ ] `@copies` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional; default=1.

<a id="cx-ct_extension"></a>

## cx:CT_Extension

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; processContents=lax.
- [ ] `@uri` — `xsd:token`; MS-ODRAWXML.

<a id="c-ct_trendlinetype"></a>

## c:CT_TrendlineType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_TrendlineType](values-charts.md#c-st_trendlinetype); Strict, Transitional; default=linear.

<a id="c-ct_order"></a>

## c:CT_Order

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Order](values-charts.md#c-st_order); Strict, Transitional; default=2.

<a id="c-ct_period"></a>

## c:CT_Period

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_Period](values-charts.md#c-st_period); Strict, Transitional; default=2.

<a id="c-ct_trendlinelbl"></a>

## c:CT_TrendlineLbl

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `layout` — [c:CT_Layout](chart-extensions.md#c-ct_layout); Strict, Transitional; minOccurs=0; maxOccurs=1. Layout; [§21.2.2.88](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3407-3412-0633.md#source-page-3410-part-1).
- [ ] `tx` — [c:CT_Tx](chart-extensions.md#c-ct_tx); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `numFmt` — [c:CT_NumFmt](chart-extensions.md#c-ct_numfmt); Strict, Transitional; minOccurs=0; maxOccurs=1. Number Format; [§21.2.2.121](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3413-3418-0634.md#source-page-3418-part-1).
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=0; maxOccurs=1. Shape Properties; [§21.2.2.197](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3443-part-1).
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); Strict, Transitional; minOccurs=0; maxOccurs=1. Text Properties; [§21.2.2.216](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3443-3448-0639.md#source-page-3447-part-1).
- [ ] `extLst` — [c:CT_ExtensionList](charts.md#c-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1. Chart Extensibility; [§21.2.2.64](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3401-3406-0632.md#source-page-3401-part-1).

<a id="c-ct_errdir"></a>

## c:CT_ErrDir

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_ErrDir](values-charts.md#c-st_errdir); Strict, Transitional; use=required.

<a id="c-ct_errbartype"></a>

## c:CT_ErrBarType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_ErrBarType](values-charts.md#c-st_errbartype); Strict, Transitional; default=both.

<a id="c-ct_errvaltype"></a>

## c:CT_ErrValType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-chart.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-chart.xsd).

- [ ] `@val` — [c:ST_ErrValType](values-charts.md#c-st_errvaltype); Strict, Transitional; default=fixedVal.

<a id="cs-ct_stylecolor"></a>

## cs:CT_StyleColor

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `@val` — [cs:ST_StyleColorVal](values-charts.md#cs-st_stylecolorval); MS-ODRAWXML.

<a id="cx-ct_numericdimension"></a>

## cx:CT_NumericDimension

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `f` — [cx:CT_Formula](chart-extensions.md#cx-ct_formula); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `nf` — [cx:CT_Formula](chart-extensions.md#cx-ct_formula); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `lvl` — [cx:CT_NumericLevel](chart-extensions.md#cx-ct_numericlevel); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `lvl` — [cx:CT_NumericLevel](chart-extensions.md#cx-ct_numericlevel); MS-ODRAWXML; minOccurs=1; maxOccurs=unbounded.
- [ ] `@type` — [cx:ST_NumericDimensionType](values-charts.md#cx-st_numericdimensiontype); MS-ODRAWXML; use=required.

<a id="cx-ct_stringdimension"></a>

## cx:CT_StringDimension

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `f` — [cx:CT_Formula](chart-extensions.md#cx-ct_formula); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `nf` — [cx:CT_Formula](chart-extensions.md#cx-ct_formula); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `lvl` — [cx:CT_StringLevel](chart-extensions.md#cx-ct_stringlevel); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `lvl` — [cx:CT_StringLevel](chart-extensions.md#cx-ct_stringlevel); MS-ODRAWXML; minOccurs=1; maxOccurs=unbounded.
- [ ] `@type` — [cx:ST_StringDimensionType](values-charts.md#cx-st_stringdimensiontype); MS-ODRAWXML; use=required.

<a id="cx-ct_text"></a>

## cx:CT_Text

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `txData` — [cx:CT_TextData](chart-extensions.md#cx-ct_textdata); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `rich` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="cx-ct_offset"></a>

## cx:CT_Offset

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@top` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@left` — `xsd:double`; MS-ODRAWXML; use=required.

<a id="cx-ct_plotarearegion"></a>

## cx:CT_PlotAreaRegion

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `plotSurface` — [cx:CT_PlotSurface](chart-extensions.md#cx-ct_plotsurface); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `series` — [cx:CT_Series](chart-extensions.md#cx-ct_series); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_axis"></a>

## cx:CT_Axis

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `catScaling` — [cx:CT_CategoryAxisScaling](chart-extensions.md#cx-ct_categoryaxisscaling); MS-ODRAWXML.
- [ ] `valScaling` — [cx:CT_ValueAxisScaling](chart-extensions.md#cx-ct_valueaxisscaling); MS-ODRAWXML.
- [ ] `title` — [cx:CT_AxisTitle](chart-extensions.md#cx-ct_axistitle); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `units` — [cx:CT_AxisUnits](chart-extensions.md#cx-ct_axisunits); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `majorGridlines` — [cx:CT_Gridlines](chart-extensions.md#cx-ct_gridlines); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `minorGridlines` — [cx:CT_Gridlines](chart-extensions.md#cx-ct_gridlines); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `majorTickMarks` — [cx:CT_TickMarks](chart-extensions.md#cx-ct_tickmarks); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `minorTickMarks` — [cx:CT_TickMarks](chart-extensions.md#cx-ct_tickmarks); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `tickLabels` — [cx:CT_TickLabels](chart-extensions.md#cx-ct_ticklabels); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `numFmt` — [cx:CT_NumberFormat](chart-extensions.md#cx-ct_numberformat); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [cx:ST_AxisId](values-charts.md#cx-st_axisid); MS-ODRAWXML; use=required.
- [ ] `@hidden` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=0.

<a id="cx-ct_binding"></a>

## cx:CT_Binding

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@name` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@id` — [cx:ST_DataId](values-charts.md#cx-st_dataid); MS-ODRAWXML; use=required.
- [ ] `@type` — [cx:ST_BindingType](values-charts.md#cx-st_bindingtype); MS-ODRAWXML; use=required.

<a id="cx-ct_artifact"></a>

## cx:CT_Artifact

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@source` — `xsd:string`; MS-ODRAWXML; use=optional; default=.
- [ ] `@name` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@type` — [cx:ST_ArtifactType](values-charts.md#cx-st_artifacttype); MS-ODRAWXML; use=required.

<a id="cx-ct_formula"></a>

## cx:CT_Formula

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@dir` — [cx:ST_FormulaDirection](values-charts.md#cx-st_formuladirection); MS-ODRAWXML; use=optional; default=col.

<a id="cx-ct_numericlevel"></a>

## cx:CT_NumericLevel

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `pt` — [cx:CT_NumericValue](chart-extensions.md#cx-ct_numericvalue); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `@ptCount` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.
- [ ] `@formatCode` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@name` — `xsd:string`; MS-ODRAWXML; use=optional.

<a id="cx-ct_stringlevel"></a>

## cx:CT_StringLevel

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `pt` — [cx:CT_StringValue](chart-extensions.md#cx-ct_stringvalue); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `@ptCount` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.
- [ ] `@name` — `xsd:string`; MS-ODRAWXML; use=optional.

<a id="cx-ct_textdata"></a>

## cx:CT_TextData

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `f` — [cx:CT_Formula](chart-extensions.md#cx-ct_formula); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `v` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `v` — `xsd:string`; MS-ODRAWXML; minOccurs=1; maxOccurs=1.

<a id="cx-ct_plotsurface"></a>

## cx:CT_PlotSurface

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_series"></a>

## cx:CT_Series

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tx` — [cx:CT_Text](chart-extensions.md#cx-ct_text); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `valueColors` — [cx:CT_ValueColors](chart-extensions.md#cx-ct_valuecolors); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `valueColorPositions` — [cx:CT_ValueColorPositions](chart-extensions.md#cx-ct_valuecolorpositions); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dataPt` — [cx:CT_DataPoint](chart-extensions.md#cx-ct_datapoint); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `dataLabels` — [cx:CT_DataLabels](chart-extensions.md#cx-ct_datalabels); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dataId` — [cx:CT_DataId](chart-extensions.md#cx-ct_dataid); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `layoutPr` — [cx:CT_SeriesLayoutProperties](chart-extensions.md#cx-ct_serieslayoutproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `axisId` — [cx:ST_AxisId](values-charts.md#cx-st_axisid); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@layoutId` — [cx:ST_SeriesLayout](values-charts.md#cx-st_serieslayout); MS-ODRAWXML; use=required.
- [ ] `@hidden` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=0.
- [ ] `@ownerIdx` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional.
- [ ] `@uniqueId` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@formatIdx` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional.

<a id="cx-ct_categoryaxisscaling"></a>

## cx:CT_CategoryAxisScaling

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@gapWidth` — [cx:ST_GapWidthRatio](values-charts.md#cx-st_gapwidthratio); MS-ODRAWXML; use=optional.

<a id="cx-ct_valueaxisscaling"></a>

## cx:CT_ValueAxisScaling

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@max` — [cx:ST_DoubleOrAutomatic](values-charts.md#cx-st_doubleorautomatic); MS-ODRAWXML; use=optional.
- [ ] `@min` — [cx:ST_DoubleOrAutomatic](values-charts.md#cx-st_doubleorautomatic); MS-ODRAWXML; use=optional.
- [ ] `@majorUnit` — [cx:ST_ValueAxisUnit](values-charts.md#cx-st_valueaxisunit); MS-ODRAWXML; use=optional.
- [ ] `@minorUnit` — [cx:ST_ValueAxisUnit](values-charts.md#cx-st_valueaxisunit); MS-ODRAWXML; use=optional.

<a id="cx-ct_axistitle"></a>

## cx:CT_AxisTitle

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tx` — [cx:CT_Text](chart-extensions.md#cx-ct_text); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `offset` — [cx:CT_Offset](chart-extensions.md#cx-ct_offset); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_axisunits"></a>

## cx:CT_AxisUnits

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `unitsLabel` — [cx:CT_AxisUnitsLabel](chart-extensions.md#cx-ct_axisunitslabel); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@unit` — [cx:ST_AxisUnit](values-charts.md#cx-st_axisunit); MS-ODRAWXML; use=optional.

<a id="cx-ct_gridlines"></a>

## cx:CT_Gridlines

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_tickmarks"></a>

## cx:CT_TickMarks

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@type` — [cx:ST_TickMarksType](values-charts.md#cx-st_tickmarkstype); MS-ODRAWXML; use=optional.

<a id="cx-ct_ticklabels"></a>

## cx:CT_TickLabels

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_numberformat"></a>

## cx:CT_NumberFormat

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@formatCode` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@sourceLinked` — `xsd:boolean`; MS-ODRAWXML; use=optional.

<a id="cx-ct_numericvalue"></a>

## cx:CT_NumericValue

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@idx` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.

<a id="cx-ct_stringvalue"></a>

## cx:CT_StringValue

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@idx` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.

<a id="cx-ct_valuecolors"></a>

## cx:CT_ValueColors

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `minColor` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `midColor` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `maxColor` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_valuecolorpositions"></a>

## cx:CT_ValueColorPositions

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `min` — [cx:CT_ValueColorEndPosition](chart-extensions.md#cx-ct_valuecolorendposition); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `mid` — [cx:CT_ValueColorMiddlePosition](chart-extensions.md#cx-ct_valuecolormiddleposition); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `max` — [cx:CT_ValueColorEndPosition](chart-extensions.md#cx-ct_valuecolorendposition); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@count` — [cx:ST_ValueColorPositionCount](values-charts.md#cx-st_valuecolorpositioncount); MS-ODRAWXML; use=optional; default=2.

<a id="cx-ct_datapoint"></a>

## cx:CT_DataPoint

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@idx` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.

<a id="cx-ct_datalabels"></a>

## cx:CT_DataLabels

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `numFmt` — [cx:CT_NumberFormat](chart-extensions.md#cx-ct_numberformat); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `visibility` — [cx:CT_DataLabelVisibilities](chart-extensions.md#cx-ct_datalabelvisibilities); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `separator` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `dataLabel` — [cx:CT_DataLabel](chart-extensions.md#cx-ct_datalabel); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `dataLabelHidden` — [cx:CT_DataLabelHidden](chart-extensions.md#cx-ct_datalabelhidden); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@pos` — [cx:ST_DataLabelPos](values-charts.md#cx-st_datalabelpos); MS-ODRAWXML; use=optional.

<a id="cx-ct_dataid"></a>

## cx:CT_DataId

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — [cx:ST_DataId](values-charts.md#cx-st_dataid); MS-ODRAWXML; use=required.

<a id="cx-ct_serieslayoutproperties"></a>

## cx:CT_SeriesLayoutProperties

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `parentLabelLayout` — [cx:CT_ParentLabelLayout](chart-extensions.md#cx-ct_parentlabellayout); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `regionLabelLayout` — [cx:CT_RegionLabelLayout](chart-extensions.md#cx-ct_regionlabellayout); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `visibility` — [cx:CT_SeriesElementVisibilities](chart-extensions.md#cx-ct_serieselementvisibilities); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `aggregation` — [cx:CT_Aggregation](chart-extensions.md#cx-ct_aggregation); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `binning` — [cx:CT_Binning](chart-extensions.md#cx-ct_binning); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geography` — [cx:CT_Geography](chart-extensions.md#cx-ct_geography); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `statistics` — [cx:CT_Statistics](chart-extensions.md#cx-ct_statistics); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `subtotals` — [cx:CT_Subtotals](chart-extensions.md#cx-ct_subtotals); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_axisunitslabel"></a>

## cx:CT_AxisUnitsLabel

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `tx` — [cx:CT_Text](chart-extensions.md#cx-ct_text); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_valuecolorendposition"></a>

## cx:CT_ValueColorEndPosition

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `extremeValue` — [cx:CT_ExtremeValueColorPosition](chart-extensions.md#cx-ct_extremevaluecolorposition); MS-ODRAWXML.
- [ ] `number` — [cx:CT_NumberColorPosition](chart-extensions.md#cx-ct_numbercolorposition); MS-ODRAWXML.
- [ ] `percent` — [cx:CT_PercentageColorPosition](chart-extensions.md#cx-ct_percentagecolorposition); MS-ODRAWXML.

<a id="cx-ct_valuecolormiddleposition"></a>

## cx:CT_ValueColorMiddlePosition

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `number` — [cx:CT_NumberColorPosition](chart-extensions.md#cx-ct_numbercolorposition); MS-ODRAWXML.
- [ ] `percent` — [cx:CT_PercentageColorPosition](chart-extensions.md#cx-ct_percentagecolorposition); MS-ODRAWXML.

<a id="cx-ct_datalabelvisibilities"></a>

## cx:CT_DataLabelVisibilities

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@seriesName` — `xsd:boolean`; MS-ODRAWXML; use=optional.
- [ ] `@categoryName` — `xsd:boolean`; MS-ODRAWXML; use=optional.
- [ ] `@value` — `xsd:boolean`; MS-ODRAWXML; use=optional.

<a id="cx-ct_datalabel"></a>

## cx:CT_DataLabel

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `numFmt` — [cx:CT_NumberFormat](chart-extensions.md#cx-ct_numberformat); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `txPr` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `visibility` — [cx:CT_DataLabelVisibilities](chart-extensions.md#cx-ct_datalabelvisibilities); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `separator` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [cx:CT_ExtensionList](chart-extensions.md#cx-ct_extensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@idx` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.
- [ ] `@pos` — [cx:ST_DataLabelPos](values-charts.md#cx-st_datalabelpos); MS-ODRAWXML; use=optional.

<a id="cx-ct_datalabelhidden"></a>

## cx:CT_DataLabelHidden

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@idx` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.

<a id="cx-ct_parentlabellayout"></a>

## cx:CT_ParentLabelLayout

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — [cx:ST_ParentLabelLayout](values-charts.md#cx-st_parentlabellayout); MS-ODRAWXML; use=required.

<a id="cx-ct_regionlabellayout"></a>

## cx:CT_RegionLabelLayout

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — [cx:ST_RegionLabelLayout](values-charts.md#cx-st_regionlabellayout); MS-ODRAWXML; use=required.

<a id="cx-ct_serieselementvisibilities"></a>

## cx:CT_SeriesElementVisibilities

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@connectorLines` — `xsd:boolean`; MS-ODRAWXML; use=optional.
- [ ] `@meanLine` — `xsd:boolean`; MS-ODRAWXML; use=optional.
- [ ] `@meanMarker` — `xsd:boolean`; MS-ODRAWXML; use=optional.
- [ ] `@nonoutliers` — `xsd:boolean`; MS-ODRAWXML; use=optional.
- [ ] `@outliers` — `xsd:boolean`; MS-ODRAWXML; use=optional.

<a id="cx-ct_aggregation"></a>

## cx:CT_Aggregation

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

Empty marker type; no attributes or child properties.

<a id="cx-ct_binning"></a>

## cx:CT_Binning

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `binSize` — `xsd:double`; MS-ODRAWXML.
- [ ] `binCount` — `xsd:unsignedInt`; MS-ODRAWXML.
- [ ] `@intervalClosed` — [cx:ST_IntervalClosedSide](values-charts.md#cx-st_intervalclosedside); MS-ODRAWXML; use=optional.
- [ ] `@underflow` — [cx:ST_DoubleOrAutomatic](values-charts.md#cx-st_doubleorautomatic); MS-ODRAWXML; use=optional.
- [ ] `@overflow` — [cx:ST_DoubleOrAutomatic](values-charts.md#cx-st_doubleorautomatic); MS-ODRAWXML; use=optional.

<a id="cx-ct_geography"></a>

## cx:CT_Geography

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoCache` — [cx:CT_GeoCache](chart-extensions.md#cx-ct_geocache); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@projectionType` — [cx:ST_GeoProjectionType](values-charts.md#cx-st_geoprojectiontype); MS-ODRAWXML; use=optional.
- [ ] `@viewedRegionType` — [cx:ST_GeoMappingLevel](values-charts.md#cx-st_geomappinglevel); MS-ODRAWXML; use=optional.
- [ ] `@cultureLanguage` — `xsd:language`; MS-ODRAWXML; use=required.
- [ ] `@cultureRegion` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@attribution` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_statistics"></a>

## cx:CT_Statistics

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@quartileMethod` — [cx:ST_QuartileMethod](values-charts.md#cx-st_quartilemethod); MS-ODRAWXML; use=optional.

<a id="cx-ct_subtotals"></a>

## cx:CT_Subtotals

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `idx` — [cx:CT_SubtotalIndex](chart-extensions.md#cx-ct_subtotalindex); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_extremevaluecolorposition"></a>

## cx:CT_ExtremeValueColorPosition

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

Empty marker type; no attributes or child properties.

<a id="cx-ct_numbercolorposition"></a>

## cx:CT_NumberColorPosition

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:double`; MS-ODRAWXML; use=required.

<a id="cx-ct_percentagecolorposition"></a>

## cx:CT_PercentageColorPosition

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:double`; MS-ODRAWXML; use=required.

<a id="cx-ct_geocache"></a>

## cx:CT_GeoCache

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `binary` — `xsd:base64Binary`; MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `clear` — [cx:CT_Clear](chart-extensions.md#cx-ct_clear); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@provider` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_subtotalindex"></a>

## cx:CT_SubtotalIndex

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@val` — `xsd:unsignedInt`; MS-ODRAWXML; use=required.

<a id="cx-ct_clear"></a>

## cx:CT_Clear

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoLocationQueryResults` — [cx:CT_GeoLocationQueryResults](chart-extensions.md#cx-ct_geolocationqueryresults); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoDataEntityQueryResults` — [cx:CT_GeoDataEntityQueryResults](chart-extensions.md#cx-ct_geodataentityqueryresults); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoDataPointToEntityQueryResults` — [cx:CT_GeoDataPointToEntityQueryResults](chart-extensions.md#cx-ct_geodatapointtoentityqueryresults); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoChildEntitiesQueryResults` — [cx:CT_GeoChildEntitiesQueryResults](chart-extensions.md#cx-ct_geochildentitiesqueryresults); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoParentEntitiesQueryResults` — [cx:CT_GeoParentEntitiesQueryResults](chart-extensions.md#cx-ct_geoparententitiesqueryresults); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_geolocationqueryresults"></a>

## cx:CT_GeoLocationQueryResults

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoLocationQueryResult` — [cx:CT_GeoLocationQueryResult](chart-extensions.md#cx-ct_geolocationqueryresult); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geodataentityqueryresults"></a>

## cx:CT_GeoDataEntityQueryResults

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoDataEntityQueryResult` — [cx:CT_GeoDataEntityQueryResult](chart-extensions.md#cx-ct_geodataentityqueryresult); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geodatapointtoentityqueryresults"></a>

## cx:CT_GeoDataPointToEntityQueryResults

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoDataPointToEntityQueryResult` — [cx:CT_GeoDataPointToEntityQueryResult](chart-extensions.md#cx-ct_geodatapointtoentityqueryresult); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geochildentitiesqueryresults"></a>

## cx:CT_GeoChildEntitiesQueryResults

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoChildEntitiesQueryResult` — [cx:CT_GeoChildEntitiesQueryResult](chart-extensions.md#cx-ct_geochildentitiesqueryresult); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geoparententitiesqueryresults"></a>

## cx:CT_GeoParentEntitiesQueryResults

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoParentEntitiesQueryResult` — [cx:CT_GeoParentEntitiesQueryResult](chart-extensions.md#cx-ct_geoparententitiesqueryresult); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geolocationqueryresult"></a>

## cx:CT_GeoLocationQueryResult

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoLocationQuery` — [cx:CT_GeoLocationQuery](chart-extensions.md#cx-ct_geolocationquery); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoLocations` — [cx:CT_GeoLocations](chart-extensions.md#cx-ct_geolocations); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_geodataentityqueryresult"></a>

## cx:CT_GeoDataEntityQueryResult

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoDataEntityQuery` — [cx:CT_GeoDataEntityQuery](chart-extensions.md#cx-ct_geodataentityquery); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoData` — [cx:CT_GeoData](chart-extensions.md#cx-ct_geodata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_geodatapointtoentityqueryresult"></a>

## cx:CT_GeoDataPointToEntityQueryResult

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoDataPointQuery` — [cx:CT_GeoDataPointQuery](chart-extensions.md#cx-ct_geodatapointquery); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoDataPointToEntityQuery` — [cx:CT_GeoDataPointToEntityQuery](chart-extensions.md#cx-ct_geodatapointtoentityquery); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_geochildentitiesqueryresult"></a>

## cx:CT_GeoChildEntitiesQueryResult

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoChildEntitiesQuery` — [cx:CT_GeoChildEntitiesQuery](chart-extensions.md#cx-ct_geochildentitiesquery); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoChildEntities` — [cx:CT_GeoChildEntities](chart-extensions.md#cx-ct_geochildentities); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_geoparententitiesqueryresult"></a>

## cx:CT_GeoParentEntitiesQueryResult

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoParentEntitiesQuery` — [cx:CT_GeoParentEntitiesQuery](chart-extensions.md#cx-ct_geoparententitiesquery); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `geoEntity` — [cx:CT_GeoEntity](chart-extensions.md#cx-ct_geoentity); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `geoParentEntity` — [cx:CT_GeoParentEntity](chart-extensions.md#cx-ct_geoparententity); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_geolocationquery"></a>

## cx:CT_GeoLocationQuery

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@countryRegion` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@adminDistrict1` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@adminDistrict2` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@postalCode` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; use=required.

<a id="cx-ct_geolocations"></a>

## cx:CT_GeoLocations

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoLocation` — [cx:CT_GeoLocation](chart-extensions.md#cx-ct_geolocation); MS-ODRAWXML; minOccurs=0; maxOccurs=1.

<a id="cx-ct_geodataentityquery"></a>

## cx:CT_GeoDataEntityQuery

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; use=required.
- [ ] `@entityId` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_geodata"></a>

## cx:CT_GeoData

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoPolygons` — [cx:CT_GeoPolygons](chart-extensions.md#cx-ct_geopolygons); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `copyrights` — [cx:CT_Copyrights](chart-extensions.md#cx-ct_copyrights); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@entityName` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@entityId` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@east` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@west` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@north` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@south` — `xsd:double`; MS-ODRAWXML; use=required.

<a id="cx-ct_geodatapointquery"></a>

## cx:CT_GeoDataPointQuery

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; use=required.
- [ ] `@latitude` — `xsd:double`; MS-ODRAWXML; use=required.
- [ ] `@longitude` — `xsd:double`; MS-ODRAWXML; use=required.

<a id="cx-ct_geodatapointtoentityquery"></a>

## cx:CT_GeoDataPointToEntityQuery

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; use=required.
- [ ] `@entityId` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_geochildentitiesquery"></a>

## cx:CT_GeoChildEntitiesQuery

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoChildTypes` — [cx:CT_GeoChildTypes](chart-extensions.md#cx-ct_geochildtypes); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@entityId` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_geochildentities"></a>

## cx:CT_GeoChildEntities

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoHierarchyEntity` — [cx:CT_GeoHierarchyEntity](chart-extensions.md#cx-ct_geohierarchyentity); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geoparententitiesquery"></a>

## cx:CT_GeoParentEntitiesQuery

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@entityId` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_geoentity"></a>

## cx:CT_GeoEntity

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@entityName` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; use=required.

<a id="cx-ct_geoparententity"></a>

## cx:CT_GeoParentEntity

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@entityId` — `xsd:string`; MS-ODRAWXML; use=required.

<a id="cx-ct_geolocation"></a>

## cx:CT_GeoLocation

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `address` — [cx:CT_Address](chart-extensions.md#cx-ct_address); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@latitude` — `xsd:double`; MS-ODRAWXML; use=optional.
- [ ] `@longitude` — `xsd:double`; MS-ODRAWXML; use=optional.
- [ ] `@entityName` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; use=required.

<a id="cx-ct_geopolygons"></a>

## cx:CT_GeoPolygons

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `geoPolygon` — [cx:CT_GeoPolygon](chart-extensions.md#cx-ct_geopolygon); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_copyrights"></a>

## cx:CT_Copyrights

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `copyright` — `xsd:string`; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geochildtypes"></a>

## cx:CT_GeoChildTypes

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="cx-ct_geohierarchyentity"></a>

## cx:CT_GeoHierarchyEntity

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@entityName` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@entityId` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@entityType` — [cx:ST_EntityType](values-charts.md#cx-st_entitytype); MS-ODRAWXML; use=required.

<a id="cx-ct_address"></a>

## cx:CT_Address

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@address1` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@countryRegion` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@adminDistrict1` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@adminDistrict2` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@postalCode` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@locality` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@isoCountryCode` — `xsd:string`; MS-ODRAWXML; use=optional.

<a id="cx-ct_geopolygon"></a>

## cx:CT_GeoPolygon

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `@polygonId` — `xsd:string`; MS-ODRAWXML; use=required.
- [ ] `@numPoints` — `xsd:integer`; MS-ODRAWXML; use=required.
- [ ] `@pcaRings` — `xsd:string`; MS-ODRAWXML; use=required.
