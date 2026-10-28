# Namespaces in XML 1.0 (Third Edition)

Source: [Namespaces in XML 1.0](https://www.w3.org/TR/2009/REC-xml-names-20091208/) · 3rd edition, 2009-12-08.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2009/REC-xml-names-20091208/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-1-motivation-and-summary.md)

[W3C](https://www.w3.org/)

<a id="title"></a>

# Namespaces in XML 1.0 (Third Edition)

<a id="w3c-doctype"></a>

## W3C Recommendation 8 December 2009

This version:
:   [http://www.w3.org/TR/2009/REC-xml-names-20091208/](../README.md)

Latest version:
:   [http://www.w3.org/TR/xml-names/](https://www.w3.org/TR/xml-names/)

Previous versions:
:   [http://www.w3.org/TR/2006/REC-xml-names-20060816/](https://www.w3.org/TR/2006/REC-xml-names-20060816/)
    [http://www.w3.org/TR/2009/PER-xml-names-20090806/](https://www.w3.org/TR/2009/PER-xml-names-20090806/)

Editors:
:   Tim Bray, Textuality [<tbray@textuality.com>](mailto:tbray@textuality.com)
:   Dave Hollander, Contivo, Inc. [<dmh@contivo.com>](mailto:dmh@contivo.com)
:   Andrew Layman, Microsoft [<andrewl@microsoft.com>](mailto:andrewl@microsoft.com)
:   Richard Tobin, University of Edinburgh and Markup Technology Ltd [<richard@inf.ed.ac.uk>](mailto:richard@inf.ed.ac.uk)
:   Henry S. Thompson, University of Edinburgh and W3C [<ht@w3.org>](mailto:ht@w3.org) - Third Edition

Please refer to the [**errata**](https://www.w3.org/XML/2009/xml-names-errata) for this document, which may
include normative corrections.

See also [**translations**](https://www.w3.org/2003/03/Translations/byTechnology?technology=xml-names).

This document is also available in these non-normative formats: [XML](https://www.w3.org/TR/2009/REC-xml-names-20091208/xml-names-10-3e.xml) and [HTML highlighting
differences from the second edition](https://www.w3.org/TR/2009/REC-xml-names-20091208/xml-names-10-3e-diff.html).

[Copyright](https://www.w3.org/Consortium/Legal/ipr-notice#Copyright) © 2009 [W3C](https://www.w3.org/)<sup>®</sup> ([MIT](http://www.csail.mit.edu/), [ERCIM](http://www.ercim.org/), [Keio](http://www.keio.ac.jp/)), All Rights Reserved. W3C [liability](https://www.w3.org/Consortium/Legal/ipr-notice#Legal_Disclaimer), [trademark](https://www.w3.org/Consortium/Legal/ipr-notice#W3C_Trademarks) and [document use](https://www.w3.org/Consortium/Legal/copyright-documents) rules apply.

---

<a id="abstract"></a>

## Abstract

XML namespaces provide a simple method for qualifying
element and attribute
names used in Extensible Markup Language documents by associating them
with namespaces identified by URI references.

<a id="status"></a>

## Status of this Document

*This section describes the status of this document at the time
of its publication. Other documents may supersede this document.
A list of current W3C publications and the latest revision of
this technical report can be found in the
[W3C technical reports index](https://www.w3.org/TR/)
at http://www.w3.org/TR/.*

This document is a product of the
[XML Core Working Group](https://www.w3.org/XML/Core/)
as part of the
[W3C XML Activity](https://www.w3.org/XML/Activity.html).
The English version of this specification is the only normative version.
However, for translations of this document, see
[http://www.w3.org/2003/03/Translations/byTechnology?technology=xml-names](https://www.w3.org/2003/03/Translations/byTechnology?technology=xml-names).

Known implementations are documented in the
[Namespaces 1.1 implementation report](https://www.w3.org/XML/2002/12/xml-names11-implementation.html)
(all known Namespaces 1.1 implementations also support Namespaces 1.0)
.
A test suite is also available via the
[XML Test Suite](https://www.w3.org/XML/Test/)
page.

This third edition incorporates all known errata as of the publication date.
It supersedes the previous
[edition of 16 August 2006](https://www.w3.org/TR/2006/REC-xml-names-20060816/).

This edition has been widely reviewed. Only minor editorial changes
have been made since the 6 August 2009 Proposed Edited Recommendation.

Please report errors in this document to
[xml-names-editor@w3.org](mailto:xml-names-editor@w3.org);
public
[archives](http://lists.w3.org/Archives/Public/xml-names-editor/)
are available. The errata list for this document
is available at
[http://www.w3.org/XML/2009/xml-names-errata](https://www.w3.org/XML/2009/xml-names-errata).

This document has been reviewed by W3C Members, by software
developers, and by other W3C groups and interested parties, and is
endorsed by the Director as a W3C Recommendation. It is a stable
document and may be used as reference material or cited from another
document. W3C's role in making the Recommendation is to draw attention
to the specification and to promote its widespread deployment. This
enhances the functionality and interoperability of the Web.

W3C maintains a
[public
list of any patent disclosures](https://www.w3.org/2002/08/xmlcore-IPR-statements)
made in connection with the deliverables of
the group; that page also includes instructions for disclosing a patent.
An individual who has actual knowledge of a patent which the individual
believes contains
[Essential
Claim(s)](https://www.w3.org/Consortium/Patent-Policy-20040205/#def-essential)
must disclose the information in accordance with
[section
6 of the W3C Patent Policy](https://www.w3.org/Consortium/Patent-Policy-20040205/#sec-Disclosure).

<a id="contents"></a>

## Table of Contents

1 [Motivation and Summary](002-1-motivation-and-summary.md#sec-intro)  
    1.1 [A Note on Notation and Usage](002-1-motivation-and-summary.md#notation)  
2 [XML Namespaces](002-1-motivation-and-summary.md#sec-namespaces)  
    2.1 [Basic Concepts](002-1-motivation-and-summary.md#concepts)  
    2.2 [Use of
URIs
as Namespace Names](002-1-motivation-and-summary.md#iri-use)  
    2.3 [Comparing
URI
References](002-1-motivation-and-summary.md#NSNameComparison)  
3 [Declaring Namespaces](002-1-motivation-and-summary.md#ns-decl)  
4 [Qualified Names](003-4-qualified-names.md#ns-qualnames)  
5 [Using Qualified Names](003-4-qualified-names.md#ns-using)  
6 [Applying Namespaces to Elements and Attributes](004-6-applying-namespaces-to-elements-and-attributes.md#scoping-defaulting)  
    6.1 [Namespace Scoping](004-6-applying-namespaces-to-elements-and-attributes.md#scoping)  
    6.2 [Namespace Defaulting](004-6-applying-namespaces-to-elements-and-attributes.md#defaulting)  
    6.3 [Uniqueness of Attributes](004-6-applying-namespaces-to-elements-and-attributes.md#uniqAttrs)  
7 [Conformance of Documents](004-6-applying-namespaces-to-elements-and-attributes.md#Conformance)  
8 [Conformance of Processors](005-8-conformance-of-processors.md#ProcessorConformance)

<a id="appendices"></a>

### Appendices

A [Normative References](005-8-conformance-of-processors.md#refs)  
B [Other references](005-8-conformance-of-processors.md#nrefs) (Non-Normative)  
C [The Internal Structure of XML Namespaces](005-8-conformance-of-processors.md#Philosophy) (Non-Normative)  
D [Changes since version 1.0](005-8-conformance-of-processors.md#changes) (Non-Normative)  
E [Acknowledgements](005-8-conformance-of-processors.md#sec-xml-and-sgml) (Non-Normative)  
F [Orphaned Productions](005-8-conformance-of-processors.md#orphans) (Non-Normative)

---

