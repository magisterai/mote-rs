# 5 Schemas and Schema-validity Assessment

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](056-4-3-layer-3-schema-document-access-and-web-interoperability.md) · [Next](058-5-3-missing-sub-components.md)

<a id="conformance"></a>

## 5 Schemas and Schema-validity Assessment

The architecture of schema-aware processing allows for a rich characterization of XML documents: schema validity is not a binary predicate.

This specification distinguishes between errors in schema construction and structure, on the
one hand, and schema validation outcomes, on the other.

<a id="conformance-schemaValidity"></a>

### 5.1 Errors in Schema Construction and Structure

Before [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) can be attempted, a schema is required.
Special-purpose applications are free to determine a schema for use in [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) by whatever
means are appropriate, but general purpose processors should implement the
strategy set out in [Schema Document Location Strategy (§4.3.2)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema_reference), starting with the
namespaces declared in the document whose [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) is being undertaken, and the [·actual value·](008-3-schema-component-details.md#key-vv)s of the
`xsi:schemaLocation` and `xsi:noNamespaceSchemaLocation`
[[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) thereof, if any, along with any other information about
schema identity or schema document location provided by users in
application-specific ways, if any.

It is an
error if a
schema and all the components which are the value of any of its properties,
recursively, fail to satisfy all the relevant Constraints on Schemas set out in
the last section of each of the subsections of [Schema Component Details (§3)](008-3-schema-component-details.md#components).

If a schema is derived from one or more schema documents (that is, one or
more [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information items) based on the
correspondence rules set out in [Schema Component Details (§3)](008-3-schema-component-details.md#components) and [Schemas and Namespaces: Access and Composition (§4)](052-4-schemas-and-namespaces-access-and-composition.md#composition), two additional
conditions hold:

- It is an error if any such schema document would not be fully valid with respect
  to a schema corresponding to the [Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema), that
  is, following schema-validation with such a schema, the [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element
  information items would have a [[validation attempted]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-validation_attempted)
  property with value *full* or *partial* and a
  [[validity]](017-3-3-5-element-declaration-information-set-contributions.md#e-validity) property
  with value *valid*.
- It is an error if any such schema document is or contains any element
  information items which violate any of the relevant Schema Representation
  Constraints set out in [Schema Representation Constraints (§C.3)](061-c-3-schema-representation-constraints.md#outcome-src).

The three cases described above are the only types of error which this
specification defines. With respect to the processes of the checking of schema structure
and the construction of schemas corresponding to schema documents, this
specification imposes no restrictions on processors after an error is detected.
However [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) with respect to schema-like entities which do *not*
satisfy all the above conditions is incoherent. Accordingly, conformant
processors must not attempt to undertake [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) using such non-schemas.

<a id="validation_outcome"></a>

### 5.2 Assessing Schema-Validity

With a schema which satisfies the conditions expressed in [Errors in Schema Construction and Structure (§5.1)](057-5-schemas-and-schema-validity-assessment.md#conformance-schemaValidity) above, the schema-validity of an element information item can be assessed. Three primary approaches to this are possible:

1 The user or application identifies a complex type definition from among the
[{type definitions}](049-3-15-schemas-as-a-whole.md#type_definitions) of the schema, and appeals to [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt) (clause [1.2](017-3-3-5-element-declaration-information-set-contributions.md#c-td));

2

<a id="c-res"></a>

The user or application identifies a element declaration from among the
[{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations) of the schema, checks that its [{name}](013-3-3-element-declarations.md#e-name) and [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) match the [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) and [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the item, and appeals to [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt) (clause [1.1](017-3-3-5-element-declaration-information-set-contributions.md#c-ed));

3 The processor starts from [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt) with no
stipulated declaration or definition, and either [·strict·](017-3-3-5-element-declaration-information-set-contributions.md#key-sva) or [·lax·](017-3-3-5-element-declaration-information-set-contributions.md#key-lva) assessment ensues, depending on whether or not the element information and the schema determine either an element declaration (by name) or a type definition (via `xsi:type`) or not.

The outcome of this effort, in any case, will be manifest in the
[validation attempted]
and [validity] properties on
the element information item and its [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) and [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element),
recursively, as defined by [Assessment Outcome (Element) (§3.3.5)](017-3-3-5-element-declaration-information-set-contributions.md#sic-e-outcome) and [Assessment Outcome (Attribute) (§3.2.5)](011-3-2-4-attribute-declaration-validation-rules.md#sic-a-outcome). It is up to applications to decide what constitutes a successful outcome.

<a id="key-vr"></a>

Note that every element and attribute information item
participating in the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) will also have a [validation context] property
which refers back to the element information item at which [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) began. [Definition:]This item, that is the element information item at which [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) began, is called the **validation root**.

**Note:**This specification does not reconstruct the XML 1.0 notion of
*root* in either schemas or instances. Equivalent
functionality is provided for at [·assessment·](004-2-1-overview-of-xml-schema.md#key-va)
invocation, via clause [2](057-5-schemas-and-schema-validity-assessment.md#c-res) above.

**Note:**This specification has nothing normative to say about multiple [·assessment·](004-2-1-overview-of-xml-schema.md#key-va)
episodes. It should however be clear from the above that if a processor
restarts [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) with respect to a [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi)
some [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) contributions from the previous [·assessment·](004-2-1-overview-of-xml-schema.md#key-va)
may be overwritten. Restarting nonetheless may be useful, particularly at a node whose
[validation attempted] property is *none*, in which
case there are three obvious cases in which additional useful information may result:

- [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) was not attempted because
  of a [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) failure, but declarations
  and/or definitions are available for at least some of the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) or [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element);
- [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) was not attempted because a
  named definition or declaration was missing, but after further effort the
  processor has retrieved it.
- [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) was not attempted because
  it was *skip*ped, but the processor has at least some declarations
  and/or definitions available for at least some of the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) or [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

