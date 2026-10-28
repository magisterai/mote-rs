# 3.2.3 Constraints on XML Representations of Attribute Declarations

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-3-2-attribute-declarations.md) · [Next](011-3-2-4-attribute-declaration-validation-rules.md)

<table class="reprcomp"><thead><tr><th><a href="030-3-5-attributeuses.md#AU_details">Attribute Use</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#required">{required}</a></td><td><i>true</i> if the <code>use</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> is present with <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> <code>required</code>, otherwise
<i>false</i>.</td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#attribute">{attribute declaration}</a></td><td>See the Attribute Declaration mapping
immediately below.</td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#au-value_constraint">{value constraint}</a></td><td>If there is a <code>default</code> or a <code>fixed</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, then a pair consisting of the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> (with respect to the
<a class="propref" href="009-3-2-attribute-declarations.md#a-simple_type_definition">{type definition}</a> of the <a class="propref" href="030-3-5-attributeuses.md#attribute">{attribute declaration}</a>) of that <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> and
either <i>default</i> or <i>fixed</i>, as appropriate, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

<table class="reprcomp"><thead><tr><th><a href="009-3-2-attribute-declarations.md#Attribute_Declaration_details">Attribute Declaration</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-name">{name}</a></td><td>The <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>name</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-target_namespace">{target namespace}</a></td><td>If <code>form</code> is present and its
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> is <code>qualified</code>, or if <code>form</code> is absent and the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of <code>attributeFormDefault</code> on the <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a>
ancestor is <code>qualified</code>, then the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the
<code>targetNamespace</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> of the parent <a class="eltref" href="049-3-15-schemas-as-a-whole.md#element-schema">&lt;schema&gt;</a>
element information item, or <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a> if there
is none, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-simple_type_definition">{type definition}</a></td><td>The simple type definition
corresponding to the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-simpleType">&lt;simpleType&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise the simple type definition <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by
the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>type</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if present, otherwise the
<a class="termref" href="048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself"><span class="arrow">·</span>simple ur-type definition<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-scope">{scope}</a></td><td>If the <a class="eltref" href="009-3-2-attribute-declarations.md#element-attribute">&lt;attribute&gt;</a> element information item
has <a class="eltref" href="021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType">&lt;complexType&gt;</a> as an ancestor, the complex definition
corresponding to that item, otherwise (the <a class="eltref" href="009-3-2-attribute-declarations.md#element-attribute">&lt;attribute&gt;</a> element
information item is within an <a class="eltref" href="030-3-5-attributeuses.md#element-attributeGroup">&lt;attributeGroup&gt;</a> definition), <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-value_constraint">{value constraint}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr><tr valign="top"><td><a class="propref" href="009-3-2-attribute-declarations.md#a-annotation">{annotation}</a></td><td>The annotation corresponding to the <a class="eltref" href="044-3-13-annotations.md#element-annotation">&lt;annotation&gt;</a> element information item in the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a>, if present, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

otherwise (the [<attribute>](009-3-2-attribute-declarations.md#element-attribute) element information item has
[<complexType>](021-3-4-2-xml-representation-of-complex-type-definitions.md#element-complexType) or [<attributeGroup>](030-3-5-attributeuses.md#element-attributeGroup) as an ancestor and the
`ref` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) is present), it corresponds to an
attribute use with properties as follows (unless `use='prohibited'`, in which case the item
corresponds to nothing at all):

<table class="reprcomp"><thead><tr><th><a href="030-3-5-attributeuses.md#AU_details">Attribute Use</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#required">{required}</a></td><td><i>true</i> if the <code>use</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> is present with <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> <code>required</code>, otherwise
<i>false</i>.</td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#attribute">{attribute declaration}</a></td><td>The (top-level) attribute declaration <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the
<a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>ref</code> <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a></td></tr><tr valign="top"><td><a class="propref" href="030-3-5-attributeuses.md#au-value_constraint">{value constraint}</a></td><td>If there is a <code>default</code> or a <code>fixed</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, then a pair consisting of the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> (with respect to the
<a class="propref" href="009-3-2-attribute-declarations.md#a-simple_type_definition">{type definition}</a> of the <a class="propref" href="030-3-5-attributeuses.md#attribute">{attribute declaration}</a>) of that <a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a> and
either <i>default</i> or <i>fixed</i>, as appropriate, otherwise <a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a>.</td></tr></tbody></table></td></tr></tbody></table>

Attribute declarations can appear at the top level of a schema document, or within complex
type definitions, either as complete (local) declarations, or by reference to top-level
declarations, or within attribute group definitions. For complete declarations, top-level or local, the `type` attribute is used when the declaration can use a
built-in or pre-declared simple type definition. Otherwise an
anonymous [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType) is provided inline.

The default when no simple type definition is referenced or
provided is the [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType), which imposes no constraints at all.

Attribute information items [·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by a top-level declaration must be qualified with the
[{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace) of that declaration (if this is [·absent·](008-3-schema-component-details.md#key-null), the item must be unqualified). Control over whether attribute information items
[·validated·](004-2-1-overview-of-xml-schema.md#key-vn) by a local declaration must be similarly qualified or not
is provided by the `form` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element), whose default is provided
by the `attributeFormDefault` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) on the enclosing [<schema>](049-3-15-schemas-as-a-whole.md#element-schema), via its determination of [{target namespace}](009-3-2-attribute-declarations.md#a-target_namespace).

The names for top-level attribute declarations are in their own
[·symbol space·](007-2-5-names-and-symbol-spaces.md#key-symbolSpace). The names of locally-scoped
attribute declarations reside in symbol spaces local to the type definition which contains
them.

<a id="d0e2403"></a>

#### 3.2.3 Constraints on XML Representations of Attribute Declarations

<a id="src-attribute"></a>

**Schema Representation Constraint: Attribute Declaration Representation OK**  

In addition to the conditions imposed on [<attribute>](009-3-2-attribute-declarations.md#element-attribute) element
information items by the schema for schemas,
**all** of the following must be true:

1 `default` and `fixed` must not both be present.

2 If `default` and `use` are both present,
`use` must have the [·actual value·](008-3-schema-component-details.md#key-vv) `optional`.

3 If the item's parent is not [<schema>](049-3-15-schemas-as-a-whole.md#element-schema), then
**all** of the following must be true:

3.1 One of `ref` or `name` must be present, but not both.

3.2 If `ref` is present, then all of [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType),
`form` and `type` must be absent.

4 `type` and [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType)
must not both be present.

