# 2.3 Lexical space

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-1-4-terminology.md) · [Next](005-2-5-1-3-union-datatypes.md)

<a id="lexical-space"></a>

### 2.3 Lexical space

In addition to its [·value space·](003-1-4-terminology.md#dt-value-space), each datatype also
has a lexical space.

<a id="dt-lexical-space"></a>

[Definition:]A
**lexical space** is the set of valid *literals*
for a datatype.

For example, "100" and "1.0E2" are two different literals from the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of [float](007-3-2-3-1-lexical-representation.md#float) which both
denote the same value. The type system defined in this specification
provides a mechanism for schema designers to control the set of values
and the corresponding set of acceptable literals of those values for
a datatype.

**Note:**
The literals in the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)s defined in this specification
have the following characteristics:

Interoperability:
:   The number of literals for each value has been kept small; for many
    datatypes there is a one-to-one mapping between literals and values.
    This makes it easy to exchange the values between different systems.
    In many cases, conversion from locale-dependent representations will
    be required on both the originator and the recipient side, both for
    computer processing and for interaction with humans.

Basic readability:
:   Textual, rather than binary, literals are used.
    This makes hand editing, debugging, and similar activities possible.

Ease of parsing and serializing:
:   Where possible, literals correspond to those found in common
    programming languages and libraries.

<a id="canonical-lexical-representation"></a>

#### 2.3.1 Canonical Lexical Representation

While the datatypes defined in this specification have, for the most part,
a single lexical representation i.e. each value in the datatype's
[·value space·](003-1-4-terminology.md#dt-value-space) is denoted by a single literal in its
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space), this is not always the case. The
example in the previous section showed two literals for the datatype
[float](007-3-2-3-1-lexical-representation.md#float) which denote the same value. Similarly, there
[·may·](003-1-4-terminology.md#dt-may) be
several literals for one of the date or time datatypes that denote the
same value using different timezone indicators.

<a id="dt-canonical-representation"></a>

[Definition:]A **canonical lexical representation**
is a set of literals from among the valid set of literals
for a datatype such that there is a one-to-one mapping between literals
in the **canonical lexical representation** and
values in the [·value space·](003-1-4-terminology.md#dt-value-space).

<a id="facets"></a>

### 2.4 Facets

2.4.1 [Fundamental facets](004-2-3-lexical-space.md#fundamental-facets)  
        2.4.2 [Constraining or Non-fundamental facets](004-2-3-lexical-space.md#non-fundamental)

<a id="dt-facet"></a>

[Definition:]A **facet** is a single
defining aspect of a [·value space·](003-1-4-terminology.md#dt-value-space). Generally
speaking, each facet characterizes a [·value space·](003-1-4-terminology.md#dt-value-space)
along independent axes or dimensions.

The facets of a datatype serve to distinguish those aspects of
one datatype which *differ* from other datatypes.
Rather than being defined solely in terms of a prose description
the datatypes in this specification are defined in terms of
the *synthesis* of facet values which together determine the
[·value space·](003-1-4-terminology.md#dt-value-space) and properties of the datatype.

Facets are of two types: *fundamental* facets that define
the datatype and *non-fundamental* or *constraining* facets that constrain the permitted values of a datatype.

<a id="fundamental-facets"></a>

#### 2.4.1 Fundamental facets

<a id="dt-fundamental-facet"></a>

[Definition:]
A **fundamental facet** is an abstract property which
serves to semantically characterize the values in a
[·value space·](003-1-4-terminology.md#dt-value-space).

All **fundamental facets** are fully described in
[Fundamental Facets (§4.2)](022-4-1-6-simple-type-definition-for-anysimpletype.md#rf-fund-facets).

<a id="non-fundamental"></a>

#### 2.4.2 Constraining or Non-fundamental facets

<a id="dt-constraining-facet"></a>

[Definition:]A
**constraining facet** is an optional property that can be
applied to a datatype to constrain its [·value space·](003-1-4-terminology.md#dt-value-space).

Constraining the [·value space·](003-1-4-terminology.md#dt-value-space) consequently constrains
the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space). Adding
[·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s to a [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype)
is described in [Derivation by restriction (§4.1.2.1)](019-4-1-2-1-derivation-by-restriction.md#derivation-by-restriction).

All **constraining facets** are fully described in
[Constraining Facets (§4.3)](024-4-2-5-numeric.md#rf-facets).

<a id="datatype-dichotomies"></a>

### 2.5 Datatype dichotomies

2.5.1 [Atomic vs. list vs. union datatypes](004-2-3-lexical-space.md#atomic-vs-list)  
        2.5.2 [Primitive vs. derived datatypes](005-2-5-1-3-union-datatypes.md#primitive-vs-derived)  
        2.5.3 [Built-in vs. user-derived datatypes](006-2-5-2-3-derived-by-union.md#built-in-vs-user-derived)

It is useful to categorize the datatypes defined in this specification
along various dimensions, forming a set of characterization dichotomies.

<a id="atomic-vs-list"></a>

#### 2.5.1 Atomic vs. list vs. union datatypes

The first distinction to be made is that between
[·atomic·](004-2-3-lexical-space.md#dt-atomic), [·list·](004-2-3-lexical-space.md#dt-list) and [·union·](004-2-3-lexical-space.md#dt-union)
datatypes.

<a id="dt-atomic"></a>

- [Definition:]**Atomic** datatypes
  are those having values which are regarded by this specification as
  being indivisible.

<a id="dt-list"></a>

- [Definition:]**List**
  datatypes are those having values each of which consists of a
  finite-length (possibly empty) sequence of values of an
  [·atomic·](004-2-3-lexical-space.md#dt-atomic) datatype.

<a id="dt-union"></a>

- [Definition:]**Union**
  datatypes are those whose [·value space·](003-1-4-terminology.md#dt-value-space)s and
  [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)s are the union of
  the [·value space·](003-1-4-terminology.md#dt-value-space)s and
  [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space)s of one or more other datatypes.

For example, a single token which [·match·](003-1-4-terminology.md#dt-match)es
[Nmtoken](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-Nmtoken) from
[[XML 1.0 (Second Edition)]](044-h-references.md#XML) could be the value of an [·atomic·](004-2-3-lexical-space.md#dt-atomic)
datatype ([NMTOKEN](013-3-2-19-notation.md#NMTOKEN)); while a sequence of such tokens
could be the value of a [·list·](004-2-3-lexical-space.md#dt-list) datatype
([NMTOKENS](014-3-3-4-1-constraining-facets.md#NMTOKENS)).

<a id="atomic"></a>

##### 2.5.1.1 Atomic datatypes

[·atomic·](004-2-3-lexical-space.md#dt-atomic) datatypes can be either
[·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) or [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived). The
[·value space·](003-1-4-terminology.md#dt-value-space) of an [·atomic·](004-2-3-lexical-space.md#dt-atomic) datatype
is a set of "atomic" values, which for the purposes of this specification,
are not further decomposable. The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of
an [·atomic·](004-2-3-lexical-space.md#dt-atomic) datatype is a set of *literals*
whose internal structure is specific to the datatype in question.

<a id="list-datatypes"></a>

##### 2.5.1.2 List datatypes

Several type systems (such as the one described in
[[ISO 11404]](044-h-references.md#ISO11404)) treat [·list·](004-2-3-lexical-space.md#dt-list) datatypes as
special cases of the more general notions of aggregate or collection
datatypes.

[·list·](004-2-3-lexical-space.md#dt-list) datatypes are always [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived).
The [·value space·](003-1-4-terminology.md#dt-value-space) of a [·list·](004-2-3-lexical-space.md#dt-list)
datatype is a set of finite-length sequences of [·atomic·](004-2-3-lexical-space.md#dt-atomic)
values. The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of a
[·list·](004-2-3-lexical-space.md#dt-list) datatype is a set of literals whose internal
structure is a space-separated
sequence of literals of the
[·atomic·](004-2-3-lexical-space.md#dt-atomic) datatype of the items in the
[·list·](004-2-3-lexical-space.md#dt-list).

<a id="dt-itemType"></a>

[Definition:]
The [·atomic·](004-2-3-lexical-space.md#dt-atomic) or [·union·](004-2-3-lexical-space.md#dt-union)
datatype that participates in the definition of a [·list·](004-2-3-lexical-space.md#dt-list) datatype
is known as the **itemType** of that [·list·](004-2-3-lexical-space.md#dt-list) datatype.

Example



```

<simpleType name='sizes'>
  <list itemType='decimal'/>
</simpleType>
```






```

<cerealSizes xsi:type='sizes'> 8 10.5 12 </cerealSizes>
```




A [·list·](004-2-3-lexical-space.md#dt-list) datatype can be [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived)
from an [·atomic·](004-2-3-lexical-space.md#dt-atomic) datatype whose
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) allows space
(such as [string](006-2-5-2-3-derived-by-union.md#string)
or [anyURI](012-3-2-15-hexbinary.md#anyURI))or a
[·union·](004-2-3-lexical-space.md#dt-union) datatype any of whose [{member type definitions}](018-4-datatype-components.md#defn-memberTypes)'s
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) allows space.
In such a case, regardless of the input, list items
will be separated at space boundaries.

Example



```

<simpleType name='listOfString'>
  <list itemType='string'/>
</simpleType>
```






```

<someElement xsi:type='listOfString'>
this is not list item 1
this is not list item 2
this is not list item 3
</someElement>
```




In the above example, the value of the *someElement* element
is not a [·list·](004-2-3-lexical-space.md#dt-list) of [·length·](024-4-2-5-numeric.md#dt-length) 3;
rather, it is a [·list·](004-2-3-lexical-space.md#dt-list) of [·length·](024-4-2-5-numeric.md#dt-length)
18.

When a datatype is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from a
[·list·](004-2-3-lexical-space.md#dt-list) datatype, the following
[·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s apply:

- [·length·](024-4-2-5-numeric.md#dt-length)
- [·maxLength·](026-4-3-3-maxlength.md#dt-maxLength)
- [·minLength·](025-4-3-1-4-constraints-on-length-schema-components.md#dt-minLength)
- [·enumeration·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-enumeration)
- [·pattern·](026-4-3-3-maxlength.md#dt-pattern)
- [·whiteSpace·](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#dt-whiteSpace)

