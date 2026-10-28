# 3.4.3 Constraints on XML Representations of Complex Type Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](025-continuation-25.md) · [Next](027-3-4-5-complex-type-definition-information-set-contributions.md)

**Note:**Aside from the simple coherence requirements enforced above, constraining
type definitions identified as restrictions to actually *be*
restrictions, that is, to [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) a
subset of the items which are
[·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by their base type definition, is enforced in [Constraints on Complex Type Definition Schema Components (§3.4.6)](027-3-4-5-complex-type-definition-information-set-contributions.md#coss-ct).

**Note:**The *only* substantive function of the value *prohibited* for the
`use` attribute of an [<attribute>](009-3-2-attribute-declarations.md#element-attribute) is in establishing
the correspondence between a complex type defined by restriction and its XML
representation. It serves to prevent
inheritance of an identically named attribute use from the [{base type definition}](020-3-4-complex-type-definitions.md#ct-base_type_definition). Such an [<attribute>](009-3-2-attribute-declarations.md#element-attribute) does not correspond to any component, and hence there is no interaction with either explicit or inherited wildcards in the operation of [Complex Type Definition Validation Rules (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#formal-complex-type) or [Constraints on Complex Type Definition Schema Components (§3.4.6)](027-3-4-5-complex-type-definition-information-set-contributions.md#coss-ct).

Careful consideration of the above concrete syntax reveals that
a type definition need consist of no more than a name, i.e. that
`<complexType name="anyThing"/>` is allowed.

Example



```
<xs:complexType name="length1">
 <xs:simpleContent>
  <xs:extension base="xs:nonNegativeInteger">
   <xs:attribute name="unit" type="xs:NMTOKEN"/>
  </xs:extension>
 </xs:simpleContent>
</xs:complexType>

<xs:element name="width" type="length1"/>

  <width unit="cm">25</width>

<xs:complexType name="length2">
 <xs:complexContent>
  <xs:restriction base="xs:anyType">
   <xs:sequence>
    <xs:element name="size" type="xs:nonNegativeInteger"/>
    <xs:element name="unit" type="xs:NMTOKEN"/>
   </xs:sequence>
  </xs:restriction>
 </xs:complexContent>
</xs:complexType>

<xs:element name="depth" type="length2"/>

  <depth>
   <size>25</size><unit>cm</unit>
  </depth>

<xs:complexType name="length3">
 <xs:sequence>
  <xs:element name="size" type="xs:nonNegativeInteger"/>
  <xs:element name="unit" type="xs:NMTOKEN"/>
 </xs:sequence>
</xs:complexType>

```




Three approaches to defining a type for length: one with
character data content constrained by reference to
a built-in datatype, and one attribute, the other two using two
elements. `length3` is the abbreviated alternative to
`length2`: they correspond to identical type definition components.

Example



```
<xs:complexType name="personName">
 <xs:sequence>
  <xs:element name="title" minOccurs="0"/>
  <xs:element name="forename" minOccurs="0" maxOccurs="unbounded"/>
  <xs:element name="surname"/>
 </xs:sequence>
</xs:complexType>

<xs:complexType name="extendedName">
 <xs:complexContent>
  <xs:extension base="personName">
   <xs:sequence>
    <xs:element name="generation" minOccurs="0"/>
   </xs:sequence>
  </xs:extension>
 </xs:complexContent>
</xs:complexType>

<xs:element name="addressee" type="extendedName"/>

  <addressee>
   <forename>Albert</forename>
   <forename>Arnold</forename>
   <surname>Gore</surname>
   <generation>Jr</generation>
  </addressee>
```




A type definition for personal names, and a definition derived by
extension which adds a single element; an element declaration referencing the
derived definition, and a [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) instance thereof.

Example



```
<xs:complexType name="simpleName">
 <xs:complexContent>
  <xs:restriction base="personName">
   <xs:sequence>
    <xs:element name="forename" minOccurs="1" maxOccurs="1"/>
    <xs:element name="surname"/>
   </xs:sequence>
  </xs:restriction>
 </xs:complexContent>
</xs:complexType>

<xs:element name="who" type="simpleName"/>

   <who>
    <forename>Bill</forename>
    <surname>Clinton</surname>
   </who>
```




A simplified type definition
derived from the base type from the previous example by restriction, eliminating one optional daughter and
fixing another to occur exactly once; an element declared by reference to it,
and a [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) instance thereof.

Example



```
<xs:complexType name="paraType" mixed="true">
 <xs:choice minOccurs="0" maxOccurs="unbounded">
  <xs:element ref="emph"/>
  <xs:element ref="strong"/>
 </xs:choice>
 <xs:attribute name="version" type="xs:number"/>
</xs:complexType>
```




A further illustration of the abbreviated form, with the
`mixed` attribute appearing on `complexType` itself.

<a id="d0e7923"></a>

#### 3.4.3 Constraints on XML Representations of Complex Type Definitions

<a id="src-ct"></a>

**Schema Representation Constraint: Complex Type Definition Representation OK**  

In addition to the conditions imposed on [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) element
information items by the schema for schemas,
**all** of the following must be true:

1 If the [<complexContent>](024-continuation-24.md#element-complexContent) alternative is chosen, the type definition [·resolved·](050-3-15-2-1-references-to-schema-components.md#src-resolve) to
by the [·actual value·](008-3-schema-component-details.md#key-vv) of the `base` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be a complex type definition;

2 If the [<simpleContent>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent) alternative is chosen,
**all** of the following must be true:

2.1 The type definition [·resolved·](050-3-15-2-1-references-to-schema-components.md#src-resolve) to
by the [·actual value·](008-3-schema-component-details.md#key-vv) of the `base` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be
**one** of the following:

2.1.1 a complex type
definition whose [{content type}](020-3-4-complex-type-definitions.md#content_type) is a simple type definition;

2.1.2

<a id="cl-esc"></a>

only if the
[<restriction>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent..restriction) alternative is also chosen, a complex type
definition whose [{content type}](020-3-4-complex-type-definitions.md#content_type) is *mixed* and a particle
which is
[·emptiable·](038-3-10-wildcards.md#cd-emptiable), as defined in [Particle Emptiable (§3.9.6)](038-3-10-wildcards.md#cos-group-emptiable);

2.1.3 only if the
[<extension>](022-continuation-22.md#element-simpleContent..extension) alternative is also chosen, a simple type definition.

2.2 If clause [2.1.2](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cl-esc) above is satisfied, then there must be a
[<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType) among the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of [<restriction>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent..restriction).

**Note:**Although not explicitly ruled out either here or in [Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema), specifying `<xs:complexType . . .mixed='true'` when the [<simpleContent>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent) alternative is chosen has no effect on the corresponding component, and should be avoided. This may be ruled out in a subsequent version of this specification.

3 The corresponding complex type definition component must satisfy the conditions set
out in [Constraints on Complex Type Definition Schema Components (§3.4.6)](027-3-4-5-complex-type-definition-information-set-contributions.md#coss-ct);

4 If clause [2.2.1](023-continuation-23.md#c-awi1) or clause [2.2.2](023-continuation-23.md#c-awi2) in the correspondence specification above for [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard) is satisfied, the intensional intersection must be expressible, as defined in [Attribute Wildcard Intersection (§3.10.6)](039-3-10-4-wildcard-validation-rules.md#cos-aw-intersect).

<a id="formal-complex-type"></a>

#### 3.4.4 Complex Type Definition Validation Rules

<a id="cvc-complex-type"></a>

**Validation Rule: Element Locally Valid (Complex Type)**  

For an element information item to be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a complex type definition
**all** of the following must be true:

1 [{abstract}](020-3-4-complex-type-definitions.md#ct-abstract) is *false*.

2 If clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl) of [Element Locally Valid (Element) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-elt) did not
apply, then the appropriate **case** among the following
must be true:

2.1 **If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) is *empty*, **then** the element
information item has no character or element information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

2.2

<a id="c-sv2"></a>

**If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) is a simple
type definition, **then** the element information item has no element
information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), and the [·normalized value·](008-3-schema-component-details.md#key-nv) of the element information item is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that simple type definition as defined by [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type).

2.3 **If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) is *element-only*, **then** the
element information item has no character information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) other
than those whose [[character code]](https://www.w3.org/TR/xml-infoset/#infoitem.character) is defined as a [white space](https://www.w3.org/TR/REC-xml#NT-S)
in [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml).

2.4 **If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) is *element-only* or
*mixed*, **then** the sequence of the element information item's element
information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), if any, taken in order, is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with
respect to the [{content type}](020-3-4-complex-type-definitions.md#content_type)'s particle, as defined in [Element Sequence Locally Valid (Particle) (§3.9.4)](035-3-9-particles.md#cvc-particle).

3

<a id="c-aam"></a>

For each attribute information item in the element information
item's [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) excepting those whose [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is identical to `http://www.w3.org/2001/XMLSchema-instance` and whose [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is one of `type`, `nil`, `schemaLocation` or `noNamespaceSchemaLocation`,
the appropriate **case** among the following
must be true:

3.1

<a id="c-ctma"></a>

**If** there is among the [{attribute uses}](020-3-4-complex-type-definitions.md#ct-attribute_declarations) an
attribute use with an [{attribute declaration}](030-3-5-attributeuses.md#attribute) whose
[{name}](009-3-2-attribute-declarations.md#a-name) matches the attribute information item's [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) and whose [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) is identical to the attribute information item's [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) (where an [·absent·](008-3-schema-component-details.md#key-null) [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) is taken to be identical to a [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) with no value), **then** the attribute information must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to that attribute use as per [Attribute Locally Valid (Use) (§3.5.4)](030-3-5-attributeuses.md#cvc-au). In this case the [{attribute declaration}](030-3-5-attributeuses.md#attribute) of that attribute use is the [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) for the attribute information item with respect to [Schema-Validity Assessment (Attribute) (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-assess-attr) and [Assessment Outcome (Attribute) (§3.2.5)](011-3-2-4-attribute-declaration-validation-rules.md#sic-a-outcome).

3.2

<a id="c-avaw"></a>

**otherwise** 
**all** of the following must be true:

3.2.1 There must be an [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard).

