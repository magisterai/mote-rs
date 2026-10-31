# Comment properties

[Inventory guide](README.md)

Comment identities, authorship, ranges and extension records. Story content uses the existing paragraph, run and table inventories; thread resolution, durable identities, people and modern extensions remain visible without implying Mote authoring support. Comment-range start/end use [w:CT_MarkupRange](anchors-and-fields.md#w-ct_markuprange); comment references use [w:CT_Markup](#w-ct_markup). Presentation comment records and authors are inventoried in [PresentationML](presentations.md) and its [extensions](presentation-extensions.md).

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_Comments](#w-ct_comments)
- [w:CT_Comment](#w-ct_comment)
- [w:CT_Markup](#w-ct_markup)
- [w15:CT_CommentsEx](#w15-ct_commentsex)
- [w15:CT_People](#w15-ct_people)
- [w16cid:CT_CommentsIds](#w16cid-ct_commentsids)
- [w16cex:CT_CommentsExtensible](#w16cex-ct_commentsextensible)
- [w2026cei:CT_CommentEntityInfo](#w2026cei-ct_commententityinfo)
- [w15:CT_CommentEx](#w15-ct_commentex)
- [w15:CT_Person](#w15-ct_person)
- [w16cid:CT_CommentId](#w16cid-ct_commentid)
- [w16cex:CT_CommentExtensible](#w16cex-ct_commentextensible)
- [w16:CT_ExtensionList](#w16-ct_extensionlist)
- [w15:CT_PresenceInfo](#w15-ct_presenceinfo)
- [w16:CT_Extension](#w16-ct_extension)

<a id="w-ct_comments"></a>

## w:CT_Comments

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `comment` — [w:CT_Comment](comments.md#w-ct_comment); Strict, Transitional; minOccurs=0; maxOccurs=unbounded.

<a id="w-ct_comment"></a>

## w:CT_Comment

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required. Authored id is metadata; package ids are allocated in reference order. Evidence: [annotations.md](../../../docs/specification/annotations.md), [annotations.rs](../../../tests/annotations.rs).
- [x] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required. Comment author, defaulting to Mote in DOCX. Evidence: [annotations.md](../../../docs/specification/annotations.md), [annotations.rs](../../../tests/annotations.rs).
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional. ISO date/typed date maps to midnight UTC; arbitrary date-time authoring is unsupported. Evidence: [annotations.md](../../../docs/specification/annotations.md), [annotations.rs](../../../tests/annotations.rs).
- [x] `@initials` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional. Optional comment initials in DOCX. Evidence: [annotations.md](../../../docs/specification/annotations.md), [annotations.rs](../../../tests/annotations.rs).

<a id="w-ct_markup"></a>

## w:CT_Markup

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.

<a id="w15-ct_commentsex"></a>

## w15:CT_CommentsEx

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `commentEx` — [w15:CT_CommentEx](comments.md#w15-ct_commentex); MS-DOCX; minOccurs=0; maxOccurs=unbounded.

<a id="w15-ct_people"></a>

## w15:CT_People

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `person` — [w15:CT_Person](comments.md#w15-ct_person); MS-DOCX; minOccurs=0; maxOccurs=unbounded.

<a id="w16cid-ct_commentsids"></a>

## w16cid:CT_CommentsIds

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `commentId` — [w16cid:CT_CommentId](comments.md#w16cid-ct_commentid); MS-DOCX; minOccurs=0; maxOccurs=unbounded.

<a id="w16cex-ct_commentsextensible"></a>

## w16cex:CT_CommentsExtensible

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `commentExtensible` — [w16cex:CT_CommentExtensible](comments.md#w16cex-ct_commentextensible); MS-DOCX; minOccurs=0; maxOccurs=unbounded.
- [ ] `extLst` — [w16:CT_ExtensionList](comments.md#w16-ct_extensionlist); MS-DOCX; minOccurs=0; maxOccurs=1.

<a id="w2026cei-ct_commententityinfo"></a>

## w2026cei:CT_CommentEntityInfo

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `extLst` — [w16:CT_ExtensionList](comments.md#w16-ct_extensionlist); MS-DOCX; minOccurs=0; maxOccurs=1.
- [ ] `@entityType` — `xsd:unsignedInt`; MS-DOCX; use=optional; default=0.

<a id="w15-ct_commentex"></a>

## w15:CT_CommentEx

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@paraId` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX; use=required.
- [ ] `@paraIdParent` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX; use=optional.
- [ ] `@done` — [w:ST_OnOff](values-word.md#w-st_onoff); MS-DOCX; use=optional.

<a id="w15-ct_person"></a>

## w15:CT_Person

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `presenceInfo` — [w15:CT_PresenceInfo](comments.md#w15-ct_presenceinfo); MS-DOCX; minOccurs=0; maxOccurs=1.
- [ ] `@author` — [w:ST_String](values-word.md#w-st_string); MS-DOCX; use=required.

<a id="w16cid-ct_commentid"></a>

## w16cid:CT_CommentId

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@paraId` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX; use=required.
- [ ] `@durableId` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX; use=required.

<a id="w16cex-ct_commentextensible"></a>

## w16cex:CT_CommentExtensible

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `extLst` — [w16:CT_ExtensionList](comments.md#w16-ct_extensionlist); MS-DOCX; minOccurs=0; maxOccurs=1.
- [ ] `@durableId` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); MS-DOCX; use=required.
- [ ] `@dateUtc` — [w:ST_DateTime](values-word.md#w-st_datetime); MS-DOCX; use=optional.
- [ ] `@intelligentPlaceholder` — [w:ST_OnOff](values-word.md#w-st_onoff); MS-DOCX; use=optional.

<a id="w16-ct_extensionlist"></a>

## w16:CT_ExtensionList

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `ext` — [w16:CT_Extension](comments.md#w16-ct_extension); MS-DOCX; minOccurs=0; maxOccurs=unbounded.

<a id="w15-ct_presenceinfo"></a>

## w15:CT_PresenceInfo

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `@providerId` — `xsd:string`; MS-DOCX; use=required.
- [ ] `@userId` — `xsd:string`; MS-DOCX; use=required.

<a id="w16-ct_extension"></a>

## w16:CT_Extension

Sources: [MS-DOCX](../../reference/reading/ooxml/microsoft/MS-DOCX/README.md).

- [ ] `*` — extension wildcard; MS-DOCX; processContents=lax.
- [ ] `@uri` — `xsd:token`; MS-DOCX.
