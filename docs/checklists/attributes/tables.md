# Table, row and cell properties

[Inventory guide](README.md)

Word and DrawingML table geometry, row flow, cell content and borders, including unsupported standard fields.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_TblPr](#w-ct_tblpr)
- [w:CT_TrPr](#w-ct_trpr)
- [w:CT_TcPr](#w-ct_tcpr)
- [w:CT_TblPrEx](#w-ct_tblprex)
- [w:CT_TblGrid](#w-ct_tblgrid)
- [a:CT_TableProperties](#a-ct_tableproperties)
- [a:CT_TableCellProperties](#a-ct_tablecellproperties)
- [a:CT_TableGrid](#a-ct_tablegrid)
- [w:CT_Tbl](#w-ct_tbl)
- [w:CT_Row](#w-ct_row)
- [w:CT_Tc](#w-ct_tc)
- [a:CT_TableRow](#a-ct_tablerow)
- [a:CT_TableCell](#a-ct_tablecell)
- [w:CT_TblPPr](#w-ct_tblppr)
- [w:CT_TblOverlap](#w-ct_tbloverlap)
- [w:CT_TblWidth](#w-ct_tblwidth)
- [w:CT_JcTable](#w-ct_jctable)
- [w:CT_TblBorders](#w-ct_tblborders)
- [w:CT_TblLayoutType](#w-ct_tbllayouttype)
- [w:CT_TblCellMar](#w-ct_tblcellmar)
- [w:CT_TblLook](#w-ct_tbllook)
- [w:CT_TblPrChange](#w-ct_tblprchange)
- [w:CT_Height](#w-ct_height)
- [w:CT_TrPrChange](#w-ct_trprchange)
- [w:CT_HMerge](#w-ct_hmerge)
- [w:CT_VMerge](#w-ct_vmerge)
- [w:CT_TcBorders](#w-ct_tcborders)
- [w:CT_TcMar](#w-ct_tcmar)
- [w:CT_VerticalJc](#w-ct_verticaljc)
- [w:CT_Headers](#w-ct_headers)
- [w:CT_CellMergeTrackChange](#w-ct_cellmergetrackchange)
- [w:CT_TcPrChange](#w-ct_tcprchange)
- [w:CT_TblPrExChange](#w-ct_tblprexchange)
- [w:CT_TblGridCol](#w-ct_tblgridcol)
- [w:CT_TblGridChange](#w-ct_tblgridchange)
- [a:CT_NoFillProperties](#a-ct_nofillproperties)
- [a:CT_SolidColorFillProperties](#a-ct_solidcolorfillproperties)
- [a:CT_GradientFillProperties](#a-ct_gradientfillproperties)
- [a:CT_PatternFillProperties](#a-ct_patternfillproperties)
- [a:CT_GroupFillProperties](#a-ct_groupfillproperties)
- [a:CT_EffectList](#a-ct_effectlist)
- [a:CT_EffectContainer](#a-ct_effectcontainer)
- [a:CT_TableStyle](#a-ct_tablestyle)
- [a:CT_LineProperties](#a-ct_lineproperties)
- [a:CT_Cell3D](#a-ct_cell3d)
- [a:CT_Headers](#a-ct_headers)
- [a:CT_TableCol](#a-ct_tablecol)
- [w:CT_TrPrBase](#w-ct_trprbase)
- [w:CT_TcPrInner](#w-ct_tcprinner)
- [w:CT_TblPrExBase](#w-ct_tblprexbase)
- [w:CT_TblGridBase](#w-ct_tblgridbase)
- [a:CT_ScRgbColor](#a-ct_scrgbcolor)
- [a:CT_SRgbColor](#a-ct_srgbcolor)
- [a:CT_HslColor](#a-ct_hslcolor)
- [a:CT_SystemColor](#a-ct_systemcolor)
- [a:CT_SchemeColor](#a-ct_schemecolor)
- [a:CT_PresetColor](#a-ct_presetcolor)
- [a:CT_GradientStopList](#a-ct_gradientstoplist)
- [a:CT_LinearShadeProperties](#a-ct_linearshadeproperties)
- [a:CT_PathShadeProperties](#a-ct_pathshadeproperties)
- [a:CT_Color](#a-ct_color)
- [a:CT_BlurEffect](#a-ct_blureffect)
- [a:CT_FillOverlayEffect](#a-ct_filloverlayeffect)
- [a:CT_GlowEffect](#a-ct_gloweffect)
- [a:CT_InnerShadowEffect](#a-ct_innershadoweffect)
- [a:CT_OuterShadowEffect](#a-ct_outershadoweffect)
- [a:CT_PresetShadowEffect](#a-ct_presetshadoweffect)
- [a:CT_ReflectionEffect](#a-ct_reflectioneffect)
- [a:CT_SoftEdgesEffect](#a-ct_softedgeseffect)
- [a:CT_EffectReference](#a-ct_effectreference)
- [a:CT_AlphaBiLevelEffect](#a-ct_alphabileveleffect)
- [a:CT_AlphaCeilingEffect](#a-ct_alphaceilingeffect)
- [a:CT_AlphaFloorEffect](#a-ct_alphaflooreffect)
- [a:CT_AlphaInverseEffect](#a-ct_alphainverseeffect)
- [a:CT_AlphaModulateEffect](#a-ct_alphamodulateeffect)
- [a:CT_AlphaModulateFixedEffect](#a-ct_alphamodulatefixedeffect)
- [a:CT_AlphaOutsetEffect](#a-ct_alphaoutseteffect)
- [a:CT_AlphaReplaceEffect](#a-ct_alphareplaceeffect)
- [a:CT_BiLevelEffect](#a-ct_bileveleffect)
- [a:CT_BlendEffect](#a-ct_blendeffect)
- [a:CT_ColorChangeEffect](#a-ct_colorchangeeffect)
- [a:CT_ColorReplaceEffect](#a-ct_colorreplaceeffect)
- [a:CT_DuotoneEffect](#a-ct_duotoneeffect)
- [a:CT_FillEffect](#a-ct_filleffect)
- [a:CT_GrayscaleEffect](#a-ct_grayscaleeffect)
- [a:CT_HSLEffect](#a-ct_hsleffect)
- [a:CT_LuminanceEffect](#a-ct_luminanceeffect)
- [a:CT_RelativeOffsetEffect](#a-ct_relativeoffseteffect)
- [a:CT_TintEffect](#a-ct_tinteffect)
- [a:CT_TransformEffect](#a-ct_transformeffect)
- [a:CT_TableBackgroundStyle](#a-ct_tablebackgroundstyle)
- [a:CT_TablePartStyle](#a-ct_tablepartstyle)
- [a:CT_PresetLineDashProperties](#a-ct_presetlinedashproperties)
- [a:CT_DashStopList](#a-ct_dashstoplist)
- [a:CT_LineJoinRound](#a-ct_linejoinround)
- [a:CT_LineJoinBevel](#a-ct_linejoinbevel)
- [a:CT_LineJoinMiterProperties](#a-ct_linejoinmiterproperties)
- [a:CT_LineEndProperties](#a-ct_lineendproperties)
- [a:CT_Bevel](#a-ct_bevel)
- [a:CT_LightRig](#a-ct_lightrig)
- [a:CT_PositiveFixedPercentage](#a-ct_positivefixedpercentage)
- [a:CT_ComplementTransform](#a-ct_complementtransform)
- [a:CT_InverseTransform](#a-ct_inversetransform)
- [a:CT_GrayscaleTransform](#a-ct_grayscaletransform)
- [a:CT_FixedPercentage](#a-ct_fixedpercentage)
- [a:CT_PositivePercentage](#a-ct_positivepercentage)
- [a:CT_PositiveFixedAngle](#a-ct_positivefixedangle)
- [a:CT_Angle](#a-ct_angle)
- [a:CT_Percentage](#a-ct_percentage)
- [a:CT_GammaTransform](#a-ct_gammatransform)
- [a:CT_InverseGammaTransform](#a-ct_inversegammatransform)
- [a:CT_GradientStop](#a-ct_gradientstop)
- [a:CT_FillProperties](#a-ct_fillproperties)
- [a:CT_EffectProperties](#a-ct_effectproperties)
- [a:CT_TableStyleTextStyle](#a-ct_tablestyletextstyle)
- [a:CT_TableStyleCellStyle](#a-ct_tablestylecellstyle)
- [a:CT_DashStop](#a-ct_dashstop)
- [a:CT_SphereCoords](#a-ct_spherecoords)
- [a:CT_TableCellBorderStyle](#a-ct_tablecellborderstyle)
- [a:CT_ThemeableLineStyle](#a-ct_themeablelinestyle)

<a id="w-ct_tblpr"></a>

## w:CT_TblPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `tblStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Referenced Table Style; [§17.4.62](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0459-0465-0089.md#source-page-464-part-1). Named table styles and conditional regions are emitted in editable DOCX. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblpPr` — [w:CT_TblPPr](tables.md#w-ct_tblppr); Strict, Transitional; minOccurs=0; maxOccurs=1. Floating Table Positioning; [§17.4.57](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0448-0453-0087.md#source-page-452-part-1).
- [ ] `tblOverlap` — [w:CT_TblOverlap](tables.md#w-ct_tbloverlap); Strict, Transitional; minOccurs=0; maxOccurs=1. Floating Table Allows Other Tables to Overlap; [§17.4.56](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0448-0453-0087.md#source-page-450-part-1).
- [ ] `bidiVisual` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Visually Right to Left Table; [§17.4.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0378-0383-0076.md#source-page-383-part-1).
- [ ] `tblStyleRowBandSize` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0; maxOccurs=1. Number of Rows in Row Band; [§17.7.6.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-670-part-1).
- [ ] `tblStyleColBandSize` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0; maxOccurs=1. Number of Columns in Column Band; [§17.7.6.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0666-0672-0126.md#source-page-667-part-1).
- [ ] `tblW` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical table width is emitted; percentage width is not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `jc` — [w:CT_JcTable](tables.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1. Four Mote alignment values map to left/center/right placement; justify maps to left. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCellSpacing` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical cell spacing is emitted; other width types are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblInd` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical indentation is emitted; other width types are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblBorders` — [w:CT_TblBorders](tables.md#w-ct_tblborders); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical outside/inside edges, widths and colors are emitted; themed border controls are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [x] `tblLayout` — [w:CT_TblLayoutType](tables.md#w-ct_tbllayouttype); Strict, Transitional; minOccurs=0; maxOccurs=1. Table layout accepts auto/autofit/fixed; fixed emits a fixed DOCX grid. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCellMar` — [w:CT_TblCellMar](tables.md#w-ct_tblcellmar); Strict, Transitional; minOccurs=0; maxOccurs=1. Physical cell margins and shorthands are emitted; logical edges are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblLook` — [w:CT_TblLook](tables.md#w-ct_tbllook); Strict, Transitional; minOccurs=0; maxOccurs=1. Declared conditional regions select table look flags; independent look-bit authorship is not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCaption` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Caption; [§17.4.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0428-0434-0084.md#source-page-434-part-1).
- [ ] `tblDescription` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Description; [§17.4.46](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0435-0440-0085.md#source-page-440-part-1).
- [ ] `tblPrChange` — [w:CT_TblPrChange](tables.md#w-ct_tblprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_trpr"></a>

## w:CT_TrPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `divId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `gridBefore` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Before First Cell; [§17.4.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-406-part-1).
- [ ] `gridAfter` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns After Last Cell; [§17.4.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-405-part-1).
- [ ] `wBefore` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width Before Table Row; [§17.4.86](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-492-part-1).
- [ ] `wAfter` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width After Table Row; [§17.4.85](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-491-part-1).
- [x] `cantSplit` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Table Row Cannot Break Across Pages; [§17.4.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0384-0390-0077.md#source-page-387-part-1). Row cant_split emits the row-splitting flag. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `trHeight` — [w:CT_Height](tables.md#w-ct_height); Strict, Transitional; minOccurs=0. Table Row Height; [§17.4.80](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0480-0485-0092.md#source-page-484-part-1). Row height and auto/at_least/exact rules are emitted. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `tblHeader` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Repeat Table Row on Every New Page; [§17.4.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0441-0447-0086.md#source-page-443-part-1). Row repeat_header emits the repeating-header flag. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCellSpacing` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `jc` — [w:CT_JcTable](tables.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `hidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `ins` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `del` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `trPrChange` — [w:CT_TrPrChange](tables.md#w-ct_trprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_tcpr"></a>

## w:CT_TcPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcW` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Table Cell Width; [§17.4.71](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0473-0479-0091.md#source-page-475-part-1). Physical cell width is emitted; percentage widths are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `gridSpan` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Spanned by Current Table Cell; [§17.4.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-409-part-1). Positive colspan creates a logical span and editable DOCX gridSpan. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `hMerge` — [w:CT_HMerge](tables.md#w-ct_hmerge); Transitional; minOccurs=0.
- [x] `vMerge` — [w:CT_VMerge](tables.md#w-ct_vmerge); Strict, Transitional; minOccurs=0. Vertically Merged Cell; [§17.4.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-489-part-1). Positive rowspan creates a logical vertical span with restart/continuation DOCX cells. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tcBorders` — [w:CT_TcBorders](tables.md#w-ct_tcborders); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Cell Borders; [§17.4.66](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-469-part-1). Physical edge styles, widths and colors are emitted; themed controls and diagonal edges are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0. Cell fill emits a solid hex background; patterns and themes are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `noWrap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Don't Wrap Cell Content; [§17.4.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0421-0427-0083.md#source-page-423-part-1). Cell no_wrap emits an editable DOCX flag. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tcMar` — [w:CT_TcMar](tables.md#w-ct_tcmar); Strict, Transitional; minOccurs=0; maxOccurs=1. Single Table Cell Margins; [§17.4.68](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1). Physical cell margins and shorthands are emitted; logical edges are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0; maxOccurs=1. Cell ltr, rtl, tb_rl and bt_lr are accepted; editable vertical directions and paragraph bidi are emitted. Other standard directions are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tcFitText` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Fit Text Within Cell; [§17.4.67](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1).
- [x] `vAlign` — [w:CT_VerticalJc](tables.md#w-ct_verticaljc); Strict, Transitional; minOccurs=0. Cell top/center/bottom alignment is normalized and emitted. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `hideMark` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Ignore End Of Cell Marker In Row Height Calculation; [§17.4.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0415-0420-0082.md#source-page-415-part-1).
- [ ] `headers` — [w:CT_Headers](tables.md#w-ct_headers); Strict, Transitional; minOccurs=0. Header Cells Associated With Table Cell; [§17.4.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-413-part-1).
- [ ] `cellIns` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellDel` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellMerge` — [w:CT_CellMergeTrackChange](tables.md#w-ct_cellmergetrackchange); Strict, Transitional; minOccurs=0.
- [ ] `tcPrChange` — [w:CT_TcPrChange](tables.md#w-ct_tcprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_tblprex"></a>

## w:CT_TblPrEx

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `tblW` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `jc` — [w:CT_JcTable](tables.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblCellSpacing` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblInd` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblBorders` — [w:CT_TblBorders](tables.md#w-ct_tblborders); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblLayout` — [w:CT_TblLayoutType](tables.md#w-ct_tbllayouttype); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblCellMar` — [w:CT_TblCellMar](tables.md#w-ct_tblcellmar); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblLook` — [w:CT_TblLook](tables.md#w-ct_tbllook); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblPrExChange` — [w:CT_TblPrExChange](tables.md#w-ct_tblprexchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_tblgrid"></a>

## w:CT_TblGrid

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `gridCol` — [w:CT_TblGridCol](tables.md#w-ct_tblgridcol); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Grid Column Definition; [§17.4.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-407-part-1).
- [ ] `tblGridChange` — [w:CT_TblGridChange](tables.md#w-ct_tblgridchange); Strict, Transitional; minOccurs=0.

<a id="a-ct_tableproperties"></a>

## a:CT_TableProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectLst` — [a:CT_EffectList](tables.md#a-ct_effectlist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectDag` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tableStyle` — [a:CT_TableStyle](tables.md#a-ct_tablestyle); Strict, Transitional.
- [ ] `tableStyleId` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@rtl` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@firstRow` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@firstCol` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@lastRow` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@lastCol` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@bandRow` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@bandCol` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_tablecellproperties"></a>

## a:CT_TableCellProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `lnL` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lnR` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lnT` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lnB` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lnTlToBr` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lnBlToTr` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `cell3D` — [a:CT_Cell3D](tables.md#a-ct_cell3d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `headers` — [a:CT_Headers](tables.md#a-ct_headers); Strict, Transitional; minOccurs=0.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@marL` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional; default=91440.
- [ ] `@marR` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional; default=91440.
- [ ] `@marT` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional; default=45720.
- [ ] `@marB` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); Strict, Transitional; use=optional; default=45720.
- [ ] `@vert` — [a:ST_TextVerticalType](values-drawing.md#a-st_textverticaltype); Strict, Transitional; use=optional; default=horz.
- [ ] `@anchor` — [a:ST_TextAnchoringType](values-drawing.md#a-st_textanchoringtype); Strict, Transitional; use=optional; default=t.
- [ ] `@anchorCtr` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@horzOverflow` — [a:ST_TextHorzOverflowType](values-drawing.md#a-st_texthorzoverflowtype); Strict, Transitional; use=optional; default=clip.

<a id="a-ct_tablegrid"></a>

## a:CT_TableGrid

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `gridCol` — [a:CT_TableCol](tables.md#a-ct_tablecol); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="w-ct_tbl"></a>

## w:CT_Tbl

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

Empty marker type; no attributes or child properties.

<a id="w-ct_row"></a>

## w:CT_Row

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@rsidRPr` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidR` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidDel` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidTr` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.

<a id="w-ct_tc"></a>

## w:CT_Tc

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@id` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.

<a id="a-ct_tablerow"></a>

## a:CT_TableRow

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@h` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.

<a id="a-ct_tablecell"></a>

## a:CT_TableCell

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@rowSpan` — `xsd:int`; Strict, Transitional; use=optional; default=1.
- [ ] `@gridSpan` — `xsd:int`; Strict, Transitional; use=optional; default=1.
- [ ] `@hMerge` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@vMerge` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@id` — `xsd:string`; Strict, Transitional; use=optional.

<a id="w-ct_tblppr"></a>

## w:CT_TblPPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@leftFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@rightFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@topFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@bottomFromText` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@vertAnchor` — [w:ST_VAnchor](values-word.md#w-st_vanchor); Strict, Transitional.
- [ ] `@horzAnchor` — [w:ST_HAnchor](values-word.md#w-st_hanchor); Strict, Transitional.
- [ ] `@tblpXSpec` — [s:ST_XAlign](values-shared.md#s-st_xalign); Strict, Transitional.
- [ ] `@tblpX` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional.
- [ ] `@tblpYSpec` — [s:ST_YAlign](values-shared.md#s-st_yalign); Strict, Transitional.
- [ ] `@tblpY` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional.

<a id="w-ct_tbloverlap"></a>

## w:CT_TblOverlap

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TblOverlap](values-word.md#w-st_tbloverlap); Strict, Transitional; use=required.

<a id="w-ct_tblwidth"></a>

## w:CT_TblWidth

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@w` — [w:ST_MeasurementOrPercent](values-word.md#w-st_measurementorpercent); Strict, Transitional.
- [ ] `@type` — [w:ST_TblWidth](values-word.md#w-st_tblwidth); Strict, Transitional.

<a id="w-ct_jctable"></a>

## w:CT_JcTable

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_JcTable](values-word.md#w-st_jctable); Strict, Transitional; use=required.

<a id="w-ct_tblborders"></a>

## w:CT_TblBorders

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `start` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `left` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `end` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `right` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `insideH` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `insideV` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.

<a id="w-ct_tbllayouttype"></a>

## w:CT_TblLayoutType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@type` — [w:ST_TblLayoutType](values-word.md#w-st_tbllayouttype); Strict, Transitional.

<a id="w-ct_tblcellmar"></a>

## w:CT_TblCellMar

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `start` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `left` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `end` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `right` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Transitional; minOccurs=0.

<a id="w-ct_tbllook"></a>

## w:CT_TblLook

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@firstRow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastRow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@firstColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@noHBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@noVBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@val` — [w:ST_ShortHexNumber](values-word.md#w-st_shorthexnumber); Transitional.

<a id="w-ct_tblprchange"></a>

## w:CT_TblPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `tblPr` — [w:CT_TblPrBase](styles.md#w-ct_tblprbase); Strict, Transitional.

<a id="w-ct_height"></a>

## w:CT_Height

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.
- [ ] `@hRule` — [w:ST_HeightRule](values-word.md#w-st_heightrule); Strict, Transitional.

<a id="w-ct_trprchange"></a>

## w:CT_TrPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `trPr` — [w:CT_TrPrBase](tables.md#w-ct_trprbase); Strict, Transitional; minOccurs=1.

<a id="w-ct_hmerge"></a>

## w:CT_HMerge

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd).

- [ ] `@val` — [w:ST_Merge](values-word.md#w-st_merge); Transitional.

<a id="w-ct_vmerge"></a>

## w:CT_VMerge

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Merge](values-word.md#w-st_merge); Strict, Transitional.

<a id="w-ct_tcborders"></a>

## w:CT_TcBorders

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `start` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `left` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `end` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `right` — [w:CT_Border](word-runs.md#w-ct_border); Transitional; minOccurs=0.
- [ ] `insideH` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `insideV` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `tl2br` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0. Table Cell Top Left to Bottom Right Diagonal Border; [§17.4.73](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0473-0479-0091.md#source-page-478-part-1).
- [ ] `tr2bl` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0. Table Cell Top Right to Bottom Left Diagonal Border; [§17.4.79](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0480-0485-0092.md#source-page-483-part-1).

<a id="w-ct_tcmar"></a>

## w:CT_TcMar

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `start` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `left` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `end` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `right` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Transitional; minOccurs=0.

<a id="w-ct_verticaljc"></a>

## w:CT_VerticalJc

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_VerticalJc](values-word.md#w-st_verticaljc); Strict, Transitional; use=required.

<a id="w-ct_headers"></a>

## w:CT_Headers

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `header` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional. Header Cell Reference; [§17.4.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-411-part-1).

<a id="w-ct_cellmergetrackchange"></a>

## w:CT_CellMergeTrackChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `@vMerge` — [w:ST_AnnotationVMerge](values-word.md#w-st_annotationvmerge); Strict, Transitional; use=optional.
- [ ] `@vMergeOrig` — [w:ST_AnnotationVMerge](values-word.md#w-st_annotationvmerge); Strict, Transitional; use=optional.

<a id="w-ct_tcprchange"></a>

## w:CT_TcPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `tcPr` — [w:CT_TcPrInner](tables.md#w-ct_tcprinner); Strict, Transitional; minOccurs=1.

<a id="w-ct_tblprexchange"></a>

## w:CT_TblPrExChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `tblPrEx` — [w:CT_TblPrExBase](tables.md#w-ct_tblprexbase); Strict, Transitional; minOccurs=1.

<a id="w-ct_tblgridcol"></a>

## w:CT_TblGridCol

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@w` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional.

<a id="w-ct_tblgridchange"></a>

## w:CT_TblGridChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `tblGrid` — [w:CT_TblGridBase](tables.md#w-ct_tblgridbase); Strict, Transitional.

<a id="a-ct_nofillproperties"></a>

## a:CT_NoFillProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_solidcolorfillproperties"></a>

## a:CT_SolidColorFillProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_gradientfillproperties"></a>

## a:CT_GradientFillProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `gsLst` — [a:CT_GradientStopList](tables.md#a-ct_gradientstoplist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lin` — [a:CT_LinearShadeProperties](tables.md#a-ct_linearshadeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `path` — [a:CT_PathShadeProperties](tables.md#a-ct_pathshadeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tileRect` — [a:CT_RelativeRect](images.md#a-ct_relativerect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@flip` — [a:ST_TileFlipMode](values-drawing.md#a-st_tileflipmode); Strict, Transitional; use=optional; default=none.
- [ ] `@rotWithShape` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="a-ct_patternfillproperties"></a>

## a:CT_PatternFillProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `fgClr` — [a:CT_Color](tables.md#a-ct_color); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `bgClr` — [a:CT_Color](tables.md#a-ct_color); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@prst` — [a:ST_PresetPatternVal](values-drawing.md#a-st_presetpatternval); Strict, Transitional; use=optional.

<a id="a-ct_groupfillproperties"></a>

## a:CT_GroupFillProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_effectlist"></a>

## a:CT_EffectList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `blur` — [a:CT_BlurEffect](tables.md#a-ct_blureffect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `fillOverlay` — [a:CT_FillOverlayEffect](tables.md#a-ct_filloverlayeffect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `glow` — [a:CT_GlowEffect](tables.md#a-ct_gloweffect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `innerShdw` — [a:CT_InnerShadowEffect](tables.md#a-ct_innershadoweffect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `outerShdw` — [a:CT_OuterShadowEffect](tables.md#a-ct_outershadoweffect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `prstShdw` — [a:CT_PresetShadowEffect](tables.md#a-ct_presetshadoweffect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `reflection` — [a:CT_ReflectionEffect](tables.md#a-ct_reflectioneffect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `softEdge` — [a:CT_SoftEdgesEffect](tables.md#a-ct_softedgeseffect); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_effectcontainer"></a>

## a:CT_EffectContainer

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `cont` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effect` — [a:CT_EffectReference](tables.md#a-ct_effectreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaBiLevel` — [a:CT_AlphaBiLevelEffect](tables.md#a-ct_alphabileveleffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaCeiling` — [a:CT_AlphaCeilingEffect](tables.md#a-ct_alphaceilingeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaFloor` — [a:CT_AlphaFloorEffect](tables.md#a-ct_alphaflooreffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaInv` — [a:CT_AlphaInverseEffect](tables.md#a-ct_alphainverseeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_AlphaModulateEffect](tables.md#a-ct_alphamodulateeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaModFix` — [a:CT_AlphaModulateFixedEffect](tables.md#a-ct_alphamodulatefixedeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaOutset` — [a:CT_AlphaOutsetEffect](tables.md#a-ct_alphaoutseteffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaRepl` — [a:CT_AlphaReplaceEffect](tables.md#a-ct_alphareplaceeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `biLevel` — [a:CT_BiLevelEffect](tables.md#a-ct_bileveleffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blend` — [a:CT_BlendEffect](tables.md#a-ct_blendeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blur` — [a:CT_BlurEffect](tables.md#a-ct_blureffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrChange` — [a:CT_ColorChangeEffect](tables.md#a-ct_colorchangeeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrRepl` — [a:CT_ColorReplaceEffect](tables.md#a-ct_colorreplaceeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `duotone` — [a:CT_DuotoneEffect](tables.md#a-ct_duotoneeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fill` — [a:CT_FillEffect](tables.md#a-ct_filleffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fillOverlay` — [a:CT_FillOverlayEffect](tables.md#a-ct_filloverlayeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `glow` — [a:CT_GlowEffect](tables.md#a-ct_gloweffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grayscl` — [a:CT_GrayscaleEffect](tables.md#a-ct_grayscaleeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hsl` — [a:CT_HSLEffect](tables.md#a-ct_hsleffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `innerShdw` — [a:CT_InnerShadowEffect](tables.md#a-ct_innershadoweffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_LuminanceEffect](tables.md#a-ct_luminanceeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `outerShdw` — [a:CT_OuterShadowEffect](tables.md#a-ct_outershadoweffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstShdw` — [a:CT_PresetShadowEffect](tables.md#a-ct_presetshadoweffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `reflection` — [a:CT_ReflectionEffect](tables.md#a-ct_reflectioneffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `relOff` — [a:CT_RelativeOffsetEffect](tables.md#a-ct_relativeoffseteffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `softEdge` — [a:CT_SoftEdgesEffect](tables.md#a-ct_softedgeseffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tint` — [a:CT_TintEffect](tables.md#a-ct_tinteffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `xfrm` — [a:CT_TransformEffect](tables.md#a-ct_transformeffect); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@type` — [a:ST_EffectContainerType](values-drawing.md#a-st_effectcontainertype); Strict, Transitional; use=optional; default=sib.
- [ ] `@name` — `xsd:token`; Strict, Transitional; use=optional.

<a id="a-ct_tablestyle"></a>

## a:CT_TableStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tblBg` — [a:CT_TableBackgroundStyle](tables.md#a-ct_tablebackgroundstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `wholeTbl` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `band1H` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `band2H` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `band1V` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `band2V` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lastCol` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `firstCol` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lastRow` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `seCell` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `swCell` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `firstRow` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `neCell` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `nwCell` — [a:CT_TablePartStyle](tables.md#a-ct_tablepartstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@styleId` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional; use=required.
- [ ] `@styleName` — `xsd:string`; Strict, Transitional; use=required.

<a id="a-ct_lineproperties"></a>

## a:CT_LineProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstDash` — [a:CT_PresetLineDashProperties](tables.md#a-ct_presetlinedashproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `custDash` — [a:CT_DashStopList](tables.md#a-ct_dashstoplist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `round` — [a:CT_LineJoinRound](tables.md#a-ct_linejoinround); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `bevel` — [a:CT_LineJoinBevel](tables.md#a-ct_linejoinbevel); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `miter` — [a:CT_LineJoinMiterProperties](tables.md#a-ct_linejoinmiterproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `headEnd` — [a:CT_LineEndProperties](tables.md#a-ct_lineendproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tailEnd` — [a:CT_LineEndProperties](tables.md#a-ct_lineendproperties); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@w` — [a:ST_LineWidth](values-drawing.md#a-st_linewidth); Strict, Transitional; use=optional. Image border.width emits physical DrawingML width and native border geometry. Other line-width users are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [ ] `@cap` — [a:ST_LineCap](values-drawing.md#a-st_linecap); Strict, Transitional; use=optional.
- [ ] `@cmpd` — [a:ST_CompoundLine](values-drawing.md#a-st_compoundline); Strict, Transitional; use=optional.
- [ ] `@algn` — [a:ST_PenAlignment](values-drawing.md#a-st_penalignment); Strict, Transitional; use=optional.

<a id="a-ct_cell3d"></a>

## a:CT_Cell3D

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `bevel` — [a:CT_Bevel](tables.md#a-ct_bevel); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lightRig` — [a:CT_LightRig](tables.md#a-ct_lightrig); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@prstMaterial` — [a:ST_PresetMaterialType](values-drawing.md#a-st_presetmaterialtype); Strict, Transitional; use=optional; default=plastic.

<a id="a-ct_headers"></a>

## a:CT_Headers

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `header` — `xsd:string`; Strict, Transitional.

<a id="a-ct_tablecol"></a>

## a:CT_TableCol

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@w` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=required.

<a id="w-ct_trprbase"></a>

## w:CT_TrPrBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `divId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `gridBefore` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Before First Cell; [§17.4.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-406-part-1).
- [ ] `gridAfter` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns After Last Cell; [§17.4.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0401-0406-0080.md#source-page-405-part-1).
- [ ] `wBefore` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width Before Table Row; [§17.4.86](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-492-part-1).
- [ ] `wAfter` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Width After Table Row; [§17.4.85](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-491-part-1).
- [x] `cantSplit` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Table Row Cannot Break Across Pages; [§17.4.6](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0384-0390-0077.md#source-page-387-part-1). Row cant_split emits the row-splitting flag. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `trHeight` — [w:CT_Height](tables.md#w-ct_height); Strict, Transitional; minOccurs=0. Table Row Height; [§17.4.80](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0480-0485-0092.md#source-page-484-part-1). Row height and auto/at_least/exact rules are emitted. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `tblHeader` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Repeat Table Row on Every New Page; [§17.4.49](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0441-0447-0086.md#source-page-443-part-1). Row repeat_header emits the repeating-header flag. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tblCellSpacing` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `jc` — [w:CT_JcTable](tables.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `hidden` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.

<a id="w-ct_tcprinner"></a>

## w:CT_TcPrInner

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcW` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1. Preferred Table Cell Width; [§17.4.71](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0473-0479-0091.md#source-page-475-part-1). Physical cell width is emitted; percentage widths are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `gridSpan` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Grid Columns Spanned by Current Table Cell; [§17.4.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-409-part-1). Positive colspan creates a logical span and editable DOCX gridSpan. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `hMerge` — [w:CT_HMerge](tables.md#w-ct_hmerge); Transitional; minOccurs=0.
- [x] `vMerge` — [w:CT_VMerge](tables.md#w-ct_vmerge); Strict, Transitional; minOccurs=0. Vertically Merged Cell; [§17.4.84](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0486-0492-0093.md#source-page-489-part-1). Positive rowspan creates a logical vertical span with restart/continuation DOCX cells. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tcBorders` — [w:CT_TcBorders](tables.md#w-ct_tcborders); Strict, Transitional; minOccurs=0; maxOccurs=1. Table Cell Borders; [§17.4.66](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-469-part-1). Physical edge styles, widths and colors are emitted; themed controls and diagonal edges are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0. Cell fill emits a solid hex background; patterns and themes are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [x] `noWrap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Don't Wrap Cell Content; [§17.4.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0421-0427-0083.md#source-page-423-part-1). Cell no_wrap emits an editable DOCX flag. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tcMar` — [w:CT_TcMar](tables.md#w-ct_tcmar); Strict, Transitional; minOccurs=0; maxOccurs=1. Single Table Cell Margins; [§17.4.68](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1). Physical cell margins and shorthands are emitted; logical edges are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0; maxOccurs=1. Cell ltr, rtl, tb_rl and bt_lr are accepted; editable vertical directions and paragraph bidi are emitted. Other standard directions are not exposed. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `tcFitText` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0; maxOccurs=1. Fit Text Within Cell; [§17.4.67](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0466-0472-0090.md#source-page-472-part-1).
- [x] `vAlign` — [w:CT_VerticalJc](tables.md#w-ct_verticaljc); Strict, Transitional; minOccurs=0. Cell top/center/bottom alignment is normalized and emitted. Evidence: [tables.md](../../../docs/specification/tables.md), [tables.rs](../../../tests/tables.rs).
- [ ] `hideMark` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Ignore End Of Cell Marker In Row Height Calculation; [§17.4.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0415-0420-0082.md#source-page-415-part-1).
- [ ] `headers` — [w:CT_Headers](tables.md#w-ct_headers); Strict, Transitional; minOccurs=0. Header Cells Associated With Table Cell; [§17.4.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-413-part-1).
- [ ] `cellIns` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellDel` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.
- [ ] `cellMerge` — [w:CT_CellMergeTrackChange](tables.md#w-ct_cellmergetrackchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_tblprexbase"></a>

## w:CT_TblPrExBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `tblW` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `jc` — [w:CT_JcTable](tables.md#w-ct_jctable); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblCellSpacing` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblInd` — [w:CT_TblWidth](tables.md#w-ct_tblwidth); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblBorders` — [w:CT_TblBorders](tables.md#w-ct_tblborders); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblLayout` — [w:CT_TblLayoutType](tables.md#w-ct_tbllayouttype); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblCellMar` — [w:CT_TblCellMar](tables.md#w-ct_tblcellmar); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tblLook` — [w:CT_TblLook](tables.md#w-ct_tbllook); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="w-ct_tblgridbase"></a>

## w:CT_TblGridBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `gridCol` — [w:CT_TblGridCol](tables.md#w-ct_tblgridcol); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Grid Column Definition; [§17.4.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0407-0414-0081.md#source-page-407-part-1).

<a id="a-ct_scrgbcolor"></a>

## a:CT_ScRgbColor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@r` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.
- [ ] `@g` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.
- [ ] `@b` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.

<a id="a-ct_srgbcolor"></a>

## a:CT_SRgbColor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@val` — [s:ST_HexColorRGB](values-shared.md#s-st_hexcolorrgb); Strict, Transitional; use=required.

<a id="a-ct_hslcolor"></a>

## a:CT_HslColor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@hue` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=required.
- [ ] `@sat` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.
- [ ] `@lum` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.

<a id="a-ct_systemcolor"></a>

## a:CT_SystemColor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@val` — [a:ST_SystemColorVal](values-drawing.md#a-st_systemcolorval); Strict, Transitional; use=required.
- [ ] `@lastClr` — [s:ST_HexColorRGB](values-shared.md#s-st_hexcolorrgb); Strict, Transitional; use=optional.

<a id="a-ct_schemecolor"></a>

## a:CT_SchemeColor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@val` — [a:ST_SchemeColorVal](values-drawing.md#a-st_schemecolorval); Strict, Transitional; use=required.

<a id="a-ct_presetcolor"></a>

## a:CT_PresetColor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tint` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `shade` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `comp` — [a:CT_ComplementTransform](tables.md#a-ct_complementtransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `inv` — [a:CT_InverseTransform](tables.md#a-ct_inversetransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gray` — [a:CT_GrayscaleTransform](tables.md#a-ct_grayscaletransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alpha` — [a:CT_PositiveFixedPercentage](tables.md#a-ct_positivefixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaOff` — [a:CT_FixedPercentage](tables.md#a-ct_fixedpercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `alphaMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hue` — [a:CT_PositiveFixedAngle](tables.md#a-ct_positivefixedangle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueOff` — [a:CT_Angle](tables.md#a-ct_angle); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hueMod` — [a:CT_PositivePercentage](tables.md#a-ct_positivepercentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sat` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `satMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lum` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lumMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `red` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `redMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `green` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `greenMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blue` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueOff` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blueMod` — [a:CT_Percentage](tables.md#a-ct_percentage); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gamma` — [a:CT_GammaTransform](tables.md#a-ct_gammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `invGamma` — [a:CT_InverseGammaTransform](tables.md#a-ct_inversegammatransform); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@val` — [a:ST_PresetColorVal](values-drawing.md#a-st_presetcolorval); Strict, Transitional; use=required.

<a id="a-ct_gradientstoplist"></a>

## a:CT_GradientStopList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `gs` — [a:CT_GradientStop](tables.md#a-ct_gradientstop); Strict, Transitional; minOccurs=2; maxOccurs=unbounded.

<a id="a-ct_linearshadeproperties"></a>

## a:CT_LinearShadeProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@ang` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional.
- [ ] `@scaled` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="a-ct_pathshadeproperties"></a>

## a:CT_PathShadeProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `fillToRect` — [a:CT_RelativeRect](images.md#a-ct_relativerect); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@path` — [a:ST_PathShadeType](values-drawing.md#a-st_pathshadetype); Strict, Transitional; use=optional.

<a id="a-ct_color"></a>

## a:CT_Color

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_blureffect"></a>

## a:CT_BlurEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@rad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@grow` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="a-ct_filloverlayeffect"></a>

## a:CT_FillOverlayEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@blend` — [a:ST_BlendMode](values-drawing.md#a-st_blendmode); Strict, Transitional; use=required.

<a id="a-ct_gloweffect"></a>

## a:CT_GlowEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@rad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.

<a id="a-ct_innershadoweffect"></a>

## a:CT_InnerShadowEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@blurRad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@dist` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@dir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional; default=0.

<a id="a-ct_outershadoweffect"></a>

## a:CT_OuterShadowEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@blurRad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@dist` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@dir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@sx` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@sy` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@kx` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@ky` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@algn` — [a:ST_RectAlignment](values-drawing.md#a-st_rectalignment); Strict, Transitional; use=optional; default=b.
- [ ] `@rotWithShape` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="a-ct_presetshadoweffect"></a>

## a:CT_PresetShadowEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@prst` — [a:ST_PresetShadowVal](values-drawing.md#a-st_presetshadowval); Strict, Transitional; use=required.
- [ ] `@dist` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@dir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional; default=0.

<a id="a-ct_reflectioneffect"></a>

## a:CT_ReflectionEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@blurRad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@stA` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@stPos` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=optional; default=0%.
- [ ] `@endA` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=optional; default=0%.
- [ ] `@endPos` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@dist` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@dir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@fadeDir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional; default=5400000.
- [ ] `@sx` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@sy` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@kx` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@ky` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@algn` — [a:ST_RectAlignment](values-drawing.md#a-st_rectalignment); Strict, Transitional; use=optional; default=b.
- [ ] `@rotWithShape` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="a-ct_softedgeseffect"></a>

## a:CT_SoftEdgesEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@rad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=required.

<a id="a-ct_effectreference"></a>

## a:CT_EffectReference

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@ref` — `xsd:token`; Strict, Transitional; use=required.

<a id="a-ct_alphabileveleffect"></a>

## a:CT_AlphaBiLevelEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@thresh` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=required.

<a id="a-ct_alphaceilingeffect"></a>

## a:CT_AlphaCeilingEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_alphaflooreffect"></a>

## a:CT_AlphaFloorEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_alphainverseeffect"></a>

## a:CT_AlphaInverseEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_alphamodulateeffect"></a>

## a:CT_AlphaModulateEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `cont` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_alphamodulatefixedeffect"></a>

## a:CT_AlphaModulateFixedEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@amt` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); Strict, Transitional; use=optional; default=100%. Image effects.transparency maps inverse opacity; native transparency clamps to 0..100. Other alpha-modulation use cases are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).

<a id="a-ct_alphaoutseteffect"></a>

## a:CT_AlphaOutsetEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@rad` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=optional; default=0.

<a id="a-ct_alphareplaceeffect"></a>

## a:CT_AlphaReplaceEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@a` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=required.

<a id="a-ct_bileveleffect"></a>

## a:CT_BiLevelEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@thresh` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=required.

<a id="a-ct_blendeffect"></a>

## a:CT_BlendEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `cont` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@blend` — [a:ST_BlendMode](values-drawing.md#a-st_blendmode); Strict, Transitional; use=required.

<a id="a-ct_colorchangeeffect"></a>

## a:CT_ColorChangeEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `clrFrom` — [a:CT_Color](tables.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrTo` — [a:CT_Color](tables.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@useA` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="a-ct_colorreplaceeffect"></a>

## a:CT_ColorReplaceEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_duotoneeffect"></a>

## a:CT_DuotoneEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_filleffect"></a>

## a:CT_FillEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_grayscaleeffect"></a>

## a:CT_GrayscaleEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_hsleffect"></a>

## a:CT_HSLEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@hue` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@sat` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=optional; default=0%.
- [ ] `@lum` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=optional; default=0%.

<a id="a-ct_luminanceeffect"></a>

## a:CT_LuminanceEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [x] `@bright` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=optional; default=0%. Image effects map brightness to thousandths of a percent in DrawingML and native painting. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).
- [x] `@contrast` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=optional; default=0%. Image effects map contrast to thousandths of a percent in DrawingML and native painting. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).

<a id="a-ct_relativeoffseteffect"></a>

## a:CT_RelativeOffsetEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@tx` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=0%.
- [ ] `@ty` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=0%.

<a id="a-ct_tinteffect"></a>

## a:CT_TintEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@hue` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@amt` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=optional; default=0%.

<a id="a-ct_transformeffect"></a>

## a:CT_TransformEffect

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@sx` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@sy` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@kx` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@ky` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); Strict, Transitional; use=optional; default=0.
- [ ] `@tx` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=optional; default=0.
- [ ] `@ty` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); Strict, Transitional; use=optional; default=0.

<a id="a-ct_tablebackgroundstyle"></a>

## a:CT_TableBackgroundStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `fill` — [a:CT_FillProperties](tables.md#a-ct_fillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fillRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effect` — [a:CT_EffectProperties](tables.md#a-ct_effectproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_tablepartstyle"></a>

## a:CT_TablePartStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tcTxStyle` — [a:CT_TableStyleTextStyle](tables.md#a-ct_tablestyletextstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tcStyle` — [a:CT_TableStyleCellStyle](tables.md#a-ct_tablestylecellstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_presetlinedashproperties"></a>

## a:CT_PresetLineDashProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_PresetLineDashVal](values-drawing.md#a-st_presetlinedashval); Strict, Transitional; use=optional. Image border.style supports solid, dashed and dotted DOCX strokes; native borders remain solid. Other dash values are not exposed. Evidence: [images.md](../../../docs/specification/images.md), [images.rs](../../../tests/images.rs).

<a id="a-ct_dashstoplist"></a>

## a:CT_DashStopList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `ds` — [a:CT_DashStop](tables.md#a-ct_dashstop); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_linejoinround"></a>

## a:CT_LineJoinRound

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_linejoinbevel"></a>

## a:CT_LineJoinBevel

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_linejoinmiterproperties"></a>

## a:CT_LineJoinMiterProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@lim` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); Strict, Transitional; use=optional.

<a id="a-ct_lineendproperties"></a>

## a:CT_LineEndProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@type` — [a:ST_LineEndType](values-drawing.md#a-st_lineendtype); Strict, Transitional; use=optional; default=none.
- [ ] `@w` — [a:ST_LineEndWidth](values-drawing.md#a-st_lineendwidth); Strict, Transitional; use=optional.
- [ ] `@len` — [a:ST_LineEndLength](values-drawing.md#a-st_lineendlength); Strict, Transitional; use=optional.

<a id="a-ct_bevel"></a>

## a:CT_Bevel

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@w` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=76200.
- [ ] `@h` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); Strict, Transitional; use=optional; default=76200.
- [ ] `@prst` — [a:ST_BevelPresetType](values-drawing.md#a-st_bevelpresettype); Strict, Transitional; use=optional; default=circle.

<a id="a-ct_lightrig"></a>

## a:CT_LightRig

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `rot` — [a:CT_SphereCoords](tables.md#a-ct_spherecoords); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@rig` — [a:ST_LightRigType](values-drawing.md#a-st_lightrigtype); Strict, Transitional; use=required.
- [ ] `@dir` — [a:ST_LightRigDirection](values-drawing.md#a-st_lightrigdirection); Strict, Transitional; use=required.

<a id="a-ct_positivefixedpercentage"></a>

## a:CT_PositiveFixedPercentage

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=required.

<a id="a-ct_complementtransform"></a>

## a:CT_ComplementTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_inversetransform"></a>

## a:CT_InverseTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_grayscaletransform"></a>

## a:CT_GrayscaleTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_fixedpercentage"></a>

## a:CT_FixedPercentage

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=required.

<a id="a-ct_positivepercentage"></a>

## a:CT_PositivePercentage

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); Strict, Transitional; use=required.

<a id="a-ct_positivefixedangle"></a>

## a:CT_PositiveFixedAngle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=required.

<a id="a-ct_angle"></a>

## a:CT_Angle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=required.

<a id="a-ct_percentage"></a>

## a:CT_Percentage

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@val` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.

<a id="a-ct_gammatransform"></a>

## a:CT_GammaTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_inversegammatransform"></a>

## a:CT_InverseGammaTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="a-ct_gradientstop"></a>

## a:CT_GradientStop

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@pos` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=required.

<a id="a-ct_fillproperties"></a>

## a:CT_FillProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_effectproperties"></a>

## a:CT_EffectProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `effectLst` — [a:CT_EffectList](tables.md#a-ct_effectlist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectDag` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_tablestyletextstyle"></a>

## a:CT_TableStyleTextStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `font` — [a:CT_FontCollection](fonts.md#a-ct_fontcollection); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fontRef` — [a:CT_FontReference](shapes-and-lines.md#a-ct_fontreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@b` — [a:ST_OnOffStyleType](values-drawing.md#a-st_onoffstyletype); Strict, Transitional; use=optional; default=def.
- [ ] `@i` — [a:ST_OnOffStyleType](values-drawing.md#a-st_onoffstyletype); Strict, Transitional; use=optional; default=def.

<a id="a-ct_tablestylecellstyle"></a>

## a:CT_TableStyleCellStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `tcBdr` — [a:CT_TableCellBorderStyle](tables.md#a-ct_tablecellborderstyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `fill` — [a:CT_FillProperties](tables.md#a-ct_fillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fillRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cell3D` — [a:CT_Cell3D](tables.md#a-ct_cell3d); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_dashstop"></a>

## a:CT_DashStop

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@d` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); Strict, Transitional; use=required.
- [ ] `@sp` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); Strict, Transitional; use=required.

<a id="a-ct_spherecoords"></a>

## a:CT_SphereCoords

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@lat` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=required.
- [ ] `@lon` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=required.
- [ ] `@rev` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); Strict, Transitional; use=required.

<a id="a-ct_tablecellborderstyle"></a>

## a:CT_TableCellBorderStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `left` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `right` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `top` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `bottom` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `insideH` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `insideV` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tl2br` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `tr2bl` — [a:CT_ThemeableLineStyle](tables.md#a-ct_themeablelinestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_themeablelinestyle"></a>

## a:CT_ThemeableLineStyle

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `ln` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lnRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional; minOccurs=1; maxOccurs=1.
