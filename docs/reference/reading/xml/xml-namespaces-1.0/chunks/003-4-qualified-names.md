# 4 Qualified Names

Source: [Namespaces in XML 1.0](https://www.w3.org/TR/2009/REC-xml-names-20091208/) · 3rd edition, 2009-12-08.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2009/REC-xml-names-20091208/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-1-motivation-and-summary.md) · [Next](004-6-applying-namespaces-to-elements-and-attributes.md)

<a id="ns-qualnames"></a>

## 4 Qualified Names

In XML
documents conforming to this specification, some
names (constructs corresponding to the nonterminal
[Name](https://www.w3.org/TR/REC-xml/#NT-Name))
*MUST*
be given as
[qualified names](002-1-motivation-and-summary.md#dt-qualname "Qualified Name"),
defined as follows:

<a id="A1153"></a>

##### Qualified Name

<a id="NT-QName"></a>

<a id="NT-PrefixedName"></a>

<a id="NT-UnprefixedName"></a>

<a id="NT-Prefix"></a>

<a id="NT-LocalPart"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [7] | `QName` | ::= | `PrefixedName` |
|  |  |  | `| UnprefixedName` |
|  |  |  |  |
| --- | --- | --- | --- |
| [8] | `PrefixedName` | ::= | `Prefix ':' LocalPart` |
|  |  |  |  |
| --- | --- | --- | --- |
| [9] | `UnprefixedName` | ::= | `LocalPart` |
|  |  |  |  |
| --- | --- | --- | --- |
| [10] | `Prefix` | ::= | `NCName` |
|  |  |  |  |
| --- | --- | --- | --- |
| [11] | `LocalPart` | ::= | `NCName` |

<a id="dt-localpart"></a>

The
[Prefix](003-4-qualified-names.md#NT-Prefix) provides the
[namespace prefix](002-1-motivation-and-summary.md#dt-prefix "Namespace Prefix")
part of the qualified name, and
*MUST*
be associated with a namespace
URI
reference in a
[namespace declaration](002-1-motivation-and-summary.md#dt-NSDecl "Namespace declaration").
[Definition:
The [LocalPart](003-4-qualified-names.md#NT-LocalPart) provides the
**local part** of the qualified name.]

Note that the prefix functions *only* as a placeholder for a
namespace name.
Applications
*SHOULD*
use the namespace name, not the prefix, in constructing
names whose scope extends beyond the
containing document.

<a id="ns-using"></a>

## 5 Using Qualified Names

In XML documents conforming to this specification,
element names are given as
[qualified names](002-1-motivation-and-summary.md#dt-qualname "Qualified Name"), as
follows:

<a id="A1329"></a>

##### Element Names

<a id="NT-STag"></a>

<a id="NT-ETag"></a>

<a id="NT-EmptyElemTag"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [12] | `STag` | ::= | `'<' QName (S Attribute)* S? '>'` | [[NSC: Prefix Declared]](003-4-qualified-names.md#nsc-NSDeclared) |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [13] | `ETag` | ::= | `'</' QName S? '>'` | [[NSC: Prefix Declared]](003-4-qualified-names.md#nsc-NSDeclared) |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [14] | `EmptyElemTag` | ::= | `'<' QName (S Attribute)* S? '/>'` | [[NSC: Prefix Declared]](003-4-qualified-names.md#nsc-NSDeclared) |

An example of a qualified name serving as an element name:



```

  <!-- the 'price' element's namespace is http://ecommerce.example.org/schema -->
  <edi:price xmlns:edi='http://ecommerce.example.org/schema' units='Euro'>32.18</edi:price>
```




Attributes are either [namespace
declarations](002-1-motivation-and-summary.md#dt-NSDecl "Namespace declaration")
or their names are given as
[qualified names](002-1-motivation-and-summary.md#dt-qualname "Qualified Name"):

<a id="A1472"></a>

##### Attribute

<a id="NT-Attribute"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [15] | `Attribute` | ::= | `NSAttName Eq AttValue` |
|  |  |  | `| QName Eq AttValue` | [[NSC: Prefix Declared]](003-4-qualified-names.md#nsc-NSDeclared) |
|  |  |  |  | [[NSC: No Prefix Undeclaring]](003-4-qualified-names.md#nsc-NoPrefixUndecl) |
|  |  |  |  | [[NSC: Attributes Unique]](004-6-applying-namespaces-to-elements-and-attributes.md#nsc-AttrsUnique) |

An example of a qualified name serving as an attribute name:



```
<x xmlns:edi='http://ecommerce.example.org/schema'>
  <!-- the 'taxClass' attribute's namespace is http://ecommerce.example.org/schema -->
  <lineItem edi:taxClass="exempt">Baby food</lineItem>
</x>
```




<a id="nsc-NSDeclared"></a>

**Namespace constraint: Prefix Declared**

The namespace prefix, unless it is `xml`
or `xmlns`,
*MUST*
have been
declared in a [namespace declaration](002-1-motivation-and-summary.md#dt-NSDecl "Namespace declaration")
attribute in either the start-tag of the element where the prefix
is used or in an ancestor element (i.e., an element in whose
[content](https://www.w3.org/TR/REC-xml/#dt-content) the
prefixed markup occurs).

<a id="nsc-NoPrefixUndecl"></a>

**Namespace constraint: No Prefix Undeclaring**

In a [namespace declaration](002-1-motivation-and-summary.md#dt-NSDecl "Namespace declaration") for a [prefix](003-4-qualified-names.md#NT-Prefix) (i.e.,
where the [NSAttName](002-1-motivation-and-summary.md#NT-NSAttName) is a [PrefixedAttName](002-1-motivation-and-summary.md#NT-PrefixedAttName)), the [attribute value](https://www.w3.org/TR/REC-xml/#NT-AttValue) *MUST NOT* be empty.

This constraint may lead to operational difficulties in the case where
the namespace declaration attribute is provided, not directly in the XML
[document entity](https://www.w3.org/TR/REC-xml/#dt-docent), but
via a default attribute declared in an external entity.
Such declarations may not be read by software which is based on a
non-validating XML processor.
Many XML applications, presumably including namespace-sensitive ones, fail to
require validating processors.
If correct operation with such applications is required,
namespace declarations
*MUST*
be
provided either directly or via default attributes declared in the
[internal subset of the DTD](https://www.w3.org/TR/REC-xml/#dt-doctype).

Element names and attribute names are also given as qualified names when
they appear in declarations in the
[DTD](https://www.w3.org/TR/REC-xml/#dt-doctype):

<a id="A1686"></a>

##### Qualified Names in Declarations

<a id="NT-doctypedecl"></a>

<a id="NT-elementdecl"></a>

<a id="NT-cp"></a>

<a id="NT-Mixed"></a>

<a id="NT-AttlistDecl"></a>

<a id="NT-AttDef"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [16] | `doctypedecl` | ::= | `'<!DOCTYPE' S QName (S ExternalID)? S? ('[' (markupdecl | PEReference | S)* ']' S?)? '>'` |
|  |  |  |  |
| --- | --- | --- | --- |
| [17] | `elementdecl` | ::= | `'<!ELEMENT' S QName S contentspec S? '>'` |
|  |  |  |  |
| --- | --- | --- | --- |
| [18] | `cp` | ::= | `(QName | choice | seq) ('?' | '*' | '+')?` |
|  |  |  |  |
| --- | --- | --- | --- |
| [19] | `Mixed` | ::= | `'(' S? '#PCDATA' (S? '|' S? QName)* S? ')*'` |
|  |  |  | `| '(' S? '#PCDATA' S? ')'` |
|  |  |  |  |
| --- | --- | --- | --- |
| [20] | `AttlistDecl` | ::= | `'<!ATTLIST' S QName AttDef* S? '>'` |
|  |  |  |  |
| --- | --- | --- | --- |
| [21] | `AttDef` | ::= | `S (QName | NSAttName) S AttType S DefaultDecl` |

Note that DTD-based validation is not namespace-aware in the following
sense: a DTD constrains the elements and attributes that may appear in
a document by their uninterpreted names, not by (namespace name, local
name) pairs. To validate a document that uses namespaces against a
DTD, the same prefixes must be used in the DTD as in the instance.
A DTD may however indirectly constrain the namespaces used in a valid
document by providing `#FIXED` values for attributes that
declare namespaces.

