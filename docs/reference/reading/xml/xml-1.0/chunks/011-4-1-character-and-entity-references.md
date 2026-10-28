# 4.1 Character and Entity References

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-3-4-conditional-sections.md) · [Next](012-4-3-parsed-entities.md)

<a id="sec-references"></a>

### 4.1 Character and Entity References

<a id="dt-charref"></a>

[Definition: A **character
reference** refers to a specific character in the ISO/IEC 10646 character
set, for example one not directly accessible from available input devices.]

<a id="d0e3895"></a>

##### Character Reference

<a id="NT-CharRef"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [66] | `CharRef` | ::= | `'&#' [0-9]+ ';'` |
|  |  |  | `| '&#x' [0-9a-fA-F]+ ';'` | [[WFC: Legal Character]](011-4-1-character-and-entity-references.md#wf-Legalchar) |

<a id="wf-Legalchar"></a>

**Well-formedness constraint: Legal Character**

Characters referred
to using character references *MUST* match the production for [Char](003-2-documents.md#NT-Char).

If the character reference begins with "
`&#x`
",
the digits and letters up to the terminating `;` provide a hexadecimal
representation of the character's code point in ISO/IEC 10646. If it begins
just with "
`&#`
", the digits up to the terminating `;`
provide a decimal representation of the character's code point.

<a id="dt-entref"></a>

<a id="dt-GERef"></a>

<a id="dt-PERef"></a>

[Definition: An **entity reference**
refers to the content of a named entity.]
[Definition: References to parsed general entities use
ampersand (`&`) and semicolon (`;`) as delimiters.]
[Definition:
**Parameter-entity references**
use percent-sign (`%`) and semicolon (`;`) as delimiters.]

<a id="d0e3966"></a>

##### Entity Reference

<a id="NT-Reference"></a>

<a id="NT-EntityRef"></a>

<a id="NT-PEReference"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [67] | `Reference` | ::= | `EntityRef | CharRef` |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [68] | `EntityRef` | ::= | `'&' Name ';'` | [[WFC: Entity Declared]](011-4-1-character-and-entity-references.md#wf-entdeclared) |
|  |  |  |  | [[VC: Entity Declared]](011-4-1-character-and-entity-references.md#vc-entdeclared) |
|  |  |  |  | [[WFC: Parsed Entity]](011-4-1-character-and-entity-references.md#textent) |
|  |  |  |  | [[WFC: No Recursion]](011-4-1-character-and-entity-references.md#norecursion) |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [69] | `PEReference` | ::= | `'%' Name ';'` | [[VC: Entity Declared]](011-4-1-character-and-entity-references.md#vc-entdeclared) |
|  |  |  |  | [[WFC: No Recursion]](011-4-1-character-and-entity-references.md#norecursion) |
|  |  |  |  | [[WFC: In DTD]](011-4-1-character-and-entity-references.md#indtd) |

<a id="wf-entdeclared"></a>

**Well-formedness constraint: Entity Declared**

In a document
without any DTD, a document with only an internal DTD subset which contains
no parameter entity references, or a document with "
`standalone='yes'`
", for
an entity reference that does not occur within the external subset or a parameter
entity, the [Name](003-2-documents.md#NT-Name) given in the entity reference *MUST*
[match](002-appendices.md#dt-match "match") that in an [entity
declaration](011-4-1-character-and-entity-references.md#sec-entity-decl) that does not occur within the external subset or a
parameter entity, except that well-formed documents need not declare
any of the following entities: `amp`,
`lt`,
`gt`,
`apos`,
`quot`. The
declaration of a general entity *MUST* precede any reference to it which appears
in a default value in an attribute-list declaration.

Note that non-validating processors are [not
obligated to](013-4-4-xml-processor-treatment-of-entities-and-references.md#include-if-valid) read and process entity declarations occurring in parameter entities or in
the external subset; for such documents,
the rule that an entity must be declared is a well-formedness constraint only
if [standalone='yes'](006-2-9-standalone-document-declaration.md#sec-rmd).

<a id="vc-entdeclared"></a>

**Validity constraint: Entity Declared**

In a document with an external subset or parameter entity references,
if the document is not standalone (either "`standalone='no'`"
is specified or there is no standalone declaration), then
the [Name](003-2-documents.md#NT-Name) given in the entity reference *MUST*
[match](002-appendices.md#dt-match "match") that in an [entity
declaration](011-4-1-character-and-entity-references.md#sec-entity-decl). For interoperability, valid documents *SHOULD* declare
the entities `amp`,
`lt`,
`gt`,
`apos`,
`quot`, in the form specified in [**4.6 Predefined Entities**](014-4-5-construction-of-entity-replacement-text.md#sec-predefined-ent).
The declaration of a parameter entity *MUST* precede any reference to it. Similarly,
the declaration of a general entity *MUST* precede any attribute-list
declaration containing a default value with a direct or indirect reference
to that general entity.

<a id="textent"></a>

**Well-formedness constraint: Parsed Entity**

An entity reference *MUST
NOT* contain the name of an [unparsed entity](010-3-4-conditional-sections.md#dt-unparsed "Unparsed Entity").
Unparsed entities may be referred to only in [attribute
values](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-attrval "Attribute Value") declared to be of type **ENTITY** or **ENTITIES**.

<a id="norecursion"></a>

**Well-formedness constraint: No Recursion**

A parsed entity *MUST NOT* contain a recursive reference to itself, either directly or indirectly.

<a id="indtd"></a>

**Well-formedness constraint: In DTD**

Parameter-entity references *MUST NOT* appear outside
the [DTD](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration").

Examples of character and entity references:



```
Type <key>less-than</key> (&#x3C;) to save options.
This document was prepared on &docdate; and
is classified &security-level;.
```




Example of a parameter-entity reference:



```
<!-- declare the parameter entity "ISOLat2"... -->
<!ENTITY % ISOLat2
         SYSTEM "http://www.xml.com/iso/isolat2-xml.entities" >
<!-- ... now reference it. -->
%ISOLat2;
```




<a id="sec-entity-decl"></a>

### 4.2 Entity Declarations

<a id="dt-entdecl"></a>

[Definition: Entities are declared
thus:]

<a id="d0e4163"></a>

##### Entity Declaration

<a id="NT-EntityDecl"></a>

<a id="NT-GEDecl"></a>

<a id="NT-PEDecl"></a>

<a id="NT-EntityDef"></a>

<a id="NT-PEDef"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [70] | `EntityDecl` | ::= | `GEDecl | PEDecl` |
| [71] | `GEDecl` | ::= | `'<!ENTITY' S Name S EntityDef S? '>'` |
| [72] | `PEDecl` | ::= | `'<!ENTITY' S '%' S Name S PEDef S? '>'` |
| [73] | `EntityDef` | ::= | `EntityValue | (ExternalID NDataDecl?)` |
| [74] | `PEDef` | ::= | `EntityValue | ExternalID` |

The [Name](003-2-documents.md#NT-Name) identifies the entity in an [entity
reference](011-4-1-character-and-entity-references.md#dt-entref "Entity Reference") or, in the case of an unparsed entity, in the value of
an **ENTITY** or **ENTITIES** attribute. If the same entity is declared
more than once, the first declaration encountered is binding; at user option,
an XML processor *MAY* issue a warning if entities are declared multiple times.

<a id="sec-internal-ent"></a>

#### 4.2.1 Internal Entities

<a id="dt-internent"></a>

[Definition: If the
entity definition is an [EntityValue](003-2-documents.md#NT-EntityValue), the defined
entity is called an **internal entity**. There is no separate physical
storage object, and the content of the entity is given in the declaration.]
Note that some processing of entity and character references in the [literal entity value](014-4-5-construction-of-entity-replacement-text.md#dt-litentval "Literal Entity Value") may be required to produce
the correct [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text"): see [**4.5 Construction of Entity Replacement Text**](014-4-5-construction-of-entity-replacement-text.md#intern-replacement).

An internal entity is a [parsed entity](010-3-4-conditional-sections.md#dt-parsedent "Text Entity").

Example of an internal entity declaration:



```
<!ENTITY Pub-Status "This is a pre-release of the
 specification.">
```




<a id="sec-external-ent"></a>

#### 4.2.2 External Entities

<a id="dt-extent"></a>

[Definition: If the entity is not internal,
it is an **external entity**, declared as follows:]

<a id="d0e4313"></a>

##### External Entity Declaration

<a id="NT-ExternalID"></a>

<a id="NT-NDataDecl"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [75] | `ExternalID` | ::= | `'SYSTEM' S SystemLiteral` |
|  |  |  | `| 'PUBLIC' S PubidLiteral S SystemLiteral` |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [76] | `NDataDecl` | ::= | `S 'NDATA' S Name` | [[VC: Notation Declared]](011-4-1-character-and-entity-references.md#not-declared) |

If the [NDataDecl](011-4-1-character-and-entity-references.md#NT-NDataDecl) is present, this is a general [unparsed entity](010-3-4-conditional-sections.md#dt-unparsed "Unparsed Entity"); otherwise it is a parsed entity.

<a id="not-declared"></a>

**Validity constraint: Notation Declared**

The [Name](003-2-documents.md#NT-Name)
*MUST* match the declared name of a [notation](014-4-5-construction-of-entity-replacement-text.md#dt-notation "Notation").

<a id="dt-sysid"></a>

[Definition: The [SystemLiteral](003-2-documents.md#NT-SystemLiteral) is called the entity's **system
identifier**. It is meant to be converted to a URI reference
(as defined in [[IETF RFC 3986]](015-5-2-using-xml-processors.md#rfc3986)),
as part of the
process of dereferencing it to obtain input for the XML processor to construct the
entity's replacement text.] It is an error for a fragment identifier
(beginning with a `#` character) to be part of a system identifier.
Unless otherwise provided by information outside the scope of this specification
(e.g. a special XML element type defined by a particular DTD, or a processing
instruction defined by a particular application specification), relative URIs
are relative to the location of the resource within which the entity declaration
occurs. This is defined to
be the external entity containing the '<' which starts the declaration, at the
point when it is parsed as a declaration.
A URI might thus be relative to the [document
entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity"), to the entity containing the [external
DTD subset](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration"), or to some other [external parameter
entity](011-4-1-character-and-entity-references.md#dt-extent "External Entity"). Attempts to
retrieve the resource identified by a URI may be redirected at the parser
level (for example, in an entity resolver) or below (at the protocol level,
for example, via an HTTP `Location:` header). In the absence of additional
information outside the scope of this specification within the resource,
the base URI of a resource is always the URI of the actual resource returned.
In other words, it is the URI of the resource retrieved after all redirection
has occurred.

System
identifiers (and other XML strings meant to be used as URI references) may contain
characters that, according to [[IETF RFC 3986]](015-5-2-using-xml-processors.md#rfc3986),
must be escaped before a URI can be used to retrieve the referenced resource. The
characters to be escaped are the control characters #x0 to #x1F and #x7F (most of
which cannot appear in XML), space #x20, the delimiters '<' #x3C, '>' #x3E and
'"' #x22, the *unwise* characters '{' #x7B, '}' #x7D, '|' #x7C, '\' #x5C, '^' #x5E and
'`' #x60, as well as all characters above #x7F. Since escaping is not always a fully
reversible process, it *MUST* be performed only when absolutely necessary and as late
as possible in a processing chain. In particular, neither the process of converting
a relative URI to an absolute one nor the process of passing a URI reference to a
process or software component responsible for dereferencing it *SHOULD* trigger escaping.
When escaping does occur, it *MUST* be performed as follows:

1. Each character to be escaped is represented in UTF-8 [[Unicode]](015-5-2-using-xml-processors.md#Unicode)
   as one or more bytes.
2. The resulting bytes are escaped with
   the URI escaping mechanism (that is, converted to `%`
   HH,
   where HH is the hexadecimal notation of the byte value).
3. The original character is replaced by the resulting character sequence.

**Note:**

In a future edition of this specification, the XML Core Working Group intends to replace the preceding paragraph
and list of steps with a normative reference to an upcoming revision of IETF RFC 3987, which will define
"Legacy Extended IRIs (LEIRIs)". When this revision is available, it is the intent of the XML Core WG to use it to replace
language similar to the above in any future revisions of XML-related specifications under its purview.

<a id="dt-pubid"></a>

[Definition: In addition to a system
identifier, an external identifier may include a **public identifier**.]
An XML processor attempting to retrieve the entity's content may use
any combination of
the public and system identifiers as well as additional information outside the
scope of this specification to try to generate an alternative URI reference.
If the processor is unable to do so, it *MUST* use the URI
reference specified in the system literal. Before a match is attempted,
all strings of white space in the public identifier *MUST* be normalized to
single space characters (#x20), and leading and trailing white space *MUST*
be removed.

Examples of external entity declarations:



