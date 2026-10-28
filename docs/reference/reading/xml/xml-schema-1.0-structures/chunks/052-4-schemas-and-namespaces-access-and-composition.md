# 4 Schemas and Namespaces: Access and Composition

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](051-3-15-5-schema-information-set-contributions.md) · [Next](053-4-2-2-including-modified-component-definitions.md)

<a id="composition"></a>

## 4 Schemas and Namespaces: Access and Composition

This chapter defines the mechanisms by which this specification establishes the necessary
precondition for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va), namely access to
one or more schemas. This chapter also sets out in detail the relationship
between schemas and namespaces, as well as mechanisms for
modularization of schemas, including provision for incorporating definitions
and declarations from one schema in another, possibly with modifications.

[Conformance (§2.4)](006-2-3-constraints-and-validation-rules.md#concepts-conformance) describes three levels of conformance for schema
processors, and [Schemas and Schema-validity Assessment (§5)](057-5-schemas-and-schema-validity-assessment.md#conformance) provides a formal definition of
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va). This section sets out
in detail the 3-layer architecture implied by the three conformance levels.
The layers
are:

1. The [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) core, relating schema components and instance
   information items;
2. Schema representation: the connections between XML
   representations and schema components, including the
   relationships between namespaces and schema components;
3. XML Schema web-interoperability guidelines: instance->schema and
   schema->schema connections for the WWW.

Layer 1 specifies the manner in which a schema composed of schema components
can be applied to in the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) of an instance element information item. Layer 2 specifies the use of [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
elements in XML documents as the standard XML representation for
schema information in a broad range of computer systems and execution
environments. To support interoperation over the World Wide Web in particular,
layer 3 provides a set of conventions for schema reference on the
Web. Additional details on each of the three layers is provided in the sections below.

<a id="layer1"></a>

### 4.1 Layer 1: Summary of the Schema-validity Assessment Core

The fundamental purpose of the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) core is to define [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) for a single
element information item and its descendants with respect to a
complex type
definition. All processors are required to implement this core predicate in a
manner which conforms exactly to this specification.

[·assessment·](004-2-1-overview-of-xml-schema.md#key-va) is defined with reference to an [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema) (note *not* a
[·schema document·](008-3-schema-component-details.md#key-schemaDoc)) which consists of (at a minimum) the set of schema
components (definitions and declarations) required for that
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va). This is not a circular definition, but rather a
*post facto* observation: no element information item can
be fully assessed unless all the components required by any aspect of
its (potentially recursive) [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) are present in the schema.

As specified above, each schema component is associated directly or
indirectly with a target namespace, or explicitly with no namespace. In the case of multi-namespace documents,
components for more than one target namespace will co-exist in a schema.

Processors have the option to assemble (and perhaps to optimize or
pre-compile) the entire schema prior to the start of an [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) episode, or to
gather the schema lazily as individual components are required. In all
cases it is required that:

- The processor succeed in locating the [·schema components·](004-2-1-overview-of-xml-schema.md#key-component)
  transitively required to complete an [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) (note that components derived
  from [·schema documents·](008-3-schema-component-details.md#key-schemaDoc) can be integrated
  with components obtained through other means);
- no definition or declaration changes once it has been established;
- if the processor chooses to acquire declarations and definitions
  dynamically, that there be no side effects of such dynamic acquisition that
  would cause the results of [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) to differ from that which would have
  been obtained from the same schema components acquired in bulk.

**Note:** the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) core is defined in terms of schema components at the
abstract level, and no mention is made of the schema definition
syntax (i.e. [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)). Although many processors will acquire
schemas in this format, others may operate on compiled representations, on a
programmatic representation as exposed in some programming language, etc.

The obligation of a schema-aware processor as far as the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va)
core is concerned is to implement one or more of the options for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) given below in [Assessing Schema-Validity (§5.2)](057-5-schemas-and-schema-validity-assessment.md#validation_outcome). Neither the
choice of element information item for that [·assessment·](004-2-1-overview-of-xml-schema.md#key-va), nor which of the
means of initiating [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) are used, is within the scope of this specification.

Although [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) is defined recursively, it is also intended to be
implementable in streaming
processors. Such processors may choose to incrementally assemble the schema during
processing in response, for example, to encountering new namespaces.
The implication of the
invariants expressed above is that such incremental assembly must
result in an [·assessment·](004-2-1-overview-of-xml-schema.md#key-va)
outcome that is the
*same* as would
be given if [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) was undertaken again
with the final, fully assembled schema.

<a id="layer2"></a>

### 4.2 Layer 2: Schema Documents, Namespaces and Composition

4.2.1 [Assembling a schema for a single target namespace from multiple schema definition documents](052-4-schemas-and-namespaces-access-and-composition.md#compound-schema)  
        4.2.2 [Including modified component definitions](053-4-2-2-including-modified-component-definitions.md#modify-schema)  
        4.2.3 [References to schema components across namespaces](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport)

The sub-sections of [Schema Component Details (§3)](008-3-schema-component-details.md#components) define an
XML representation for type definitions and element declarations and so on,
specifying their target namespace and collecting them into schema documents.
The two following sections relate to assembling a complete schema for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) from multiple sources. They should *not* be understood as a form of text substitution, but rather as providing mechanisms for distributed definition of schema components, with appropriate schema-specific semantics.

**Note:** The core [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) architecture requires that a complete schema with
all the necessary declarations and definitions be
available. This may involve resolving both
instance->schema and schema->schema references. As observed earlier in [Conformance (§2.4)](006-2-3-constraints-and-validation-rules.md#concepts-conformance), the precise mechanisms for resolving such references are expected to evolve over time.
In support of such evolution, this specification observes the design principle that references from
one schema document to a schema use mechanisms that directly parallel those used to
reference a schema from an instance document.

**Note:**In the sections below, "schemaLocation" really belongs at layer 3.
For convenience, it is documented with the layer 2 mechanisms of import and
include, with which it is closely associated.

<a id="compound-schema"></a>

#### 4.2.1 Assembling a schema for a single target namespace from multiple schema definition documents

Schema components for a single target namespace can be assembled from
several [·schema documents·](008-3-schema-component-details.md#key-schemaDoc), that is several [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element
information items:

XML Representation Summary: `include` Element Information Item

<a id="element-include"></a>

<include  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **schemaLocation** = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</include>

A [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) information item may contain any number of [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include) elements. Their `schemaLocation` attributes, consisting of a URI reference, identify other [·schema documents·](008-3-schema-component-details.md#key-schemaDoc), that is [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) information items.

The [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema) corresponding
to [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) contains not only the components corresponding to its definition and declaration [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), but also
all the components of all the [·XML Schemas·](004-2-1-overview-of-xml-schema.md#key-schema) corresponding to any [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)d schema documents.
Such included schema documents must either (a) have the same
`targetNamespace` as the [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)ing schema document, or
(b) no `targetNamespace` at all, in which case the [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)d schema document is converted to the [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)ing schema document's `targetNamespace`.

<a id="src-include"></a>

**Schema Representation Constraint: Inclusion Constraints and Semantics**  

In addition to the conditions imposed on [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include) element
information items by the schema for schemas,
**all** of the following must be true:

1

<a id="c-ins"></a>

If the [·actual value·](008-3-schema-component-details.md#key-vv) of the `schemaLocation` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element)
successfully resolves
**one** of the following must be true:

1.1

<a id="c-vxd"></a>

It resolves to (a fragment of) a resource which is an XML
document (of type
`application/xml` or `text/xml` with an XML declaration
for preference, but this is not required), which in turn corresponds to a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
element information item in a well-formed information set, which in turn
corresponds to a valid schema.

1.2 It resolves to a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
element information item in a well-formed information set, which in turn
corresponds to a valid schema.

In either case call the [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)d [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) item **SII**, the valid
schema **I** and the [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)ing item's parent [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) item **SII’**.

2 **One** of the following must be true:

2.1

<a id="c-normi"></a>

**SII** has a `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element), and its [·actual value·](008-3-schema-component-details.md#key-vv) is identical to the [·actual value·](008-3-schema-component-details.md#key-vv) of the `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of **SII’** (which must have such an [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element)).

2.2

