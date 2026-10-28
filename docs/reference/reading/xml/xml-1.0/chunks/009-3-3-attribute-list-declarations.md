# 3.3 Attribute-List Declarations

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-3-1-start-tags-end-tags-and-empty-element-tags.md) · [Next](010-3-4-conditional-sections.md)

<a id="attdecls"></a>

### 3.3 Attribute-List Declarations

[Attributes](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-attr "Attribute") are used to associate name-value
pairs with [elements](007-2-12-language-identification.md#dt-element "Element"). Attribute specifications
*MUST NOT* appear outside of [start-tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag") and [empty-element tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-eetag "empty-element tag"); thus, the productions used to
recognize them appear in [**3.1 Start-Tags, End-Tags, and Empty-Element Tags**](008-3-1-start-tags-end-tags-and-empty-element-tags.md#sec-starttags). Attribute-list declarations
may be used:

- To define the set of attributes pertaining to a given element type.
- To establish type constraints for these attributes.
- To provide [default values](009-3-3-attribute-list-declarations.md#dt-default "Attribute Default") for
  attributes.

<a id="dt-attdecl"></a>

[Definition:
**Attribute-list
declarations** specify the name, data type, and default value (if any)
of each attribute associated with a given element type:]

<a id="d0e2926"></a>

##### Attribute-list Declaration

<a id="NT-AttlistDecl"></a>

<a id="NT-AttDef"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [52] | `AttlistDecl` | ::= | `'<!ATTLIST' S Name AttDef* S? '>'` |
|  |  |  |  |
| --- | --- | --- | --- |
| [53] | `AttDef` | ::= | `S Name S AttType S DefaultDecl` |

The [Name](003-2-documents.md#NT-Name) in the [AttlistDecl](009-3-3-attribute-list-declarations.md#NT-AttlistDecl)
rule is the type of an element. At user option, an XML processor *MAY* issue
a warning if attributes are declared for an element type not itself declared,
but this is not an error. The [Name](003-2-documents.md#NT-Name) in the [AttDef](009-3-3-attribute-list-declarations.md#NT-AttDef)
rule is the name of the attribute.

When more than one [AttlistDecl](009-3-3-attribute-list-declarations.md#NT-AttlistDecl) is provided
for a given element type, the contents of all those provided are merged. When
more than one definition is provided for the same attribute of a given element
type, the first declaration is binding and later declarations are ignored. [For interoperability,](002-appendices.md#dt-interop "For interoperability") writers of DTDs may choose
to provide at most one attribute-list declaration for a given element type,
at most one attribute definition for a given attribute name in an attribute-list
declaration, and at least one attribute definition in each attribute-list
declaration. For interoperability, an XML processor *MAY* at user option
issue a warning when more than one attribute-list declaration is provided
for a given element type, or more than one attribute definition is provided
for a given attribute, but this is not an error.

<a id="sec-attribute-types"></a>

#### 3.3.1 Attribute Types

XML attribute types are of three kinds: a string type, a set of tokenized
types, and enumerated types. The string type may take any literal string as
a value; the tokenized types are more constrained.
The validity constraints noted in the grammar are applied after the attribute
value has been normalized as described in [**3.3.3 Attribute-Value Normalization**](009-3-3-attribute-list-declarations.md#AVNormalize).

<a id="d0e3004"></a>

##### Attribute Types

<a id="NT-AttType"></a>

<a id="NT-StringType"></a>

<a id="NT-TokenizedType"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [54] | `AttType` | ::= | `StringType | TokenizedType | EnumeratedType` |
| [55] | `StringType` | ::= | `'CDATA'` |
| [56] | `TokenizedType` | ::= | `'ID'` | [[VC: ID]](009-3-3-attribute-list-declarations.md#id) |
|  |  |  |  | [[VC: One ID per Element Type]](009-3-3-attribute-list-declarations.md#one-id-per-el) |
|  |  |  |  | [[VC: ID Attribute Default]](009-3-3-attribute-list-declarations.md#id-default) |
|  |  |  | `| 'IDREF'` | [[VC: IDREF]](009-3-3-attribute-list-declarations.md#idref) |
|  |  |  | `| 'IDREFS'` | [[VC: IDREF]](009-3-3-attribute-list-declarations.md#idref) |
|  |  |  | `| 'ENTITY'` | [[VC: Entity Name]](009-3-3-attribute-list-declarations.md#entname) |
|  |  |  | `| 'ENTITIES'` | [[VC: Entity Name]](009-3-3-attribute-list-declarations.md#entname) |
|  |  |  | `| 'NMTOKEN'` | [[VC: Name Token]](009-3-3-attribute-list-declarations.md#nmtok) |
|  |  |  | `| 'NMTOKENS'` | [[VC: Name Token]](009-3-3-attribute-list-declarations.md#nmtok) |

<a id="id"></a>

**Validity constraint: ID**

Values of type **ID**
*MUST* match the [Name](003-2-documents.md#NT-Name) production. A name *MUST NOT* appear more than once
in an XML document as a value of this type; i.e., ID values *MUST* uniquely
identify the elements which bear them.

<a id="one-id-per-el"></a>

**Validity constraint: One ID per Element Type**

An element type *MUST NOT* have more than one ID attribute specified.

<a id="id-default"></a>

**Validity constraint: ID Attribute Default**

An ID attribute
*MUST* have a declared default of **#IMPLIED** or **#REQUIRED**.

<a id="idref"></a>

**Validity constraint: IDREF**

Values of type **IDREF**
*MUST*
match the [Name](003-2-documents.md#NT-Name) production, and values of type **IDREFS**
*MUST* match [Names](003-2-documents.md#NT-Names); each [Name](003-2-documents.md#NT-Name)
*MUST* match the value of an ID attribute on some element in the XML document;
i.e. **IDREF** values *MUST* match the value of some ID attribute.

<a id="entname"></a>

**Validity constraint: Entity Name**

Values of type **ENTITY**
*MUST* match the [Name](003-2-documents.md#NT-Name) production, values of type **ENTITIES**
*MUST* match [Names](003-2-documents.md#NT-Names); each [Name](003-2-documents.md#NT-Name)
*MUST* match the name of an [unparsed entity](010-3-4-conditional-sections.md#dt-unparsed "Unparsed Entity")
declared in the [DTD](005-2-8-prolog-and-document-type-declaration.md#dt-doctype "Document Type Declaration").

<a id="nmtok"></a>

**Validity constraint: Name Token**

Values of type **NMTOKEN**
*MUST* match the [Nmtoken](003-2-documents.md#NT-Nmtoken) production; values of type **NMTOKENS**
*MUST* match [Nmtokens](003-2-documents.md#NT-Nmtokens).

<a id="dt-enumerated"></a>

[Definition:
**Enumerated attributes**
have a list of allowed values in their declaration
]. They *MUST* take one of those values. There are two kinds of enumerated attribute types:

<a id="d0e3200"></a>

##### Enumerated Attribute Types

<a id="NT-EnumeratedType"></a>

<a id="NT-NotationType"></a>

<a id="NT-Enumeration"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [57] | `EnumeratedType` | ::= | `NotationType | Enumeration` |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [58] | `NotationType` | ::= | `'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'` | [[VC: Notation Attributes]](009-3-3-attribute-list-declarations.md#notatn) |
|  |  |  |  | [[VC: One Notation Per Element Type]](009-3-3-attribute-list-declarations.md#OneNotationPer) |
|  |  |  |  | [[VC: No Notation on Empty Element]](009-3-3-attribute-list-declarations.md#NoNotationEmpty) |
|  |  |  |  | [[VC: No Duplicate Tokens]](009-3-3-attribute-list-declarations.md#NoDuplicateTokens) |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [59] | `Enumeration` | ::= | `'(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'` | [[VC: Enumeration]](009-3-3-attribute-list-declarations.md#enum) |
|  |  |  |  | [[VC: No Duplicate Tokens]](009-3-3-attribute-list-declarations.md#NoDuplicateTokens) |

A **NOTATION** attribute identifies a [notation](014-4-5-construction-of-entity-replacement-text.md#dt-notation "Notation"),
declared in the DTD with associated system and/or public identifiers, to be
used in interpreting the element to which the attribute is attached.

<a id="notatn"></a>

**Validity constraint: Notation Attributes**

Values of this type
*MUST* match one of the [notation](014-4-5-construction-of-entity-replacement-text.md#Notations) names
included in the declaration; all notation names in the declaration *MUST* be
declared.

<a id="OneNotationPer"></a>

**Validity constraint: One Notation Per Element Type**

An element type *MUST NOT* have more than one **NOTATION**
attribute specified.

<a id="NoNotationEmpty"></a>

**Validity constraint: No Notation on Empty Element**

[For compatibility](002-appendices.md#dt-compat "For Compatibility"),
an attribute of type **NOTATION**
*MUST NOT* be declared on an element
declared **EMPTY**.

<a id="NoDuplicateTokens"></a>

**Validity constraint: No Duplicate Tokens**

The notation names in a single [NotationType](009-3-3-attribute-list-declarations.md#NT-NotationType)
attribute declaration, as well as the [NmToken](003-2-documents.md#NT-Nmtoken)s in a single
[Enumeration](009-3-3-attribute-list-declarations.md#NT-Enumeration) attribute declaration, *MUST* all be distinct.

<a id="enum"></a>

**Validity constraint: Enumeration**

Values of this type *MUST* match
one of the [Nmtoken](003-2-documents.md#NT-Nmtoken) tokens in the declaration.

[For interoperability,](002-appendices.md#dt-interop "For interoperability") the same [Nmtoken](003-2-documents.md#NT-Nmtoken)
*SHOULD NOT* occur more than once in the enumerated
attribute types of a single element type.

<a id="sec-attr-defaults"></a>

#### 3.3.2 Attribute Defaults

An [attribute declaration](009-3-3-attribute-list-declarations.md#dt-attdecl "Attribute-List Declaration") provides information
on whether the attribute's presence is *REQUIRED*, and if not, how an XML processor
is to react if a declared attribute is absent in a document.

<a id="d0e3369"></a>

##### Attribute Defaults

<a id="NT-DefaultDecl"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [60] | `DefaultDecl` | ::= | `'#REQUIRED' | '#IMPLIED'` |
|  |  |  | `| (('#FIXED' S)? AttValue)` | [[VC: Required Attribute]](009-3-3-attribute-list-declarations.md#RequiredAttr) |
|  |  |  |  | [[VC: Attribute Default Value Syntactically Correct]](009-3-3-attribute-list-declarations.md#defattrvalid) |
|  |  |  |  | [[WFC: No < in Attribute Values]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#CleanAttrVals) |
|  |  |  |  | [[VC: Fixed Attribute Default]](009-3-3-attribute-list-declarations.md#FixedAttr) |
|  |  |  |  | [[WFC: No External Entity References]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NoExternalRefs) |

<a id="dt-default"></a>

In an attribute declaration, **#REQUIRED** means that the attribute
*MUST* always be provided, **#IMPLIED** that no default value is provided.
[Definition: If
the declaration is neither **#REQUIRED** nor **#IMPLIED**, then
the [AttValue](003-2-documents.md#NT-AttValue) value contains the declared **default**
value; the **#FIXED** keyword states that the attribute *MUST* always have
the default value.
When an XML processor encounters
an element
without a specification for an attribute for which it has read a default
value declaration, it *MUST* report the attribute with the declared default
value to the application.]

<a id="RequiredAttr"></a>

**Validity constraint: Required Attribute**

If the default
declaration is the keyword **#REQUIRED**, then the attribute *MUST* be
specified for all elements of the type in the attribute-list declaration.

<a id="defattrvalid"></a>

**Validity constraint: Attribute Default Value Syntactically Correct**

The declared default value *MUST* meet the syntactic
constraints of the declared attribute type. That is, the default value of an attribute:

- of type IDREF or ENTITY must match the [Name](003-2-documents.md#NT-Name) production;
- of type IDREFS or ENTITIES must match the [Names](003-2-documents.md#NT-Names) production;
- of type NMTOKEN must match the [Nmtoken](003-2-documents.md#NT-Nmtoken) production;
- of type NMTOKENS must match the [Nmtokens](003-2-documents.md#NT-Nmtokens) production;
- of an [enumerated type](009-3-3-attribute-list-declarations.md#NT-EnumeratedType) (either a [NOTATION](009-3-3-attribute-list-declarations.md#NT-NotationType) type or an [enumeration](009-3-3-attribute-list-declarations.md#NT-Enumeration)) must match one of the enumerated values.

Note that only the
syntactic constraints of the type are required here; other constraints (e.g.
that the value be the name of a declared unparsed entity, for an attribute of
type ENTITY) will be reported by a validating
parser only if an element without a specification for this attribute
actually occurs.

<a id="FixedAttr"></a>

**Validity constraint: Fixed Attribute Default**

If an attribute
has a default value declared with the **#FIXED** keyword, instances of
that attribute *MUST* match the default value.

Examples of attribute-list declarations:



```
<!ATTLIST termdef
          id      ID      #REQUIRED
          name    CDATA   #IMPLIED>
<!ATTLIST list
          type    (bullets|ordered|glossary)  "ordered">
<!ATTLIST form
          method  CDATA   #FIXED "POST">
```




<a id="AVNormalize"></a>

#### 3.3.3 Attribute-Value Normalization

Before the value of an attribute is passed to the application or checked
for validity, the XML processor *MUST* normalize the attribute value by applying
the algorithm below, or by using some other method such that the value passed
to the application is the same as that produced by the algorithm.

