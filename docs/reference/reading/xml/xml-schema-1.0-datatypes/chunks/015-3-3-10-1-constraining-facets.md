# 3.3.10.1 Constraining facets

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-3-3-4-1-constraining-facets.md) · [Next](016-3-3-16-long.md)

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology)) **IDREFS**
should be used only on attributes.

<a id="IDREFS-facets"></a>

##### 3.3.10.1 Constraining facets

**IDREFS** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)

<a id="ENTITY"></a>

#### 3.3.11 ENTITY

<a id="dt-ENTITY"></a>

[Definition:]
**ENTITY** represents the
[ENTITY](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-TokenizedType) attribute type from
[[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space) of
**ENTITY** is the set of all strings that [·match·](003-1-4-terminology.md#dt-match)
the [NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production in
[[Namespaces in XML]](044-h-references.md#XMLNS) and have been declared as an
[unparsed entity](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-unparsed) in
a [document type definition](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-doctype).
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **ENTITY** is the set
of all strings that [·match·](003-1-4-terminology.md#dt-match) the
[NCName](https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName) production in
[[Namespaces in XML]](044-h-references.md#XMLNS).
The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **ENTITY** is [NCName](014-3-3-4-1-constraining-facets.md#NCName).

**Note:**
The [·value space·](003-1-4-terminology.md#dt-value-space) of **ENTITY** is scoped
to a specific instance document.

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology)) **ENTITY**
should be used only on attributes.

<a id="ENTITY-facets"></a>

##### 3.3.11.1 Constraining facets

**ENTITY** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)

<a id="ENTITY-derived-types"></a>

##### 3.3.11.2 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**ENTITY**:

- [ENTITIES](015-3-3-10-1-constraining-facets.md#ENTITIES)

<a id="ENTITIES"></a>

#### 3.3.12 ENTITIES

<a id="dt-ENTITIES"></a>

[Definition:]
**ENTITIES**
represents the [ENTITIES attribute
type](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-TokenizedType) from [[XML 1.0 (Second Edition)]](044-h-references.md#XML). The [·value space·](003-1-4-terminology.md#dt-value-space)
of **ENTITIES** is the set of finite, non-zero-length sequences of
[·ENTITY·](015-3-3-10-1-constraining-facets.md#dt-ENTITY)s that have been declared as
[unparsed entities](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-unparsed)
in a [document type definition](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-doctype).
The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **ENTITIES** is the
set of space-separated lists of tokens, of which each token is in the
[·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of [ENTITY](015-3-3-10-1-constraining-facets.md#ENTITY).
The [·itemType·](004-2-3-lexical-space.md#dt-itemType) of **ENTITIES** is
[ENTITY](015-3-3-10-1-constraining-facets.md#ENTITY).

**Note:**
The [·value space·](003-1-4-terminology.md#dt-value-space) of **ENTITIES** is scoped
to a specific instance document.

For compatibility (see [Terminology (§1.4)](003-1-4-terminology.md#terminology)) **ENTITIES**
should be used only on attributes.

<a id="ENTITIES-facets"></a>

##### 3.3.12.1 Constraining facets

**ENTITIES** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [length](024-4-2-5-numeric.md#rf-length)
- [minLength](025-4-3-1-4-constraints-on-length-schema-components.md#rf-minLength)
- [maxLength](026-4-3-3-maxlength.md#rf-maxLength)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)

<a id="integer"></a>

#### 3.3.13 integer

<a id="dt-integer"></a>

[Definition:]
**integer** is
[·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [decimal](006-2-5-2-3-derived-by-union.md#decimal) by fixing the
value of [·fractionDigits·](032-4-3-11-3-totaldigits-validation-rules.md#dt-fractionDigits) to be 0and
disallowing the trailing decimal point.
This results in the standard
mathematical concept of the integer numbers. The
[·value space·](003-1-4-terminology.md#dt-value-space) of **integer** is the infinite
set {...,-2,-1,0,1,2,...}. The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of
**integer** is [decimal](006-2-5-2-3-derived-by-union.md#decimal).

<a id="integer-lexical-representation"></a>

##### 3.3.13.1 Lexical representation

**integer** has a lexical representation consisting of a finite-length sequence
of decimal digits (#x30-#x39) with an optional leading sign. If the sign is omitted,
"+" is assumed. For example: -1, 0, 12678967543233, +100000.

<a id="integer-canonical-repr"></a>

##### 3.3.13.2 Canonical representation

The canonical representation for **integer** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.13.1)](015-3-3-10-1-constraining-facets.md#integer-lexical-representation). Specifically, the preceding optional "+" sign is prohibited and leading zeroes are prohibited.

<a id="integer-facets"></a>

##### 3.3.13.3 Constraining facets

**integer** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [totalDigits](031-4-3-10-mininclusive.md#rf-totalDigits)
- [fractionDigits](032-4-3-11-3-totaldigits-validation-rules.md#rf-fractionDigits)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="integer-derived-types"></a>

##### 3.3.13.4 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**integer**:

- [nonPositiveInteger](015-3-3-10-1-constraining-facets.md#nonPositiveInteger)
- [long](016-3-3-16-long.md#long)
- [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger)

<a id="nonPositiveInteger"></a>

#### 3.3.14 nonPositiveInteger

<a id="dt-nonPositiveInteger"></a>

[Definition:]
**nonPositiveInteger** is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
[integer](015-3-3-10-1-constraining-facets.md#integer) by setting the value of
[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) to be 0. This results in the
standard mathematical concept of the non-positive integers.
The [·value space·](003-1-4-terminology.md#dt-value-space) of **nonPositiveInteger**
is the infinite set {...,-2,-1,0}. The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype)
of **nonPositiveInteger** is [integer](015-3-3-10-1-constraining-facets.md#integer).

<a id="nonPositiveInteger-lexical-representation"></a>

##### 3.3.14.1 Lexical representation

**nonPositiveInteger** has a lexical representation consisting of
an optional preceding sign
followed by a finite-length sequence of decimal digits (#x30-#x39).
The sign may be "+" or may be omitted only for
lexical forms denoting zero; in all other lexical forms, the negative
sign ("-") must be present.
For example: -1, 0, -12678967543233, -100000.

<a id="nonPositiveInteger-canonical-repr"></a>

##### 3.3.14.2 Canonical representation

The canonical representation for **nonPositiveInteger** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.14.1)](015-3-3-10-1-constraining-facets.md#nonPositiveInteger-lexical-representation).
In the canonical form for zero, the sign must be
omitted. Leading zeroes are prohibited.

<a id="nonPositiveInteger-facets"></a>

##### 3.3.14.3 Constraining facets

**nonPositiveInteger** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [totalDigits](031-4-3-10-mininclusive.md#rf-totalDigits)
- [fractionDigits](032-4-3-11-3-totaldigits-validation-rules.md#rf-fractionDigits)
- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="nonPositiveInteger-derived-types"></a>

##### 3.3.14.4 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**nonPositiveInteger**:

- [negativeInteger](015-3-3-10-1-constraining-facets.md#negativeInteger)

<a id="negativeInteger"></a>

#### 3.3.15 negativeInteger

<a id="dt-negativeInteger"></a>

[Definition:]
**negativeInteger** is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
[nonPositiveInteger](015-3-3-10-1-constraining-facets.md#nonPositiveInteger) by setting the value of
[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) to be -1. This results in the
standard mathematical concept of the negative integers. The
[·value space·](003-1-4-terminology.md#dt-value-space) of **negativeInteger**
is the infinite set {...,-2,-1}. The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype)
of **negativeInteger** is [nonPositiveInteger](015-3-3-10-1-constraining-facets.md#nonPositiveInteger).

<a id="negativeInteger-lexical-representation"></a>

##### 3.3.15.1 Lexical representation

**negativeInteger** has a lexical representation consisting of
a negative sign ("-") followed by a finite-length
sequence of decimal digits (#x30-#x39). For example: -1, -12678967543233, -100000.

<a id="negativeInteger-canonical-repr"></a>

##### 3.3.15.2 Canonical representation

The canonical representation for **negativeInteger** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.15.1)](015-3-3-10-1-constraining-facets.md#negativeInteger-lexical-representation). Specifically, leading zeroes are prohibited.

<a id="negativeInteger-facets"></a>

##### 3.3.15.3 Constraining facets

**negativeInteger** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

