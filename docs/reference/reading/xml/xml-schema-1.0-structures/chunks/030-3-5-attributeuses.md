# 3.5 AttributeUses

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](029-3-4-7-built-in-complex-type-definition.md) · [Next](031-3-6-3-constraints-on-xml-representations-of-attribute-group-definitions.md)

<a id="cAttributeUse"></a>

### 3.5 AttributeUses

3.5.1 [The Attribute Use Schema Component](030-3-5-attributeuses.md#AU_details)  
        3.5.2 [XML Representation of Attribute Use Components](030-3-5-attributeuses.md#d0e9471)  
        3.5.3 [Constraints on XML Representations of Attribute Uses](030-3-5-attributeuses.md#d0e9488)  
        3.5.4 [Attribute Use Validation Rules](030-3-5-attributeuses.md#d0e9493)  
        3.5.5 [Attribute Use Information Set Contributions](030-3-5-attributeuses.md#d0e9515)  
        3.5.6 [Constraints on Attribute Use Schema Components](030-3-5-attributeuses.md#coss-attruse)

An attribute use is a utility component which controls the occurrence and
defaulting behavior of attribute declarations. It plays the same role for
attribute declarations in complex types that particles play for element declarations.

Example



```
<xs:complexType>
 . . .
 <xs:attribute ref="xml:lang" use="required"/>
 <xs:attribute ref="xml:space" default="preserve"/>
 <xs:attribute name="version" type="xs:number" fixed="1.0"/>
</xs:complexType>
     
```




XML representations which all involve attribute uses, illustrating some of
the possibilities for controlling occurrence.

<a id="AU_details"></a>

#### 3.5.1 The Attribute Use Schema Component

The attribute use schema component has the following properties:

Schema Component: [Attribute Use](005-2-2-1-3-complex-type-definition.md#Attribute_Use)

<a id="required"></a>

{required}
:   A boolean.

<a id="attribute"></a>

{attribute declaration}
:   An attribute declaration.

<a id="au-value_constraint"></a>

{value constraint}
:   Optional. A pair
    consisting of a value and one of *default*, *fixed*.

[{required}](030-3-5-attributeuses.md#required) determines whether this use of an attribute
declaration requires an appropriate attribute information item to be present, or
merely allows it.

[{attribute declaration}](030-3-5-attributeuses.md#attribute) provides the attribute declaration itself,
which will in turn determine the simple type definition used.

[{value constraint}](030-3-5-attributeuses.md#au-value_constraint) allows for local specification of a
default or fixed value. This must be consistent with that of the [{attribute declaration}](030-3-5-attributeuses.md#attribute), in that if the [{attribute declaration}](030-3-5-attributeuses.md#attribute) specifies a fixed value, the only allowed [{value constraint}](030-3-5-attributeuses.md#au-value_constraint) is the same fixed value.

<a id="d0e9471"></a>

#### 3.5.2 XML Representation of Attribute Use Components

Attribute uses correspond to all uses of [<attribute>](009-3-2-attribute-declarations.md#element-attribute) which
allow a `use` attribute. These in turn correspond to
*two* components in each case, an attribute use and its [{attribute declaration}](030-3-5-attributeuses.md#attribute) (although note the latter is not new when the attribute use is a reference to a top-level attribute declaration). The appropriate mapping is described in [XML Representation of Attribute Declaration Schema Components (§3.2.2)](009-3-2-attribute-declarations.md#declare-attribute).

<a id="d0e9488"></a>

#### 3.5.3 Constraints on XML Representations of Attribute Uses

None as such.

<a id="d0e9493"></a>

#### 3.5.4 Attribute Use Validation Rules

<a id="cvc-au"></a>

**Validation Rule: Attribute Locally Valid (Use)**  

For an attribute information item to be[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an attribute use its [·normalized value·](008-3-schema-component-details.md#key-nv) must match the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the attribute use's [{value constraint}](030-3-5-attributeuses.md#au-value_constraint) value, if it is present and
*fixed*.

<a id="d0e9515"></a>

#### 3.5.5 Attribute Use Information Set Contributions

None as such.

<a id="coss-attruse"></a>

#### 3.5.6 Constraints on Attribute Use Schema Components

All attribute uses (see [AttributeUses (§3.5)](030-3-5-attributeuses.md#cAttributeUse)) must satisfy the following constraints.

<a id="au-props-correct"></a>

**Schema Component Constraint: Attribute Use Correct**  

**All** of the following must be true:

1 The values of the properties of an attribute use must be as described in
the property tableau in
[The Attribute Use Schema Component (§3.5.1)](030-3-5-attributeuses.md#AU_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 If the [{attribute declaration}](030-3-5-attributeuses.md#attribute) has a *fixed* [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint), then if the attribute use itself has a [{value constraint}](030-3-5-attributeuses.md#au-value_constraint), it must also be *fixed* and its value must match that of the [{attribute declaration}](030-3-5-attributeuses.md#attribute)'s [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint).

<a id="cAttribute_Group_Definitions"></a>

### 3.6 Attribute Group Definitions

3.6.1 [The Attribute Group Definition Schema Component](030-3-5-attributeuses.md#Attribute_Group_Definition_details)  
        3.6.2 [XML Representation of Attribute Group Definition Schema Components](030-3-5-attributeuses.md#declare-attributeGroup)  
        3.6.3 [Constraints on XML Representations of Attribute Group Definitions](031-3-6-3-constraints-on-xml-representations-of-attribute-group-definitions.md#d0e9758)  
        3.6.4 [Attribute Group Definition Validation Rules](031-3-6-3-constraints-on-xml-representations-of-attribute-group-definitions.md#d0e9831)  
        3.6.5 [Attribute Group Definition Information Set
Contributions](031-3-6-3-constraints-on-xml-representations-of-attribute-group-definitions.md#d0e9836)  
        3.6.6 [Constraints on Attribute Group Definition Schema Components](031-3-6-3-constraints-on-xml-representations-of-attribute-group-definitions.md#coss-attrGroup)

A schema can name a group of attribute declarations so that they may be incorporated as a
group into complex type definitions.

Attribute group definitions do not participate in [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) as such, but the
[{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) and [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard) of one or
more complex type definitions may be constructed in whole or part by reference
to an attribute group. Thus, attribute group definitions provide a
replacement for some uses of XML's
[parameter entity](https://www.w3.org/TR/REC-xml#dt-PE) facility.
Attribute group definitions are provided primarily for reference from the XML
representation of schema components
(see [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) and [<attributeGroup>](022-continuation-22.md#element-simpleContent..attributeGroup)).

Example



```
<xs:attributeGroup name="myAttrGroup">
    <xs:attribute . . ./>
    . . .
</xs:attributeGroup>

<xs:complexType name="myelement">
    . . .
    <xs:attributeGroup ref="myAttrGroup"/>
</xs:complexType>
```




XML representations for attribute group definitions. The effect is as if the attribute
declarations in the group were present in the type definition.

<a id="Attribute_Group_Definition_details"></a>

#### 3.6.1 The Attribute Group Definition Schema Component

The attribute group definition schema component has the
following properties:

Schema Component: [Attribute Group Definition](005-2-2-1-3-complex-type-definition.md#Attribute_Group_Definition)

<a id="ag-name"></a>

{name}
:   An NCName as defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="ag-target_namespace"></a>

{target namespace}
:   Either [·absent·](008-3-schema-component-details.md#key-null) or a namespace name, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="ag-attribute_declarations"></a>

{attribute uses}
:   A set of attribute uses.

<a id="ag-attribute_wildcard"></a>

{attribute wildcard}
:   Optional. A wildcard.

<a id="ag-annotation"></a>

{annotation}
:   Optional. An annotation.

Attribute groups are identified by their [{name}](030-3-5-attributeuses.md#ag-name) and [{target namespace}](030-3-5-attributeuses.md#ag-target_namespace); attribute group identities must be unique within an [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema). See [References to schema components across namespaces (§4.2.3)](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport) for the use of component
identifiers when importing one schema into another.

[{attribute uses}](030-3-5-attributeuses.md#ag-attribute_declarations) is a set attribute uses, allowing
for local specification of occurrence and default or fixed values.

[{attribute wildcard}](030-3-5-attributeuses.md#ag-attribute_wildcard) provides for an attribute wildcard to be included in an
attribute group.
See above under [Complex Type Definitions (§3.4)](020-3-4-complex-type-definitions.md#Complex_Type_Definitions) for the
interpretation of
attribute wildcards during [·validation·](004-2-1-overview-of-xml-schema.md#key-vn).

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](030-3-5-attributeuses.md#ag-annotation) property.

<a id="declare-attributeGroup"></a>

#### 3.6.2 XML Representation of Attribute Group Definition Schema Components

The XML representation for an attribute group definition schema component is an
[<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup) element information item. It provides for
naming a group of attribute declarations and an attribute wildcard for use by reference in the XML representation of
complex type definitions and other attribute group definitions. The correspondences between the
properties of the information item and
properties of the component it corresponds to are as follows:

XML Representation Summary: `attributeGroup` Element Information Item

<a id="element-attributeGroup"></a>

<attributeGroup  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  name = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  ref = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, (([attribute](009-3-2-attribute-declarations.md#element-attribute) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup))\*, [anyAttribute](022-continuation-22.md#element-anyAttribute)?))  
</attributeGroup>

When an [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup) appears as a daughter of
[<schema>](049-3-15-schemas-as-a-whole.md#element-schema) or [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine), it corresponds to an attribute group definition as
below. When it appears as a daughter of [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) or [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup), it does not correspond to any component as such.

