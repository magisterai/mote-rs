# 4 Datatype components

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](017-3-3-21-unsignedlong.md) · [Next](019-4-1-2-1-derivation-by-restriction.md)

<a id="datatype-components"></a>

## 4 Datatype components

The following sections provide full details on the properties and
significance of each kind of schema component involved in datatype
definitions. For each property, the kinds of values it is allowed to have is
specified. Any property not identified as optional is required to
be present; optional properties which are not present have
[absent](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-null) as their value.
Any property identified as a having a set, subset or [·list·](004-2-3-lexical-space.md#dt-list)
value may have an empty value unless this is explicitly ruled out: this is
not the same as [absent](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-null).
Any property value identified as a superset or a subset of some set may
be equal to that set, unless a proper superset or subset is explicitly
called for.

For more information on the notion of datatype (schema) components,
see [Schema Component Details](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#components)
of [[XML Schema Part 1: Structures]](044-h-references.md#structural-schemas).

<a id="rf-defn"></a>

### 4.1 Simple Type Definition

4.1.1 [The Simple Type Definition Schema Component](018-4-datatype-components.md#dc-defn)  
        4.1.2 [XML Representation of Simple Type Definition Schema Components](018-4-datatype-components.md#xr-defn)  
        4.1.3 [Constraints on XML Representation of Simple Type Definition](020-4-1-2-3-derivation-by-union.md#defn-rep-constr)  
        4.1.4 [Simple Type Definition Validation Rules](020-4-1-2-3-derivation-by-union.md#defn-validation-rules)  
        4.1.5 [Constraints on Simple Type Definition Schema Components](020-4-1-2-3-derivation-by-union.md#defn-coss)  
        4.1.6 [Simple Type Definition for anySimpleType](022-4-1-6-simple-type-definition-for-anysimpletype.md#anySimpleType-component)

Simple Type definitions provide for:

- Establishing the [·value space·](003-1-4-terminology.md#dt-value-space) and [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)
  of a datatype, through
  the combined set of [·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s specified
  in the definition;
- Attaching a unique name (actually a [QName](012-3-2-15-hexbinary.md#QName)) to the
  [·value space·](003-1-4-terminology.md#dt-value-space) and [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space).

<a id="dc-defn"></a>

#### 4.1.1 The Simple Type Definition Schema Component

The Simple Type Definition schema component has the following properties:

Schema Component: [Simple Type Definition](003-1-4-terminology.md#datatype)

<a id="defn-name"></a>

{name}
:   Optional. An NCName as defined by
    [[Namespaces in XML]](044-h-references.md#XMLNS).

<a id="defn-target-namespace"></a>

{target namespace}
:   Either [absent](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-null) or a
    namespace name, as defined in [[Namespaces in XML]](044-h-references.md#XMLNS).

<a id="defn-variety"></a>

{variety}
:   One of {*atomic*, *list*, *union*}. Depending on the
    value of [{variety}](018-4-datatype-components.md#defn-variety), further properties are defined as follows:

    atomic
    :   <a id="defn-primitive"></a>

        {primitive type definition}
        :   A [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in) [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive)
            datatype definition).

    list
    :   <a id="defn-itemType"></a>

        {item type definition}
        :   An [·atomic·](004-2-3-lexical-space.md#dt-atomic) or [·union·](004-2-3-lexical-space.md#dt-union) simple type definition.

    union
    :   <a id="defn-memberTypes"></a>

        {member type definitions}
        :   A non-empty sequence of simple type definitions.

<a id="defn-facets"></a>

{facets}
:   A possibly empty set of [Facets (§2.4)](004-2-3-lexical-space.md#facets).

<a id="defn-fund-facets"></a>

{fundamental facets}
:   A set of [Fundamental facets (§2.4.1)](004-2-3-lexical-space.md#fundamental-facets)

<a id="defn-basetype"></a>

{base type definition}
:   If the datatype has been [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by
    [·restriction·](005-2-5-1-3-union-datatypes.md#dt-restriction) then the [Simple Type Definition](018-4-datatype-components.md#dc-defn) component
    from which it is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived), otherwise
    the [Simple Type Definition for anySimpleType (§4.1.6)](022-4-1-6-simple-type-definition-for-anysimpletype.md#anySimpleType-component).

<a id="defn-final"></a>

{final}
:   A subset of *{restriction, list, union}*.

<a id="defn-annotation"></a>

{annotation}
:   Optional. An [annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#Annotation).

Datatypes are identified by their [{name}](018-4-datatype-components.md#defn-name)
and [{target namespace}](018-4-datatype-components.md#defn-target-namespace). Except
for anonymous datatypes (those with no [{name}](018-4-datatype-components.md#defn-name)),
datatype definitions [·must·](003-1-4-terminology.md#dt-must) be uniquely identified
within a schema.

If [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic)
then the [·value space·](003-1-4-terminology.md#dt-value-space) of the datatype defined will
be a subset of the [·value space·](003-1-4-terminology.md#dt-value-space) of
[{base type definition}](018-4-datatype-components.md#defn-basetype) (which is a subset of the
[·value space·](003-1-4-terminology.md#dt-value-space) of [{primitive type definition}](018-4-datatype-components.md#defn-primitive)).
If [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list)
then the [·value space·](003-1-4-terminology.md#dt-value-space) of the datatype defined will
be the set of finite-length sequence of values from the
[·value space·](003-1-4-terminology.md#dt-value-space) of [{item type definition}](018-4-datatype-components.md#defn-itemType).
If [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union) then the
[·value space·](003-1-4-terminology.md#dt-value-space) of the datatype defined will be the
union of the [·value space·](003-1-4-terminology.md#dt-value-space)s of each datatype in
[{member type definitions}](018-4-datatype-components.md#defn-memberTypes).

If [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic)
then the [{variety}](018-4-datatype-components.md#defn-variety) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
must be [·atomic·](004-2-3-lexical-space.md#dt-atomic).
If [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list)
then the [{variety}](018-4-datatype-components.md#defn-variety) of [{item type definition}](018-4-datatype-components.md#defn-itemType)
must be either [·atomic·](004-2-3-lexical-space.md#dt-atomic) or [·union·](004-2-3-lexical-space.md#dt-union).
If [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union)
then
[{member type definitions}](018-4-datatype-components.md#defn-memberTypes) must be a list of datatype definitions.

The value of [{facets}](018-4-datatype-components.md#defn-facets) consists of the set of
[·facet·](004-2-3-lexical-space.md#dt-facet)s specified directly in the datatype definition
unioned with the possibly empty set of [{facets}](018-4-datatype-components.md#defn-facets) of
[{base type definition}](018-4-datatype-components.md#defn-basetype).

The value of [{fundamental facets}](018-4-datatype-components.md#defn-fund-facets) consists of the set of
[·fundamental facet·](004-2-3-lexical-space.md#dt-fundamental-facet)s and their values.

If [{final}](018-4-datatype-components.md#defn-final) is the empty set then the type can be used
in deriving other types; the explicit values *restriction*,
*list* and *union* prevent further derivations
by [·restriction·](005-2-5-1-3-union-datatypes.md#dt-restriction), [·list·](004-2-3-lexical-space.md#dt-list) and
[·union·](004-2-3-lexical-space.md#dt-union) respectively.

<a id="xr-defn"></a>

#### 4.1.2 XML Representation of Simple Type Definition Schema Components

The XML representation for a [Simple Type Definition](018-4-datatype-components.md#dc-defn) schema component
is a [<simpleType>](018-4-datatype-components.md#element-simpleType) element information item. The
correspondences between the properties of the information item and
properties of the component are as follows:

XML Representation Summary: `simpleType` Element Information Item

<a id="element-simpleType"></a>

<simpleType  
  final =
(#all | List of (list | union | restriction))
  
  id = [ID](014-3-3-4-1-constraining-facets.md#ID)  
  **name** = [NCName](014-3-3-4-1-constraining-facets.md#NCName)  
  *{any attributes with non-schema namespace . . .}*>  
*Content:* ([annotation](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#element-annotation)?, ([restriction](019-4-1-2-1-derivation-by-restriction.md#element-restriction) | [list](019-4-1-2-1-derivation-by-restriction.md#element-list) | [union](020-4-1-2-3-derivation-by-union.md#element-union)))  
</simpleType>

