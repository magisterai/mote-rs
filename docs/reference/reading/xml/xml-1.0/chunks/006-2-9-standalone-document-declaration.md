# 2.9 Standalone Document Declaration

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-2-8-prolog-and-document-type-declaration.md) · [Next](007-2-12-language-identification.md)

<a id="sec-rmd"></a>

### 2.9 Standalone Document Declaration

<a id="dt-extmkpdecl"></a>

Markup declarations can affect the content of the document, as passed from
an [XML processor](002-appendices.md#dt-xml-proc "XML Processor") to an application; examples
are attribute defaults and entity declarations. The standalone document declaration,
which may appear as a component of the XML declaration, signals whether or
not there are such declarations which appear external to the [document
entity](014-4-5-construction-of-entity-replacement-text.md#dt-docent "Document Entity")
or in parameter entities. [Definition: An **external
markup declaration** is defined as a markup declaration occurring in
the external subset or in a parameter entity (external or internal, the latter
being included because non-validating processors are not required to read
them).]

<a id="fulldtd"></a>

##### Standalone Document Declaration

<a id="NT-SDDecl"></a>

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| [32] | `SDDecl` | ::= | `S 'standalone' Eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))` | [[VC: Standalone Document Declaration]](006-2-9-standalone-document-declaration.md#vc-check-rmd) |

In a standalone document declaration, the value "yes" indicates
that there are no [external markup declarations](006-2-9-standalone-document-declaration.md#dt-extmkpdecl "External Markup Declaration") which
affect the information passed from the XML processor to the application. The
value "no" indicates that there are or may be such external
markup declarations. Note that the standalone document declaration only denotes
the presence of external *declarations*; the presence, in a document,
of references to external *entities*, when those entities are internally
declared, does not change its standalone status.

If there are no external markup declarations, the standalone document declaration
has no meaning. If there are external markup declarations but there is no
standalone document declaration, the value "no" is assumed.

Any XML document for which `standalone="no"` holds can be converted
algorithmically to a standalone document, which may be desirable for some
network delivery applications.

<a id="vc-check-rmd"></a>

**Validity constraint: Standalone Document Declaration**

The
standalone document declaration *MUST* have the value "no" if
any external markup declarations contain declarations of:

- attributes with [default](009-3-3-attribute-list-declarations.md#dt-default "Attribute Default") values,
  if elements to which these attributes apply appear in the document without
  specifications of values for these attributes, or
- entities (other than `amp`,
  `lt`,
  `gt`,
  `apos`,
  `quot`), if [references](011-4-1-character-and-entity-references.md#dt-entref "Entity Reference")
  to those entities appear in the document, or
- attributes with
  tokenized types, where the
  attribute appears in the document with a value such that
  [normalization](009-3-3-attribute-list-declarations.md#AVNormalize)
  will produce a different value from that which would be produced
  in the absence of the declaration, or
- element types with [element content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-elemcontent "Element content"),
  if white space occurs directly within any instance of those types.

An example XML declaration with a standalone document declaration:



```
<?xml version="1.0" standalone='yes'?>
```




<a id="sec-white-space"></a>

### 2.10 White Space Handling

In editing XML documents, it is often convenient to use "white space"
(spaces, tabs, and blank lines)
to set apart the markup for greater readability. Such white space is typically
not intended for inclusion in the delivered version of the document. On the
other hand, "significant" white space that should be preserved
in the delivered version is common, for example in poetry and source code.

An [XML processor](002-appendices.md#dt-xml-proc "XML Processor")
*MUST* always pass
all characters in a document that are not markup through to the application.
A  [validating XML processor](014-4-5-construction-of-entity-replacement-text.md#dt-validating "Validating Processor")
*MUST* also
inform the application which of these characters constitute white space appearing
in [element content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-elemcontent "Element content").

A special [attribute](008-3-1-start-tags-end-tags-and-empty-element-tags.md#dt-attr "Attribute") named `xml:space` may be attached to an element to signal an intention that in that element,
white space should be preserved by applications. In valid documents, this
attribute, like any other, *MUST* be [declared](009-3-3-attribute-list-declarations.md#dt-attdecl "Attribute-List Declaration")
if it is used. When declared, it *MUST* be given as an [enumerated
type](009-3-3-attribute-list-declarations.md#dt-enumerated "Enumerated Attribute Values") whose values
are one or both of "default" and "preserve".
For example:



```
<!ATTLIST poem  xml:space (default|preserve) 'preserve'>

<!ATTLIST pre xml:space (preserve) #FIXED 'preserve'>
```




The value "default" signals that applications' default white-space
processing modes are acceptable for this element; the value "preserve"
indicates the intent that applications preserve all the white space. This
declared intent is considered to apply to all elements within the content
of the element where it is specified, unless overridden with
another instance of the `xml:space` attribute. This specification does not give meaning to any value of `xml:space` other than "default" and "preserve". It is an error for other values to be specified; the XML processor *MAY* report the error or *MAY* recover by ignoring the attribute specification or by reporting the (erroneous) value to the application. Applications may ignore or reject erroneous values.

The [root element](003-2-documents.md#dt-root "Root Element") of any document is considered
to have signaled no intentions as regards application space handling, unless
it provides a value for this attribute or the attribute is declared with a
default value.

<a id="sec-line-ends"></a>

### 2.11 End-of-Line Handling

XML [parsed entities](010-3-4-conditional-sections.md#dt-parsedent "Text Entity") are often stored
in computer files which, for editing convenience, are organized into lines.
These lines are typically separated by some combination of the characters
CARRIAGE RETURN (#xD) and LINE FEED (#xA).

To
simplify the tasks of [applications](002-appendices.md#dt-app "Application"), the
[XML
processor](002-appendices.md#dt-xml-proc "XML Processor")
*MUST* behave as if it normalized all line breaks in external parsed
entities (including the document entity) on input, before parsing, by translating
both the two-character sequence #xD #xA and any #xD that is not followed by
#xA to a single #xA character.

