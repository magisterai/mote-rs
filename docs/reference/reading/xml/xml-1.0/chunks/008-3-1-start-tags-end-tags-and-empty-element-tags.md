# 3.1 Start-Tags, End-Tags, and Empty-Element Tags

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-2-12-language-identification.md) · [Next](009-3-3-attribute-list-declarations.md)

<a id="sec-starttags"></a>

### 3.1 Start-Tags, End-Tags, and Empty-Element Tags

<a id="dt-stag"></a>

[Definition: The beginning of every non-empty
XML element is marked by a **start-tag**.]

<a id="d0e2259"></a>

##### Start-tag

<a id="NT-STag"></a>

<a id="NT-Attribute"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [40] | `STag` | ::= | `'<' Name (S Attribute)* S? '>'` | [[WFC: Unique Att Spec]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#uniqattspec) |
| [41] | `Attribute` | ::= | `Name Eq AttValue` | [[VC: Attribute Value Type]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#ValueType) |
|  |  |  |  | [[WFC: No External Entity References]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NoExternalRefs) |
|  |  |  |  | [[WFC: No < in Attribute Values]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#CleanAttrVals) |

<a id="dt-attr"></a>

<a id="dt-attrname"></a>

<a id="dt-attrval"></a>

The [Name](003-2-documents.md#NT-Name) in the start- and end-tags gives the element's **type**. [Definition: The [Name](003-2-documents.md#NT-Name)-[AttValue](003-2-documents.md#NT-AttValue)
pairs are referred to as the **attribute specifications** of the
element], [Definition: with the [Name](003-2-documents.md#NT-Name) in each pair referred to as the **attribute name**
]
and [Definition: the content of the [AttValue](003-2-documents.md#NT-AttValue) (the text between the `'` or `"`
delimiters) as the **attribute value**.] Note
that the order of attribute specifications in a start-tag or empty-element
tag is not significant.

<a id="uniqattspec"></a>

**Well-formedness constraint: Unique Att Spec**

An attribute name
*MUST NOT* appear more than once in the same start-tag or empty-element tag.

<a id="ValueType"></a>

**Validity constraint: Attribute Value Type**

The attribute *MUST*
have been declared; the value *MUST* be of the type declared for it. (For attribute
types, see [**3.3 Attribute-List Declarations**](009-3-3-attribute-list-declarations.md#attdecls).)

<a id="NoExternalRefs"></a>

**Well-formedness constraint: No External Entity References**

Attribute
values *MUST NOT* contain direct or indirect entity references to external entities.

<a id="CleanAttrVals"></a>

**Well-formedness constraint: No `<` in Attribute Values**

The [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text") of any entity
referred to directly or indirectly in an attribute value *MUST NOT* contain a `<`.

An example of a start-tag:



```
<termdef id="dt-dog" term="dog">
```




<a id="dt-etag"></a>

[Definition: The end of every element that begins
with a start-tag *MUST* be marked by an **end-tag** containing a name
that echoes the element's type as given in the start-tag:]

<a id="d0e2403"></a>

##### End-tag

<a id="NT-ETag"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [42] | `ETag` | ::= | `'</' Name S? '>'` |

An example of an end-tag:



```
</termdef>
```




<a id="dt-content"></a>

[Definition: The [text](003-2-documents.md#dt-text "Text")
between the start-tag and end-tag is called the element's **content**:]

<a id="d0e2433"></a>

##### Content of Elements

<a id="NT-content"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [43] | `content` | ::= | `CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*` |

<a id="dt-empty"></a>

<a id="dt-eetag"></a>

[Definition: An element
with no [content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-content) is said to be **empty**.] The representation
of an empty element is either a start-tag immediately followed by an end-tag,
or an empty-element tag. [Definition: An **empty-element
tag** takes a special form:]

<a id="d0e2480"></a>

##### Tags for Empty Elements

<a id="NT-EmptyElemTag"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [44] | `EmptyElemTag` | ::= | `'<' Name (S Attribute)* S? '/>'` | [[WFC: Unique Att Spec]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#uniqattspec) |

Empty-element tags may be used for any element which has no content, whether
or not it is declared using the keyword **EMPTY**. [For
interoperability](002-appendices.md#dt-interop "For interoperability"), the empty-element tag *SHOULD*
be used, and *SHOULD* only be used, for elements which are declared
EMPTY.

Examples of empty elements:



```
<IMG align="left"
 src="http://www.w3.org/Icons/WWW/w3c_home" />
<br></br>
<br/>
```




<a id="elemdecls"></a>

### 3.2 Element Type Declarations

The [element](007-2-12-language-identification.md#dt-element "Element") structure of an [XML document](003-2-documents.md#dt-xml-doc "XML Document") may, for [validation](005-2-8-prolog-and-document-type-declaration.md#dt-valid "Validity")
purposes, be constrained using element type and attribute-list declarations.
An element type declaration constrains the element's [content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-content "Content").

Element type declarations often constrain which element types can appear
as [children](003-2-documents.md#dt-parentchild "Parent/Child") of the element. At user
option, an XML processor *MAY* issue a warning when a declaration mentions an
element type for which no declaration is provided, but this is not an error.

<a id="dt-eldecl"></a>

[Definition: An **element
type declaration** takes the form:]

<a id="d0e2553"></a>

##### Element Type Declaration

<a id="NT-elementdecl"></a>

<a id="NT-contentspec"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [45] | `elementdecl` | ::= | `'<!ELEMENT' S Name S contentspec S? '>'` | [[VC: Unique Element Type Declaration]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#EDUnique) |
| [46] | `contentspec` | ::= | `'EMPTY' | 'ANY' | Mixed | children` |

where the [Name](003-2-documents.md#NT-Name) gives the element type being declared.

<a id="EDUnique"></a>

**Validity constraint: Unique Element Type Declaration**

An element type *MUST NOT* be declared more than once.

Examples of element type declarations:



```
<!ELEMENT br EMPTY>
<!ELEMENT p (#PCDATA|emph)* >
<!ELEMENT %name.para; %content.para; >
<!ELEMENT container ANY>
```




<a id="sec-element-content"></a>

#### 3.2.1 Element Content

<a id="dt-elemcontent"></a>

<a id="dt-content-model"></a>

[Definition: An element [type](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag") has **element content** when elements
of that type *MUST* contain only [child](003-2-documents.md#dt-parentchild "Parent/Child")
elements (no character data), optionally separated by white space (characters
matching the nonterminal [S](003-2-documents.md#NT-S)).]
[Definition: In this case, the constraint includes a **content
model**, a simple grammar governing the allowed types of the
child elements and the order in which they are allowed to appear.]
The grammar is built on content particles ([cp](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-cp)s), which
consist of names, choice lists of content particles, or sequence lists of
content particles:

<a id="d0e2638"></a>

##### Element-content Models

<a id="NT-children"></a>

<a id="NT-cp"></a>

<a id="NT-choice"></a>

<a id="NT-seq"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [47] | `children` | ::= | `(choice | seq) ('?' | '*' | '+')?` |
| [48] | `cp` | ::= | `(Name | choice | seq) ('?' | '*' | '+')?` |
| [49] | `choice` | ::= | `'(' S? cp ( S? '|' S? cp )+ S? ')'` | [[VC: Proper Group/PE Nesting]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#vc-PEinGroup) |
| [50] | `seq` | ::= | `'(' S? cp ( S? ',' S? cp )* S? ')'` | [[VC: Proper Group/PE Nesting]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#vc-PEinGroup) |

where each [Name](003-2-documents.md#NT-Name) is the type of an element which
may appear as a [child](003-2-documents.md#dt-parentchild "Parent/Child"). Any content
particle in a choice list may appear in the [element
content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-elemcontent "Element content") at the location where the choice list appears in the grammar;
content particles occurring in a sequence list *MUST* each appear in the [element content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-elemcontent "Element content") in the order given in the list.
The optional character following a name or list governs whether the element
or the content particles in the list may occur one or more (`+`),
zero or more (`*`), or zero or one times (`?`). The
absence of such an operator means that the element or content particle *MUST*
appear exactly once. This syntax and meaning are identical to those used in
the productions in this specification.

The content of an element matches a content model if and only if it is
possible to trace out a path through the content model, obeying the sequence,
choice, and repetition operators and matching each element in the content
against an element type in the content model. [For
compatibility](002-appendices.md#dt-compat "For Compatibility"), it is an error if the content model
allows an element to match more than one occurrence of an element type in the
content model. For more information, see [**E Deterministic Content Models**](017-c-xml-and-sgml-non-normative.md#determinism).

<a id="vc-PEinGroup"></a>

**Validity constraint: Proper Group/PE Nesting**

Parameter-entity [replacement text](014-4-5-construction-of-entity-replacement-text.md#dt-repltext "Replacement Text")
*MUST* be properly nested with parenthesized
groups. That is to say, if either of the opening or closing parentheses in
a [choice](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-choice), [seq](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-seq), or [Mixed](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-Mixed)
construct is contained in the replacement text for a [parameter
entity](011-4-1-character-and-entity-references.md#dt-PERef "Parameter-entity reference"), both *MUST* be contained in the same replacement text.

[For interoperability](002-appendices.md#dt-interop "For interoperability"), if a parameter-entity reference
appears in a [choice](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-choice), [seq](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-seq), or [Mixed](008-3-1-start-tags-end-tags-and-empty-element-tags.md#NT-Mixed) construct, its replacement text *SHOULD* contain at
least one non-blank character, and neither the first nor last non-blank character
of the replacement text *SHOULD* be a connector (`|` or `,`).

Examples of element-content models:



```
<!ELEMENT spec (front, body, back?)>
<!ELEMENT div1 (head, (p | list | note)*, div2*)>
<!ELEMENT dictionary-body (%div.mix; | %dict.mix;)*>
```




<a id="sec-mixed-content"></a>

#### 3.2.2 Mixed Content

<a id="dt-mixed"></a>

[Definition: An element [type](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-stag "Start-Tag")
has **mixed content** when elements of that type may contain character
data, optionally interspersed with [child](003-2-documents.md#dt-parentchild "Parent/Child")
elements.] In this case, the types of the child elements may be constrained,
but not their order or their number of occurrences:

<a id="d0e2826"></a>

##### Mixed-content Declaration

<a id="NT-Mixed"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [51] | `Mixed` | ::= | `'(' S? '#PCDATA' (S? '|' S? Name)* S? ')*'` |
|  |  |  | `| '(' S? '#PCDATA' S? ')'` | [[VC: Proper Group/PE Nesting]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#vc-PEinGroup) |
|  |  |  |  | [[VC: No Duplicate Types]](008-3-1-start-tags-end-tags-and-empty-element-tags.md#vc-MixedChildrenUnique) |

where the [Name](003-2-documents.md#NT-Name)s give the types of elements that
may appear as children. The
keyword **#PCDATA** derives historically from the term "parsed
character data."

<a id="vc-MixedChildrenUnique"></a>

**Validity constraint: No Duplicate Types**

The
same name *MUST NOT* appear more than once in a single mixed-content declaration.

Examples of mixed content declarations:



```
<!ELEMENT p (#PCDATA|a|ul|b|i|em)*>
<!ELEMENT p (#PCDATA | %font; | %phrase; | %special; | %form;)* >
<!ELEMENT b (#PCDATA)>
```




