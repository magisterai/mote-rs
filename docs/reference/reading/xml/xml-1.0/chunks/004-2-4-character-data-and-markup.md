# 2.4 Character Data and Markup

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-2-documents.md) · [Next](005-2-8-prolog-and-document-type-declaration.md)

<a id="syntax"></a>

### 2.4 Character Data and Markup

<a id="dt-markup"></a>

[Text](003-2-documents.md#dt-text "Text") consists of intermingled [character data](004-2-4-character-data-and-markup.md#dt-chardata "Character Data") and markup. [Definition:
**Markup** takes the form of [start-tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag"), [end-tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-etag "End Tag"), [empty-element tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-empty "Empty"), [entity references](011-4-1-character-and-entity-references.md#dt-entref "Entity Reference"), [character
references](011-4-1-character-and-entity-references.md#dt-charref "Character Reference"), [comments](004-2-4-character-data-and-markup.md#dt-comment "Comment"), [CDATA section](004-2-4-character-data-and-markup.md#dt-cdsection "CDATA Section") delimiters, [document
type declarations](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration"), [processing instructions](004-2-4-character-data-and-markup.md#dt-pi "Processing instruction"), [XML declarations](005-2-8-prolog-and-document-type-declaration.md#NT-XMLDecl), [text declarations](012-4-3-parsed-entities.md#NT-TextDecl),
and any white space that is at the top level of the document entity (that
is, outside the document element and not inside any other markup).]

<a id="dt-chardata"></a>

[Definition: All text that is not markup
constitutes the **character data** of the document.]

The ampersand character (&) and the left angle bracket (<) *MUST NOT* appear
in their literal form, except when used as markup delimiters, or
within a [comment](004-2-4-character-data-and-markup.md#dt-comment "Comment"), a [processing
instruction](004-2-4-character-data-and-markup.md#dt-pi "Processing instruction"), or a [CDATA section](004-2-4-character-data-and-markup.md#dt-cdsection "CDATA Section").
If they are needed elsewhere, they *MUST* be [escaped](014-4-5-construction-of-entity-replacement-text.md#dt-escape "escape")
using either [numeric character references](011-4-1-character-and-entity-references.md#dt-charref "Character Reference")
or the strings "
`&amp;`
" and "
`&lt;`
"
respectively. The right angle bracket (>) may be represented using the string "
`&gt;`
",
and *MUST*, [for compatibility](002-appendices.md#dt-compat "For Compatibility"), be escaped
using either "
`&gt;`
" or a character reference when it
appears in the string "
`]]>`
" in content, when
that string is not marking the end of a [CDATA
section](004-2-4-character-data-and-markup.md#dt-cdsection "CDATA Section").

In the content of elements, character data is any string of characters
which does not contain the start-delimiter of any markup and does not include the CDATA-section-close
delimiter, "
`]]>`
". In a CDATA section,
character data is any string of characters not including the CDATA-section-close
delimiter, "
`]]>`
".

To allow attribute values to contain both single and double quotes, the
apostrophe or single-quote character (') may be represented as "
`&apos;`
",
and the double-quote character (") as "
`&quot;`
".

<a id="d0e1106"></a>

##### Character Data

<a id="NT-CharData"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [14] | `CharData` | ::= | `[^<&]* - ([^<&]* ']]>' [^<&]*)` |

<a id="sec-comments"></a>

### 2.5 Comments

<a id="dt-comment"></a>

[Definition:
**Comments** may appear
anywhere in a document outside other [markup](004-2-4-character-data-and-markup.md#dt-markup "Markup");
in addition, they may appear within the document type declaration at places
allowed by the grammar. They are not part of the document's [character
data](004-2-4-character-data-and-markup.md#dt-chardata "Character Data"); an XML processor *MAY*, but need not, make it possible for an
application to retrieve the text of comments. [For
compatibility](002-appendices.md#dt-compat "For Compatibility"), the string "
`--`
" (double-hyphen)
*MUST NOT* occur within comments.] Parameter
entity references *MUST NOT* be recognized within comments.

<a id="d0e1149"></a>

##### Comments

<a id="NT-Comment"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [15] | `Comment` | ::= | `'<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'` |

An example of a comment:



```
<!-- declarations for <head> & <body> -->
```




Note
that the grammar does not allow a comment ending in `--->`. The
following example is *not* well-formed.



```
<!-- B+, B, or B--->
```




<a id="sec-pi"></a>

### 2.6 Processing Instructions

<a id="dt-pi"></a>

[Definition:
**Processing instructions**
(PIs) allow documents to contain instructions for applications.]

<a id="d0e1188"></a>

##### Processing Instructions

<a id="NT-PI"></a>

<a id="NT-PITarget"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [16] | `PI` | ::= | `'<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'` |
|  |  |  |  |
| --- | --- | --- | --- |
| [17] | `PITarget` | ::= | `Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))` |

PIs are not part of the document's [character
data](004-2-4-character-data-and-markup.md#dt-chardata "Character Data"), but *MUST* be passed through to the application. The PI begins
with a target ([PITarget](004-2-4-character-data-and-markup.md#NT-PITarget)) used to identify the application
to which the instruction is directed. The target names "
`XML`
", "
`xml`
",
and so on are reserved for standardization in this or future versions of this
specification. The XML [Notation](014-4-5-construction-of-entity-replacement-text.md#dt-notation "Notation") mechanism
may be used for formal declaration of PI targets. Parameter
entity references *MUST NOT* be recognized within processing instructions.

<a id="sec-cdata-sect"></a>

### 2.7 CDATA Sections

<a id="dt-cdsection"></a>

[Definition:
**CDATA sections** may occur anywhere character data may occur; they are used to escape blocks
of text containing characters which would otherwise be recognized as markup.
CDATA sections begin with the string "
`<![CDATA[`
"
and end with the string "
`]]>`
":]

<a id="d0e1271"></a>

##### CDATA Sections

<a id="NT-CDSect"></a>

<a id="NT-CDStart"></a>

<a id="NT-CData"></a>

<a id="NT-CDEnd"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [18] | `CDSect` | ::= | `CDStart CData CDEnd` |
|  |  |  |  |
| --- | --- | --- | --- |
| [19] | `CDStart` | ::= | `'<![CDATA['` |
|  |  |  |  |
| --- | --- | --- | --- |
| [20] | `CData` | ::= | `(Char* - (Char* ']]>' Char*))` |
|  |  |  |  |
| --- | --- | --- | --- |
| [21] | `CDEnd` | ::= | `']]>'` |

Within a CDATA section, only the [CDEnd](004-2-4-character-data-and-markup.md#NT-CDEnd) string is
recognized as markup, so that left angle brackets and ampersands may occur
in their literal form; they need not (and cannot) be escaped using "
`&lt;`
"
and "
`&amp;`
". CDATA sections cannot nest.

An example of a CDATA section, in which "
`<greeting>`
"
and "
`</greeting>`
" are recognized as [character data](004-2-4-character-data-and-markup.md#dt-chardata "Character Data"), not [markup](004-2-4-character-data-and-markup.md#dt-markup "Markup"):



```
<![CDATA[<greeting>Hello, world!</greeting>]]> 
```




