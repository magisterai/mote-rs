# Naming table header

Source: [Naming table](https://learn.microsoft.com/en-us/typography/opentype/spec/name) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/name).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-name-ids.md)

The naming table allows multilingual strings to be associated with the OpenType™ font. These strings can represent copyright notices, font names, family names, style names, and so on. To keep this table short, the font manufacturer may wish to make a limited set of entries in some small set of languages; later, the font can be “localized” and the strings translated or added. Other parts of the OpenType font that require these strings can refer to them using a language-independent name ID. In addition to language variants, the table also allows for platform-specific character-encoding variants. Applications that need a particular string can look it up by its platform ID, encoding ID, language ID and name ID. Note that different platforms may have different requirements for the encoding of strings.

Many newer platforms can use strings intended for different platforms if a font does not include strings for that platform. Some applications might display incorrect strings, however, if strings for the current platform are not included.

<a id="nameHeader"></a>

<a id="naming-table-header"></a>

## Naming table header

There are two versions of the Naming Table. Version 0 uses platform-specific, numeric language identifiers. Version 1 allows for use of language-tag strings to indicate the language of strings. Both versions include variable-size string-data storage, and an array of name records that are used to identify the type of string (name ID), platform, encoding and language variants of the string, and the location within the storage.

<a id="nameHeader_0"></a>

<a id="naming-table-version-0"></a>

### Naming table version 0

The version 0 naming table is organized as follows:

| Type | Name | Description |
| --- | --- | --- |
| uint16 | version | Table version number (=0). |
| uint16 | count | Number of name records. |
| Offset16 | storageOffset | Offset to start of string storage (from start of table). |
| NameRecord | nameRecord[count] | The name records where *count* is the number of records. |
| (Variable) |  | Storage for the actual string data. |

Version 0 differs from version 1 in regard to handling of language identification: it uses only numeric language IDs, which generally are values less than 0x8000 and have platform-specific interpretations. See Name Records below for more detail.

<a id="nameHeader_1"></a>

<a id="naming-table-version-1"></a>

### Naming table version 1

The version 1 naming table adds additional elements, as follows:

| Type | Name | Description |
| --- | --- | --- |
| uint16 | version | Table version number (=1). |
| uint16 | count | Number of name records. |
| Offset16 | storageOffset | Offset to start of string storage (from start of table). |
| NameRecord | nameRecord[count] | The name records where *count* is the number of records. |
| uint16 | langTagCount | Number of language-tag records. |
| LangTagRecord | langTagRecord[langTagCount] | The language-tag records where *langTagCount* is the number of records. |
| (Variable) |  | Storage for the actual string data. |

When version 1 is used, the language IDs in name records can be less than or greater than 0x8000. If a language ID is less than 0x8000, it has a platform-specific interpretation as with a version 0 naming table. If a language ID is equal to or greater than 0x8000, it is associated with a language-tag record (LangTagRecord) that references a language-tag string. In this way, the language ID is associated with a language-tag string that specifies the language for name records using that language ID, regardless of the platform. These can be used for any platform that supports this language-tag mechanism.

A font using a version 1 naming table may use a combination of platform-specific language IDs as well as language-tag records for a given platform and encoding.

<a id="langTagRec"></a>

Each LangTagRecord is organized as follows:

<a id="length_ltr"></a>

<a id="offset_ltr"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | length | Language-tag string length (in bytes). |
| Offset16 | langTagOffset | Language-tag string offset from start of storage area (in bytes). |

Language-tag strings stored in the naming table must be encoded in UTF-16BE. The language tags must conform to [IETF specification BCP 47](https://tools.ietf.org/html/bcp47). This provides tags such as “en”, “fr-CA” and “zh-Hant” to identify languages, including dialects, written form and other language variants.

The language-tag records are associated sequentially with language IDs starting with 0x8000. Each language-tag record corresponds to a language ID one greater than that for the previous language-tag record. Thus, language IDs associated with language-tag records must be within the range 0x8000 to 0x8000 + langTagCount - 1. If a name record uses a language ID that is greater than this, the identity of the language is unknown; such name records should not be used.

For example, suppose a font has two language-tag records referencing strings in the storage: the first references the string “en”, and the second references the string “zh-Hant-HK” In this case, the language ID 0x8000 is used in name records to index English-language strings. The language ID 0x8001 is used in name records to index strings in Traditional Chinese as used in Hong Kong SAR.

<a id="nameRecord"></a>

<a id="name-records"></a>

## Name records

Each string in the string storage is referenced by a name record. The name record has a multi-part key, to identify the logical type of string and its language or platform-specific implementation variants, plus the location of the string in the string storage.

Each NameRecord is organized as follows:

<a id="platformID_nr"></a>

<a id="encodingID_nr"></a>

<a id="languageID_nr"></a>

<a id="nameID_nr"></a>

<a id="length_nr"></a>

<a id="offset_nr"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | platformID | Platform ID. |
| uint16 | encodingID | Platform-specific encoding ID. |
| uint16 | languageID | Language ID. |
| uint16 | nameID | Name ID. |
| uint16 | length | String length (in bytes). |
| Offset16 | stringOffset | String offset from start of storage area (in bytes). |

The name ID identifies a logical string category, such as family name or copyright. Name IDs are the same for all platforms and languages; these are described in detail below. The other three elements of the key allow for platform-specific implementations: a platform ID, a platform-specific encoding ID, and a language ID.

As with encoding records in the ['cmap' table](../../cmap/README.md), name records must be sorted first by platform ID, then by platform-specific encoding ID, then by language ID, and then by name ID. Descriptions of the various IDs follow.

<a id="platform-encoding-and-language"></a>

## Platform, encoding and language

The platform, encoding and language IDs of a name record allow for platform-specific implementations. Different platforms can support different encodings, and different languages. All encoding IDs are platform-specific. Language IDs are similarly platform-specific, except in the case of IDs used in conjunction with the language-tag mechanism of naming table version 1, described above.

> *Note:* Platform IDs, platform-specific encoding IDs and, in some cases, platform-specific language IDs are also used in the 'cmap' table.

Language IDs refer to a value that identifies the language in which a particular string is written. Values less than 0x8000 are defined on a platform-specific basis. A version 0 naming table must use only language ID values less than 0x8000 from the platform-specific enumerations given below. (Exceptions to this requirement are permitted, however, in the case of user-defined platforms — platform IDs 240 to 255.) Values greater than or equal to 0x8000 may be used in a version 1 naming table in conjunction with language-tag records, as described above. Not all platforms have platform-specific language IDs, and not all platforms support language-tag records.

Tables with detailed listings of platform, encoding and language IDs used within the 'name' table are provided at the end of this chapter. See [Platform, encoding and language IDs](005-platform-encoding-and-language-ids.md#platform-encoding-and-language-ids).

