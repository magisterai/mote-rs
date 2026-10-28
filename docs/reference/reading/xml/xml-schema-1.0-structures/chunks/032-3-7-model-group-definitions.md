# 3.7 Model Group Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](031-3-6-3-constraints-on-xml-representations-of-attribute-group-definitions.md) · [Next](033-3-7-6-constraints-on-model-group-definition-schema-components.md)

<a id="cModel_Group_Definitions"></a>

### 3.7 Model Group Definitions

3.7.1 [The Model Group Definition Schema Component](032-3-7-model-group-definitions.md#Model_Group_Definition_details)  
        3.7.2 [XML Representation of Model Group Definition Schema Components](032-3-7-model-group-definitions.md#declare-namedModelGroup)  
        3.7.3 [Constraints on XML Representations of Model Group Definitions](032-3-7-model-group-definitions.md#d0e10150)  
        3.7.4 [Model Group Definition Validation Rules](032-3-7-model-group-definitions.md#d0e10162)  
        3.7.5 [Model Group Definition Information Set Contributions](032-3-7-model-group-definitions.md#d0e10167)  
        3.7.6 [Constraints on Model Group Definition Schema Components](033-3-7-6-constraints-on-model-group-definition-schema-components.md#coss-groupDef)

A model group definition associates a name and optional annotations with a [Model Group (§2.2.3.1)](005-2-2-1-3-complex-type-definition.md#Model_Group).
By reference to the name, the entire model group can be incorporated by reference into a [{term}](035-3-9-particles.md#term).

Model group definitions are provided
primarily for reference from the [XML Representation of Complex Type Definitions (§3.4.2)](021-3-4-2-xml-representation-of-complex-type-definitions.md#declare-type) (see [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType)
and [<group>](032-3-7-model-group-definitions.md#element-group)). Thus, model group definitions provide a
replacement for some uses of XML's
[parameter entity](https://www.w3.org/TR/REC-xml#dt-PE) facility.

Example



```
<xs:group name="myModelGroup">
 <xs:sequence>
  <xs:element ref="someThing"/>
  . . .
 </xs:sequence>
</xs:group>

<xs:complexType name="trivial">
 <xs:group ref="myModelGroup"/>
 <xs:attribute .../>
</xs:complexType>

<xs:complexType name="moreSo">
 <xs:choice>
  <xs:element ref="anotherThing"/>
  <xs:group ref="myModelGroup"/>
 </xs:choice>
 <xs:attribute .../>
</xs:complexType>
```




A minimal model group is defined and used by reference, first as the whole
content model, then as one alternative in a choice.

<a id="Model_Group_Definition_details"></a>

#### 3.7.1 The Model Group Definition Schema Component

The model group definition schema component has the following
properties:

Schema Component: [Model Group Definition](005-2-2-1-3-complex-type-definition.md#Model_Group_Definition)

<a id="mg-name"></a>

{name}
:   An NCName as defined by [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="mg-target_namespace"></a>

{target namespace}
:   Either [·absent·](008-3-schema-component-details.md#key-null) or a namespace name, as defined in [[XML-Namespaces]](060-b-references-normative.md#ref-xml-namespaces).

<a id="model_group"></a>

{model group}
:   A model group.

<a id="mg-annotation"></a>

{annotation}
:   Optional. An annotation.

Model group definitions are identified by their [{name}](032-3-7-model-group-definitions.md#mg-name) and [{target namespace}](032-3-7-model-group-definitions.md#mg-target_namespace); model group identities must be unique within an [·XML Schema·](004-2-1-overview-of-xml-schema.md#key-schema). See [References to schema components across namespaces (§4.2.3)](054-4-2-3-references-to-schema-components-across-namespaces.md#composition-schemaImport) for the use of component
identifiers when importing one schema into another.

Model group definitions *per se* do not participate in [·validation·](004-2-1-overview-of-xml-schema.md#key-vn), but the [{term}](035-3-9-particles.md#term) of
a particle may correspond in whole or in part
to a model group from a model group definition.

[{model group}](032-3-7-model-group-definitions.md#model_group) is the [Model Group (§2.2.3.1)](005-2-2-1-3-complex-type-definition.md#Model_Group) for which the model group definition provides a name.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](032-3-7-model-group-definitions.md#mg-annotation) property.

<a id="declare-namedModelGroup"></a>

#### 3.7.2 XML Representation of Model Group Definition Schema Components

The XML representation for a model group definition schema component is a
[<group>](032-3-7-model-group-definitions.md#element-group) element information item. It provides for
naming a model group for use by reference in the XML representation of
complex type definitions and model groups. The correspondences between the
properties of the information item and
properties of the component it corresponds to are as follows:

XML Representation Summary: `group` Element Information Item

<a id="element-group"></a>

<group  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  maxOccurs =
([nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) | unbounded)
 : 1  
  minOccurs = [nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) : 1  
  name = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  ref = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([all](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all) | [choice](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) | [sequence](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence))?)  
</group>

If there is a `name` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) (in which case the
item will have [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) or [<redefine>](053-4-2-2-including-modified-component-definitions.md#element-redefine) as parent), then the item corresponds to
a model group definition component with properties as follows:

<table class="reprcomp"><thead><tr><th><a href="032-3-7-model-group-definitions.md#Model_Group_Definition_details">Model Group Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="032-3-7-model-group-definitions.md#mg-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="032-3-7-model-group-definitions.md#mg-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <code>schema</code>
element information item.</td></tr><tr valign="top"><td><a class="propref" href="032-3-7-model-group-definitions.md#model_group">{model group}</a></td><td>A model group which is the <a class="propref" href="035-3-9-particles.md#term">{term}</a> of a
particle corresponding to the <a class="eltref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all">&lt;all&gt;</a>, <a class="eltref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice">&lt;choice&gt;</a> or
<a class="eltref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence">&lt;sequence&gt;</a> among the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> (there must be one).</td></tr><tr valign="top"><td><a class="propref" href="032-3-7-model-group-definitions.md#mg-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

Otherwise, the item will have a `ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element),
in which case it corresponds to a particle component with properties as follows (unless `minOccurs=maxOccurs=0`, in which case the item
corresponds to no component at all):

<table class="reprcomp"><thead><tr><th><a href="005-2-2-1-3-complex-type-definition.md#Particle">Particle</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-min_occurs">{min occurs}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>minOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-max_occurs">{max occurs}</a></td><td><i>unbounded</i>, if the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> equals <i>unbounded</i>, otherwise the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#term">{term}</a></td><td>The <a class="propref" href="032-3-7-model-group-definitions.md#model_group">{model group}</a> of the
model group definition <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>ref</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr></tbody></table></td></tr></tbody></table>

The name of this section is slightly misleading, in that the second, un-named,
case above (with a
`ref` and no `name`) is not really a named model
group at all, but a reference to one. Also note that in the first (named)
case above no reference is made to `minOccurs` or
`maxOccurs`: this is because the schema for schemas does not allow
them on the child of [<group>](032-3-7-model-group-definitions.md#element-group) when it is named. This in turn is
because the [{min occurs}](035-3-9-particles.md#p-min_occurs) and [{max occurs}](035-3-9-particles.md#p-max_occurs) of
the particles which *refer* to the definition are what count.

Given the constraints on its appearance in content models, an
[<all>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all) should only occur as the only item in the
[[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of a named model group definition or a content model: see [Constraints on Model Group Schema Components (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#coss-modelGroup).

<a id="d0e10150"></a>

#### 3.7.3 Constraints on XML Representations of Model Group Definitions

<a id="src-model_group_defn"></a>

**Schema Representation Constraint: Model Group Definition Representation OK**  

In addition to the conditions imposed on [<group>](032-3-7-model-group-definitions.md#element-group) element
information items by the schema for schemas, the corresponding model group definition, if any, must satisfy the conditions set
out in [Constraints on Model Group Schema Components (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#coss-modelGroup).

<a id="d0e10162"></a>

#### 3.7.4 Model Group Definition Validation Rules

None as such.

<a id="d0e10167"></a>

#### 3.7.5 Model Group Definition Information Set Contributions

None as such.

