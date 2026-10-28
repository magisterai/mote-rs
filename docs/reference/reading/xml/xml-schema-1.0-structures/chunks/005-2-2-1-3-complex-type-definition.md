# 2.2.1.3 Complex Type Definition

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-2-1-overview-of-xml-schema.md) · [Next](006-2-3-constraints-and-validation-rules.md)

Simple types may
also be defined whose members are lists of items
themselves constrained by some other simple type definition, or whose
membership is the union of the memberships of some other simple type
definitions. Such list and union simple type definitions are also restrictions of the [·simple ur-type
definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType).

For detailed information on simple type definitions, see [Simple Type Definitions (§3.14)](045-3-14-simple-type-definitions.md#Simple_Type_Definitions) and [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2). The latter also defines an extensive inventory of
pre-defined simple types.

<a id="Complex_Type_Definition"></a>

##### 2.2.1.3 Complex Type Definition

A complex type definition is a set of attribute declarations and a content type, applicable to the [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) and
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of an element information item respectively. The content type may
require the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) to contain neither element nor character information
items (that is, to be empty), to be a string which belongs to a particular simple
type or to contain a sequence of element information items which conforms to a particular model group, with or without character information items as well.

Each complex type definition other than the
[·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType) is either

- a restriction of a complex [·base
  type definition·](004-2-1-overview-of-xml-schema.md#key-baseTypeDefinition)

or

- an [·extension·](004-2-1-overview-of-xml-schema.md#key-typeExtension) of a simple or complex [·base
  type definition·](004-2-1-overview-of-xml-schema.md#key-baseTypeDefinition).

A
complex type which extends another does so by having additional content model
particles at the end of the other definition's content model,
or by having additional attribute declarations, or both.

**Note:**This specification allows only appending, and not other kinds of
extensions. This decision
simplifies application processing required to cast instances from derived to
base type. Future versions may allow more kinds of extension, requiring more
complex transformations to effect casting.

For detailed information on complex type definitions, see [Complex Type Definitions (§3.4)](020-3-4-complex-type-definitions.md#Complex_Type_Definitions).

<a id="Declarations_Summary"></a>

#### 2.2.2 Declaration Components

There are three kinds of declaration component: element, attribute, and
notation. Each is described in a section below. Also included is a discussion
of element substitution groups, which is a feature provided in conjunction with
element declarations.

<a id="Element_Declaration"></a>

##### 2.2.2.1 Element Declaration

An element declaration is an association of a name with a type definition, either simple or
complex, an (optional) default value and a (possibly empty) set of identity-constraint
definitions. The association is either global or scoped to a containing complex type definition. A
top-level element declaration with name 'A' is broadly comparable to a pair of
DTD declarations as follows, where the associated type definition
fills in the ellipses:



```
<!ELEMENT A . . .>
<!ATTLIST A . . .>
```




Element declarations contribute to
[·validation·](004-2-1-overview-of-xml-schema.md#key-vn) as part of model group [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), when their defaults and type components are checked against an element
information item with a matching name and namespace, and by triggering
identity-constraint definition [·validation·](004-2-1-overview-of-xml-schema.md#key-vn).

For detailed information on element declarations, see [Element Declarations (§3.3)](013-3-3-element-declarations.md#cElement_Declarations).

<a id="Element_Equivalence_Class"></a>

##### 2.2.2.2 Element Substitution Group

In XML 1.0, the name and content of an element must correspond exactly to the element type referenced in the corresponding content model.

<a id="key-equivalenceClass"></a>

[Definition:]Through
the new mechanism of **element substitution groups**, XML Schemas provides a more powerful model supporting substitution of one named element for another.
Any top-level element declaration can serve as the defining member, or
head, for an element substitution group. Other top-level element declarations,
regardless of target namespace, can be designated as members of the
substitution group headed by this element. In a suitably enabled content
model, a reference to the head [·validates·](004-2-1-overview-of-xml-schema.md#key-vn) not just the head itself, but elements
corresponding to any other member of the substitution group as well.

All such members must have type definitions which are either the same as the
head's type definition or
restrictions or extensions of it.
Therefore, although the names of elements can vary widely as new
namespaces and members of the substitution group are defined, the
content of member elements is strictly limited according to the type
definition of the substitution group head.

Note that element substitution groups are not represented as separate components. They are
specified in the property values for element declarations (see [Element Declarations (§3.3)](013-3-3-element-declarations.md#cElement_Declarations)).

<a id="Attribute_Declaration"></a>

##### 2.2.2.3 Attribute Declaration

An attribute declaration is an association between a name and a simple type definition, together
with occurrence information and (optionally) a default value. The
association is either global, or local to its containing complex type definition. Attribute declarations contribute to
[·validation·](004-2-1-overview-of-xml-schema.md#key-vn) as part of complex type definition [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), when their
occurrence, defaults and type components are checked against an attribute
information item with a matching name and namespace.

For detailed information on attribute declarations, see [Attribute Declarations (§3.2)](009-3-2-attribute-declarations.md#cAttribute_Declarations).

<a id="Notation_Declaration"></a>

##### 2.2.2.4 Notation Declaration

A notation declaration is an association between a name and an identifier for a
notation. For an attribute information item to be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a
`NOTATION` simple type definition, its value must have been declared
with a notation declaration.

For detailed information on notation declarations, see [Notation Declarations (§3.12)](043-3-12-notation-declarations.md#cNotation_Declarations).

<a id="Model_Group_Summary"></a>

#### 2.2.3 Model Group Components

The model group, particle, and wildcard components contribute to
the portion of a complex type definition that controls an element
information item's content.

<a id="Model_Group"></a>

##### 2.2.3.1 Model Group

A model group is a constraint in the form of a grammar fragment that applies to
lists of element information items. It consists of a list of particles, i.e.
element declarations, wildcards and model groups. There are three varieties of
model group:

- Sequence (the element information items
  match the particles in sequential order);
- Conjunction (the element information items match the
  particles, in any order);
- Disjunction (the element information items match
  one of the particles).

For detailed information on model groups, see [Model Groups (§3.8)](033-3-7-6-constraints-on-model-group-definition-schema-components.md#Model_Groups).

<a id="Particle"></a>

##### 2.2.3.2 Particle

A particle is a term in the grammar for element content, consisting of either an element
declaration, a wildcard or a model group, together with
occurrence constraints. Particles contribute to
[·validation·](004-2-1-overview-of-xml-schema.md#key-vn) as part of complex type definition [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), when they allow anywhere
from zero to many element information items or sequences thereof, depending on
their contents and occurrence
constraints.

<a id="key-contentModel"></a>

[Definition:]A particle can
be used in a complex type definition to constrain the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn)
of the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of an element information item; such a particle is called
a **content model**.

**Note:***XML Schema: Structures* [·content models·](005-2-2-1-3-complex-type-definition.md#key-contentModel) are similar to but more expressive than
[[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml) content models; unlike [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml), *XML Schema: Structures* applies [·content models·](005-2-2-1-3-complex-type-definition.md#key-contentModel) to the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of both mixed and element-only content.

For detailed information on particles, see [Particles (§3.9)](035-3-9-particles.md#cParticles).

<a id="Attribute_Use"></a>

##### 2.2.3.3 Attribute Use

An attribute use plays a role similar to that of a particle, but for
attribute declarations: an attribute declaration within a complex type definition
is embedded within an attribute use, which specifies whether the declaration
requires or merely allows its attribute, and whether it has a default or fixed value.

<a id="Wildcard"></a>

##### 2.2.3.4 Wildcard

A wildcard is a special kind of particle which matches element and attribute information items dependent on their namespace name, independently
of their local names.

For detailed information on wildcards, see [Wildcards (§3.10)](038-3-10-wildcards.md#Wildcards).

<a id="Identity-constraint_Definition"></a>

#### 2.2.4 Identity-constraint Definition Components

An identity-constraint definition is an association between a name and one of
several varieties of
identity-constraint related to uniqueness and reference. All the
varieties use [[XPath]](060-b-references-normative.md#bib-xpath) expressions to pick out sets of
information items relative to particular target element
information items which are unique, or a key, or a [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) reference, within
a specified scope. An element information item is only [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with
respect to an element declaration
with identity-constraint definitions if those definitions are all satisfied for all the descendants
of that element information item which they pick out.

For detailed information on identity-constraint definitions, see [Identity-constraint Definitions (§3.11)](040-3-11-identity-constraint-definitions.md#cIdentity-constraint_Definitions).

<a id="Group_Definitions"></a>

#### 2.2.5 Group Definition Components

There are two kinds of convenience definitions provided to enable
the re-use of pieces of complex type definitions: model group definitions
and attribute group definitions.

<a id="Model_Group_Definition"></a>

##### 2.2.5.1 Model Group Definition

A model group definition is an association between a name and a model group,
enabling re-use of the same model group in several complex type
definitions.

For detailed information on model group definitions, see [Model Group Definitions (§3.7)](032-3-7-model-group-definitions.md#cModel_Group_Definitions).

<a id="Attribute_Group_Definition"></a>

##### 2.2.5.2 Attribute Group Definition

An attribute group definition is an association between a name and a set of attribute declarations,
enabling re-use of the same set in several complex type
definitions.

For detailed information on attribute group definitions, see [Attribute Group Definitions (§3.6)](030-3-5-attributeuses.md#cAttribute_Group_Definitions).

<a id="Annotation"></a>

#### 2.2.6 Annotation Components

An annotation is information for human and/or mechanical
consumers. The interpretation of such information is
not defined in this specification.

For detailed information on annotations, see [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations).

