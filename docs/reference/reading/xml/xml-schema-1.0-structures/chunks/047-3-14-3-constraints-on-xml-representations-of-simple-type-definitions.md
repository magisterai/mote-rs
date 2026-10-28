# 3.14.3 Constraints on XML Representations of Simple Type Definitions

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](046-continuation-46.md) · [Next](048-3-14-7-built-in-simple-type-definition.md)

<table class="reprcomp"><thead><tr><th><a href="045-3-14-simple-type-definitions.md#Simple_Type_Definition_details">Union Simple Type Definition</a><strong> Schema Component</strong></th></tr></thead><tbody><tr><td><a id="key-exm"></a><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Representation</th></tr></thead><tbody valign="top"><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-member_type_definitions">{member type definitions}</a></td><td>The appropriate <b>case</b> among the following:<div class="constraintlist"><div class="clnumber">1 <b>If </b>the <a class="eltref" href="046-continuation-46.md#element-union">&lt;union&gt;</a> alternative is chosen, <b>then </b><span class="termdef"><a id="key-exm" name="key-exm">[Definition:]  </a>define the
<b>explicit members</b> as</span> the type definitions <a class="termref" href="050-3-15-2-1-references-to-schema-components.md#src-resolve"><span class="arrow">·</span>resolved<span class="arrow">·</span></a> to by the
items in the <a class="termref" href="008-3-schema-component-details.md#key-vv"><span class="arrow">·</span>actual value<span class="arrow">·</span></a> of the <code>memberTypes</code>
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[attribute]</a>, if any, followed by the
type definitions corresponding to the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-simpleType">&lt;simpleType&gt;</a>s among the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> of <a class="eltref" href="046-continuation-46.md#element-union">&lt;union&gt;</a>, if any.  The actual value is then formed by replacing any union type definition
in the <a class="termref" href="047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#key-exm"><span class="arrow">·</span>explicit members<span class="arrow">·</span></a> with the members of
their <a class="propref" href="045-3-14-simple-type-definitions.md#st-member_type_definitions">{member type definitions}</a>, in order.</div><div class="clnumber">2 <b>If </b>the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> option is chosen, <b>then </b>the <a class="propref" href="045-3-14-simple-type-definitions.md#st-member_type_definitions">{member type definitions}</a> of the <a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a>.</div></div>
</td></tr><tr valign="top"><td><a class="propref" href="045-3-14-simple-type-definitions.md#facets">{facets}</a></td><td>If the <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> alternative is
chosen, a set of facet components <a class="termref" href="048-3-14-7-built-in-simple-type-definition.md#key-facets-restriction"><span class="arrow">·</span>constituting a restriction<span class="arrow">·</span></a>
of the <a class="propref" href="045-3-14-simple-type-definitions.md#facets">{facets}</a> of the
<a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a> with respect to a
set of facet components corresponding to the appropriate element information items among the
<a class="xpropref" href="https://www.w3.org/TR/xml-infoset/#infoitem.element">[children]</a> of <a class="eltref" href="045-3-14-simple-type-definitions.md#element-restriction">&lt;restriction&gt;</a> (i.e. those which specify facets, if any), as
defined in <a href="048-3-14-7-built-in-simple-type-definition.md#st-restrict-facets">Simple Type Restriction (Facets) (§3.14.6)</a>, otherwise the empty set.</td></tr></tbody></table></td></tr></tbody></table>

<a id="d0e15439"></a>

#### 3.14.3 Constraints on XML Representations of Simple Type Definitions

<a id="src-simple-type"></a>

**Schema Representation Constraint: Simple Type Definition Representation OK**  

In addition to the conditions imposed on [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType) element
information items by the schema for schemas,
**all** of the following must be true:

1 The corresponding simple type definition, if any, must satisfy the conditions set
out in [Constraints on Simple Type Definition
Schema Components (§3.14.6)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#coss-st).

2 If the [<restriction>](045-3-14-simple-type-definitions.md#element-restriction) alternative is chosen, either
it must have a `base` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) or a [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType)
among its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), but not both.

3 If the [<list>](046-continuation-46.md#element-list) alternative is chosen, either
it must have an `itemType` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) or a [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType)
among its [[children]](https://www.w3.org/TR/xml-infoset/#infoitem.element), but not both.

4 Circular union type definition is disallowed. That is, if the [<union>](046-continuation-46.md#element-union) alternative is chosen, there must not be any entries in the `memberTypes` [[attribute]](https://www.w3.org/TR/xml-infoset/#infoitem.element) at any depth which resolve to the component corresponding to the [<simpleType>](045-3-14-simple-type-definitions.md#element-simpleType).

<a id="d0e15503"></a>

#### 3.14.4 Simple Type Definition Validation Rules

<a id="cvc-simple-type"></a>

**Validation Rule: String Valid**  

For a string to be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a simple type definition
**all** of the following must be true:

1 It is schema-valid with respect to that definition as defined by
[Datatype Valid](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#cvc-datatype-valid) in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

2 The appropriate **case** among the following
must be true:

2.1 **If** The definition is [ENTITY](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ENTITY) or is validly
derived from [ENTITY](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ENTITY) given the empty set,
as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok), **then** the string must be a [·declared
entity name·](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#key-vde).

2.2 **If** The definition is [ENTITIES](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ENTITIES) or is validly
derived from [ENTITIES](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#ENTITIES) given the empty set,
as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok), **then** every whitespace-delimited substring of the string must be a [·declared
entity name·](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#key-vde).

2.3 **otherwise** no further condition applies.

<a id="key-vde"></a>

[Definition:]A string is a **declared entity name** if it is equal to the [[name]](https://www.w3.org/TR/xml-infoset/#infoitem.entity.unparsed) of some unparsed entity
information item in the value of the
[[unparsedEntities]](https://www.w3.org/TR/xml-infoset/#infoitem.document) property of the document information item
at the root of the infoset containing the element or attribute information item
whose [·normalized value·](008-3-schema-component-details.md#key-nv) the string is.

<a id="d0e15574"></a>

#### 3.14.5 Simple Type Definition Information Set Contributions

None as such.

<a id="coss-st"></a>

#### 3.14.6 Constraints on Simple Type Definition Schema Components

All simple type definitions other than the [·simple ur-type
definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself) and the built-in primitive datatype definitions (see [Simple Type Definitions (§3.14)](045-3-14-simple-type-definitions.md#Simple_Type_Definitions)) must satisfy both the following constraints.

<a id="st-props-correct"></a>

**Schema Component Constraint: Simple Type Definition Properties Correct**  

**All** of the following must be true:

1 The values of the properties of a simple type definition must be as described in
the property tableau in
[Datatype definition](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dc-defn), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

2 All simple type definitions must be derived
ultimately from the [·simple ur-type
definition (so·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself) circular definitions are disallowed). That is, it must be possible to reach a built-in
primitive datatype or the [·simple ur-type definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself) by repeatedly following the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition).

3 The [{final}](045-3-14-simple-type-definitions.md#st-final) of the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) must not contain *restriction*.

<a id="cos-st-restricts"></a>

**Schema Component Constraint: Derivation Valid (Restriction, Simple)**  

The appropriate **case** among the following
must be true:

1 **If** the [{variety}](045-3-14-simple-type-definitions.md#variety) is *atomic*, **then** 
**all** of the following must be true:

1.1 The [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) must be an atomic simple type
definition or a built-in
primitive datatype.

1.2 The [{final}](045-3-14-simple-type-definitions.md#st-final) of the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) must not contain *restriction*.

1.3 For each facet in the [{facets}](045-3-14-simple-type-definitions.md#facets) (call this **DF**)
**all** of the following must be true:

1.3.1 **DF** must be an allowed constraining facet for the
[{primitive type definition}](045-3-14-simple-type-definitions.md#st-primitive_type_definition), as specified in the appropriate
subsection of [3.2 Primitive datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#built-in-primitive-datatypes).

1.3.2 If there is a facet of the same kind in the
[{facets}](045-3-14-simple-type-definitions.md#facets) of the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) (call
this **BF**),then the **DF**'s
{value} must be a valid restriction of **BF**'s {value} as
defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

2 **If** the [{variety}](045-3-14-simple-type-definitions.md#variety) is *list*, **then** 
**all** of the following must be true:

2.1 The [{item type definition}](045-3-14-simple-type-definitions.md#st-item_type_definition) must have a [{variety}](045-3-14-simple-type-definitions.md#variety) of *atomic* or *union* (in which case all the [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) must be *atomic*).

2.2

2.3 The appropriate **case** among the following
must be true:

2.3.1 **If** the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition)
is the
[·simple ur-type definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself)
, **then** 
**all** of the following must be true:

2.3.1.1 The [{final}](045-3-14-simple-type-definitions.md#st-final) of the [{item type definition}](045-3-14-simple-type-definitions.md#st-item_type_definition) must not contain *list*.

2.3.1.2 The [{facets}](045-3-14-simple-type-definitions.md#facets) must only contain the
*whiteSpace* facet component.

2.3.2 **otherwise** 
**all** of the following must be true:

2.3.2.1 The [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) must have a [{variety}](045-3-14-simple-type-definitions.md#variety) of *list*.

2.3.2.2 The [{final}](045-3-14-simple-type-definitions.md#st-final) of the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) must not contain *restriction*.

2.3.2.3 The [{item type definition}](045-3-14-simple-type-definitions.md#st-item_type_definition) must
be validly derived from the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition)'s [{item type definition}](045-3-14-simple-type-definitions.md#st-item_type_definition) given the empty set, as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok).

