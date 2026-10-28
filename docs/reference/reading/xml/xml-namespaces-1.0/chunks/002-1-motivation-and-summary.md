# 1 Motivation and Summary

Source: [Namespaces in XML 1.0](https://www.w3.org/TR/2009/REC-xml-names-20091208/) · 3rd edition, 2009-12-08.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2009/REC-xml-names-20091208/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-namespaces-in-xml-1-0-third-edition.md) · [Next](003-4-qualified-names.md)

<a id="sec-intro"></a>

## 1 Motivation and Summary

We envision applications of Extensible Markup Language (XML) where
a single XML document may
contain elements and attributes
(here referred to as a "markup vocabulary")
that are defined for and used by multiple software modules.
One motivation for this is modularity: if such a markup vocabulary exists
which is well-understood and for which there is useful software
available, it is better to re-use this markup rather than re-invent it.

Such documents, containing multiple markup vocabularies,
pose problems of recognition and collision. Software modules need to
be able to recognize the elements and attributes which they are designed
to process, even in the face
of "collisions" occurring when markup intended for some other software
package uses the same element name
or attribute name.

These considerations require that document constructs
should have names constructed so as to avoid clashes
between names from different markup vocabularies.
This specification describes a mechanism,
*XML namespaces*, which accomplishes this
by assigning [expanded names](002-1-motivation-and-summary.md#dt-expname "Expanded Name")
to elements and attributes.

<a id="notation"></a>

### 1.1 A Note on Notation and Usage

Where *EMPHASIZED*, the key words
*MUST*,
*MUST NOT*,
*REQUIRED*,
*SHOULD*,
*SHOULD NOT*,
*MAY*
in this document are to be interpreted as described in
[[Keywords]](005-8-conformance-of-processors.md#keywords).

Note that many of the
nonterminals in the productions in
this specification are defined not here but in
the XML specification [[XML]](005-8-conformance-of-processors.md#XML).
When nonterminals defined here have the same names as nonterminals
defined in the XML specification, the productions here
in all cases match a subset of the strings matched by the
corresponding ones there.

In this document's productions,
the `NSC` is a "Namespace Constraint",
one of the rules that documents conforming to this specification
*MUST*
follow.

<a id="sec-namespaces"></a>

## 2 XML Namespaces

<a id="concepts"></a>

### 2.1 Basic Concepts

<a id="dt-namespace"></a>

[Definition:
An **XML namespace** is identified by
a URI reference [[RFC3986]](005-8-conformance-of-processors.md#URIRef);
element and attribute names
may be placed in an XML namespace using the mechanisms described
in this specification.
]

<a id="dt-expname"></a>

<a id="dt-NSName"></a>

<a id="dt-localname"></a>

[Definition:
An **expanded name**
is a pair consisting of a
[namespace name](002-1-motivation-and-summary.md#dt-NSName "Namespace Name")
and a
[local name](002-1-motivation-and-summary.md#dt-localname "Local Name").
]
[Definition:
For a name N in a namespace identified
by
a URI
I, the
**namespace name**
is I. For a name N that is not in a namespace, the
**namespace name**
has no value.
]
[Definition:
In either case the
**local name**
is N.
]
It is this combination of the universally managed URI namespace
with the vocabulary's local names that is effective in avoiding
name clashes.

<a id="dt-qualname"></a>

URI
references can contain characters not allowed in names, and are often
inconveniently long, so expanded names are not used directly to name
elements and attributes in XML documents. Instead
[qualified names](002-1-motivation-and-summary.md#dt-qualname "Qualified Name")
are used.
[Definition:
A
**qualified name**
is a name subject to namespace interpretation.
]
In documents conforming to this specification,
element and attribute names appear as qualified names.
Syntactically, they are either
[prefixed names](003-4-qualified-names.md#NT-PrefixedName) or
[unprefixed names](003-4-qualified-names.md#NT-UnprefixedName).
An attribute-based declaration syntax is provided to bind prefixes to
namespace names and to bind a default namespace that applies to
unprefixed element names;
these declarations are scoped by the elements on which they appear so that
different bindings may apply in different parts of a document.
Processors conforming to this specification
*MUST*
recognize and act on these declarations and prefixes.

<a id="iri-use"></a>

### 2.2 Use of URIs as Namespace Names

The empty string, though it is a legal
URI
reference, cannot be used as a namespace name.

The use of relative
URI
references,
including same-document references, in namespace declarations is
deprecated.

**Note:**

This deprecation of relative URI references was decided on by a
W3C XML Plenary Ballot [[Relative URI deprecation]](005-8-conformance-of-processors.md#reluri). It also declares that
"later specifications such as DOM, XPath, etc. will define no
interpretation for them".

<a id="NSNameComparison"></a>

### 2.3 Comparing URI References

<a id="dt-identical"></a>

URI
references identifying namespaces are compared when determining
whether a name belongs to a given namespace, and whether two names
belong to the same namespace.
[Definition:
The two
URIs
are treated as strings, and they are
**identical**
if and only if the strings are identical, that is, if they
are the same sequence of characters.
]
The comparison is case-sensitive, and no %-escaping is done or undone.

A consequence of this is that
URI
references which are not identical
in this sense may resolve to the same resource. Examples include
URI
references which differ only in case or %-escaping, or which are
in external entities which have different base URIs (but note that
relative
URIs
are deprecated as namespace names).

In a namespace declaration, the
URI
reference is the
[normalized value](https://www.w3.org/TR/REC-xml/#AVNormalize)
of the attribute, so replacement of XML character and entity references
has already been done before any comparison.

Examples:

The
URI
references below are all different for the purposes of identifying
namespaces, since they differ in case:

- `http://www.example.org/wine`
- `http://www.Example.org/wine`
- `http://www.example.org/Wine`

The URI references below are also all different for the purposes of identifying
namespaces:

- `http://www.example.org/~wilbur`
- `http://www.example.org/%7ewilbur`
- `http://www.example.org/%7Ewilbur`

Because of the risk of confusion between
URIs
that would be equivalent
if dereferenced, the use of %-escaped characters in namespace names is
strongly discouraged.

<a id="ns-decl"></a>

## 3 Declaring Namespaces

<a id="dt-NSDecl"></a>

[Definition: A namespace
(or more precisely, a namespace binding)
is
**declared** using
a family of reserved attributes.
Such an attribute's name must either
be **xmlns** or begin **xmlns:**.
These attributes, like any other XML attributes, may be provided
directly or by [default](https://www.w3.org/TR/REC-xml/#dt-default).
]

<a id="A785"></a>

##### Attribute Names for Namespace Declaration

<a id="NT-NSAttName"></a>

<a id="NT-PrefixedAttName"></a>

<a id="NT-DefaultAttName"></a>

<a id="NT-NCName"></a>

|  |  |  |  |
| --- | --- | --- | --- |
| [1] | `NSAttName` | ::= | `PrefixedAttName` |
|  |  |  | `| DefaultAttName` |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [2] | `PrefixedAttName` | ::= | `'xmlns:' NCName` | [[NSC: Reserved Prefixes and Namespace Names]](002-1-motivation-and-summary.md#xmlReserved) |
|  |  |  |  |
| --- | --- | --- | --- |
| [3] | `DefaultAttName` | ::= | `'xmlns'` |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [4] | `NCName` | ::= | `Name - (Char* ':' Char*)` | */\* An XML [Name](https://www.w3.org/TR/REC-xml/#NT-Name), minus the ":" \*/* |

The attribute's
[normalized value](https://www.w3.org/TR/REC-xml/#AVNormalize)
*MUST*
be either
a URI
reference — the
[namespace name](002-1-motivation-and-summary.md#dt-NSName "Namespace Name")
identifying the namespace —
or an empty string.
The namespace name, to serve its
intended purpose,
*SHOULD*
have the characteristics of uniqueness and
persistence.
It is not a goal that it be directly usable for retrieval of a schema (if
any exists).
Uniform Resource Names [[RFC2141]](005-8-conformance-of-processors.md#URNs) is an example of a syntax that
is designed with these goals in mind.
However, it should be noted that ordinary URLs can be managed in such a way as
to achieve these same goals.

<a id="dt-prefix"></a>

[Definition: If the
attribute name matches [PrefixedAttName](002-1-motivation-and-summary.md#NT-PrefixedAttName),
then the
[NCName](002-1-motivation-and-summary.md#NT-NCName) gives the **namespace prefix**,
used to associate element and attribute names with the
[namespace name](002-1-motivation-and-summary.md#dt-NSName "Namespace Name") in the attribute value
in the scope of the element to which the declaration
is attached.
]

<a id="dt-defaultNS"></a>

[Definition: If the
attribute name matches [DefaultAttName](002-1-motivation-and-summary.md#NT-DefaultAttName),
then the
[namespace name](002-1-motivation-and-summary.md#dt-NSName "Namespace Name") in the
attribute value is
that of the **default namespace**
in the scope of the element to which the declaration
is attached.]
Default namespaces and overriding of declarations are discussed in
[**6 Applying Namespaces to Elements and Attributes**](004-6-applying-namespaces-to-elements-and-attributes.md#scoping-defaulting).

An example namespace declaration, which associates the
namespace prefix **edi** with the namespace name
`http://ecommerce.example.org/schema`:



```
<x xmlns:edi='http://ecommerce.example.org/schema'>
  <!-- the "edi" prefix is bound to http://ecommerce.example.org/schema
       for the "x" element and contents -->
</x>
```




<a id="xmlReserved"></a>

**Namespace constraint: Reserved Prefixes and Namespace Names**

The prefix **xml** is by definition bound to the namespace name
`http://www.w3.org/XML/1998/namespace`. It
*MAY*,
but need not, be
declared, and
*MUST NOT*
be
bound to any other namespace name. Other prefixes
*MUST NOT*
be bound to this namespace
name,
and it
*MUST NOT*
be declared as the default namespace.

The prefix **xmlns** is used only to declare namespace bindings and is by
definition bound to the namespace name
`http://www.w3.org/2000/xmlns/`. It
*MUST NOT*
be declared
.
Other prefixes
*MUST NOT*
be bound to this namespace
name,
and it
*MUST NOT*
be declared as the default namespace.
Element names
*MUST NOT*
have the prefix
`xmlns`.

All other prefixes beginning with the three-letter sequence x, m, l,
in any case combination, are reserved. This means that:

- users
  *SHOULD NOT*
  use them except as defined by later specifications
- processors
  *MUST NOT*
  treat them as fatal errors.

Though they are not themselves reserved, it is inadvisable to use
prefixed names whose LocalPart begins with the letters x, m, l, in any
case combination, as
these names would be reserved if used without a prefix.

