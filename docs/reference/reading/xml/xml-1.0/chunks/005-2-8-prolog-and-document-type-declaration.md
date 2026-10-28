# 2.8 Prolog and Document Type Declaration

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-2-4-character-data-and-markup.md) · [Next](006-2-9-standalone-document-declaration.md)

<a id="sec-prolog-dtd"></a>

### 2.8 Prolog and Document Type Declaration

<a id="dt-xmldecl"></a>

[Definition: XML documents *SHOULD*
begin with an **XML declaration** which specifies the version of
XML being used.] For example, the following is a complete XML document, [well-formed](003-2-documents.md#dt-wellformed "Well-Formed") but not [valid](005-2-8-prolog-and-document-type-declaration.md#dt-valid "Validity"):



```
<?xml version="1.0"?>
<greeting>Hello, world!</greeting> 
```




and so is this:



```
<greeting>Hello, world!</greeting>
```




<a id="dt-valid"></a>

The function of the markup in an XML document is to describe its storage and
logical structure and to associate attribute
name-value pairs with its logical structures. XML provides a mechanism, the
[document
type declaration](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration"), to define constraints on the logical structure
and to support the use of predefined storage units. [Definition: An XML document is **valid** if it has an associated
document type declaration and if the document complies with the constraints
expressed in it.]

The document type declaration *MUST* appear before the first [element](007-2-12-language-identification.md#dt-element "Element")
in the document.

<a id="xmldoc"></a>

##### Prolog

<a id="NT-prolog"></a>

<a id="NT-XMLDecl"></a>

<a id="NT-VersionInfo"></a>

<a id="NT-Eq"></a>

<a id="NT-VersionNum"></a>

<a id="NT-Misc"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [22] | `prolog` | ::= | `XMLDecl? Misc* (doctypedecl Misc*)?` |
| [23] | `XMLDecl` | ::= | `'<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'` |
| [24] | `VersionInfo` | ::= | `S 'version' Eq ("'" VersionNum "'" | '"' VersionNum '"')` |
| [25] | `Eq` | ::= | `S? '=' S?` |
| [26] | `VersionNum` | ::= | `'1.' [0-9]+` |
| [27] | `Misc` | ::= | `Comment | PI | S` |

Even though the [VersionNum](005-2-8-prolog-and-document-type-declaration.md#NT-VersionNum) production matches
any version number of the form '1.x', XML 1.0 documents *SHOULD NOT* specify a version number other than '1.0'.

**Note:**

When an XML 1.0 processor encounters a document that specifies
a 1.x version number other than '1.0', it will process it as
a 1.0 document. This means that an XML 1.0 processor will accept
1.x documents provided they do not use any non-1.0 features.

<a id="dt-doctype"></a>

[Definition: The XML **document
type declaration** contains or points to [markup
declarations](005-2-8-prolog-and-document-type-declaration.md#dt-markupdecl "markup declaration") that provide a grammar for a class of documents. This
grammar is known as a document type definition, or **DTD**. The document
type declaration can point to an external subset (a special kind of [external entity](011-4-1-character-and-entity-references.md#dt-extent "External Entity")) containing markup declarations,
or can contain the markup declarations directly in an internal subset, or
can do both. The DTD for a document consists of both subsets taken together.]

<a id="dt-markupdecl"></a>

[Definition: A **markup declaration**
is an [element type declaration](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-eldecl "Element Type declaration"), an [attribute-list declaration](009-3-3-attribute-list-declarations.md#dt-attdecl "Attribute-List Declaration"), an [entity
declaration](011-4-1-character-and-entity-references.md#dt-entdecl "entity declaration"), or a [notation declaration](014-4-5-construction-of-entity-replacement-text.md#dt-notdecl "Notation Declaration").]
These declarations may be contained in whole or in part within [parameter
entities](010-3-4-conditional-sections.md#dt-PE "Parameter entity"), as described in the well-formedness and validity constraints
below. For further
information, see [**4 Physical Structures**](010-3-4-conditional-sections.md#sec-physical-struct).

<a id="dtd"></a>

##### Document Type Definition

<a id="NT-doctypedecl"></a>

<a id="NT-DeclSep"></a>

<a id="NT-intSubset"></a>

<a id="NT-markupdecl"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [28] | `doctypedecl` | ::= | `'<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'` | [[VC: Root Element Type]](005-2-8-prolog-and-document-type-declaration.md#vc-roottype) |
|  |  |  |  | [[WFC: External Subset]](005-2-8-prolog-and-document-type-declaration.md#ExtSubset) |
| [28a] | `DeclSep` | ::= | `PEReference | S` | [[WFC: PE Between Declarations]](005-2-8-prolog-and-document-type-declaration.md#PE-between-Decls) |
| [28b] | `intSubset` | ::= | `(markupdecl | DeclSep)*` |
| [29] | `markupdecl` | ::= | `elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment` | [[VC: Proper Declaration/PE Nesting]](005-2-8-prolog-and-document-type-declaration.md#vc-PEinMarkupDecl) |
|  |  |  |  | [[WFC: PEs in Internal Subset]](005-2-8-prolog-and-document-type-declaration.md#wfc-PEinInternalSubset) |

Note
that it is possible to construct a well-formed document containing a [doctypedecl](005-2-8-prolog-and-document-type-declaration.md#NT-doctypedecl)
that neither points to an external subset nor contains an internal subset.

The markup declarations may be made up in whole or in part of the [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text") of [parameter
entities](010-3-4-conditional-sections.md#dt-PE "Parameter entity"). The productions later in this specification for individual
nonterminals ([elementdecl](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-elementdecl), [AttlistDecl](009-3-3-attribute-list-declarations.md#NT-AttlistDecl),
and so on) describe the declarations *after* all the parameter
entities have been [included](013-4-4-xml-processor-treatment-of-entities-and-references.md#dt-include "Include").

Parameter
entity references are recognized anywhere in the DTD (internal and external
subsets and external parameter entities), except in literals, processing instructions,
comments, and the contents of ignored conditional sections (see [**3.4 Conditional Sections**](010-3-4-conditional-sections.md#sec-condition-sect)).
They are also recognized in entity value literals. The use of parameter entities
in the internal subset is restricted as described below.

<a id="vc-roottype"></a>

**Validity constraint: Root Element Type**

The [Name](003-2-documents.md#NT-Name)
in the document type declaration *MUST* match the element type of the [root element](003-2-documents.md#dt-root "Root Element").

<a id="vc-PEinMarkupDecl"></a>

**Validity constraint: Proper Declaration/PE Nesting**

Parameter-entity [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text")
*MUST* be properly nested with markup declarations. That is to say, if either
the first character or the last character of a markup declaration ([markupdecl](005-2-8-prolog-and-document-type-declaration.md#NT-markupdecl)
above) is contained in the replacement text for a [parameter-entity
reference](011-4-1-character-and-entity-references.md#dt-PERef "Parameter-entity reference"), both *MUST* be contained in the same replacement text.

<a id="wfc-PEinInternalSubset"></a>

**Well-formedness constraint: PEs in Internal Subset**

In
the internal DTD subset, [parameter-entity references](011-4-1-character-and-entity-references.md#dt-PERef "Parameter-entity reference")
*MUST NOT* occur within markup declarations; they may occur where markup declarations can occur.
(This does not apply to references that occur in external parameter entities
or to the external subset.)

<a id="ExtSubset"></a>

**Well-formedness constraint: External Subset**

The external subset, if any, *MUST* match the production for [extSubset](005-2-8-prolog-and-document-type-declaration.md#NT-extSubset).

<a id="PE-between-Decls"></a>

**Well-formedness constraint: PE Between Declarations**

The replacement text of a parameter entity reference
in a [DeclSep](005-2-8-prolog-and-document-type-declaration.md#NT-DeclSep)
*MUST* match the production [extSubsetDecl](005-2-8-prolog-and-document-type-declaration.md#NT-extSubsetDecl).

Like the internal subset, the external subset and any external parameter
entities referenced
in a [DeclSep](005-2-8-prolog-and-document-type-declaration.md#NT-DeclSep)
*MUST* consist of a series of
complete markup declarations of the types allowed by the non-terminal symbol [markupdecl](005-2-8-prolog-and-document-type-declaration.md#NT-markupdecl), interspersed with white space or [parameter-entity references](011-4-1-character-and-entity-references.md#dt-PERef "Parameter-entity reference"). However, portions of
the contents of the external subset or of these
external parameter entities may conditionally be ignored by using the [conditional section](010-3-4-conditional-sections.md#dt-cond-section "conditional section") construct; this is not
allowed in the internal subset but is
allowed in external parameter entities referenced in the internal subset.

<a id="ext-Subset"></a>

##### External Subset

<a id="NT-extSubset"></a>

<a id="NT-extSubsetDecl"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [30] | `extSubset` | ::= | `TextDecl? extSubsetDecl` |
| [31] | `extSubsetDecl` | ::= | `( markupdecl | conditionalSect | DeclSep)*` |

The external subset and external parameter entities also differ from the
internal subset in that in them, [parameter-entity
references](011-4-1-character-and-entity-references.md#dt-PERef "Parameter-entity reference") are permitted *within* markup declarations,
not only *between* markup declarations.

An example of an XML document with a document type declaration:



```
<?xml version="1.0"?>
<!DOCTYPE greeting SYSTEM "hello.dtd">
<greeting>Hello, world!</greeting> 
```




The [system identifier](011-4-1-character-and-entity-references.md#dt-sysid "System Identifier")
"
`hello.dtd`
"
gives the address (a URI reference) of a DTD for the document.

The declarations can also be given locally, as in this example:



```
<?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE greeting [
  <!ELEMENT greeting (#PCDATA)>
]>
<greeting>Hello, world!</greeting>
```




If both the external and internal subsets are used, the internal subset
*MUST* be considered to occur before the external subset.
This has the effect that entity and attribute-list declarations in the internal
subset take precedence over those in the external subset.

