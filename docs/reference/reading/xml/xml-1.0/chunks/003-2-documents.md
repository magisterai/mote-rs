# 2 Documents

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-appendices.md) · [Next](004-2-4-character-data-and-markup.md)

<a id="sec-documents"></a>

## 2 Documents

<a id="dt-xml-doc"></a>

[Definition: A data object is an **XML
document** if it is [well-formed](003-2-documents.md#dt-wellformed "Well-Formed"),
as defined in this specification. In addition, the XML document is
[valid](005-2-8-prolog-and-document-type-declaration.md#dt-valid "Validity") if it meets certain further constraints.]

Each XML document has both a logical and a physical structure. Physically,
the document is composed of units called [entities](010-3-4-conditional-sections.md#dt-entity "Entity").
An entity may
[refer](011-4-1-character-and-entity-references.md#dt-entref "Entity Reference") to other entities to
cause their inclusion in the document. A document begins in a "root"
or [document entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity"). Logically, the document
is composed of declarations, elements, comments, character references, and
processing instructions, all of which are indicated in the document by explicit
markup. The logical and physical structures *MUST* nest properly, as described
in [**4.3.2 Well-Formed Parsed Entities**](012-4-3-parsed-entities.md#wf-entities).

<a id="sec-well-formed"></a>

### 2.1 Well-Formed XML Documents

<a id="dt-wellformed"></a>

[Definition: A textual object is a **well-formed**
XML document if:]

1. Taken as a whole, it matches the production labeled [document](003-2-documents.md#NT-document).
2. It meets all the well-formedness constraints given in this specification.
3. Each of the [parsed entities](010-3-4-conditional-sections.md#dt-parsedent "Text Entity")
   which is referenced directly or indirectly within the document is [well-formed](003-2-documents.md#dt-wellformed "Well-Formed").

<a id="document"></a>

##### Document

<a id="NT-document"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [1] | `document` | ::= | `prolog element Misc*` |

Matching the [document](003-2-documents.md#NT-document) production implies that:

1. It contains one or more [elements](007-2-12-language-identification.md#dt-element "Element").
2. <a id="dt-root"></a>

   [Definition: There is exactly one element,
   called the **root**, or document element, no part of which appears
   in the [content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-content "Content") of any other element.] For
   all other elements, if the [start-tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag") is in
   the content of another element, the [end-tag](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-etag "End Tag")
   is in the content of the same element. More simply stated, the elements,
   delimited by start- and end-tags, nest properly within each other.

<a id="dt-parentchild"></a>

[Definition: As a consequence of this,
for each non-root element `C` in the document, there is one other element `P`
in the document such that `C` is in the content of `P`, but
is not in the content of any other element that is in the content of `P`. `P`
is referred to as the **parent** of `C`, and `C` as
a **child** of `P`.]

<a id="charsets"></a>

### 2.2 Characters

<a id="dt-text"></a>

<a id="dt-character"></a>

[Definition: A parsed entity contains **text**,
a sequence of [characters](003-2-documents.md#dt-character "Character"), which may
represent markup or character data.]
[Definition: A **character**
is an atomic unit of text as specified by ISO/IEC 10646:2000 [[ISO/IEC 10646]](015-5-2-using-xml-processors.md#ISO10646). Legal characters are tab, carriage
return, line feed, and the legal characters
of Unicode and ISO/IEC 10646. The
versions of these standards cited in [**A.1 Normative References**](015-5-2-using-xml-processors.md#sec-existing-stds) were
current at the time this document was prepared. New characters may be added
to these standards by amendments or new editions. Consequently, XML processors
*MUST* accept any character in the range specified for [Char](003-2-documents.md#NT-Char).
]

<a id="char32"></a>

##### Character Range

<a id="NT-Char"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [2] | `Char` | ::= | `#x9 | #xA | #xD | [#x20-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF]` | */\* any Unicode character, excluding the surrogate blocks, FFFE, and FFFF. \*/* |

The mechanism for encoding character code points into bit patterns may
vary from entity to entity. All XML processors *MUST* accept the UTF-8 and UTF-16
encodings of Unicode [[Unicode]](015-5-2-using-xml-processors.md#Unicode);
the mechanisms for signaling which of the two is in use,
or for bringing other encodings into play, are discussed later, in [**4.3.3 Character Encoding in Entities**](012-4-3-parsed-entities.md#charencoding).

**Note:**

Document authors are encouraged to avoid
"compatibility characters", as defined
in section 2.3 of [[Unicode]](015-5-2-using-xml-processors.md#Unicode). The characters defined in the following ranges are also
discouraged. They are either control characters or permanently undefined Unicode
characters:



```
[#x7F-#x84], [#x86-#x9F], [#xFDD0-#xFDEF],
[#x1FFFE-#x1FFFF], [#x2FFFE-#x2FFFF], [#x3FFFE-#x3FFFF],
[#x4FFFE-#x4FFFF], [#x5FFFE-#x5FFFF], [#x6FFFE-#x6FFFF],
[#x7FFFE-#x7FFFF], [#x8FFFE-#x8FFFF], [#x9FFFE-#x9FFFF],
[#xAFFFE-#xAFFFF], [#xBFFFE-#xBFFFF], [#xCFFFE-#xCFFFF],
[#xDFFFE-#xDFFFF], [#xEFFFE-#xEFFFF], [#xFFFFE-#xFFFFF],
[#x10FFFE-#x10FFFF].
```




<a id="sec-common-syn"></a>

### 2.3 Common Syntactic Constructs

This section defines some symbols used widely in the grammar.

[S](003-2-documents.md#NT-S) (white space) consists of one or more space (#x20)
characters, carriage returns, line feeds, or tabs.

<a id="white"></a>

##### White Space

<a id="NT-S"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [3] | `S` | ::= | `(#x20 | #x9 | #xD | #xA)+` |

**Note:**

The presence of #xD in the above production is
maintained purely for backward compatibility with the
[First Edition](https://www.w3.org/TR/1998/REC-xml-19980210).
As explained in [**2.11 End-of-Line Handling**](006-2-9-standalone-document-declaration.md#sec-line-ends),
all #xD characters literally present in an XML document
are either removed or replaced by #xA characters before
any other processing is done. The only way to get a #xD character to match this production is to
use a character reference in an entity value literal.

An [Nmtoken](003-2-documents.md#NT-Nmtoken) (name token) is any mixture of name
characters.

<a id="dt-name"></a>

[Definition: A [Name](003-2-documents.md#NT-Name) is an [Nmtoken](003-2-documents.md#NT-Nmtoken) with a restricted set of initial characters.] Disallowed initial characters for [Names](003-2-documents.md#NT-Name) include digits, diacritics, the full stop and the hyphen.

Names beginning with the string "`xml`",
or with any string which would match `(('X'|'x') ('M'|'m') ('L'|'l'))`,
are reserved for standardization in this or future versions of this specification.

**Note:**

The
Namespaces in XML Recommendation [[XML Names]](016-a-2-other-references.md#xml-names) assigns a meaning
to names containing colon characters. Therefore, authors should not use the
colon in XML names except for namespace purposes, but XML processors must
accept the colon as a name character.

The first character of a [Name](003-2-documents.md#NT-Name) *MUST* be a [NameStartChar](003-2-documents.md#NT-NameStartChar), and any
other characters *MUST* be [NameChars](003-2-documents.md#NT-NameChar); this mechanism is used to
prevent names from beginning with European (ASCII) digits or with
basic combining characters. Almost all characters are permitted in
names, except those which either are or reasonably could be used as
delimiters. The intention is to be inclusive rather than exclusive,
so that writing systems not yet encoded in Unicode can be used in
XML names. See [**J Suggestions for XML Names**](019-i-production-notes-non-normative.md#sec-suggested-names) for suggestions on the creation of
names.

Document authors are encouraged to use names which are
meaningful words or combinations of words in natural languages, and
to avoid symbolic or white space characters in names. Note that
COLON, HYPHEN-MINUS, FULL STOP (period), LOW LINE (underscore), and
MIDDLE DOT are explicitly permitted.

The ASCII symbols and punctuation marks, along with a fairly
large group of Unicode symbol characters, are excluded from names
because they are more useful as delimiters in contexts where XML
names are used outside XML documents; providing this group gives
those contexts hard guarantees about what *cannot* be part of
an XML name. The character #x037E, GREEK QUESTION MARK, is excluded
because when normalized it becomes a semicolon, which could change
the meaning of entity references.

<a id="d0e804"></a>

##### Names and Tokens

<a id="NT-NameStartChar"></a>

<a id="NT-NameChar"></a>

<a id="NT-Name"></a>

<a id="NT-Names"></a>

<a id="NT-Nmtoken"></a>

<a id="NT-Nmtokens"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [4] | `NameStartChar` | ::= | `":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]` |
|  |  |  |  |
| --- | --- | --- | --- |
| [4a] | `NameChar` | ::= | `NameStartChar | "-" | "." | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]` |
|  |  |  |  |
| --- | --- | --- | --- |
| [5] | `Name` | ::= | `NameStartChar (NameChar)*` |
|  |  |  |  |
| --- | --- | --- | --- |
| [6] | `Names` | ::= | `Name (#x20 Name)*` |
|  |  |  |  |
| --- | --- | --- | --- |
| [7] | `Nmtoken` | ::= | `(NameChar)+` |
|  |  |  |  |
| --- | --- | --- | --- |
| [8] | `Nmtokens` | ::= | `Nmtoken (#x20 Nmtoken)*` |

**Note:**

The [Names](003-2-documents.md#NT-Names)
and [Nmtokens](003-2-documents.md#NT-Nmtokens) productions are used to define the validity
of tokenized attribute values after normalization (see [**3.3.1 Attribute Types**](009-3-3-attribute-list-declarations.md#sec-attribute-types)).

Literal data is any quoted string not containing the quotation mark used
as a delimiter for that string. Literals are used for specifying the content
of internal entities ([EntityValue](003-2-documents.md#NT-EntityValue)), the values
of attributes ([AttValue](003-2-documents.md#NT-AttValue)), and external identifiers
([SystemLiteral](003-2-documents.md#NT-SystemLiteral)). Note that a [SystemLiteral](003-2-documents.md#NT-SystemLiteral)
can be parsed without scanning for markup.

<a id="d0e888"></a>

##### Literals

<a id="NT-EntityValue"></a>

<a id="NT-AttValue"></a>

<a id="NT-SystemLiteral"></a>

<a id="NT-PubidLiteral"></a>

<a id="NT-PubidChar"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [9] | `EntityValue` | ::= | `'"' ([^%&"] | PEReference | Reference)* '"'` |
|  |  |  | `|  "'" ([^%&'] | PEReference | Reference)* "'"` |
|  |  |  |  |
| --- | --- | --- | --- |
| [10] | `AttValue` | ::= | `'"' ([^<&"] | Reference)* '"'` |
|  |  |  | `|  "'" ([^<&'] | Reference)* "'"` |
|  |  |  |  |
| --- | --- | --- | --- |
| [11] | `SystemLiteral` | ::= | `('"' [^"]* '"') | ("'" [^']* "'")` |
|  |  |  |  |
| --- | --- | --- | --- |
| [12] | `PubidLiteral` | ::= | `'"' PubidChar* '"' | "'" (PubidChar - "'")* "'"` |
|  |  |  |  |
| --- | --- | --- | --- |
| [13] | `PubidChar` | ::= | `#x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]` |

**Note:**

Although
the [EntityValue](003-2-documents.md#NT-EntityValue) production allows the definition
of a general entity consisting of a single explicit `<` in the literal
(e.g., `<!ENTITY mylt "<">`), it is strongly advised to avoid
this practice since any reference to that entity will cause a well-formedness
error.

