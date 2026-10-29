# Extensions value definitions

[Inventory guide](README.md)

These are complete schema value domains for the selected properties.
Property rows record Mote support and supported subsets separately.
Identical Strict and Transitional definitions are combined.
See also [prose-defined extensions](prose-extensions.md).

## Types

- [ahyp:ST_HyperlinkColor](#ahyp-st_hyperlinkcolor)
- [w14:ST_BevelPresetType](#w14-st_bevelpresettype)
- [w14:ST_CompoundLine](#w14-st_compoundline)
- [w14:ST_Ligatures](#w14-st_ligatures)
- [w14:ST_LightRigDirection](#w14-st_lightrigdirection)
- [w14:ST_LightRigType](#w14-st_lightrigtype)
- [w14:ST_LineCap](#w14-st_linecap)
- [w14:ST_NumForm](#w14-st_numform)
- [w14:ST_NumSpacing](#w14-st_numspacing)
- [w14:ST_OnOff](#w14-st_onoff)
- [w14:ST_PathShadeType](#w14-st_pathshadetype)
- [w14:ST_PenAlignment](#w14-st_penalignment)
- [w14:ST_PresetCameraType](#w14-st_presetcameratype)
- [w14:ST_PresetLineDashVal](#w14-st_presetlinedashval)
- [w14:ST_PresetMaterialType](#w14-st_presetmaterialtype)
- [w14:ST_RectAlignment](#w14-st_rectalignment)
- [w14:ST_SchemeColorVal](#w14-st_schemecolorval)
- [w15:ST_SdtAppearance](#w15-st_sdtappearance)

<a id="ahyp-st_hyperlinkcolor"></a>

## ahyp:ST_HyperlinkColor

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `hlink`
- enumeration: value: `tx`

<a id="w14-st_bevelpresettype"></a>

## w14:ST_BevelPresetType

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

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

<a id="w14-st_compoundline"></a>

## w14:ST_CompoundLine

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `sng`
- enumeration: value: `dbl`
- enumeration: value: `thickThin`
- enumeration: value: `thinThick`
- enumeration: value: `tri`

<a id="w14-st_ligatures"></a>

## w14:ST_Ligatures

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `standard`
- enumeration: value: `contextual`
- enumeration: value: `historical`
- enumeration: value: `discretional`
- enumeration: value: `standardContextual`
- enumeration: value: `standardHistorical`
- enumeration: value: `contextualHistorical`
- enumeration: value: `standardDiscretional`
- enumeration: value: `contextualDiscretional`
- enumeration: value: `historicalDiscretional`
- enumeration: value: `standardContextualHistorical`
- enumeration: value: `standardContextualDiscretional`
- enumeration: value: `standardHistoricalDiscretional`
- enumeration: value: `contextualHistoricalDiscretional`
- enumeration: value: `all`

<a id="w14-st_lightrigdirection"></a>

## w14:ST_LightRigDirection

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `tl`
- enumeration: value: `t`
- enumeration: value: `tr`
- enumeration: value: `l`
- enumeration: value: `r`
- enumeration: value: `bl`
- enumeration: value: `b`
- enumeration: value: `br`

<a id="w14-st_lightrigtype"></a>

## w14:ST_LightRigType

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

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

<a id="w14-st_linecap"></a>

## w14:ST_LineCap

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `rnd`
- enumeration: value: `sq`
- enumeration: value: `flat`

<a id="w14-st_numform"></a>

## w14:ST_NumForm

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `default`
- enumeration: value: `lining`
- enumeration: value: `oldStyle`

<a id="w14-st_numspacing"></a>

## w14:ST_NumSpacing

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `default`
- enumeration: value: `proportional`
- enumeration: value: `tabular`

<a id="w14-st_onoff"></a>

## w14:ST_OnOff

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `true`
- enumeration: value: `false`
- enumeration: value: `0`
- enumeration: value: `1`

<a id="w14-st_pathshadetype"></a>

## w14:ST_PathShadeType

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `shape`
- enumeration: value: `circle`
- enumeration: value: `rect`

<a id="w14-st_penalignment"></a>

## w14:ST_PenAlignment

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `ctr`
- enumeration: value: `in`

<a id="w14-st_presetcameratype"></a>

## w14:ST_PresetCameraType

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

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

<a id="w14-st_presetlinedashval"></a>

## w14:ST_PresetLineDashVal

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `solid`
- enumeration: value: `dot`
- enumeration: value: `sysDot`
- enumeration: value: `dash`
- enumeration: value: `sysDash`
- enumeration: value: `lgDash`
- enumeration: value: `dashDot`
- enumeration: value: `sysDashDot`
- enumeration: value: `lgDashDot`
- enumeration: value: `lgDashDotDot`
- enumeration: value: `sysDashDotDot`

<a id="w14-st_presetmaterialtype"></a>

## w14:ST_PresetMaterialType

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

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
- enumeration: value: `none`

<a id="w14-st_rectalignment"></a>

## w14:ST_RectAlignment

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `tl`
- enumeration: value: `t`
- enumeration: value: `tr`
- enumeration: value: `l`
- enumeration: value: `ctr`
- enumeration: value: `r`
- enumeration: value: `bl`
- enumeration: value: `b`
- enumeration: value: `br`

<a id="w14-st_schemecolorval"></a>

## w14:ST_SchemeColorVal

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
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
- enumeration: value: `dk1`
- enumeration: value: `lt1`
- enumeration: value: `dk2`
- enumeration: value: `lt2`
- enumeration: value: `phClr`

<a id="w15-st_sdtappearance"></a>

## w15:ST_SdtAppearance

[MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `boundingBox`
- enumeration: value: `tags`
- enumeration: value: `hidden`
