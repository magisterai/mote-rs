# 4.1.6 Simple Type Definition for anySimpleType

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](021-continuation-21.md) · [Next](023-4-2-3-bounded.md)

<a id="cos-list-of-atomic"></a>

**Schema Component Constraint: list of atomic**  

If [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list), then
the [{variety}](018-4-datatype-components.md#defn-variety) of [{item type definition}](018-4-datatype-components.md#defn-itemType)
 [·must·](003-1-4-terminology.md#dt-must) be [·atomic·](004-2-3-lexical-space.md#dt-atomic) or
[·union·](004-2-3-lexical-space.md#dt-union).

<a id="cos-no-circular-unions"></a>

**Schema Component Constraint: no circular unions**  

If [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union),
then
it is an [·error·](003-1-4-terminology.md#dt-error) if
[{name}](018-4-datatype-components.md#defn-name) and [{target namespace}](018-4-datatype-components.md#defn-target-namespace)
 [·match·](003-1-4-terminology.md#dt-match) [{name}](018-4-datatype-components.md#defn-name)
and [{target namespace}](018-4-datatype-components.md#defn-target-namespace) of any member of
[{member type definitions}](018-4-datatype-components.md#defn-memberTypes).

<a id="anySimpleType-component"></a>

#### 4.1.6 Simple Type Definition for anySimpleType

There is a simple type definition nearly equivalent to the simple version
of the [ur-type definition](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-urType) present
in every schema by definition. It has the following properties:

Schema Component: [anySimpleType](005-2-5-1-3-union-datatypes.md#dt-anySimpleType)

<a id="anySimpleType-name"></a>

{name}
:   anySimpleType

<a id="anySimpleType-target_namespace"></a>

{target namespace}
:   http://www.w3.org/2001/XMLSchema

<a id="anySimpleType-basetype"></a>

{basetype definition}
:   [the ur-type definition](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#ur-type-itself)

<a id="anySimpleType-final"></a>

{final}
:   the empty set

<a id="anySimpleType-variety"></a>

{variety}
:   [absent](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-null)

<a id="rf-fund-facets"></a>

### 4.2 Fundamental Facets

4.2.1 [equal](022-4-1-6-simple-type-definition-for-anysimpletype.md#equal)  
        4.2.2 [ordered](022-4-1-6-simple-type-definition-for-anysimpletype.md#rf-ordered)  
        4.2.3 [bounded](023-4-2-3-bounded.md#rf-bounded)  
        4.2.4 [cardinality](023-4-2-3-bounded.md#rf-cardinality)  
        4.2.5 [numeric](024-4-2-5-numeric.md#rf-numeric)

<a id="equal"></a>

#### 4.2.1 equal

Every [·value space·](003-1-4-terminology.md#dt-value-space) supports the notion of equality,
with the following rules:

- for any *a* and *b* in
  the [·value space·](003-1-4-terminology.md#dt-value-space),
  either *a* is equal to *b*,
  denoted *a = b*, or *a*
  is not equal to *b*, denoted *a != b*
- there is no pair *a* and *b*
  from the [·value space·](003-1-4-terminology.md#dt-value-space) such that both
  *a = b* and *a != b*
- for all *a* in the [·value space·](003-1-4-terminology.md#dt-value-space),
  *a = a*
- for any *a* and *b*
  in the [·value space·](003-1-4-terminology.md#dt-value-space),
  *a = b* if and only if *b = a*
- for any *a*, *b* and
  *c* in the [·value space·](003-1-4-terminology.md#dt-value-space),
  if *a = b* and
  *b = c*, then *a = c*
- for any *a* and *b*
  in the [·value space·](003-1-4-terminology.md#dt-value-space)
  if *a = b*, then *a*
  and *b* cannot be distinguished
  (i.e., equality is identity)
- the [·value space·](003-1-4-terminology.md#dt-value-space)s of all
  [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) datatypes are disjoint (they do not
  share any values)

On every datatype, the operation Equal is defined in terms of the equality
property of the [·value space·](003-1-4-terminology.md#dt-value-space): for any values
*a, b* drawn from the
[·value space·](003-1-4-terminology.md#dt-value-space), *Equal(a,b)* is
true if *a = b*, and false otherwise.

Note that in consequence of the above:

- given [·value space·](003-1-4-terminology.md#dt-value-space) *A* and
  [·value space·](003-1-4-terminology.md#dt-value-space) *B* where
  *A* and *B* are disjoint,
  every pair of values *a* from *A*
  and *b* from *B*,
  *a != b*
- two values which are members of the [·value space·](003-1-4-terminology.md#dt-value-space)
  of the same [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) datatype may always be
  compared with each other
- if a datatype *T* is
  [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by [·union·](004-2-3-lexical-space.md#dt-union) from
  [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes) *A, B, ...*
  then the [·value space·](003-1-4-terminology.md#dt-value-space) of *T* is the
  union of [·value space·](003-1-4-terminology.md#dt-value-space)s of its
  [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes) *A, B, ...*.
  Some values in the [·value space·](003-1-4-terminology.md#dt-value-space) of
  *T* are also values in the
  [·value space·](003-1-4-terminology.md#dt-value-space) of *A*.
  Other values in the [·value space·](003-1-4-terminology.md#dt-value-space) of
  *T* will be values in the
  [·value space·](003-1-4-terminology.md#dt-value-space) of *B* and so on.
  Values in the [·value space·](003-1-4-terminology.md#dt-value-space) of *T*
  which are also in the [·value space·](003-1-4-terminology.md#dt-value-space) of
  *A* can be compared with other values in the
  [·value space·](003-1-4-terminology.md#dt-value-space) of *A* according
  to the above rules. Similarly for values of type
  *T* and *B* and all the other
  [·memberTypes·](005-2-5-1-3-union-datatypes.md#dt-memberTypes).
- if a datatype *T'* is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
  by [·restriction·](005-2-5-1-3-union-datatypes.md#dt-restriction) from an atomic datatype *T*
  then the [·value space·](003-1-4-terminology.md#dt-value-space) of *T'* is
  a subset of the [·value space·](003-1-4-terminology.md#dt-value-space) of *T*.
  Values in the [·value space·](003-1-4-terminology.md#dt-value-space)s of
  *T* and *T'* can be compared
  according to the above rules
- if datatypes *T'* and *T''* are
  [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by [·restriction·](005-2-5-1-3-union-datatypes.md#dt-restriction) from a
  common atomic ancestor *T* then the
  [·value space·](003-1-4-terminology.md#dt-value-space)s of *T'* and
  *T''* may overlap. Values in the
  [·value space·](003-1-4-terminology.md#dt-value-space)s
  of *T'* and *T''* can be
  compared according to the above rules

**Note:**
There is no schema component corresponding to the **equal**
[·fundamental facet·](004-2-3-lexical-space.md#dt-fundamental-facet).

<a id="rf-ordered"></a>

#### 4.2.2 ordered

<a id="dt-order-relation"></a>

[Definition:]An
**order relation** on a [·value space·](003-1-4-terminology.md#dt-value-space)
is a mathematical relation that imposes a
[·total order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-total-order) or a [·partial order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-partial-order) on the
members of the [·value space·](003-1-4-terminology.md#dt-value-space).

<a id="dt-ordered"></a>

[Definition:]A
[·value space·](003-1-4-terminology.md#dt-value-space), and hence a datatype, is said to be
**ordered** if there exists an
[·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) defined for that
[·value space·](003-1-4-terminology.md#dt-value-space).

<a id="dt-partial-order"></a>

[Definition:]
A **partial order** is an [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation)
that is **irreflexive**, **asymmetric** and
**transitive**.

A [·partial order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-partial-order) has the following properties:

- for no *a* in the [·value space·](003-1-4-terminology.md#dt-value-space),
  *a < a*
  (irreflexivity)
- for all *a* and *b*
  in the [·value space·](003-1-4-terminology.md#dt-value-space),
  *a < b*
  implies not(*b < a*)
  (asymmetry)
- for all *a*, *b*
  and *c* in the [·value space·](003-1-4-terminology.md#dt-value-space),
  *a < b* and *b < c*
  implies *a < c*
  (transitivity)

The notation *a <> b* is used to indicate the
case when *a != b* and neither
*a < b* nor *b < a*.
For any values *a* and *b*
from different [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) [·value space·](003-1-4-terminology.md#dt-value-space)s,
*a <> b*.

<a id="dt-incomparable"></a>

<a id="dt-comparable"></a>

[Definition:]When *a <> b*, *a* and *b* are **incomparable**,[Definition:]otherwise they are **comparable**.

<a id="dt-total-order"></a>

[Definition:]
A **total order** is an [·partial order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-partial-order)
such that for no *a* and *b*
is it the case that *a <> b*.

A [·total order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-total-order) has all of the properties specified
above for [·partial order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-partial-order), plus
the following property:

- for all *a* and *b*
  in the [·value space·](003-1-4-terminology.md#dt-value-space),
  either *a < b* or *b < a*
  or *a = b*

**Note:**
The fact that this specification does not define an
[·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) for some datatype does not
mean that some other application cannot treat that datatype as
being ordered by imposing its own order relation.

[·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) provides for:

- indicating whether an [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) is
  defined on a [·value space·](003-1-4-terminology.md#dt-value-space), and if so,
  whether that [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) is
  a [·partial order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-partial-order) or a [·total order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-total-order)

<a id="dc-ordered"></a>

##### 4.2.2.1 The ordered Schema Component

Schema Component: [ordered](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)

<a id="ordered-value"></a>

{value}
:   One of *{false, partial, total}*.

[{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value) depends on [{variety}](018-4-datatype-components.md#defn-variety),
[{facets}](018-4-datatype-components.md#defn-facets) and [{member type definitions}](018-4-datatype-components.md#defn-memberTypes)
in the [Simple Type Definition](018-4-datatype-components.md#dc-defn) component in which a
[·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered) component appears as a member of
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets).

