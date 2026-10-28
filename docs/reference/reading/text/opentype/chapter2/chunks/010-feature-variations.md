# Feature variations

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-device-and-variationindex-tables.md) · [Next](011-featuretablesubstitution-table.md)

<a id="featVarTbl"></a>

<a id="feature-variations"></a>

## Feature variations

<a id="featurevariations-table"></a>

### FeatureVariations table

A feature variations table describes variations on the effects of features based on various conditions. That is, it allows the default set of lookups for a given feature to be substituted with alternates of lookups under particular conditions.

The feature list provides an array of feature tables and associated feature tags, and a LangSys table identifies a particular set of the feature-table/tag pairs that will be supported for a given script and language system. The feature tables specified in a LangSys table are used by default when current conditions do not match any of the conditions for variation defined in the feature variations table. Those defaults will also be used under all conditions in implementations that do not support the feature variations table.

The feature variations table has an array of condition records, each of which references a set of conditions (a *condition set* table), and a set of alternate feature tables to use when a runtime context matches the condition set.

The substitutions given are replacements of one feature table for another. The alternate feature tables are appended at the end of the feature variations table, and are not included in the feature list table. Hence, there are no feature records in the feature list table that correspond to the alternate feature tables. An alternate feature table maintains the same feature tag association as the default feature table. Also, whereas the default feature tables in the feature list table are referenced using 16-bit offsets, the alternate feature tables are referenced using 32-bit offsets within the feature variations table.

When processing text, a default set of feature tables, each with an associated feature tag, is obtained from a LangSys table for a given script and language system. Condition sets are evaluated in order, testing for a condition set that matches the current runtime context. When the first match is found, the corresponding feature table substitution table is used to revise the set of feature tables obtained by default via the LangSys table, as described below (see [FeatureTableSubstitution table](011-featuretablesubstitution-table.md#ftsTbl)).

The format of the feature variations table is as follows.

<a id="featureVariationsTable"></a>

*FeatureVariations table*

<a id="majorVersion_fvt"></a>

<a id="minorVersion_fvt"></a>

<a id="featVarRecCount"></a>

<a id="featVarRecs"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the FeatureVariations table — set to 1. |
| uint16 | minorVersion | Minor version of the FeatureVariations table — set to 0. |
| uint32 | featureVariationRecordCount | Number of feature variation records. |
| FeatureVariationRecord | featureVariationRecords[featureVariationRecordCount] | Array of feature variation records. |

A feature variation record has offsets to a condition set table and to a feature table substitution table.

If the ConditionSet offset is 0, there is no condition set table. This is treated as the universal condition: all contexts are matched.

If the FeatureTableSubstitution offset is 0, there is no feature table substitution table, and no substitutions are made.

Feature variation records must be ordered in the order of precedence for the condition sets. During processing, the feature variation records will be read, and the corresponding condition sets tested, in the order in which they occur. If the condition set for a given record does not match the runtime context, then the next record is checked. The first feature variation record for which the condition set matches the runtime context will be considered as a candidate: if the version of the FeatureTableSubstitution table is supported, then this feature variation record will be used, and no additional feature variation records will be considered. If the version of the FeatureTableSubtitution table is not supported, then this feature variation record is rejected and processing will move to the next feature variation record.

<a id="featVarRec"></a>

*FeatureVariationRecord*

<a id="condSet"></a>

<a id="featTblSubst"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset32 | conditionSetOffset | Offset to a condition set table, from beginning of FeatureVariations table. |
| Offset32 | featureTableSubstitutionOffset | Offset to a feature table substitution table, from beginning of the FeatureVariations table. |

<a id="condSetTbl"></a>

<a id="conditionset-table"></a>

### ConditionSet table

A condition set table specifies a set of conditions under which a feature table substitution is to be applied. A condition set may specify conditions related to various factors; currently, one type of factor is supported: the variation instance of a variable font. Individual conditions are represented in subtables, which may use different formats according to the nature of the factor defining the condition.

For a given condition set, conditions are conjunctively related (boolean AND): all of the specified conditions must be met in order for the associated feature table substitution to be applied. A condition set does not need to specify conditional values for all possible factors. If no values are specified for some factor, then the condition set matches all runtime values for that factor.

If a given condition set contains no conditions, then it matches all contexts, and the associated feature table substitution is always applied, unless there was a FeatureVariation record earlier in the array with a condition set matching the current context.

<a id="conditionSetTable"></a>

*ConditionSet Table*

<a id="conditionCount"></a>

<a id="conditionTables"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | conditionCount | Number of conditions for this condition set. |
| Offset32 | conditionOffsets[conditionCount] | Array of offsets to condition tables, from beginning of the ConditionSet table. |

<a id="condTbl"></a>

<a id="condition-table"></a>

### Condition table

The condition table describes a particular condition. Different formats for the condition table may be defined, with each format used for a particular kind of condition qualifier. Currently, one format is defined: ConditionTableFormat1, which is used to specify a value range for a variation axis value in a variable font.

New condition table formats for other condition qualifiers may be added in the future. If a layout engine encounters a condition table with an unrecognized format, it should fail to match the condition set, but continue to test other condition sets. In this way, new condition formats can be defined and used in fonts that can work in a backward-compatible way in existing implementations.

<a id="conditionFormat1"></a>

<a id="condition-table-format-1-font-variation-axis-range"></a>

#### Condition table format 1: font variation axis range

A font variation axis range condition refers to a range of values for a design variation axis in a variable font. The axes of variation are specified in the [font variations ('fvar')](https://learn.microsoft.com/en-us/typography/opentype/spec/fvar) table of a font. If a format 1 condition table is used, there must be an 'fvar' table in the font, and the axisIndex value (which is zero-based) must be less than the axisCount value in the 'fvar' table. If the axisIndex is invalid, the feature variation record containing this condition table is ignored.

A format 1 condition table specifies a matching range of variation instance values along a single axis. Absence of a format 1 condition for a given variation axis implies that that axis is not a factor in determining applicability of the condition set.

The 'fvar' table defines a range of valid values for each variation axis. During processing for a particular variation instance, a normalization process is applied that maps user values in the range defined within the 'fvar' table to a normalized scale with a range from -1 to 1. The values specified in a format 1 condition table are expressed in terms of the normalized scale, and so can be any value from -1 to 1.

A font variation axis range condition is met if the currently-selected variation instance has a value for the given axis that is greater than or equal to the filterRangeMinValue, and that is less than or equal to the filterRangeMaxValue.

<a id="conditionTable1"></a>

*ConditionFormat1 table*

<a id="format_ct1"></a>

<a id="axisIndex_ct1"></a>

<a id="filterRangeMin_ct1"></a>

<a id="filterRangeMax_ct1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format, = 1 |
| uint16 | axisIndex | Index (zero-based) for the variation axis within the 'fvar' table. |
| F2DOT14 | filterRangeMinValue | Minimum value of the font variation instances that satisfy this condition. |
| F2DOT14 | filterRangeMaxValue | Maximum value of the font variation instances that satisfy this condition. |

