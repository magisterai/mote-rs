# 2.3 Constraints and Validation Rules

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-2-2-1-3-complex-type-definition.md) · [Next](007-2-5-names-and-symbol-spaces.md)

<a id="concepts-schemaConstraints"></a>

### 2.3 Constraints and Validation Rules

The [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml) specification describes two kinds of
constraints on XML documents: *well-formedness* and
*validity* constraints. Informally, the well-formedness constraints
are those imposed by the definition of XML itself (such as the rules for the
use of the < and > characters and the rules for proper nesting of
elements), while validity constraints are the further constraints on document
structure provided by a particular DTD.

The preceding section focused on [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), that is
the constraints on information items which schema components supply. In fact
however this specification provides four different kinds of normative statements about schema
components, their representations in XML and their contribution to the
[·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of information items:

Schema Component Constraint

<a id="gloss-cos"></a>

:   [Definition:]Constraints on the schema components themselves, i.e.
    conditions components must satisfy to be components at all. Located in the
    sixth sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Schema Component Constraints (§C.4)](062-c-4-schema-component-constraints.md#outcome-cos).

Schema Representation Constraint

<a id="gloss-src"></a>

:   [Definition:]Constraints on the
    representation of schema components in XML beyond those which are expressed
    in [Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema). Located in the
    third sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Schema Representation Constraints (§C.3)](061-c-3-schema-representation-constraints.md#outcome-src).

Validation Rules

<a id="gloss-cvc"></a>

:   [Definition:]Contributions to [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) associated
    with schema components. Located in the
    fourth sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Validation Rules (§C.1)](060-b-references-normative.md#validation_failures).

Schema Information Set Contribution

<a id="gloss-sic"></a>

:   [Definition:]Augmentations to [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi)s
    expressed by schema components, which follow
    as a consequence of [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) and/or [·assessment·](004-2-1-overview-of-xml-schema.md#key-va).
    Located in the
    fifth sub-section of the per-component sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components)
    and tabulated in [Contributions to the post-schema-validation infoset (§C.2)](060-b-references-normative.md#PSVI_contributions).

The last of these, schema information set
contributions, are not as new as they might at first seem. XML 1.0
validation augments the XML 1.0 information set in similar ways,
for example by
providing values for attributes not present in instances, and by implicitly
exploiting type information for normalization or access.
(As an example of the latter case, consider the
effect of `NMTOKENS` on attribute white space, and the semantics of
`ID` and `IDREF`.) By including schema
information set contributions, this specification makes explicit some features
that XML 1.0 left implicit.

<a id="concepts-conformance"></a>

### 2.4 Conformance

This specification describes three levels of conformance for schema aware processors. The first is
required of all processors. Support for the other two will depend on the application environments
for which the processor is intended.

<a id="key-minimallyConforming"></a>

[Definition:]**Minimally conforming** processors must completely and
correctly implement the [·Schema Component
Constraints·](006-2-3-constraints-and-validation-rules.md#gloss-cos), [·Validation Rules·](006-2-3-constraints-and-validation-rules.md#gloss-cvc),
and [·Schema Information
Set Contributions·](006-2-3-constraints-and-validation-rules.md#gloss-sic) contained in this specification.

<a id="key-interchange"></a>

[Definition:][·Minimally conforming·](006-2-3-constraints-and-validation-rules.md#key-minimallyConforming) processors which accept
schemas represented in the form of XML documents as described in [Layer 2: Schema Documents, Namespaces and Composition (§4.2)](052-4-schemas-and-namespaces-access-and-composition.md#layer2) are
additionally said to provide **conformance to the XML Representation of Schemas**.
Such processors must, when processing schema documents, completely and
correctly implement all [·Schema Representation
Constraints·](006-2-3-constraints-and-validation-rules.md#gloss-src) in this specification, and must adhere exactly to the
specifications in [Schema Component Details (§3)](008-3-schema-component-details.md#components) for mapping the contents of
such documents to [·schema
components·](004-2-1-overview-of-xml-schema.md#key-component) for use in [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) and [·assessment·](004-2-1-overview-of-xml-schema.md#key-va).

**Note:**By separating the conformance requirements relating to the concrete syntax of XML schema
documents, this specification admits processors
which use schemas stored in optimized binary
representations, dynamically created schemas represented as programming language data structures, or implementations in which particular schemas are compiled into executable code
such as C or Java. Such processors can be said to be [·minimally conforming·](006-2-3-constraints-and-validation-rules.md#key-minimallyConforming) but not necessarily in [·conformance to the XML Representation of Schemas·](006-2-3-constraints-and-validation-rules.md#key-interchange).

<a id="key-fullyConforming"></a>

[Definition:] **Fully conforming**
processors are network-enabled processors which are not only both [·minimally conforming·](006-2-3-constraints-and-validation-rules.md#key-minimallyConforming) and [·in conformance to the XML Representation of Schemas·](006-2-3-constraints-and-validation-rules.md#key-interchange), but which additionally must be capable of accessing
schema documents from the World Wide Web according to [Representation of Schemas on the World Wide Web (§2.7)](007-2-5-names-and-symbol-spaces.md#web-representation) and [How schema definitions are located on the Web (§4.3.2)](056-4-3-layer-3-schema-document-access-and-web-interoperability.md#schema-loc).
.

**Note:**Although this specification provides just these three standard levels of conformance, it is
anticipated that other conventions can be established in the future. For example, the World Wide
Web Consortium is considering conventions for packaging on the Web a variety of
resources relating to individual documents and namespaces. Should such
developments lead to new conventions for representing schemas, or for accessing them on the Web,
new levels of conformance can be established and named at that time. There is no need to modify
or republish this specification to define such additional levels of conformance.

See [Schemas and Namespaces: Access and Composition (§4)](052-4-schemas-and-namespaces-access-and-composition.md#composition) for a more detailed explanation of the
mechanisms supporting these levels of conformance.

