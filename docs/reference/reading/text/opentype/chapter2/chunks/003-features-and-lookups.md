# Features and lookups

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-scripts-and-languages.md) · [Next](004-lookuplist-table.md)

<a id="feats_lookups"></a>

<a id="features-and-lookups"></a>

## Features and lookups

<a id="overview-1"></a>

### Overview

Features define the advanced layout capabilities of a font and are named to convey meaning to the text-processing client. Consider a feature identified using the tag 'liga' to create ligatures. Because of its name, the client knows what the feature does and can decide whether to apply it. Several features have been defined and can be used in fonts and applications; see the [Feature Tags](https://learn.microsoft.com/en-us/typography/opentype/spec/featuretags) section of the OpenType Layout Tag Registry for more information. Font developers can also define their own features.

When designing a font, the font developer selects features according to the typographic capabilities the font will support. For each feature, they then implement one or more lookups that describe the glyph substitution or glyph positioning actions to be performed. Multiple lookups may be used for a given feature; in that case the lookup actions will be performed in sequential order. In some cases, multiple lookups applied in a specific order could be needed to obtain a desired effect.

When performing layout on a run of text, a client chooses the features to be applied to the run and then processes the lookups referenced by these features in the order the lookup definitions occur in the LookupList. As a result, within the GSUB or GPOS table, lookups from several different features may be interleaved during text processing. It is up to the font developer to determine the proper order of actions performed by the lookups.

A client may process GSUB features in ordered stages, with particular features processed in each stage. A client may also perform certain operations on glyph sequences, such as reordering of glyphs, before or between these stages. Such processing is required for correct support of many scripts. Details on such script-specific processing is outside the scope of this specification. Within any such feature-processing stage, however, lookups referenced by the features applied in that stage must be processed in LookupList order.

Lookup data is defined in one or more subtables that contain information about specific glyphs and the operations to be performed on them. Different lookup types support different types of operation; for example, positioning adjustment on a single glyph versus positioning adjustments on pairs of glyphs. Each type of lookup has one or more corresponding subtable format definitions. The choice of a subtable type and format depends upon two factors: the precise content of the information being applied to an operation, and the required storage efficiency. For complete definitions of all lookup types and subtables, see the [GSUB](../../gsub/README.md) and [GPOS](../../gpos/README.md) chapters of this document.

Processing of a lookup is finished when the client locates a target glyph or glyph context and performs the substitution or positioning action described.

Features and lookups define information that is specific to the glyphs in a given font. They do not encode information that is constant within the conventions of a particular language or the typography of a particular script. Information that would be replicated across all fonts for a given script or language belongs in the text-processing application for that language, not in the fonts.

<a id="flTbl"></a>

<a id="featurelist-table"></a>

### FeatureList table

The headers of the GSUB and GPOS tables contain offsets to FeatureList tables that enumerate all the features in a font. Features in a FeatureList are not limited to any single script. A FeatureList contains the entire list of either the GSUB or GPOS features that are used for layout of glyphs for all the scripts supported by the font.

The FeatureList table enumerates features in an array of records (FeatureRecord). Every feature requires a FeatureRecord, which consists of a feature tag that identifies the feature and an offset to a Feature table (described in the next section).

> *Note:*
> The values stored in the FeatureIndex array of a LangSys table are zero-based indices used to locate records in the FeatureRecord array of a FeatureList table.

A Feature table describes the lookups used to implement a given feature for a given script and language system. The implementation for a feature will typically be different for different scripts or language systems, requiring distinct Feature tables. Thus, the FeatureList table may include two or more records with the same feature tag when the feature is implemented for multiple scripts or language systems.

The FeatureRecord array should be sorted alphabetically by feature tag. If two or more records have the same feature tag, their relative order is arbitrary.

<a id="featureList"></a>

*FeatureList table*

<a id="featureCount_fl"></a>

<a id="featureRecs"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | featureCount | Number of records in the featureRecords array. |
| FeatureRecord | featureRecords[featureCount] | Array of FeatureRecords. |

<a id="featureRec"></a>

*FeatureRecord*

<a id="featureTag"></a>

<a id="featureTblOffset"></a>

| Type | Name | Description |
| --- | --- | --- |
| Tag | featureTag | 4-byte feature identification tag. |
| Offset16 | featureOffset | Offset to Feature table, from beginning of FeatureList. |

<a id="featTbl"></a>

<a id="feature-table"></a>

### Feature table

A feature table defines the implementation of a feature using one or more lookups. Feature tables defined within the GSUB table contain references to glyph substitution lookups; feature tables defined within the GPOS table contain references to glyph positioning lookups. If a feature requires both glyph substitution and positioning actions, then Feature tables referenced using the same feature tag will need to be defined in both the GSUB and GPOS tables.

A Feature table consists of an offset to a feature parameters table, a count of the lookups listed for the feature, and an arbitrarily ordered array of indices into a LookupList (described in the next section).

Feature parameters tables may only be used for certain features. The format of a features parameters table is specific to a particular feature and is specified in the description for that feature in the [Feature Tags](https://learn.microsoft.com/en-us/typography/opentype/spec/featuretags) section of the OpenType Layout Tag Registry. Currently, feature parameters tables are defined only for the following features:

- ['cv01' – 'cv99'](https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#cv01-cv99)
- ['size'](https://learn.microsoft.com/en-us/typography/opentype/spec/features_pt#size)
- ['ss01' – 'ss20'](https://learn.microsoft.com/en-us/typography/opentype/spec/features_pt#ssxx)

A feature parameters table may be required or optional, according to the specifications for a given feature. The length of a feature parameters table must be implicitly or explicitly specified in the table itself. The featureParamsOffset field in the Feature table gives the offset relative to the beginning of the Feature table. If a feature parameters table is not defined for a given feature, or if a feature parameters table is defined but not used in a given font, the featureParamsOffset field must be set to NULL.

To identify the features in a GSUB or GPOS table for a given script and language system, a text-processing client reads the feature tag of each FeatureRecord referenced in the given LangSys table. Then the client selects the features it wants to implement and uses the lookupListIndices arrays from the Feature tables for the selected features to obtain a list of Lookup indices for the chosen features. Next, the client arranges the indices numerically into their LookupList order. Finally, the client retrieves the referenced Lookup tables from the LookupList and applies the lookup data to substitute or position glyphs.

Example 3 at the end of this chapter shows the FeatureList and Feature tables used to substitute ligatures in two languages.

<a id="featureTable"></a>

*Feature table*

<a id="featureParams"></a>

<a id="lookupCount_ft"></a>

<a id="lookupIndices"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | featureParamsOffset | Offset from start of Feature table to feature parameters table, if defined for the feature and present, else NULL. |
| uint16 | lookupIndexCount | Number of elements in the lookupListIndices array. |
| uint16 | lookupListIndices[lookupIndexCount] | Array of indices into the LookupList — zero-based. |

