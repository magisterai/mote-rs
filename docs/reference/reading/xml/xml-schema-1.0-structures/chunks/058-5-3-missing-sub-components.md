# 5.3 Missing Sub-components

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](057-5-schemas-and-schema-validity-assessment.md) · [Next](059-continuation-59.md)

<a id="conformance-missing"></a>

### 5.3 Missing Sub-components

At the beginning of [Schema Component Details (§3)](008-3-schema-component-details.md#components), attention is drawn to the
fact that most kinds of schema components have properties which are described therein
as having other components, or sets of other components, as values, but that
when components are constructed on the basis of their correspondence with
element information items in schema documents, such properties usually
correspond to [QNames](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName), and the
[·resolution·](050-3-15-2-1-references-to-schema-components.md#src-resolve) of such [QNames](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName) may fail, resulting in one or more values of or containing [·absent·](008-3-schema-component-details.md#key-null) where a component is mandated.

If at any time during [·assessment·](004-2-1-overview-of-xml-schema.md#key-va), an
element or attribute information item is being [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a component of any kind any of whose
properties has or contains such an [·absent·](008-3-schema-component-details.md#key-null) value,
the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) is modified, as following:

- In the case of attribute information items, the effect is
  as if clause [1](011-3-2-4-attribute-declaration-validation-rules.md#c-a1) of [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute) had failed;
- In the case of element information items, the effect is
  as if clause [1](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-ea) of [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) had failed;
- In the case of element information items, processors may choose to
  continue [·assessment·](004-2-1-overview-of-xml-schema.md#key-va): see [·lax assessment·](017-3-3-5-element-declaration-information-set-contributions.md#key-lva).

Because of the value specification for [[validation attempted]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-validation_attempted) in [Assessment Outcome (Element) (§3.3.5)](017-3-3-5-element-declaration-information-set-contributions.md#sic-e-outcome), if this situation ever arises, the
document as a whole cannot show a [[validation attempted]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-validation_attempted)
of *full*.

<a id="conformance-processorResponsibilities"></a>

### 5.4 Responsibilities of Schema-aware Processors

Schema-aware processors are responsible for processing XML documents,
schemas and schema documents, as appropriate given the level of conformance
(as defined in [Conformance (§2.4)](006-2-3-constraints-and-validation-rules.md#concepts-conformance)) they support,
consistently with the conditions set out above.

<a id="normative-schemaSchema"></a>

## A Schema for Schemas (normative)

The XML representation of the schema for schema documents is presented here
as a normative
part of the specification, and as an illustrative example of how the XML Schema
language can
define itself using its own constructs. The names of XML
Schema language types, elements, attributes and groups defined here
are evocative of their purpose, but are occasionally verbose.

There is some annotation in comments, but a fuller annotation will require
the use of embedded documentation facilities or a hyperlinked external
annotation for which tools are not yet readily available.

Since a schema document is an XML document, it has optional XML and doctype
declarations that are provided here for completeness. The root
`schema` element defines a new schema. Since this is a schema for
*XML Schema: Structures*, the `targetNamespace` references the XML Schema namespace itself.



