# 5 Conformance

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](032-4-3-11-3-totaldigits-validation-rules.md) · [Next](034-continuation-34.md)

<a id="conformance"></a>

## 5 Conformance

This specification describes two levels of conformance for
datatype processors. The first is
required of all processors. Support for the other will depend on the
application environments for which the processor is intended.

<a id="dt-minimally-conforming"></a>

[Definition:]
**Minimally conforming** processors [·must·](003-1-4-terminology.md#dt-must)
completely and correctly implement the [·Constraint on Schemas·](003-1-4-terminology.md#dt-cos) and
[·Validation Rule·](003-1-4-terminology.md#dt-cvc)
.

<a id="dt-interchange"></a>

[Definition:]
Processors which accept schemas in the form of XML documents as described
in [XML Representation of Simple Type Definition Schema Components (§4.1.2)](018-4-datatype-components.md#xr-defn) (and other relevant portions of
[Datatype components (§4)](018-4-datatype-components.md#datatype-components)) are additionally said to provide
**conformance to the XML Representation of Schemas**,
and [·must·](003-1-4-terminology.md#dt-must), when processing schema documents, completely and
correctly implement all
[·Schema Representation Constraint·](003-1-4-terminology.md#dt-src)s
in this specification, and [·must·](003-1-4-terminology.md#dt-must) adhere exactly to the
specifications in [XML Representation of Simple Type Definition Schema Components (§4.1.2)](018-4-datatype-components.md#xr-defn) (and other relevant portions of
[Datatype components (§4)](018-4-datatype-components.md#datatype-components)) for mapping
the contents of such
documents to [schema components](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-component)
for use in validation.

**Note:**
By separating the conformance requirements relating to the concrete
syntax of XML schema documents, this specification admits processors
which validate using schemas stored in optimized binary representations,
dynamically created schemas represented as programming language data
structures, or implementations in which particular schemas are compiled
into executable code such as C or Java. Such processors can be said to
be [·minimally conforming·](033-5-conformance.md#dt-minimally-conforming)
but not necessarily in [·conformance to
the XML Representation of Schemas·](033-5-conformance.md#dt-interchange).

<a id="schema"></a>

## A Schema for Datatype Definitions (normative)



