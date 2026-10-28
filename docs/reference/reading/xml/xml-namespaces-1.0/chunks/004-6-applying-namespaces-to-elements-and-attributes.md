# 6 Applying Namespaces to Elements and Attributes

Source: [Namespaces in XML 1.0](https://www.w3.org/TR/2009/REC-xml-names-20091208/) · 3rd edition, 2009-12-08.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2009/REC-xml-names-20091208/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-4-qualified-names.md) · [Next](005-8-conformance-of-processors.md)

<a id="scoping-defaulting"></a>

## 6 Applying Namespaces to Elements and Attributes

<a id="scoping"></a>

### 6.1 Namespace Scoping

The scope of a namespace declaration declaring a prefix extends from
the beginning of the start-tag in which it appears to the end of the
corresponding end-tag, excluding the scope of any inner declarations
with the same NSAttName part.
In the case of an empty tag, the scope is the tag itself.

Such a namespace declaration applies to all element and attribute
names within its scope whose prefix matches that specified in the
declaration.

The
[expanded name](002-1-motivation-and-summary.md#dt-expname "Expanded Name")
corresponding to a prefixed element or attribute name has the
URI
to which the
[prefix](003-4-qualified-names.md#NT-Prefix)
is bound as its
[namespace name](002-1-motivation-and-summary.md#dt-NSName "Namespace Name"),
and the
[local part](003-4-qualified-names.md#NT-LocalPart)
as its
[local name](002-1-motivation-and-summary.md#dt-localname "Local Name").



```
<?xml version="1.0"?>

<html:html xmlns:html='http://www.w3.org/1999/xhtml'>

  <html:head><html:title>Frobnostication</html:title></html:head>
  <html:body><html:p>Moved to 
    <html:a href='http://frob.example.com'>here.</html:a></html:p></html:body>
</html:html>
```




Multiple namespace prefixes can be declared as attributes of a single element,
as shown in this example:



```
<?xml version="1.0"?>
<!-- both namespace prefixes are available throughout -->
<bk:book xmlns:bk='urn:loc.gov:books'
         xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <bk:title>Cheaper by the Dozen</bk:title>
    <isbn:number>1568491379</isbn:number>
</bk:book>
```




<a id="defaulting"></a>

### 6.2 Namespace Defaulting

The scope of a
[default namespace](002-1-motivation-and-summary.md#dt-defaultNS "Default Namespace") declaration
extends from the beginning of the
start-tag in which it appears to the end of the corresponding end-tag,
excluding the scope of any inner default namespace declarations.
In the case of an empty tag, the scope is the tag itself.

A default namespace declaration applies to all unprefixed element names
within its scope.
Default namespace declarations do not apply directly to attribute names;
the interpretation of unprefixed attributes is
determined by the element on which they appear.

If there is a default namespace declaration in scope, the
[expanded name](002-1-motivation-and-summary.md#dt-expname "Expanded Name")
corresponding to an unprefixed element name has the
URI
of the
[default namespace](002-1-motivation-and-summary.md#dt-defaultNS "Default Namespace")
as its
[namespace name](002-1-motivation-and-summary.md#dt-NSName "Namespace Name").
If there is no default namespace declaration in scope, the
namespace name has no value.
The namespace name for an unprefixed attribute name always has no value.
In all cases, the
[local name](002-1-motivation-and-summary.md#dt-localname "Local Name") is
[local part](003-4-qualified-names.md#NT-LocalPart)
(which is of course the same as the unprefixed name itself).



```
<?xml version="1.0"?>
<!-- elements are in the HTML namespace, in this case by default -->
<html xmlns='http://www.w3.org/1999/xhtml'>
  <head><title>Frobnostication</title></head>
  <body><p>Moved to 
    <a href='http://frob.example.com'>here</a>.</p></body>
</html>
```






```
<?xml version="1.0"?>
<!-- unprefixed element types are from "books" -->
<book xmlns='urn:loc.gov:books'
      xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <title>Cheaper by the Dozen</title>
    <isbn:number>1568491379</isbn:number>
</book>
```




A larger example of namespace scoping:



```
<?xml version="1.0"?>
<!-- initially, the default namespace is "books" -->
<book xmlns='urn:loc.gov:books'
      xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <title>Cheaper by the Dozen</title>
    <isbn:number>1568491379</isbn:number>
    <notes>
      <!-- make HTML the default namespace for some commentary -->
      <p xmlns='http://www.w3.org/1999/xhtml'>
          This is a <i>funny</i> book!
      </p>
    </notes>
</book>
```




The attribute value in a default namespace declaration
*MAY*
be empty.
This has the same
effect, within the scope of the declaration, of there being no default
namespace.



```
<?xml version='1.0'?>
<Beers>
  <!-- the default namespace inside tables is that of HTML -->
  <table xmlns='http://www.w3.org/1999/xhtml'>
   <th><td>Name</td><td>Origin</td><td>Description</td></th>
   <tr> 
     <!-- no default namespace inside table cells -->
     <td><brandName xmlns="">Huntsman</brandName></td>
     <td><origin xmlns="">Bath, UK</origin></td>
     <td>
       <details xmlns=""><class>Bitter</class><hop>Fuggles</hop>
         <pro>Wonderful hop, light alcohol, good summer beer</pro>
         <con>Fragile; excessive variance pub to pub</con>
         </details>
        </td>
      </tr>
    </table>
  </Beers>
```




<a id="uniqAttrs"></a>

### 6.3 Uniqueness of Attributes

<a id="nsc-AttrsUnique"></a>

**Namespace constraint: Attributes Unique**

In XML documents conforming
to this specification, no tag
may contain two attributes which:

1. have identical names, or
2. have qualified names with the same
   [local part](003-4-qualified-names.md#dt-localpart "Local Part") and with
   [prefixes](002-1-motivation-and-summary.md#dt-prefix "Namespace Prefix") which have been bound
   to [namespace names](002-1-motivation-and-summary.md#dt-NSName "Namespace Name") that
   are [identical](002-1-motivation-and-summary.md#dt-identical "Identical").

This constraint is equivalent to requiring that no element have two
attributes with the same
[expanded name](002-1-motivation-and-summary.md#dt-expname "Expanded Name").

For example, each of the `bad` empty-element tags is illegal in the
following:



```
<!-- http://www.w3.org is bound to n1 and n2 -->
<x xmlns:n1="http://www.w3.org" 
   xmlns:n2="http://www.w3.org" >
  <bad a="1"     a="2" />
  <bad n1:a="1"  n2:a="2" />
</x>
```




However, each of the following is legal, the second because the default
namespace does not apply to attribute names:



```
<!-- http://www.w3.org is bound to n1 and is the default -->
<x xmlns:n1="http://www.w3.org" 
   xmlns="http://www.w3.org" >
  <good a="1"     b="2" />
  <good a="1"     n1:a="2" />
</x>
```




<a id="Conformance"></a>

## 7 Conformance of Documents

This specification applies to XML 1.0
documents. To conform to this
specification, a document
*MUST*
be well-formed according to the
XML 1.0 specification [[XML]](005-8-conformance-of-processors.md#XML).

In XML documents which conform to this specification, element
and attribute names
*MUST*
match the production for
[QName](003-4-qualified-names.md#NT-QName)
and
*MUST*
satisfy the "Namespace Constraints". All other tokens in the
document which are
*REQUIRED*,
for XML 1.0 well-formedness, to match the
XML production for
[Name](https://www.w3.org/TR/REC-xml/#NT-Name)
*MUST*
match this specification's production for
[NCName](002-1-motivation-and-summary.md#NT-NCName).

<a id="dt-nwf"></a>

[Definition:
A document is **namespace-well-formed**
if it conforms to this specification.
]

It follows that in a namespace-well-formed document:

- All element and attribute names contain either zero or one
  colon;
- No entity names, processing instruction targets, or notation names contain any colons.

In addition, a namespace-well-formed document may also be namespace-valid.

<a id="dt-nv"></a>

[Definition:
A namespace-well-formed document is **namespace-valid**
if it is valid according to the XML 1.0 specification, and all tokens
other than element and attribute names which are
*REQUIRED*,
for XML 1.0 validity, to match the XML production for
[Name](https://www.w3.org/TR/REC-xml/#NT-Name)
match this specification's production for
[NCName](002-1-motivation-and-summary.md#NT-NCName).
]

It follows that in a namespace-valid document:

- No attributes with a declared type of
  **ID**, **IDREF(S)**, **ENTITY(IES)**, or **NOTATION**
  contain any colons.

