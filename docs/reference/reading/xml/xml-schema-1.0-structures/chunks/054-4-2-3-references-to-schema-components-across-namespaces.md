# 4.2.3 References to schema components across namespaces

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](053-4-2-2-including-modified-component-definitions.md) · [Next](055-continuation-55.md)

4.2 **If** clause [3.3](053-4-2-2-including-modified-component-definitions.md#c-chamir) above is satisfied, **then** the schema corresponding to **SII’** must include not only definitions or
declarations corresponding to the appropriate members of its own [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element),
but also components identical to all the [·schema
components·](004-2-1-overview-of-xml-schema.md#key-component) of **I**, with the
exception of those explicitly redefined (see [Individual Component Redefinition (§4.2.2)](054-4-2-3-references-to-schema-components-across-namespaces.md#src-expredef)
below), except that anywhere the [·absent·](008-3-schema-component-details.md#key-null) target
namespace name would have appeared, the [·actual value·](008-3-schema-component-details.md#key-vv) of the
`targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of **SII’** is
used (see clause [3.2](053-4-2-2-including-modified-component-definitions.md#c-docham) in [Inclusion Constraints and Semantics (§4.2.1)](052-4-schemas-and-namespaces-access-and-composition.md#src-include) for details).

5 Within the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), each [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType) must
have a [<restriction>](045-3-14-simple-type-definitions.md#element-restriction) among its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) and each [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) must have a `restriction` or `extension` among its grand-[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) the [·actual value·](008-3-schema-component-details.md#key-vv) of whose `base` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be the same as the [·actual value·](008-3-schema-component-details.md#key-vv) of its own `name` attribute plus target namespace;

6 Within the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), for each [<group>](032-3-7-model-group-definitions.md#element-group) the appropriate **case** among the following
must be true:

6.1 **If** it has a [<group>](032-3-7-model-group-definitions.md#element-group) among its contents at some level the [·actual value·](008-3-schema-component-details.md#key-vv) of whose
`ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is the same as the [·actual value·](008-3-schema-component-details.md#key-vv) of its own
`name` attribute plus target namespace, **then** 
**all** of the following must be true:

6.1.1 It must have exactly one such group.

6.1.2 The [·actual value·](008-3-schema-component-details.md#key-vv) of both that group's `minOccurs` and
`maxOccurs` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be `1` (or [·absent·](008-3-schema-component-details.md#key-null)).

6.2 **If** it has no such self-reference, **then** 
**all** of the following must be true:

6.2.1 The [·actual value·](008-3-schema-component-details.md#key-vv) of its own `name` attribute plus target
namespace must successfully [·resolve·](050-3-15-2-1-references-to-schema-components.md#key-resolve) to a
model group definition in **I**.

6.2.2 The [{model group}](032-3-7-model-group-definitions.md#model_group) of the model group definition which
corresponds to it per [XML Representation of Model Group Definition Schema Components (§3.7.2)](032-3-7-model-group-definitions.md#declare-namedModelGroup) must be a [·valid
restriction·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the [{model group}](032-3-7-model-group-definitions.md#model_group) of that model group
definition in **I**, as defined in [Particle Valid (Restriction) (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#cos-particle-restrict).

7 Within the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), for each [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup)
the appropriate **case** among the following
must be true:

7.1 **If** it has an [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup) among its contents the [·actual value·](008-3-schema-component-details.md#key-vv) of whose
`ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is the same as the [·actual value·](008-3-schema-component-details.md#key-vv) of its own
`name` attribute plus target namespace, **then** it must have exactly one such group.

7.2

<a id="c-agrere"></a>

**If** it has no such self-reference, **then** 
**all** of the following must be true:

7.2.1 The [·actual value·](008-3-schema-component-details.md#key-vv) of its own `name` attribute plus target
namespace must successfully [·resolve·](050-3-15-2-1-references-to-schema-components.md#key-resolve) to an
attribute group definition in **I**.

7.2.2 The [{attribute uses}](030-3-5-attributeuses.md#ag-attribute_declarations) and [{attribute wildcard}](030-3-5-attributeuses.md#ag-attribute_wildcard) of the attribute group definition which
corresponds to it per [XML Representation of Attribute Group Definition Schema Components (§3.6.2)](030-3-5-attributeuses.md#declare-attributeGroup) must be [·valid
restrictions·](036-3-9-5-particle-information-set-contributions.md#cd-model-restriction) of the [{attribute uses}](030-3-5-attributeuses.md#ag-attribute_declarations) and
[{attribute wildcard}](030-3-5-attributeuses.md#ag-attribute_wildcard) of that attribute group
definition in **I**, as defined in clause [2](028-continuation-28.md#c-rad), clause [3](028-continuation-28.md#c-rad2) and clause [4](028-continuation-28.md#c-raw) of [Derivation Valid (Restriction, Complex) (§3.4.6)](028-continuation-28.md#derivation-ok-restriction) (where references to the base type definition are understood as references to the attribute group
definition in **I**).

**Note:**An attribute group restrictively redefined per clause [7.2](054-4-2-3-references-to-schema-components-across-namespaces.md#c-agrere) corresponds to an attribute group whose [{attribute uses}](030-3-5-attributeuses.md#ag-attribute_declarations) consist all and only of those attribute uses corresponding to [<attribute>](009-3-2-attribute-declarations.md#element-attribute)s explicitly present among the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup). No inheritance from the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d attribute group occurs. Its [{attribute wildcard}](030-3-5-attributeuses.md#ag-attribute_wildcard) is similarly based purely on an explicit [<anyAttribute>](022-continuation-22.md#element-anyAttribute), if present.

<a id="src-expredef"></a>

**Schema Representation Constraint: Individual Component Redefinition**  

Corresponding to each non-[<annotation>](044-3-13-annotations.md#element-annotation) member of the
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of a [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine) there are one or two schema components in
the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing schema:

1 The [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType) and [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType)
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) information items each
correspond to two components:

1.1 One component which corresponds to the top-level definition item with
the same `name` in
the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d schema document, as defined in [Schema Component Details (§3)](008-3-schema-component-details.md#components), except that its {name} is [·absent·](008-3-schema-component-details.md#key-null);

1.2 One component which corresponds to the information item itself, as defined
in [Schema Component Details (§3)](008-3-schema-component-details.md#components), except that its {base type definition} is
the component defined in 1.1 above.

This pairing ensures the coherence constraints on type definitions
are respected, while at the same time achieving the desired effect, namely that
references to names of redefined components in both the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing and
[<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d schema documents resolve to the redefined component
as specified in 1.2 above.

2 The [<group>](032-3-7-model-group-definitions.md#element-group) and [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup)
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) each correspond to a single component, as defined in [Schema Component Details (§3)](008-3-schema-component-details.md#components), except that if and when a self-reference based on a `ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) whose [·actual value·](008-3-schema-component-details.md#key-vv) is the same as the item's `name` plus target namespace is resolved, a component which corresponds to the top-level definition item of that name and the appropriate kind in
**I** is used.

In all cases there must be a top-level definition item of the appropriate name and kind in
the [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)d schema document.

**Note:**The above is carefully worded so that multiple equivalent [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing of the same schema document will not constitute a violation of
clause [2](051-3-15-5-schema-information-set-contributions.md#c-nmd) of [Schema Properties Correct (§3.15.6)](051-3-15-5-schema-information-set-contributions.md#sch-props-correct), but applications are
allowed, indeed encouraged, to avoid [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine)ing the same
schema document in the same way more than once to forestall the necessity of
establishing identity component by component (although this will have to be
done for the individual redefinitions themselves).

<a id="composition-schemaImport"></a>

#### 4.2.3 References to schema components across namespaces

As described in [XML Schema Abstract Data Model (§2.2)](004-2-1-overview-of-xml-schema.md#concepts-data-model), every top-level schema component is associated with
a target namespace (or, explicitly, with none). This section sets out
the exact mechanism and syntax in the XML form of
schema definition by which a reference to a foreign component is made, that is, a component with a different target namespace from that of the referring component.

Two things are required: not only a means
of addressing such foreign components but also a signal to schema-aware processors that a
schema document contains such references:

XML Representation Summary: `import` Element Information Item

<a id="element-import"></a>

<import  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  namespace = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  schemaLocation = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</import>

The [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) element information item identifies namespaces
used in external references, i.e. those whose
[·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) identifies them as coming from a
different namespace (or none) than the enclosing schema document's
`targetNamespace`. The [·actual value·](008-3-schema-component-details.md#key-vv) of its `namespace`
[[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) indicates that the containing schema document may contain
qualified references to schema components in that namespace (via one or more
prefixes declared with namespace declarations in the normal way). If that
attribute is absent, then the import allows unqualified reference to components
with no target namespace.
Note that components to be imported need not be in the form of a
[·schema document·](008-3-schema-component-details.md#key-schemaDoc); the processor
is free to access or construct components using means of its own
choosing.

