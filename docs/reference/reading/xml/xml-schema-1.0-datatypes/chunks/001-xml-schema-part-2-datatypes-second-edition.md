# XML Schema Part 2: Datatypes Second Edition

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-appendices.md)

[W3C](https://www.w3.org/)

<a id="title"></a>

# XML Schema Part 2: Datatypes Second Edition

<a id="w3c-doctype"></a>

## W3C Recommendation 28 October 2004

This version:
:   [http://www.w3.org/TR/2004/REC-xmlschema-2-20041028/](../README.md)

Latest version:
:   [http://www.w3.org/TR/xmlschema-2/](https://www.w3.org/TR/xmlschema-2/)

Previous version:
:   [http://www.w3.org/TR/2004/PER-xmlschema-2-20040318/](https://www.w3.org/TR/2004/PER-xmlschema-2-20040318/)

Editors:
:   Paul V. Biron, Kaiser Permanente, for Health Level Seven [<Paul.V.Biron@kp.org>](mailto:Paul.V.Biron@kp.org)
:   Ashok Malhotra, Microsoft (formerly of IBM)
    [<ashokma@microsoft.com>](mailto:ashokma@microsoft.com)

Please refer to the [**errata**](https://www.w3.org/2004/03/xmlschema-errata)
for this document, which may include some normative corrections.

This document is also available in these non-normative formats: [XML](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.xml), [XHTML with visible change markup](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes-with-errata.html), [Independent copy of the schema for schema documents](https://www.w3.org/2001/XMLSchema.xsd), [A schema for built-in datatypes only, in a separate namespace](https://www.w3.org/2001/XMLSchema-datatypes.xsd), and [Independent copy of the DTD for schema documents](https://www.w3.org/2001/XMLSchema.dtd).
See also [**translations**](https://www.w3.org/2003/03/Translations/byTechnology?technology=xmlschema).

[Copyright](https://www.w3.org/Consortium/Legal/ipr-notice#Copyright) © 2004 [W3C](https://www.w3.org/)<sup>®</sup> ([MIT](http://www.csail.mit.edu/), [ERCIM](http://www.ercim.org/), [Keio](http://www.keio.ac.jp/)), All Rights Reserved. W3C [liability](https://www.w3.org/Consortium/Legal/ipr-notice#Legal_Disclaimer), [trademark](https://www.w3.org/Consortium/Legal/ipr-notice#W3C_Trademarks) and [document use](https://www.w3.org/Consortium/Legal/copyright-documents)

rules apply.

---

<a id="abstract"></a>

## Abstract

*XML Schema: Datatypes* is part 2 of the specification of the XML
Schema language. It defines facilities for defining datatypes to be used
in XML Schemas as well as other XML specifications.
The datatype language, which is itself represented in
XML 1.0, provides a superset of the capabilities found in XML 1.0
document type definitions (DTDs) for specifying datatypes on elements
and attributes.

<a id="status"></a>

## Status of this Document

*This section describes the status of this document at the
time of its publication. Other documents may supersede this document.
A list of current W3C publications and the latest
revision of this technical report can be found in the [W3C technical reports index](https://www.w3.org/TR/) at
http://www.w3.org/TR/.*

This is a [W3C
Recommendation](https://www.w3.org/2004/02/Process-20040205/tr.html#RecsW3C), which forms part of the Second Edition of XML
Schema. This document has been reviewed by W3C Members and
other interested parties and has been endorsed by the Director as a
W3C Recommendation. It is a stable document and may be used as
reference material or cited as a normative reference
from another document.
W3C's role in making the Recommendation is to draw attention
to the specification and to promote its widespread deployment. This
enhances the functionality and interoperability of the Web.

This document has been produced by the [W3C XML Schema Working Group](https://www.w3.org/XML/Schema)
as part of the W3C [XML
Activity](https://www.w3.org/XML/Activity). The goals of the XML Schema language are discussed in
the [XML Schema
Requirements](https://www.w3.org/TR/NOTE-xml-schema-req) document. The authors of this document are the
members of the XML Schema Working Group. Different parts of this
specification have different editors.

This document was produced under the [24
January 2002 Current Patent Practice (CPP)](https://www.w3.org/TR/2002/NOTE-patent-practice-20020124) as amended by the [W3C Patent Policy
Transition Procedure](https://www.w3.org/2004/02/05-pp-transition). The Working Group maintains a [public
list of patent disclosures](https://www.w3.org/2002/11/xml-schema-IPR-statements.html) relevant to this document;
that page also includes instructions for disclosing a patent.
An individual who
has actual knowledge of a patent which the individual believes
contains Essential Claim(s) with respect to this specification should
disclose the information in accordance with [section
6 of the W3C Patent Policy](https://www.w3.org/Consortium/Patent-Policy-20040205/#sec-Disclosure).

The English version of this specification is the only normative
version. Information about translations of this document is available
at [http://www.w3.org/2001/05/xmlschema-translations](https://www.w3.org/2001/05/xmlschema-translations).

This second edition is *not* a new version,
it merely incorporates the changes dictated by the corrections to
errors found in the [first
edition](https://www.w3.org/TR/2001/REC-xmlschema-2-20010502/) as agreed by the XML Schema Working Group, as a
convenience to readers. A separate list of all such corrections is
available at [http://www.w3.org/2001/05/xmlschema-errata](https://www.w3.org/2001/05/xmlschema-errata).

The errata list for this second edition is available at [http://www.w3.org/2004/03/xmlschema-errata](https://www.w3.org/2004/03/xmlschema-errata).

Please report errors in this document to [www-xml-schema-comments@w3.org](mailto:www-xml-schema-comments@w3.org)
([archive](http://lists.w3.org/Archives/Public/www-xml-schema-comments/)).

**Note:**Ashok Malhotra's
affiliation has changed since the completion of
editorial work on this second edition. He is now at Oracle, and can be
contacted at [<ashok.malhotra@oracle.com>](mailto:ashok.malhotra@oracle.com).

<a id="contents"></a>

## Table of Contents

1 [Introduction](002-appendices.md#Intro)  
    1.1 [Purpose](002-appendices.md#purpose)  
    1.2 [Requirements](002-appendices.md#requirements)  
    1.3 [Scope](002-appendices.md#scope)  
    1.4 [Terminology](003-1-4-terminology.md#terminology)  
    1.5 [Constraints and Contributions](003-1-4-terminology.md#constraints-and-contributions)  
2 [Type System](003-1-4-terminology.md#typesystem)  
    2.1 [Datatype](003-1-4-terminology.md#datatype)  
    2.2 [Value space](003-1-4-terminology.md#value-space)  
    2.3 [Lexical space](004-2-3-lexical-space.md#lexical-space)  
    2.4 [Facets](004-2-3-lexical-space.md#facets)  
    2.5 [Datatype dichotomies](004-2-3-lexical-space.md#datatype-dichotomies)  
3 [Built-in datatypes](006-2-5-2-3-derived-by-union.md#built-in-datatypes)  
    3.1 [Namespace considerations](006-2-5-2-3-derived-by-union.md#namespaces)  
    3.2 [Primitive datatypes](006-2-5-2-3-derived-by-union.md#built-in-primitive-datatypes)  
    3.3 [Derived datatypes](013-3-2-19-notation.md#built-in-derived)  
4 [Datatype components](018-4-datatype-components.md#datatype-components)  
    4.1 [Simple Type Definition](018-4-datatype-components.md#rf-defn)  
    4.2 [Fundamental Facets](022-4-1-6-simple-type-definition-for-anysimpletype.md#rf-fund-facets)  
    4.3 [Constraining Facets](024-4-2-5-numeric.md#rf-facets)  
5 [Conformance](033-5-conformance.md#conformance)

