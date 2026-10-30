# Presentation extensions

[Inventory guide](README.md)

Complete retained Microsoft PresentationML extension property records. Includes unsupported collaboration, media, transitions, slide zoom, drawing commands and metadata. These inventory entries do not introduce authoring syntax.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [p2010:CT_LeftRightDirectionTransition](#p2010-ct_leftrightdirectiontransition)
- [p2010:CT_RippleTransition](#p2010-ct_rippletransition)
- [p2010:CT_PrismTransition](#p2010-ct_prismtransition)
- [p2010:CT_GlitterTransition](#p2010-ct_glittertransition)
- [p2010:CT_FlyThroughTransition](#p2010-ct_flythroughtransition)
- [p2010:CT_ShredTransition](#p2010-ct_shredtransition)
- [p2010:CT_RevealTransition](#p2010-ct_revealtransition)
- [p2010:CT_MediaBookmarkTarget](#p2010-ct_mediabookmarktarget)
- [p2010:CT_ContentPartNonVisual](#p2010-ct_contentpartnonvisual)
- [p2010:CT_MediaTrim](#p2010-ct_mediatrim)
- [p2010:CT_MediaFade](#p2010-ct_mediafade)
- [p2010:CT_MediaBookmark](#p2010-ct_mediabookmark)
- [p2010:CT_MediaBookmarkList](#p2010-ct_mediabookmarklist)
- [p2010:CT_Media](#p2010-ct_media)
- [p2010:CT_SectionSlideIdListEntry](#p2010-ct_sectionslideidlistentry)
- [p2010:CT_SectionSlideIdList](#p2010-ct_sectionslideidlist)
- [p2010:CT_Section](#p2010-ct_section)
- [p2010:CT_SectionList](#p2010-ct_sectionlist)
- [p2010:CT_BrowseMode](#p2010-ct_browsemode)
- [p2010:CT_DefaultImageDpi](#p2010-ct_defaultimagedpi)
- [p2010:CT_DiscardImageEditData](#p2010-ct_discardimageeditdata)
- [p2010:CT_ShowMediaControls](#p2010-ct_showmediacontrols)
- [p2010:CT_LaserTracePoint](#p2010-ct_lasertracepoint)
- [p2010:CT_LaserTrace](#p2010-ct_lasertrace)
- [p2010:CT_LaserTraceList](#p2010-ct_lasertracelist)
- [p2010:CT_RandomId](#p2010-ct_randomid)
- [p2010:CT_TriggerEventRecord](#p2010-ct_triggereventrecord)
- [p2010:CT_NullEventRecord](#p2010-ct_nulleventrecord)
- [p2010:CT_MediaPlaybackEventRecord](#p2010-ct_mediaplaybackeventrecord)
- [p2010:CT_MediaSeekEventRecord](#p2010-ct_mediaseekeventrecord)
- [p2010:CT_ShowEventRecordList](#p2010-ct_showeventrecordlist)
- [p2012:CT_PresetTransition](#p2012-ct_presettransition)
- [p2012:CT_PresenceInfo](#p2012-ct_presenceinfo)
- [p2012:CT_ParentCommentIdentifier](#p2012-ct_parentcommentidentifier)
- [p2012:CT_CommentThreading](#p2012-ct_commentthreading)
- [p2012:CT_ExtendedGuide](#p2012-ct_extendedguide)
- [p2012:CT_ExtendedGuideList](#p2012-ct_extendedguidelist)
- [p2012:CT_ChartTrackingRefBased](#p2012-ct_charttrackingrefbased)
- [p2012:CT_IsNarration](#p2012-ct_isnarration)
- [p2013__command:CT_DocumentMoniker](#p2013__command-ct_documentmoniker)
- [p2013__command:CT_SlideMoniker](#p2013__command-ct_slidemoniker)
- [p2013__command:CT_MainMasterMoniker](#p2013__command-ct_mainmastermoniker)
- [p2013__command:CT_SlideLayoutMoniker](#p2013__command-ct_slidelayoutmoniker)
- [p2013__command:CT_CommentMoniker](#p2013__command-ct_commentmoniker)
- [p2013__command:CT_CommentAuthorMoniker](#p2013__command-ct_commentauthormoniker)
- [p2013__command:CT_DocumentMonikerList](#p2013__command-ct_documentmonikerlist)
- [p2013__command:CT_SlideBaseMonikerList](#p2013__command-ct_slidebasemonikerlist)
- [p2013__command:CT_MainMasterMonikerList](#p2013__command-ct_mainmastermonikerlist)
- [p2013__command:CT_SlideLayoutMonikerList](#p2013__command-ct_slidelayoutmonikerlist)
- [p2013__command:CT_SlideMonikerList](#p2013__command-ct_slidemonikerlist)
- [p2013__command:CT_CommentMonikerList](#p2013__command-ct_commentmonikerlist)
- [p2013__command:CT_CommentAuthorMonikerList](#p2013__command-ct_commentauthormonikerlist)
- [p2013__command:CT_CommentChanges](#p2013__command-ct_commentchanges)
- [p2013__command:CT_CommentAuthorChanges](#p2013__command-ct_commentauthorchanges)
- [p2013__command:CT_SlideChanges](#p2013__command-ct_slidechanges)
- [p2013__command:CT_SlideLayoutChanges](#p2013__command-ct_slidelayoutchanges)
- [p2013__command:CT_MainMasterChanges](#p2013__command-ct_mainmasterchanges)
- [p2013__command:CT_DocumentChanges](#p2013__command-ct_documentchanges)
- [p2013__command:CT_DocumentChangesList](#p2013__command-ct_documentchangeslist)
- [p2013__command:CT_ChangesInfo](#p2013__command-ct_changesinfo)
- [p2015_09:CT_MorphTransition](#p2015_09-ct_morphtransition)
- [p2015_10:CT_ClientRevision](#p2015_10-ct_clientrevision)
- [p2015_10:CT_ClientRevisionList](#p2015_10-ct_clientrevisionlist)
- [p2015_10:CT_RevisionInfo](#p2015_10-ct_revisioninfo)
- [p2015:CT_DesignElement](#p2015-ct_designelement)
- [p2016_6:CT_ZoomObjectProperties](#p2016_6-ct_zoomobjectproperties)
- [p2016_sectionzoom:CT_SectionZoomObject](#p2016_sectionzoom-ct_sectionzoomobject)
- [p2016_sectionzoom:CT_SectionZoom](#p2016_sectionzoom-ct_sectionzoom)
- [p2016_slidezoom:CT_SlideZoomObject](#p2016_slidezoom-ct_slidezoomobject)
- [p2016_slidezoom:CT_SlideZoom](#p2016_slidezoom-ct_slidezoom)
- [p2016_summaryzoom:CT_SummaryZoomObject](#p2016_summaryzoom-ct_summaryzoomobject)
- [p2016_summaryzoom:CT_GridLayout](#p2016_summaryzoom-ct_gridlayout)
- [p2016_summaryzoom:CT_FixedLayout](#p2016_summaryzoom-ct_fixedlayout)
- [p2016_summaryzoom:CT_SummaryZoom](#p2016_summaryzoom-ct_summaryzoom)
- [p2017_10:CT_ReadonlyRecommended](#p2017_10-ct_readonlyrecommended)
- [p2017_3:CT_Track](#p2017_3-ct_track)
- [p2017_3:CT_TrackList](#p2017_3-ct_tracklist)
- [p2017_3:CT_TracksInfo](#p2017_3-ct_tracksinfo)
- [p2018_4:CT_ClassificationOutcome](#p2018_4-ct_classificationoutcome)
- [p2018_8:CT_Author](#p2018_8-ct_author)
- [p2018_8:CT_AuthorList](#p2018_8-ct_authorlist)
- [p2018_8:CT_CommentReply](#p2018_8-ct_commentreply)
- [p2018_8:CT_CommentReplyList](#p2018_8-ct_commentreplylist)
- [p2018_8:CT_CommentUnknownAnchor](#p2018_8-ct_commentunknownanchor)
- [p2018_8:CT_Comment](#p2018_8-ct_comment)
- [p2018_8:CT_CommentList](#p2018_8-ct_commentlist)
- [p2018_8:CT_CommentRelationship](#p2018_8-ct_commentrelationship)
- [p2019_9__command:CT_CommentV2MonikerList](#p2019_9__command-ct_commentv2monikerlist)
- [p2019_9__command:CT_CommentReplyV2MonikerList](#p2019_9__command-ct_commentreplyv2monikerlist)
- [p2019_9__command:CT_CommentV2Moniker](#p2019_9__command-ct_commentv2moniker)
- [p2019_9__command:CT_CommentReplyV2Moniker](#p2019_9__command-ct_commentreplyv2moniker)
- [p2020_02:CT_DesignerTag](#p2020_02-ct_designertag)
- [p2020_02:CT_DesignerTagList](#p2020_02-ct_designertaglist)
- [p2020_02:CT_DesignerDrawingProps](#p2020_02-ct_designerdrawingprops)
- [p2022_03:CT_ReactionInstance](#p2022_03-ct_reactioninstance)
- [p2022_03:CT_Reaction](#p2022_03-ct_reaction)
- [p2022_03:CT_Reactions](#p2022_03-ct_reactions)
- [p2022_06__command:CT_CommentReplyV2Changes](#p2022_06__command-ct_commentreplyv2changes)
- [p2022_06__command:CT_CommentV2Changes](#p2022_06__command-ct_commentv2changes)
- [p2022_08:CT_TaskScheduleEventInfo](#p2022_08-ct_taskscheduleeventinfo)
- [p2022_08:CT_TaskProgressEventInfo](#p2022_08-ct_taskprogresseventinfo)
- [p2022_08:CT_TaskTitleEventInfo](#p2022_08-ct_tasktitleeventinfo)
- [p2022_08:CT_TaskAssignUnassignUser](#p2022_08-ct_taskassignunassignuser)
- [p2022_08:CT_TaskUnknownRecord](#p2022_08-ct_taskunknownrecord)
- [p2022_08:CT_CommentAnchor](#p2022_08-ct_commentanchor)
- [p2022_08:CT_TaskAnchor](#p2022_08-ct_taskanchor)
- [p2022_08:CT_TaskUndo](#p2022_08-ct_taskundo)
- [p2022_08:CT_TaskHistoryEvent](#p2022_08-ct_taskhistoryevent)
- [p2022_08:CT_TaskHistory](#p2022_08-ct_taskhistory)
- [p2022_08:CT_TaskDetails](#p2022_08-ct_taskdetails)
- [p2023_02:CT_PlaceholderTypeACB](#p2023_02-ct_placeholdertypeacb)
- [p2023_02:CT_PlaceholderTypeExtension](#p2023_02-ct_placeholdertypeextension)
- [a14:CT_NonVisualInkContentPartProperties](#a14-ct_nonvisualinkcontentpartproperties)
- [p:CT_ApplicationNonVisualDrawingProps](#p-ct_applicationnonvisualdrawingprops)
- [a:CT_Color](#a-ct_color)
- [a2013__command:CT_ChangesData](#a2013__command-ct_changesdata)
- [a2013__command:CT_ShapeChanges](#a2013__command-ct_shapechanges)
- [a2013__command:CT_GroupShapeChanges](#a2013__command-ct_groupshapechanges)
- [a2013__command:CT_GraphicFrameChanges](#a2013__command-ct_graphicframechanges)
- [a2013__command:CT_PictureChanges](#a2013__command-ct_picturechanges)
- [a2013__command:CT_InkChanges](#a2013__command-ct_inkchanges)
- [a2013__command:CT_ConnectorChanges](#a2013__command-ct_connectorchanges)
- [a2013__command:CT_DrawingElementMonikerList](#a2013__command-ct_drawingelementmonikerlist)
- [a2013__command:CT_TextCharRangeMonikerList](#a2013__command-ct_textcharrangemonikerlist)
- [p:CT_Empty](#p-ct_empty)
- [a14:CT_ContentPartLocking](#a14-ct_contentpartlocking)
- [p:CT_Placeholder](#p-ct_placeholder)
- [a:CT_AudioCD](#a-ct_audiocd)
- [a:CT_AudioFile](#a-ct_audiofile)
- [a:CT_VideoFile](#a-ct_videofile)
- [a:CT_QuickTimeFile](#a-ct_quicktimefile)
- [a2013__command:CT_ShapeMonikerList](#a2013__command-ct_shapemonikerlist)
- [a2013__command:CT_GroupShapeMonikerList](#a2013__command-ct_groupshapemonikerlist)
- [a2013__command:CT_GraphicFrameMonikerList](#a2013__command-ct_graphicframemonikerlist)
- [a2013__command:CT_PictureMonikerList](#a2013__command-ct_picturemonikerlist)
- [a2013__command:CT_InkMonikerList](#a2013__command-ct_inkmonikerlist)
- [a2013__command:CT_ConnectorMonikerList](#a2013__command-ct_connectormonikerlist)
- [a:CT_AudioCDTime](#a-ct_audiocdtime)

<a id="p2010-ct_leftrightdirectiontransition"></a>

## p2010:CT_LeftRightDirectionTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@dir` — [p2010:ST_TransitionLeftRightDirectionType](values-extensions.md#p2010-st_transitionleftrightdirectiontype); MS-PPTX.

<a id="p2010-ct_rippletransition"></a>

## p2010:CT_RippleTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@dir` — [p2010:ST_TransitionCornerAndCenterDirectionType](values-extensions.md#p2010-st_transitioncornerandcenterdirectiontype); MS-PPTX; use=optional; default=center.

<a id="p2010-ct_prismtransition"></a>

## p2010:CT_PrismTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@dir` — [p:ST_TransitionSideDirectionType](values-drawing.md#p-st_transitionsidedirectiontype); MS-PPTX; use=optional; default=l.
- [ ] `@isContent` — `xsd:boolean`; MS-PPTX; use=optional; default=false.
- [ ] `@isInverted` — `xsd:boolean`; MS-PPTX; use=optional; default=false.

<a id="p2010-ct_glittertransition"></a>

## p2010:CT_GlitterTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@dir` — [p:ST_TransitionSideDirectionType](values-drawing.md#p-st_transitionsidedirectiontype); MS-PPTX; use=optional; default=l.
- [ ] `@pattern` — [p2010:ST_TransitionPattern](values-extensions.md#p2010-st_transitionpattern); MS-PPTX; use=optional; default=diamond.

<a id="p2010-ct_flythroughtransition"></a>

## p2010:CT_FlyThroughTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@dir` — [p:ST_TransitionInOutDirectionType](values-drawing.md#p-st_transitioninoutdirectiontype); MS-PPTX; use=optional; default=in.
- [ ] `@hasBounce` — `xsd:boolean`; MS-PPTX; use=optional; default=false.

<a id="p2010-ct_shredtransition"></a>

## p2010:CT_ShredTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@pattern` — [p2010:ST_TransitionShredPattern](values-extensions.md#p2010-st_transitionshredpattern); MS-PPTX; use=optional; default=strip.
- [ ] `@dir` — [p:ST_TransitionInOutDirectionType](values-drawing.md#p-st_transitioninoutdirectiontype); MS-PPTX; use=optional; default=in.

<a id="p2010-ct_revealtransition"></a>

## p2010:CT_RevealTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@thruBlk` — `xsd:boolean`; MS-PPTX; use=optional; default=false.
- [ ] `@dir` — [p2010:ST_TransitionLeftRightDirectionType](values-extensions.md#p2010-st_transitionleftrightdirectiontype); MS-PPTX; use=optional; default=l.

<a id="p2010-ct_mediabookmarktarget"></a>

## p2010:CT_MediaBookmarkTarget

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@spid` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); MS-PPTX; use=required.
- [ ] `@bmkName` — `xsd:string`; MS-PPTX; use=required.

<a id="p2010-ct_contentpartnonvisual"></a>

## p2010:CT_ContentPartNonVisual

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `cNvPr` — [a:CT_NonVisualDrawingProps](images.md#a-ct_nonvisualdrawingprops); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `cNvContentPartPr` — [a14:CT_NonVisualInkContentPartProperties](presentation-extensions.md#a14-ct_nonvisualinkcontentpartproperties); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `nvPr` — [p:CT_ApplicationNonVisualDrawingProps](presentation-extensions.md#p-ct_applicationnonvisualdrawingprops); MS-PPTX; minOccurs=1; maxOccurs=1.

<a id="p2010-ct_mediatrim"></a>

## p2010:CT_MediaTrim

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@st` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=optional; default=0.
- [ ] `@end` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=optional; default=0.

<a id="p2010-ct_mediafade"></a>

## p2010:CT_MediaFade

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@in` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=optional; default=0.
- [ ] `@out` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=optional; default=0.

<a id="p2010-ct_mediabookmark"></a>

## p2010:CT_MediaBookmark

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@name` — `xsd:string`; MS-PPTX.
- [ ] `@time` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX.

<a id="p2010-ct_mediabookmarklist"></a>

## p2010:CT_MediaBookmarkList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `bmk` — [p2010:CT_MediaBookmark](presentation-extensions.md#p2010-ct_mediabookmark); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2010-ct_media"></a>

## p2010:CT_Media

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `trim` — [p2010:CT_MediaTrim](presentation-extensions.md#p2010-ct_mediatrim); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `fade` — [p2010:CT_MediaFade](presentation-extensions.md#p2010-ct_mediafade); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `bmkLst` — [p2010:CT_MediaBookmarkList](presentation-extensions.md#p2010-ct_mediabookmarklist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@r:embed` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-PPTX; use=optional; default=.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-PPTX; use=optional; default=.

<a id="p2010-ct_sectionslideidlistentry"></a>

## p2010:CT_SectionSlideIdListEntry

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — [p:ST_SlideId](values-drawing.md#p-st_slideid); MS-PPTX; use=required.

<a id="p2010-ct_sectionslideidlist"></a>

## p2010:CT_SectionSlideIdList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `sldId` — [p2010:CT_SectionSlideIdListEntry](presentation-extensions.md#p2010-ct_sectionslideidlistentry); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2010-ct_section"></a>

## p2010:CT_Section

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `sldIdLst` — [p2010:CT_SectionSlideIdList](presentation-extensions.md#p2010-ct_sectionslideidlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@name` — `xsd:string`; MS-PPTX.
- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX.

<a id="p2010-ct_sectionlist"></a>

## p2010:CT_SectionList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `section` — [p2010:CT_Section](presentation-extensions.md#p2010-ct_section); MS-PPTX; minOccurs=1; maxOccurs=unbounded.

<a id="p2010-ct_browsemode"></a>

## p2010:CT_BrowseMode

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@showStatus` — `xsd:boolean`; MS-PPTX; use=optional; default=true.

<a id="p2010-ct_defaultimagedpi"></a>

## p2010:CT_DefaultImageDpi

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:unsignedInt`; MS-PPTX; use=required.

<a id="p2010-ct_discardimageeditdata"></a>

## p2010:CT_DiscardImageEditData

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:boolean`; MS-PPTX; use=required.

<a id="p2010-ct_showmediacontrols"></a>

## p2010:CT_ShowMediaControls

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:boolean`; MS-PPTX; use=required.

<a id="p2010-ct_lasertracepoint"></a>

## p2010:CT_LaserTracePoint

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@t` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=required.
- [ ] `@x` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); MS-PPTX; use=required.
- [ ] `@y` — [a:ST_Coordinate](values-drawing.md#a-st_coordinate); MS-PPTX; use=required.

<a id="p2010-ct_lasertrace"></a>

## p2010:CT_LaserTrace

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `tracePt` — [p2010:CT_LaserTracePoint](presentation-extensions.md#p2010-ct_lasertracepoint); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2010-ct_lasertracelist"></a>

## p2010:CT_LaserTraceList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `tracePtLst` — [p2010:CT_LaserTrace](presentation-extensions.md#p2010-ct_lasertrace); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2010-ct_randomid"></a>

## p2010:CT_RandomId

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:unsignedInt`; MS-PPTX; use=required.

<a id="p2010-ct_triggereventrecord"></a>

## p2010:CT_TriggerEventRecord

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@type` — [p:ST_TLTriggerEvent](values-drawing.md#p-st_tltriggerevent); MS-PPTX; use=required.
- [ ] `@time` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=required.
- [ ] `@objId` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); MS-PPTX; use=required.

<a id="p2010-ct_nulleventrecord"></a>

## p2010:CT_NullEventRecord

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@time` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=required.
- [ ] `@objId` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); MS-PPTX; use=required.

<a id="p2010-ct_mediaplaybackeventrecord"></a>

## p2010:CT_MediaPlaybackEventRecord

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@time` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=required.
- [ ] `@objId` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); MS-PPTX; use=required.

<a id="p2010-ct_mediaseekeventrecord"></a>

## p2010:CT_MediaSeekEventRecord

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@time` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=required.
- [ ] `@objId` — [a:ST_DrawingElementId](values-drawing.md#a-st_drawingelementid); MS-PPTX; use=required.
- [ ] `@seek` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=required.

<a id="p2010-ct_showeventrecordlist"></a>

## p2010:CT_ShowEventRecordList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `triggerEvt` — [p2010:CT_TriggerEventRecord](presentation-extensions.md#p2010-ct_triggereventrecord); MS-PPTX.
- [ ] `playEvt` — [p2010:CT_MediaPlaybackEventRecord](presentation-extensions.md#p2010-ct_mediaplaybackeventrecord); MS-PPTX.
- [ ] `stopEvt` — [p2010:CT_MediaPlaybackEventRecord](presentation-extensions.md#p2010-ct_mediaplaybackeventrecord); MS-PPTX.
- [ ] `pauseEvt` — [p2010:CT_MediaPlaybackEventRecord](presentation-extensions.md#p2010-ct_mediaplaybackeventrecord); MS-PPTX.
- [ ] `resumeEvt` — [p2010:CT_MediaPlaybackEventRecord](presentation-extensions.md#p2010-ct_mediaplaybackeventrecord); MS-PPTX.
- [ ] `seekEvt` — [p2010:CT_MediaSeekEventRecord](presentation-extensions.md#p2010-ct_mediaseekeventrecord); MS-PPTX.
- [ ] `nullEvt` — [p2010:CT_NullEventRecord](presentation-extensions.md#p2010-ct_nulleventrecord); MS-PPTX.

<a id="p2012-ct_presettransition"></a>

## p2012:CT_PresetTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@prst` — `xsd:string`; MS-PPTX.
- [ ] `@invX` — `xsd:boolean`; MS-PPTX; use=optional; default=false.
- [ ] `@invY` — `xsd:boolean`; MS-PPTX; use=optional; default=false.

<a id="p2012-ct_presenceinfo"></a>

## p2012:CT_PresenceInfo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@userId` — `xsd:string`; MS-PPTX; use=required.
- [ ] `@providerId` — `xsd:string`; MS-PPTX; use=required.

<a id="p2012-ct_parentcommentidentifier"></a>

## p2012:CT_ParentCommentIdentifier

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@authorId` — `xsd:unsignedInt`; MS-PPTX.
- [ ] `@idx` — `xsd:unsignedInt`; MS-PPTX.

<a id="p2012-ct_commentthreading"></a>

## p2012:CT_CommentThreading

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `parentCm` — [p2012:CT_ParentCommentIdentifier](presentation-extensions.md#p2012-ct_parentcommentidentifier); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@timeZoneBias` — `xsd:int`; MS-PPTX.

<a id="p2012-ct_extendedguide"></a>

## p2012:CT_ExtendedGuide

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `clr` — [a:CT_Color](presentation-extensions.md#a-ct_color); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@id` — `xsd:unsignedInt`; MS-PPTX; use=required.
- [ ] `@name` — `xsd:string`; MS-PPTX; use=optional; default=.
- [ ] `@orient` — [p:ST_Direction](values-drawing.md#p-st_direction); MS-PPTX; use=optional; default=vert.
- [ ] `@pos` — [a:ST_Coordinate32](values-drawing.md#a-st_coordinate32); MS-PPTX; use=optional; default=0.
- [ ] `@userDrawn` — `xsd:boolean`; MS-PPTX; use=optional; default=false.

<a id="p2012-ct_extendedguidelist"></a>

## p2012:CT_ExtendedGuideList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `guide` — [p2012:CT_ExtendedGuide](presentation-extensions.md#p2012-ct_extendedguide); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2012-ct_charttrackingrefbased"></a>

## p2012:CT_ChartTrackingRefBased

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:boolean`; MS-PPTX; use=required.

<a id="p2012-ct_isnarration"></a>

## p2012:CT_IsNarration

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:boolean`; MS-PPTX; use=optional; default=false.

<a id="p2013__command-ct_documentmoniker"></a>

## p2013__command:CT_DocumentMoniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

Empty marker type; no attributes or child properties.

<a id="p2013__command-ct_slidemoniker"></a>

## p2013__command:CT_SlideMoniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@cId` — [p2013__command:ST_CreationId](values-extensions.md#p2013__command-st_creationid); MS-PPTX; use=optional.
- [ ] `@sldId` — [p:ST_SlideId](values-drawing.md#p-st_slideid); MS-PPTX; use=required.

<a id="p2013__command-ct_mainmastermoniker"></a>

## p2013__command:CT_MainMasterMoniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@cId` — [p2013__command:ST_CreationId](values-extensions.md#p2013__command-st_creationid); MS-PPTX; use=optional.
- [ ] `@sldId` — [p:ST_SlideMasterId](values-drawing.md#p-st_slidemasterid); MS-PPTX; use=required.

<a id="p2013__command-ct_slidelayoutmoniker"></a>

## p2013__command:CT_SlideLayoutMoniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@cId` — [p2013__command:ST_CreationId](values-extensions.md#p2013__command-st_creationid); MS-PPTX; use=optional.
- [ ] `@sldId` — [p:ST_SlideLayoutId](values-drawing.md#p-st_slidelayoutid); MS-PPTX; use=required.

<a id="p2013__command-ct_commentmoniker"></a>

## p2013__command:CT_CommentMoniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@authorId` — `xsd:unsignedInt`; MS-PPTX; use=required.
- [ ] `@idx` — [p:ST_Index](values-drawing.md#p-st_index); MS-PPTX; use=required.

<a id="p2013__command-ct_commentauthormoniker"></a>

## p2013__command:CT_CommentAuthorMoniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — `xsd:unsignedInt`; MS-PPTX; use=required.

<a id="p2013__command-ct_documentmonikerlist"></a>

## p2013__command:CT_DocumentMonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2013__command-ct_slidebasemonikerlist"></a>

## p2013__command:CT_SlideBaseMonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2013__command-ct_mainmastermonikerlist"></a>

## p2013__command:CT_MainMasterMonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2013__command-ct_slidelayoutmonikerlist"></a>

## p2013__command:CT_SlideLayoutMonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2013__command-ct_slidemonikerlist"></a>

## p2013__command:CT_SlideMonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2013__command-ct_commentmonikerlist"></a>

## p2013__command:CT_CommentMonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2013__command-ct_commentauthormonikerlist"></a>

## p2013__command:CT_CommentAuthorMonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2013__command-ct_commentchanges"></a>

## p2013__command:CT_CommentChanges

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `cmMkLst` — [p2013__command:CT_CommentMonikerList](presentation-extensions.md#p2013__command-ct_commentmonikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2013__command:ST_CommentChangeBits](values-extensions.md#p2013__command-st_commentchangebits); MS-PPTX; use=required.

<a id="p2013__command-ct_commentauthorchanges"></a>

## p2013__command:CT_CommentAuthorChanges

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `cmAuthorMkLst` — [p2013__command:CT_CommentAuthorMonikerList](presentation-extensions.md#p2013__command-ct_commentauthormonikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2013__command:ST_CommentAuthorChangeBits](values-extensions.md#p2013__command-st_commentauthorchangebits); MS-PPTX; use=required.

<a id="p2013__command-ct_slidechanges"></a>

## p2013__command:CT_SlideChanges

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `sldMkLst` — [p2013__command:CT_SlideMonikerList](presentation-extensions.md#p2013__command-ct_slidemonikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `spChg` — [a2013__command:CT_ShapeChanges](presentation-extensions.md#a2013__command-ct_shapechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `grpChg` — [a2013__command:CT_GroupShapeChanges](presentation-extensions.md#a2013__command-ct_groupshapechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `graphicFrameChg` — [a2013__command:CT_GraphicFrameChanges](presentation-extensions.md#a2013__command-ct_graphicframechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `picChg` — [a2013__command:CT_PictureChanges](presentation-extensions.md#a2013__command-ct_picturechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `inkChg` — [a2013__command:CT_InkChanges](presentation-extensions.md#a2013__command-ct_inkchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `cxnChg` — [a2013__command:CT_ConnectorChanges](presentation-extensions.md#a2013__command-ct_connectorchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `cmChg` — [p2013__command:CT_CommentChanges](presentation-extensions.md#p2013__command-ct_commentchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2013__command:ST_SlideChangeBits](values-extensions.md#p2013__command-st_slidechangebits); MS-PPTX; use=required.

<a id="p2013__command-ct_slidelayoutchanges"></a>

## p2013__command:CT_SlideLayoutChanges

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `sldLayoutMkLst` — [p2013__command:CT_SlideLayoutMonikerList](presentation-extensions.md#p2013__command-ct_slidelayoutmonikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `spChg` — [a2013__command:CT_ShapeChanges](presentation-extensions.md#a2013__command-ct_shapechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `grpChg` — [a2013__command:CT_GroupShapeChanges](presentation-extensions.md#a2013__command-ct_groupshapechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `graphicFrameChg` — [a2013__command:CT_GraphicFrameChanges](presentation-extensions.md#a2013__command-ct_graphicframechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `picChg` — [a2013__command:CT_PictureChanges](presentation-extensions.md#a2013__command-ct_picturechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `inkChg` — [a2013__command:CT_InkChanges](presentation-extensions.md#a2013__command-ct_inkchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `cxnChg` — [a2013__command:CT_ConnectorChanges](presentation-extensions.md#a2013__command-ct_connectorchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2013__command:ST_SlideLayoutChangeBits](values-extensions.md#p2013__command-st_slidelayoutchangebits); MS-PPTX; use=required.

<a id="p2013__command-ct_mainmasterchanges"></a>

## p2013__command:CT_MainMasterChanges

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `sldMasterMkLst` — [p2013__command:CT_MainMasterMonikerList](presentation-extensions.md#p2013__command-ct_mainmastermonikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `spChg` — [a2013__command:CT_ShapeChanges](presentation-extensions.md#a2013__command-ct_shapechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `grpChg` — [a2013__command:CT_GroupShapeChanges](presentation-extensions.md#a2013__command-ct_groupshapechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `graphicFrameChg` — [a2013__command:CT_GraphicFrameChanges](presentation-extensions.md#a2013__command-ct_graphicframechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `picChg` — [a2013__command:CT_PictureChanges](presentation-extensions.md#a2013__command-ct_picturechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `inkChg` — [a2013__command:CT_InkChanges](presentation-extensions.md#a2013__command-ct_inkchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `cxnChg` — [a2013__command:CT_ConnectorChanges](presentation-extensions.md#a2013__command-ct_connectorchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `sldLayoutChg` — [p2013__command:CT_SlideLayoutChanges](presentation-extensions.md#p2013__command-ct_slidelayoutchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2013__command:ST_MainMasterChangeBits](values-extensions.md#p2013__command-st_mainmasterchangebits); MS-PPTX; use=required.

<a id="p2013__command-ct_documentchanges"></a>

## p2013__command:CT_DocumentChanges

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `docMkLst` — [p2013__command:CT_DocumentMonikerList](presentation-extensions.md#p2013__command-ct_documentmonikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `sldChg` — [p2013__command:CT_SlideChanges](presentation-extensions.md#p2013__command-ct_slidechanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `sldMasterChg` — [p2013__command:CT_MainMasterChanges](presentation-extensions.md#p2013__command-ct_mainmasterchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `cmAuthorChg` — [p2013__command:CT_CommentAuthorChanges](presentation-extensions.md#p2013__command-ct_commentauthorchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2013__command:ST_DocumentChangeBits](values-extensions.md#p2013__command-st_documentchangebits); MS-PPTX; use=required.

<a id="p2013__command-ct_documentchangeslist"></a>

## p2013__command:CT_DocumentChangesList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `docChg` — [p2013__command:CT_DocumentChanges](presentation-extensions.md#p2013__command-ct_documentchanges); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2013__command-ct_changesinfo"></a>

## p2013__command:CT_ChangesInfo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `docChgLst` — [p2013__command:CT_DocumentChangesList](presentation-extensions.md#p2013__command-ct_documentchangeslist); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2015_09-ct_morphtransition"></a>

## p2015_09:CT_MorphTransition

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@option` — [p2015_09:ST_TransitionMorphOption](values-extensions.md#p2015_09-st_transitionmorphoption); MS-PPTX; use=required.

<a id="p2015_10-ct_clientrevision"></a>

## p2015_10:CT_ClientRevision

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — [p2015_10:ST_ClientID](values-extensions.md#p2015_10-st_clientid); MS-PPTX; use=required.
- [ ] `@v` — [p2015_10:ST_ClientRevisionNumber](values-extensions.md#p2015_10-st_clientrevisionnumber); MS-PPTX; use=optional; default=0.
- [ ] `@vWet` — [p2015_10:ST_ClientRevisionNumber](values-extensions.md#p2015_10-st_clientrevisionnumber); MS-PPTX; use=optional; default=0.
- [ ] `@dt` — `xsd:dateTime`; MS-PPTX; use=required.

<a id="p2015_10-ct_clientrevisionlist"></a>

## p2015_10:CT_ClientRevisionList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `client` — [p2015_10:CT_ClientRevision](presentation-extensions.md#p2015_10-ct_clientrevision); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2015_10-ct_revisioninfo"></a>

## p2015_10:CT_RevisionInfo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `revLst` — [p2015_10:CT_ClientRevisionList](presentation-extensions.md#p2015_10-ct_clientrevisionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2015-ct_designelement"></a>

## p2015:CT_DesignElement

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:boolean`; MS-PPTX.

<a id="p2016_6-ct_zoomobjectproperties"></a>

## p2016_6:CT_ZoomObjectProperties

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `blipFill` — [a:CT_BlipFillProperties](images.md#a-ct_blipfillproperties); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `spPr` — [a:CT_ShapeProperties](images.md#a-ct_shapeproperties); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.
- [ ] `@returnToParent` — `xsd:boolean`; MS-PPTX; use=optional; default=true.
- [ ] `@imageType` — [p2016_6:ST_ZoomObjectImageType](values-extensions.md#p2016_6-st_zoomobjectimagetype); MS-PPTX; use=optional; default=preview.
- [ ] `@transitionDur` — [p2010:ST_UniversalTimeOffset](values-extensions.md#p2010-st_universaltimeoffset); MS-PPTX; use=optional.
- [ ] `@showBg` — `xsd:boolean`; MS-PPTX; use=optional; default=true.

<a id="p2016_sectionzoom-ct_sectionzoomobject"></a>

## p2016_sectionzoom:CT_SectionZoomObject

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `zmPr` — [p2016_6:CT_ZoomObjectProperties](presentation-extensions.md#p2016_6-ct_zoomobjectproperties); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@sectionId` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.

<a id="p2016_sectionzoom-ct_sectionzoom"></a>

## p2016_sectionzoom:CT_SectionZoom

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `sectionZmObj` — [p2016_sectionzoom:CT_SectionZoomObject](presentation-extensions.md#p2016_sectionzoom-ct_sectionzoomobject); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2016_slidezoom-ct_slidezoomobject"></a>

## p2016_slidezoom:CT_SlideZoomObject

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `zmPr` — [p2016_6:CT_ZoomObjectProperties](presentation-extensions.md#p2016_6-ct_zoomobjectproperties); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@sldId` — [p:ST_SlideId](values-drawing.md#p-st_slideid); MS-PPTX; use=required.
- [ ] `@cId` — `xsd:unsignedInt`; MS-PPTX; use=optional.

<a id="p2016_slidezoom-ct_slidezoom"></a>

## p2016_slidezoom:CT_SlideZoom

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `sldZmObj` — [p2016_slidezoom:CT_SlideZoomObject](presentation-extensions.md#p2016_slidezoom-ct_slidezoomobject); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2016_summaryzoom-ct_summaryzoomobject"></a>

## p2016_summaryzoom:CT_SummaryZoomObject

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `zmPr` — [p2016_6:CT_ZoomObjectProperties](presentation-extensions.md#p2016_6-ct_zoomobjectproperties); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@sectionId` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.
- [ ] `@title` — `xsd:string`; MS-PPTX; use=optional; default=.
- [ ] `@descr` — `xsd:string`; MS-PPTX; use=optional; default=.
- [ ] `@offsetFactorX` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-PPTX; use=optional; default=0.
- [ ] `@offsetFactorY` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-PPTX; use=optional; default=0.
- [ ] `@scaleFactorX` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-PPTX; use=optional; default=100000.
- [ ] `@scaleFactorY` — [a:ST_Percentage](values-drawing.md#a-st_percentage); MS-PPTX; use=optional; default=100000.

<a id="p2016_summaryzoom-ct_gridlayout"></a>

## p2016_summaryzoom:CT_GridLayout

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

Empty marker type; no attributes or child properties.

<a id="p2016_summaryzoom-ct_fixedlayout"></a>

## p2016_summaryzoom:CT_FixedLayout

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

Empty marker type; no attributes or child properties.

<a id="p2016_summaryzoom-ct_summaryzoom"></a>

## p2016_summaryzoom:CT_SummaryZoom

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `summaryZmObj` — [p2016_summaryzoom:CT_SummaryZoomObject](presentation-extensions.md#p2016_summaryzoom-ct_summaryzoomobject); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `gridLayout` — [p2016_summaryzoom:CT_GridLayout](presentation-extensions.md#p2016_summaryzoom-ct_gridlayout); MS-PPTX.
- [ ] `fixedLayout` — [p2016_summaryzoom:CT_FixedLayout](presentation-extensions.md#p2016_summaryzoom-ct_fixedlayout); MS-PPTX.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2017_10-ct_readonlyrecommended"></a>

## p2017_10:CT_ReadonlyRecommended

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:boolean`; MS-PPTX; use=required.

<a id="p2017_3-ct_track"></a>

## p2017_3:CT_Track

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.
- [ ] `@label` — `xsd:string`; MS-PPTX; use=required.
- [ ] `@lang` — [a:ST_TextLanguageID](values-drawing.md#a-st_textlanguageid); MS-PPTX; use=optional.
- [ ] `@r:embed` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-PPTX; use=optional; default=.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-PPTX; use=optional; default=.

<a id="p2017_3-ct_tracklist"></a>

## p2017_3:CT_TrackList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `track` — [p2017_3:CT_Track](presentation-extensions.md#p2017_3-ct_track); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2017_3-ct_tracksinfo"></a>

## p2017_3:CT_TracksInfo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `trackLst` — [p2017_3:CT_TrackList](presentation-extensions.md#p2017_3-ct_tracklist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@displayLoc` — [p2017_3:ST_DisplayLocation](values-extensions.md#p2017_3-st_displaylocation); MS-PPTX; use=required.

<a id="p2018_4-ct_classificationoutcome"></a>

## p2018_4:CT_ClassificationOutcome

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — [p2018_4:ST_ClassificationOutcomeType](values-extensions.md#p2018_4-st_classificationoutcometype); MS-PPTX.

<a id="p2018_8-ct_author"></a>

## p2018_8:CT_Author

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [p2018_8:ST_AuthorId](values-extensions.md#p2018_8-st_authorid); MS-PPTX; use=required.
- [ ] `@name` — `xsd:string`; MS-PPTX; use=required.
- [ ] `@initials` — `xsd:string`; MS-PPTX; use=optional.
- [ ] `@userId` — `xsd:string`; MS-PPTX; use=required.
- [ ] `@providerId` — `xsd:string`; MS-PPTX; use=required.

<a id="p2018_8-ct_authorlist"></a>

## p2018_8:CT_AuthorList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `author` — [p2018_8:CT_Author](presentation-extensions.md#p2018_8-ct_author); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2018_8-ct_commentreply"></a>

## p2018_8:CT_CommentReply

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `txBody` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [p2018_8:ST_CommentId](values-extensions.md#p2018_8-st_commentid); MS-PPTX; use=required.
- [ ] `@authorId` — [p2018_8:ST_AuthorId](values-extensions.md#p2018_8-st_authorid); MS-PPTX; use=required.
- [ ] `@status` — [p2018_8:ST_CommentStatus](values-extensions.md#p2018_8-st_commentstatus); MS-PPTX; use=optional; default=active.
- [ ] `@created` — `xsd:dateTime`; MS-PPTX; use=required.

<a id="p2018_8-ct_commentreplylist"></a>

## p2018_8:CT_CommentReplyList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `reply` — [p2018_8:CT_CommentReply](presentation-extensions.md#p2018_8-ct_commentreply); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2018_8-ct_commentunknownanchor"></a>

## p2018_8:CT_CommentUnknownAnchor

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

Empty marker type; no attributes or child properties.

<a id="p2018_8-ct_comment"></a>

## p2018_8:CT_Comment

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `p2013__command:sldMkLst` — [p2013__command:CT_SlideMonikerList](presentation-extensions.md#p2013__command-ct_slidemonikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `a2013__command:deMkLst` — [a2013__command:CT_DrawingElementMonikerList](presentation-extensions.md#a2013__command-ct_drawingelementmonikerlist); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `a2013__command:txMkLst` — [a2013__command:CT_TextCharRangeMonikerList](presentation-extensions.md#a2013__command-ct_textcharrangemonikerlist); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `unknownAnchor` — [p2018_8:CT_CommentUnknownAnchor](presentation-extensions.md#p2018_8-ct_commentunknownanchor); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `pos` — [a:CT_Point2D](images.md#a-ct_point2d); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `replyLst` — [p2018_8:CT_CommentReplyList](presentation-extensions.md#p2018_8-ct_commentreplylist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `txBody` — [a:CT_TextBody](drawing-text.md#a-ct_textbody); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@id` — [p2018_8:ST_CommentId](values-extensions.md#p2018_8-st_commentid); MS-PPTX; use=required.
- [ ] `@authorId` — [p2018_8:ST_AuthorId](values-extensions.md#p2018_8-st_authorid); MS-PPTX; use=required.
- [ ] `@status` — [p2018_8:ST_CommentStatus](values-extensions.md#p2018_8-st_commentstatus); MS-PPTX; use=optional; default=active.
- [ ] `@created` — `xsd:dateTime`; MS-PPTX; use=required.
- [ ] `@startDate` — `xsd:dateTime`; MS-PPTX; use=optional.
- [ ] `@dueDate` — `xsd:dateTime`; MS-PPTX; use=optional.
- [ ] `@assignedTo` — [p2018_8:ST_AuthorIdList](values-extensions.md#p2018_8-st_authoridlist); MS-PPTX; use=optional; default=.
- [ ] `@complete` — [s:ST_PositiveFixedPercentage](values-shared.md#s-st_positivefixedpercentage); MS-PPTX; default=0%; use=optional.
- [ ] `@title` — `xsd:string`; MS-PPTX; use=optional; default=.

<a id="p2018_8-ct_commentlist"></a>

## p2018_8:CT_CommentList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `cm` — [p2018_8:CT_Comment](presentation-extensions.md#p2018_8-ct_comment); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2018_8-ct_commentrelationship"></a>

## p2018_8:CT_CommentRelationship

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); MS-PPTX; use=required.

<a id="p2019_9__command-ct_commentv2monikerlist"></a>

## p2019_9__command:CT_CommentV2MonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2019_9__command-ct_commentreplyv2monikerlist"></a>

## p2019_9__command:CT_CommentReplyV2MonikerList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `*` — extension wildcard; MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2019_9__command-ct_commentv2moniker"></a>

## p2019_9__command:CT_CommentV2Moniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.

<a id="p2019_9__command-ct_commentreplyv2moniker"></a>

## p2019_9__command:CT_CommentReplyV2Moniker

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.

<a id="p2020_02-ct_designertag"></a>

## p2020_02:CT_DesignerTag

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@name` — `xsd:string`; MS-PPTX; use=required.
- [ ] `@val` — `xsd:string`; MS-PPTX; use=required.

<a id="p2020_02-ct_designertaglist"></a>

## p2020_02:CT_DesignerTagList

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `designTag` — [p2020_02:CT_DesignerTag](presentation-extensions.md#p2020_02-ct_designertag); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2020_02-ct_designerdrawingprops"></a>

## p2020_02:CT_DesignerDrawingProps

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `p2020_02:designTagLst` — [p2020_02:CT_DesignerTagList](presentation-extensions.md#p2020_02-ct_designertaglist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@edtDesignElem` — `xsd:boolean`; MS-PPTX; use=optional; default=false.

<a id="p2022_03-ct_reactioninstance"></a>

## p2022_03:CT_ReactionInstance

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@time` — `xsd:dateTime`; MS-PPTX; use=required.
- [ ] `@authorId` — [p2018_8:ST_AuthorId](values-extensions.md#p2018_8-st_authorid); MS-PPTX; use=required.

<a id="p2022_03-ct_reaction"></a>

## p2022_03:CT_Reaction

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `instance` — [p2022_03:CT_ReactionInstance](presentation-extensions.md#p2022_03-ct_reactioninstance); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `@type` — `xsd:string`; MS-PPTX; use=required.

<a id="p2022_03-ct_reactions"></a>

## p2022_03:CT_Reactions

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `rxn` — [p2022_03:CT_Reaction](presentation-extensions.md#p2022_03-ct_reaction); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2022_06__command-ct_commentreplyv2changes"></a>

## p2022_06__command:CT_CommentReplyV2Changes

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `p2019_9__command:cmRplyMkLst` — [p2019_9__command:CT_CommentReplyV2MonikerList](presentation-extensions.md#p2019_9__command-ct_commentreplyv2monikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2022_06__command:ST_CommentReplyV2ChangeBits](values-extensions.md#p2022_06__command-st_commentreplyv2changebits); MS-PPTX; use=required.

<a id="p2022_06__command-ct_commentv2changes"></a>

## p2022_06__command:CT_CommentV2Changes

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `p2019_9__command:cmMkLst` — [p2019_9__command:CT_CommentV2MonikerList](presentation-extensions.md#p2019_9__command-ct_commentv2monikerlist); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `cmRplyChg` — [p2022_06__command:CT_CommentReplyV2Changes](presentation-extensions.md#p2022_06__command-ct_commentreplyv2changes); MS-PPTX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [p2022_06__command:ST_CommentV2ChangeBits](values-extensions.md#p2022_06__command-st_commentv2changebits); MS-PPTX; use=required.

<a id="p2022_08-ct_taskscheduleeventinfo"></a>

## p2022_08:CT_TaskScheduleEventInfo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@stDt` — `xsd:dateTime`; MS-PPTX; use=optional.
- [ ] `@endDt` — `xsd:dateTime`; MS-PPTX; use=optional.

<a id="p2022_08-ct_taskprogresseventinfo"></a>

## p2022_08:CT_TaskProgressEventInfo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — [s:ST_PositiveFixedPercentage](values-shared.md#s-st_positivefixedpercentage); MS-PPTX; use=required.

<a id="p2022_08-ct_tasktitleeventinfo"></a>

## p2022_08:CT_TaskTitleEventInfo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@val` — `xsd:string`; MS-PPTX; use=required.

<a id="p2022_08-ct_taskassignunassignuser"></a>

## p2022_08:CT_TaskAssignUnassignUser

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@authorId` — [p2018_8:ST_AuthorId](values-extensions.md#p2018_8-st_authorid); MS-PPTX; use=required.

<a id="p2022_08-ct_taskunknownrecord"></a>

## p2022_08:CT_TaskUnknownRecord

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

Empty marker type; no attributes or child properties.

<a id="p2022_08-ct_commentanchor"></a>

## p2022_08:CT_CommentAnchor

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.

<a id="p2022_08-ct_taskanchor"></a>

## p2022_08:CT_TaskAnchor

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `comment` — [p2022_08:CT_CommentAnchor](presentation-extensions.md#p2022_08-ct_commentanchor); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2022_08-ct_taskundo"></a>

## p2022_08:CT_TaskUndo

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.

<a id="p2022_08-ct_taskhistoryevent"></a>

## p2022_08:CT_TaskHistoryEvent

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `atrbtn` — [p2022_08:CT_TaskAssignUnassignUser](presentation-extensions.md#p2022_08-ct_taskassignunassignuser); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `anchr` — [p2022_08:CT_TaskAnchor](presentation-extensions.md#p2022_08-ct_taskanchor); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `asgn` — [p2022_08:CT_TaskAssignUnassignUser](presentation-extensions.md#p2022_08-ct_taskassignunassignuser); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `add` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `title` — [p2022_08:CT_TaskTitleEventInfo](presentation-extensions.md#p2022_08-ct_tasktitleeventinfo); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `date` — [p2022_08:CT_TaskScheduleEventInfo](presentation-extensions.md#p2022_08-ct_taskscheduleeventinfo); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `pcntCmplt` — [p2022_08:CT_TaskProgressEventInfo](presentation-extensions.md#p2022_08-ct_taskprogresseventinfo); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `unasgnAll` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `undo` — [p2022_08:CT_TaskUndo](presentation-extensions.md#p2022_08-ct_taskundo); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `unknown` — [p2022_08:CT_TaskUnknownRecord](presentation-extensions.md#p2022_08-ct_taskunknownrecord); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.
- [ ] `@time` — `xsd:dateTime`; MS-PPTX; use=required.
- [ ] `@id` — [s:ST_Guid](values-shared.md#s-st_guid); MS-PPTX; use=required.

<a id="p2022_08-ct_taskhistory"></a>

## p2022_08:CT_TaskHistory

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `event` — [p2022_08:CT_TaskHistoryEvent](presentation-extensions.md#p2022_08-ct_taskhistoryevent); MS-PPTX; minOccurs=0; maxOccurs=unbounded.

<a id="p2022_08-ct_taskdetails"></a>

## p2022_08:CT_TaskDetails

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `history` — [p2022_08:CT_TaskHistory](presentation-extensions.md#p2022_08-ct_taskhistory); MS-PPTX; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); MS-PPTX; minOccurs=0; maxOccurs=1.

<a id="p2023_02-ct_placeholdertypeacb"></a>

## p2023_02:CT_PlaceholderTypeACB

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `cameo` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); MS-PPTX.
- [ ] `unknown` — [p:CT_Empty](presentation-extensions.md#p-ct_empty); MS-PPTX.

<a id="p2023_02-ct_placeholdertypeextension"></a>

## p2023_02:CT_PlaceholderTypeExtension

Sources: [MS-PPTX](../../reference/reading/ooxml/microsoft/MS-PPTX/README.md).

- [ ] `type` — [p2023_02:CT_PlaceholderTypeACB](presentation-extensions.md#p2023_02-ct_placeholdertypeacb); MS-PPTX; minOccurs=1; maxOccurs=1.

<a id="a14-ct_nonvisualinkcontentpartproperties"></a>

## a14:CT_NonVisualInkContentPartProperties

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `cpLocks` — [a14:CT_ContentPartLocking](presentation-extensions.md#a14-ct_contentpartlocking); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@isComment` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=true.

<a id="p-ct_applicationnonvisualdrawingprops"></a>

## p:CT_ApplicationNonVisualDrawingProps

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `ph` — [p:CT_Placeholder](presentation-extensions.md#p-ct_placeholder); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `audioCd` — [a:CT_AudioCD](presentation-extensions.md#a-ct_audiocd); Strict, Transitional.
- [ ] `wavAudioFile` — [a:CT_EmbeddedWAVAudioFile](links.md#a-ct_embeddedwavaudiofile); Strict, Transitional.
- [ ] `audioFile` — [a:CT_AudioFile](presentation-extensions.md#a-ct_audiofile); Strict, Transitional.
- [ ] `videoFile` — [a:CT_VideoFile](presentation-extensions.md#a-ct_videofile); Strict, Transitional.
- [ ] `quickTimeFile` — [a:CT_QuickTimeFile](presentation-extensions.md#a-ct_quicktimefile); Strict, Transitional.
- [ ] `custDataLst` — [p:CT_CustomerDataList](presentations.md#p-ct_customerdatalist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [p:CT_ExtensionList](drawing-text.md#p-ct_extensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@isPhoto` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@userDrawn` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_color"></a>

## a:CT_Color

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `scrgbClr` — [a:CT_ScRgbColor](tables.md#a-ct_scrgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `srgbClr` — [a:CT_SRgbColor](tables.md#a-ct_srgbcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `hslClr` — [a:CT_HslColor](tables.md#a-ct_hslcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `sysClr` — [a:CT_SystemColor](tables.md#a-ct_systemcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `schemeClr` — [a:CT_SchemeColor](tables.md#a-ct_schemecolor); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `prstClr` — [a:CT_PresetColor](tables.md#a-ct_presetcolor); Strict, Transitional; minOccurs=1; maxOccurs=1.

<a id="a2013__command-ct_changesdata"></a>

## a2013__command:CT_ChangesData

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@name` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@userId` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@providerId` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@clId` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@email` — `xsd:string`; MS-ODRAWXML; use=optional.
- [ ] `@dt` — `xsd:dateTime`; MS-ODRAWXML; use=optional.
- [ ] `@v` — `xsd:unsignedInt`; MS-ODRAWXML; use=optional.
- [ ] `@id` — [a:ST_Guid](values-drawing.md#a-st_guid); MS-ODRAWXML; use=optional.
- [ ] `@actId` — `xsd:int`; MS-ODRAWXML; use=optional.

<a id="a2013__command-ct_shapechanges"></a>

## a2013__command:CT_ShapeChanges

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `spMkLst` — [a2013__command:CT_ShapeMonikerList](presentation-extensions.md#a2013__command-ct_shapemonikerlist); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [a2013__command:ST_ShapeChangesBits](values-extensions.md#a2013__command-st_shapechangesbits); MS-ODRAWXML; use=required.

<a id="a2013__command-ct_groupshapechanges"></a>

## a2013__command:CT_GroupShapeChanges

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `grpSpMkLst` — [a2013__command:CT_GroupShapeMonikerList](presentation-extensions.md#a2013__command-ct_groupshapemonikerlist); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [a2013__command:ST_GroupShapeChangesBits](values-extensions.md#a2013__command-st_groupshapechangesbits); MS-ODRAWXML; use=required.

<a id="a2013__command-ct_graphicframechanges"></a>

## a2013__command:CT_GraphicFrameChanges

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `graphicFrameMkLst` — [a2013__command:CT_GraphicFrameMonikerList](presentation-extensions.md#a2013__command-ct_graphicframemonikerlist); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [a2013__command:ST_GraphicFrameChangesBits](values-extensions.md#a2013__command-st_graphicframechangesbits); MS-ODRAWXML; use=required.

<a id="a2013__command-ct_picturechanges"></a>

## a2013__command:CT_PictureChanges

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `picMkLst` — [a2013__command:CT_PictureMonikerList](presentation-extensions.md#a2013__command-ct_picturemonikerlist); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [a2013__command:ST_PictureChangesBits](values-extensions.md#a2013__command-st_picturechangesbits); MS-ODRAWXML; use=required.

<a id="a2013__command-ct_inkchanges"></a>

## a2013__command:CT_InkChanges

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `inkMkLst` — [a2013__command:CT_InkMonikerList](presentation-extensions.md#a2013__command-ct_inkmonikerlist); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [a2013__command:ST_InkChangesBits](values-extensions.md#a2013__command-st_inkchangesbits); MS-ODRAWXML; use=required.

<a id="a2013__command-ct_connectorchanges"></a>

## a2013__command:CT_ConnectorChanges

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `chgData` — [a2013__command:CT_ChangesData](presentation-extensions.md#a2013__command-ct_changesdata); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `cxnSpMkLst` — [a2013__command:CT_ConnectorMonikerList](presentation-extensions.md#a2013__command-ct_connectormonikerlist); MS-ODRAWXML; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@chg` — [a2013__command:ST_ConnectorChangesBits](values-extensions.md#a2013__command-st_connectorchangesbits); MS-ODRAWXML; use=required.

<a id="a2013__command-ct_drawingelementmonikerlist"></a>

## a2013__command:CT_DrawingElementMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="a2013__command-ct_textcharrangemonikerlist"></a>

## a2013__command:CT_TextCharRangeMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="p-ct_empty"></a>

## p:CT_Empty

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

Empty marker type; no attributes or child properties.

<a id="a14-ct_contentpartlocking"></a>

## a14:CT_ContentPartLocking

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); MS-ODRAWXML; minOccurs=0; maxOccurs=1.
- [ ] `@noGrp` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noSelect` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noRot` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noChangeAspect` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noMove` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noResize` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noEditPoints` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noAdjustHandles` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noChangeArrowheads` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.
- [ ] `@noChangeShapeType` — `xsd:boolean`; MS-ODRAWXML; use=optional; default=false.

<a id="p-ct_placeholder"></a>

## p:CT_Placeholder

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/pml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/pml.xsd).

- [ ] `extLst` — [p:CT_ExtensionListModify](shapes-and-lines.md#p-ct_extensionlistmodify); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@type` — [p:ST_PlaceholderType](values-drawing.md#p-st_placeholdertype); Strict, Transitional; use=optional; default=obj.
- [ ] `@orient` — [p:ST_Direction](values-drawing.md#p-st_direction); Strict, Transitional; use=optional; default=horz.
- [ ] `@sz` — [p:ST_PlaceholderSize](values-drawing.md#p-st_placeholdersize); Strict, Transitional; use=optional; default=full.
- [ ] `@idx` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=0.
- [ ] `@hasCustomPrompt` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_audiocd"></a>

## a:CT_AudioCD

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `st` — [a:CT_AudioCDTime](presentation-extensions.md#a-ct_audiocdtime); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `end` — [a:CT_AudioCDTime](presentation-extensions.md#a-ct_audiocdtime); Strict, Transitional; minOccurs=1; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.

<a id="a-ct_audiofile"></a>

## a:CT_AudioFile

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
- [ ] `@contentType` — `xsd:string`; Strict, Transitional; use=optional.

<a id="a-ct_videofile"></a>

## a:CT_VideoFile

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
- [ ] `@contentType` — `xsd:string`; Strict, Transitional; use=optional.

<a id="a-ct_quicktimefile"></a>

## a:CT_QuickTimeFile

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:link` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.

<a id="a2013__command-ct_shapemonikerlist"></a>

## a2013__command:CT_ShapeMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="a2013__command-ct_groupshapemonikerlist"></a>

## a2013__command:CT_GroupShapeMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="a2013__command-ct_graphicframemonikerlist"></a>

## a2013__command:CT_GraphicFrameMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="a2013__command-ct_picturemonikerlist"></a>

## a2013__command:CT_PictureMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="a2013__command-ct_inkmonikerlist"></a>

## a2013__command:CT_InkMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="a2013__command-ct_connectormonikerlist"></a>

## a2013__command:CT_ConnectorMonikerList

Sources: [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md).

- [ ] `*` — extension wildcard; MS-ODRAWXML; minOccurs=0; maxOccurs=unbounded.

<a id="a-ct_audiocdtime"></a>

## a:CT_AudioCDTime

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@track` — `xsd:unsignedByte`; Strict, Transitional; use=required.
- [ ] `@time` — `xsd:unsignedInt`; Strict, Transitional; use=optional; default=0.
