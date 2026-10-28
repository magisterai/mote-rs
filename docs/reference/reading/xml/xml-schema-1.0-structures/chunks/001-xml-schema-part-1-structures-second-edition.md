# XML Schema Part 1: Structures Second Edition

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-appendices.md)

[W3C](https://www.w3.org/)

<a id="title"></a>

# XML Schema Part 1: Structures Second Edition

<a id="w3c-doctype"></a>

## W3C Recommendation 28 October 2004

This version:
:   [http://www.w3.org/TR/2004/REC-xmlschema-1-20041028/](../README.md)

Latest version:
:   [http://www.w3.org/TR/xmlschema-1/](https://www.w3.org/TR/xmlschema-1/)

Previous version:
:   [http://www.w3.org/TR/2004/PER-xmlschema-1-20040318/](https://www.w3.org/TR/2004/PER-xmlschema-1-20040318/)

Editors:
:   Henry S. Thompson, University of Edinburgh [<ht@cogsci.ed.ac.uk>](mailto:ht@cogsci.ed.ac.uk)
:   David Beech, Oracle Corporation [<David.Beech@oracle.com>](mailto:David.Beech@oracle.com)
:   Murray Maloney, for Commerce One [<murray@muzmo.com>](mailto:murray@muzmo.com)
:   Noah Mendelsohn, Lotus Development Corporation [<Noah\_Mendelsohn@lotus.com>](mailto:Noah_Mendelsohn@lotus.com)

Please refer to the [**errata**](https://www.w3.org/2004/03/xmlschema-errata)
for this document, which may include some normative corrections.

This document is also available in these non-normative formats: [XML](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.xml), [XHTML with visible change markup](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures-with-errata.html), [Independent copy of the schema for schema
documents](https://www.w3.org/2001/XMLSchema.xsd), and [Independent copy of the DTD for schema documents](https://www.w3.org/2001/XMLSchema.dtd).
See also [**translations**](https://www.w3.org/2003/03/Translations/byTechnology?technology=xmlschema).

[Copyright](https://www.w3.org/Consortium/Legal/ipr-notice#Copyright) © 2004 [W3C](https://www.w3.org/)<sup>®</sup> ([MIT](http://www.csail.mit.edu/), [ERCIM](http://www.ercim.org/), [Keio](http://www.keio.ac.jp/)), All Rights Reserved. W3C [liability](https://www.w3.org/Consortium/Legal/ipr-notice#Legal_Disclaimer), [trademark](https://www.w3.org/Consortium/Legal/ipr-notice#W3C_Trademarks) and [document use](https://www.w3.org/Consortium/Legal/copyright-documents)

rules apply.

---

<a id="abstract"></a>

## Abstract

*XML Schema: Structures* specifies the XML Schema definition language,
which offers facilities for describing the structure and constraining the contents
of XML 1.0 documents, including those which exploit the XML
Namespace facility. The schema language, which is itself represented in XML
1.0 and uses namespaces, substantially reconstructs and considerably extends the capabilities found in XML 1.0 document type
definitions (DTDs). This specification depends on *XML Schema Part 2:
Datatypes*.

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
edition](https://www.w3.org/TR/2001/REC-xmlschema-1-20010502/) as agreed by the XML Schema Working Group, as a
convenience to readers. A separate list of all such corrections is
available at [http://www.w3.org/2001/05/xmlschema-errata](https://www.w3.org/2001/05/xmlschema-errata).

The errata list for this second edition is available at [http://www.w3.org/2004/03/xmlschema-errata](https://www.w3.org/2004/03/xmlschema-errata).

Please report errors in this document to [www-xml-schema-comments@w3.org](mailto:www-xml-schema-comments@w3.org)
([archive](http://lists.w3.org/Archives/Public/www-xml-schema-comments/)).

**Note:**David Beech has retired since the publication of the first edition,
and can be reached at [davidbeech@earthlink.net](mailto:davidbeech@earthlink.net).Murray Maloney is no longer affiliated with Commerce One; his contact
details are unchanged.Noah Mendelsohn's affiliation has changed since the publication of the
first edition. He is now at IBM, and can be contacted at [noah\_mendelsohn@us.ibm.com](mailto:noah_mendelsohn@us.ibm.com)

<a id="contents"></a>

## Table of Contents

1 [Introduction](002-appendices.md#intro)  
    1.1 [Purpose](002-appendices.md#intro-purpose)  
    1.2 [Dependencies on Other Specifications](002-appendices.md#intro-relatedWork)  
    1.3 [Documentation Conventions and Terminology](003-1-3-documentation-conventions-and-terminology.md#intro-terminology)  
2 [Conceptual Framework](003-1-3-documentation-conventions-and-terminology.md#concepts)  
    2.1 [Overview of XML Schema](004-2-1-overview-of-xml-schema.md#d0e504)  
    2.2 [XML Schema Abstract Data Model](004-2-1-overview-of-xml-schema.md#concepts-data-model)  
    2.3 [Constraints and Validation Rules](006-2-3-constraints-and-validation-rules.md#concepts-schemaConstraints)  
    2.4 [Conformance](006-2-3-constraints-and-validation-rules.md#concepts-conformance)  
    2.5 [Names and Symbol Spaces](007-2-5-names-and-symbol-spaces.md#concepts-nameSymbolSpaces)  
    2.6 [Schema-Related Markup in
Documents Being Validated](007-2-5-names-and-symbol-spaces.md#Instance_Document_Constructions)  
    2.7 [Representation of Schemas on the World Wide Web](007-2-5-names-and-symbol-spaces.md#web-representation)  
3 [Schema Component Details](008-3-schema-component-details.md#components)  
    3.1 [Introduction](008-3-schema-component-details.md#scIntro)  
    3.2 [Attribute Declarations](009-3-2-attribute-declarations.md#cAttribute_Declarations)  
    3.3 [Element Declarations](013-3-3-element-declarations.md#cElement_Declarations)  
    3.4 [Complex Type Definitions](020-3-4-complex-type-definitions.md#Complex_Type_Definitions)  
    3.5 [AttributeUses](030-3-5-attributeuses.md#cAttributeUse)  
    3.6 [Attribute Group Definitions](030-3-5-attributeuses.md#cAttribute_Group_Definitions)  
    3.7 [Model Group Definitions](032-3-7-model-group-definitions.md#cModel_Group_Definitions)  
    3.8 [Model Groups](033-3-7-6-constraints-on-model-group-definition-schema-components.md#Model_Groups)  
    3.9 [Particles](035-3-9-particles.md#cParticles)  
    3.10 [Wildcards](038-3-10-wildcards.md#Wildcards)  
    3.11 [Identity-constraint Definitions](040-3-11-identity-constraint-definitions.md#cIdentity-constraint_Definitions)  
    3.12 [Notation Declarations](043-3-12-notation-declarations.md#cNotation_Declarations)  
    3.13 [Annotations](044-3-13-annotations.md#cAnnotations)  
    3.14 [Simple Type Definitions](045-3-14-simple-type-definitions.md#Simple_Type_Definitions)  
    3.15 [Schemas as a Whole](049-3-15-schemas-as-a-whole.md#Schemas)  
4 [Schemas and Namespaces: Access and Composition](052-4-schemas-and-namespaces-access-and-composition.md#composition)  
    4.1 [Layer 1: Summary of the Schema-validity Assessment Core](052-4-schemas-and-namespaces-access-and-composition.md#layer1)  
    4.2 [Layer 2: Schema Documents, Namespaces and Composition](052-4-schemas-and-namespaces-access-and-composition.md#layer2)  
    4.3 [Layer 3: Schema Document Access and Web-interoperability](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#composition-instances)  
5 [Schemas and Schema-validity Assessment](057-5-schemas-and-schema-validity-assessment.md#conformance)  
    5.1 [Errors in Schema Construction and Structure](057-5-schemas-and-schema-validity-assessment.md#conformance-schemaValidity)  
    5.2 [Assessing Schema-Validity](057-5-schemas-and-schema-validity-assessment.md#validation_outcome)  
    5.3 [Missing Sub-components](058-5-3-missing-sub-components.md#conformance-missing)  
    5.4 [Responsibilities of Schema-aware Processors](058-5-3-missing-sub-components.md#conformance-processorResponsibilities)

