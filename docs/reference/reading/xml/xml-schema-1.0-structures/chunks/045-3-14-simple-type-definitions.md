# 3.14 Simple Type Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](044-3-13-annotations.md) · [Next](046-continuation-46.md)

<a id="Simple_Type_Definitions"></a>

### 3.14 Simple Type Definitions

3.14.1 [(non-normative) The Simple Type Definition Schema Component](045-3-14-simple-type-definitions.md#Simple_Type_Definition_details)  
        3.14.2 [(non-normative) XML Representation of Simple Type Definition Schema Components](045-3-14-simple-type-definitions.md#declare-datatype)  
        3.14.3 [(non-normative) Constraints on XML Representations of Simple Type Definitions](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#d0e15439)  
        3.14.4 [Simple Type Definition Validation Rules](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#d0e15503)  
        3.14.5 [Simple Type Definition Information Set
Contributions](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#d0e15574)  
        3.14.6 [Constraints on Simple Type Definition
Schema Components](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#coss-st)  
        3.14.7 [Built-in Simple Type Definition](048-3-14-7-built-in-simple-type-definition.md#d0e16395)

**Note:**This section consists of a combination of non-normative versions of
normative material from [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2), for local cross-reference
purposes, and normative material relating to the interface between schema
components defined in this specification and the simple type definition component.

Simple type definitions provide for constraining character information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of element and attribute
information items.

Example



```
<xs:simpleType name="fahrenheitWaterTemp">
 <xs:restriction base="xs:number">
  <xs:fractionDigits value="2"/>
  <xs:minExclusive value="0.00"/>
  <xs:maxExclusive value="100.00"/>
 </xs:restriction>
</xs:simpleType>
```




The XML representation of a simple type definition.

<a id="Simple_Type_Definition_details"></a>

#### 3.14.1 (non-normative) The Simple Type Definition Schema Component

The simple type definition schema component has the following properties:

Schema Component: [Simple Type Definition](004-2-1-overview-of-xml-schema.md#Simple_Type_Definition)

<a id="st-name"></a>

{name}
:   Optional. An NCName as defined by
    [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="st-target_namespace"></a>

{target namespace}
:   Either
    [·absent·](008-3-schema-component-details.md#key-null) or a namespace name, as defined in
    [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="st-base_type_definition"></a>

{base type definition}
:   A simple type definition, which may be the
    [·simple ur-type definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself).

<a id="facets"></a>

{facets}
:   A set of constraining
    facets.

<a id="fundamental_facets"></a>

{fundamental facets}
:   A set of
    fundamental facets.

<a id="st-final"></a>

{final}
:   A subset of {*extension*, *list*, *restriction*, *union*}.

<a id="variety"></a>

{variety}
:   One of {*atomic*, *list*,
    *union*}. Depending on the value of [{variety}](045-3-14-simple-type-definitions.md#variety), further properties are
    defined as follows:

    atomic
    :   <a id="st-primitive_type_definition"></a>

        {primitive type definition}
        :   A
            built-in primitive simple type definition.

    list
    :   <a id="st-item_type_definition"></a>

        {item type definition}
        :   A
            simple type definition.

    union
    :   <a id="st-member_type_definitions"></a>

        {member type definitions}
        :   A non-empty sequence of
            simple type definitions.

<a id="st-annotation"></a>

{annotation}
:   Optional. An
    annotation.

Simple types are identified by their [{name}](045-3-14-simple-type-definitions.md#st-name) and [{target namespace}](045-3-14-simple-type-definitions.md#st-target_namespace). Except
for anonymous simple types (those with no [{name}](045-3-14-simple-type-definitions.md#st-name)), since
type definitions (i.e. both simple and complex type definitions taken together) must be uniquely identified within an [·XML
Schema·](004-2-1-overview-of-xml-schema.md#key-schema), no simple type definition can have the same name as another
simple or complex type definition. Simple type [{name}](045-3-14-simple-type-definitions.md#st-name)s and [{target namespace}](020-3-4-complex-type-definitions.md#ct-target_namespace)s
are provided for reference from
instances (see [xsi:type (§2.6.1)](007-2-5-names-and-symbol-spaces.md#xsi_type)), and for use in the XML
representation of schema components
(specifically in [<element>](013-3-3-element-declarations.md#element-element) and [<attribute>](009-3-2-attribute-declarations.md#element-attribute)). See [References to schema components across namespaces (§4.2.3)](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport) for the use of component
identifiers when importing one schema into another.

**Note:**The [{name}](045-3-14-simple-type-definitions.md#st-name) of a simple type is not *ipso
facto* the [[(local) name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the
element or attribute information items [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by that definition. The connection between a
name and a type definition is described in [Element Declarations (§3.3)](013-3-3-element-declarations.md#cElement_Declarations) and [Attribute Declarations (§3.2)](009-3-2-attribute-declarations.md#cAttribute_Declarations).

A simple type definition with an empty specification for [{final}](045-3-14-simple-type-definitions.md#st-final) can be used as the
[{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) for other types derived by either of
extension or restriction, or as the [{item type definition}](045-3-14-simple-type-definitions.md#st-item_type_definition) in
the definition of a list, or in the [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) of
a union; the explicit values *extension*, *restriction*,
*list* and *union* prevent further
derivations by extension (to yield a complex type) and restriction (to yield a
simple type) and use in constructing lists and unions respectively.

[{variety}](045-3-14-simple-type-definitions.md#variety) determines whether the simple type corresponds to
an *atomic*, *list* or *union* type as defined by [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

As described in [Type Definition Hierarchy (§2.2.1.1)](004-2-1-overview-of-xml-schema.md#Type_Derivation), every simple type definition is
a [·restriction·](004-2-1-overview-of-xml-schema.md#key-typeRestriction) of some other simple
type (the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition)), which is the [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType) if and only if the type
definition in question is one of the built-in primitive datatypes, or a list or
union type definition which is not itself derived by restriction from a
list or union respectively. Each
*atomic* type is ultimately a restriction of exactly one such
built-in primitive datatype, which is its [{primitive type definition}](045-3-14-simple-type-definitions.md#st-primitive_type_definition).

[{facets}](045-3-14-simple-type-definitions.md#facets) for each simple type definition are selected from those defined in
[[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2). For *atomic* definitions, these are restricted to those appropriate for
the corresponding [{primitive type definition}](045-3-14-simple-type-definitions.md#st-primitive_type_definition). Therefore, the value
space and lexical space (i.e. what is [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by any atomic simple type) is determined by the
pair ([{primitive type definition}](045-3-14-simple-type-definitions.md#st-primitive_type_definition), [{facets}](045-3-14-simple-type-definitions.md#facets)).

As specified in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2), *list* simple type definitions [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) space separated tokens, each of
which conforms to a specified simple type definition, the [{item type definition}](045-3-14-simple-type-definitions.md#st-item_type_definition). The item type specified
must not itself be a *list* type, and must be one of the types identified in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2) as a
suitable item type for a list simple type. In this case the [{facets}](045-3-14-simple-type-definitions.md#facets)
apply to the list itself, and are restricted to those appropriate for lists.

A *union* simple type definition [·validates·](004-2-1-overview-of-xml-schema.md#key-vn) strings which satisfy at
least one of its [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions). As in the case of
*list*, the [{facets}](045-3-14-simple-type-definitions.md#facets)
apply to the union itself, and are restricted to those appropriate for unions.

The [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType)
must *not* be named as the [·base type definition·](045-3-14-simple-type-definitions.md#st-base_type_definition) of any user-defined atomic simple type definitions: as it has no constraining facets, this would be incoherent.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](045-3-14-simple-type-definitions.md#st-annotation) property.

<a id="declare-datatype"></a>

#### 3.14.2 (non-normative) XML Representation of Simple Type Definition Schema Components

**Note:**This section reproduces a version of material from [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2), for local cross-reference purposes.

XML Representation Summary: `simpleType` Element Information Item

<a id="element-simpleType"></a>

<simpleType  
  final =
(#all | List of (list | union | restriction))
  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  name = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([restriction](045-3-14-simple-type-definitions.md#element-restriction) | [list](046-continuation-46.md#element-list) | [union](046-continuation-46.md#element-union)))  
</simpleType>

<a id="element-restriction"></a>

<restriction  
  base = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([simpleType](045-3-14-simple-type-definitions.md#element-simpleType)?, ([minExclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minExclusive) | [minInclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minInclusive) | [maxExclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxExclusive) | [maxInclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxInclusive) | [totalDigits](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-totalDigits) | [fractionDigits](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-fractionDigits) | [length](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-length) | [minLength](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minLength) | [maxLength](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxLength) | [enumeration](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-enumeration) | [whiteSpace](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-whiteSpace) | [pattern](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-pattern))\*))  
</restriction>

