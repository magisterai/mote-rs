# 3.12 Notation Declarations

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](042-3-11-5-identity-constraint-definition-information-set-contributions.md) · [Next](044-3-13-annotations.md)

<a id="cNotation_Declarations"></a>

### 3.12 Notation Declarations

3.12.1 [The Notation Declaration Schema Component](043-3-12-notation-declarations.md#Notation_Declaration_details)  
        3.12.2 [XML Representation of Notation Declaration Schema Components](043-3-12-notation-declarations.md#declare-notation)  
        3.12.3 [Constraints on XML Representations of Notation Declarations](043-3-12-notation-declarations.md#d0e14542)  
        3.12.4 [Notation Declaration Validation Rules](043-3-12-notation-declarations.md#d0e14554)  
        3.12.5 [Notation Declaration Information Set Contributions](043-3-12-notation-declarations.md#d0e14559)  
        3.12.6 [Constraints on Notation Declaration Schema Components](043-3-12-notation-declarations.md#coss-notation)

Notation declarations reconstruct XML 1.0 NOTATION declarations.

Example



```
<xs:notation name="jpeg" public="image/jpeg" system="viewer.exe">
```




The XML representation of a notation declaration.

<a id="Notation_Declaration_details"></a>

#### 3.12.1 The Notation Declaration Schema Component

The notation declaration schema component has the following
properties:

Schema Component: [Notation Declaration](005-2-2-1-3-complex-type-definition.md#Notation_Declaration)

<a id="n-name"></a>

{name}
:   An NCName as defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="n-target_namespace"></a>

{target namespace}
:   Either [·absent·](008-3-schema-component-details.md#key-null) or a namespace name, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="system_identifier"></a>

{system identifier}
:   Optional if [{public identifier}](043-3-12-notation-declarations.md#public_identifier) is present. A URI reference.

<a id="public_identifier"></a>

{public identifier}
:   Optional if [{system identifier}](043-3-12-notation-declarations.md#system_identifier) is present. A public identifier, as defined in [[XML 1.0 (Second Edition)]](060-b-references-normative.md#ref-xml).

<a id="n-annotation"></a>

{annotation}
:   Optional. An annotation.

Notation declarations do not participate in [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) as such.
They are referenced in the course of [·validating·](004-2-1-overview-of-xml-schema.md#key-vn) strings as members of
the [NOTATION](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NOTATION) simple type.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](043-3-12-notation-declarations.md#n-annotation) property.

<a id="declare-notation"></a>

#### 3.12.2 XML Representation of Notation Declaration Schema Components

The XML representation for a notation declaration schema component is
a
[<notation>](043-3-12-notation-declarations.md#element-notation)
element information item. The correspondences between the
properties of that information item and
properties of the component it corresponds to are as follows:

XML Representation Summary: `notation` Element Information Item

<a id="element-notation"></a>

<notation  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  **name** = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  public = [token](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#token)  
  system = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?)  
</notation>

<table class="reprcomp"><thead><tr><th><a href="043-3-12-notation-declarations.md#Notation_Declaration_details">Notation Declaration</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="043-3-12-notation-declarations.md#n-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="043-3-12-notation-declarations.md#n-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <code>schema</code>
element information item.</td></tr><tr valign="top"><td><a class="propref" href="043-3-12-notation-declarations.md#system_identifier">{system identifier}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>system</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="043-3-12-notation-declarations.md#public_identifier">{public identifier}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>public</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="043-3-12-notation-declarations.md#n-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

Example



```
<xs:notation name="jpeg"
             public="image/jpeg" system="viewer.exe" />

<xs:element name="picture">
 <xs:complexType>
  <xs:simpleContent>
   <xs:extension base="xs:hexBinary">
    <xs:attribute name="pictype">
     <xs:simpleType>
      <xs:restriction base="xs:NOTATION">
       <xs:enumeration value="jpeg"/>
       <xs:enumeration value="png"/>
       . . .
      </xs:restriction>
     </xs:simpleType>
    </xs:attribute>
   </xs:extension>
  </xs:simpleContent>
 </xs:complexType>
</xs:element>

<picture pictype="jpeg">...</picture>
```




<a id="d0e14542"></a>

#### 3.12.3 Constraints on XML Representations of Notation Declarations

<a id="src-notation"></a>

**Schema Representation Constraint: Notation Definition Representation OK**  

In addition to the conditions imposed on [<notation>](043-3-12-notation-declarations.md#element-notation) element
information items by the schema for schemas, the corresponding notation definition must satisfy the conditions set
out in [Constraints on Notation Declaration Schema Components (§3.12.6)](043-3-12-notation-declarations.md#coss-notation).

<a id="d0e14554"></a>

#### 3.12.4 Notation Declaration Validation Rules

None as such.

<a id="d0e14559"></a>

#### 3.12.5 Notation Declaration Information Set Contributions

<a id="sic-notation-used"></a>

**Schema Information Set Contribution: Validated with Notation**  

Whenever an attribute information item is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a [NOTATION](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NOTATION), in the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) its parent element information item either has a property as follows:

PSVI Contributions
for element information items

<a id="e-notation"></a>

[notation]
:   An
    [·item isomorphic·](051-3-15-5-schema-information-set-contributions.md#key-iso) to the notation declaration whose [{name}](043-3-12-notation-declarations.md#n-name) and [{target namespace}](043-3-12-notation-declarations.md#n-target_namespace) match the [·local name·](050-3-15-2-1-references-to-schema-components.md#q-local) and [·namespace name·](050-3-15-2-1-references-to-schema-components.md#q-uri) (as defined in [QName Interpretation (§3.15.3)](050-3-15-2-1-references-to-schema-components.md#src-qname)) of the attribute item's [·actual value·](008-3-schema-component-details.md#key-vv)

or has a pair of properties as follows:

PSVI Contributions
for element information items

<a id="e-notation_system"></a>

[notation system]
:   The value of the [{system identifier}](043-3-12-notation-declarations.md#system_identifier) of that notation declaration.

<a id="e-notation_public"></a>

[notation public]
:   The value of the [{public identifier}](043-3-12-notation-declarations.md#public_identifier) of that notation declaration.

**Note:**For compatibility, only one such attribute should appear on any given
element. If more than one such attribute *does* appear, which one
supplies the infoset property or properties above is not defined.

<a id="coss-notation"></a>

#### 3.12.6 Constraints on Notation Declaration Schema Components

All notation declarations (see [Notation Declarations (§3.12)](043-3-12-notation-declarations.md#cNotation_Declarations)) must satisfy the following constraint.

<a id="n-props-correct"></a>

**Schema Component Constraint: Notation Declaration Correct**  

The values of the properties of a notation declaration must be as described in
the property tableau in
[The Notation Declaration Schema Component (§3.12.1)](043-3-12-notation-declarations.md#Notation_Declaration_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

