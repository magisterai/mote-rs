# 3.4.2 XML Representation of Complex Type Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](020-3-4-complex-type-definitions.md) · [Next](022-continuation-22.md)

[{prohibited substitutions}](020-3-4-complex-type-definitions.md#ct-exact) determine
whether an element declaration appearing in a [·
content model·](005-2-2-1-3-complex-type-definition.md#key-contentModel) is prevented from additionally
[·validating·](004-2-1-overview-of-xml-schema.md#key-vn) element items with an [xsi:type (§2.6.1)](007-2-5-names-and-symbol-spaces.md#xsi_type) attribute that
identifies a complex type definition derived by *extension* or
*restriction* from this definition, or element items in
a substitution group whose type definition is similarly derived:
If [{prohibited substitutions}](020-3-4-complex-type-definitions.md#ct-exact) is empty,
then all such substitutions are allowed, otherwise, the derivation method(s) it
names are disallowed.

See [Annotations (§3.13)](044-3-13-annotations.md#cAnnotations) for information on the role of the
[{annotations}](020-3-4-complex-type-definitions.md#ct-annotations) property.

<a id="declare-type"></a>

#### 3.4.2 XML Representation of Complex Type Definitions

The XML representation for a complex type definition schema component is a
[<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) element information item.

The XML representation for complex type definitions with
a simple type definition [{content type}](020-3-4-complex-type-definitions.md#content_type) is significantly different
from that of those with other [{content type}](020-3-4-complex-type-definitions.md#content_type)s, and this
is reflected in the presentation below, which displays first the elements
involved in the first case, then those for the second. The property mapping is shown once for each case.

XML Representation Summary: `complexType` Element Information Item

<a id="element-complexType"></a>

<complexType  
  abstract = [boolean](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#boolean) : false  
  block =
(#all | List of (extension | restriction))
  
  final =
(#all | List of (extension | restriction))
  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  mixed = [boolean](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#boolean) : false  
  name = [NCName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#NCName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([simpleContent](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent) | [complexContent](024-continuation-24.md#element-complexContent) | (([group](032-3-7-model-group-definitions.md#element-group) | [all](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-all) | [choice](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-choice) | [sequence](033-3-7-6-constraints-on-model-group-definition-schema-components.md#element-sequence))?, (([attribute](009-3-2-attribute-declarations.md#element-attribute) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup))\*, [anyAttribute](022-continuation-22.md#element-anyAttribute)?))))  
</complexType>

Whichever alternative for the content of [<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) is
chosen, the following property mappings apply:

<table class="reprcomp"><thead><tr><th><a href="020-3-4-complex-type-definitions.md#Complex_Type_Definition_details">Complex Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> ancestor
element information item if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-abstract">{abstract}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>abstract</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise <i>false</i>.</td></tr><tr valign="top"><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-exact">{prohibited substitutions}</a></td><td>A set corresponding to the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>block</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise on the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>blockDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the ancestor <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> element
information item, if present, otherwise on the empty string.  Call this the <b>EBV</b> (for effective block value).  Then the value of this property is
 the appropriate <b>case</b> among the following:<div class="constraintlist"><div class="clnumber">1 <b>If </b>the <b>EBV</b> is the empty string, <b>then </b>the empty set;</div><div class="clnumber">2 <b>If </b>the <b>EBV</b> is <code>#all</code>, <b>then </b><code>{</code><i>extension</i>, <i>restriction</i><code>}</code>;</div><div class="clnumber">3 <b>otherwise </b>a set with members drawn from the set above, each being present or
absent depending on whether the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> (which is a list) contains an
equivalently named item.
   <div class="note"><b>Note: </b>Although the <code>blockDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> may include values other than <i>restriction</i> or<i>extension</i>, those values are ignored in the determination of <a class="propref" href="020-3-4-complex-type-definitions.md#ct-exact">{prohibited substitutions}</a> for complex type definitions (they <em>are</em> used elsewhere).</div>
</div></div>
</td></tr><tr valign="top"><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-final">{final}</a></td><td>As for <a class="propref" href="020-3-4-complex-type-definitions.md#ct-exact">{prohibited substitutions}</a> above, but using the
<code>final</code> and <code>finalDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attributes]</a> in place of the
<code>block</code> and <code>blockDefault</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attributes]</a>.</td></tr><tr valign="top"><td><a class="propref" href="020-3-4-complex-type-definitions.md#ct-annotations">{annotations}</a></td><td>The annotations corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, in the <a class="eltref" href="021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent">&lt;simpleContent&gt;</a> and
<a class="eltref" href="024-continuation-24.md#element-complexContent">&lt;complexContent&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, and in their <a class="eltref" href="021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent..restriction">&lt;restriction&gt;</a> and <a class="eltref" href="022-continuation-22.md#element-simpleContent..extension">&lt;extension&gt;</a> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

When the [<simpleContent>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent) alternative is chosen, the following
elements are relevant, and the remaining property mappings are as below. Note that either
[<restriction>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent..restriction) or [<extension>](022-continuation-22.md#element-simpleContent..extension) must be chosen as the
content of [<simpleContent>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent).

<a id="element-simpleContent"></a>

<simpleContent  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([restriction](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-simpleContent..restriction) | [extension](022-continuation-22.md#element-simpleContent..extension)))  
</simpleContent>

<a id="element-simpleContent..restriction"></a>

<restriction  
  **base** = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, ([simpleType](045-3-14-simple-type-definitions.md#element-simpleType)?, ([minExclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minExclusive) | [minInclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minInclusive) | [maxExclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxExclusive) | [maxInclusive](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxInclusive) | [totalDigits](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-totalDigits) | [fractionDigits](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-fractionDigits) | [length](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-length) | [minLength](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-minLength) | [maxLength](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-maxLength) | [enumeration](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-enumeration) | [whiteSpace](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-whiteSpace) | [pattern](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#element-pattern))\*)?, (([attribute](009-3-2-attribute-declarations.md#element-attribute) | [attributeGroup](030-3-5-attributeuses.md#element-attributeGroup))\*, [anyAttribute](022-continuation-22.md#element-anyAttribute)?))  
</restriction>

