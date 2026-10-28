# 3.6.3 Constraints on XML Representations of Attribute Group Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](030-3-5-attributeuses.md) · [Next](032-3-7-model-group-definitions.md)

<table class="reprcomp"><thead><tr><th><a href="030-3-5-attributeuses.md#Attribute_Group_Definition_details">Attribute Group Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#ag-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#ag-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <code>schema</code>
element information item.</td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#ag-attribute_declarations">{attribute uses}</a></td><td>The union of the set of attribute uses corresponding to the <a class="eltref" href="009-3-2-attribute-declarations.md#element-attribute">&lt;attribute&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, with the <a class="propref" href="030-3-5-attributeuses.md#ag-attribute_declarations">{attribute uses}</a> of the
attribute groups <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a>s of the <code>ref</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the <a class="eltref" href="022-continuation-22.md#element-simpleContent..attributeGroup">&lt;attributeGroup&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any.</td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#ag-attribute_wildcard">{attribute wildcard}</a></td><td>As for the <a class="termref" href="023-continuation-23.md#key-eaw"><span class="arrow">·</span>complete wildcard<span class="arrow">·</span></a> as described in <a href="021-3-4-2-xml-representation-of-complex-type-definitions.md#declare-type">XML Representation of Complex Type Definitions (§3.4.2)</a>.</td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#ag-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

The example above illustrates a pattern which
recurs in the XML representation of schemas: The same element, in this case `attributeGroup`, serves both to
define and to incorporate by reference. In the first case the
`name` attribute is required, in the second the `ref`
attribute is required, and the element must be empty. These two are mutually exclusive, and also conditioned
by context: the defining form, with a `name`, must occur at the top
level of a schema, whereas the referring form, with a `ref`, must
occur within a complex type definition or an attribute group definition.

<a id="d0e9758"></a>

#### 3.6.3 Constraints on XML Representations of Attribute Group Definitions

<a id="src-attribute_group"></a>

**Schema Representation Constraint: Attribute Group Definition Representation OK**  

In addition to the conditions imposed on [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup) element
information items by the schema for schemas,
**all** of the following must be true:

1 The corresponding attribute group definition, if any, must satisfy the conditions set
out in [Constraints on Attribute Group Definition Schema Components (§3.6.6)](031-3-6-3-constraints-on-xml-representations-of-attribute-group-definitions.md#coss-attrGroup).

2 If clause [2.2.1](023-continuation-23.md#c-awi1) or clause [2.2.2](023-continuation-23.md#c-awi2) in the correspondence specification in [XML Representation of Complex Type Definitions (§3.4.2)](021-3-4-2-xml-representation-of-complex-type-definitions.md#declare-type) for [{attribute wildcard}](020-3-4-complex-type-definitions.md#ct-attribute_wildcard), as referenced above, is satisfied, the intensional intersection must be expressible, as defined in [Attribute Wildcard Intersection (§3.10.6)](039-3-10-4-wildcard-validation-rules.md#cos-aw-intersect).

3 Circular group reference is disallowed outside [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine). That is, unless this element information item's parent is [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine), then among the
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), if any, there must not be an [<attributeGroup>](022-continuation-22.md#element-simpleContent..attributeGroup) with `ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) which resolves to the component corresponding to this [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup). Indirect circularity is also ruled out. That is, when [QName resolution (Schema Document) (§3.15.3)](050-3-15-2-1-references-to-schema-components.md#src-resolve) is applied to a [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) arising from any [<attributeGroup>](022-continuation-22.md#element-simpleContent..attributeGroup)s with a
`ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) among the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), it must not be the case that a [·QName·](050-3-15-2-1-references-to-schema-components.md#gloss-QName) is
encountered at any depth which resolves to the
component corresponding to this [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup).

<a id="d0e9831"></a>

#### 3.6.4 Attribute Group Definition Validation Rules

None as such.

<a id="d0e9836"></a>

#### 3.6.5 Attribute Group Definition Information Set Contributions

None as such.

<a id="coss-attrGroup"></a>

#### 3.6.6 Constraints on Attribute Group Definition Schema Components

All attribute group definitions (see [Attribute Group Definitions (§3.6)](030-3-5-attributeuses.md#cAttribute_Group_Definitions)) must satisfy the following constraint.

<a id="ag-props-correct"></a>

**Schema Component Constraint: Attribute Group Definition Properties Correct**  

**All** of the following must be true:

1 The values of the properties of an attribute group definition must be as described in
the property tableau in
[The Attribute Group Definition Schema Component (§3.6.1)](030-3-5-attributeuses.md#Attribute_Group_Definition_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing);

2 Two distinct members of the [{attribute uses}](030-3-5-attributeuses.md#ag-attribute_declarations)
must not have [{attribute declaration}](030-3-5-attributeuses.md#attribute)s both of whose [{name}](009-3-2-attribute-declarations.md#a-name)s
match and whose [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace)s are identical.

3 Two distinct members of the [{attribute uses}](030-3-5-attributeuses.md#ag-attribute_declarations)
must not have [{attribute declaration}](030-3-5-attributeuses.md#attribute)s both of whose [{type definition}](009-3-2-attribute-declarations.md#a-simple_type_definition)s are or are derived from [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID).

