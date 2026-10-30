# Presentation, slide and layout properties

[Inventory guide](README.md)

Complete PresentationML deck, slide, master, layout and notes families, with shared DrawingML themes. Includes unsupported transitions, timing, media, comments, embedded objects and extension carriers. Standard fields do not define new Mote arguments.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [p:CT_Presentation](#p-ct_presentation)
- [p:CT_Slide](#p-ct_slide)
- [p:CT_SlideLayout](#p-ct_slidelayout)
- [p:CT_SlideMaster](#p-ct_slidemaster)
- [p:CT_NotesSlide](#p-ct_notesslide)
- [p:CT_NotesMaster](#p-ct_notesmaster)
- [a:CT_OfficeStyleSheet](#a-ct_officestylesheet)
- [p:CT_SlideMasterIdList](#p-ct_slidemasteridlist)
- [p:CT_NotesMasterIdList](#p-ct_notesmasteridlist)
- [p:CT_HandoutMasterIdList](#p-ct_handoutmasteridlist)
- [p:CT_SlideIdList](#p-ct_slideidlist)
- [p:CT_SlideSize](#p-ct_slidesize)
- [p:CT_SmartTags](#p-ct_smarttags)
- [p:CT_CustomShowList](#p-ct_customshowlist)
- [p:CT_PhotoAlbum](#p-ct_photoalbum)
- [p:CT_CustomerDataList](#p-ct_customerdatalist)
- [p:CT_Kinsoku](#p-ct_kinsoku)
- [p:CT_ModifyVerifier](#p-ct_modifyverifier)
- [p:CT_CommonSlideData](#p-ct_commonslidedata)
- [a:CT_ColorMappingOverride](#a-ct_colormappingoverride)
- [p:CT_SlideTransition](#p-ct_slidetransition)
- [p:CT_SlideTiming](#p-ct_slidetiming)
- [p:CT_HeaderFooter](#p-ct_headerfooter)
- [p:CT_SlideLayoutIdList](#p-ct_slidelayoutidlist)
- [a:CT_BaseStyles](#a-ct_basestyles)
- [a:CT_ObjectStyleDefaults](#a-ct_objectstyledefaults)
- [a:CT_ColorSchemeList](#a-ct_colorschemelist)
- [a:CT_CustomColorList](#a-ct_customcolorlist)
- [p:CT_SlideMasterIdListEntry](#p-ct_slidemasteridlistentry)
- [p:CT_NotesMasterIdListEntry](#p-ct_notesmasteridlistentry)
- [p:CT_HandoutMasterIdListEntry](#p-ct_handoutmasteridlistentry)
- [p:CT_SlideIdListEntry](#p-ct_slideidlistentry)
- [p:CT_CustomShow](#p-ct_customshow)
- [p:CT_CustomerData](#p-ct_customerdata)
- [p:CT_TagsData](#p-ct_tagsdata)
- [p:CT_Background](#p-ct_background)
- [p:CT_GroupShape](#p-ct_groupshape)
- [p:CT_ControlList](#p-ct_controllist)
- [a:CT_EmptyElement](#a-ct_emptyelement)
- [p:CT_OrientationTransition](#p-ct_orientationtransition)
- [p:CT_EightDirectionTransition](#p-ct_eightdirectiontransition)
- [p:CT_OptionalBlackTransition](#p-ct_optionalblacktransition)
- [p:CT_SideDirectionTransition](#p-ct_sidedirectiontransition)
- [p:CT_SplitTransition](#p-ct_splittransition)
- [p:CT_CornerDirectionTransition](#p-ct_cornerdirectiontransition)
- [p:CT_WheelTransition](#p-ct_wheeltransition)
- [p:CT_InOutTransition](#p-ct_inouttransition)
- [p:CT_TransitionSoundAction](#p-ct_transitionsoundaction)
- [p:CT_TimeNodeList](#p-ct_timenodelist)
- [p:CT_BuildList](#p-ct_buildlist)
- [p:CT_SlideLayoutIdListEntry](#p-ct_slidelayoutidlistentry)
- [a:CT_ColorScheme](#a-ct_colorscheme)
- [a:CT_StyleMatrix](#a-ct_stylematrix)
- [a:CT_DefaultShapeDefinition](#a-ct_defaultshapedefinition)
- [a:CT_ColorSchemeAndMapping](#a-ct_colorschemeandmapping)
- [a:CT_CustomColor](#a-ct_customcolor)
- [p:CT_SlideRelationshipList](#p-ct_sliderelationshiplist)
- [p:CT_BackgroundProperties](#p-ct_backgroundproperties)
- [p:CT_GroupShapeNonVisual](#p-ct_groupshapenonvisual)
- [p:CT_GraphicalObjectFrame](#p-ct_graphicalobjectframe)
- [p:CT_Picture](#p-ct_picture)
- [p:CT_Rel](#p-ct_rel)
- [p:CT_Control](#p-ct_control)
- [p:CT_TransitionStartSoundAction](#p-ct_transitionstartsoundaction)
- [p:CT_TLTimeNodeParallel](#p-ct_tltimenodeparallel)
- [p:CT_TLTimeNodeSequence](#p-ct_tltimenodesequence)
- [p:CT_TLTimeNodeExclusive](#p-ct_tltimenodeexclusive)
- [p:CT_TLAnimateBehavior](#p-ct_tlanimatebehavior)
- [p:CT_TLAnimateColorBehavior](#p-ct_tlanimatecolorbehavior)
- [p:CT_TLAnimateEffectBehavior](#p-ct_tlanimateeffectbehavior)
- [p:CT_TLAnimateMotionBehavior](#p-ct_tlanimatemotionbehavior)
- [p:CT_TLAnimateRotationBehavior](#p-ct_tlanimaterotationbehavior)
- [p:CT_TLAnimateScaleBehavior](#p-ct_tlanimatescalebehavior)
- [p:CT_TLCommandBehavior](#p-ct_tlcommandbehavior)
- [p:CT_TLSetBehavior](#p-ct_tlsetbehavior)
- [p:CT_TLMediaNodeAudio](#p-ct_tlmedianodeaudio)
- [p:CT_TLMediaNodeVideo](#p-ct_tlmedianodevideo)
- [p:CT_TLBuildParagraph](#p-ct_tlbuildparagraph)
- [p:CT_TLBuildDiagram](#p-ct_tlbuilddiagram)
- [p:CT_TLOleBuildChart](#p-ct_tlolebuildchart)
- [p:CT_TLGraphicalObjectBuild](#p-ct_tlgraphicalobjectbuild)
- [a:CT_FillStyleList](#a-ct_fillstylelist)
- [a:CT_LineStyleList](#a-ct_linestylelist)
- [a:CT_EffectStyleList](#a-ct_effectstylelist)
- [a:CT_BackgroundFillStyleList](#a-ct_backgroundfillstylelist)
- [p:CT_SlideRelationshipListEntry](#p-ct_sliderelationshiplistentry)
- [p:CT_GraphicalObjectFrameNonVisual](#p-ct_graphicalobjectframenonvisual)
- [p:CT_PictureNonVisual](#p-ct_picturenonvisual)
- [p:CT_TLCommonTimeNodeData](#p-ct_tlcommontimenodedata)
- [p:CT_TLTimeConditionList](#p-ct_tltimeconditionlist)
- [p:CT_TLCommonBehaviorData](#p-ct_tlcommonbehaviordata)
- [p:CT_TLTimeAnimateValueList](#p-ct_tltimeanimatevaluelist)
- [p:CT_TLByAnimateColorTransform](#p-ct_tlbyanimatecolortransform)
- [p:CT_TLAnimVariant](#p-ct_tlanimvariant)
- [p:CT_TLPoint](#p-ct_tlpoint)
- [p:CT_TLCommonMediaNodeData](#p-ct_tlcommonmedianodedata)
- [p:CT_TLTemplateList](#p-ct_tltemplatelist)
- [a:CT_AnimationGraphicalObjectBuildProperties](#a-ct_animationgraphicalobjectbuildproperties)
- [a:CT_EffectStyleItem](#a-ct_effectstyleitem)
- [p:CT_TLTimeCondition](#p-ct_tltimecondition)
- [p:CT_TLIterateData](#p-ct_tliteratedata)
- [p:CT_TLTimeTargetElement](#p-ct_tltimetargetelement)
- [p:CT_TLBehaviorAttributeNameList](#p-ct_tlbehaviorattributenamelist)
- [p:CT_TLTimeAnimateValue](#p-ct_tltimeanimatevalue)
- [p:CT_TLByRgbColorTransform](#p-ct_tlbyrgbcolortransform)
- [p:CT_TLByHslColorTransform](#p-ct_tlbyhslcolortransform)
- [p:CT_TLAnimVariantBooleanVal](#p-ct_tlanimvariantbooleanval)
- [p:CT_TLAnimVariantIntegerVal](#p-ct_tlanimvariantintegerval)
- [p:CT_TLAnimVariantFloatVal](#p-ct_tlanimvariantfloatval)
- [p:CT_TLAnimVariantStringVal](#p-ct_tlanimvariantstringval)
- [p:CT_TLTemplate](#p-ct_tltemplate)
- [a:CT_AnimationDgmBuildProperties](#a-ct_animationdgmbuildproperties)
- [a:CT_AnimationChartBuildProperties](#a-ct_animationchartbuildproperties)
- [p:CT_TLTriggerTimeNodeID](#p-ct_tltriggertimenodeid)
- [p:CT_TLTriggerRuntimeNode](#p-ct_tltriggerruntimenode)
- [p:CT_TLIterateIntervalTime](#p-ct_tliterateintervaltime)
- [p:CT_TLIterateIntervalPercentage](#p-ct_tliterateintervalpercentage)
- [p:CT_TLShapeTargetElement](#p-ct_tlshapetargetelement)
- [p:CT_TLSubShapeId](#p-ct_tlsubshapeid)
- [p:CT_TLOleChartTargetElement](#p-ct_tlolecharttargetelement)
- [p:CT_TLTextTargetElement](#p-ct_tltexttargetelement)
- [a:CT_AnimationElementChoice](#a-ct_animationelementchoice)
- [p:CT_IndexRange](#p-ct_indexrange)
- [a:CT_AnimationDgmElement](#a-ct_animationdgmelement)
- [a:CT_AnimationChartElement](#a-ct_animationchartelement)

<a id="p-ct_presentation"></a>

## p:CT_Presentation

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `sldMasterIdLst` — [p:CT_SlideMasterIdList](presentations.md#p-ct_slidemasteridlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `notesMasterIdLst` — [p:CT_NotesMasterIdList](presentations.md#p-ct_notesmasteridlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `handoutMasterIdLst` — [p:CT_HandoutMasterIdList](presentations.md#p-ct_handoutmasteridlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `sldIdLst` — [p:CT_SlideIdList](presentations.md#p-ct_slideidlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `sldSz` — [p:CT_SlideSize](presentations.md#p-ct_slidesize); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `notesSz` — [a:CT_PositiveSize2D](images.md#a-ct_positivesize2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `smartTags` — [p:CT_SmartTags](presentations.md#p-ct_smarttags); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `embeddedFontLst` — [p:CT_EmbeddedFontList](fonts.md#p-ct_embeddedfontlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `custShowLst` — [p:CT_CustomShowList](presentations.md#p-ct_customshowlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `photoAlbum` — [p:CT_PhotoAlbum](presentations.md#p-ct_photoalbum); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `custDataLst` — [p:CT_CustomerDataList](presentations.md#p-ct_customerdatalist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `kinsoku` — [p:CT_Kinsoku](presentations.md#p-ct_kinsoku); Strict, Transitional; minOccurs=0.
- [ ] `defaultTextStyle` — [a:CT_TextListStyle](drawing-text.md#a-ct_textliststyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `modifyVerifier` — [p:CT_ModifyVerifier](presentations.md#p-ct_modifyverifier); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@serverZoom` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=50%.
- [ ] `@firstSlideNum` — `xsd:int`; Strict, Transitional; use=optional; default=1.
- [ ] `@showSpecialPlsOnTitleSld` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@rtl` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@removePersonalInfoOnSave` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@compatMode` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@strictFirstAndLastChars` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@embedTrueTypeFonts` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@saveSubsetFonts` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@autoCompressPictures` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@bookmarkIdSeed` — [p:ST_BookmarkIdSeed](values-drawing.md#p-st_bookmarkidseed); Strict, Transitional; use=optional; default=1.
- [ ] `@conformance` — [s:ST_ConformanceClass](values-shared.md#s-st_conformanceclass); Strict, Transitional.

<a id="p-ct_slide"></a>

## p:CT_Slide

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cSld` — [p:CT_CommonSlideData](presentations.md#p-ct_commonslidedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrMapOvr` — [a:CT_ColorMappingOverride](presentations.md#a-ct_colormappingoverride); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `transition` — [p:CT_SlideTransition](presentations.md#p-ct_slidetransition); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `timing` — [p:CT_SlideTiming](presentations.md#p-ct_slidetiming); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@showMasterSp` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@showMasterPhAnim` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@show` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="p-ct_slidelayout"></a>

## p:CT_SlideLayout

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cSld` — [p:CT_CommonSlideData](presentations.md#p-ct_commonslidedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrMapOvr` — [a:CT_ColorMappingOverride](presentations.md#a-ct_colormappingoverride); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `transition` — [p:CT_SlideTransition](presentations.md#p-ct_slidetransition); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `timing` — [p:CT_SlideTiming](presentations.md#p-ct_slidetiming); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `hf` — [p:CT_HeaderFooter](presentations.md#p-ct_headerfooter); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@showMasterSp` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@showMasterPhAnim` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@matchingName` — `xsd:string`; Strict, Transitional; use=optional; default=.
- [ ] `@type` — [p:ST_SlideLayoutType](values-drawing.md#p-st_slidelayouttype); Strict, Transitional; use=optional; default=cust.
- [ ] `@preserve` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@userDrawn` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_slidemaster"></a>

## p:CT_SlideMaster

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cSld` — [p:CT_CommonSlideData](presentations.md#p-ct_commonslidedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrMap` — [a:CT_ColorMapping](charts.md#a-ct_colormapping); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sldLayoutIdLst` — [p:CT_SlideLayoutIdList](presentations.md#p-ct_slidelayoutidlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `transition` — [p:CT_SlideTransition](presentations.md#p-ct_slidetransition); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `timing` — [p:CT_SlideTiming](presentations.md#p-ct_slidetiming); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `hf` — [p:CT_HeaderFooter](presentations.md#p-ct_headerfooter); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `txStyles` — [p:CT_SlideMasterTextStyles](drawing-text.md#p-ct_slidemastertextstyles); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@preserve` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_notesslide"></a>

## p:CT_NotesSlide

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cSld` — [p:CT_CommonSlideData](presentations.md#p-ct_commonslidedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrMapOvr` — [a:CT_ColorMappingOverride](presentations.md#a-ct_colormappingoverride); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@showMasterSp` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@showMasterPhAnim` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="p-ct_notesmaster"></a>

## p:CT_NotesMaster

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cSld` — [p:CT_CommonSlideData](presentations.md#p-ct_commonslidedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrMap` — [a:CT_ColorMapping](charts.md#a-ct_colormapping); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hf` — [p:CT_HeaderFooter](presentations.md#p-ct_headerfooter); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `notesStyle` — [a:CT_TextListStyle](drawing-text.md#a-ct_textliststyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_officestylesheet"></a>

## a:CT_OfficeStyleSheet

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `themeElements` — [a:CT_BaseStyles](presentations.md#a-ct_basestyles); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `objectDefaults` — [a:CT_ObjectStyleDefaults](presentations.md#a-ct_objectstyledefaults); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extraClrSchemeLst` — [a:CT_ColorSchemeList](presentations.md#a-ct_colorschemelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `custClrLst` — [a:CT_CustomColorList](presentations.md#a-ct_customcolorlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=optional; default=.

<a id="p-ct_slidemasteridlist"></a>

## p:CT_SlideMasterIdList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `sldMasterId` — [p:CT_SlideMasterIdListEntry](presentations.md#p-ct_slidemasteridlistentry); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_notesmasteridlist"></a>

## p:CT_NotesMasterIdList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `notesMasterId` — [p:CT_NotesMasterIdListEntry](presentations.md#p-ct_notesmasteridlistentry); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_handoutmasteridlist"></a>

## p:CT_HandoutMasterIdList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `handoutMasterId` — [p:CT_HandoutMasterIdListEntry](presentations.md#p-ct_handoutmasteridlistentry); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_slideidlist"></a>

## p:CT_SlideIdList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `sldId` — [p:CT_SlideIdListEntry](presentations.md#p-ct_slideidlistentry); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_slidesize"></a>

## p:CT_SlideSize

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@cx` — [p:ST_SlideSizeCoordinate](values-drawing.md#p-st_slidesizecoordinate); Strict, Transitional; use=required.
- [ ] `@cy` — [p:ST_SlideSizeCoordinate](values-drawing.md#p-st_slidesizecoordinate); Strict, Transitional; use=required.
- [ ] `@type` — [p:ST_SlideSizeType](values-drawing.md#p-st_slidesizetype); Strict, Transitional; use=optional; default=custom.

<a id="p-ct_smarttags"></a>

## p:CT_SmartTags

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_customshowlist"></a>

## p:CT_CustomShowList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `custShow` — [p:CT_CustomShow](presentations.md#p-ct_customshow); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_photoalbum"></a>

## p:CT_PhotoAlbum

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@bw` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@showCaptions` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@layout` — [p:ST_PhotoAlbumLayout](values-drawing.md#p-st_photoalbumlayout); Strict, Transitional; use=optional; default=fitToSlide.
- [ ] `@frame` — [p:ST_PhotoAlbumFrameShape](values-drawing.md#p-st_photoalbumframeshape); Strict, Transitional; use=optional; default=frameStyle1.

<a id="p-ct_customerdatalist"></a>

## p:CT_CustomerDataList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `custData` — [p:CT_CustomerData](presentations.md#p-ct_customerdata); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.
- [ ] `tags` — [p:CT_TagsData](presentations.md#p-ct_tagsdata); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_kinsoku"></a>

## p:CT_Kinsoku

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@lang` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@invalStChars` — `xsd:string`; Strict, Transitional; use=required.
- [ ] `@invalEndChars` — `xsd:string`; Strict, Transitional; use=required.

<a id="p-ct_modifyverifier"></a>

## p:CT_ModifyVerifier

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@algorithmName` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@hashValue` — `xsd:base64Binary`; Strict, Transitional; use=optional.
- [ ] `@saltValue` — `xsd:base64Binary`; Strict, Transitional; use=optional.
- [ ] `@spinValue` — `xsd:unsignedInt`; Strict, Transitional; use=optional.
- [ ] `@cryptProviderType` — [s:ST_CryptProv](values-shared.md#s-st_cryptprov); Transitional; use=optional.
- [ ] `@cryptAlgorithmClass` — [s:ST_AlgClass](values-shared.md#s-st_algclass); Transitional; use=optional.
- [ ] `@cryptAlgorithmType` — [s:ST_AlgType](values-shared.md#s-st_algtype); Transitional; use=optional.
- [ ] `@cryptAlgorithmSid` — `xsd:unsignedInt`; Transitional; use=optional.
- [ ] `@spinCount` — `xsd:unsignedInt`; Transitional; use=optional.
- [ ] `@saltData` — `xsd:base64Binary`; Transitional; use=optional.
- [ ] `@hashData` — `xsd:base64Binary`; Transitional; use=optional.
- [ ] `@cryptProvider` — `xsd:string`; Transitional; use=optional.
- [ ] `@algIdExt` — `xsd:unsignedInt`; Transitional; use=optional.
- [ ] `@algIdExtSource` — `xsd:string`; Transitional; use=optional.
- [ ] `@cryptProviderTypeExt` — `xsd:unsignedInt`; Transitional; use=optional.
- [ ] `@cryptProviderTypeExtSource` — `xsd:string`; Transitional; use=optional.

<a id="p-ct_commonslidedata"></a>

## p:CT_CommonSlideData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `bg` — [p:CT_Background](presentations.md#p-ct_background); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `spTree` — [p:CT_GroupShape](presentations.md#p-ct_groupshape); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `custDataLst` — [p:CT_CustomerDataList](presentations.md#p-ct_customerdatalist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `controls` — [p:CT_ControlList](presentations.md#p-ct_controllist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=optional; default=.

<a id="a-ct_colormappingoverride"></a>

## a:CT_ColorMappingOverride

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `masterClrMapping` — [a:CT_EmptyElement](presentations.md#a-ct_emptyelement); Strict, Transitional.
- [ ] `overrideClrMapping` — [a:CT_ColorMapping](charts.md#a-ct_colormapping); Strict, Transitional.

<a id="p-ct_slidetransition"></a>

## p:CT_SlideTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `blinds` — [p:CT_OrientationTransition](presentations.md#p-ct_orientationtransition); Strict, Transitional.
- [ ] `checker` — [p:CT_OrientationTransition](presentations.md#p-ct_orientationtransition); Strict, Transitional.
- [ ] `circle` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `dissolve` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `comb` — [p:CT_OrientationTransition](presentations.md#p-ct_orientationtransition); Strict, Transitional.
- [ ] `cover` — [p:CT_EightDirectionTransition](presentations.md#p-ct_eightdirectiontransition); Strict, Transitional.
- [ ] `cut` — [p:CT_OptionalBlackTransition](presentations.md#p-ct_optionalblacktransition); Strict, Transitional.
- [ ] `diamond` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `fade` — [p:CT_OptionalBlackTransition](presentations.md#p-ct_optionalblacktransition); Strict, Transitional.
- [ ] `newsflash` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `plus` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `pull` — [p:CT_EightDirectionTransition](presentations.md#p-ct_eightdirectiontransition); Strict, Transitional.
- [ ] `push` — [p:CT_SideDirectionTransition](presentations.md#p-ct_sidedirectiontransition); Strict, Transitional.
- [ ] `random` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `randomBar` — [p:CT_OrientationTransition](presentations.md#p-ct_orientationtransition); Strict, Transitional.
- [ ] `split` — [p:CT_SplitTransition](presentations.md#p-ct_splittransition); Strict, Transitional.
- [ ] `strips` — [p:CT_CornerDirectionTransition](presentations.md#p-ct_cornerdirectiontransition); Strict, Transitional.
- [ ] `wedge` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `wheel` — [p:CT_WheelTransition](presentations.md#p-ct_wheeltransition); Strict, Transitional.
- [ ] `wipe` — [p:CT_SideDirectionTransition](presentations.md#p-ct_sidedirectiontransition); Strict, Transitional.
- [ ] `zoom` — [p:CT_InOutTransition](presentations.md#p-ct_inouttransition); Strict, Transitional.
- [ ] `sndAc` — [p:CT_TransitionSoundAction](presentations.md#p-ct_transitionsoundaction); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@spd` — [p:ST_TransitionSpeed](values-drawing.md#p-st_transitionspeed); Strict, Transitional; use=optional; default=fast.
- [ ] `@advClick` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@advTm` — `xsd:unsignedInt`; Strict, Transitional; use=optional.

<a id="p-ct_slidetiming"></a>

## p:CT_SlideTiming

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `tnLst` — [p:CT_TimeNodeList](presentations.md#p-ct_timenodelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `bldLst` — [p:CT_BuildList](presentations.md#p-ct_buildlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_headerfooter"></a>

## p:CT_HeaderFooter

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@sldNum` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@hdr` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@ftr` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@dt` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="p-ct_slidelayoutidlist"></a>

## p:CT_SlideLayoutIdList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `sldLayoutId` — [p:CT_SlideLayoutIdListEntry](presentations.md#p-ct_slidelayoutidlistentry); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_basestyles"></a>

## a:CT_BaseStyles

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `clrScheme` — [a:CT_ColorScheme](presentations.md#a-ct_colorscheme); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fontScheme` — [a:CT_FontScheme](fonts.md#a-ct_fontscheme); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `fmtScheme` — [a:CT_StyleMatrix](presentations.md#a-ct_stylematrix); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_objectstyledefaults"></a>

## a:CT_ObjectStyleDefaults

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `spDef` — [a:CT_DefaultShapeDefinition](presentations.md#a-ct_defaultshapedefinition); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `lnDef` — [a:CT_DefaultShapeDefinition](presentations.md#a-ct_defaultshapedefinition); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `txDef` — [a:CT_DefaultShapeDefinition](presentations.md#a-ct_defaultshapedefinition); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_colorschemelist"></a>

## a:CT_ColorSchemeList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extraClrScheme` — [a:CT_ColorSchemeAndMapping](presentations.md#a-ct_colorschemeandmapping); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_customcolorlist"></a>

## a:CT_CustomColorList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `custClr` — [a:CT_CustomColor](presentations.md#a-ct_customcolor); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_slidemasteridlistentry"></a>

## p:CT_SlideMasterIdListEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [p:ST_SlideMasterId](values-drawing.md#p-st_slidemasterid); Strict, Transitional; use=optional.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_notesmasteridlistentry"></a>

## p:CT_NotesMasterIdListEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_handoutmasteridlistentry"></a>

## p:CT_HandoutMasterIdListEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_slideidlistentry"></a>

## p:CT_SlideIdListEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [p:ST_SlideId](values-drawing.md#p-st_slideid); Strict, Transitional; use=required.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_customshow"></a>

## p:CT_CustomShow

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `sldLst` — [p:CT_SlideRelationshipList](presentations.md#p-ct_sliderelationshiplist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@name` — [p:ST_Name](values-drawing.md#p-st_name); Strict, Transitional; use=required.
- [ ] `@id` — `xsd:unsignedInt`; Strict, Transitional; use=required.

<a id="p-ct_customerdata"></a>

## p:CT_CustomerData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_tagsdata"></a>

## p:CT_TagsData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_background"></a>

## p:CT_Background

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `bgPr` — [p:CT_BackgroundProperties](presentations.md#p-ct_backgroundproperties); Strict, Transitional.
- [ ] `bgRef` — [a:CT_StyleMatrixReference](shapes-and-lines.md#a-ct_stylematrixreference); Strict, Transitional.
- [ ] `@bwMode` — [a:ST_BlackWhiteMode](values-drawing.md#a-st_blackwhitemode); Strict, Transitional; use=optional; default=white.

<a id="p-ct_groupshape"></a>

## p:CT_GroupShape

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `nvGrpSpPr` — [p:CT_GroupShapeNonVisual](presentations.md#p-ct_groupshapenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpSpPr` — [a:CT_GroupShapeProperties](charts.md#a-ct_groupshapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sp` — [p:CT_Shape](shapes-and-lines.md#p-ct_shape); Strict, Transitional.
- [ ] `grpSp` — [p:CT_GroupShape](presentations.md#p-ct_groupshape); Strict, Transitional.
- [ ] `graphicFrame` — [p:CT_GraphicalObjectFrame](presentations.md#p-ct_graphicalobjectframe); Strict, Transitional.
- [ ] `cxnSp` — [p:CT_Connector](shapes-and-lines.md#p-ct_connector); Strict, Transitional.
- [ ] `pic` — [p:CT_Picture](presentations.md#p-ct_picture); Strict, Transitional.
- [ ] `contentPart` — [p:CT_Rel](presentations.md#p-ct_rel); Strict, Transitional.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_controllist"></a>

## p:CT_ControlList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `control` — [p:CT_Control](presentations.md#p-ct_control); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_emptyelement"></a>

## a:CT_EmptyElement

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

Empty marker type; no attributes or child properties.

<a id="p-ct_orientationtransition"></a>

## p:CT_OrientationTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@dir` — [p:ST_Direction](values-drawing.md#p-st_direction); Strict, Transitional; use=optional; default=horz.

<a id="p-ct_eightdirectiontransition"></a>

## p:CT_EightDirectionTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@dir` — [p:ST_TransitionEightDirectionType](values-drawing.md#p-st_transitioneightdirectiontype); Strict, Transitional; use=optional; default=l.

<a id="p-ct_optionalblacktransition"></a>

## p:CT_OptionalBlackTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@thruBlk` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_sidedirectiontransition"></a>

## p:CT_SideDirectionTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@dir` — [p:ST_TransitionSideDirectionType](values-drawing.md#p-st_transitionsidedirectiontype); Strict, Transitional; use=optional; default=l.

<a id="p-ct_splittransition"></a>

## p:CT_SplitTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@orient` — [p:ST_Direction](values-drawing.md#p-st_direction); Strict, Transitional; use=optional; default=horz.
- [ ] `@dir` — [p:ST_TransitionInOutDirectionType](values-drawing.md#p-st_transitioninoutdirectiontype); Strict, Transitional; use=optional; default=out.

<a id="p-ct_cornerdirectiontransition"></a>

## p:CT_CornerDirectionTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@dir` — [p:ST_TransitionCornerDirectionType](values-drawing.md#p-st_transitioncornerdirectiontype); Strict, Transitional; use=optional; default=lu.

<a id="p-ct_wheeltransition"></a>

## p:CT_WheelTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@spokes` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=4.

<a id="p-ct_inouttransition"></a>

## p:CT_InOutTransition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@dir` — [p:ST_TransitionInOutDirectionType](values-drawing.md#p-st_transitioninoutdirectiontype); Strict, Transitional; use=optional; default=out.

<a id="p-ct_transitionsoundaction"></a>

## p:CT_TransitionSoundAction

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `stSnd` — [p:CT_TransitionStartSoundAction](presentations.md#p-ct_transitionstartsoundaction); Strict, Transitional.
- [ ] `endSnd` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.

<a id="p-ct_timenodelist"></a>

## p:CT_TimeNodeList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `par` — [p:CT_TLTimeNodeParallel](presentations.md#p-ct_tltimenodeparallel); Strict, Transitional.
- [ ] `seq` — [p:CT_TLTimeNodeSequence](presentations.md#p-ct_tltimenodesequence); Strict, Transitional.
- [ ] `excl` — [p:CT_TLTimeNodeExclusive](presentations.md#p-ct_tltimenodeexclusive); Strict, Transitional.
- [ ] `anim` — [p:CT_TLAnimateBehavior](presentations.md#p-ct_tlanimatebehavior); Strict, Transitional.
- [ ] `animClr` — [p:CT_TLAnimateColorBehavior](presentations.md#p-ct_tlanimatecolorbehavior); Strict, Transitional.
- [ ] `animEffect` — [p:CT_TLAnimateEffectBehavior](presentations.md#p-ct_tlanimateeffectbehavior); Strict, Transitional.
- [ ] `animMotion` — [p:CT_TLAnimateMotionBehavior](presentations.md#p-ct_tlanimatemotionbehavior); Strict, Transitional.
- [ ] `animRot` — [p:CT_TLAnimateRotationBehavior](presentations.md#p-ct_tlanimaterotationbehavior); Strict, Transitional.
- [ ] `animScale` — [p:CT_TLAnimateScaleBehavior](presentations.md#p-ct_tlanimatescalebehavior); Strict, Transitional.
- [ ] `cmd` — [p:CT_TLCommandBehavior](presentations.md#p-ct_tlcommandbehavior); Strict, Transitional.
- [ ] `set` — [p:CT_TLSetBehavior](presentations.md#p-ct_tlsetbehavior); Strict, Transitional.
- [ ] `audio` — [p:CT_TLMediaNodeAudio](presentations.md#p-ct_tlmedianodeaudio); Strict, Transitional.
- [ ] `video` — [p:CT_TLMediaNodeVideo](presentations.md#p-ct_tlmedianodevideo); Strict, Transitional.

<a id="p-ct_buildlist"></a>

## p:CT_BuildList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `bldP` — [p:CT_TLBuildParagraph](presentations.md#p-ct_tlbuildparagraph); Strict, Transitional.
- [ ] `bldDgm` — [p:CT_TLBuildDiagram](presentations.md#p-ct_tlbuilddiagram); Strict, Transitional.
- [ ] `bldOleChart` — [p:CT_TLOleBuildChart](presentations.md#p-ct_tlolebuildchart); Strict, Transitional.
- [ ] `bldGraphic` — [p:CT_TLGraphicalObjectBuild](presentations.md#p-ct_tlgraphicalobjectbuild); Strict, Transitional.

<a id="p-ct_slidelayoutidlistentry"></a>

## p:CT_SlideLayoutIdListEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [p:ST_SlideLayoutId](values-drawing.md#p-st_slidelayoutid); Strict, Transitional; use=optional.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="a-ct_colorscheme"></a>

## a:CT_ColorScheme

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `dk1` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lt1` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `dk2` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lt2` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `accent1` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `accent2` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `accent3` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `accent4` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `accent5` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `accent6` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hlink` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `folHlink` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=required.

<a id="a-ct_stylematrix"></a>

## a:CT_StyleMatrix

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `fillStyleLst` — [a:CT_FillStyleList](presentations.md#a-ct_fillstylelist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `lnStyleLst` — [a:CT_LineStyleList](presentations.md#a-ct_linestylelist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectStyleLst` — [a:CT_EffectStyleList](presentations.md#a-ct_effectstylelist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `bgFillStyleLst` — [a:CT_BackgroundFillStyleList](presentations.md#a-ct_backgroundfillstylelist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=optional; default=.

<a id="a-ct_defaultshapedefinition"></a>

## a:CT_DefaultShapeDefinition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `bodyPr` — [a:CT_TextBodyProperties](drawing-text.md#a-ct_textbodyproperties); Strict, Transitional; minOccurs=1; maxOccurs=1. Body Properties; [§21.1.2.1.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3196-3200-0593.md#source-page-3197-part-1).
- [ ] `lstStyle` — [a:CT_TextListStyle](drawing-text.md#a-ct_textliststyle); Strict, Transitional; minOccurs=1; maxOccurs=1. Text List Styles; [§21.1.2.4.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-3273-3279-0607.md#source-page-3279-part-1).
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_colorschemeandmapping"></a>

## a:CT_ColorSchemeAndMapping

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `clrScheme` — [a:CT_ColorScheme](presentations.md#a-ct_colorscheme); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `clrMap` — [a:CT_ColorMapping](charts.md#a-ct_colormapping); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_customcolor"></a>

## a:CT_CustomColor

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=optional; default=.

<a id="p-ct_sliderelationshiplist"></a>

## p:CT_SlideRelationshipList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `sld` — [p:CT_SlideRelationshipListEntry](presentations.md#p-ct_sliderelationshiplistentry); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_backgroundproperties"></a>

## p:CT_BackgroundProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectLst` — [a:CT_EffectList](tables.md#a-ct_effectlist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectDag` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@shadeToTitle` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_groupshapenonvisual"></a>

## p:CT_GroupShapeNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvGrpSpPr` — [a:CT_NonVisualGroupDrawingShapeProps](charts.md#a-ct_nonvisualgroupdrawingshapeprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `nvPr` — [p:CT_ApplicationNonVisualDrawingProps](presentation-extensions.md#p-ct_applicationnonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="p-ct_graphicalobjectframe"></a>

## p:CT_GraphicalObjectFrame

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `nvGraphicFramePr` — [p:CT_GraphicalObjectFrameNonVisual](presentations.md#p-ct_graphicalobjectframenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `xfrm` — [a:CT_Transform2D](images.md#a-ct_transform2d); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `a:graphic` — [a:CT_GraphicalObject](images.md#a-ct_graphicalobject); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@bwMode` — [a:ST_BlackWhiteMode](values-drawing.md#a-st_blackwhitemode); Strict, Transitional; use=optional.

<a id="p-ct_picture"></a>

## p:CT_Picture

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `nvPicPr` — [p:CT_PictureNonVisual](presentations.md#p-ct_picturenonvisual); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `style` — [a:CT_ShapeStyle](shapes-and-lines.md#a-ct_shapestyle); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_rel"></a>

## p:CT_Rel

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_control"></a>

## p:CT_Control

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `pic` — [p:CT_Picture](presentations.md#p-ct_picture); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@spid` — [a:ST_ShapeID](values-drawing.md#a-st_shapeid); Transitional; use=optional.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=optional; default=.
- [ ] `@showAsIcon` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.
- [ ] `@imgW` — [a:ST_PositiveCoordinate32](values-drawing.md#a-st_positivecoordinate32); Strict, Transitional; use=optional.
- [ ] `@imgH` — [a:ST_PositiveCoordinate32](values-drawing.md#a-st_positivecoordinate32); Strict, Transitional; use=optional.

<a id="p-ct_transitionstartsoundaction"></a>

## p:CT_TransitionStartSoundAction

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `snd` — [a:CT_EmbeddedWAVAudioFile](links.md#a-ct_embeddedwavaudiofile); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@loop` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_tltimenodeparallel"></a>

## p:CT_TLTimeNodeParallel

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cTn` — [p:CT_TLCommonTimeNodeData](presentations.md#p-ct_tlcommontimenodedata); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="p-ct_tltimenodesequence"></a>

## p:CT_TLTimeNodeSequence

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cTn` — [p:CT_TLCommonTimeNodeData](presentations.md#p-ct_tlcommontimenodedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prevCondLst` — [p:CT_TLTimeConditionList](presentations.md#p-ct_tltimeconditionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `nextCondLst` — [p:CT_TLTimeConditionList](presentations.md#p-ct_tltimeconditionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@concurrent` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@prevAc` — [p:ST_TLPreviousActionType](values-drawing.md#p-st_tlpreviousactiontype); Strict, Transitional; use=optional.
- [ ] `@nextAc` — [p:ST_TLNextActionType](values-drawing.md#p-st_tlnextactiontype); Strict, Transitional; use=optional.

<a id="p-ct_tltimenodeexclusive"></a>

## p:CT_TLTimeNodeExclusive

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cTn` — [p:CT_TLCommonTimeNodeData](presentations.md#p-ct_tlcommontimenodedata); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="p-ct_tlanimatebehavior"></a>

## p:CT_TLAnimateBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tavLst` — [p:CT_TLTimeAnimateValueList](presentations.md#p-ct_tltimeanimatevaluelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@by` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@from` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@to` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@calcmode` — [p:ST_TLAnimateBehaviorCalcMode](values-drawing.md#p-st_tlanimatebehaviorcalcmode); Strict, Transitional; use=optional.
- [ ] `@valueType` — [p:ST_TLAnimateBehaviorValueType](values-drawing.md#p-st_tlanimatebehaviorvaluetype); Strict, Transitional; use=optional.

<a id="p-ct_tlanimatecolorbehavior"></a>

## p:CT_TLAnimateColorBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `by` — [p:CT_TLByAnimateColorTransform](presentations.md#p-ct_tlbyanimatecolortransform); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `from` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `to` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@clrSpc` — [p:ST_TLAnimateColorSpace](values-drawing.md#p-st_tlanimatecolorspace); Strict, Transitional; use=optional.
- [ ] `@dir` — [p:ST_TLAnimateColorDirection](values-drawing.md#p-st_tlanimatecolordirection); Strict, Transitional; use=optional.

<a id="p-ct_tlanimateeffectbehavior"></a>

## p:CT_TLAnimateEffectBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `progress` — [p:CT_TLAnimVariant](presentations.md#p-ct_tlanimvariant); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@transition` — [p:ST_TLAnimateEffectTransition](values-drawing.md#p-st_tlanimateeffecttransition); Strict, Transitional; default=in; use=optional.
- [ ] `@filter` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@prLst` — `xsd:string`; Strict, Transitional; use=optional.

<a id="p-ct_tlanimatemotionbehavior"></a>

## p:CT_TLAnimateMotionBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `by` — [p:CT_TLPoint](presentations.md#p-ct_tlpoint); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `from` — [p:CT_TLPoint](presentations.md#p-ct_tlpoint); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `to` — [p:CT_TLPoint](presentations.md#p-ct_tlpoint); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `rCtr` — [p:CT_TLPoint](presentations.md#p-ct_tlpoint); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@origin` — [p:ST_TLAnimateMotionBehaviorOrigin](values-drawing.md#p-st_tlanimatemotionbehaviororigin); Strict, Transitional; use=optional.
- [ ] `@path` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@pathEditMode` — [p:ST_TLAnimateMotionPathEditMode](values-drawing.md#p-st_tlanimatemotionpatheditmode); Strict, Transitional; use=optional.
- [ ] `@rAng` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=optional.
- [ ] `@ptsTypes` — `xsd:string`; Strict, Transitional; use=optional.

<a id="p-ct_tlanimaterotationbehavior"></a>

## p:CT_TLAnimateRotationBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@by` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=optional.
- [ ] `@from` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=optional.
- [ ] `@to` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=optional.

<a id="p-ct_tlanimatescalebehavior"></a>

## p:CT_TLAnimateScaleBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `by` — [p:CT_TLPoint](presentations.md#p-ct_tlpoint); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `from` — [p:CT_TLPoint](presentations.md#p-ct_tlpoint); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `to` — [p:CT_TLPoint](presentations.md#p-ct_tlpoint); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@zoomContents` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="p-ct_tlcommandbehavior"></a>

## p:CT_TLCommandBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@type` — [p:ST_TLCommandType](values-drawing.md#p-st_tlcommandtype); Strict, Transitional; use=optional.
- [ ] `@cmd` — `xsd:string`; Strict, Transitional; use=optional.

<a id="p-ct_tlsetbehavior"></a>

## p:CT_TLSetBehavior

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cBhvr` — [p:CT_TLCommonBehaviorData](presentations.md#p-ct_tlcommonbehaviordata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `to` — [p:CT_TLAnimVariant](presentations.md#p-ct_tlanimvariant); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_tlmedianodeaudio"></a>

## p:CT_TLMediaNodeAudio

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cMediaNode` — [p:CT_TLCommonMediaNodeData](presentations.md#p-ct_tlcommonmedianodedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@isNarration` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_tlmedianodevideo"></a>

## p:CT_TLMediaNodeVideo

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cMediaNode` — [p:CT_TLCommonMediaNodeData](presentations.md#p-ct_tlcommonmedianodedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@fullScrn` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_tlbuildparagraph"></a>

## p:CT_TLBuildParagraph

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `tmplLst` — [p:CT_TLTemplateList](presentations.md#p-ct_tltemplatelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@spid` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); Strict, Transitional; use=required.
- [ ] `@grpId` — `xsd:unsignedInt`; Strict, Transitional; use=required.
- [ ] `@uiExpand` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@build` — [p:ST_TLParaBuildType](values-drawing.md#p-st_tlparabuildtype); Strict, Transitional; use=optional; default=whole.
- [ ] `@bldLvl` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=1.
- [ ] `@animBg` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@autoUpdateAnimBg` — `xsd:boolean`; Strict, Transitional; default=true; use=optional.
- [ ] `@rev` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@advAuto` — [p:ST_TLTime](values-drawing.md#p-st_tltime); Strict, Transitional; use=optional; default=indefinite.

<a id="p-ct_tlbuilddiagram"></a>

## p:CT_TLBuildDiagram

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@spid` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); Strict, Transitional; use=required.
- [ ] `@grpId` — `xsd:unsignedInt`; Strict, Transitional; use=required.
- [ ] `@uiExpand` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@bld` — [p:ST_TLDiagramBuildType](values-drawing.md#p-st_tldiagrambuildtype); Strict, Transitional; use=optional; default=whole.

<a id="p-ct_tlolebuildchart"></a>

## p:CT_TLOleBuildChart

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@spid` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); Strict, Transitional; use=required.
- [ ] `@grpId` — `xsd:unsignedInt`; Strict, Transitional; use=required.
- [ ] `@uiExpand` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@bld` — [p:ST_TLOleChartBuildType](values-drawing.md#p-st_tlolechartbuildtype); Strict, Transitional; use=optional; default=allAtOnce.
- [ ] `@animBg` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="p-ct_tlgraphicalobjectbuild"></a>

## p:CT_TLGraphicalObjectBuild

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `bldAsOne` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `bldSub` — [a:CT_AnimationGraphicalObjectBuildProperties](presentations.md#a-ct_animationgraphicalobjectbuildproperties); Strict, Transitional.
- [ ] `@spid` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); Strict, Transitional; use=required.
- [ ] `@grpId` — `xsd:unsignedInt`; Strict, Transitional; use=required.
- [ ] `@uiExpand` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_fillstylelist"></a>

## a:CT_FillStyleList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a-ct_linestylelist"></a>

## a:CT_LineStyleList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `ln` — [a:CT_LineProperties](tables.md#a-ct_lineproperties); Strict, Transitional; minOccurs=3; maxOccurs=unbounded.

<a id="a-ct_effectstylelist"></a>

## a:CT_EffectStyleList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `effectStyle` — [a:CT_EffectStyleItem](presentations.md#a-ct_effectstyleitem); Strict, Transitional; minOccurs=3; maxOccurs=unbounded.

<a id="a-ct_backgroundfillstylelist"></a>

## a:CT_BackgroundFillStyleList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `noFill` — [a:CT_NoFillProperties](tables.md#a-ct_nofillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `solidFill` — [a:CT_SolidColorFillProperties](tables.md#a-ct_solidcolorfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `gradFill` — [a:CT_GradientFillProperties](tables.md#a-ct_gradientfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `pattFill` — [a:CT_PatternFillProperties](tables.md#a-ct_patternfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `grpFill` — [a:CT_GroupFillProperties](tables.md#a-ct_groupfillproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="p-ct_sliderelationshiplistentry"></a>

## p:CT_SlideRelationshipListEntry

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="p-ct_graphicalobjectframenonvisual"></a>

## p:CT_GraphicalObjectFrameNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvGraphicFramePr` — [a:CT_NonVisualGraphicFrameProperties](images.md#a-ct_nonvisualgraphicframeproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `nvPr` — [p:CT_ApplicationNonVisualDrawingProps](presentation-extensions.md#p-ct_applicationnonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="p-ct_picturenonvisual"></a>

## p:CT_PictureNonVisual

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `cNvPicPr` — [a:CT_NonVisualPictureProperties](images.md#a-ct_nonvisualpictureproperties); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `nvPr` — [p:CT_ApplicationNonVisualDrawingProps](presentation-extensions.md#p-ct_applicationnonvisualdrawingprops); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="p-ct_tlcommontimenodedata"></a>

## p:CT_TLCommonTimeNodeData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `stCondLst` — [p:CT_TLTimeConditionList](presentations.md#p-ct_tltimeconditionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `endCondLst` — [p:CT_TLTimeConditionList](presentations.md#p-ct_tltimeconditionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `endSync` — [p:CT_TLTimeCondition](presentations.md#p-ct_tltimecondition); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `iterate` — [p:CT_TLIterateData](presentations.md#p-ct_tliteratedata); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `childTnLst` — [p:CT_TimeNodeList](presentations.md#p-ct_timenodelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `subTnLst` — [p:CT_TimeNodeList](presentations.md#p-ct_timenodelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [p:ST_TLTimeNodeID](values-drawing.md#p-st_tltimenodeid); Strict, Transitional; use=optional.
- [ ] `@presetID` — `xsd:int`; Strict, Transitional; use=optional.
- [ ] `@presetClass` — [p:ST_TLTimeNodePresetClassType](values-drawing.md#p-st_tltimenodepresetclasstype); Strict, Transitional; use=optional.
- [ ] `@presetSubtype` — `xsd:int`; Strict, Transitional; use=optional.
- [ ] `@dur` — [p:ST_TLTime](values-drawing.md#p-st_tltime); Strict, Transitional; use=optional.
- [ ] `@repeatCount` — [p:ST_TLTime](values-drawing.md#p-st_tltime); Strict, Transitional; use=optional; default=1000.
- [ ] `@repeatDur` — [p:ST_TLTime](values-drawing.md#p-st_tltime); Strict, Transitional; use=optional.
- [ ] `@spd` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=optional; default=100%.
- [ ] `@accel` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=optional; default=0%.
- [ ] `@decel` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; use=optional; default=0%.
- [ ] `@autoRev` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@restart` — [p:ST_TLTimeNodeRestartType](values-drawing.md#p-st_tltimenoderestarttype); Strict, Transitional; use=optional.
- [ ] `@fill` — [p:ST_TLTimeNodeFillType](values-drawing.md#p-st_tltimenodefilltype); Strict, Transitional; use=optional.
- [ ] `@syncBehavior` — [p:ST_TLTimeNodeSyncType](values-drawing.md#p-st_tltimenodesynctype); Strict, Transitional; use=optional.
- [ ] `@tmFilter` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@evtFilter` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@display` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@masterRel` — [p:ST_TLTimeNodeMasterRelation](values-drawing.md#p-st_tltimenodemasterrelation); Strict, Transitional; use=optional.
- [ ] `@bldLvl` — `xsd:int`; Strict, Transitional; use=optional.
- [ ] `@grpId` — `xsd:unsignedInt`; Strict, Transitional; use=optional.
- [ ] `@afterEffect` — `xsd:boolean`; Strict, Transitional; use=optional.
- [ ] `@nodeType` — [p:ST_TLTimeNodeType](values-drawing.md#p-st_tltimenodetype); Strict, Transitional; use=optional.
- [ ] `@nodePh` — `xsd:boolean`; Strict, Transitional; use=optional.

<a id="p-ct_tltimeconditionlist"></a>

## p:CT_TLTimeConditionList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cond` — [p:CT_TLTimeCondition](presentations.md#p-ct_tltimecondition); Strict, Transitional; minOccurs=1; maxOccurs=unbounded.

<a id="p-ct_tlcommonbehaviordata"></a>

## p:CT_TLCommonBehaviorData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cTn` — [p:CT_TLCommonTimeNodeData](presentations.md#p-ct_tlcommontimenodedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tgtEl` — [p:CT_TLTimeTargetElement](presentations.md#p-ct_tltimetargetelement); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `attrNameLst` — [p:CT_TLBehaviorAttributeNameList](presentations.md#p-ct_tlbehaviorattributenamelist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@additive` — [p:ST_TLBehaviorAdditiveType](values-drawing.md#p-st_tlbehavioradditivetype); Strict, Transitional; use=optional.
- [ ] `@accumulate` — [p:ST_TLBehaviorAccumulateType](values-drawing.md#p-st_tlbehavioraccumulatetype); Strict, Transitional; use=optional.
- [ ] `@xfrmType` — [p:ST_TLBehaviorTransformType](values-drawing.md#p-st_tlbehaviortransformtype); Strict, Transitional; use=optional.
- [ ] `@from` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@to` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@by` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@rctx` — `xsd:string`; Strict, Transitional; use=optional.
- [ ] `@override` — [p:ST_TLBehaviorOverrideType](values-drawing.md#p-st_tlbehavioroverridetype); Strict, Transitional; use=optional.

<a id="p-ct_tltimeanimatevaluelist"></a>

## p:CT_TLTimeAnimateValueList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `tav` — [p:CT_TLTimeAnimateValue](presentations.md#p-ct_tltimeanimatevalue); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_tlbyanimatecolortransform"></a>

## p:CT_TLByAnimateColorTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `rgb` — [p:CT_TLByRgbColorTransform](presentations.md#p-ct_tlbyrgbcolortransform); Strict, Transitional.
- [ ] `hsl` — [p:CT_TLByHslColorTransform](presentations.md#p-ct_tlbyhslcolortransform); Strict, Transitional.

<a id="p-ct_tlanimvariant"></a>

## p:CT_TLAnimVariant

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `boolVal` — [p:CT_TLAnimVariantBooleanVal](presentations.md#p-ct_tlanimvariantbooleanval); Strict, Transitional.
- [ ] `intVal` — [p:CT_TLAnimVariantIntegerVal](presentations.md#p-ct_tlanimvariantintegerval); Strict, Transitional.
- [ ] `fltVal` — [p:CT_TLAnimVariantFloatVal](presentations.md#p-ct_tlanimvariantfloatval); Strict, Transitional.
- [ ] `strVal` — [p:CT_TLAnimVariantStringVal](presentations.md#p-ct_tlanimvariantstringval); Strict, Transitional.
- [ ] `clrVal` — [a:CT_Color](presentation-extensions.md#a-ct_color); Strict, Transitional.

<a id="p-ct_tlpoint"></a>

## p:CT_TLPoint

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@x` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.
- [ ] `@y` — [a:ST_Percentage](values-drawing.md#a-st_percentage); Strict, Transitional; use=required.

<a id="p-ct_tlcommonmedianodedata"></a>

## p:CT_TLCommonMediaNodeData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `cTn` — [p:CT_TLCommonTimeNodeData](presentations.md#p-ct_tlcommontimenodedata); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `tgtEl` — [p:CT_TLTimeTargetElement](presentations.md#p-ct_tltimetargetelement); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@vol` — [a:ST_PositiveFixedPercentage](values-drawing.md#a-st_positivefixedpercentage); Strict, Transitional; default=50%; use=optional.
- [ ] `@mute` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@numSld` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=1.
- [ ] `@showWhenStopped` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="p-ct_tltemplatelist"></a>

## p:CT_TLTemplateList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `tmpl` — [p:CT_TLTemplate](presentations.md#p-ct_tltemplate); Strict, Transitional; minOccurs=0; maxOccurs=9.

<a id="a-ct_animationgraphicalobjectbuildproperties"></a>

## a:CT_AnimationGraphicalObjectBuildProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `bldDgm` — [a:CT_AnimationDgmBuildProperties](presentations.md#a-ct_animationdgmbuildproperties); Strict, Transitional.
- [ ] `bldChart` — [a:CT_AnimationChartBuildProperties](presentations.md#a-ct_animationchartbuildproperties); Strict, Transitional.

<a id="a-ct_effectstyleitem"></a>

## a:CT_EffectStyleItem

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `effectLst` — [a:CT_EffectList](tables.md#a-ct_effectlist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `effectDag` — [a:CT_EffectContainer](tables.md#a-ct_effectcontainer); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `scene3d` — [a:CT_Scene3D](drawing-text.md#a-ct_scene3d); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `sp3d` — [a:CT_Shape3D](drawing-text.md#a-ct_shape3d); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="p-ct_tltimecondition"></a>

## p:CT_TLTimeCondition

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `tgtEl` — [p:CT_TLTimeTargetElement](presentations.md#p-ct_tltimetargetelement); Strict, Transitional.
- [ ] `tn` — [p:CT_TLTriggerTimeNodeID](presentations.md#p-ct_tltriggertimenodeid); Strict, Transitional.
- [ ] `rtn` — [p:CT_TLTriggerRuntimeNode](presentations.md#p-ct_tltriggerruntimenode); Strict, Transitional.
- [ ] `@evt` — [p:ST_TLTriggerEvent](values-drawing.md#p-st_tltriggerevent); Strict, Transitional; use=optional.
- [ ] `@delay` — [p:ST_TLTime](values-drawing.md#p-st_tltime); Strict, Transitional; use=optional.

<a id="p-ct_tliteratedata"></a>

## p:CT_TLIterateData

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `tmAbs` — [p:CT_TLIterateIntervalTime](presentations.md#p-ct_tliterateintervaltime); Strict, Transitional.
- [ ] `tmPct` — [p:CT_TLIterateIntervalPercentage](presentations.md#p-ct_tliterateintervalpercentage); Strict, Transitional.
- [ ] `@type` — [p:ST_IterateType](values-drawing.md#p-st_iteratetype); Strict, Transitional; use=optional; default=el.
- [ ] `@backwards` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="p-ct_tltimetargetelement"></a>

## p:CT_TLTimeTargetElement

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `sldTgt` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `sndTgt` — [a:CT_EmbeddedWAVAudioFile](links.md#a-ct_embeddedwavaudiofile); Strict, Transitional.
- [ ] `spTgt` — [p:CT_TLShapeTargetElement](presentations.md#p-ct_tlshapetargetelement); Strict, Transitional.
- [ ] `inkTgt` — [p:CT_TLSubShapeId](presentations.md#p-ct_tlsubshapeid); Strict, Transitional.

<a id="p-ct_tlbehaviorattributenamelist"></a>

## p:CT_TLBehaviorAttributeNameList

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `attrName` — `xsd:string`; Strict, Transitional; minOccurs=1; maxOccurs=unbounded.

<a id="p-ct_tltimeanimatevalue"></a>

## p:CT_TLTimeAnimateValue

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `val` — [p:CT_TLAnimVariant](presentations.md#p-ct_tlanimvariant); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@tm` — [p:ST_TLTimeAnimateValueTime](values-drawing.md#p-st_tltimeanimatevaluetime); Strict, Transitional; use=optional; default=indefinite.
- [ ] `@fmla` — `xsd:string`; Strict, Transitional; use=optional; default=.

<a id="p-ct_tlbyrgbcolortransform"></a>

## p:CT_TLByRgbColorTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@r` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=required.
- [ ] `@g` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=required.
- [ ] `@b` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=required.

<a id="p-ct_tlbyhslcolortransform"></a>

## p:CT_TLByHslColorTransform

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@h` — [a:ST_Angle](values-drawing.md#a-st_angle); Strict, Transitional; use=required.
- [ ] `@s` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=required.
- [ ] `@l` — [a:ST_FixedPercentage](values-drawing.md#a-st_fixedpercentage); Strict, Transitional; use=required.

<a id="p-ct_tlanimvariantbooleanval"></a>

## p:CT_TLAnimVariantBooleanVal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — `xsd:boolean`; Strict, Transitional; use=required.

<a id="p-ct_tlanimvariantintegerval"></a>

## p:CT_TLAnimVariantIntegerVal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — `xsd:int`; Strict, Transitional; use=required.

<a id="p-ct_tlanimvariantfloatval"></a>

## p:CT_TLAnimVariantFloatVal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — `xsd:float`; Strict, Transitional; use=required.

<a id="p-ct_tlanimvariantstringval"></a>

## p:CT_TLAnimVariantStringVal

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — `xsd:string`; Strict, Transitional; use=required.

<a id="p-ct_tltemplate"></a>

## p:CT_TLTemplate

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `tnLst` — [p:CT_TimeNodeList](presentations.md#p-ct_timenodelist); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `@lvl` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=0.

<a id="a-ct_animationdgmbuildproperties"></a>

## a:CT_AnimationDgmBuildProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@bld` — [a:ST_AnimationDgmBuildType](values-drawing.md#a-st_animationdgmbuildtype); Strict, Transitional; use=optional; default=allAtOnce.
- [ ] `@rev` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_animationchartbuildproperties"></a>

## a:CT_AnimationChartBuildProperties

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@bld` — [a:ST_AnimationChartBuildType](values-drawing.md#a-st_animationchartbuildtype); Strict, Transitional; use=optional; default=allAtOnce.
- [ ] `@animBg` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.

<a id="p-ct_tltriggertimenodeid"></a>

## p:CT_TLTriggerTimeNodeID

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — [p:ST_TLTimeNodeID](values-drawing.md#p-st_tltimenodeid); Strict, Transitional; use=required.

<a id="p-ct_tltriggerruntimenode"></a>

## p:CT_TLTriggerRuntimeNode

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — [p:ST_TLTriggerRuntimeNode](values-drawing.md#p-st_tltriggerruntimenode); Strict, Transitional; use=required.

<a id="p-ct_tliterateintervaltime"></a>

## p:CT_TLIterateIntervalTime

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — [p:ST_TLTime](values-drawing.md#p-st_tltime); Strict, Transitional; use=required.

<a id="p-ct_tliterateintervalpercentage"></a>

## p:CT_TLIterateIntervalPercentage

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@val` — [a:ST_PositivePercentage](values-drawing.md#a-st_positivepercentage); Strict, Transitional; use=required.

<a id="p-ct_tlshapetargetelement"></a>

## p:CT_TLShapeTargetElement

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `bg` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); Strict, Transitional.
- [ ] `subSp` — [p:CT_TLSubShapeId](presentations.md#p-ct_tlsubshapeid); Strict, Transitional.
- [ ] `oleChartEl` — [p:CT_TLOleChartTargetElement](presentations.md#p-ct_tlolecharttargetelement); Strict, Transitional.
- [ ] `txEl` — [p:CT_TLTextTargetElement](presentations.md#p-ct_tltexttargetelement); Strict, Transitional.
- [ ] `graphicEl` — [a:CT_AnimationElementChoice](presentations.md#a-ct_animationelementchoice); Strict, Transitional.
- [ ] `@spid` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); Strict, Transitional; use=required.

<a id="p-ct_tlsubshapeid"></a>

## p:CT_TLSubShapeId

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@spid` — [a:ST_ShapeID](values-drawing.md#a-st_shapeid); Strict, Transitional; use=required.

<a id="p-ct_tlolecharttargetelement"></a>

## p:CT_TLOleChartTargetElement

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@type` — [p:ST_TLChartSubelementType](values-drawing.md#p-st_tlchartsubelementtype); Strict, Transitional; use=required.
- [ ] `@lvl` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=0.

<a id="p-ct_tltexttargetelement"></a>

## p:CT_TLTextTargetElement

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `charRg` — [p:CT_IndexRange](presentations.md#p-ct_indexrange); Strict, Transitional.
- [ ] `pRg` — [p:CT_IndexRange](presentations.md#p-ct_indexrange); Strict, Transitional.

<a id="a-ct_animationelementchoice"></a>

## a:CT_AnimationElementChoice

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `dgm` — [a:CT_AnimationDgmElement](presentations.md#a-ct_animationdgmelement); Strict, Transitional.
- [ ] `chart` — [a:CT_AnimationChartElement](presentations.md#a-ct_animationchartelement); Strict, Transitional.

<a id="p-ct_indexrange"></a>

## p:CT_IndexRange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `@st` — [p:ST_Index](values-drawing.md#p-st_index); Strict, Transitional; use=required.
- [ ] `@end` — [p:ST_Index](values-drawing.md#p-st_index); Strict, Transitional; use=required.

<a id="a-ct_animationdgmelement"></a>

## a:CT_AnimationDgmElement

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); Strict, Transitional; use=optional; default={00000000-0000-0000-0000-000000000000}.
- [ ] `@bldStep` — [a:ST_DgmBuildStep](values-drawing.md#a-st_dgmbuildstep); Strict, Transitional; use=optional; default=sp.

<a id="a-ct_animationchartelement"></a>

## a:CT_AnimationChartElement

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@seriesIdx` — `xsd:int`; Strict, Transitional; use=optional; default=-1.
- [ ] `@categoryIdx` — `xsd:int`; Strict, Transitional; use=optional; default=-1.
- [ ] `@bldStep` — [a:ST_ChartBuildStep](values-drawing.md#a-st_chartbuildstep); Strict, Transitional; use=required.
