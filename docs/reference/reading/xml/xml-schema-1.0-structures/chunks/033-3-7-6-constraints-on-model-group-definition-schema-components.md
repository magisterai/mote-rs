# 3.7.6 Constraints on Model Group Definition Schema Components

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](032-3-7-model-group-definitions.md) · [Next](034-3-8-5-model-group-information-set-contributions.md)

<a id="coss-groupDef"></a>

#### 3.7.6 Constraints on Model Group Definition Schema Components

All model group definitions (see [Model Group Definitions (§3.7)](032-3-7-model-group-definitions.md#cModel_Group_Definitions)) must satisfy the following constraint.

<a id="mgd-props-correct"></a>

**Schema Component Constraint: Model Group Definition Properties Correct**  

The values of the properties of a model group definition must be as described in
the property tableau in
[The Model Group Definition Schema Component (§3.7.1)](032-3-7-model-group-definitions.md#Model_Group_Definition_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

<a id="Model_Groups"></a>

### 3.8 Model Groups

3.8.1 [The Model Group Schema Component](033-3-7-6-constraints-on-model-group-definition-schema-components.md#Model_Group_details)  
        3.8.2 [XML Representation of Model Group Schema Components](033-3-7-6-constraints-on-model-group-definition-schema-components.md#declare-contentModel)  
        3.8.3 [Constraints on XML Representations of Model Groups](033-3-7-6-constraints-on-model-group-definition-schema-components.md#d0e10427)  
        3.8.4 [Model Group Validation Rules](033-3-7-6-constraints-on-model-group-definition-schema-components.md#d0e10445)  
        3.8.5 [Model Group Information Set Contributions](034-3-8-5-model-group-information-set-contributions.md#d0e10582)  
        3.8.6 [Constraints on Model Group Schema Components](034-3-8-5-model-group-information-set-contributions.md#coss-modelGroup)

When the [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) of element information items are not constrained
to be *empty* or by reference to a simple type definition
([Simple Type Definitions (§3.14)](045-3-14-simple-type-definitions.md#Simple_Type_Definitions)), the sequence of element
information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) content may be specified in
more detail with a model group. Because the [{term}](035-3-9-particles.md#term) property of a particle can be a
model group, and model groups contain particles, model groups can indirectly contain other model groups; the grammar for content models
is therefore recursive.

Example



```
<xs:all>
 <xs:element ref="cats"/>
 <xs:element ref="dogs"/>
</xs:all>

<xs:sequence>
 <xs:choice>
  <xs:element ref="left"/>
  <xs:element ref="right"/>
 </xs:choice>
 <xs:element ref="landmark"/>
</xs:sequence>
```




XML representations for the three kinds of model group, the third nested
inside the second.

<a id="Model_Group_details"></a>

#### 3.8.1 The Model Group Schema Component

The model group schema component has the following
properties:

Schema Component: [Model Group](005-2-2-1-3-complex-type-definition.md#Model_Group)

<a id="compositor"></a>

{compositor}
:   One of *all*, *choice* or *sequence*.

<a id="particles"></a>

{particles}
:   A list of particles

<a id="amg-annotation"></a>

{annotation}
:   Optional. An annotation.

specifies a sequential (*sequence*),
disjunctive (*choice*) or conjunctive (*all*) interpretation of
the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles). This in turn
determines whether the element
information item [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element) [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by the model group must:

- (*sequence*) correspond, in order, to the specified [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles);
- (*choice*) corresponded to exactly one of the specified [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles);
- (*all*) contain all and only exactly zero or one of each
  element specified in [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles). The elements can occur in any
  order. In this case, to reduce implementation complexity, [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) is restricted to contain local and top-level element
  declarations only, with [{min occurs}](035-3-9-particles.md#p-min_occurs)`=0` or
  `1`, [{max occurs}](035-3-9-particles.md#p-max_occurs)`=1`.

When two or more particles contained directly or indirectly in the
[{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of a model group have identically named
element declarations as their
[{term}](035-3-9-particles.md#term), the type definitions of those declarations must be the
same. By 'indirectly' is meant particles within the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles)
of a group which is itself the [{term}](035-3-9-particles.md#term) of a directly contained
particle, and so on recursively.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotation}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#amg-annotation) property.

<a id="declare-contentModel"></a>

#### 3.8.2 XML Representation of Model Group Schema Components

The XML representation for a model group schema component is
either an
[<all>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all), a [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) or a [<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence)
element information item. The correspondences between the
properties of those information items and
properties of the component they correspond to are as follows:

XML Representation Summary: `all` Element Information Item

<a id="element-all"></a>

<all  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  maxOccurs = 1 : 1  
  minOccurs = (0 | 1) : 1  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, [element](013-3-3-element-declarations.md#element-element)\*)  
</all>

<a id="element-choice"></a>

<choice  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  maxOccurs =
([nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) | unbounded)
 : 1  
  minOccurs = [nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) : 1  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([element](013-3-3-element-declarations.md#element-element) | [group](032-3-7-model-group-definitions.md#element-group) | [choice](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) | [sequence](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence) | [any](038-3-10-wildcards.md#element-any))\*)  
</choice>

<a id="element-sequence"></a>

<sequence  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  maxOccurs =
([nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) | unbounded)
 : 1  
  minOccurs = [nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) : 1  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([element](013-3-3-element-declarations.md#element-element) | [group](032-3-7-model-group-definitions.md#element-group) | [choice](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) | [sequence](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence) | [any](038-3-10-wildcards.md#element-any))\*)  
</sequence>

Each of the above items corresponds to a particle containing a model
group, with properties as follows (unless `minOccurs=maxOccurs=0`, in which case the item
corresponds to no component at all):

<table class="reprcomp"><thead><tr><th><a href="035-3-9-particles.md#Particle_details">Particle</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-min_occurs">{min occurs}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>minOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-max_occurs">{max occurs}</a></td><td><i>unbounded</i>, if the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> equals <i>unbounded</i>, otherwise the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#term">{term}</a></td><td>A model group as given below:</td></tr></tbody></table></td></tr></tbody></table>

<table class="reprcomp"><thead><tr><th><a href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#Model_Group_details">Model Group</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#compositor">{compositor}</a></td><td>One of <i>all</i>, <i>choice</i>,
<i>sequence</i> depending on the element information item.</td></tr><tr valign="top"><td><a class="propref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles">{particles}</a></td><td>A sequence of particles
corresponding to all the <a class="eltref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all">&lt;all&gt;</a>, <a class="eltref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice">&lt;choice&gt;</a>,
<a class="eltref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence">&lt;sequence&gt;</a>, <a class="eltref" href="038-3-10-wildcards.md#element-any">&lt;any&gt;</a>,
<a class="eltref" href="032-3-7-model-group-definitions.md#element-group">&lt;group&gt;</a> or <a class="eltref" href="013-3-3-element-declarations.md#element-element">&lt;element&gt;</a> items among the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>,
in order.</td></tr><tr valign="top"><td><a class="propref" href="033-3-7-6-constraints-on-model-group-definition-schema-components.md#amg-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

<a id="d0e10427"></a>

#### 3.8.3 Constraints on XML Representations of Model Groups

<a id="src-model_group"></a>

**Schema Representation Constraint: Model Group Representation OK**  

In addition to the conditions imposed on [<all>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all), [<choice>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) and [<sequence>](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence) element
information items by the schema for schemas, the corresponding particle and model group must satisfy the conditions set
out in [Constraints on Model Group Schema Components (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#coss-modelGroup) and [Constraints on Particle Schema Components (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#coss-particle).

<a id="d0e10445"></a>

#### 3.8.4 Model Group Validation Rules

<a id="cvc-model-group"></a>

**Validation Rule: Element Sequence Valid**  

