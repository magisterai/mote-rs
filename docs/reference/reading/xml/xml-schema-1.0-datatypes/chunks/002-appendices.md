# Appendices

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-xml-schema-part-2-datatypes-second-edition.md) · [Next](003-1-4-terminology.md)

<a id="appendices"></a>

### Appendices

A [Schema for Datatype Definitions (normative)](033-5-conformance.md#schema)  
B [DTD for Datatype Definitions (non-normative)](035-b-dtd-for-datatype-definitions-non-normative.md#dtd-for-datatypeDefs)  
C [Datatypes and Facets](035-b-dtd-for-datatype-definitions-non-normative.md#d0e11239)  
    C.1 [Fundamental Facets](035-b-dtd-for-datatype-definitions-non-normative.md#app-fundamental-facets)  
D [ISO 8601 Date and Time Formats](037-d-iso-8601-date-and-time-formats.md#isoformats)  
    D.1 [ISO 8601 Conventions](037-d-iso-8601-date-and-time-formats.md#formatdetails)  
    D.2 [Truncated and Reduced Formats](038-d-2-truncated-and-reduced-formats.md#truncatedformats)  
    D.3 [Deviations from ISO 8601 Formats](038-d-2-truncated-and-reduced-formats.md#deviantformats)  
E [Adding durations to dateTimes](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes)  
    E.1 [Algorithm](039-e-1-algorithm.md#d0e11648)  
    E.2 [Commutativity and Associativity](039-e-1-algorithm.md#adding-durations-to-instants-commutativity-associativity)  
F [Regular Expressions](039-e-1-algorithm.md#regexs)  
    F.1 [Character Classes](040-f-1-character-classes.md#charcter-classes)  
G [Glossary (non-normative)](043-g-glossary-non-normative.md#normative-glossary)  
H [References](044-h-references.md#biblio)  
    H.1 [Normative](044-h-references.md#normative-biblio)  
    H.2 [Non-normative](044-h-references.md#non-normative-biblio)  
I [Acknowledgements (non-normative)](045-i-acknowledgements-non-normative.md#acknowledgments)

---

<a id="Intro"></a>

## 1 Introduction

<a id="purpose"></a>

### 1.1 Purpose

The [[XML 1.0 (Second Edition)]](044-h-references.md#XML) specification defines limited
facilities for applying datatypes to document content in that documents
may contain or refer to DTDs that assign types to elements and attributes.
However, document authors, including authors of traditional
*documents* and those transporting *data* in XML,
often require a higher degree of type checking to ensure robustness in
document understanding and data interchange.

The table below offers two typical examples of XML instances
in which datatypes are implicit: the instance on the left
represents a billing invoice, the instance on the
right a memo or perhaps an email message in XML.

<table border="1" class="dtdemo"><thead><tr><th>Data oriented</th><th>Document oriented</th></tr></thead><tbody><tr><td>
<pre>&lt;invoice&gt;
  &lt;orderDate&gt;1999-01-21&lt;/orderDate&gt;
  &lt;shipDate&gt;1999-01-25&lt;/shipDate&gt;
  &lt;billingAddress&gt;
   &lt;name&gt;Ashok Malhotra&lt;/name&gt;
   &lt;street&gt;123 Microsoft Ave.&lt;/street&gt;
   &lt;city&gt;Hawthorne&lt;/city&gt;
   &lt;state&gt;NY&lt;/state&gt;
   &lt;zip&gt;10532-0000&lt;/zip&gt;
  &lt;/billingAddress&gt;
  &lt;voice&gt;555-1234&lt;/voice&gt;
  &lt;fax&gt;555-4321&lt;/fax&gt;
&lt;/invoice&gt;</pre>
</td><td>
<pre>&lt;memo importance='high'
      date='1999-03-23'&gt;
  &lt;from&gt;Paul V. Biron&lt;/from&gt;
  &lt;to&gt;Ashok Malhotra&lt;/to&gt;
  &lt;subject&gt;Latest draft&lt;/subject&gt;
  &lt;body&gt;
    We need to discuss the latest
    draft &lt;emph&gt;immediately&lt;/emph&gt;.
    Either email me at &lt;email&gt;
    mailto:paul.v.biron@kp.org&lt;/email&gt;
    or call &lt;phone&gt;555-9876&lt;/phone&gt;
  &lt;/body&gt;
&lt;/memo&gt;</pre>
</td></tr></tbody></table>

The invoice contains several dates and telephone numbers, the postal
abbreviation for a state
(which comes from an enumerated list of sanctioned values), and a ZIP code
(which takes a definable regular form). The memo contains many
of the same types of information: a date, telephone number, email address
and an "importance" value (from an enumerated
list, such as "low", "medium" or "high"). Applications which process
invoices and memos need to raise exceptions if something that was
supposed to be a date or telephone number does not conform to the rules
for valid dates or telephone numbers.

In both cases, validity constraints exist on the content of the
instances that are not expressible in XML DTDs. The limited datatyping
facilities in XML have prevented validating XML processors from supplying
the rigorous type checking required in these situations. The result
has been that individual applications writers have had to implement type
checking in an ad hoc manner. This specification addresses
the need of both document authors and applications writers for a robust,
extensible datatype system for XML which could be incorporated into
XML processors. As discussed below, these datatypes could be used in other
XML-related standards as well.

<a id="requirements"></a>

### 1.2 Requirements

The [[XML Schema Requirements]](044-h-references.md#schema-requirements) document spells out
concrete requirements to be fulfilled by this specification,
which state that the XML Schema Language must:

1. provide for primitive data typing, including byte, date,
   integer, sequence, SQL and Java primitive datatypes, etc.;
2. define a type system that is adequate for import/export
   from database systems (e.g., relational, object, OLAP);
3. distinguish requirements relating to lexical data representation
   vs. those governing an underlying information set;
4. allow creation of user-defined datatypes, such as
   datatypes that are derived from existing datatypes and which
   may constrain certain of its properties (e.g., range,
   precision, length, format).

<a id="scope"></a>

### 1.3 Scope

This portion of the XML Schema Language discusses datatypes that can be
used in an XML Schema. These datatypes can be specified for element
content that would be specified as
[#PCDATA](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-chardata) and attribute
values of [various
types](https://www.w3.org/TR/2000/WD-xml-2e-20000814#sec-attribute-types)  in a DTD. It is the intention of this specification
that it be usable outside of the context of XML Schemas for a wide range
of other XML-related activities such as [[XSL]](044-h-references.md#XSL) and
[[RDF Schema]](044-h-references.md#RDFSchema).

