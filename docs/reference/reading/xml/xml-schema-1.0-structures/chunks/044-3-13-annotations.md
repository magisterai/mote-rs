# 3.13 Annotations

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](043-3-12-notation-declarations.md) · [Next](045-3-14-simple-type-definitions.md)

<a id="cAnnotations"></a>

### 3.13 Annotations

3.13.1 [The Annotation Schema Component](044-3-13-annotations.md#Annotation_details)  
        3.13.2 [XML Representation of Annotation Schema Components](044-3-13-annotations.md#declare-annotation)  
        3.13.3 [Constraints on XML Representations of Annotations](044-3-13-annotations.md#d0e14747)  
        3.13.4 [Annotation Validation Rules](044-3-13-annotations.md#d0e14759)  
        3.13.5 [Annotation Information Set Contributions](044-3-13-annotations.md#d0e14764)  
        3.13.6 [Constraints on Annotation Schema Components](044-3-13-annotations.md#coss-annotation)

Annotations provide for human- and machine-targeted annotations of
schema components.

Example



```
<xs:simpleType fn:note="special">
  <xs:annotation>
   <xs:documentation>A type for experts only</xs:documentation>
   <xs:appinfo>
    <fn:specialHandling>checkForPrimes</fn:specialHandling>
   </xs:appinfo>
  </xs:annotation>
     
```




XML representations of three kinds of annotation.

<a id="Annotation_details"></a>

#### 3.13.1 The Annotation Schema Component

The annotation schema component has the following
properties:

Schema Component: [Annotation](005-2-2-1-3-complex-type-definition.md#Annotation)

<a id="application_information"></a>

{application information}
:   A sequence of element information items.

<a id="user_information"></a>

{user information}
:   A sequence of element information items.

<a id="attributes"></a>

{attributes}
:   A sequence of attribute
    information items.

[{user information}](044-3-13-annotations.md#user_information) is intended for human consumption,
[{application information}](044-3-13-annotations.md#application_information) for automatic processing. In both
cases, provision is made for an optional URI reference to supplement the local
information, as the value of the `source` attribute of the
respective element information items. [·Validation·](004-2-1-overview-of-xml-schema.md#key-vn) does *not* involve dereferencing these URIs, when present. In the case of [{user information}](044-3-13-annotations.md#user_information), indication should be given as to the identity of the (human) language used in the contents, using the `xml:lang` attribute.

[{attributes}](044-3-13-annotations.md#attributes) ensures that when schema authors take
advantage of the provision for adding attributes from namespaces other than the
XML Schema namespace to schema documents, they are available within the components
corresponding to the element items where such attributes appear.

Annotations do not participate in [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) as such. Provided
an annotation itself satisfies all relevant [·Schema
Component Constraints·](006-2-3-constraints-and-validation-rules.md#gloss-cos) it *cannot* affect the [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of element information items.

<a id="declare-annotation"></a>

#### 3.13.2 XML Representation of Annotation Schema Components

Annotation of schemas and schema components, with material for human or
computer consumption, is provided for by allowing application information and
human information at the beginning of most major schema elements, and anywhere
at the top level of schemas. The XML representation for an annotation schema component is
an
[<annotation>](044-3-13-annotations.md#element-annotation)
element information item. The correspondences between the
properties of that information item and
properties of the component it corresponds to are as follows:

XML Representation Summary: `annotation` Element Information Item

<a id="element-annotation"></a>

<annotation  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([appinfo](044-3-13-annotations.md#element-appinfo) | [documentation](044-3-13-annotations.md#element-documentation))\*  
</annotation>

<a id="element-appinfo"></a>

<appinfo  
  source = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* (*{any}*)\*  
</appinfo>

<a id="element-documentation"></a>

<documentation  
  source = [anyURI](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#anyURI)  
  xml:lang = [language](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#language)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* (*{any}*)\*  
</documentation>

<table class="reprcomp"><thead><tr><th><a href="044-3-13-annotations.md#Annotation_details">Annotation</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="044-3-13-annotations.md#application_information">{application information}</a></td><td>A sequence of the <a class="eltref" href="044-3-13-annotations.md#element-appinfo">&lt;appinfo&gt;</a> element
information items from among the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, in order, if any, otherwise the
empty sequence.</td></tr><tr valign="top"><td><a class="propref" href="044-3-13-annotations.md#user_information">{user information}</a></td><td>A sequence of the <a class="eltref" href="044-3-13-annotations.md#element-documentation">&lt;documentation&gt;</a> element
information items from among the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, in order, if any, otherwise the
empty sequence.</td></tr><tr valign="top"><td><a class="propref" href="044-3-13-annotations.md#attributes">{attributes}</a></td><td>A sequence of attribute information items, namely
those allowed by the attribute wildcard in the type definition for the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> item itself or for the enclosing items which correspond to the component within which the annotation component is located.</td></tr></tbody></table></td></tr></tbody></table>

The annotation component corresponding to the [<annotation>](044-3-13-annotations.md#element-annotation)
element in the example above will have one element item in each of its [{user information}](044-3-13-annotations.md#user_information) and [{application information}](044-3-13-annotations.md#application_information) and one attribute item in its [{attributes}](044-3-13-annotations.md#attributes).

<a id="d0e14747"></a>

#### 3.13.3 Constraints on XML Representations of Annotations

<a id="src-annotation"></a>

**Schema Representation Constraint: Annotation Definition Representation OK**  

In addition to the conditions imposed on [<annotation>](044-3-13-annotations.md#element-annotation) element
information items by the schema for schemas, the corresponding annotation must satisfy the conditions set
out in [Constraints on Annotation Schema Components (§3.13.6)](044-3-13-annotations.md#coss-annotation).

<a id="d0e14759"></a>

#### 3.13.4 Annotation Validation Rules

None as such.

<a id="d0e14764"></a>

#### 3.13.5 Annotation Information Set Contributions

None as such: the addition of annotations to the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) is
covered by the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi) contributions of the enclosing components.

<a id="coss-annotation"></a>

#### 3.13.6 Constraints on Annotation Schema Components

All annotations (see [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations)) must satisfy the following constraint.

<a id="an-props-correct"></a>

**Schema Component Constraint: Annotation Correct**  

The values of the properties of an annotation must be as described in
the property tableau in
[The Annotation Schema Component (§3.13.1)](044-3-13-annotations.md#Annotation_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

