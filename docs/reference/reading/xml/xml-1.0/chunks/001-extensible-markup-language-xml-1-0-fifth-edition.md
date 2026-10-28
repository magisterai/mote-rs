# Extensible Markup Language (XML) 1.0 (Fifth Edition)

Source: [Extensible Markup Language (XML) 1.0](https://www.w3.org/TR/2008/REC-xml-20081126/) · 5th edition, 2008-11-26.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2008/REC-xml-20081126/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-appendices.md)

[W3C](https://www.w3.org/)

<a id="title"></a>

# Extensible Markup Language (XML) 1.0 (Fifth Edition)

<a id="w3c-doctype"></a>

## W3C Recommendation 26 November 2008

<a id="notice-20130207"></a>

**Note:** On 7 February 2013, this specification was modified in place to replace broken links to RFC4646 and RFC4647.

This version:
:   [http://www.w3.org/TR/2008/REC-xml-20081126/](../README.md)

Latest version:
:   [http://www.w3.org/TR/xml/](https://www.w3.org/TR/xml/)

Previous versions:
:   [http://www.w3.org/TR/2008/PER-xml-20080205/](https://www.w3.org/TR/2008/PER-xml-20080205/)
      
    [http://www.w3.org/TR/2006/REC-xml-20060816/](https://www.w3.org/TR/2006/REC-xml-20060816/)

Editors:
:   Tim Bray, Textuality and Netscape [<tbray@textuality.com>](mailto:tbray@textuality.com)
:   Jean Paoli, Microsoft [<jeanpa@microsoft.com>](mailto:jeanpa@microsoft.com)
:   C. M. Sperberg-McQueen, W3C [<cmsmcq@w3.org>](mailto:cmsmcq@w3.org)
:   Eve Maler, Sun Microsystems, Inc. [<eve.maler@east.sun.com>](mailto:elm@east.sun.com)
:   François Yergeau

Please refer to the [**errata**](https://www.w3.org/XML/xml-V10-5e-errata) for this document, which may
include some normative corrections.

The [previous errata](https://www.w3.org/XML/xml-V10-4e-errata) for this document, are also available.

See also [**translations**](https://www.w3.org/2003/03/Translations/byTechnology?technology=xml).

This document is also available in these non-normative formats: [XML](https://www.w3.org/TR/2008/REC-xml-20081126/REC-xml-20081126.xml) and [XHTML with color-coded revision indicators](https://www.w3.org/TR/2008/REC-xml-20081126/REC-xml-20081126-review.html).

[Copyright](https://www.w3.org/Consortium/Legal/ipr-notice#Copyright) © 2008 [W3C](https://www.w3.org/)<sup>®</sup> ([MIT](http://www.csail.mit.edu/), [ERCIM](http://www.ercim.org/), [Keio](http://www.keio.ac.jp/)), All Rights Reserved. W3C [liability](https://www.w3.org/Consortium/Legal/ipr-notice#Legal_Disclaimer), [trademark](https://www.w3.org/Consortium/Legal/ipr-notice#W3C_Trademarks) and [document use](https://www.w3.org/Consortium/Legal/copyright-documents) rules apply.

---

<a id="abstract"></a>

## Abstract

The Extensible Markup Language (XML) is a subset of SGML that is completely
described in this document. Its goal is to enable generic SGML to be served,
received, and processed on the Web in the way that is now possible with HTML.
XML has been designed for ease of implementation and for interoperability
with both SGML and HTML.

<a id="status"></a>

## Status of this Document

*This section describes the status of this document at the time of its publication.
Other documents may supersede this document. A list of current W3C publications and the
latest revision of this technical report can be found in the [W3C technical reports index](https://www.w3.org/TR/) at
http://www.w3.org/TR/.*

This document specifies a syntax created by subsetting an existing, widely
used international text processing standard (Standard Generalized Markup Language,
ISO 8879:1986(E) as amended and corrected) for use on the World Wide Web.
It is a product of the [XML Core Working Group](https://www.w3.org/XML/Core/)
as part of the [XML Activity](https://www.w3.org/XML/Activity).
The English version of this specification is the only normative version. However,
for translations of this document, see [http://www.w3.org/2003/03/Translations/byTechnology?technology=xml](https://www.w3.org/2003/03/Translations/byTechnology?technology=xml).

This document is a [W3C Recommendation](https://www.w3.org/2005/10/Process-20051014/tr.html#q74). This fifth edition is *not* a new version of XML. As a convenience to readers,
it incorporates the changes dictated by the accumulated errata (available at
[http://www.w3.org/XML/xml-V10-4e-errata](https://www.w3.org/XML/xml-V10-4e-errata)) to the [Fourth
Edition of XML 1.0, dated 16 August 2006](https://www.w3.org/TR/2006/REC-xml-20060816/). In particular, erratum [[E09]](https://www.w3.org/XML/xml-V10-4e-errata#E09)
relaxes the restrictions on element and attribute names, thereby providing in XML 1.0 the major end user benefit
currently achievable only by using XML
1.1. As a consequence, many possible
documents which were not well-formed according to previous editions of this
specification are now well-formed, and previously invalid documents
using the newly-allowed name characters in, for example, ID
attributes, are now valid.

This edition supersedes the previous [W3C Recommendation
of 16 August 2006](https://www.w3.org/TR/2006/REC-xml-20060816/).

Please report errors in this document to
the public [xml-editor@w3.org](mailto:xml-editor@w3.org) mail list; public
[archives](http://lists.w3.org/Archives/Public/xml-editor/) are available. For the convenience of readers,
an [XHTML version with color-coded revision indicators](https://www.w3.org/TR/2008/REC-xml-20081126/REC-xml-20081126-review.html) is
also provided; this version highlights each change due to an erratum published in the
[errata
list](https://www.w3.org/XML/xml-V10-4e-errata) for the previous edition, together with a link to the particular
erratum in that list. Most of the
errata in the list provide a rationale for the change. The errata
list for this fifth edition is available at [http://www.w3.org/XML/xml-V10-5e-errata](https://www.w3.org/XML/xml-V10-5e-errata).

An implementation report is available at [http://www.w3.org/XML/2008/01/xml10-5e-implementation.html](https://www.w3.org/XML/2008/01/xml10-5e-implementation.html).
A [Test Suite](https://www.w3.org/XML/Test/) is maintained to help assessing conformance to this specification.

This document has been reviewed by W3C Members, by software developers, and by other W3C groups and interested parties, and is endorsed by the Director as a W3C Recommendation. It is a stable document and may be used as reference material or cited from another document. W3C's role in making the Recommendation is to draw attention to the specification and to promote its widespread deployment. This enhances the functionality and interoperability of the Web.

W3C maintains a [public list of
any patent disclosures](https://www.w3.org/2002/08/xmlcore-IPR-statements) made in connection with the deliverables of
the group; that page also includes instructions for disclosing a patent.
An individual who has actual knowledge of a patent which the individual
believes contains [Essential
Claim(s)](https://www.w3.org/Consortium/Patent-Policy-20040205/#def-essential) must disclose the information in accordance with
[section 6 of the W3C Patent Policy](https://www.w3.org/Consortium/Patent-Policy-20040205/#sec-Disclosure).

<a id="contents"></a>

## Table of Contents

1 [Introduction](002-appendices.md#sec-intro)  
    1.1 [Origin and Goals](002-appendices.md#sec-origin-goals)  
    1.2 [Terminology](002-appendices.md#sec-terminology)  
2 [Documents](003-2-documents.md#sec-documents)  
    2.1 [Well-Formed XML Documents](003-2-documents.md#sec-well-formed)  
    2.2 [Characters](003-2-documents.md#charsets)  
    2.3 [Common Syntactic Constructs](003-2-documents.md#sec-common-syn)  
    2.4 [Character Data and Markup](004-2-4-character-data-and-markup.md#syntax)  
    2.5 [Comments](004-2-4-character-data-and-markup.md#sec-comments)  
    2.6 [Processing Instructions](004-2-4-character-data-and-markup.md#sec-pi)  
    2.7 [CDATA Sections](004-2-4-character-data-and-markup.md#sec-cdata-sect)  
    2.8 [Prolog and Document Type Declaration](005-2-8-prolog-and-document-type-declaration.md#sec-prolog-dtd)  
    2.9 [Standalone Document Declaration](006-2-9-standalone-document-declaration.md#sec-rmd)  
    2.10 [White Space Handling](006-2-9-standalone-document-declaration.md#sec-white-space)  
    2.11 [End-of-Line Handling](006-2-9-standalone-document-declaration.md#sec-line-ends)  
    2.12 [Language Identification](007-2-12-language-identification.md#sec-lang-tag)  
3 [Logical Structures](007-2-12-language-identification.md#sec-logical-struct)  
    3.1 [Start-Tags, End-Tags, and Empty-Element Tags](008-3-1-start-tags-end-tags-and-empty-element-tags.md#sec-starttags)  
    3.2 [Element Type Declarations](008-3-1-start-tags-end-tags-and-empty-element-tags.md#elemdecls)  
        3.2.1 [Element Content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#sec-element-content)  
        3.2.2 [Mixed Content](008-3-1-start-tags-end-tags-and-empty-element-tags.md#sec-mixed-content)  
    3.3 [Attribute-List Declarations](009-3-3-attribute-list-declarations.md#attdecls)  
        3.3.1 [Attribute Types](009-3-3-attribute-list-declarations.md#sec-attribute-types)  
        3.3.2 [Attribute Defaults](009-3-3-attribute-list-declarations.md#sec-attr-defaults)  
        3.3.3 [Attribute-Value Normalization](009-3-3-attribute-list-declarations.md#AVNormalize)  
    3.4 [Conditional Sections](010-3-4-conditional-sections.md#sec-condition-sect)  
4 [Physical Structures](010-3-4-conditional-sections.md#sec-physical-struct)  
    4.1 [Character and Entity References](011-4-1-character-and-entity-references.md#sec-references)  
    4.2 [Entity Declarations](011-4-1-character-and-entity-references.md#sec-entity-decl)  
        4.2.1 [Internal Entities](011-4-1-character-and-entity-references.md#sec-internal-ent)  
        4.2.2 [External Entities](011-4-1-character-and-entity-references.md#sec-external-ent)  
    4.3 [Parsed Entities](012-4-3-parsed-entities.md#TextEntities)  
        4.3.1 [The Text Declaration](012-4-3-parsed-entities.md#sec-TextDecl)  
        4.3.2 [Well-Formed Parsed Entities](012-4-3-parsed-entities.md#wf-entities)  
        4.3.3 [Character Encoding in Entities](012-4-3-parsed-entities.md#charencoding)  
    4.4 [XML Processor Treatment of Entities and References](013-4-4-xml-processor-treatment-of-entities-and-references.md#entproc)  
        4.4.1 [Not Recognized](013-4-4-xml-processor-treatment-of-entities-and-references.md#not-recognized)  
        4.4.2 [Included](013-4-4-xml-processor-treatment-of-entities-and-references.md#included)  
        4.4.3 [Included If Validating](013-4-4-xml-processor-treatment-of-entities-and-references.md#include-if-valid)  
        4.4.4 [Forbidden](013-4-4-xml-processor-treatment-of-entities-and-references.md#forbidden)  
        4.4.5 [Included in Literal](013-4-4-xml-processor-treatment-of-entities-and-references.md#inliteral)  
        4.4.6 [Notify](013-4-4-xml-processor-treatment-of-entities-and-references.md#notify)  
        4.4.7 [Bypassed](013-4-4-xml-processor-treatment-of-entities-and-references.md#bypass)  
        4.4.8 [Included as PE](013-4-4-xml-processor-treatment-of-entities-and-references.md#as-PE)  
        4.4.9 [Error](013-4-4-xml-processor-treatment-of-entities-and-references.md#error)  
    4.5 [Construction of Entity Replacement Text](014-4-5-construction-of-entity-replacement-text.md#intern-replacement)  
    4.6 [Predefined Entities](014-4-5-construction-of-entity-replacement-text.md#sec-predefined-ent)  
    4.7 [Notation Declarations](014-4-5-construction-of-entity-replacement-text.md#Notations)  
    4.8 [Document Entity](014-4-5-construction-of-entity-replacement-text.md#sec-doc-entity)  
5 [Conformance](014-4-5-construction-of-entity-replacement-text.md#sec-conformance)  
    5.1 [Validating and Non-Validating Processors](014-4-5-construction-of-entity-replacement-text.md#proc-types)  
    5.2 [Using XML Processors](015-5-2-using-xml-processors.md#safe-behavior)  
6 [Notation](015-5-2-using-xml-processors.md#sec-notation)

