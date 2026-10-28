# 3 Schema Component Details

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-2-5-names-and-symbol-spaces.md) · [Next](009-3-2-attribute-declarations.md)

<a id="components"></a>

## 3 Schema Component Details

<a id="scIntro"></a>

### 3.1 Introduction

3.1.1 [Components and Properties](008-3-schema-component-details.md#d0e1531)  
        3.1.2 [XML Representations of Components](008-3-schema-component-details.md#d0e1568)  
        3.1.3 [The Mapping between XML Representations and Components](008-3-schema-component-details.md#d0e1610)  
        3.1.4 [White Space Normalization during Validation](008-3-schema-component-details.md#d0e1654)

The following sections provide full details on the composition of all schema components, together
with their XML representations and their contributions to [·assessment·](004-2-1-overview-of-xml-schema.md#key-va). Each section is devoted to a single component, with separate subsections for

1. properties: their values and significance
2. XML representation and the mapping to properties
3. constraints on representation
4. validation rules
5. [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) contributions
6. constraints on the components themselves

The sub-sections immediately below introduce conventions and terminology used throughout the component sections.

<a id="d0e1531"></a>

#### 3.1.1 Components and Properties

Components are defined in terms of their
properties, and each property in turn is defined by giving its range,
that is the values it may have. This can be understood as
defining a schema as a labeled directed graph, where the root is a schema,
every other vertex is a schema
component or a literal (string, boolean, number) and every labeled edge is a
property. The graph is *not* acyclic: multiple copies of
components with the same name in the same [·symbol space·](007-2-5-names-and-symbol-spaces.md#key-symbolSpace) may not exist, so in some cases re-entrant chains
of properties must exist. Equality of components for the purposes of this
specification is always defined as equality of names (including target
namespaces) within symbol spaces.

**Note:**A schema and its components as defined in this chapter are an idealization of the information a schema-aware
processor requires: implementations are not constrained in how they provide
it. In particular, no implications about literal embedding versus indirection
follow from the use below of language such as "properties . . . having . . .
components as values".

<a id="key-null"></a>

[Definition:]Throughout this specification, the
term **absent** is used as a distinguished property value denoting absence.

Any property not
identified as optional is required to be present; optional properties which are
not present are taken to have [·absent·](008-3-schema-component-details.md#key-null) as their value. Any
property identified as a having a set, subset or list value may have an empty value unless this is explicitly
ruled out: this is *not* the same as [·absent·](008-3-schema-component-details.md#key-null). Any property value identified as a superset or subset of some set may be equal to that set, unless a proper superset or subset is explicitly called for.
By 'string' in Part 1 of this specification is meant a
sequence of ISO 10646 characters identified as [legal XML characters](https://www.w3.org/TR/REC-xml#charsets)
in [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml).

<a id="d0e1568"></a>

#### 3.1.2 XML Representations of Components

<a id="key-schemaDoc"></a>

The principal purpose of *XML Schema: Structures* is to define a set of
schema components that constrain the contents of instances and augment the
information sets thereof. Although no external representation
of schemas is required for this purpose, such representations will
obviously be widely used. To provide for this in an appropriate and
interoperable way, this specification provides a normative XML representation for schemas which
makes provision for every kind of schema
component. [Definition:]A document in
this form (i.e. a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item) is a **schema document**. For the schema document as a whole, and
its constituents, the sections below define correspondences between element
information items (with declarations in
[Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema) and [DTD for Schemas (non-normative) (§G)](065-g-dtd-for-schemas-non-normative.md#nonnormative-schemaDTD)) and
schema components. All the element information items in the XML representation
of a schema must be in the XML Schema namespace, that is their [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be `http://www.w3.org/2001/XMLSchema`. Although a common way of creating the XML Infosets which are or contain [·schema documents·](008-3-schema-component-details.md#key-schemaDoc) will be using an XML parser, this is not required: any mechanism which constructs conformant infosets as defined in [[XML-Infoset]](060-b-references-normative.md#ref-xmlinfo) is a possible starting point.

Two aspects of the XML representations of components presented in the
following sections are constant across them all:

1. All of them allow attributes qualified with namespace names other than
   the XML Schema namespace itself: these appear as annotations in the
   corresponding schema component;
2. All of them allow an [<annotation>](044-3-13-annotations.md#element-annotation) as their first child, for human-readable documentation and/or machine-targeted information.

<a id="d0e1610"></a>

#### 3.1.3 The Mapping between XML Representations and Components

For each kind of schema component there is a corresponding normative XML representation.
The sections below describe the correspondences between the properties of each kind of
schema component on the one hand and the properties of information items in
that XML representation on the other, together
with constraints on that representation above and beyond those implicit in the
[Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema).

The language used is as if the correspondences were mappings from XML representation to
schema component, but the mapping in the other direction, and therefore the
correspondence in the abstract, can always be
constructed therefrom.

<a id="key-vv"></a>

In discussing the mapping from XML representations to schema
components below, the value of a component property is often determined by the
value of an attribute information item, one of the [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of an element
information item. Since schema documents are constrained by the
[Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema), there is always a simple type
definition associated with any such attribute information item. [Definition:]The
phrase **actual value** is used to refer to the member of the value space of the
simple type definition associated with an attribute information item which corresponds to
its [·normalized value·](008-3-schema-component-details.md#key-nv). This will often be a string, but may also be an
integer, a boolean, a URI reference, etc. This term is also occasionally used with respect to element or attribute information items in a document being [·validated·](004-2-1-overview-of-xml-schema.md#key-va).

Many properties are identified below as having
other schema components or sets of components as values. For the purposes of exposition, the definitions in
this section assume that (unless the property is explicitly identified as
optional) all such values are in fact present. When schema
components are constructed from XML representations involving reference by name
to other components, this assumption may be violated if one or more references
cannot be resolved. This specification addresses the matter of missing
components in a uniform manner, described in [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing): no mention of
handling missing components will be found in the individual component
descriptions below.

Forward reference to named definitions and declarations *is*
allowed, both within and between [·schema documents·](008-3-schema-component-details.md#key-schemaDoc).
By the time the component corresponding to an XML representation which
contains a forward reference is actually needed for [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) an appropriately-named component may have become available to discharge the reference: see [Schemas and Namespaces: Access and Composition (§4)](052-4-schemas-and-namespaces-access-and-composition.md#composition) for details.

<a id="d0e1654"></a>

#### 3.1.4 White Space Normalization during Validation

<a id="key-iv"></a>

Throughout this specification, [Definition:]the
**initial value** of some
attribute information item is the value of the
[[normalized
value]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) property of that item. Similarly, the **initial value** of an element information item is the string composed of, in order, the
[[character code]](https://www.w3.org/TR/xml-infoset/#infoitem.character) of each character information item in the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of that
element information item.

The above definition means that comments and processing instructions,
even in the midst of text, are ignored for all [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) purposes.

<a id="key-nv"></a>

[Definition:]The
**normalized value** of an element or
attribute information item is an [·initial value·](008-3-schema-component-details.md#key-iv) whose white space, if any, has been
normalized according to the value of the [whiteSpace facet](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-whiteSpace) of the
simple type definition used in its [·validation·](004-2-1-overview-of-xml-schema.md#key-vn):

preserve
:   No normalization is done, the value is the [·normalized value·](008-3-schema-component-details.md#key-nv)

replace
:   All occurrences of `#x9` (tab), `#xA` (line feed) and
    `#xD` (carriage return) are replaced with `#x20` (space).

collapse
:   Subsequent to the replacements specified above under **replace**,
    contiguous sequences of `#x20`s are collapsed to a single
    `#x20`, and initial and/or final `#x20`s are deleted.

If the simple type definition used in an item's [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) is the [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType), the [·normalized value·](008-3-schema-component-details.md#key-nv) must be determined as in the **preserve** case above.

There are three alternative validation rules which may supply the
necessary background for the above: [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute) (clause [3](011-3-2-4-attribute-declaration-validation-rules.md#c-sva)), [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type) (clause [3.1.3](017-3-3-5-element-declaration-information-set-contributions.md#c-sv1)) or [Element Locally Valid (Complex Type) (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type) (clause [2.2](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#c-sv2)).

These three levels of normalization correspond to the processing mandated
in XML 1.0 for element content, CDATA attribute content and tokenized
attributed content, respectively. See [Attribute Value Normalization](https://www.w3.org/TR/REC-xml#AVNormalize) in [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml) for the precedent for **replace** and **collapse** for attributes. Extending this processing to element content is necessary to ensure a consistent [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) semantics for simple types, regardless of whether they are applied to attributes or elements. Performing it twice in the case of attributes whose [[normalized
value]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) has already been subject to replacement or collapse on the basis of
information in a DTD is necessary to ensure consistent treatment of attributes
regardless of the extent to which DTD-based information has been made use of
during infoset construction.

**Note:**Even when DTD-based information *has* been appealed to, and
[Attribute Value
Normalization](https://www.w3.org/TR/REC-xml#AVNormalize) has taken place, the above definition of [·normalized value·](008-3-schema-component-details.md#key-nv) may
mean *further* normalization takes place, as for instance when
character entity references in attribute values result in white space characters
other than spaces in their [·initial value·](008-3-schema-component-details.md#key-iv)s.

