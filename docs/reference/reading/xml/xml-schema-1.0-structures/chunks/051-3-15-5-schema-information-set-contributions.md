# 3.15.5 Schema Information Set Contributions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](050-3-15-2-1-references-to-schema-components.md) · [Next](052-4-schemas-and-namespaces-access-and-composition.md)

A pair of a local name and a namespace name (or [·absent·](008-3-schema-component-details.md#key-null))
resolve to a schema component of a specified kind in the context of [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) by appeal to the appropriate
property of the schema being used for the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va). Each such property indexes components by name. The property to use is determined by the kind of component specified, that is,
the appropriate **case** among the following
must be true:

1 **If** the kind specified is simple or complex type definition, **then** the property is the [{type definitions}](049-3-15-schemas-as-a-whole.md#type_definitions).

2 **If** the kind specified is attribute declaration, **then** the property is the [{attribute declarations}](049-3-15-schemas-as-a-whole.md#attribute_declarations).

3 **If** the kind specified is element declaration, **then** the property is the [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations).

4 **If** the kind specified
is attribute group, **then** the property is the [{attribute group definitions}](049-3-15-schemas-as-a-whole.md#attribute_group_definitions).

5 **If** the kind specified is
model group, **then** the property is the [{model group definitions}](049-3-15-schemas-as-a-whole.md#model_group_definitions).

6 **If** the kind specified is notation declaration, **then** the property is the [{notation declarations}](049-3-15-schemas-as-a-whole.md#notation_declarations).

The component resolved to is the entry in the table whose {name} matches the local name of the pair and whose {target namespace} is identical to the namespace name of the pair.

<a id="d0e17275"></a>

#### 3.15.5 Schema Information Set Contributions

<a id="sic-schema"></a>

**Schema Information Set Contribution: Schema Information**  

Schema components provide a wealth of information about the basis of
[·assessment·](004-2-1-overview-of-xml-schema.md#key-va), which may well be of relevance to
subsequent processing. Reflecting component structure into a form suitable for
inclusion in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) is the way this specification provides for making this
information available.Accordingly, 

<a id="key-iso"></a>

[Definition:] by an **item isomorphic** to a component is meant an information item whose type is equivalent to the component's, with one property per property of the component, with the same name, and value either the same atomic value, or an information item corresponding in the same way to its component value, recursively, as necessary.Processors must add a property in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi)
to the element information item at which [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) began, as follows:

PSVI Contributions
for element information items

<a id="e-schema_information"></a>

[schema information]
:   A set of **namespace schema information** information items, one for each namespace name which appears as the
    {target namespace} of any schema component in the schema used for that
    assessment, and one for [·absent·](008-3-schema-component-details.md#key-null) if any schema
    component in the schema had no {target namespace}. Each **namespace schema information** information item has the
    following properties and values:

    PSVI Contributions
    for namespace schema information information items

    <a id="nsi-schema_namespace"></a>

    [schema namespace]
    :   A namespace
        name or [·absent·](008-3-schema-component-details.md#key-null).

    <a id="nsi-schema_components"></a>

    [schema components]
    :   A (possibly
        empty) set of schema component information items, each one an [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to a component whose {target
        namespace} is the sibling [[schema namespace]](051-3-15-5-schema-information-set-contributions.md#nsi-schema_namespace)
        property above, drawn from the schema used for [·assessment·](004-2-1-overview-of-xml-schema.md#key-va).

    <a id="nsi-schema_documents"></a>

    [schema documents]
    :   A
        (possibly empty) set of **schema document** information items, with
        properties and values as follows, for each schema document which
        contributed components to the schema, and whose
        `targetNamespace` matches the sibling [[schema namespace]](051-3-15-5-schema-information-set-contributions.md#nsi-schema_namespace) property above (or whose
        `targetNamespace` was [·absent·](008-3-schema-component-details.md#key-null)
        but that contributed components to that namespace by being [<include>](052-4-schemas-and-namespaces-access-and-composition.md#element-include)d
        by a schema document with that `targetNamespace` as per [Assembling a schema for a single target namespace from multiple schema definition documents (§4.2.1)](052-4-schemas-and-namespaces-access-and-composition.md#compound-schema)):

        PSVI Contributions
        for schema document information items

        <a id="sd-document_location"></a>

        [document location]
        :   Either a URI reference, if available, otherwise
            [·absent·](008-3-schema-component-details.md#key-null)

        <a id="sd-document"></a>

        [document]
        :   A document
            information item, if available, otherwise [·absent·](008-3-schema-component-details.md#key-null).

The [{schema components}](051-3-15-5-schema-information-set-contributions.md#nsi-schema_components) property is provided for
processors which wish to provide a single access point to the
components of the schema which was used during [·assessment·](004-2-1-overview-of-xml-schema.md#key-va). Lightweight processors are free to leave it empty, but if it *is* provided, it must contain at a minimum all the top-level (i.e. named) components which actually figured in the [·assessment·](004-2-1-overview-of-xml-schema.md#key-va), either directly or (because an anonymous component which figured is contained within) indirectly.

<a id="sic-id"></a>

**Schema Information Set Contribution: ID/IDREF Table**  

In the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) a set of **ID/IDREF binding** information items is associated
with the [·validation root·](057-5-schemas-and-schema-validity-assessment.md#key-vr) element information
item:

PSVI Contributions
for element information items

<a id="e-ii_table"></a>

[ID/IDREF table]
:   A (possibly empty) set of
    **ID/IDREF binding** information items, as specified below.

<a id="key-eas"></a>

[Definition:]Let the
**eligible item set** be the set of consisting of every attribute or element
information item for which
**all** of the following are true

1 its [validation context] is the
[·validation root·](057-5-schemas-and-schema-validity-assessment.md#key-vr);

2 it was successfully [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an attribute
declaration as per [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute) or element declaration as per
[Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) (as appropriate) whose attribute [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition) or element [{type definition}](013-3-3-element-declarations.md#type_definition) (respectively) is the
built-in [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID), [IDREF](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#IDREF) or [IDREFS](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#IDREFS) simple type definition or a type derived from one of them.

Then there is one **ID/IDREF binding** in the [[ID/IDREF table]](051-3-15-5-schema-information-set-contributions.md#e-ii_table)
for every distinct string which is**one** of the following:

1 the [·actual value·](008-3-schema-component-details.md#key-vv) of a member of the [·eligible
item set·](051-3-15-5-schema-information-set-contributions.md#key-eas) whose type definition is or is derived from [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID) or [IDREF](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#IDREF);

2 one of the items in the [·actual value·](008-3-schema-component-details.md#key-vv) of a member of the [·eligible
item set·](051-3-15-5-schema-information-set-contributions.md#key-eas) whose type definition is or is derived from [IDREFS](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#IDREFS).

Each **ID/IDREF binding** has properties as follows:

PSVI Contributions
for ID/IDREF binding information items

<a id="iib-id"></a>

[id]
:   The string identified above.

<a id="iib-binding"></a>

[binding]
:   A set consisting of every element information item for which
    **all** of the following are true

    1 its [[validation context]](017-3-3-5-element-declaration-information-set-contributions.md#e-validation_context) is the
    [·validation root·](057-5-schemas-and-schema-validity-assessment.md#key-vr);

    2 it has an attribute information item in
    its [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) or an element information item in its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) which was [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by the
    built-in [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID) simple type definition or a type derived from it whose
    [schema normalized value] is the [[id]](051-3-15-5-schema-information-set-contributions.md#iib-id) of
    this **ID/IDREF binding**.

The net effect of the above is to have one entry for every string used as an
id, whether by declaration or by reference, associated with those elements, if
any, which actually purport to have that id. See [Validation Root Valid (ID/IDREF) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-id) above
for the validation rule which actually checks for errors here.

**Note:**The **ID/IDREF binding**
information item, unlike most other aspects of this
specification, is essentially an internal bookkeeping mechanism. It is introduced to
support the definition of [Validation Root Valid (ID/IDREF) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-id) above.
Accordingly, conformant processors may, but are *not* required to,
expose it in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi).
In other words, the above constraint may be read as saying [·assessment·](004-2-1-overview-of-xml-schema.md#key-va) proceeds *as if* such an infoset item existed.

<a id="coss-schema"></a>

#### 3.15.6 Constraints on Schemas as a Whole

All schemas (see [Schemas as a Whole (§3.15)](049-3-15-schemas-as-a-whole.md#Schemas)) must satisfy the following constraint.

<a id="sch-props-correct"></a>

**Schema Component Constraint: Schema Properties Correct**  

**All** of the following must be true:

1 The values of the properties of a schema must be as described in
the property tableau in
[The Schema Itself (§3.15.1)](049-3-15-schemas-as-a-whole.md#Schema_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing);

2

<a id="c-nmd"></a>

Each of the [{type definitions}](049-3-15-schemas-as-a-whole.md#type_definitions), [{element declarations}](049-3-15-schemas-as-a-whole.md#element_declarations), [{attribute group definitions}](049-3-15-schemas-as-a-whole.md#attribute_group_definitions), [{model group definitions}](049-3-15-schemas-as-a-whole.md#model_group_definitions) and [{notation declarations}](049-3-15-schemas-as-a-whole.md#notation_declarations) must not contain two or more schema components with the same {name} and {target
namespace}.

