# 3.2 Attribute Declarations

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-3-schema-component-details.md) · [Next](010-3-2-3-constraints-on-xml-representations-of-attribute-declarations.md)

<a id="cAttribute_Declarations"></a>

### 3.2 Attribute Declarations

3.2.1 [The Attribute Declaration Schema Component](009-3-2-attribute-declarations.md#Attribute_Declaration_details)  
        3.2.2 [XML Representation of Attribute Declaration Schema Components](009-3-2-attribute-declarations.md#declare-attribute)  
        3.2.3 [Constraints on XML Representations of Attribute Declarations](010-3-2-3-constraints-on-xml-representations-of-attribute-declarations.md#d0e2403)  
        3.2.4 [Attribute Declaration Validation Rules](011-3-2-4-attribute-declaration-validation-rules.md#d0e2483)  
        3.2.5 [Attribute Declaration Information Set Contributions](011-3-2-4-attribute-declaration-validation-rules.md#d0e2622)  
        3.2.6 [Constraints on Attribute Declaration Schema Components](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#coss-attribute)  
        3.2.7 [Built-in Attribute Declarations](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#d0e3067)

Attribute declarations provide for:

- Local [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of attribute information item values using a simple type definition;
- Specifying default or fixed values for attribute information items.

Example



```
<xs:attribute name="age" type="xs:positiveInteger" use="required"/>
```




The XML representation of an attribute declaration.

<a id="Attribute_Declaration_details"></a>

#### 3.2.1 The Attribute Declaration Schema Component

The attribute declaration schema component has the following
properties:

Schema Component: [Attribute Declaration](005-2-2-1-3-complex-type-definition.md#Attribute_Declaration)

<a id="a-name"></a>

{name}
:   An NCName as defined by
    [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="a-target_namespace"></a>

{target namespace}
:   Either [·absent·](008-3-schema-component-details.md#key-null) or
    a namespace name, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="a-simple_type_definition"></a>

{type definition}
:   A
    simple type definition.

<a id="a-scope"></a>

{scope}
:   Optional. Either *global* or a complex type
    definition.

<a id="a-value_constraint"></a>

{value constraint}
:   Optional. A pair
    consisting of a value and one of *default*, *fixed*.

<a id="a-annotation"></a>

{annotation}
:   Optional. An annotation.

The [{name}](009-3-2-attribute-declarations.md#a-name) property must match the local part of the names of attributes being [·validated·](004-2-1-overview-of-xml-schema.md#key-vn).

The value of the attribute must conform to the supplied [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition).

A non-[·absent·](008-3-schema-component-details.md#key-null) value of the [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) property provides for [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) of
namespace-qualified attribute information items (which must be explicitly
prefixed in the character-level form of XML documents). [·Absent·](008-3-schema-component-details.md#key-null) values of
[{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) unqualified (unprefixed) items.

A [{scope}](009-3-2-attribute-declarations.md#a-scope) of *global* identifies attribute declarations
available for use in complex type definitions throughout the schema. Locally scoped declarations are available for use only within the
complex type definition identified by the [{scope}](009-3-2-attribute-declarations.md#a-scope) property. This property is [·absent·](008-3-schema-component-details.md#key-null) in the case of declarations within attribute group definitions: their scope will be determined when they are used in the construction of complex type definitions.

[{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint) reproduces the functions of XML 1.0 default and `#FIXED`
attribute values. *default* specifies that the attribute is to appear unconditionally in
the [·post-schema-validation infoset·](004-2-1-overview-of-xml-schema.md#key-psvi), with the supplied value used
whenever the attribute is not actually present; *fixed* indicates that the attribute value if present must equal the supplied
constraint value, and if absent receives the supplied value as for
*default*. Note that it is *values* that are supplied and/or
checked, not strings.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](009-3-2-attribute-declarations.md#a-annotation) property.

**Note:**A more complete and formal presentation of the semantics of [{name}](009-3-2-attribute-declarations.md#a-name), [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) and [{value constraint}](009-3-2-attribute-declarations.md#a-value_constraint) is provided in
conjunction with other aspects of complex type [·validation·](004-2-1-overview-of-xml-schema.md#key-vn) (see [Element Locally Valid (Complex Type) (§3.4.4)](026-3-4-3-constraints-on-xml-representations-of-complex-type-definitions.md#cvc-complex-type).)

[[XML-Infoset]](060-b-references-normative.md#ref-xmlinfo) distinguishes attributes with names such as `xmlns` or `xmlns:xsl` from
ordinary attributes, identifying them as [[namespace attributes]](https://www.w3.org/TR/xml-infoset/#infoitem.element). Accordingly, it is unnecessary and in fact not possible for
schemas to contain attribute declarations corresponding to such
namespace declarations, see [xmlns Not Allowed (§3.2.6)](012-3-2-6-constraints-on-attribute-declaration-schema-components.md#no-xmlns). No means is provided in
this specification to supply a
default value for a namespace declaration.

<a id="declare-attribute"></a>

#### 3.2.2 XML Representation of Attribute Declaration Schema Components

The XML representation for an attribute declaration schema component is an
[<attribute>](009-3-2-attribute-declarations.md#element-attribute) element information item. It specifies a simple type
definition for an attribute either by reference or explicitly, and may provide default information. The correspondences between the
properties of the information item and
properties of the component are as follows:

XML Representation Summary: `attribute` Element Information Item

<a id="element-attribute"></a>

<attribute  
  default = [string](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#string)  
  fixed = [string](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#string)  
  form = (qualified | unqualified)  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  name = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  ref = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  type = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  use = (optional | prohibited | required) : optional  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, [simpleType](045-3-14-simple-type-definitions.md#element-simpleType)?)  
</attribute>

If the [<attribute>](009-3-2-attribute-declarations.md#element-attribute) element information item has [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) as its parent, the corresponding schema component is as follows:

<table class="reprcomp"><thead><tr><th><a href="009-3-2-attribute-declarations.md#Attribute_Declaration_details">Attribute Declaration</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a>
element information item, or <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a> if there is none.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-simple_type_definition">{type definition}</a></td><td>The simple type definition
corresponding to the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-simpleType">&lt;simpleType&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise the simple type definition <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by
the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>type</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise the
<a class="termref" href="048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself"><span class="arrow">·</span>simple ur-type definition<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-scope">{scope}</a></td><td><i>global</i>.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-value_constraint">{value constraint}</a></td><td>If there is a <code>default</code> or a <code>fixed</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, then a pair consisting of the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> (with respect to the
<a class="propref" href="009-3-2-attribute-declarations.md#a-simple_type_definition">{type definition}</a>) of that <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> and
either <i>default</i> or <i>fixed</i>, as appropriate, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

otherwise if the [<attribute>](009-3-2-attribute-declarations.md#element-attribute) element information item has
[<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) or [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup) as an ancestor
and the `ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is absent, it corresponds to an
attribute use with properties as follows (unless `use='prohibited'`, in which case the item
corresponds to nothing at all):

