# 3.4 Complex Type Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](019-continuation-19.md) · [Next](021-3-4-2-xml-representation-of-complex-type-definitions.md)

<a id="Complex_Type_Definitions"></a>

### 3.4 Complex Type Definitions

3.4.1 [The Complex Type Definition Schema Component](020-3-4-complex-type-definitions.md#Complex_Type_Definition_details)  
        3.4.2 [XML Representation of Complex Type Definitions](021-3-4-2-xml-representation-of-complex-type-definitions.md#declare-type)  
        3.4.3 [Constraints on XML Representations of Complex Type Definitions](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#d0e7923)  
        3.4.4 [Complex Type Definition Validation Rules](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#formal-complex-type)  
        3.4.5 [Complex Type Definition Information Set Contributions](027-3-4-5-complex-type-definition-information-set-contributions.md#d0e8328)  
        3.4.6 [Constraints on Complex Type Definition Schema Components](027-3-4-5-complex-type-definition-information-set-contributions.md#coss-ct)  
        3.4.7 [Built-in Complex Type Definition](029-3-4-7-built-in-complex-type-definition.md#d0e9252)

Complex Type Definitions provide for:

- Constraining element information items by providing [Attribute Declaration (§2.2.2.3)](005-2-2-1-3-complex-type-definition.md#Attribute_Declaration)s governing the appearance and content of
  [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element)
- Constraining element information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) to be empty,
  or to conform to a specified element-only or mixed content model, or else
  constraining the character information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) to conform to a
  specified simple type definition.
- Using the mechanisms of [Type Definition Hierarchy (§2.2.1.1)](004-2-1-overview-of-xml-schema.md#Type_Derivation) to derive a complex type from another simple or complex type.
- Specifying [·post-schema-validation infoset contributions·](006-2-3-constraints-and-validation-rules.md#gloss-sic) for elements.
- Limiting the ability to derive additional types from a given complex type.
- Controlling the permission to substitute, in an instance, elements of a derived
  type for elements declared in a content model to be of a given complex type.

Example



```
<xs:complexType name="PurchaseOrderType">
  <xs:sequence>
   <xs:element name="shipTo" type="USAddress"/>
   <xs:element name="billTo" type="USAddress"/>
   <xs:element ref="comment" minOccurs="0"/>
   <xs:element name="items"  type="Items"/>
  </xs:sequence>
  <xs:attribute name="orderDate" type="xs:date"/>
 </xs:complexType>
```




The XML representation of a complex type definition.

<a id="Complex_Type_Definition_details"></a>

#### 3.4.1 The Complex Type Definition Schema Component

A complex type definition schema component has the following
properties:

Schema Component: [Complex Type Definition](005-2-2-1-3-complex-type-definition.md#Complex_Type_Definition)

<a id="ct-name"></a>

{name}
:   Optional. An NCName as defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="ct-target_namespace"></a>

{target namespace}
:   Either [·absent·](008-3-schema-component-details.md#key-null) or a namespace name, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="ct-base_type_definition"></a>

{base type definition}
:   Either a simple type definition or a complex type definition.

<a id="derivation_method"></a>

{derivation method}
:   Either *extension* or *restriction*.

<a id="ct-final"></a>

{final}
:   A subset of {*extension*, *restriction*}.

<a id="ct-abstract"></a>

{abstract}
:   A boolean

<a id="ct-attribute_declarations"></a>

{attribute uses}
:   A set of attribute uses.

<a id="ct-attribute_wildcard"></a>

{attribute wildcard}
:   Optional. A wildcard.

<a id="content_type"></a>

{content type}
:   One of *empty*, a simple type definition or a pair
    consisting of a [·content model·](005-2-2-1-3-complex-type-definition.md#key-contentModel) (I.e. a [Particle (§2.2.3.2)](005-2-2-1-3-complex-type-definition.md#Particle)) and one of *mixed*, *element-only*.

<a id="ct-exact"></a>

{prohibited substitutions}
:   A subset of {*extension*, *restriction*}.

<a id="ct-annotations"></a>

{annotations}
:   A set of annotations.

Complex types definitions are identified by their [{name}](020-3-4-complex-type-definitions.md#ct-name) and [{target namespace}](020-3-4-complex-type-definitions.md#ct-target_namespace). Except
for anonymous complex type definitions (those with no [{name}](020-3-4-complex-type-definitions.md#ct-name)), since
type definitions (i.e. both simple and complex type definitions taken together) must be uniquely identified within an [·XML
Schema·](004-2-1-overview-of-xml-schema.md#key-schema), no complex type definition can have the same name as another
simple or complex type definition. Complex type [{name}](020-3-4-complex-type-definitions.md#ct-name)s and [{target namespace}](020-3-4-complex-type-definitions.md#ct-target_namespace)s
are provided for reference from
instances (see [xsi:type (§2.6.1)](007-2-5-names-and-symbol-spaces.md#xsi_type)), and for use in the XML
representation of schema components
(specifically in [<element>](013-3-3-element-declarations.md#element-element)). See [References to schema components across namespaces (§4.2.3)](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport) for the use of component
identifiers when importing one schema into another.

**Note:**The [{name}](020-3-4-complex-type-definitions.md#ct-name) of a complex type is not *ipso
facto* the [[(local) name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of the
element information items [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by that definition. The connection between a
name and a type definition is described in [Element Declarations (§3.3)](013-3-3-element-declarations.md#cElement_Declarations).

As described in [Type Definition Hierarchy (§2.2.1.1)](004-2-1-overview-of-xml-schema.md#Type_Derivation), each complex type is derived from a
[{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) which is itself either a [Simple Type Definition (§2.2.1.2)](004-2-1-overview-of-xml-schema.md#Simple_Type_Definition) or a [Complex Type Definition (§2.2.1.3)](005-2-2-1-3-complex-type-definition.md#Complex_Type_Definition). [{derivation method}](020-3-4-complex-type-definitions.md#derivation_method) specifies the means of derivation as either *extension* or *restriction* (see [Type Definition Hierarchy (§2.2.1.1)](004-2-1-overview-of-xml-schema.md#Type_Derivation)).

<a id="key-ct-final"></a>

A complex type with an empty specification for [{final}](020-3-4-complex-type-definitions.md#ct-final) can be used as a
[{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition) for other types derived by either of
extension or restriction; the explicit values *extension*, and *restriction* prevent further
derivations by extension and restriction respectively. If all values are specified, then [Definition:]the complex type is said to be
**final**, because no
further derivations are possible. Finality is *not*
inherited, that is, a type definition derived by restriction from a type
definition which is final for extension is not itself, in the absence of any
explicit `final` attribute of its own, final for anything.

Complex types for which [{abstract}](020-3-4-complex-type-definitions.md#ct-abstract) is *true* must
not be used as the
[{type definition}](013-3-3-element-declarations.md#type_definition) for the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of element information items. It follows that they must not be referenced from an
[xsi:type (§2.6.1)](007-2-5-names-and-symbol-spaces.md#xsi_type) attribute in an instance document. Abstract complex types can be
used as [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition)s, or even as the [{type definition}](013-3-3-element-declarations.md#type_definition)s of element declarations, provided in every case a concrete derived type definition is used for [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), either via [xsi:type (§2.6.1)](007-2-5-names-and-symbol-spaces.md#xsi_type) or the operation of a substitution group.

[{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) are a set of attribute uses. See [Element Locally Valid (Complex Type) (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type)
and [Attribute Locally Valid (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-attribute) for details of attribute [·validation·](004-2-1-overview-of-xml-schema.md#key-vn).

[{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard)s provide a more flexible specification for [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of
attributes not explicitly included in [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations).
Informally, the specific values
of [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard) are interpreted as follows:

- *any*: [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) can include attributes with any qualified or unqualified name.
- a set whose
  members are either namespace names or [·absent·](008-3-schema-component-details.md#key-null): [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) can
  include any attribute(s) from the specified namespace(s). If [·absent·](008-3-schema-component-details.md#key-null) is included in the set, then any unqualified attributes are (also) allowed.
- *'not'* and a namespace name: [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) cannot include attributes from the specified namespace.
- *'not'* and [·absent·](008-3-schema-component-details.md#key-null): [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) cannot include
  unqualified attributes.

See [Element Locally Valid (Complex Type) (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type) and [Wildcard allows Namespace Name (§3.10.4)](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard-namespace) for formal
details of attribute wildcard [·validation·](004-2-1-overview-of-xml-schema.md#key-vn).

[{content type}](020-3-4-complex-type-definitions.md#content_type) determines the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of element information items. Informally:

- A [{content type}](020-3-4-complex-type-definitions.md#content_type) with the distinguished value *empty* [·validates·](004-2-1-overview-of-xml-schema.md#key-vn) elements
  with no character or element information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element).
- A [{content type}](020-3-4-complex-type-definitions.md#content_type) which is a [Simple Type Definition (§2.2.1.2)](004-2-1-overview-of-xml-schema.md#Simple_Type_Definition) [·validates·](004-2-1-overview-of-xml-schema.md#key-vn)
  elements with character-only [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element).
- An *element-only* [{content type}](020-3-4-complex-type-definitions.md#content_type) [·validates·](004-2-1-overview-of-xml-schema.md#key-vn) elements with [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) that
  conform to the supplied [·content model·](005-2-2-1-3-complex-type-definition.md#key-contentModel).
- A *mixed* [{content type}](020-3-4-complex-type-definitions.md#content_type) [·validates·](004-2-1-overview-of-xml-schema.md#key-vn) elements whose element [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) (i.e. specifically ignoring other [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) such as character information items)
  conform to the supplied [·content model·](005-2-2-1-3-complex-type-definition.md#key-contentModel).

