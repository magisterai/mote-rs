# 4.2.3 bounded

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](022-4-1-6-simple-type-definition-for-anysimpletype.md) · [Next](024-4-2-5-numeric.md)

When [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic),
[{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value) is inherited from
[{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value) of [{base type definition}](018-4-datatype-components.md#defn-basetype).
For all [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) types [{value}](024-4-2-5-numeric.md#numeric-value)
is as specified in the table in [Fundamental Facets (§C.1)](035-b-dtd-for-datatype-definitions-non-normative.md#app-fundamental-facets).

When [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list),
[{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value) is *false*.

When [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union),
[{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value) is *partial* unless one of the
following:

- If every member of [{member type definitions}](018-4-datatype-components.md#defn-memberTypes) is derived from
  a common ancestor other than the simple ur-type, then [{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value)
  is the same as that ancestor's **ordered** facet
- If every member of [{member type definitions}](018-4-datatype-components.md#defn-memberTypes) has a
  [{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value) of *false* for the **ordered**
  facet, then [{value}](022-4-1-6-simple-type-definition-for-anysimpletype.md#ordered-value) is *false*

<a id="rf-bounded"></a>

#### 4.2.3 bounded

<a id="dt-inclusive-upper-bound"></a>

[Definition:]
A value *u* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
 [·value space·](003-1-4-terminology.md#dt-value-space) *U*
is said to be an **inclusive upper bound** of a
[·value space·](003-1-4-terminology.md#dt-value-space) *V*
(where *V* is a subset of *U*)
if for all *v* in *V*,
*u* >= *v*.

<a id="dt-exclusive-upper-bound"></a>

[Definition:]
A value *u* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
 [·value space·](003-1-4-terminology.md#dt-value-space) *U*
is said to be an **exclusive upper bound** of a
[·value space·](003-1-4-terminology.md#dt-value-space) *V*
(where *V* is a subset of *U*)
if for all *v* in *V*,
*u* > *v*.

<a id="dt-inclusive-lower-bound"></a>

[Definition:]
A value *l* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
 [·value space·](003-1-4-terminology.md#dt-value-space) *L*
is said to be an **inclusive lower bound** of a
[·value space·](003-1-4-terminology.md#dt-value-space) *V*
(where *V* is a subset of *L*)
if for all *v* in *V*,
*l* <= *v*.

<a id="dt-exclusive-lower-bound"></a>

[Definition:]
A value *l* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
 [·value space·](003-1-4-terminology.md#dt-value-space) *L*
is said to be an **exclusive lower bound** of a
[·value space·](003-1-4-terminology.md#dt-value-space) *V*
(where *V* is a subset of *L*)
if for all *v* in *V*,
*l* < *v*.

<a id="dt-bounded"></a>

[Definition:]A datatype is **bounded**
if its [·value space·](003-1-4-terminology.md#dt-value-space) has either an
[·inclusive upper bound·](023-4-2-3-bounded.md#dt-inclusive-upper-bound) or an [·exclusive upper bound·](023-4-2-3-bounded.md#dt-exclusive-upper-bound)
and either an [·inclusive lower bound·](023-4-2-3-bounded.md#dt-inclusive-lower-bound) or
an
[·exclusive lower bound·](023-4-2-3-bounded.md#dt-exclusive-lower-bound).

[·bounded·](023-4-2-3-bounded.md#dt-bounded) provides for:

- indicating whether a [·value space·](003-1-4-terminology.md#dt-value-space) is
  [·bounded·](023-4-2-3-bounded.md#dt-bounded)

<a id="dc-bounded"></a>

##### 4.2.3.1 The bounded Schema Component

Schema Component: [bounded](023-4-2-3-bounded.md#dt-bounded)

<a id="bounded-value"></a>

{value}
:   A [boolean](006-2-5-2-3-derived-by-union.md#boolean).

[{value}](023-4-2-3-bounded.md#bounded-value) depends on [{variety}](018-4-datatype-components.md#defn-variety),
[{facets}](018-4-datatype-components.md#defn-facets) and [{member type definitions}](018-4-datatype-components.md#defn-memberTypes)
in the [Simple Type Definition](018-4-datatype-components.md#dc-defn) component in which a
[·bounded·](023-4-2-3-bounded.md#dt-bounded) component appears as a member of
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets).

When [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic),
if one of [·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) or [·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive)
and one of [·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) or [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive)
are among [{facets}](018-4-datatype-components.md#defn-facets) , then
[{value}](023-4-2-3-bounded.md#bounded-value) is *true*; else
[{value}](023-4-2-3-bounded.md#bounded-value) is *false*.

When [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list),
if [·length·](024-4-2-5-numeric.md#dt-length) or both of
[·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength) and [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
are among [{facets}](018-4-datatype-components.md#defn-facets), then
[{value}](023-4-2-3-bounded.md#bounded-value) is *true*; else
[{value}](023-4-2-3-bounded.md#bounded-value) is *false*.

When [{variety}](018-4-datatype-components.md#defn-variety) is [·union·](004-2-3-lexical-space.md#dt-union),
if [{value}](023-4-2-3-bounded.md#bounded-value) is *true*
for every member of [{member type definitions}](018-4-datatype-components.md#defn-memberTypes)
and all members of [{member type definitions}](018-4-datatype-components.md#defn-memberTypes) share a common
ancestor, then [{value}](023-4-2-3-bounded.md#bounded-value) is *true*;
else [{value}](023-4-2-3-bounded.md#bounded-value) is *false*.

<a id="rf-cardinality"></a>

#### 4.2.4 cardinality

<a id="dt-cardinality"></a>

[Definition:]Every
[·value space·](003-1-4-terminology.md#dt-value-space) has associated with it the concept of
**cardinality**. Some [·value space·](003-1-4-terminology.md#dt-value-space)s
are finite, some are countably infinite while still others could
conceivably be uncountably infinite (although no [·value space·](003-1-4-terminology.md#dt-value-space)
defined by this specification is uncountable infinite). A datatype is
said to have the cardinality of its
[·value space·](003-1-4-terminology.md#dt-value-space).

It
is sometimes useful to categorize [·value space·](003-1-4-terminology.md#dt-value-space)s
(and hence, datatypes) as to their cardinality. There are two
significant cases:

- [·value space·](003-1-4-terminology.md#dt-value-space)s that are finite
- [·value space·](003-1-4-terminology.md#dt-value-space)s that are countably infinite

[·cardinality·](023-4-2-3-bounded.md#dt-cardinality) provides for:

- indicating whether the [·cardinality·](023-4-2-3-bounded.md#dt-cardinality)
  of a [·value space·](003-1-4-terminology.md#dt-value-space) is
  *finite* or *countably infinite*

<a id="dc-cardinality"></a>

##### 4.2.4.1 The cardinality Schema Component

Schema Component: [cardinality](023-4-2-3-bounded.md#dt-cardinality)

<a id="cardinality-value"></a>

{value}
:   One of *{finite, countably infinite}*.

[{value}](023-4-2-3-bounded.md#cardinality-value) depends on [{variety}](018-4-datatype-components.md#defn-variety),
[{facets}](018-4-datatype-components.md#defn-facets) and [{member type definitions}](018-4-datatype-components.md#defn-memberTypes)
in the [Simple Type Definition](018-4-datatype-components.md#dc-defn) component in which a
[·cardinality·](023-4-2-3-bounded.md#dt-cardinality) component appears as a member of
[{fundamental facets}](018-4-datatype-components.md#defn-fund-facets).

When [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic) and
[{value}](023-4-2-3-bounded.md#cardinality-value) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
is *finite*, then [{value}](023-4-2-3-bounded.md#cardinality-value) is
*finite*.

When [{variety}](018-4-datatype-components.md#defn-variety) is [·atomic·](004-2-3-lexical-space.md#dt-atomic) and
[{value}](023-4-2-3-bounded.md#cardinality-value) of [{base type definition}](018-4-datatype-components.md#defn-basetype)
is *countably infinite* and **either** of the following
conditions are true, then [{value}](023-4-2-3-bounded.md#cardinality-value) is
*finite*; else [{value}](023-4-2-3-bounded.md#cardinality-value)
is *countably infinite*:

1. one of [·length·](024-4-2-5-numeric.md#dt-length), [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength),
   [·totalDigits·](031-4-3-10-mininclusive.md#dt-totalDigits) is among [{facets}](018-4-datatype-components.md#defn-facets),
2. **all** of the following are true:
   1. one of [·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) or
      [·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive)
      is among [{facets}](018-4-datatype-components.md#defn-facets)
   2. one of [·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) or
      [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive)
      is among [{facets}](018-4-datatype-components.md#defn-facets)
   3. **either** of the following are true:
      1. [·fractionDigits·](032-4-3-11-3-totaldigits-validation-rules.md#dt-fractionDigits) is among [{facets}](018-4-datatype-components.md#defn-facets)
      2. [{base type definition}](018-4-datatype-components.md#defn-basetype) is one of [date](010-3-2-7-5-totally-ordered-datetimes.md#date),
         [gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), [gYear](011-3-2-10-1-lexical-representation.md#gYear), [gMonthDay](011-3-2-10-1-lexical-representation.md#gMonthDay),
         [gDay](011-3-2-10-1-lexical-representation.md#gDay) or [gMonth](011-3-2-10-1-lexical-representation.md#gMonth) or any type [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
         from them

When [{variety}](018-4-datatype-components.md#defn-variety) is [·list·](004-2-3-lexical-space.md#dt-list),
if [·length·](024-4-2-5-numeric.md#dt-length) or both of
[·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength) and [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
are among [{facets}](018-4-datatype-components.md#defn-facets), then
[{value}](023-4-2-3-bounded.md#cardinality-value) is *finite*; else
[{value}](023-4-2-3-bounded.md#cardinality-value) is *countably infinite*.

