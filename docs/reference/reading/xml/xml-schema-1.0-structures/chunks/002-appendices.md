# Appendices

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-xml-schema-part-1-structures-second-edition.md) · [Next](003-1-3-documentation-conventions-and-terminology.md)

<a id="appendices"></a>

### Appendices

A [Schema for Schemas (normative)](058-5-3-missing-sub-components.md#normative-schemaSchema)  
B [References (normative)](060-b-references-normative.md#normative-references)  
C [Outcome Tabulations (normative)](060-b-references-normative.md#outcomes)  
    C.1 [Validation Rules](060-b-references-normative.md#validation_failures)  
    C.2 [Contributions to the post-schema-validation infoset](060-b-references-normative.md#PSVI_contributions)  
    C.3 [Schema Representation Constraints](061-c-3-schema-representation-constraints.md#outcome-src)  
    C.4 [Schema Component Constraints](062-c-4-schema-component-constraints.md#outcome-cos)  
D [Required Information Set Items and Properties (normative)](063-d-required-information-set-items-and-properties-normative.md#infoset)  
E [Schema Components Diagram (non-normative)](063-d-required-information-set-items-and-properties-normative.md#component-diagram)  
F [Glossary (non-normative)](063-d-required-information-set-items-and-properties-normative.md#normative-glossary)  
G [DTD for Schemas (non-normative)](065-g-dtd-for-schemas-non-normative.md#nonnormative-schemaDTD)  
H [Analysis of the Unique Particle Attribution Constraint (non-normative)](067-h-analysis-of-the-unique-particle-attribution-constraint-non-normative.md#non-ambig)  
I [References (non-normative)](067-h-analysis-of-the-unique-particle-attribution-constraint-non-normative.md#nonnormative-references)  
J [Acknowledgements (non-normative)](067-h-analysis-of-the-unique-particle-attribution-constraint-non-normative.md#acknowledgments)

---

<a id="intro"></a>

## 1 Introduction

This document sets out the structural part (*XML Schema: Structures*) of the XML Schema definition language.

Chapter 2 presents a [Conceptual Framework (§2)](003-1-3-documentation-conventions-and-terminology.md#concepts) for XML Schemas, including
an introduction to the nature of XML Schemas and an introduction
to the XML Schema abstract data model, along with
other terminology used throughout this document.

Chapter 3, [Schema Component Details (§3)](008-3-schema-component-details.md#components), specifies the precise
semantics of each component of the abstract model, the representation of each
component in XML, with reference to a DTD and XML Schema
for an XML Schema document type, along with a detailed mapping between the elements and
attribute vocabulary of this representation and the components and properties
of the abstract model.

Chapter 4 presents [Schemas and Namespaces: Access and Composition (§4)](052-4-schemas-and-namespaces-access-and-composition.md#composition), including the
connection between documents and schemas, the import, inclusion and redefinition of declarations and definitions and
the foundations of schema-validity assessment.

Chapter 5 discusses [Schemas and Schema-validity Assessment (§5)](057-5-schemas-and-schema-validity-assessment.md#conformance), including the
overall approach to schema-validity assessment of documents, and responsibilities of schema-aware
processors.

The normative appendices include a [Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema) for the XML representation of schemas and
[References (normative) (§B)](060-b-references-normative.md#normative-references).

The non-normative appendices include the [DTD for Schemas (non-normative) (§G)](065-g-dtd-for-schemas-non-normative.md#nonnormative-schemaDTD) and a [Glossary (non-normative) (§F)](063-d-required-information-set-items-and-properties-normative.md#normative-glossary).

This document is primarily intended as a language definition reference.
As such, although it contains a few examples, it is *not* primarily designed
to serve as a motivating introduction to the design and its features, or as a
tutorial for new users.
Rather it presents a careful and fully explicit definition of that design, suitable
for guiding implementations. For those in search of a step-by-step
introduction to the design, the non-normative [[XML Schema: Primer]](067-h-analysis-of-the-unique-particle-attribution-constraint-non-normative.md#bib-expo) is a much better
starting point than this document.

<a id="intro-purpose"></a>

### 1.1 Purpose

The purpose of *XML Schema: Structures* is to define the nature of XML schemas
and their component parts,
provide an inventory of XML markup
constructs with which to represent schemas, and define the
application of schemas to XML documents.

The purpose of an *XML Schema: Structures* schema is to define and describe a class of
XML documents by using schema components to constrain and document the meaning,
usage and relationships of their constituent parts: datatypes, elements and
their content and attributes and their values. Schemas may also provide for the specification of additional
document information, such as normalization and defaulting of attribute
and element values. Schemas have
facilities for self-documentation. Thus, *XML Schema: Structures* can be used to define, describe and catalogue XML
vocabularies for classes of XML documents.

Any application that consumes well-formed XML can use the *XML Schema: Structures*
formalism to express syntactic, structural and value constraints applicable to
its document instances. The *XML Schema: Structures* formalism allows a useful level of
constraint checking to be described and implemented for a wide spectrum of XML
applications. However, the language defined by this specification does not attempt to provide
*all* the facilities that might be needed by *any*
application. Some applications may require constraint capabilities not
expressible in this language, and so may need to perform their own additional
validations.

<a id="intro-relatedWork"></a>

### 1.2 Dependencies on Other Specifications

The definition of *XML Schema: Structures* depends on the following specifications:
[[XML-Infoset]](060-b-references-normative.md#ref-xmlinfo),
[[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces),
[[XPath]](060-b-references-normative.md#bib-xpath), and
[[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

See [Required Information Set Items and Properties (normative) (§D)](063-d-required-information-set-items-and-properties-normative.md#infoset) for a tabulation of the information items
and properties specified in [[XML-Infoset]](060-b-references-normative.md#ref-xmlinfo) which this
specification requires as a precondition to schema-aware processing.

