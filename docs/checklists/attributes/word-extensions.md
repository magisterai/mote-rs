# Word text and control extensions

[Inventory guide](README.md)

MS-DOCX extensions for run appearance and typography, paragraph metadata, numbering, section boundaries, symbols, and content controls. The attachment points are described in the inventory guide.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w14:glow](#w14-glow)
- [w14:shadow](#w14-shadow)
- [w14:reflection](#w14-reflection)
- [w14:textOutline](#w14-textoutline)
- [w14:textFill](#w14-textfill)
- [w14:scene3d](#w14-scene3d)
- [w14:props3d](#w14-props3d)
- [w14:ligatures](#w14-ligatures)
- [w14:numForm](#w14-numform)
- [w14:numSpacing](#w14-numspacing)
- [w14:stylisticSets](#w14-stylisticsets)
- [w14:cntxtAlts](#w14-cntxtalts)
- [w14:checkbox](#w14-checkbox)
- [w14:entityPicker](#w14-entitypicker)
- [w15:collapsed](#w15-collapsed)
- [w15:footnoteColumns](#w15-footnotecolumns)
- [w15:appearance](#w15-appearance)
- [w15:color](#w15-color)
- [w15:dataBinding](#w15-databinding)
- [w15:repeatingSection](#w15-repeatingsection)
- [w15:repeatingSectionItem](#w15-repeatingsectionitem)
- [w15:webExtensionCreated](#w15-webextensioncreated)
- [w15:webExtensionLinked](#w15-webextensionlinked)
- [w16se:symEx](#w16se-symex)
- [w14:noSpellErr](#w14-nospellerr)
- [w15:restartNumberingAfterBreak](#w15-restartnumberingafterbreak)
- [w16sdtdh:storeItemChecksum](#w16sdtdh-storeitemchecksum)
- [w16sdtfl:formattingAllowed](#w16sdtfl-formattingallowed)
- [w14:AG_Parids](#w14-ag_parids)
- [w14:CT_Glow](#w14-ct_glow)
- [w14:CT_Shadow](#w14-ct_shadow)
- [w14:CT_Reflection](#w14-ct_reflection)
- [w14:CT_TextOutlineEffect](#w14-ct_textoutlineeffect)
- [w14:CT_FillTextEffect](#w14-ct_filltexteffect)
- [w14:CT_Scene3D](#w14-ct_scene3d)
- [w14:CT_Props3D](#w14-ct_props3d)
- [w14:CT_Ligatures](#w14-ct_ligatures)
- [w14:CT_NumForm](#w14-ct_numform)
- [w14:CT_NumSpacing](#w14-ct_numspacing)
- [w14:CT_StylisticSets](#w14-ct_stylisticsets)
- [w14:CT_OnOff](#w14-ct_onoff)
- [w14:CT_SdtCheckbox](#w14-ct_sdtcheckbox)
- [w15:CT_SdtAppearance](#w15-ct_sdtappearance)
- [w15:CT_SdtRepeatedSection](#w15-ct_sdtrepeatedsection)
- [w16se:CT_SymEx](#w16se-ct_symex)
- [w14:CT_SRgbColor](#w14-ct_srgbcolor)
- [w14:CT_SchemeColor](#w14-ct_schemecolor)
- [w14:CT_SolidColorFillProperties](#w14-ct_solidcolorfillproperties)
- [w14:CT_GradientFillProperties](#w14-ct_gradientfillproperties)
- [w14:CT_PresetLineDashProperties](#w14-ct_presetlinedashproperties)
- [w14:CT_LineJoinMiterProperties](#w14-ct_linejoinmiterproperties)
- [w14:CT_Camera](#w14-ct_camera)
- [w14:CT_LightRig](#w14-ct_lightrig)
- [w14:CT_Bevel](#w14-ct_bevel)
- [w14:CT_Color](#w14-ct_color)
- [w14:CT_StyleSet](#w14-ct_styleset)
- [w14:CT_SdtCheckboxSymbol](#w14-ct_sdtcheckboxsymbol)
- [w14:CT_PositiveFixedPercentage](#w14-ct_positivefixedpercentage)
- [w14:CT_PositivePercentage](#w14-ct_positivepercentage)
- [w14:CT_Percentage](#w14-ct_percentage)
- [w14:CT_GradientStopList](#w14-ct_gradientstoplist)
- [w14:CT_LinearShadeProperties](#w14-ct_linearshadeproperties)
- [w14:CT_PathShadeProperties](#w14-ct_pathshadeproperties)
- [w14:CT_SphereCoords](#w14-ct_spherecoords)
- [w14:CT_GradientStop](#w14-ct_gradientstop)
- [w14:CT_RelativeRect](#w14-ct_relativerect)

<a id="w14-glow"></a>

## w14:glow

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0040-0044-0013.md).

- [ ] `glow` — [w14:CT_Glow](word-extensions.md#w14-ct_glow); MS-DOCX; minOccurs=0.

<a id="w14-shadow"></a>

## w14:shadow

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `shadow` — [w14:CT_Shadow](word-extensions.md#w14-ct_shadow); MS-DOCX; minOccurs=0.

<a id="w14-reflection"></a>

## w14:reflection

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `reflection` — [w14:CT_Reflection](word-extensions.md#w14-ct_reflection); MS-DOCX; minOccurs=0.

<a id="w14-textoutline"></a>

## w14:textOutline

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `textOutline` — [w14:CT_TextOutlineEffect](word-extensions.md#w14-ct_textoutlineeffect); MS-DOCX; minOccurs=0.

<a id="w14-textfill"></a>

## w14:textFill

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `textFill` — [w14:CT_FillTextEffect](word-extensions.md#w14-ct_filltexteffect); MS-DOCX; minOccurs=0.

<a id="w14-scene3d"></a>

## w14:scene3d

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `scene3d` — [w14:CT_Scene3D](word-extensions.md#w14-ct_scene3d); MS-DOCX; minOccurs=0.

<a id="w14-props3d"></a>

## w14:props3d

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `props3d` — [w14:CT_Props3D](word-extensions.md#w14-ct_props3d); MS-DOCX; minOccurs=0.

<a id="w14-ligatures"></a>

## w14:ligatures

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0040-0044-0013.md).

- [ ] `ligatures` — [w14:CT_Ligatures](word-extensions.md#w14-ct_ligatures); MS-DOCX; minOccurs=0.

<a id="w14-numform"></a>

## w14:numForm

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0040-0044-0013.md).

- [ ] `numForm` — [w14:CT_NumForm](word-extensions.md#w14-ct_numform); MS-DOCX; minOccurs=0.

<a id="w14-numspacing"></a>

## w14:numSpacing

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `numSpacing` — [w14:CT_NumSpacing](word-extensions.md#w14-ct_numspacing); MS-DOCX; minOccurs=0.

<a id="w14-stylisticsets"></a>

## w14:stylisticSets

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0045-0049-0014.md).

- [ ] `stylisticSets` — [w14:CT_StylisticSets](word-extensions.md#w14-ct_stylisticsets); MS-DOCX; minOccurs=0.

<a id="w14-cntxtalts"></a>

## w14:cntxtAlts

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/chunks/pages-0035-0039-0012.md).

- [ ] `cntxtAlts` — [w14:CT_OnOff](word-extensions.md#w14-ct_onoff); MS-DOCX; minOccurs=0.

<a id="w14-checkbox"></a>

## w14:checkbox

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `checkbox` — [w14:CT_SdtCheckbox](word-extensions.md#w14-ct_sdtcheckbox); MS-DOCX. Partial: `checkbox.kind=content_control` emits editable task controls with fixed checked/unchecked symbols; custom symbols are not exposed. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_controls.rs](../../../tests/docx_controls.rs).

<a id="w14-entitypicker"></a>

## w14:entityPicker

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `entityPicker` — [w:CT_Empty](controls.md#w-ct_empty); MS-DOCX.

<a id="w15-collapsed"></a>

## w15:collapsed

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `collapsed` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); MS-DOCX.

<a id="w15-footnotecolumns"></a>

## w15:footnoteColumns

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `footnoteColumns` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); MS-DOCX.

<a id="w15-appearance"></a>

## w15:appearance

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `appearance` — [w15:CT_SdtAppearance](word-extensions.md#w15-ct_sdtappearance); MS-DOCX.

<a id="w15-color"></a>

## w15:color

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `color` — [w:CT_Color](word-runs.md#w-ct_color); MS-DOCX.

<a id="w15-databinding"></a>

## w15:dataBinding

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `dataBinding` — [w:CT_DataBinding](controls.md#w-ct_databinding); MS-DOCX.

<a id="w15-repeatingsection"></a>

## w15:repeatingSection

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `repeatingSection` — [w15:CT_SdtRepeatedSection](word-extensions.md#w15-ct_sdtrepeatedsection); MS-DOCX.

<a id="w15-repeatingsectionitem"></a>

## w15:repeatingSectionItem

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `repeatingSectionItem` — [w:CT_Empty](controls.md#w-ct_empty); MS-DOCX.

<a id="w15-webextensioncreated"></a>

## w15:webExtensionCreated

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `webExtensionCreated` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); MS-DOCX.

<a id="w15-webextensionlinked"></a>

## w15:webExtensionLinked

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `webExtensionLinked` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); MS-DOCX.

<a id="w16se-symex"></a>

## w16se:symEx

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `symEx` — [w16se:CT_SymEx](word-extensions.md#w16se-ct_symex); MS-DOCX.

<a id="w14-nospellerr"></a>

## w14:noSpellErr

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@noSpellErr` — [w:ST_OnOff](values-word.md#w-st_onoff); MS-DOCX.

<a id="w15-restartnumberingafterbreak"></a>

## w15:restartNumberingAfterBreak

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@restartNumberingAfterBreak` — [w:ST_OnOff](values-word.md#w-st_onoff); MS-DOCX.

<a id="w16sdtdh-storeitemchecksum"></a>

## w16sdtdh:storeItemChecksum

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@storeItemChecksum` — [w:ST_String](values-word.md#w-st_string); MS-DOCX.

<a id="w16sdtfl-formattingallowed"></a>

## w16sdtfl:formattingAllowed

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@formattingAllowed` — [w:ST_OnOff](values-word.md#w-st_onoff); MS-DOCX.

<a id="w14-ag_parids"></a>

## w14:AG_Parids

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@paraId` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX.
- [ ] `@textId` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX.

<a id="w14-ct_glow"></a>

## w14:CT_Glow

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `srgbClr` — [w14:CT_SRgbColor](word-extensions.md#w14-ct_srgbcolor); MS-DOCX.
- [ ] `schemeClr` — [w14:CT_SchemeColor](word-extensions.md#w14-ct_schemecolor); MS-DOCX.
- [ ] `@rad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.

<a id="w14-ct_shadow"></a>

## w14:CT_Shadow

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `srgbClr` — [w14:CT_SRgbColor](word-extensions.md#w14-ct_srgbcolor); MS-DOCX.
- [ ] `schemeClr` — [w14:CT_SchemeColor](word-extensions.md#w14-ct_schemecolor); MS-DOCX.
- [ ] `@blurRad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@dist` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@dir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); MS-DOCX; use=optional.
- [ ] `@sx` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
- [ ] `@sy` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
- [ ] `@kx` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); MS-DOCX; use=optional.
- [ ] `@ky` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); MS-DOCX; use=optional.
- [ ] `@algn` — [w14:ST_RectAlignment](values-extensions.md#w14-st_rectalignment); MS-DOCX; use=optional.

<a id="w14-ct_reflection"></a>

## w14:CT_Reflection

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@blurRad` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@stA` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); MS-DOCX; use=optional.
- [ ] `@stPos` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); MS-DOCX; use=optional.
- [ ] `@endA` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); MS-DOCX; use=optional.
- [ ] `@endPos` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); MS-DOCX; use=optional.
- [ ] `@dist` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@dir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); MS-DOCX; use=optional.
- [ ] `@fadeDir` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); MS-DOCX; use=optional.
- [ ] `@sx` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
- [ ] `@sy` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
- [ ] `@kx` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); MS-DOCX; use=optional.
- [ ] `@ky` — [a:ST_FixedAngle](values-drawing.md#a-st_fixedangle); MS-DOCX; use=optional.
- [ ] `@algn` — [w14:ST_RectAlignment](values-extensions.md#w14-st_rectalignment); MS-DOCX; use=optional.

<a id="w14-ct_textoutlineeffect"></a>

## w14:CT_TextOutlineEffect

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `noFill` — [w:CT_Empty](controls.md#w-ct_empty); MS-DOCX.
- [ ] `solidFill` — [w14:CT_SolidColorFillProperties](word-extensions.md#w14-ct_solidcolorfillproperties); MS-DOCX.
- [ ] `gradFill` — [w14:CT_GradientFillProperties](word-extensions.md#w14-ct_gradientfillproperties); MS-DOCX.
- [ ] `prstDash` — [w14:CT_PresetLineDashProperties](word-extensions.md#w14-ct_presetlinedashproperties); MS-DOCX.
- [ ] `round` — [w:CT_Empty](controls.md#w-ct_empty); MS-DOCX.
- [ ] `bevel` — [w:CT_Empty](controls.md#w-ct_empty); MS-DOCX.
- [ ] `miter` — [w14:CT_LineJoinMiterProperties](word-extensions.md#w14-ct_linejoinmiterproperties); MS-DOCX.
- [ ] `@w` — [a:ST_LineWidth](values-drawing.md#a-st_linewidth); MS-DOCX; use=optional.
- [ ] `@cap` — [w14:ST_LineCap](values-extensions.md#w14-st_linecap); MS-DOCX; use=optional.
- [ ] `@cmpd` — [w14:ST_CompoundLine](values-extensions.md#w14-st_compoundline); MS-DOCX; use=optional.
- [ ] `@algn` — [w14:ST_PenAlignment](values-extensions.md#w14-st_penalignment); MS-DOCX; use=optional.

<a id="w14-ct_filltexteffect"></a>

## w14:CT_FillTextEffect

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `noFill` — [w:CT_Empty](controls.md#w-ct_empty); MS-DOCX.
- [ ] `solidFill` — [w14:CT_SolidColorFillProperties](word-extensions.md#w14-ct_solidcolorfillproperties); MS-DOCX.
- [ ] `gradFill` — [w14:CT_GradientFillProperties](word-extensions.md#w14-ct_gradientfillproperties); MS-DOCX.

<a id="w14-ct_scene3d"></a>

## w14:CT_Scene3D

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `camera` — [w14:CT_Camera](word-extensions.md#w14-ct_camera); MS-DOCX.
- [ ] `lightRig` — [w14:CT_LightRig](word-extensions.md#w14-ct_lightrig); MS-DOCX.

<a id="w14-ct_props3d"></a>

## w14:CT_Props3D

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `bevelT` — [w14:CT_Bevel](word-extensions.md#w14-ct_bevel); MS-DOCX; minOccurs=0.
- [ ] `bevelB` — [w14:CT_Bevel](word-extensions.md#w14-ct_bevel); MS-DOCX; minOccurs=0.
- [ ] `extrusionClr` — [w14:CT_Color](word-extensions.md#w14-ct_color); MS-DOCX; minOccurs=0.
- [ ] `contourClr` — [w14:CT_Color](word-extensions.md#w14-ct_color); MS-DOCX; minOccurs=0.
- [ ] `@extrusionH` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@contourW` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@prstMaterial` — [w14:ST_PresetMaterialType](values-extensions.md#w14-st_presetmaterialtype); MS-DOCX; use=optional.

<a id="w14-ct_ligatures"></a>

## w14:CT_Ligatures

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [w14:ST_Ligatures](values-extensions.md#w14-st_ligatures); MS-DOCX; use=required.

<a id="w14-ct_numform"></a>

## w14:CT_NumForm

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [w14:ST_NumForm](values-extensions.md#w14-st_numform); MS-DOCX; use=required.

<a id="w14-ct_numspacing"></a>

## w14:CT_NumSpacing

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [w14:ST_NumSpacing](values-extensions.md#w14-st_numspacing); MS-DOCX; use=required.

<a id="w14-ct_stylisticsets"></a>

## w14:CT_StylisticSets

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `styleSet` — [w14:CT_StyleSet](word-extensions.md#w14-ct_styleset); MS-DOCX; minOccurs=0; maxOccurs=unbounded.

<a id="w14-ct_onoff"></a>

## w14:CT_OnOff

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [w14:ST_OnOff](values-extensions.md#w14-st_onoff); MS-DOCX.

<a id="w14-ct_sdtcheckbox"></a>

## w14:CT_SdtCheckbox

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [x] `checked` — [w14:CT_OnOff](word-extensions.md#w14-ct_onoff); MS-DOCX; minOccurs=0. `item.checked` supplies Word 2010 task-control state and a matching fallback glyph; nested state alone does not create a task. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_controls.rs](../../../tests/docx_controls.rs).
- [ ] `checkedState` — [w14:CT_SdtCheckboxSymbol](word-extensions.md#w14-ct_sdtcheckboxsymbol); MS-DOCX; minOccurs=0.
- [ ] `uncheckedState` — [w14:CT_SdtCheckboxSymbol](word-extensions.md#w14-ct_sdtcheckboxsymbol); MS-DOCX; minOccurs=0.

<a id="w15-ct_sdtappearance"></a>

## w15:CT_SdtAppearance

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [w15:ST_SdtAppearance](values-extensions.md#w15-st_sdtappearance); MS-DOCX.

<a id="w15-ct_sdtrepeatedsection"></a>

## w15:CT_SdtRepeatedSection

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `sectionTitle` — [w:CT_String](word-paragraphs.md#w-ct_string); MS-DOCX; minOccurs=0.
- [ ] `doNotAllowInsertDeleteSection` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); MS-DOCX; minOccurs=0.

<a id="w16se-ct_symex"></a>

## w16se:CT_SymEx

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@font` — [w:ST_String](values-word.md#w-st_string); MS-DOCX.
- [ ] `@char` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX.

<a id="w14-ct_srgbcolor"></a>

## w14:CT_SRgbColor

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `tint` — [w14:CT_PositiveFixedPercentage](word-extensions.md#w14-ct_positivefixedpercentage); MS-DOCX.
- [ ] `shade` — [w14:CT_PositiveFixedPercentage](word-extensions.md#w14-ct_positivefixedpercentage); MS-DOCX.
- [ ] `alpha` — [w14:CT_PositiveFixedPercentage](word-extensions.md#w14-ct_positivefixedpercentage); MS-DOCX.
- [ ] `hueMod` — [w14:CT_PositivePercentage](word-extensions.md#w14-ct_positivepercentage); MS-DOCX.
- [ ] `sat` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `satOff` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `satMod` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `lum` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `lumOff` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `lumMod` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `@val` — [w:ST_HexColorRGB](values-word.md#w-st_hexcolorrgb); MS-DOCX; use=required.

<a id="w14-ct_schemecolor"></a>

## w14:CT_SchemeColor

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `tint` — [w14:CT_PositiveFixedPercentage](word-extensions.md#w14-ct_positivefixedpercentage); MS-DOCX.
- [ ] `shade` — [w14:CT_PositiveFixedPercentage](word-extensions.md#w14-ct_positivefixedpercentage); MS-DOCX.
- [ ] `alpha` — [w14:CT_PositiveFixedPercentage](word-extensions.md#w14-ct_positivefixedpercentage); MS-DOCX.
- [ ] `hueMod` — [w14:CT_PositivePercentage](word-extensions.md#w14-ct_positivepercentage); MS-DOCX.
- [ ] `sat` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `satOff` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `satMod` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `lum` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `lumOff` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `lumMod` — [w14:CT_Percentage](word-extensions.md#w14-ct_percentage); MS-DOCX.
- [ ] `@val` — [w14:ST_SchemeColorVal](values-extensions.md#w14-st_schemecolorval); MS-DOCX; use=required.

<a id="w14-ct_solidcolorfillproperties"></a>

## w14:CT_SolidColorFillProperties

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `srgbClr` — [w14:CT_SRgbColor](word-extensions.md#w14-ct_srgbcolor); MS-DOCX.
- [ ] `schemeClr` — [w14:CT_SchemeColor](word-extensions.md#w14-ct_schemecolor); MS-DOCX.

<a id="w14-ct_gradientfillproperties"></a>

## w14:CT_GradientFillProperties

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `gsLst` — [w14:CT_GradientStopList](word-extensions.md#w14-ct_gradientstoplist); MS-DOCX; minOccurs=0.
- [ ] `lin` — [w14:CT_LinearShadeProperties](word-extensions.md#w14-ct_linearshadeproperties); MS-DOCX.
- [ ] `path` — [w14:CT_PathShadeProperties](word-extensions.md#w14-ct_pathshadeproperties); MS-DOCX.

<a id="w14-ct_presetlinedashproperties"></a>

## w14:CT_PresetLineDashProperties

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [w14:ST_PresetLineDashVal](values-extensions.md#w14-st_presetlinedashval); MS-DOCX; use=optional.

<a id="w14-ct_linejoinmiterproperties"></a>

## w14:CT_LineJoinMiterProperties

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@lim` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); MS-DOCX; use=optional.

<a id="w14-ct_camera"></a>

## w14:CT_Camera

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@prst` — [w14:ST_PresetCameraType](values-extensions.md#w14-st_presetcameratype); MS-DOCX; use=required.

<a id="w14-ct_lightrig"></a>

## w14:CT_LightRig

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `rot` — [w14:CT_SphereCoords](word-extensions.md#w14-ct_spherecoords); MS-DOCX; minOccurs=0.
- [ ] `@rig` — [w14:ST_LightRigType](values-extensions.md#w14-st_lightrigtype); MS-DOCX; use=required.
- [ ] `@dir` — [w14:ST_LightRigDirection](values-extensions.md#w14-st_lightrigdirection); MS-DOCX; use=required.

<a id="w14-ct_bevel"></a>

## w14:CT_Bevel

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@w` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@h` — [a:ST_PositiveCoordinate](values-drawing.md#a-st_positivecoordinate); MS-DOCX; use=optional.
- [ ] `@prst` — [w14:ST_BevelPresetType](values-extensions.md#w14-st_bevelpresettype); MS-DOCX; use=optional.

<a id="w14-ct_color"></a>

## w14:CT_Color

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `srgbClr` — [w14:CT_SRgbColor](word-extensions.md#w14-ct_srgbcolor); MS-DOCX.
- [ ] `schemeClr` — [w14:CT_SchemeColor](word-extensions.md#w14-ct_schemecolor); MS-DOCX.

<a id="w14-ct_styleset"></a>

## w14:CT_StyleSet

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@id` — [w:ST_UnsignedDecimalNumber](values-word.md#w-st_unsigneddecimalnumber); MS-DOCX; use=required.
- [ ] `@val` — [w14:ST_OnOff](values-extensions.md#w14-st_onoff); MS-DOCX; use=optional.

<a id="w14-ct_sdtcheckboxsymbol"></a>

## w14:CT_SdtCheckboxSymbol

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@font` — [w:ST_String](values-word.md#w-st_string); MS-DOCX.
- [ ] `@val` — [w:ST_ShortHexNumber](values-word.md#w-st_shorthexnumber); MS-DOCX.

<a id="w14-ct_positivefixedpercentage"></a>

## w14:CT_PositiveFixedPercentage

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); MS-DOCX; use=required.

<a id="w14-ct_positivepercentage"></a>

## w14:CT_PositivePercentage

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); MS-DOCX; use=required.

<a id="w14-ct_percentage"></a>

## w14:CT_Percentage

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@val` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=required.

<a id="w14-ct_gradientstoplist"></a>

## w14:CT_GradientStopList

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `gs` — [w14:CT_GradientStop](word-extensions.md#w14-ct_gradientstop); MS-DOCX; minOccurs=2; maxOccurs=10.

<a id="w14-ct_linearshadeproperties"></a>

## w14:CT_LinearShadeProperties

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@ang` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); MS-DOCX; use=optional.
- [ ] `@scaled` — [w14:ST_OnOff](values-extensions.md#w14-st_onoff); MS-DOCX; use=optional.

<a id="w14-ct_pathshadeproperties"></a>

## w14:CT_PathShadeProperties

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `fillToRect` — [w14:CT_RelativeRect](word-extensions.md#w14-ct_relativerect); MS-DOCX; minOccurs=0.
- [ ] `@path` — [w14:ST_PathShadeType](values-extensions.md#w14-st_pathshadetype); MS-DOCX; use=optional.

<a id="w14-ct_spherecoords"></a>

## w14:CT_SphereCoords

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@lat` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); MS-DOCX; use=required.
- [ ] `@lon` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); MS-DOCX; use=required.
- [ ] `@rev` — [a:ST_PositiveFixedAngle](values-drawing.md#a-st_positivefixedangle); MS-DOCX; use=required.

<a id="w14-ct_gradientstop"></a>

## w14:CT_GradientStop

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `srgbClr` — [w14:CT_SRgbColor](word-extensions.md#w14-ct_srgbcolor); MS-DOCX.
- [ ] `schemeClr` — [w14:CT_SchemeColor](word-extensions.md#w14-ct_schemecolor); MS-DOCX.
- [ ] `@pos` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); MS-DOCX; use=required.

<a id="w14-ct_relativerect"></a>

## w14:CT_RelativeRect

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@l` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
- [ ] `@t` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
- [ ] `@r` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
- [ ] `@b` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-DOCX; use=optional.
