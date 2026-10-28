# Continuation 46

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](045-3-14-simple-type-definitions.md) · [Next](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md)

<a id="element-list"></a>

<list  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  itemType = [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, [simpleType](045-3-14-simple-type-definitions.md#element-simpleType)?)  
</list>

<a id="element-union"></a>

<union  
  id = [ID](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ID)  
  memberTypes = List of [QName](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#QName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](044-3-13-annotations.md#element-annotation)?, [simpleType](045-3-14-simple-type-definitions.md#element-simpleType)\*)  
</union>

<table class="reprcomp"><thead><tr><th><a href="045-3-14-simple-type-definitions.md#Simple_Type_Definition_details">Simple Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-target_namespace">{target namespace}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a> ancestor
element information item if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a></td><td>
   The appropriate <b>case</b> among the following:<div class="constraintlist"><div class="clnumber">1 <b>If </b>the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> alternative is chosen, <b>then </b>the type definition <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>base</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a>, if present, otherwise the
type definition corresponding to the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-simpleType">&lt;simpleType&gt;</a> among
the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> of <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a>.</div><div class="clnumber">2 <b>If </b>the <a class="eltref" href="046-continuation-46.md#element-list">&lt;list&gt;</a> or <a class="eltref" href="046-continuation-46.md#element-union">&lt;union&gt;</a> alternative is chosen, <b>then </b>the <a class="termref" href="048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself"><span class="arrow">·</span>simple ur-type definition<span class="arrow">·</span></a>.</div></div>
</td></tr><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-final">{final}</a></td><td>As for the <a class="propref" href="020-3-4-complex-type-definitions.md#ct-exact">{prohibited substitutions}</a> property of
complex type definitions, but using the
<code>final</code> and <code>finalDefault</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attributes]</a> in place of the
<code>block</code> and <code>blockDefault</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attributes]</a> and with the
relevant set being <code>{</code><i>extension</i>,
<i>restriction</i>, <i>list</i>, <i>union</i><code>}</code>.</td></tr><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#variety">{variety}</a></td><td>If the <a class="eltref" href="046-continuation-46.md#element-list">&lt;list&gt;</a> alternative is chosen,
then <i>list</i>, otherwise if the <a class="eltref" href="046-continuation-46.md#element-union">&lt;union&gt;</a> alternative is
chosen, then <i>union</i>, otherwise (the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a>
alternative is chosen), then the <a class="propref" href="045-3-14-simple-type-definitions.md#variety">{variety}</a> of the <a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a>.</td></tr></tbody></table></td></tr></tbody></table>

If the [{variety}](045-3-14-simple-type-definitions.md#variety) is *atomic*, the following
additional property mappings also apply:

<table class="reprcomp"><thead><tr><th><a href="045-3-14-simple-type-definitions.md#Simple_Type_Definition_details">Atomic Simple Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-primitive_type_definition">{primitive type definition}</a></td><td>The built-in primitive type
definition from which the <a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a> is derived.</td></tr><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#facets">{facets}</a></td><td>A set of facet components <a class="termref" href="048-3-14-7-built-in-simple-type-definition.md#key-facets-restriction"><span class="arrow">·</span>constituting a restriction<span class="arrow">·</span></a>
of the <a class="propref" href="045-3-14-simple-type-definitions.md#facets">{facets}</a> of the
<a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a> with respect to a
set of facet components corresponding to the appropriate element information items among the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> of <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> (i.e. those which specify facets, if any), as
defined in <a href="048-3-14-7-built-in-simple-type-definition.md#st-restrict-facets">Simple Type Restriction (Facets) (§3.14.6)</a>.</td></tr></tbody></table></td></tr></tbody></table>

If the [{variety}](045-3-14-simple-type-definitions.md#variety) is *list*, the following
additional property mappings also apply:

<table class="reprcomp"><thead><tr><th><a href="045-3-14-simple-type-definitions.md#Simple_Type_Definition_details">List Simple Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-item_type_definition">{item type definition}</a></td><td>
    The appropriate <b>case</b> among the following:<div class="constraintlist"><div class="clnumber">1 <b>If </b>the <a class="eltref" href="046-continuation-46.md#element-list">&lt;list&gt;</a> alternative is chosen, <b>then </b>the type definition <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>itemType</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of <a class="eltref" href="046-continuation-46.md#element-list">&lt;list&gt;</a>, if present, otherwise the
type definition corresponding to the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-simpleType">&lt;simpleType&gt;</a> among
the <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> of <a class="eltref" href="046-continuation-46.md#element-list">&lt;list&gt;</a>.</div><div class="clnumber">2 <b>If </b>the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> option is chosen, <b>then </b>the <a class="propref" href="045-3-14-simple-type-definitions.md#st-item_type_definition">{item type definition}</a> of the <a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a>.</div></div>
</td></tr><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#facets">{facets}</a></td><td>If the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> alternative is
chosen, a set of facet components <a class="termref" href="048-3-14-7-built-in-simple-type-definition.md#key-facets-restriction"><span class="arrow">·</span>constituting a restriction<span class="arrow">·</span></a>
of the <a class="propref" href="045-3-14-simple-type-definitions.md#facets">{facets}</a> of the
<a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a> with respect to a
set of facet components corresponding to the appropriate element information items among the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> of <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> (i.e. those which specify facets, if any), as
defined in <a href="048-3-14-7-built-in-simple-type-definition.md#st-restrict-facets">Simple Type Restriction (Facets) (§3.14.6)</a>, otherwise the empty set.</td></tr></tbody></table></td></tr></tbody></table>

If the [{variety}](045-3-14-simple-type-definitions.md#variety) is *union*, the following
additional property mappings also apply:

