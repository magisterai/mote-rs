# 4.5 Construction of Entity Replacement Text

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-4-4-xml-processor-treatment-of-entities-and-references.md) · [Next](015-5-2-using-xml-processors.md)

<a id="intern-replacement"></a>

### 4.5 Construction of Entity Replacement Text

<a id="dt-litentval"></a>

<a id="dt-extlitentval"></a>

<a id="dt-repltext"></a>

<a id="dt-extrepltext"></a>

In discussing the treatment of entities, it is useful to distinguish
two forms of the entity's value.
[Definition: For an
internal entity, the **literal
entity value** is the quoted string actually present in the entity declaration,
corresponding to the non-terminal [EntityValue](003-2-documents.md#NT-EntityValue).]
[Definition: For an external entity, the **literal
entity value** is the exact text contained in the entity.]
[Definition: For an
internal entity, the **replacement text**
is the content of the entity, after replacement of character references and
parameter-entity references.]
[Definition: For
an external entity, the **replacement text** is the content of the entity,
after stripping the text declaration (leaving any surrounding whitespace) if there
is one but without any replacement of character references or parameter-entity
references.]

The literal entity value as given in an internal entity declaration ([EntityValue](003-2-documents.md#NT-EntityValue)) may contain character, parameter-entity,
and general-entity references. Such references *MUST* be contained entirely
within the literal entity value. The actual replacement text that is [included](013-4-4-xml-processor-treatment-of-entities-and-references.md#dt-include "Include") (or [included in literal](013-4-4-xml-processor-treatment-of-entities-and-references.md#inliteral)) as described above
*MUST* contain the *replacement
text* of any parameter entities referred to, and *MUST* contain the character
referred to, in place of any character references in the literal entity value;
however, general-entity references *MUST* be left as-is, unexpanded. For example,
given the following declarations:



```
<!ENTITY % pub    "&#xc9;ditions Gallimard" >
<!ENTITY   rights "All rights reserved" >
<!ENTITY   book   "La Peste: Albert Camus,
&#xA9; 1947 %pub;. &rights;" >
```




then the replacement text for the entity "
`book`
"
is:



```
La Peste: Albert Camus,
© 1947 Éditions Gallimard. &rights;
```




The general-entity reference "
`&rights;`
" would
be expanded should the reference "
`&book;`
" appear
in the document's content or an attribute value.

These simple rules may have complex interactions; for a detailed discussion
of a difficult example, see [**D Expansion of Entity and Character References**](017-c-xml-and-sgml-non-normative.md#sec-entexpand).

<a id="sec-predefined-ent"></a>

### 4.6 Predefined Entities

<a id="dt-escape"></a>

[Definition: Entity and character references may
both be used to **escape** the left angle bracket, ampersand, and
other delimiters. A set of general entities (`amp`,
`lt`,
`gt`,
`apos`,
`quot`) is specified for
this purpose. Numeric character references may also be used; they are expanded
immediately when recognized and *MUST* be treated as character data, so the
numeric character references "
`&#60;`
" and "
`&#38;`
" may be used to escape `<` and `&` when they occur
in character data.]

All XML processors *MUST* recognize these entities whether they are declared
or not. [For interoperability](002-appendices.md#dt-interop "For interoperability"), valid XML
documents *SHOULD* declare these entities, like any others, before using them. If
the entities `lt` or `amp` are declared, they *MUST* be
declared as internal entities whose replacement text is a character reference
to the respective
character (less-than sign or ampersand) being escaped; the double
escaping is *REQUIRED* for these entities so that references to them produce
a well-formed result. If the entities `gt`, `apos`,
or `quot` are declared, they *MUST* be declared as internal entities
whose replacement text is the single character being escaped (or a character
reference to that character; the double escaping here is *OPTIONAL* but harmless).
For example:



```
<!ENTITY lt     "&#38;#60;">
<!ENTITY gt     "&#62;">
<!ENTITY amp    "&#38;#38;">
<!ENTITY apos   "&#39;">
<!ENTITY quot   "&#34;">
```




<a id="Notations"></a>

### 4.7 Notation Declarations

<a id="dt-notation"></a>

[Definition:
**Notations** identify
by name the format of [unparsed entities](010-3-4-conditional-sections.md#dt-unparsed "Unparsed Entity"),
the format of elements which bear a notation attribute, or the application
to which a [processing instruction](004-2-4-character-data-and-markup.md#dt-pi "Processing instruction") is addressed.]

<a id="dt-notdecl"></a>

[Definition:
**Notation declarations**
provide a name for the notation, for use in entity and attribute-list declarations
and in attribute specifications, and an external identifier for the notation
which may allow an XML processor or its client application to locate a helper
application capable of processing data in the given notation.]

<a id="d0e5457"></a>

##### Notation Declarations

<a id="NT-NotationDecl"></a>

<a id="NT-PublicID"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [82] | `NotationDecl` | ::= | `'<!NOTATION' S Name S (ExternalID | PublicID) S? '>'` | [[VC: Unique Notation Name]](014-4-5-construction-of-entity-replacement-text.md#UniqueNotationName) |
|  |  |  |  |
| --- | --- | --- | --- |
| [83] | `PublicID` | ::= | `'PUBLIC' S PubidLiteral` |

<a id="UniqueNotationName"></a>

**Validity constraint: Unique Notation Name**

A given [Name](003-2-documents.md#NT-Name)
*MUST NOT* be declared in more than one notation declaration.

XML processors *MUST* provide applications with the name and external identifier(s)
of any notation declared and referred to in an attribute value, attribute
definition, or entity declaration. They *MAY* additionally resolve the external
identifier into the [system identifier](011-4-1-character-and-entity-references.md#dt-sysid "System Identifier"), file
name, or other information needed to allow the application to call a processor
for data in the notation described. (It is not an error, however, for XML
documents to declare and refer to notations for which notation-specific applications
are not available on the system where the XML processor or application is
running.)

<a id="sec-doc-entity"></a>

### 4.8 Document Entity

<a id="dt-docent"></a>

[Definition: The **document entity**
serves as the root of the entity tree and a starting-point for an [XML processor](002-appendices.md#dt-xml-proc "XML Processor").] This specification does
not specify how the document entity is to be located by an XML processor;
unlike other entities, the document entity has no name and might well appear
on a processor input stream without any identification at all.

<a id="sec-conformance"></a>

## 5 Conformance

<a id="proc-types"></a>

### 5.1 Validating and Non-Validating Processors

Conforming [XML processors](002-appendices.md#dt-xml-proc "XML Processor") fall into
two classes: validating and non-validating.

Validating and non-validating processors alike *MUST* report violations of
this specification's well-formedness constraints in the content of the [document entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity") and any other [parsed
entities](010-3-4-conditional-sections.md#dt-parsedent "Text Entity") that they read.

<a id="dt-validating"></a>

[Definition:
**Validating
processors**
*MUST*,
at user option, report violations of the constraints expressed by
the declarations in the [DTD](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration"), and failures
to fulfill the validity constraints given in this specification.]
To accomplish this, validating XML processors *MUST* read and process the entire
DTD and all external parsed entities referenced in the document.

<a id="dt-use-mdecl"></a>

Non-validating processors are *REQUIRED* to check only the [document
entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity"), including the entire internal DTD subset, for well-formedness. [Definition: While they are not required
to check the document for validity, they are *REQUIRED* to **process**
all the declarations they read in the internal DTD subset and in any parameter
entity that they read, up to the first reference to a parameter entity that
they do *not* read; that is to say, they *MUST* use the information
in those declarations to [normalize](009-3-3-attribute-list-declarations.md#AVNormalize)
attribute values, [include](013-4-4-xml-processor-treatment-of-entities-and-references.md#included) the replacement
text of internal entities, and supply [default
attribute values](009-3-3-attribute-list-declarations.md#sec-attr-defaults).] Except when `standalone="yes"`, they
*MUST NOT*
[process](014-4-5-construction-of-entity-replacement-text.md#dt-use-mdecl "Process Declarations")
[entity
declarations](011-4-1-character-and-entity-references.md#dt-entdecl "entity declaration") or [attribute-list declarations](009-3-3-attribute-list-declarations.md#dt-attdecl "Attribute-List Declaration")
encountered after a reference to a parameter entity that is not read, since
the entity may have contained overriding declarations; when `standalone="yes"`, processors *MUST*
process these declarations.

Note that when processing invalid documents with a non-validating
processor the application may not be presented with consistent
information. For example, several requirements for uniqueness
within the document may not be met, including more than one element
with the same id, duplicate declarations of elements or notations
with the same name, etc. In these cases the behavior of the parser
with respect to reporting such information to the application is
undefined.

