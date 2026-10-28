# Continuation 15

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-continuation-14.md) · [Next](016-3-3-3-constraints-on-xml-representations-of-element-declarations.md)

<table class="reprcomp"><thead><tr><th><a href="035-3-9-particles.md#Particle_details">Particle</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-min_occurs">{min occurs}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>minOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-max_occurs">{max occurs}</a></td><td><i>unbounded</i>, if the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> equals <i>unbounded</i>, otherwise the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#term">{term}</a></td><td>A (local) element declaration as given below.</td></tr></tbody></table></td></tr></tbody></table>

An element declaration as in the first case above, with the exception of its [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) and [{scope}](013-3-3-element-declarations.md#e-scope) properties, which are as below:

<table class="reprcomp"><thead><tr><th><a href="013-3-3-element-declarations.md#Element_Declaration_details">Element Declaration</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-target_namespace">{target namespace}</a></td><td>If <code>form</code> is present and its
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> is <code>qualified</code>, or if <code>form</code> is absent and the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of <code>elementFormDefault</code> on the <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a>
ancestor is <code>qualified</code>, then the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a>
element information item, or <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a> if there
is none, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-scope">{scope}</a></td><td>If the <a class="eltref" href="013-3-3-element-declarations.md#element-element">&lt;element&gt;</a> element information item
has <a class="eltref" href="021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType">&lt;complexType&gt;</a> as an ancestor, the complex definition
corresponding to that item, otherwise (the <a class="eltref" href="013-3-3-element-declarations.md#element-element">&lt;element&gt;</a> element
information item is within a named <a class="eltref" href="032-3-7-model-group-definitions.md#element-group">&lt;group&gt;</a> definition), <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

otherwise (the [<element>](013-3-3-element-declarations.md#element-element) element information item has
[<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) or [<group>](032-3-7-model-group-definitions.md#element-group) as an ancestor and the
`ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is present), the corresponding schema component is as
follows (unless `minOccurs=maxOccurs=0`, in which case the item
corresponds to no component at all):

<table class="reprcomp"><thead><tr><th><a href="035-3-9-particles.md#Particle_details">Particle</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-min_occurs">{min occurs}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>minOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#p-max_occurs">{max occurs}</a></td><td><i>unbounded</i>, if the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> equals <i>unbounded</i>, otherwise the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>maxOccurs</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <code>1</code>.</td></tr><tr valign="top"><td><a class="propref" href="035-3-9-particles.md#term">{term}</a></td><td>The (top-level) element declaration <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>ref</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>.</td></tr></tbody></table></td></tr></tbody></table>

[<element>](013-3-3-element-declarations.md#element-element) corresponds to an element declaration, and allows
the type definition of that declaration to be specified either by reference or
by explicit inclusion.

[<element>](013-3-3-element-declarations.md#element-element)s within [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) produce
*global* element declarations; [<element>](013-3-3-element-declarations.md#element-element)s within [<group>](032-3-7-model-group-definitions.md#element-group) or [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) produce either particles which contain *global* element declarations (if there's a `ref` attribute) or local declarations (otherwise). For complete declarations, top-level or local, the `type` attribute is used when the declaration can use a
built-in or pre-declared type definition. Otherwise an
anonymous [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType) or [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) is provided inline.

Element information items [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by a top-level declaration must be qualified with the
[{target namespace}](013-3-3-element-declarations.md#e-target_namespace) of that declaration (if this is [·absent·](008-3-schema-component-details.md#key-null), the item must be unqualified). Control over whether element information items [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by a local declaration must be similarly qualified or not
is provided by the `form` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element), whose default is provided
by the `elementFormDefault` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) on the enclosing [<schema>](049-3-15-schemas-as-a-whole.md#element-schema), via its determination of [{target namespace}](013-3-3-element-declarations.md#e-target_namespace).

As noted above the names for top-level element declarations are in a separate
[·symbol space·](007-2-5-names-and-symbol-spaces.md#key-symbolSpace) from the symbol spaces for
the names of type definitions, so there can (but need
not be) a simple or complex type definition with the same name as a
top-level element. As with attribute names, the names of locally-scoped
element declarations with no [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) reside in symbol spaces local to the type definition which contains
them.

Note that the above allows for two levels of defaulting for unspecified
type definitions. An [<element>](013-3-3-element-declarations.md#element-element) with no referenced or included type definition will
correspond to an element declaration which has the same type definition as the
head of its substitution group if it identifies one, otherwise the [·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType). This has the important consequence that the minimum valid element declaration, that is, one with only a `name` attribute and no contents, is also (nearly) the most general, validating any combination of text and element content and allowing any attributes, and providing for recursive validation where possible.

See below at [XML Representation of Identity-constraint Definition Schema Components (§3.11.2)](040-3-11-identity-constraint-definitions.md#declare-key) for [<key>](040-3-11-identity-constraint-definitions.md#element-key), [<unique>](040-3-11-identity-constraint-definitions.md#element-unique) and [<keyref>](040-3-11-identity-constraint-definitions.md#element-keyref).

Example



```
<xs:element name="unconstrained"/>

<xs:element name="emptyElt">
 <xs:complexType>
  <xs:attribute ...>. . .</xs:attribute>
 </xs:complexType>
</xs:element>

<xs:element name="contextOne">
 <xs:complexType>
  <xs:sequence>
   <xs:element name="myLocalElement" type="myFirstType"/>
   <xs:element ref="globalElement"/>
  </xs:sequence>
 </xs:complexType>
</xs:element>

<xs:element name="contextTwo">
 <xs:complexType>
  <xs:sequence>
   <xs:element name="myLocalElement" type="mySecondType"/>
   <xs:element ref="globalElement"/>
  </xs:sequence>
 </xs:complexType>
</xs:element>
```




The first example above declares an element whose type, by default, is the
[·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType). The second uses an embedded anonymous complex
type definition.The last two examples illustrate the use of local element declarations. Instances of `myLocalElement` within
`contextOne` will be constrained by `myFirstType`,
while those within `contextTwo` will be constrained by
`mySecondType`.

**Note:**The possibility that differing attribute declarations and/or content models
would apply to elements with the same name in different contexts is an
extension beyond the expressive power of a DTD in XML 1.0.

Example



