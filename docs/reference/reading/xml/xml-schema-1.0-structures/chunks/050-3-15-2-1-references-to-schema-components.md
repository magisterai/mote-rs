# 3.15.2.1 References to Schema Components

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](049-3-15-schemas-as-a-whole.md) · [Next](051-3-15-5-schema-information-set-contributions.md)

Aside from [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include) and [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import), which do not correspond directly to any schema component at all, each of the element information
items which may appear in the content of [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) corresponds to
a schema component, and all except [<annotation>](044-3-13-annotations.md#element-annotation) are named. The
sections below
present each such item in turn, setting out the
components to which it may correspond.

<a id="refSchemaConstructs"></a>

##### 3.15.2.1 References to Schema Components

Reference to
schema components from a schema document is managed in a uniform way,
whether the component corresponds to an element information item from the same schema document or is imported
([References to schema components across namespaces (§4.2.3)](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport)) from an external schema (which may,
but need not, correspond to an actual schema document). The form
of all such references is a
[·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName).

<a id="gloss-QName"></a>

[Definition:]A **QName** is a name
with an optional namespace qualification, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces). When used in connection with the XML
representation of schema components or references to them, this refers to the
simple type [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName) as defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

<a id="gloss-NCName"></a>

[Definition:]An **NCName** is a name
with no colon, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces). When used in connection with the XML
representation of schema components in this specification, this refers to the
simple type [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName) as defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

In each of the XML
representation expositions in the following sections, an attribute is shown as
having type `QName` if and only if it is
interpreted as referencing a schema component.

Example



```
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
            xmlns:xhtml="http://www.w3.org/1999/xhtml"
            xmlns="http://www.example.com"
            targetNamespace="http://www.example.com">
  . . .

  <xs:element name="elem1" type="Address"/>

  <xs:element name="elem2" type="xhtml:blockquote"/>

  <xs:attribute name="attr1"
                type="xsl:quantity"/>
  . . .
</xs:schema>

```




The first of these is most probably a local reference, i.e. a reference
to a type
definition corresponding to a [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) element information item
located elsewhere in the schema document, the other two refer to type
definitions from schemas for other namespaces and assume that their namespaces
have been declared for import. See [References to schema components across namespaces (§4.2.3)](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport) for a discussion of importing.

<a id="d0e16826"></a>

##### 3.15.2.2 References to Schema Components from Elsewhere

The names of schema components such as type definitions and element
declarations are not of type [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID): they are not
unique within a schema, just within a symbol space. This means that simple
fragment identifiers will not always work to reference schema components from outside
the context of schema documents.

There is currently no provision in the definition of the interpretation
of fragment identifiers for the `text/xml` MIME type, which is the
MIME type for schemas, for referencing
schema components as such. However,
[[XPointer]](060-b-references-normative.md#ref-xpointer) provides a mechanism which maps well onto the
notion of symbol spaces as it is reflected in the XML representation of schema components. A fragment identifier of the form
`#xpointer(xs:schema/xs:element[@name="person"])` will uniquely identify
the representation of a top-level element declaration with name `person`, and similar fragment
identifiers can obviously be constructed for the other global symbol spaces.

Short-form fragment identifiers may also be used in some cases, that is
when a DTD or XML Schema is available for the schema in question, and the
provision of an `id` attribute for the representations of all primary and secondary schema
components, which *is* of type
[ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID), has been exploited.

It is a matter for applications to specify whether they interpret
document-level references of either of the above varieties as being to the relevant element information item (i.e. without
special recognition of the relation of schema documents to schema components) or as being to the
corresponding schema component.

<a id="d0e16860"></a>

#### 3.15.3 Constraints on XML Representations of Schemas

<a id="src-qname"></a>

**Schema Representation Constraint: QName Interpretation**  

Where the type of an attribute information item in a document involved in
[·validation·](004-2-1-overview-of-xml-schema.md#key-vn) is
identified as
[·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName), its [·actual value·](008-3-schema-component-details.md#key-vv) is composed of a

<a id="q-local"></a>

[Definition:]**local name** and a 

<a id="q-uri"></a>

[Definition:]**namespace name**. Its [·actual value·](008-3-schema-component-details.md#key-vv) is determined based on its [·normalized value·](008-3-schema-component-details.md#key-nv) and
the containing element information item's [[in-scope
namespaces]](https://www.w3.org/TR/xml-infoset/#infoitem.element) following [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces):The appropriate **case** among the following
must be true:

1 **If** its [·normalized value·](008-3-schema-component-details.md#key-nv) is prefixed, **then** 
**all** of the following must be true:

1.1 There must be a namespace in the [[in-scope
namespaces]](https://www.w3.org/TR/xml-infoset/#infoitem.element) whose [[prefix]](https://www.w3.org/TR/xml-infoset/#infoitem.namespace) matches the prefix.

1.2 its [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) is the
[[namespace
name]](https://www.w3.org/TR/xml-infoset/#infoitem.namespace) of that namespace.

1.3 Its [·local name·](050-3-15-2-1-references-to-schema-components.md#q-local) is the portion of
its [·normalized value·](008-3-schema-component-details.md#key-nv) after the colon (`':'`).

2 **otherwise** (its [·normalized value·](008-3-schema-component-details.md#key-nv) is unprefixed) **all** of the following must be true:

2.1 its [·local name·](050-3-15-2-1-references-to-schema-components.md#q-local) is its [·normalized value·](008-3-schema-component-details.md#key-nv).

2.2 The appropriate **case** among the following
must be true:

2.2.1 **If** there is a namespace in the [[in-scope
namespaces]](https://www.w3.org/TR/xml-infoset/#infoitem.element) whose [[prefix]](https://www.w3.org/TR/xml-infoset/#infoitem.namespace) has no value, **then** its [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) is the
[[namespace
name]](https://www.w3.org/TR/xml-infoset/#infoitem.namespace) of that namespace.

2.2.2 **otherwise** its [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) is [·absent·](008-3-schema-component-details.md#key-null).

In the absence of the [[in-scope namespaces]](https://www.w3.org/TR/xml-infoset/#infoitem.element) property in the infoset for the schema document in question, processors must reconstruct equivalent information as necessary, using the [[ namespace attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the containing element information item and its ancestors.

<a id="key-resolve"></a>

[Definition:]Whenever the word **resolve** in any form is used in this
chapter in connection with a [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) in a
schema document, the
following definition [QName resolution (Schema Document) (§3.15.3)](050-3-15-2-1-references-to-schema-components.md#src-resolve) should be understood:

<a id="src-resolve"></a>

**Schema Representation Constraint: QName resolution (Schema Document)**  

For a [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName)
to resolve to a schema component of a specified kind
**all** of the following must be true:

1 That component is a member of the value of the appropriate
property of the schema which corresponds to the schema
document within which the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName)
appears, that is
the appropriate **case** among the following
must be true:

1.1 **If** the kind specified is simple or complex type definition, **then** the property is the [{type definitions}](049-3-15-schemas-as-a-whole.md#type_definitions).

1.2 **If** the kind specified is attribute declaration, **then** the property is the [{attribute declarations}](049-3-15-schemas-as-a-whole.md#attribute_declarations).

1.3 **If** the kind specified is element declaration, **then** the property is the [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations).

1.4 **If** the kind specified
is attribute group, **then** the property is the [{attribute group definitions}](049-3-15-schemas-as-a-whole.md#attribute_group_definitions).

1.5 **If** the kind specified is
model group, **then** the property is the [{model group definitions}](049-3-15-schemas-as-a-whole.md#model_group_definitions).

1.6 **If** the kind specified is notation declaration, **then** the property is the [{notation declarations}](049-3-15-schemas-as-a-whole.md#notation_declarations).

2 The
component's {name} matches the [·local
name·](050-3-15-2-1-references-to-schema-components.md#q-local) of the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName);

3 The component's {target namespace} is identical to the [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) of the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName);

4 The appropriate **case** among the following
must be true:

4.1 **If** the [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) of the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) is [·absent·](008-3-schema-component-details.md#key-null), **then** 
**one** of the following must be true:

4.1.1 The [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item of the schema document containing the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) has no `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

4.1.2 The [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item of the that schema document contains an [<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) element
information item with no `namespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

4.2 **otherwise** the [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) of the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) is
the same as
**one** of the following:

4.2.1 The [·actual value·](008-3-schema-component-details.md#key-vv) of the `targetNamespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of
the [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item of the schema document containing the [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName).

4.2.2 The [·actual value·](008-3-schema-component-details.md#key-vv) of the `namespace` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of some
[<import>](054-4-2-3-references-to-schema-components-across-namespaces.md#element-import) element information item contained in the [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) element information item of that schema document.

.

<a id="d0e17196"></a>

#### 3.15.4 Validation Rules for Schemas as a Whole

As the discussion above at [Schema Component Details (§3)](008-3-schema-component-details.md#components) makes clear, at the level of schema components and [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), reference to components by name is normally not involved. In a
few cases, however, qualified names appearing in information items being
[·validated·](004-2-1-overview-of-xml-schema.md#key-vn) must be resolved to schema components by such lookup. The following
constraint is appealed to in these cases.

<a id="cvc-resolve-instance"></a>

**Validation Rule: QName resolution (Instance)**  

