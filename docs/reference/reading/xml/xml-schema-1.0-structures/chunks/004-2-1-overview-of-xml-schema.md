# 2.1 Overview of XML Schema

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-1-3-documentation-conventions-and-terminology.md) · [Next](005-2-2-1-3-complex-type-definition.md)

<a id="d0e504"></a>

### 2.1 Overview of XML Schema

<a id="key-psvi"></a>

An XML Schema
consists of components such as type definitions
and element declarations. These can be used to assess the validity of
well-formed element and attribute information items (as defined
in [[XML-Infoset]](060-b-references-normative.md#ref-xmlinfo)), and furthermore
may specify augmentations to those items and their descendants. This augmentation makes explicit information which may have
been implicit in the original document, such as normalized and/or default values for
attributes and elements and
the types of element and attribute information items. [Definition:]We refer to the augmented infoset which results from conformant processing as defined in this specification as the **post-schema-validation infoset**, or PSVI.

Schema-validity assessment has two aspects:

1

<a id="c-lsv"></a>

Determining local schema-validity, that is
whether an element or attribute information item satisfies the
constraints embodied in the relevant
components of an XML Schema;

2 Synthesizing an overall validation outcome for the item,
combining local schema-validity with the results of schema-validity
assessments of its descendants, if any, and
adding appropriate augmentations to the infoset to record this outcome.

<a id="key-vn"></a>

Throughout this specification, [Definition:]the
word **valid** and its derivatives are used to refer to
clause [1](004-2-1-overview-of-xml-schema.md#c-lsv) above, the determination of local
schema-validity.

<a id="key-va"></a>

Throughout this specification, [Definition:] the word **assessment** is used to refer
to the overall process of
local validation, schema-validity assessment and infoset augmentation.

<a id="concepts-data-model"></a>

### 2.2 XML Schema Abstract Data Model

2.2.1 [Type Definition Components](004-2-1-overview-of-xml-schema.md#Type_Definition_Summary)  
        2.2.2 [Declaration Components](005-2-2-1-3-complex-type-definition.md#Declarations_Summary)  
        2.2.3 [Model Group Components](005-2-2-1-3-complex-type-definition.md#Model_Group_Summary)  
        2.2.4 [Identity-constraint Definition Components](005-2-2-1-3-complex-type-definition.md#Identity-constraint_Definition)  
        2.2.5 [Group Definition Components](005-2-2-1-3-complex-type-definition.md#Group_Definitions)  
        2.2.6 [Annotation Components](005-2-2-1-3-complex-type-definition.md#Annotation)

This specification builds on [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml) and
[[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces). The concepts and definitions used
herein regarding XML are framed at the abstract level of [information
items](https://www.w3.org/TR/xml-infoset/#infoitem) as defined in [[XML-Infoset]](060-b-references-normative.md#ref-xmlinfo). By
definition, this use of the infoset provides *a priori* guarantees of [well-formedness](https://www.w3.org/TR/REC-xml#sec-well-formed)
(as defined in [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml)) and [namespace
conformance](https://www.w3.org/TR/REC-xml-names/#Conformance) (as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces)) for
all candidates for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) and for all [·schema documents·](008-3-schema-component-details.md#key-schemaDoc).

Just as [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml) and
[[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces) can be described in terms of
information items, XML Schemas can be described in terms of an
abstract data model. In defining XML Schemas in terms of an abstract
data model, this specification rigorously specifies the information which
must be available to a conforming XML Schema processor. The abstract
model for schemas is conceptual only, and does not mandate any
particular implementation or representation of this information. To
facilitate interoperation and sharing of schema information, a
normative XML interchange format for schemas is provided.

<a id="key-component"></a>

<a id="key-schema"></a>

[Definition:] **Schema component** is the generic term for the building blocks that comprise the abstract data model of the schema.
[Definition:]
An **XML Schema** is a
set of [·schema components·](004-2-1-overview-of-xml-schema.md#key-component). There are 13 kinds of
component in all, falling into three groups. The primary components, which may
(type definitions) or must (element and attribute declarations) have names
are as follows:

- Simple type definitions
- Complex type definitions
- Attribute declarations
- Element declarations

The secondary components, which must have names, are as follows:

- Attribute group definitions
- Identity-constraint definitions
- Model group definitions
- Notation declarations

Finally, the "helper" components provide small parts of
other components; they are not independent of their context:

- Annotations
- Model groups
- Particles
- Wildcards
- Attribute Uses

<a id="key-declaration"></a>

During [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), [Definition:]**declaration** components are associated by
(qualified) name to information items being [·validated·](004-2-1-overview-of-xml-schema.md#key-vn).

<a id="key-definition"></a>

On the other hand, [Definition:]**definition** components define
internal schema components that can be used in other schema components.

<a id="key-compName"></a>

[Definition:]Declarations and
definitions may have and be identified by **name**s, which are NCNames as defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="key-targetNS"></a>

[Definition:]Several kinds
of component have a **target namespace**, which is either
[·absent·](008-3-schema-component-details.md#key-null) or a namespace name, also as
defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces). The [·target
namespace·](004-2-1-overview-of-xml-schema.md#key-targetNS) serves to identify the namespace within which the
association between the component and its name exists. In the case of
declarations, this in turn determines the namespace name of, for example, the element
information items it may [·validate·](004-2-1-overview-of-xml-schema.md#key-vn).

**Note:**At the abstract level, there is
no requirement that the components of a schema share a
[·target namespace·](004-2-1-overview-of-xml-schema.md#key-targetNS). Any schema for use in
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va) of documents containing names from more than one namespace
will of necessity include components with different [·target namespaces·](004-2-1-overview-of-xml-schema.md#key-targetNS). This contrasts with
the situation at the level of the XML representation of components, in which each schema document contributes
definitions and declarations to a single target namespace.

[·Validation·](004-2-1-overview-of-xml-schema.md#key-vn), defined in detail in [Schema Component Details (§3)](008-3-schema-component-details.md#components), is a
relation between information items and schema components. For example, an
attribute information item may [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an attribute
declaration, a list of element information items may [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) with
respect to a content model, and so on. The following sections briefly
introduce the kinds of components in the
schema abstract data model, other major features of the abstract
model, and how they contribute to [·validation·](004-2-1-overview-of-xml-schema.md#key-vn).

<a id="Type_Definition_Summary"></a>

#### 2.2.1 Type Definition Components

The abstract model provides two kinds of type definition component: simple
and complex.

<a id="key-typeDefn"></a>

[Definition:]This specification uses
the phrase **type definition** in cases where no distinction
need be made between simple and complex types.

Type definitions form a hierarchy with a single root. The subsections below first describe characteristics of that
hierarchy, then provide an introduction to simple and complex type definitions themselves.

<a id="Type_Derivation"></a>

##### 2.2.1.1 Type Definition Hierarchy

<a id="key-typeDefinitionHierarchy"></a>

[Definition:]Except for a distinguished [·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType), every [·type definition·](004-2-1-overview-of-xml-schema.md#key-typeDefn) is, by construction, either a
[·restriction·](004-2-1-overview-of-xml-schema.md#key-typeRestriction) or an [·extension·](004-2-1-overview-of-xml-schema.md#key-typeExtension) of some other type definition. The graph of these relationships forms a tree known as the **Type Definition Hierarchy**.

<a id="key-typeRestriction"></a>

[Definition:]A type
definition whose
declarations or facets are in a one-to-one relation with those of another
specified type
definition, with each in turn restricting the possibilities of the one it
corresponds to, is said to be a **restriction**.
The specific restrictions might include narrowed ranges or reduced
alternatives.
Members of a type, A, whose definition is a [·restriction·](004-2-1-overview-of-xml-schema.md#key-typeRestriction) of the definition of another type, B, are always members of type B as well.

<a id="key-typeExtension"></a>

[Definition:]A complex type definition
which allows element or attribute content in addition to that allowed by
another specified type
definition is said to be an **extension**.

<a id="key-urType"></a>

[Definition:]A distinguished complex
type definition, the **ur-type
definition**, whose
name is *anyType* in the XML Schema namespace, is present in each [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema), serving as the root of the type
definition hierarchy for that schema.

<a id="key-baseTypeDefinition"></a>

[Definition:]A type definition used as the
basis for an [·extension·](004-2-1-overview-of-xml-schema.md#key-typeExtension) or
[·restriction·](004-2-1-overview-of-xml-schema.md#key-typeRestriction) is known as
the **base type definition** of that definition.

<a id="Simple_Type_Definition"></a>

##### 2.2.1.2 Simple Type Definition

A simple type definition is a set of constraints on strings and information about the values they encode, applicable to the [·normalized value·](008-3-schema-component-details.md#key-nv) of an attribute
information item or of an element information item with no element children.
Informally, it applies to the values of attributes and the text-only content of elements.

<a id="key-simpleUrType"></a>

Each simple type definition, whether built-in (that is, defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2)) or
user-defined, is a [·restriction·](004-2-1-overview-of-xml-schema.md#key-typeRestriction) of some particular
simple [·base type
definition·](004-2-1-overview-of-xml-schema.md#key-baseTypeDefinition). For the built-in primitive type definitions, this is [Definition:]the **simple
ur-type definition**, a special restriction of the
[·ur-type
definition·](004-2-1-overview-of-xml-schema.md#key-urType), whose name is **anySimpleType** in the XML Schema namespace. The [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType) is considered to have an unconstrained lexical space, and a value space consisting of the union of the value spaces of all the built-in primitive datatypes and the set of all lists of all members of the value spaces of all the built-in primitive datatypes.

The mapping from lexical space to value space is
unspecified for items whose type definition is the
[·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType).
Accordingly this specification does not constrain processors' behaviour in
areas where this mapping is implicated, for example checking such items against
enumerations, constructing default attributes or elements whose declared type
definition is the
[·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType), checking
identity constraints involving such items.

**Note:**The Working Group expects to return to this area in a future
version of this specification.

