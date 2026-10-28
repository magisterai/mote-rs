# Scripts and languages

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-overview.md) · [Next](003-features-and-lookups.md)

<a id="scriptsLangs"></a>

<a id="scripts-and-languages"></a>

## Scripts and languages

Three tables and their associated records apply to scripts and languages: the ScriptList table and its script record (ScriptRecord), the Script table and its language system record (LangSysRecord), and the Language System table (LangSys).

<a id="slTbl_sRec"></a>

<a id="scriptlist-table"></a>

### ScriptList table

A font may contain one or more groups of glyphs used to render various scripts, which are enumerated in a ScriptList table. Both the GSUB and GPOS tables define ScriptList tables:

- The GSUB table uses the ScriptList table to access the glyph substitution features that apply to a script. For details, see the chapter, [The Glyph Substitution Table (GSUB)](../../gsub/README.md).
- The GPOS table uses the ScriptList table to access the glyph positioning features that apply to a script. For details, see the chapter, [The Glyph Positioning Table (GPOS)](../../gpos/README.md).

A ScriptList table consists of a count of the scripts represented by the glyphs in the font (ScriptCount) and an array of records (ScriptRecord), one for each script for which the font defines script-specific features (a script without script-specific features does not need a ScriptRecord). Each ScriptRecord consists of a ScriptTag that identifies a script, and an offset to a Script table. The ScriptRecord array is stored in alphabetic order of the script tags.

A Script table with the script tag DFLT (default) may be used in a font to define features that are not script-specific. An application should use a DFLT script table if there is not a script table associated with the specific script of the text being formatted, or if the text does not have a specific script (for example, it contains only symbols or punctuation).

> *Note:*
> If symbols or punctuation have a Unicode script property “Common” but are used together with characters of a specific script, features that apply to those symbol or punctuation characters should not necessarily be organized under the DFLT script, but can be organized under the specific script. Applications may process script-neutral characters together with immediately-preceding or following script-specific characters for better processing efficiency. In that case, an application would look for features that operate on the neutral characters by using the Script table for the specific script. The DFLT script would still be used if the text contained only the neutral characters, however.

If there is a DFLT script table, it must have a default language system table (defaultLangSysOffset must not equal NULL—see below).

As languages are written using particular scripts, it is normally expected that language-specific typographic effects will be associated with a particular script, not with the generic DFLT script. For this reason, a DFLT script table should normally have only the default language system table, not language-specific tables. However, a font may have a DFLT script table with non-default language system tables, and an application may use features associated with one of these if the DFLT script table is applicable — no script table is present for the specific script, or there is no specific script in the text context — and if one of the particular language systems is specified. Under such conditions, applications should support use of a non-default language system table that is associated with DFLT script if a font includes tables with this configuration.

Example 1 at the end of this chapter shows a ScriptList table and ScriptRecords for a Japanese font that uses three scripts.

<a id="scriptList"></a>

*ScriptList table*

<a id="scriptCount"></a>

<a id="scriptRecords"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | scriptCount | Number of ScriptRecords |
| ScriptRecord | scriptRecords[scriptCount] | Array of ScriptRecords, listed alphabetically by script tag |

<a id="scriptRec"></a>

*ScriptRecord*

<a id="scriptTag"></a>

<a id="scriptTblOffset"></a>

| Type | Name | Description |
| --- | --- | --- |
| Tag | scriptTag | 4-byte script tag identifier |
| Offset16 | scriptOffset | Offset to Script table, from beginning of ScriptList |

<a id="sTbl_lsRec"></a>

<a id="script-table"></a>

### Script table

A Script table may specify one or more language systems that define behaviors of the glyphs in a script for a particular language. It also references a default language system that defines behaviors of the script’s glyphs to be used in the absence of language-specific information.

A Script table begins with an offset to the default language system table (defaultLangSysOffset), which defines the set of features that regulate the default behavior for the script. Next, a count of language-specific language systems is given (langSysCount), followed by a corresponding array of language system records (LangSysRecord). Each record specifies a language system using a language system tag (langSysTag), and specifies an offset to a language system table (LangSys). The LangSysRecord array must be sorted alphabetically by language system tag. If no language-specific behavior is defined, the langSysCount field is set to zero and the LangSysRecord array is empty.

<a id="scriptTable"></a>

*Script table*

<a id="dfltLangSys"></a>

<a id="langSysCount"></a>

<a id="langSysRecords"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | defaultLangSysOffset | Offset to default LangSys table, from beginning of Script table — may be NULL. |
| uint16 | langSysCount | Number of records in the langSysRecords array. |
| LangSysRecord | langSysRecords[langSysCount] | Array of LangSysRecords, sorted alphabetically by LangSys tag. |

<a id="langSysRec"></a>

*LangSysRecord*

<a id="langSysTag"></a>

<a id="langSysTblOffset"></a>

| Type | Name | Description |
| --- | --- | --- |
| Tag | langSysTag | 4-byte LangSysTag identifier. |
| Offset16 | langSysOffset | Offset to LangSys table, from beginning of Script table. |

<a id="langSysTbl"></a>

<a id="language-system-table"></a>

### Language system table

The language system table (LangSys) identifies features used for layout of the glyphs in a script. Features are specified as zero-based indices into a FeatureList table, defined in the next section of this chapter.

Optionally, a LangSys table may define a required feature index (the requiredFeatureIndex field) to specify one feature as required within the context of a particular language system. For example, in the Cyrillic script, the Serbian language system uses different glyphs for certain characters than the Russian language system. Only one feature index value can be specified as a required feature. This is not a functional limitation, however, because feature and lookup definitions are structured so that one feature table can reference many glyph substitution or positioning lookups. When no required features are defined, then requiredFeatureIndex is set to 0xFFFF.

All other features are considered optional with respect to the general requirements for processing the language system table. Applications, however, may treat certain features as required in certain contexts, regardless of whether a feature is referenced using the requiredFeatureIndex field. In particular, applications may treat certain features as required for correct layout of certain Unicode characters or scripts. Such higher-level requirements are outside the scope of this specification.

Example 2 at the end of this chapter shows a Script table, LangSysRecord, and LangSys table used for contextual positioning in the Arabic script.

<a id="langSysTable"></a>

*LangSys table*

<a id="lookupOrder"></a>

<a id="reqFeatureIndex"></a>

<a id="featureCount_lst"></a>

<a id="featureIndices"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | lookupOrderOffset | Reserved—set to NULL. |
| uint16 | requiredFeatureIndex | Index of a feature required for this language system; if no required features, set to 0xFFFF. |
| uint16 | featureIndexCount | Number of elements in the featureIndices array. |
| uint16 | featureIndices[featureIndexCount] | Array of indices into the FeatureList, in arbitrary order. |

The lookupOrderOffset field is reserved for future use. Indices in the featureIndices array may be in arbitrary order.

