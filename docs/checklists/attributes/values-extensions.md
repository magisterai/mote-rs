# Extensions value definitions

[Inventory guide](README.md)

These are complete schema value domains for the selected properties.
Property rows record Mote support and supported subsets separately.
Identical Strict and Transitional definitions are combined.
See also [prose-defined extensions](prose-extensions.md).

## Types

- [a2013__command:ST_ConnectorChangeBit](#a2013__command-st_connectorchangebit)
- [a2013__command:ST_ConnectorChangesBits](#a2013__command-st_connectorchangesbits)
- [a2013__command:ST_GraphicFrameChangeBit](#a2013__command-st_graphicframechangebit)
- [a2013__command:ST_GraphicFrameChangesBits](#a2013__command-st_graphicframechangesbits)
- [a2013__command:ST_GroupShapeChangeBit](#a2013__command-st_groupshapechangebit)
- [a2013__command:ST_GroupShapeChangesBits](#a2013__command-st_groupshapechangesbits)
- [a2013__command:ST_InkChangeBit](#a2013__command-st_inkchangebit)
- [a2013__command:ST_InkChangesBits](#a2013__command-st_inkchangesbits)
- [a2013__command:ST_PictureChangeBit](#a2013__command-st_picturechangebit)
- [a2013__command:ST_PictureChangesBits](#a2013__command-st_picturechangesbits)
- [a2013__command:ST_ShapeChangeBit](#a2013__command-st_shapechangebit)
- [a2013__command:ST_ShapeChangesBits](#a2013__command-st_shapechangesbits)
- [ahyp:ST_HyperlinkColor](#ahyp-st_hyperlinkcolor)
- [p2010:ST_TransitionCenterDirectionType](#p2010-st_transitioncenterdirectiontype)
- [p2010:ST_TransitionCornerAndCenterDirectionType](#p2010-st_transitioncornerandcenterdirectiontype)
- [p2010:ST_TransitionLeftRightDirectionType](#p2010-st_transitionleftrightdirectiontype)
- [p2010:ST_TransitionPattern](#p2010-st_transitionpattern)
- [p2010:ST_TransitionShredPattern](#p2010-st_transitionshredpattern)
- [p2010:ST_UniversalTimeOffset](#p2010-st_universaltimeoffset)
- [p2013__command:ST_CommentAuthorChangeBit](#p2013__command-st_commentauthorchangebit)
- [p2013__command:ST_CommentAuthorChangeBits](#p2013__command-st_commentauthorchangebits)
- [p2013__command:ST_CommentChangeBit](#p2013__command-st_commentchangebit)
- [p2013__command:ST_CommentChangeBits](#p2013__command-st_commentchangebits)
- [p2013__command:ST_CreationId](#p2013__command-st_creationid)
- [p2013__command:ST_DocumentChangeBit](#p2013__command-st_documentchangebit)
- [p2013__command:ST_DocumentChangeBits](#p2013__command-st_documentchangebits)
- [p2013__command:ST_MainMasterChangeBit](#p2013__command-st_mainmasterchangebit)
- [p2013__command:ST_MainMasterChangeBits](#p2013__command-st_mainmasterchangebits)
- [p2013__command:ST_SlideChangeBit](#p2013__command-st_slidechangebit)
- [p2013__command:ST_SlideChangeBits](#p2013__command-st_slidechangebits)
- [p2013__command:ST_SlideLayoutChangeBit](#p2013__command-st_slidelayoutchangebit)
- [p2013__command:ST_SlideLayoutChangeBits](#p2013__command-st_slidelayoutchangebits)
- [p2015_09:ST_TransitionMorphOption](#p2015_09-st_transitionmorphoption)
- [p2015_10:ST_ClientID](#p2015_10-st_clientid)
- [p2015_10:ST_ClientRevisionNumber](#p2015_10-st_clientrevisionnumber)
- [p2016_6:ST_ZoomObjectImageType](#p2016_6-st_zoomobjectimagetype)
- [p2017_3:ST_DisplayLocation](#p2017_3-st_displaylocation)
- [p2018_4:ST_ClassificationOutcomeType](#p2018_4-st_classificationoutcometype)
- [p2018_8:ST_AuthorId](#p2018_8-st_authorid)
- [p2018_8:ST_AuthorIdList](#p2018_8-st_authoridlist)
- [p2018_8:ST_CommentId](#p2018_8-st_commentid)
- [p2018_8:ST_CommentStatus](#p2018_8-st_commentstatus)
- [p2022_06__command:ST_CommentReplyV2ChangeBit](#p2022_06__command-st_commentreplyv2changebit)
- [p2022_06__command:ST_CommentReplyV2ChangeBits](#p2022_06__command-st_commentreplyv2changebits)
- [p2022_06__command:ST_CommentV2ChangeBit](#p2022_06__command-st_commentv2changebit)
- [p2022_06__command:ST_CommentV2ChangeBits](#p2022_06__command-st_commentv2changebits)
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

<a id="a2013__command-st_connectorchangebit"></a>

## a2013__command:ST_ConnectorChangeBit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `topLvl`
- enumeration: value: `modVis`
- enumeration: value: `replST`
- enumeration: value: `delST`
- enumeration: value: `replId`

<a id="a2013__command-st_connectorchangesbits"></a>

## a2013__command:ST_ConnectorChangesBits

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [a2013__command:ST_ConnectorChangeBit](values-extensions.md#a2013__command-st_connectorchangebit)

<a id="a2013__command-st_graphicframechangebit"></a>

## a2013__command:ST_GraphicFrameChangeBit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `topLvl`
- enumeration: value: `modVis`
- enumeration: value: `replST`
- enumeration: value: `delST`
- enumeration: value: `replId`
- enumeration: value: `modGraphic`

<a id="a2013__command-st_graphicframechangesbits"></a>

## a2013__command:ST_GraphicFrameChangesBits

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [a2013__command:ST_GraphicFrameChangeBit](values-extensions.md#a2013__command-st_graphicframechangebit)

<a id="a2013__command-st_groupshapechangebit"></a>

## a2013__command:ST_GroupShapeChangeBit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `topLvl`
- enumeration: value: `modVis`
- enumeration: value: `replST`
- enumeration: value: `delST`
- enumeration: value: `replId`

<a id="a2013__command-st_groupshapechangesbits"></a>

## a2013__command:ST_GroupShapeChangesBits

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [a2013__command:ST_GroupShapeChangeBit](values-extensions.md#a2013__command-st_groupshapechangebit)

<a id="a2013__command-st_inkchangebit"></a>

## a2013__command:ST_InkChangeBit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `topLvl`
- enumeration: value: `modVis`
- enumeration: value: `replST`
- enumeration: value: `delST`
- enumeration: value: `replId`
- enumeration: value: `reco`
- enumeration: value: `modStrokes`

<a id="a2013__command-st_inkchangesbits"></a>

## a2013__command:ST_InkChangesBits

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [a2013__command:ST_InkChangeBit](values-extensions.md#a2013__command-st_inkchangebit)

<a id="a2013__command-st_picturechangebit"></a>

## a2013__command:ST_PictureChangeBit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `topLvl`
- enumeration: value: `modVis`
- enumeration: value: `replST`
- enumeration: value: `delST`
- enumeration: value: `replId`
- enumeration: value: `modCrop`

<a id="a2013__command-st_picturechangesbits"></a>

## a2013__command:ST_PictureChangesBits

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [a2013__command:ST_PictureChangeBit](values-extensions.md#a2013__command-st_picturechangebit)

<a id="a2013__command-st_shapechangebit"></a>

## a2013__command:ST_ShapeChangeBit

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `topLvl`
- enumeration: value: `modVis`
- enumeration: value: `replST`
- enumeration: value: `delST`
- enumeration: value: `replId`
- enumeration: value: `modCrop`

<a id="a2013__command-st_shapechangesbits"></a>

## a2013__command:ST_ShapeChangesBits

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- list: itemType: [a2013__command:ST_ShapeChangeBit](values-extensions.md#a2013__command-st_shapechangebit)

<a id="ahyp-st_hyperlinkcolor"></a>

## ahyp:ST_HyperlinkColor

[MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `hlink`
- enumeration: value: `tx`

<a id="p2010-st_transitioncenterdirectiontype"></a>

## p2010:ST_TransitionCenterDirectionType

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `center`

<a id="p2010-st_transitioncornerandcenterdirectiontype"></a>

## p2010:ST_TransitionCornerAndCenterDirectionType

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- union: memberTypes: [p:ST_TransitionCornerDirectionType](values-drawing.md#p-st_transitioncornerdirectiontype), [p2010:ST_TransitionCenterDirectionType](values-extensions.md#p2010-st_transitioncenterdirectiontype)

<a id="p2010-st_transitionleftrightdirectiontype"></a>

## p2010:ST_TransitionLeftRightDirectionType

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `l`
- enumeration: value: `r`

<a id="p2010-st_transitionpattern"></a>

## p2010:ST_TransitionPattern

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `diamond`
- enumeration: value: `hexagon`

<a id="p2010-st_transitionshredpattern"></a>

## p2010:ST_TransitionShredPattern

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `strip`
- enumeration: value: `rectangle`

<a id="p2010-st_universaltimeoffset"></a>

## p2010:ST_UniversalTimeOffset

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:string`

<a id="p2013__command-st_commentauthorchangebit"></a>

## p2013__command:ST_CommentAuthorChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `replId`

<a id="p2013__command-st_commentauthorchangebits"></a>

## p2013__command:ST_CommentAuthorChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2013__command:ST_CommentAuthorChangeBit](values-extensions.md#p2013__command-st_commentauthorchangebit)

<a id="p2013__command-st_commentchangebit"></a>

## p2013__command:ST_CommentChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `replId`

<a id="p2013__command-st_commentchangebits"></a>

## p2013__command:ST_CommentChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2013__command:ST_CommentChangeBit](values-extensions.md#p2013__command-st_commentchangebit)

<a id="p2013__command-st_creationid"></a>

## p2013__command:ST_CreationId

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:unsignedInt`

<a id="p2013__command-st_documentchangebit"></a>

## p2013__command:ST_DocumentChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `undo`
- enumeration: value: `redo`
- enumeration: value: `ext`
- enumeration: value: `custSel`
- enumeration: value: `mod`
- enumeration: value: `addSld`
- enumeration: value: `delSld`
- enumeration: value: `modSld`
- enumeration: value: `sldOrd`
- enumeration: value: `addMainMaster`
- enumeration: value: `delMainMaster`
- enumeration: value: `modMainMaster`
- enumeration: value: `mainMasterOrd`
- enumeration: value: `addSection`
- enumeration: value: `delSection`
- enumeration: value: `modSection`
- enumeration: value: `addCmAuthor`
- enumeration: value: `delCmAuthor`
- enumeration: value: `modCmAuthor`
- enumeration: value: `replTag`
- enumeration: value: `delTag`
- enumeration: value: `addCustShow`
- enumeration: value: `delCustShow`
- enumeration: value: `modCustShow`
- enumeration: value: `modNotesMaster`
- enumeration: value: `modHandout`
- enumeration: value: `modShowInfo`
- enumeration: value: `addOsfTaskPaneApp`
- enumeration: value: `delOsfTaskPaneApp`
- enumeration: value: `setSldSz`
- enumeration: value: `modRtl`
- enumeration: value: `modChgInfo`

<a id="p2013__command-st_documentchangebits"></a>

## p2013__command:ST_DocumentChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2013__command:ST_DocumentChangeBit](values-extensions.md#p2013__command-st_documentchangebit)

<a id="p2013__command-st_mainmasterchangebit"></a>

## p2013__command:ST_MainMasterChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `addSp`
- enumeration: value: `delSp`
- enumeration: value: `modSp`
- enumeration: value: `spOrd`
- enumeration: value: `new`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `replId`
- enumeration: value: `modTransition`
- enumeration: value: `modMedia`
- enumeration: value: `setBg`
- enumeration: value: `setFolMasterAnim`
- enumeration: value: `setFolMasterObjs`
- enumeration: value: `modClrScheme`
- enumeration: value: `addAnim`
- enumeration: value: `delAnim`
- enumeration: value: `modAnim`
- enumeration: value: `replTag`
- enumeration: value: `delTag`
- enumeration: value: `setClrOvrMap`
- enumeration: value: `delDesignElem`
- enumeration: value: `modShow`
- enumeration: value: `addSldLayout`
- enumeration: value: `delSldLayout`
- enumeration: value: `modSldLayout`
- enumeration: value: `sldLayoutOrd`

<a id="p2013__command-st_mainmasterchangebits"></a>

## p2013__command:ST_MainMasterChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2013__command:ST_MainMasterChangeBit](values-extensions.md#p2013__command-st_mainmasterchangebit)

<a id="p2013__command-st_slidechangebit"></a>

## p2013__command:ST_SlideChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `addSp`
- enumeration: value: `delSp`
- enumeration: value: `modSp`
- enumeration: value: `spOrd`
- enumeration: value: `new`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `replId`
- enumeration: value: `modTransition`
- enumeration: value: `modMedia`
- enumeration: value: `setBg`
- enumeration: value: `setFolMasterAnim`
- enumeration: value: `setFolMasterObjs`
- enumeration: value: `modClrScheme`
- enumeration: value: `addAnim`
- enumeration: value: `delAnim`
- enumeration: value: `modAnim`
- enumeration: value: `replTag`
- enumeration: value: `delTag`
- enumeration: value: `setClrOvrMap`
- enumeration: value: `delDesignElem`
- enumeration: value: `modShow`
- enumeration: value: `addCm`
- enumeration: value: `delCm`
- enumeration: value: `modCm`
- enumeration: value: `chgLayout`
- enumeration: value: `modNotes`
- enumeration: value: `modNotesTx`
- enumeration: value: `setSldSyncInfo`
- enumeration: value: `newSectionLinks`

<a id="p2013__command-st_slidechangebits"></a>

## p2013__command:ST_SlideChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2013__command:ST_SlideChangeBit](values-extensions.md#p2013__command-st_slidechangebit)

<a id="p2013__command-st_slidelayoutchangebit"></a>

## p2013__command:ST_SlideLayoutChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `addSp`
- enumeration: value: `delSp`
- enumeration: value: `modSp`
- enumeration: value: `spOrd`
- enumeration: value: `new`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `ord`
- enumeration: value: `replId`
- enumeration: value: `modTransition`
- enumeration: value: `modMedia`
- enumeration: value: `setBg`
- enumeration: value: `setFolMasterAnim`
- enumeration: value: `setFolMasterObjs`
- enumeration: value: `modClrScheme`
- enumeration: value: `addAnim`
- enumeration: value: `delAnim`
- enumeration: value: `modAnim`
- enumeration: value: `replTag`
- enumeration: value: `delTag`
- enumeration: value: `setClrOvrMap`
- enumeration: value: `delDesignElem`
- enumeration: value: `modShow`

<a id="p2013__command-st_slidelayoutchangebits"></a>

## p2013__command:ST_SlideLayoutChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2013__command:ST_SlideLayoutChangeBit](values-extensions.md#p2013__command-st_slidelayoutchangebit)

<a id="p2015_09-st_transitionmorphoption"></a>

## p2015_09:ST_TransitionMorphOption

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `byObject`
- enumeration: value: `byWord`
- enumeration: value: `byChar`

<a id="p2015_10-st_clientid"></a>

## p2015_10:ST_ClientID

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:string`

<a id="p2015_10-st_clientrevisionnumber"></a>

## p2015_10:ST_ClientRevisionNumber

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:unsignedInt`

<a id="p2016_6-st_zoomobjectimagetype"></a>

## p2016_6:ST_ZoomObjectImageType

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `preview`
- enumeration: value: `cover`

<a id="p2017_3-st_displaylocation"></a>

## p2017_3:ST_DisplayLocation

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:string`
- enumeration: value: `media`
- enumeration: value: `slide`

<a id="p2018_4-st_classificationoutcometype"></a>

## p2018_4:ST_ClassificationOutcomeType

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `none`
- enumeration: value: `hdr`
- enumeration: value: `ftr`
- enumeration: value: `watermark`

<a id="p2018_8-st_authorid"></a>

## p2018_8:ST_AuthorId

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: [s:ST_Guid](values-shared.md#s-st_guid)

<a id="p2018_8-st_authoridlist"></a>

## p2018_8:ST_AuthorIdList

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2018_8:ST_AuthorId](values-extensions.md#p2018_8-st_authorid)

<a id="p2018_8-st_commentid"></a>

## p2018_8:ST_CommentId

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: [s:ST_Guid](values-shared.md#s-st_guid)

<a id="p2018_8-st_commentstatus"></a>

## p2018_8:ST_CommentStatus

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `active`
- enumeration: value: `resolved`
- enumeration: value: `closed`

<a id="p2022_06__command-st_commentreplyv2changebit"></a>

## p2022_06__command:ST_CommentReplyV2ChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `modRxn`

<a id="p2022_06__command-st_commentreplyv2changebits"></a>

## p2022_06__command:ST_CommentReplyV2ChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2022_06__command:ST_CommentReplyV2ChangeBit](values-extensions.md#p2022_06__command-st_commentreplyv2changebit)

<a id="p2022_06__command-st_commentv2changebit"></a>

## p2022_06__command:ST_CommentV2ChangeBit

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- restriction: base: `xsd:token`
- enumeration: value: `add`
- enumeration: value: `del`
- enumeration: value: `mod`
- enumeration: value: `modTsk`
- enumeration: value: `modRxn`

<a id="p2022_06__command-st_commentv2changebits"></a>

## p2022_06__command:ST_CommentV2ChangeBits

[MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md):

- list: itemType: [p2022_06__command:ST_CommentV2ChangeBit](values-extensions.md#p2022_06__command-st_commentv2changebit)

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
