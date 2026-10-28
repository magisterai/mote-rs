# 3.3.21 unsignedLong

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-3-3-16-long.md) · [Next](018-4-datatype-components.md)

- [unsignedLong](017-3-3-21-unsignedlong.md#unsignedLong)
- [positiveInteger](017-3-3-21-unsignedlong.md#positiveInteger)

<a id="unsignedLong"></a>

#### 3.3.21 unsignedLong

<a id="dt-unsignedLong"></a>

[Definition:]
**unsignedLong** is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
[nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger) by setting the value of
[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) to be 18446744073709551615.
The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **unsignedLong** is
[nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

<a id="unsignedLong-lexical-representation"></a>

##### 3.3.21.1 Lexical representation

**unsignedLong** has a lexical representation consisting
of a finite-length sequence of decimal digits (#x30-#x39).
For example: 0,
12678967543233, 100000.

<a id="unsignedLong-canonical-repr"></a>

##### 3.3.21.2 Canonical representation

The canonical representation for **unsignedLong** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.21.1)](017-3-3-21-unsignedlong.md#unsignedLong-lexical-representation). Specifically,
leading zeroes are prohibited.

<a id="unsignedLong-facets"></a>

##### 3.3.21.3 Constraining facets

**unsignedLong** has the
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

<a id="unsignedLong-derived-types"></a>

##### 3.3.21.4 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**unsignedLong**:

- [unsignedInt](017-3-3-21-unsignedlong.md#unsignedInt)

<a id="unsignedInt"></a>

#### 3.3.22 unsignedInt

<a id="dt-unsignedInt"></a>

[Definition:]
**unsignedInt** is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
[unsignedLong](017-3-3-21-unsignedlong.md#unsignedLong) by setting the value of
[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) to be 4294967295. The
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **unsignedInt** is
[unsignedLong](017-3-3-21-unsignedlong.md#unsignedLong).

<a id="unsignedInt-lexical-representation"></a>

##### 3.3.22.1 Lexical representation

**unsignedInt** has a lexical representation consisting
of a finite-length
sequence of decimal digits (#x30-#x39). For example: 0,
1267896754, 100000.

<a id="unsignedInt-canonical-repr"></a>

##### 3.3.22.2 Canonical representation

The canonical representation for **unsignedInt** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.22.1)](017-3-3-21-unsignedlong.md#unsignedInt-lexical-representation). Specifically,
leading zeroes are prohibited.

<a id="unsignedInt-facets"></a>

##### 3.3.22.3 Constraining facets

**unsignedInt** has the
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

<a id="unsignedInt-derived-types"></a>

##### 3.3.22.4 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**unsignedInt**:

- [unsignedShort](017-3-3-21-unsignedlong.md#unsignedShort)

<a id="unsignedShort"></a>

#### 3.3.23 unsignedShort

<a id="dt-unsignedShort"></a>

[Definition:]
**unsignedShort** is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
[unsignedInt](017-3-3-21-unsignedlong.md#unsignedInt) by setting the value of
[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) to be 65535. The
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **unsignedShort** is
[unsignedInt](017-3-3-21-unsignedlong.md#unsignedInt).

<a id="unsignedShort-lexical-representation"></a>

##### 3.3.23.1 Lexical representation

**unsignedShort** has a lexical representation consisting
of a finite-length
sequence of decimal digits (#x30-#x39).
For example: 0,
12678, 10000.

<a id="unsignedShort-canonical-repr"></a>

##### 3.3.23.2 Canonical representation

The canonical representation for **unsignedShort** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.23.1)](017-3-3-21-unsignedlong.md#unsignedShort-lexical-representation). Specifically, the
leading zeroes are prohibited.

<a id="unsingedShort-facets"></a>

##### 3.3.23.3 Constraining facets

**unsignedShort** has the
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

<a id="unsignedShort-derived-types"></a>

##### 3.3.23.4 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**unsignedShort**:

- [unsignedByte](017-3-3-21-unsignedlong.md#unsignedByte)

<a id="unsignedByte"></a>

#### 3.3.24 unsignedByte

<a id="dt-unsignedByte"></a>

[Definition:]
**unsignedByte** is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
[unsignedShort](017-3-3-21-unsignedlong.md#unsignedShort) by setting the value of
[·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive) to be 255. The
[·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of **unsignedByte** is
[unsignedShort](017-3-3-21-unsignedlong.md#unsignedShort).

<a id="unsignedByte-lexical-representation"></a>

##### 3.3.24.1 Lexical representation

**unsignedByte** has a lexical representation consisting
of a finite-length
sequence of decimal digits (#x30-#x39).
For example: 0,
126, 100.

<a id="unsignedByte-canonical-repr"></a>

##### 3.3.24.2 Canonical representation

The canonical representation for **unsignedByte** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.24.1)](017-3-3-21-unsignedlong.md#unsignedByte-lexical-representation). Specifically,
leading zeroes are prohibited.

<a id="unisngedByte-facets"></a>

##### 3.3.24.3 Constraining facets

**unsignedByte** has the
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

<a id="positiveInteger"></a>

#### 3.3.25 positiveInteger

<a id="dt-positiveInteger"></a>

[Definition:]
**positiveInteger** is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
[nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger) by setting the value of
[·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive) to be 1. This results in the standard
mathematical concept of the positive integer numbers.
The [·value space·](003-1-4-terminology.md#dt-value-space) of **positiveInteger**
is the infinite set {1,2,...}. The [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype) of
**positiveInteger** is [nonNegativeInteger](016-3-3-16-long.md#nonNegativeInteger).

<a id="positiveInteger-lexical-representation"></a>

##### 3.3.25.1 Lexical representation

**positiveInteger** has a lexical representation consisting
of an optional positive sign ("+") followed by a finite-length
sequence of decimal digits (#x30-#x39).
For example: 1, 12678967543233, +100000.

<a id="positiveInteger-canonical-repr"></a>

##### 3.3.25.2 Canonical representation

The canonical representation for **positiveInteger** is defined
by prohibiting certain options from the
[Lexical representation (§3.3.25.1)](017-3-3-21-unsignedlong.md#positiveInteger-lexical-representation). Specifically, the
optional "+" sign is prohibited and leading zeroes are prohibited.

<a id="positiveInteger-facets"></a>

##### 3.3.25.3 Constraining facets

**positiveInteger** has the
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

