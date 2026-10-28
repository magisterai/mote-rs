# 4.2.2 Including modified component definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](052-4-schemas-and-namespaces-access-and-composition.md) · [Next](054-4-2-3-references-to-schema-components-across-namespaces.md)

<a id="c-normi2"></a>

Neither **SII** nor **SII’** have a `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

2.3

<a id="c-chami"></a>

**SII** has no `targetNamespace`
[[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) (but **SII’** does).

3 The appropriate **case** among the following
must be true:

3.1 **If** clause [2.1](052-4-schemas-and-namespaces-access-and-composition.md#c-normi) or clause [2.2](053-4-2-2-including-modified-component-definitions.md#c-normi2) above is satisfied, **then** the schema corresponding to **SII’** must include not only definitions or
declarations corresponding to the appropriate members of its own [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), but also components identical to all the [·schema components·](004-2-1-overview-of-xml-schema.md#key-component) of **I**.

3.2

<a id="c-docham"></a>

**If** clause [2.3](053-4-2-2-including-modified-component-definitions.md#c-chami) above is satisfied, **then** the schema corresponding to the
[<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)d
item's parent [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) must include not only definitions or
declarations corresponding to the appropriate members of its own [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element),
but also components identical to all the [·schema
components·](004-2-1-overview-of-xml-schema.md#key-component) of **I**, except that anywhere the [·absent·](008-3-schema-component-details.md#key-null) target
namespace name would have appeared, the [·actual value·](008-3-schema-component-details.md#key-vv) of the
`targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of **SII’** is used. In
particular, it replaces [·absent·](008-3-schema-component-details.md#key-null) in the following places:

3.2.1 The {target namespace} of named schema
components, both at the top level and (in the case of nested type
definitions and nested attribute and
element declarations whose `code` was *qualified*) nested within definitions;

3.2.2 The [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) of a wildcard, whether negated or not;

It is *not* an error for the [·actual value·](008-3-schema-component-details.md#key-vv) of the
`schemaLocation` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) to fail to resolve it all, in which case no
corresponding inclusion is performed. It
*is* an error for it to resolve but the rest of clause 1 above to
fail to be satisfied. Failure to resolve may well cause less than complete
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va) outcomes, of course.As discussed in [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing), [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName)s in XML representations may fail to
[·resolve·](050-3-15-2-1-references-to-schema-components.md#key-resolve), rendering components incomplete
and unusable because of missing subcomponents. During schema construction,
implementations must retain [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) values for such references, in case an appropriately-named component becomes available to discharge the reference by the time it is actually needed. [·Absent·](008-3-schema-component-details.md#key-null) target [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri)s of such as-yet unresolved reference [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName)s in [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)d components must also be converted if clause [3.2](053-4-2-2-including-modified-component-definitions.md#c-docham) is satisfied.

**Note:**The above is carefully worded so that multiple [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)ing of the same schema document will not constitute a violation of
clause [2](051-3-15-5-schema-information-set-contributions.md#c-nmd) of [Schema Properties Correct (§3.15.6)](051-3-15-5-schema-information-set-contributions.md#sch-props-correct), but applications are
allowed, indeed encouraged, to avoid [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)ing the same schema document more than once to forestall the necessity of establishing identity
component by component.

<a id="modify-schema"></a>

#### 4.2.2 Including modified component definitions

In order to provide some support for evolution and versioning, it is
possible to incorporate components corresponding to a schema document
*with modifications*. The modifications have a pervasive impact,
that is, only the redefined components are used, even when referenced from
other incorporated components, whether redefined themselves or not.

XML Representation Summary: `redefine` Element Information Item

<a id="element-redefine"></a>

<redefine  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **schemaLocation** = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation) | ([simpleType](045-3-14-simple-type-definitions.md#element-simpleType) | [complexType](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) | [group](032-3-7-model-group-definitions.md#element-group) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup)))\*  
</redefine>

A [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) information item may contain any number of [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine) elements. Their `schemaLocation` attributes, consisting of a URI reference, identify other [·schema documents·](008-3-schema-component-details.md#key-schemaDoc), that is [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) information items.

The [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema) corresponding
to [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) contains not only the components corresponding to its definition and declaration [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), but also
all the components of all the [·XML Schemas·](004-2-1-overview-of-xml-schema.md#key-schema) corresponding to any [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d schema documents.
Such schema documents must either (a) have the same
`targetNamespace` as the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing schema document, or
(b) no `targetNamespace` at all, in which case the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d schema document is converted to the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing schema document's `targetNamespace`.

The definitions within the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine) element itself are
restricted to be redefinitions of components from the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d
schema document, *in terms of themselves*. That is,

- Type
  definitions must use themselves as their base type definition;
- Attribute
  group definitions and model group definitions must be supersets or subsets of their original
  definitions, either by including exactly one
  reference to themselves or by containing only (possibly restricted) components
  which appear in a corresponding way in their [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d selves.

Not all the components of the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d
schema document need be redefined.

This mechanism is intended to provide a declarative and modular approach to
schema modification, with functionality no different except in scope from what
would be achieved by wholesale text copying and redefinition by editing. In
particular redefining a type is not guaranteed to be side-effect free: it may
have unexpected impacts on other type definitions which are based
on the redefined one, even to the extent that some such definitions become
ill-formed.

**Note:**The pervasive impact of redefinition reinforces the need for
implementations to adopt some form of lazy or 'just-in-time' approach to
component construction, which is also called for in order to avoid
inappropriate dependencies on the order in which definitions and references appear in (collections of) schema documents.

Example



```
v1.xsd:
 <xs:complexType name="personName">
  <xs:sequence>
   <xs:element name="title" minOccurs="0"/>
   <xs:element name="forename" minOccurs="0" maxOccurs="unbounded"/>
  </xs:sequence>
 </xs:complexType>

 <xs:element name="addressee" type="personName"/>

v2.xsd:
 <xs:redefine schemaLocation="v1.xsd">
  <xs:complexType name="personName">
   <xs:complexContent>
    <xs:extension base="personName">
     <xs:sequence>
      <xs:element name="generation" minOccurs="0"/>
     </xs:sequence>
    </xs:extension>
   </xs:complexContent>
  </xs:complexType>
 </xs:redefine>

 <xs:element name="author" type="personName"/>
  
```




The schema corresponding to `v2.xsd` has everything specified
by `v1.xsd`, with the `personName` type redefined, as
well as everything it specifies itself. According to
this schema, elements constrained
by the `personName` type may end with a `generation`
element. This includes not only the `author` element, but also the
`addressee` element.

<a id="src-redefine"></a>

**Schema Representation Constraint: Redefinition Constraints and Semantics**  

In addition to the conditions imposed on [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine) element
information items by the schema for schemas
**all** of the following must be true:

1 If there are any element information items among the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element)
other than [<annotation>](044-3-13-annotations.md#element-annotation) then the [·actual value·](008-3-schema-component-details.md#key-vv) of the
`schemaLocation` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must
successfully resolve.

2 If the [·actual value·](008-3-schema-component-details.md#key-vv) of the `schemaLocation` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element)
successfully resolves
**one** of the following must be true:

2.1 it resolves to (a fragment of) a resource which is an XML document
(see clause [1.1](052-4-schemas-and-namespaces-access-and-composition.md#c-vxd)), which in turn corresponds to a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
element information item in a well-formed information set, which in turn
corresponds to a valid schema.

2.2 It resolves to a [<schema>](049-3-15-schemas-as-a-whole.md#element-schema)
element information item in a well-formed information set, which in turn
corresponds to a valid schema.

In either case call the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) item **SII**, the valid
schema **I** and the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing item's parent [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) item **SII’**.

3 **One** of the following must be true:

3.1

<a id="c-normir"></a>

**SII** has a `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element), and its [·actual value·](008-3-schema-component-details.md#key-vv) is identical to the [·actual value·](008-3-schema-component-details.md#key-vv) of the `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of **SII’** (which must have such an [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element)).

3.2

<a id="c-normi2r"></a>

Neither **SII** nor **SII’** have a `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

3.3

<a id="c-chamir"></a>

**SII** has no `targetNamespace`
[[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) (but **SII’** does).

4 The appropriate **case** among the following
must be true:

4.1 **If** clause [3.1](053-4-2-2-including-modified-component-definitions.md#c-normir) or clause [3.2](053-4-2-2-including-modified-component-definitions.md#c-normi2r) above is satisfied, **then** the schema corresponding to **SII’** must include not only definitions or
declarations corresponding to the appropriate members of its own
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), but also components identical to all the [·schema components·](004-2-1-overview-of-xml-schema.md#key-component) of **I**, with the
exception of those explicitly redefined (see [Individual Component Redefinition (§4.2.2)](054-4-2-3-references-to-schema-components-across-namespaces.md#src-expredef) below).

