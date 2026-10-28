# 4.3 Parsed Entities

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-4-1-character-and-entity-references.md) · [Next](013-4-4-xml-processor-treatment-of-entities-and-references.md)

```
<!ENTITY open-hatch
         SYSTEM "http://www.textuality.com/boilerplate/OpenHatch.xml">
<!ENTITY open-hatch
         PUBLIC "-//Textuality//TEXT Standard open-hatch boilerplate//EN"
         "http://www.textuality.com/boilerplate/OpenHatch.xml">
<!ENTITY hatch-pic
         SYSTEM "../grafix/OpenHatch.gif"
         NDATA gif >
```




<a id="TextEntities"></a>

### 4.3 Parsed Entities

<a id="sec-TextDecl"></a>

#### 4.3.1 The Text Declaration

External parsed entities *SHOULD* each begin with a **text declaration**.

<a id="d0e4481"></a>

##### Text Declaration

<a id="NT-TextDecl"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [77] | `TextDecl` | ::= | `'<?xml' VersionInfo? EncodingDecl S? '?>'` |

The text declaration *MUST* be provided literally, not by reference
to a parsed entity. The text declaration
*MUST NOT* appear at any
position other than the beginning of an external parsed entity. The text declaration
in an external parsed entity is not considered part of its [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text").

<a id="wf-entities"></a>

#### 4.3.2 Well-Formed Parsed Entities

The document entity is well-formed if it matches the production labeled [document](003-2-documents.md#NT-document). An external general parsed entity is well-formed
if it matches the production labeled [extParsedEnt](012-4-3-parsed-entities.md#NT-extParsedEnt). All
external parameter entities are well-formed by definition.

**Note:**

Only parsed entities that are referenced directly or indirectly within the document are required to be well-formed.

<a id="d0e4524"></a>

##### Well-Formed External Parsed Entity

<a id="NT-extParsedEnt"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [78] | `extParsedEnt` | ::= | `TextDecl? content` |

An internal general parsed entity is well-formed if its replacement text
matches the production labeled [content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-content). All internal
parameter entities are well-formed by definition.

A consequence of well-formedness in general
entities is that the logical and physical
structures in an XML document are properly nested; no [start-tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag"), [end-tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-etag "End Tag"), [empty-element tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-empty "Empty"), [element](007-2-12-language-identification.md#dt-element "Element"), [comment](004-2-4-character-data-and-markup.md#dt-comment "Comment"), [processing instruction](004-2-4-character-data-and-markup.md#dt-pi "Processing instruction"), [character
reference](011-4-1-character-and-entity-references.md#dt-charref "Character Reference"), or [entity reference](011-4-1-character-and-entity-references.md#dt-entref "Entity Reference")
can begin in one entity and end in another.

<a id="charencoding"></a>

#### 4.3.3 Character Encoding in Entities

Each external parsed entity in an XML document may use a different encoding
for its characters. All XML processors *MUST* be able to read entities in both
the UTF-8 and UTF-16 encodings. The terms "UTF-8"
and "UTF-16" in this specification do not apply to
related character encodings, including but not limited to UTF-16BE, UTF-16LE, or CESU-8.

Entities encoded in UTF-16 *MUST* and entities
encoded in UTF-8 *MAY* begin with the Byte Order Mark described by
Annex H of [[ISO/IEC 10646:2000]](015-5-2-using-xml-processors.md#ISO10646-2000), section
16.8 of [[Unicode]](015-5-2-using-xml-processors.md#Unicode)
(the ZERO WIDTH NO-BREAK SPACE character, #xFEFF). This is an encoding signature,
not part of either the markup or the character data of the XML document. XML
processors *MUST* be able to use this character to differentiate between UTF-8
and UTF-16 encoded documents.

If the replacement text of an external entity is to
begin with the character U+FEFF, and no text declaration
is present, then a Byte Order Mark MUST be present,
whether the entity is encoded in UTF-8 or UTF-16.

Although an XML processor is required to read only entities in the UTF-8
and UTF-16 encodings, it is recognized that other encodings are used around
the world, and it may be desired for XML processors to read entities that
use them. In
the absence of external character encoding information (such as MIME headers),
parsed entities which are stored in an encoding other than UTF-8 or UTF-16
*MUST* begin with a text declaration (see [**4.3.1 The Text Declaration**](012-4-3-parsed-entities.md#sec-TextDecl)) containing
an encoding declaration:

<a id="d0e4623"></a>

##### Encoding Declaration

<a id="NT-EncodingDecl"></a>

<a id="NT-EncName"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [80] | `EncodingDecl` | ::= | `S 'encoding' Eq ('"' EncName '"' | "'" EncName "'" )` |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [81] | `EncName` | ::= | `[A-Za-z] ([A-Za-z0-9._] | '-')*` | */\* Encoding name contains only Latin characters \*/* |

In the [document entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity"), the encoding
declaration is part of the [XML declaration](005-2-8-prolog-and-document-type-declaration.md#dt-xmldecl "XML Declaration").
The [EncName](012-4-3-parsed-entities.md#NT-EncName) is the name of the encoding used.

In an encoding declaration, the values "
`UTF-8`
", "
`UTF-16`
",
"
`ISO-10646-UCS-2`
", and "
`ISO-10646-UCS-4`
"
*SHOULD* be used
for the various encodings and transformations of Unicode / ISO/IEC 10646,
the values "
`ISO-8859-1`
", "
`ISO-8859-2`
",
... "
`ISO-8859-`
n
" (where n
is the part number) *SHOULD* be used for the parts of ISO 8859, and
the values "
`ISO-2022-JP`
", "
`Shift_JIS`
",
and "
`EUC-JP`
"
*SHOULD* be used for the various encoded
forms of JIS X-0208-1997. It
is *RECOMMENDED* that character encodings registered (as *charset*s)
with the Internet Assigned Numbers Authority [[IANA-CHARSETS]](015-5-2-using-xml-processors.md#IANA),
other than those just listed, be referred to using their registered names;
other encodings *SHOULD* use names starting with an "x-" prefix.
XML processors *SHOULD* match character encoding names in a case-insensitive
way and *SHOULD* either interpret an IANA-registered name as the encoding registered
at IANA for that name or treat it as unknown (processors are, of course, not
required to support all IANA-registered encodings).

In the absence of information provided by an external transport protocol
(e.g. HTTP or MIME), it is a [fatal error](002-appendices.md#dt-fatal "Fatal Error") for
an entity including an encoding declaration to be presented to the XML processor
in an encoding other than that named in the declaration, or for an entity which
begins with neither a Byte Order Mark
nor an encoding declaration to use an encoding other than UTF-8. Note that
since ASCII is a subset of UTF-8, ordinary ASCII entities do not strictly
need an encoding declaration.

It is a [fatal error](002-appendices.md#dt-fatal "Fatal Error") for a [TextDecl](012-4-3-parsed-entities.md#NT-TextDecl) to occur other
than at the beginning of an external entity.

It is a [fatal error](002-appendices.md#dt-fatal "Fatal Error") when an XML processor
encounters an entity with an encoding that it is unable to process. It
is a [fatal error](002-appendices.md#dt-fatal "Fatal Error") if an XML entity is determined (via default, encoding declaration,
or higher-level protocol) to be in a certain encoding but contains byte
sequences that are not legal in that encoding. Specifically, it is a
fatal error if an entity encoded in UTF-8 contains any ill-formed code unit sequences,
as defined in section 3.9 of  Unicode [[Unicode]](015-5-2-using-xml-processors.md#Unicode). Unless an encoding
is determined by a higher-level protocol, it is also a [fatal error](002-appendices.md#dt-fatal "Fatal Error") if an XML entity
contains no encoding declaration and its content is not legal UTF-8 or UTF-16.

Examples of text declarations containing encoding declarations:



```
<?xml encoding='UTF-8'?>
<?xml encoding='EUC-JP'?>
```




