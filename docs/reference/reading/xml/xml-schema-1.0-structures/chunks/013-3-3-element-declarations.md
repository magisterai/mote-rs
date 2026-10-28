# 3.3 Element Declarations

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-3-2-6-constraints-on-attribute-declaration-schema-components.md) · [Next](014-continuation-14.md)

<a id="cElement_Declarations"></a>

### 3.3 Element Declarations

3.3.1 [The Element Declaration Schema Component](013-3-3-element-declarations.md#Element_Declaration_details)  
        3.3.2 [XML Representation of Element Declaration Schema Components](013-3-3-element-declarations.md#declare-element)  
        3.3.3 [Constraints on XML Representations of Element Declarations](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#d0e4233)  
        3.3.4 [Element Declaration Validation Rules](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#d0e4333)  
        3.3.5 [Element Declaration Information Set Contributions](017-3-3-5-element-declaration-information-set-contributions.md#d0e5070)  
        3.3.6 [Constraints on Element Declaration Schema Components](018-3-3-6-constraints-on-element-declaration-schema-components.md#coss-element)

Element declarations provide for:

- Local [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of element information item values using a type definition;
- Specifying default or fixed values for an element information items;
- Establishing uniquenesses and reference constraint relationships among the values of related elements and
  attributes;
- Controlling the substitutability of elements through the
  mechanism of [·element substitution groups·](005-2-2-1-3-complex-type-definition.md#key-equivalenceClass).

Example



```
<xs:element name="PurchaseOrder" type="PurchaseOrderType"/>

<xs:element name="gift">
 <xs:complexType>
  <xs:sequence>
   <xs:element name="birthday" type="xs:date"/>
   <xs:element ref="PurchaseOrder"/>
  </xs:sequence>
 </xs:complexType>
</xs:element>
```




XML representations of several different types of element declaration

<a id="Element_Declaration_details"></a>

#### 3.3.1 The Element Declaration Schema Component

The element declaration schema component has the following
properties:

Schema Component: [Element Declaration](005-2-2-1-3-complex-type-definition.md#Element_Declaration)

<a id="e-name"></a>

{name}
:   An NCName as defined by
    [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="e-target_namespace"></a>

{target namespace}
:   Either [·absent·](008-3-schema-component-details.md#key-null) or
    a namespace name, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="type_definition"></a>

{type definition}
:   Either a simple type
    definition or a complex type definition.

<a id="e-scope"></a>

{scope}
:   Optional. Either *global* or a complex type
    definition.

<a id="e-value_constraint"></a>

{value constraint}
:   Optional. A
    pair consisting of a value and one of *default*, *fixed*.

<a id="nillable"></a>

{nillable}
:   A boolean.

<a id="identity-constraint_definitions"></a>

{identity-constraint definitions}
:   A set
    of constraint definitions.

<a id="class_exemplar"></a>

{substitution group affiliation}
:   Optional. A top-level
    element definition.

<a id="e-final"></a>

{substitution group exclusions}
:   A subset of
    {*extension*,
    *restriction*}.

<a id="e-exact"></a>

{disallowed substitutions}
:   A subset of {*substitution*, *extension*,
    *restriction*}.

<a id="e-abstract"></a>

{abstract}
:   A boolean.

<a id="e-annotation"></a>

{annotation}
:   Optional. An annotation.

The [{name}](013-3-3-element-declarations.md#e-name) property must match the local part of the names
of element information items being [·validated·](004-2-1-overview-of-xml-schema.md#key-vn).

A [{scope}](013-3-3-element-declarations.md#e-scope) of *global* identifies element declarations available for use in content
models throughout the schema. Locally scoped declarations are available for use only within the
complex type identified by the [{scope}](013-3-3-element-declarations.md#e-scope) property. This property is [·absent·](008-3-schema-component-details.md#key-null) in the case of declarations within named model groups: their scope is determined when they are used in the construction of complex type definitions.

A non-[·absent·](008-3-schema-component-details.md#key-null) value of the [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) property provides for [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of
namespace-qualified element information items. [·Absent·](008-3-schema-component-details.md#key-null) values of
[{target namespace}](013-3-3-element-declarations.md#e-target_namespace) [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) unqualified items.

An element information item is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn)
if it satisfies the [{type definition}](013-3-3-element-declarations.md#type_definition). For such an
item, schema information set contributions appropriate to the [{type definition}](013-3-3-element-declarations.md#type_definition) are added to the
corresponding element information item in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi).

If [{nillable}](013-3-3-element-declarations.md#nillable) is *true*, then an element may
also be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) if it
carries the namespace qualified attribute with [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) `nil` from namespace `http://www.w3.org/2001/XMLSchema-instance` and value `true` (see [xsi:nil (§2.6.2)](007-2-5-names-and-symbol-spaces.md#xsi_nil)) even if it has
no text or element content despite a [{content type}](020-3-4-complex-type-definitions.md#content_type) which would
otherwise require content. Formal details of element [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) are described in [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt).

[{value constraint}](013-3-3-element-declarations.md#e-value_constraint) establishes a default or fixed value for an element. If *default* is specified, and if the element
being [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) is empty, then the
canonical form of the supplied
constraint value becomes the [[schema normalized value]](018-3-3-6-constraints-on-element-declaration-schema-components.md#e-schema_normalized_value) of the [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) element in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi). If *fixed* is specified, then the element's content
must either be empty, in which case *fixed* behaves as *default*,
or its value must match the supplied constraint value.

**Note:**The provision of defaults for elements goes beyond what is possible in
XML 1.0 DTDs, and does not exactly correspond to defaults for attributes. In
particular, an element with a non-empty [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) whose simple
type definition includes the empty string in its lexical space will
nonetheless never receive that value, because the [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) will override it.

[{identity-constraint definitions}](013-3-3-element-declarations.md#identity-constraint_definitions) express constraints establishing uniquenesses and reference relationships among the values of related elements and
attributes. See [Identity-constraint Definitions (§3.11)](040-3-11-identity-constraint-definitions.md#cIdentity-constraint_Definitions).

Element declarations are potential members of the substitution group, if any, identified
by [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar). Potential membership is transitive but not symmetric; an element
declaration is a potential member of any group of which its [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) is a potential member. Actual membership may be blocked by the effects of [{substitution group exclusions}](013-3-3-element-declarations.md#e-final) or [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact), see below.

An empty [{substitution group exclusions}](013-3-3-element-declarations.md#e-final) allows a declaration to be nominated as
the [{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) of other element declarations having the same [{type definition}](013-3-3-element-declarations.md#type_definition) or
types derived therefrom. The explicit
values of [{substitution group exclusions}](013-3-3-element-declarations.md#e-final) rule out element declarations having types which
are *extension*s or *restriction*s respectively of [{type definition}](013-3-3-element-declarations.md#type_definition). If
both values are specified, then the declaration may not be nominated as the
[{substitution group affiliation}](013-3-3-element-declarations.md#class_exemplar) of any other declaration.

The supplied values for [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) determine
whether an element declaration appearing in a [·content model·](005-2-2-1-3-complex-type-definition.md#key-contentModel) will be prevented from additionally
[·validating·](004-2-1-overview-of-xml-schema.md#key-vn) elements (a) with an [xsi:type (§2.6.1)](007-2-5-names-and-symbol-spaces.md#xsi_type) that identifies an
*extension* or *restriction* of the type of the declared element, and/or (b) from [·validating·](004-2-1-overview-of-xml-schema.md#key-vn) elements which are in the
substitution group headed by the declared element.
If [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) is empty, then all derived types and substitution group members are allowed.

Element declarations for which [{abstract}](013-3-3-element-declarations.md#e-abstract) is *true* can appear in
content models only when substitution is allowed;
such declarations may not themselves ever be used to [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) element content.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](013-3-3-element-declarations.md#e-annotation) property.

<a id="declare-element"></a>

#### 3.3.2 XML Representation of Element Declaration Schema Components

The XML representation for an element declaration schema component is an
[<element>](013-3-3-element-declarations.md#element-element) element information item. It specifies a type
definition for an element either by reference or explicitly, and may provide
occurrence and default information. The correspondences between the
properties of the information item and
properties of the component(s) it corresponds to are as follows:

XML Representation Summary: `element` Element Information Item

<a id="element-element"></a>

<element  
  abstract = [boolean](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#boolean) : false  
  block =
(#all | List of (extension | restriction | substitution))
  
  default = [string](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#string)  
  final =
(#all | List of (extension | restriction))
  
