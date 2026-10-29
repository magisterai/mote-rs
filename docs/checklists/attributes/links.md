# Hyperlink properties

[Inventory guide](README.md)

External link destinations, interaction metadata and related DrawingML properties. Internal anchors are inventoried but not yet a language feature.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_Hyperlink](#w-ct_hyperlink)
- [a:CT_Hyperlink](#a-ct_hyperlink)
- [a:CT_EmbeddedWAVAudioFile](#a-ct_embeddedwavaudiofile)

<a id="w-ct_hyperlink"></a>

## w:CT_Hyperlink

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@tgtFrame` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.
- [ ] `@tooltip` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.
- [ ] `@docLocation` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.
- [ ] `@history` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.
- [ ] `@anchor` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.
- [x] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional. External `link.url` is emitted through a package relationship; relationship identifiers are generated, not authored. Evidence: [links.md](../../../docs/specification/links.md), [links.rs](../../../tests/links.rs), [docx_packages.rs](../../../tests/docx_packages.rs).

<a id="a-ct_hyperlink"></a>

## a:CT_Hyperlink

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `snd` — [a:CT_EmbeddedWAVAudioFile](links.md#a-ct_embeddedwavaudiofile); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `extLst` — [a:CT_OfficeArtExtensionList](fonts.md#a-ct_officeartextensionlist); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.
- [ ] `@invalidUrl` — `xsd:string`; Strict, Transitional; use=optional; default=.
- [ ] `@action` — `xsd:string`; Strict, Transitional; use=optional; default=.
- [ ] `@tgtFrame` — `xsd:string`; Strict, Transitional; use=optional; default=.
- [ ] `@tooltip` — `xsd:string`; Strict, Transitional; use=optional; default=.
- [ ] `@history` — `xsd:boolean`; Strict, Transitional; use=optional; default=true.
- [ ] `@highlightClick` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.
- [ ] `@endSnd` — `xsd:boolean`; Strict, Transitional; use=optional; default=false.

<a id="a-ct_embeddedwavaudiofile"></a>

## a:CT_EmbeddedWAVAudioFile

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/dml-main.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/dml-main.xsd).

- [ ] `@r:embed` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=required.
- [ ] `@name` — `xsd:string`; Strict, Transitional; use=optional; default=.
