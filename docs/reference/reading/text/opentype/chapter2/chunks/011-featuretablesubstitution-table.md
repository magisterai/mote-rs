# FeatureTableSubstitution table

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-feature-variations.md) · [Next](012-example-4-lookuplist-table-and-lookup-table.md)

<a id="ftsTbl"></a>

<a id="FTST"></a>

<a id="featuretablesubstitution-table"></a>

### FeatureTableSubstitution table

A feature table substitution table describes a set of feature table substitutions to be applied when the corresponding condition set matches the current runtime context. These substitutions are represented using an array of feature table substitution records. Each record gives a simple substitution of one feature table for another. When checking for a particular feature index, the first record having that index is matched, and searching ends if a record is encountered with a higher index value.

Note that the records must be ordered in increasing order of the FeatureIndex values, and no two records may have the same FeatureIndex value.

<a id="featTblSubstTable"></a>

*FeatureTableSubstitution table*

<a id="majorVersion_ftst"></a>

<a id="minorVersion_ftst"></a>

<a id="substitutionCount_ftst"></a>

<a id="substitutions_ftst"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the feature table substitution table — set to 1 |
| uint16 | minorVersion | Minor version of the feature table substitution table — set to 0. |
| uint16 | substitutionCount | Number of feature table substitution records. |
| FeatureTableSubstitutionRecord | substitutions[substitutionCount] | Array of feature table substitution records. |

<a id="featTblSubstRec"></a>

*FeatureTableSubstitutionRecord*

<a id="featIdx_ftsRec"></a>

<a id="altFeatIdx_ftsRec"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | featureIndex | The feature table index to match. |
| Offset32 | alternateFeatureOffset | Offset to an alternate feature table, from start of the FeatureTableSubstitution table. |

As described above, condition sets are evaluated and may be selected for processing of the associated feature table substitution table to replace a default feature table obtained from a LangSys table with an alternate feature table. Given a default array of feature tables for selected features obtained from the LangSys table, substitution of alternate feature tables can be done as follows:

1. For each feature index, evaluate the FeatureTableSubstitutionRecords in order.
2. If a matching record is encountered (FeatureIndex = the current feature index), then replace the feature table for that feature index using the alternate feature table at the offset given in the record. Stop processing for that feature index.
3. If a record is encountered with a higher feature index value, stop searching for that feature index; no substitution is made.

<a id="examples"></a>

<a id="common-table-examples"></a>

## Common table examples

The rest of this chapter describes and illustrates examples of all the common table formats. All the examples reflect unique parameters, but the samples provide a useful reference for building tables specific to other situations.

The examples have three columns showing hex data, source, and comments.

<a id="example-1-scriptlist-table-and-scriptrecords"></a>

### Example 1: ScriptList table and ScriptRecords

Example 1 illustrates a ScriptList table and ScriptRecord definitions for a Japanese font with multiple scripts: Han Ideographic, Kana, and Latin. Each script has script-specific behavior.

*Example 1*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **ScriptList**  TheScriptList | ScriptList table definintion |
| 0003 | 3 | scriptCount |
|  | scriptRecords[0] | In alphabetical order by script tag. |
| 68616E69 | 'hani' | scriptTag, Han Ideographic script |
| 0014 | HanIScriptTable | offset to Script table |
|  | scriptRecords[1] | In alphabetical order by script tag. |
| 6B616E61 | 'kana' | scriptTag, Hiragana and Katakana scripts |
| 0018 | KanaScriptTable | offset to Script table |
|  | scriptRecords[2] | In alphabetical order by script tag. |
| 6C61746E | 'latn' | scriptTag, Latin script |
| 001C | LatinScriptTable | offset to Script table |

<a id="example-2-script-table-langsysrecord-and-langsys-table"></a>

### Example 2: Script table, LangSysRecord, and LangSys table

Example 2 illustrates the Script table, LangSysRecord, and LangSys table definitions for the Arabic script and the Urdu language system. The default LangSys table defines three default Arabic script features used to replace certain glyphs in words with their proper initial, medial, and final glyph forms. These contextual substitutions are invariant and occur in all language systems that use the Arabic script.

Many alternative glyphs in the Arabic script have language-specific uses. For instance, the Arabic, Farsi, and Urdu language systems use different glyphs for numerals. To maintain character-set compatibility, the Unicode standard includes separate character codes for the Arabic and Farsi numeral glyphs. However, the standard uses the same character codes for Farsi and Urdu numerals, even though three of the Urdu glyphs (4, 6, and 7) differ from the Farsi glyphs. To access and display the proper glyphs for the Urdu numerals, users of the text-processing client must enter the character codes for the Farsi numerals. Then the text-processing client uses a required OpenType Layout glyph substitution feature, defined in the Urdu LangSys table, to access the correct Urdu glyphs for the 4, 6, and 7 numerals.

Note that the Urdu LangSys table repeats the default script features. This repetition is necessary because the Urdu language system also uses alternative glyphs in the initial, medial, and final glyph positions in words.

*Example 2*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **Script**  ArabicScriptTable | Script table definition |
| 000A | DefLangSys | offset to default LangSys table |
| 0001 | 1 | langSysCount |
|  | langSysRecords[0] | In alphabetical order by LangSys tag. |
| 55524420 | “URD ” | langSysTag, Urdu language |
| 0016 | UrduLangSys | offset to LangSys table for Urdu |
|  | **LangSys**  DefLangSys | default LangSys table definition |
| 0000 | NULL | lookupOrderOffset, reserved, null |
| FFFF | 0xFFFF | requiredFeatureIndex, no required features |
| 0003 | 3 | featureIndexCount |
| 0000 | 0 | featureIndices[0], in arbitrary order  'init' feature (initial glyph) |
| 0001 | 1 | featureIndices[1], 'fina' feature (final glyph) |
| 0002 | 2 | featureIndices[2], for 'medi' feature (medial glyph) |
|  | **LangSys**  UrduLangSys | LangSys table definition |
| 0000 | NULL | lookupOrderOffset, reserved, null |
| 0003 | 3 | requiredFeatureIndex, numeral subsitution in Urdu |
| 0003 | 3 | featureIndexCount |
| 0000 | 0 | featureIndices[0], in arbitrary order  'init' feature (initial glyph) |
| 0001 | 1 | featureIndices[1], 'fina' feature (final glyph) |
| 0002 | 2 | featureIndices[2], 'medi' feature (medial glyph) |

<a id="example-3-featurelist-table-and-feature-table"></a>

### Example 3: FeatureList table and Feature table

Example 3 shows the FeatureList and Feature table definitions for ligatures in the Latin script. The FeatureList has three features, all optional and named 'liga'. One feature, also a default, implements ligatures in Latin if no language-specific feature specifies other ligatures. Two other features implement ligatures in the Turkish and German languages, respectively.

Three lookups define glyph substitutions for rendering ligatures in this font. The first lookup produces the “ffi” and “fi” ligatures; the second produces the “ffl”, “fl”, and “ff” ligatures; and the third produces the eszet ligature.

The ligatures that begin with an “f” are separated into two sets because Turkish has a dotless “i” glyph and so does not use “ffi” and “fi” ligatures. However, Turkish does use the “ffl”, “fl”, and “ff” ligatures, and the TurkishLigatures feature table lists this one lookup.

Only the German language system uses the eszet ligature, so the GermanLigatures feature table includes a lookup for rendering that ligature.

Because the Latin script can use both sets of ligatures, the DefaultLigatures feature table defines two LookupList indices: one for the “ffi” and “fi” ligatures, and one for the “ffl”, “fl”, and “ff” ligatures. If the text-processing client selects this feature, then the font applies both lookups.

Note that the TurkishLigatures and DefaultLigatures feature tables both list a LookupListIndex of one (1) for the “ffl”, “fl”, and “ff” ligatures lookup. This is because language-specific lookups override all default language-system lookups, and a language-system feature table must explicitly list all lookups that apply to the language.

*Example 3*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **FeatureList**  TheFeatureList | FeatureList table definition |
| 0003 | 3 | featureCount |
|  | featureRecords[0] |  |
| 6C696761 | 'liga' | featureTag |
| 0014 | TurkishLigatures | offset to Feature table, FflFfFlLiga |
|  | featureRecords[1] |  |
| 6C696761 | 'liga' | featureTag |
| 001A | DefaultLigatures | offset to Feature table, FfiFiLiga, FflFfFlLiga |
|  | featureRecords[2] |  |
| 6C696761 | 'liga' | featureTag |
| 0022 | GermanLigatures | offset to Feature table, EszetLiga |
|  | **Feature**  TurkishLigatures | Feature table definition |
| 0000 | NULL | featureParamsOffset, null |
| 0001 | 1 | lookupIndexCount |
| 0001 | 1 | lookupListIndices[1], ffl, fl, ff ligature substitution Lookup |
|  | **Feature**  DefaultLigatures | Feature table definition |
| 0000 | NULL | featureParamsOffset, null |
| 0002 | 2 | lookupIndexCount |
| 0000 | 0 | lookupListIndices[0], in arbitrary order, ffi, fi ligatures |
| 0001 | 1 | lookupListIndices[1], ffl, fl, ff ligature substitution Lookup |
|  | **Feature**  GermanLigatures | Feature table definition |
| 0000 | NULL | featureParamsOffset, null |
| 0003 | 3 | lookupIndexCount |
| 0000 | 0 | lookupListIndices[0], in arbitrary order, ffi, fi ligatures |
| 0001 | 1 | lookupListIndices[1], ffl, fl, ff ligature substitution Lookup |
| 0002 | 2 | lookupListIndices[2], eszet ligature substitution Lookup |

