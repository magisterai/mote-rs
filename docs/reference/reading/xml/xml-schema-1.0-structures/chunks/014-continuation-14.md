# Continuation 14

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-3-3-element-declarations.md) · [Next](015-continuation-15.md)

  fixed = [string](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#string)  
  form = (qualified | unqualified)  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  maxOccurs =
([nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) | unbounded)
 : 1  
  minOccurs = [nonNegativeInteger](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#nonNegativeInteger) : 1  
  name = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  nillable = [boolean](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#boolean) : false  
  ref = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  substitutionGroup = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  type = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, (([simpleType](045-3-14-simple-type-definitions.md#element-simpleType) | [complexType](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType))?, ([unique](040-3-11-identity-constraint-definitions.md#element-unique) | [key](040-3-11-identity-constraint-definitions.md#element-key) | [keyref](040-3-11-identity-constraint-definitions.md#element-keyref))\*))  
</element>

If the [<element>](013-3-3-element-declarations.md#element-element) element information item has [<schema>](049-3-15-schemas-as-a-whole.md#element-schema) as its parent, the corresponding schema component is as follows:

<table class="reprcomp"><thead><tr><th><a href="013-3-3-element-declarations.md#Element_Declaration_details">Element Declaration</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a>
element information item, or <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a> if there is none.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-scope">{scope}</a></td><td><i>global</i>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a></td><td>The type definition
corresponding to the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-simpleType">&lt;simpleType&gt;</a> or <a class="eltref" href="021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType">&lt;complexType&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if either is present, otherwise the type definition <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by
the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>type</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, otherwise the <a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a> of the element declaration <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>substitutionGroup</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise the
<a class="termref" href="029-3-4-7-built-in-complex-type-definition.md#ur-type-itself"><span class="arrow">·</span>ur-type definition<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#nillable">{nillable}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>nillable</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <i>false</i>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-value_constraint">{value constraint}</a></td><td>If there is a <code>default</code> or a <code>fixed</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, then a pair consisting of the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> (with respect to the
<a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a>, if it is a simple type definition, or the
<a class="propref" href="013-3-3-element-declarations.md#type_definition">{type definition}</a>'s <a class="propref" href="020-3-4-complex-type-definitions.md#content_type">{content type}</a>, if that is a
simple type definition, or else with respect to the built-in <a href="https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#string">string</a> simple type definition) of that <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> and
either <i>default</i> or <i>fixed</i>, as appropriate, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#identity-constraint_definitions">{identity-constraint definitions}</a></td><td>A set consisting of the
identity-constraint-definitions corresponding to all the <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-key">&lt;key&gt;</a>, <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-unique">&lt;unique&gt;</a> and <a class="eltref" href="040-3-11-identity-constraint-definitions.md#element-keyref">&lt;keyref&gt;</a> element information items in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if any, otherwise the empty set.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#class_exemplar">{substitution group affiliation}</a></td><td>The element declaration <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>substitutionGroup</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-exact">{disallowed substitutions}</a></td><td>A set depending on the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>block</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise on the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>blockDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the ancestor <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> element
information item, if present, otherwise on the empty string.  Call this the <b>EBV</b> (for effective block value).  Then the value of this property is
 the appropriate <b>case</b> among the following:<div class="constraintlist"><div class="clnumber">1 <b>If </b>the <b>EBV</b> is the empty string, <b>then </b>the empty set;</div><div class="clnumber">2 <b>If </b>the <b>EBV</b> is <code>#all</code>, <b>then </b><code>{</code><i>extension</i>, <i>restriction</i>, <i>substitution</i><code>}</code>;</div><div class="clnumber">3 <b>otherwise </b>a set with members drawn from the set above, each being present or
absent depending on whether the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> (which is a list) contains an
equivalently named item.
   <div class="note"><b>Note: </b>Although the <code>blockDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> may include values other than <i>extension</i>, <i>restriction</i> or <i>substitution</i>, those values are ignored in the determination of <a class="propref" href="013-3-3-element-declarations.md#e-exact">{disallowed substitutions}</a> for element declarations (they <em>are</em> used elsewhere).</div>
</div></div>
</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-final">{substitution group exclusions}</a></td><td>As for <a class="propref" href="013-3-3-element-declarations.md#e-exact">{disallowed substitutions}</a> above, but using the
<code>final</code> and <code>finalDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attributes]</a> in place of the
<code>block</code> and <code>blockDefault</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attributes]</a> and with the
relevant set being <code>{</code><i>extension</i>, <i>restriction</i><code>}</code>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-abstract">{abstract}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>abstract</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <i>false</i>.</td></tr><tr valign="top"><td><a class="propref" href="013-3-3-element-declarations.md#e-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

otherwise if the [<element>](013-3-3-element-declarations.md#element-element) element information item has
[<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) or [<group>](032-3-7-model-group-definitions.md#element-group) as an ancestor and the
`ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is absent, the corresponding schema components
are as follows (unless `minOccurs=maxOccurs=0`, in which case the item
corresponds to no component at all):

