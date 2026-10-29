# Word numbering and list properties

[Inventory guide](README.md)

Used by lists, ordered lists, list items, and heading numbering. Includes abstract definitions, concrete instances, level overrides, marker text and formatting, and picture-bullet references.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_Numbering](#w-ct_numbering)
- [w:CT_NumPicBullet](#w-ct_numpicbullet)
- [w:CT_AbstractNum](#w-ct_abstractnum)
- [w:CT_Num](#w-ct_num)
- [w:CT_Picture](#w-ct_picture)
- [w:CT_Drawing](#w-ct_drawing)
- [w:CT_LongHexNumber](#w-ct_longhexnumber)
- [w:CT_MultiLevelType](#w-ct_multileveltype)
- [w:CT_Lvl](#w-ct_lvl)
- [w:CT_NumLvl](#w-ct_numlvl)
- [w:CT_Control](#w-ct_control)
- [w:CT_LevelSuffix](#w-ct_levelsuffix)
- [w:CT_LevelText](#w-ct_leveltext)
- [w:CT_LvlLegacy](#w-ct_lvllegacy)

<a id="w-ct_numbering"></a>

## w:CT_Numbering

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `numPicBullet` — [w:CT_NumPicBullet](numbering.md#w-ct_numpicbullet); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Picture Numbering Symbol Definition; [§17.9.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0731-0736-0137.md#source-page-731-part-1).
- [ ] `abstractNum` — [w:CT_AbstractNum](numbering.md#w-ct_abstractnum); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Abstract Numbering Definition; [§17.9.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0700-0706-0132.md#source-page-703-part-1).
- [ ] `num` — [w:CT_Num](numbering.md#w-ct_num); Strict, Transitional; minOccurs=0; maxOccurs=unbounded. Numbering Definition Instance; [§17.9.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0725-0730-0136.md#source-page-725-part-1).
- [ ] `numIdMacAtCleanup` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Last Reviewed Abstract Numbering Definition; [§17.9.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0725-0730-0136.md#source-page-730-part-1).

<a id="w-ct_numpicbullet"></a>

## w:CT_NumPicBullet

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `pict` — [w:CT_Picture](numbering.md#w-ct_picture); Transitional.
- [ ] `drawing` — [w:CT_Drawing](numbering.md#w-ct_drawing); Strict, Transitional. DrawingML Object; [§17.3.3.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0339-0344-0069.md#source-page-342-part-1).
- [ ] `@numPicBulletId` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.

<a id="w-ct_abstractnum"></a>

## w:CT_AbstractNum

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `nsid` — [w:CT_LongHexNumber](numbering.md#w-ct_longhexnumber); Strict, Transitional; minOccurs=0. Abstract Numbering Definition Identifier; [§17.9.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0719-0724-0135.md#source-page-724-part-1).
- [ ] `multiLevelType` — [w:CT_MultiLevelType](numbering.md#w-ct_multileveltype); Strict, Transitional; minOccurs=0. Abstract Numbering Definition Type; [§17.9.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0719-0724-0135.md#source-page-722-part-1).
- [ ] `tmpl` — [w:CT_LongHexNumber](numbering.md#w-ct_longhexnumber); Strict, Transitional; minOccurs=0. Numbering Template Code; [§17.9.29](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0743-0748-0139.md#source-page-743-part-1).
- [ ] `name` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0.
- [ ] `styleLink` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Numbering Style Definition; [§17.9.27](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0737-0742-0138.md#source-page-740-part-1).
- [ ] `numStyleLink` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Numbering Style Reference; [§17.9.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0731-0736-0137.md#source-page-733-part-1).
- [ ] `lvl` — [w:CT_Lvl](numbering.md#w-ct_lvl); Strict, Transitional; minOccurs=0; maxOccurs=9.
- [ ] `@abstractNumId` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.

<a id="w-ct_num"></a>

## w:CT_Num

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `abstractNumId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=1. Abstract Numbering Definition Reference; [§17.9.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0700-0706-0132.md#source-page-704-part-1).
- [ ] `lvlOverride` — [w:CT_NumLvl](numbering.md#w-ct_numlvl); Strict, Transitional; minOccurs=0; maxOccurs=9. Numbering Level Definition Override; [§17.9.8](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0713-0718-0134.md#source-page-715-part-1).
- [x] `@numId` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required. Sequence identity, definition geometry/format and restart rules allocate concrete Word numbering instances; references are verified. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs), [docx_packages.rs](../../../tests/docx_packages.rs).

<a id="w-ct_picture"></a>

## w:CT_Picture

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd).

- [ ] `*` — extension wildcard; Transitional; processContents=lax; namespace=urn:schemas-microsoft-com:vml; minOccurs=0; maxOccurs=unbounded.
- [ ] `*` — extension wildcard; Transitional; processContents=lax; namespace=urn:schemas-microsoft-com:office:office; minOccurs=0; maxOccurs=unbounded.
- [ ] `movie` — [w:CT_Rel](word-paragraphs.md#w-ct_rel); Transitional; minOccurs=0. Embedded Video; [§17.3.3.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0345-0350-0070.md#source-page-348-part-1).
- [ ] `control` — [w:CT_Control](numbering.md#w-ct_control); Transitional; minOccurs=0. Embedded Control; [§17.3.3.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0333-0338-0068.md#source-page-337-part-1).

<a id="w-ct_drawing"></a>

## w:CT_Drawing

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `wp:anchor` — [wp:CT_Anchor](images.md#wp-ct_anchor); Strict, Transitional; minOccurs=0.
- [ ] `wp:inline` — [wp:CT_Inline](images.md#wp-ct_inline); Strict, Transitional; minOccurs=0.

<a id="w-ct_longhexnumber"></a>

## w:CT_LongHexNumber

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=required.

<a id="w-ct_multileveltype"></a>

## w:CT_MultiLevelType

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_MultiLevelType](values-word.md#w-st_multileveltype); Strict, Transitional; use=required.

<a id="w-ct_lvl"></a>

## w:CT_Lvl

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `start` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Partial: Positive starts are emitted through instance-level overrides; the abstract definition starts at one. Zero starts and independently authored level defaults are not exposed. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs).
- [ ] `numFmt` — [w:CT_NumFmt](word-paragraphs.md#w-ct_numfmt); Strict, Transitional; minOccurs=0. Numbering Format; [§17.9.17](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0725-0730-0136.md#source-page-728-part-1). Partial: `format` covers decimal, lower/upper alpha, lower/upper Roman, and bullet; other numbering systems are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `lvlRestart` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Restart Numbering Level Symbol; [§17.9.10](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0713-0718-0134.md#source-page-718-part-1). Partial: `restart` allocates a fresh numbering instance; a numeric restart-after-level policy is not exposed. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs).
- [ ] `pStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0.
- [ ] `isLgl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Display All Levels Using Arabic Numerals; [§17.9.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0707-0712-0133.md#source-page-707-part-1).
- [ ] `suff` — [w:CT_LevelSuffix](numbering.md#w-ct_levelsuffix); Strict, Transitional; minOccurs=0. Content Between Numbering Symbol and Paragraph Text; [§17.9.28](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0737-0742-0138.md#source-page-742-part-1).
- [ ] `lvlText` — [w:CT_LevelText](numbering.md#w-ct_leveltext); Strict, Transitional; minOccurs=0. Numbering Level Text; [§17.9.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0719-0724-0135.md#source-page-721-part-1). Partial: Heading separators produce hierarchical labels and numeric lists use level counters; arbitrary number templates and custom DOCX bullets are not exposed. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs).
- [ ] `lvlPicBulletId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Picture Numbering Symbol Definition Reference; [§17.9.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0713-0718-0134.md#source-page-717-part-1).
- [ ] `legacy` — [w:CT_LvlLegacy](numbering.md#w-ct_lvllegacy); Transitional; minOccurs=0.
- [ ] `lvlJc` — [w:CT_Jc](word-paragraphs.md#w-ct_jc); Strict, Transitional; minOccurs=0. Justification; [§17.9.7](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0713-0718-0134.md#source-page-714-part-1).
- [ ] `pPr` — [w:CT_PPrGeneral](styles.md#w-ct_pprgeneral); Strict, Transitional; minOccurs=0. Partial: Scoped list/item indentation and spacing exist; full numbering-level paragraph properties are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `rPr` — [w:CT_RPr](word-runs.md#w-ct_rpr); Strict, Transitional; minOccurs=0. Partial: Inherited text formatting exists; independently authored marker run formatting is missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `@ilvl` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required. Positive Mote levels one through nine map to zero-based Word levels; out-of-range list levels fail explicitly. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs).
- [ ] `@tplc` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional; use=optional.
- [ ] `@tentative` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.

<a id="w-ct_numlvl"></a>

## w:CT_NumLvl

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `startOverride` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Numbering Level Starting Value Override; [§17.9.26](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0737-0742-0138.md#source-page-738-part-1).
- [ ] `lvl` — [w:CT_Lvl](numbering.md#w-ct_lvl); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `@ilvl` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.

<a id="w-ct_control"></a>

## w:CT_Control

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@name` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.
- [ ] `@shapeid` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional.
- [ ] `@r:id` — [r:ST_RelationshipId](values-shared.md#r-st_relationshipid); Strict, Transitional; use=optional.

<a id="w-ct_levelsuffix"></a>

## w:CT_LevelSuffix

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_LevelSuffix](values-word.md#w-st_levelsuffix); Strict, Transitional; use=required.

<a id="w-ct_leveltext"></a>

## w:CT_LevelText

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=optional. Partial: Hierarchical heading labels use the authored separator and numeric lists use level counters. Label/prefix metadata does not implement arbitrary Word numbering templates. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs).
- [ ] `@null` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.

<a id="w-ct_lvllegacy"></a>

## w:CT_LvlLegacy

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd).

- [ ] `@legacy` — [s:ST_OnOff](values-shared.md#s-st_onoff); Transitional; use=optional.
- [ ] `@legacySpace` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Transitional; use=optional.
- [ ] `@legacyIndent` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Transitional; use=optional.
