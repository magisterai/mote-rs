# 3.3.3 Constraints on XML Representations of Element Declarations

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](015-continuation-15.md) · [Next](017-3-3-5-element-declaration-information-set-contributions.md)

```
 <xs:complexType name="facet">
  <xs:complexContent>
   <xs:extension base="xs:annotated">
    <xs:attribute name="value" use="required"/>
   </xs:extension>
  </xs:complexContent>
 </xs:complexType>

 <xs:element name="facet" type="xs:facet" abstract="true"/>

 <xs:element name="encoding" substitutionGroup="xs:facet">
  <xs:complexType>
   <xs:complexContent>
    <xs:restriction base="xs:facet">
     <xs:sequence>
      <xs:element ref="annotation" minOccurs="0"/>
     </xs:sequence>
     <xs:attribute name="value" type="xs:encodings"/>
    </xs:restriction>
   </xs:complexContent>
  </xs:complexType>
 </xs:element>

 <xs:element name="period" substitutionGroup="xs:facet">
  <xs:complexType>
   <xs:complexContent>
    <xs:restriction base="xs:facet">
     <xs:sequence>
      <xs:element ref="annotation" minOccurs="0"/>
     </xs:sequence>
     <xs:attribute name="value" type="xs:duration"/>
    </xs:restriction>
   </xs:complexContent>
  </xs:complexType>
 </xs:element>

 <xs:complexType name="datatype">
  <xs:sequence>
   <xs:element ref="facet" minOccurs="0" maxOccurs="unbounded"/>
  </xs:sequence>
  <xs:attribute name="name" type="xs:NCName" use="optional"/>
  . . .
 </xs:complexType>
```




An example from a previous version of the schema for datatypes. The
`facet` type is defined
and the `facet` element is declared to use it. The `facet` element is abstract -- it's
*only* defined to stand as the head for a substitution group. Two further
elements are declared, each a member of the `facet` substitution group. Finally a type is defined which refers to `facet`, thereby
allowing *either* `period` or `encoding` (or
any other member of the group).

<a id="d0e4233"></a>

#### 3.3.3 Constraints on XML Representations of Element Declarations

<a id="src-element"></a>

**Schema Representation Constraint: Element Declaration Representation OK**  

In addition to the conditions imposed on [<element>](013-3-3-element-declarations.md#element-element) element
information items by the schema for schemas:
**all** of the following must be true:

1 `default` and `fixed` must not both be present.

2 If the item's parent is not [<schema>](049-3-15-schemas-as-a-whole.md#element-schema), then
**all** of the following must be true:

2.1 One of `ref` or `name` must be present, but not both.

2.2 If `ref` is present, then all of [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType),
[<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType), [<key>](040-3-11-identity-constraint-definitions.md#element-key), [<keyref>](040-3-11-identity-constraint-definitions.md#element-keyref),
[<unique>](040-3-11-identity-constraint-definitions.md#element-unique), `nillable`, `default`,
`fixed`, `form`, `block` and `type` must be absent,
i.e. only `minOccurs`, `maxOccurs`, `id` are
allowed in addition to `ref`, along with [<annotation>](044-3-13-annotations.md#element-annotation).

3 `type` and either [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType) or [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) are mutually exclusive.

4 The corresponding particle and/or element declarations must satisfy the conditions set
out in [Constraints on Element Declaration Schema Components (§3.3.6)](018-3-3-6-constraints-on-element-declaration-schema-components.md#coss-element) and [Constraints on Particle Schema Components (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#coss-particle).

<a id="d0e4333"></a>

#### 3.3.4 Element Declaration Validation Rules

<a id="cvc-elt"></a>

**Validation Rule: Element Locally Valid (Element)**  

For an element information item to be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to an
element declaration
**all** of the following must be true:

1

<a id="c-ea"></a>

The declaration must not be [·absent·](008-3-schema-component-details.md#key-null).

2 Its
[{abstract}](013-3-3-element-declarations.md#e-abstract) must be *false*.

3 The appropriate **case** among the following
must be true:

3.1 **If** [{nillable}](013-3-3-element-declarations.md#nillable) is *false*, **then** there must be no attribute information item among the element
information item's [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) whose [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is identical to `http://www.w3.org/2001/XMLSchema-instance` and whose [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is `nil`.

3.2

<a id="c-nl"></a>

**If** [{nillable}](013-3-3-element-declarations.md#nillable) is *true* and there is such an attribute
information item and its [·actual value·](008-3-schema-component-details.md#key-vv) is `true`
, **then** 
**all** of the following must be true:

3.2.1 The element information item must have no character or element information item
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

3.2.2 There must be no *fixed* [{value constraint}](013-3-3-element-declarations.md#e-value_constraint).

4 If there is an attribute information item among the element
information item's [[attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element) whose [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is identical to `http://www.w3.org/2001/XMLSchema-instance` and whose [[local name]](https://www.w3.org/TR/xml-infoset/#infoitem.attribute) is `type`, then
**all** of the following must be true:

4.1 The [·normalized value·](008-3-schema-component-details.md#key-nv) of that attribute information item must be
[·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the built-in [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName) simple type, as defined by [String Valid (§3.14.4)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cvc-simple-type);

4.2 The [·local name·](050-3-15-2-1-references-to-schema-components.md#q-local) and [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) (as defined in [QName Interpretation (§3.15.3)](050-3-15-2-1-references-to-schema-components.md#src-qname)), of the [·actual value·](008-3-schema-component-details.md#key-vv) of that attribute information item must resolve to a type definition, as defined in [QName resolution (Instance) (§3.15.4)](050-3-15-2-1-references-to-schema-components.md#cvc-resolve-instance) -- 

<a id="key-ltd"></a>

[Definition:]call this type definition the **local type definition**;

4.3 The [·local type definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-ltd) must be
validly derived from the [{type definition}](013-3-3-element-declarations.md#type_definition) given the
union of the [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) and the [{type definition}](013-3-3-element-declarations.md#type_definition)'s [{prohibited substitutions}](020-3-4-complex-type-definitions.md#ct-exact), as defined in [Type Derivation OK (Complex) (§3.4.6)](028-continuation-28.md#cos-ct-derived-ok) (if it is a complex type definition), or given [{disallowed substitutions}](013-3-3-element-declarations.md#e-exact) as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok) (if it is a simple type definition).

<a id="key-atd"></a>

[Definition:]The phrase
**actual type definition** occurs below. If the above three clauses are satisfied, this
should be understood as referring to the [·local type
definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-ltd), otherwise to the [{type definition}](013-3-3-element-declarations.md#type_definition).

5 The appropriate **case** among the following
must be true:

5.1 **If** the declaration has a [{value constraint}](013-3-3-element-declarations.md#e-value_constraint), the item has neither element nor character [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) and clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl) has not applied, **then** 
**all** of the following must be true:

5.1.1 If the [·actual type definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-atd) is a [·local type
definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-ltd) then the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) value must
be a valid default for the [·actual
type definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-atd) as defined in [Element Default Valid (Immediate) (§3.3.6)](019-continuation-19.md#cos-valid-default).

5.1.2 The element information item with the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) value used as its [·normalized value·](008-3-schema-component-details.md#key-nv)
must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the [·actual type definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-atd) as defined by [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type).

5.2 **If** the declaration has no [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) or the item has either element or character [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) or clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl) has applied, **then** **all** of the following must be true:

5.2.1 The element information item
must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the [·actual type definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-atd) as defined by [Element Locally Valid (Type) (§3.3.4)](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#cvc-type).

5.2.2 If there is a *fixed* [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) and
clause [3.2](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#c-nl) has not applied,
**all** of the following must be true:

5.2.2.1 The element information item must have no element information
item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element).

5.2.2.2 The appropriate **case** among the following
must be true:

5.2.2.2.1 **If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) of the [·actual type definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-atd) is *mixed*, **then** the [·initial value·](008-3-schema-component-details.md#key-iv)
of the item must match the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) value.

5.2.2.2.2 **If** the [{content type}](020-3-4-complex-type-definitions.md#content_type) of the [·actual type definition·](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md#key-atd) is a simple type definition, **then** the [·actual value·](008-3-schema-component-details.md#key-vv) of the item must match the [canonical lexical representation](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-canonical-representation) of the [{value constraint}](013-3-3-element-declarations.md#e-value_constraint) value.

6 The element information item must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to
each of the [{identity-constraint definitions}](013-3-3-element-declarations.md#identity-constraint_definitions) as per [Identity-constraint Satisfied (§3.11.4)](041-3-11-3-constraints-on-xml-representations-of-identity-constraint-definitions.md#cvc-identity-constraint).

7 If the element information item is the [·validation root·](057-5-schemas-and-schema-validity-assessment.md#key-vr), it must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) per [Validation Root Valid (ID/IDREF) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-id).

<a id="cvc-type"></a>

**Validation Rule: Element Locally Valid (Type)**  

For an element information item to be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a type definition
**all** of the following must be true:

1

<a id="c-ct"></a>

The type definition must not be [·absent·](008-3-schema-component-details.md#key-null);

2
It must not have
[{abstract}](020-3-4-complex-type-definitions.md#ct-abstract) with value *true*.

3 The appropriate **case** among the following
must be true:

3.1 **If** the type definition is a simple type
definition, **then** 
**all** of the following must be true:

