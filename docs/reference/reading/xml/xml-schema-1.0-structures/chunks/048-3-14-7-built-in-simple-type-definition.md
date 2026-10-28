# 3.14.7 Built-in Simple Type Definition

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md) · [Next](049-3-15-schemas-as-a-whole.md)

2.3.2.4 Only *length*, *minLength*, *maxLength*, *whiteSpace*,
*pattern* and *enumeration* facet components are allowed among
the [{facets}](045-3-14-simple-type-definitions.md#facets).

2.3.2.5 For each facet in the [{facets}](045-3-14-simple-type-definitions.md#facets) (call this **DF**), if there is a facet of the same kind in the
[{facets}](045-3-14-simple-type-definitions.md#facets) of the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) (call
this **BF**),then the **DF**'s
{value} must be a valid restriction of **BF**'s {value} as
defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

The first case above will apply when a list is derived by
specifying an item type, the second when derived by restriction from another list.

3 **If** the [{variety}](045-3-14-simple-type-definitions.md#variety) is *union*, **then** 
**all** of the following must be true:

3.1 The [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) must all have [{variety}](045-3-14-simple-type-definitions.md#variety) of *atomic* or *list*.

3.2

3.3 The appropriate **case** among the following
must be true:

3.3.1 **If** the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition)
is the
[·simple ur-type definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself)
, **then** 
**all** of the following must be true:

3.3.1.1 All of the [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) must
have a [{final}](045-3-14-simple-type-definitions.md#st-final) which does not contain *union*.

3.3.1.2 The [{facets}](045-3-14-simple-type-definitions.md#facets) must be empty.

3.3.2 **otherwise** 
**all** of the following must be true:

3.3.2.1 The [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) must have a [{variety}](045-3-14-simple-type-definitions.md#variety) of *union*.

3.3.2.2 The [{final}](045-3-14-simple-type-definitions.md#st-final) of the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) must not contain *restriction*.

3.3.2.3 The [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions), in order, must
be validly derived from the corresponding type definitions in the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition)'s [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) given the empty set, as defined in [Type Derivation OK (Simple) (§3.14.6)](048-3-14-7-built-in-simple-type-definition.md#cos-st-derived-ok).

3.3.2.4 Only *pattern* and *enumeration* facet components are allowed among
the [{facets}](045-3-14-simple-type-definitions.md#facets).

3.3.2.5 For each facet in the [{facets}](045-3-14-simple-type-definitions.md#facets) (call this **DF**), if there is a facet of the same kind in the
[{facets}](045-3-14-simple-type-definitions.md#facets) of the [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) (call
this **BF**),then the **DF**'s
{value} must be a valid restriction of **BF**'s {value} as
defined in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2).

The first case above will apply when a union is derived by
specifying one or more member types, the second when derived by restriction from another union.

<a id="cd-st-restriction"></a>

[Definition:]If this
constraint [Derivation Valid (Restriction, Simple) (§3.14.6)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#cos-st-restricts) holds of a simple type definition, it is a **valid
restriction** of its [·base type definition·](045-3-14-simple-type-definitions.md#st-base_type_definition).

The following constraint defines relations appealed to elsewhere in this specification.

<a id="cos-st-derived-ok"></a>

**Schema Component Constraint: Type Derivation OK (Simple)**  

For a simple type definition (call it **D**, for derived) to be validly
derived from a type definition (call this **B**, for base) given a
subset of {*extension*, *restriction*, *list*, *union*} (of which
only *restriction* is actually relevant)
**one** of the following must be true:

1

<a id="c-stid"></a>

They are the same type
definition.

2 **All** of the following must be true:

2.1 *restriction* is not in the
subset, or in the [{final}](045-3-14-simple-type-definitions.md#st-final) of its own [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition);

2.2 **One** of the following must be true:

2.2.1 **D**'s [·base type definition·](045-3-14-simple-type-definitions.md#st-base_type_definition) is **B**.

2.2.2 **D**'s [·base type definition·](045-3-14-simple-type-definitions.md#st-base_type_definition) is not the
[·ur-type definition·](029-3-4-7-built-in-complex-type-definition.md#ur-type-itself) and is validly derived
from **B** given the subset, as defined by this constraint.

2.2.3 **D**'s [{variety}](045-3-14-simple-type-definitions.md#variety) is *list* or *union* and **B**
is the [·simple ur-type definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself).

2.2.4 **B**'s [{variety}](045-3-14-simple-type-definitions.md#variety) is *union* and
**D** is validly derived
from a type definition in **B**'s [{member type definitions}](045-3-14-simple-type-definitions.md#st-member_type_definitions) given the subset, as defined by this constraint.

**Note:**With respect to clause [1](048-3-14-7-built-in-simple-type-definition.md#c-stid), see the Note on identity at
the end of  [(§3.4.6)](029-3-4-7-built-in-complex-type-definition.md#no-identity) above.

<a id="st-restrict-facets"></a>

**Schema Component Constraint: Simple Type Restriction (Facets)**  

For a simple type definition (call it **R**) to restrict another simple type
definition (call it **B**) with a
set of facets (call this **S**)
**all** of the following must be true:

1 The [{variety}](045-3-14-simple-type-definitions.md#variety) of **R** is the same as that of **B**.

2 If [{variety}](045-3-14-simple-type-definitions.md#variety) is *atomic*, the
[{primitive type definition}](045-3-14-simple-type-definitions.md#st-primitive_type_definition) of **R** is the same as that of **B**.

3

<a id="c-fr"></a>

The [{facets}](045-3-14-simple-type-definitions.md#facets) of **R** are the union of **S** and
the [{facets}](045-3-14-simple-type-definitions.md#facets) of **B**, eliminating duplicates. To eliminate
duplicates, when a facet of the same kind occurs in both **S** and
the [{facets}](045-3-14-simple-type-definitions.md#facets) of **B**, the one in the [{facets}](045-3-14-simple-type-definitions.md#facets)
of **B** is not included, with the exception of [enumeration](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-enumeration) and [pattern](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#dt-pattern) facets, for which multiple occurrences with distinct values are allowed.Additional constraint(s) may apply depending on the kind of facet, see
the appropriate sub-section of [4.3
Constraining Facets](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#rf-facets)

<a id="key-facets-restriction"></a>

[Definition:]If
clause [3](048-3-14-7-built-in-simple-type-definition.md#c-fr) above holds, the [{facets}](045-3-14-simple-type-definitions.md#facets) of **R**
**constitute a restriction** of the [{facets}](045-3-14-simple-type-definitions.md#facets) of
**B** with respect to **S**.

<a id="d0e16395"></a>

#### 3.14.7 Built-in Simple Type Definition

There is a simple type definition nearly equivalent to the [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType) present in every
schema by definition. It has the following properties:

<a id="simple-ur-type-itself"></a>

<table border="1" class="schemaComp"><thead><tr><th><strong><a id="simple-ur-type-itself" name="simple-ur-type-itself">Simple Type Definition of the Ur-Type</a></strong></th></tr></thead><tbody><tr><td><table border="0" cellpadding="3"><thead><tr><th align="left">Property</th><th align="left">Value</th></tr></thead><tbody valign="top"><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-name">{name}</a></td><td>anySimpleType</td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-target_namespace">{target namespace}</a></td><td>http://www.w3.org/2001/XMLSchema</td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-base_type_definition">{base type definition}</a></td><td><a class="termref" href="029-3-4-7-built-in-complex-type-definition.md#ur-type-itself"><span class="arrow">·</span>the ur-type definition<span class="arrow">·</span></a></td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#st-final">{final}</a></td><td>The empty set</td></tr><tr><td><a class="propref" href="045-3-14-simple-type-definitions.md#variety">{variety}</a></td><td><a class="termref" href="008-3-schema-component-details.md#key-null"><span class="arrow">·</span>absent<span class="arrow">·</span></a></td></tr></tbody></table></td></tr></tbody></table>

The [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType) is the root of the simple type definition
hierarchy, and as such mediates between the other simple type
definitions, which all eventually trace back to it via their [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition) properties, and the [·ur-type definition·](004-2-1-overview-of-xml-schema.md#key-urType), which is
*its* [{base type definition}](045-3-14-simple-type-definitions.md#st-base_type_definition). This is
why the [·simple ur-type definition·](048-3-14-7-built-in-simple-type-definition.md#simple-ur-type-itself) is exempted
from the first clause of [Simple Type Definition Properties Correct (§3.14.6)](047-3-14-3-constraints-on-xml-representations-of-simple-type-definitions.md#st-props-correct), which would
otherwise bar it because of its derivation from a complex type definition and absence of [{variety}](045-3-14-simple-type-definitions.md#variety).

Simple type definitions for all the built-in primitive datatypes, namely *string*, *boolean*, *float*,
*double*, *number*, *dateTime*, *duration*,
*time*, *date*, *gMonth*, *gMonthDay*, *gDay*, *gYear*, *gYearMonth*, *hexBinary*, *base64Binary*, *anyURI* (see the [Primitive
Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#built-in-primitive-datatypes) section of [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2)) are present by definition in every schema. All
are in the XML Schema [{target namespace}](045-3-14-simple-type-definitions.md#st-target_namespace) (namespace
name `http://www.w3.org/2001/XMLSchema`), have an *atomic* [{variety}](045-3-14-simple-type-definitions.md#variety) with an empty
[{facets}](045-3-14-simple-type-definitions.md#facets) and the [·simple ur-type definition·](004-2-1-overview-of-xml-schema.md#key-simpleUrType) as
their [·base type definition·](045-3-14-simple-type-definitions.md#st-base_type_definition) and themselves as [{primitive type definition}](045-3-14-simple-type-definitions.md#st-primitive_type_definition).

Similarly, simple type definitions for all the built-in derived
datatypes (see the [Derived
Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/datatypes.html#built-in-derived) section of [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2)) are present by definition in every schema, with
properties as specified in [[XML Schemas: Datatypes]](060-b-references-normative.md#ref-xsp2) and as represented in XML in
[Schema for Schemas (normative) (§A)](058-5-3-missing-sub-components.md#normative-schemaSchema).

