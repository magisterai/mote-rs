# 3.2.3.1 Lexical representation

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-2-5-2-3-derived-by-union.md) · [Next](008-3-2-5-1-lexical-representation.md)

<a id="dt-decimal"></a>

[Definition:]**decimal**
represents a subset of the real numbers, which can be represented by decimal numerals.
The [·value space·](003-1-4-terminology.md#dt-value-space) of **decimal**
is the set of
numbers that can be obtained by multiplying an integer by a non-positive
power of ten, i.e., expressible as *i × 10^-n*
where *i* and *n* are integers
and *n >= 0*.
Precision is not reflected in this value space;
the number 2.0 is not distinct from the number 2.00.
The [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) on **decimal**
is the order relation on real numbers, restricted
to this subset.

**Note:**
All [·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors [·must·](003-1-4-terminology.md#dt-must)
support decimal numbers with a minimum of 18 decimal digits (i.e., with a
[·totalDigits·](031-4-3-10-mininclusive.md#dt-totalDigits) of 18). However,
[·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors [·may·](003-1-4-terminology.md#dt-may)
set an application-defined limit on the maximum number of decimal digits
they are prepared to support, in which case that application-defined
maximum number [·must·](003-1-4-terminology.md#dt-must) be clearly documented.

<a id="decimal-lexical-representation"></a>

##### 3.2.3.1 Lexical representation

**decimal** has a lexical representation
consisting of a finite-length sequence of decimal digits (#x30-#x39) separated
by a period as a decimal indicator.
An optional leading sign is allowed.
If the sign is omitted, "+" is assumed. Leading and trailing zeroes are optional.
If the fractional part is zero, the period and following zero(es) can
be omitted.
For example: `-1.23, 12678967.543233, +100000.00, 210`.

<a id="decimal-canonical-representation"></a>

##### 3.2.3.2 Canonical representation

The canonical representation for **decimal** is defined by
prohibiting certain options from the
[Lexical representation (§3.2.3.1)](007-3-2-3-1-lexical-representation.md#decimal-lexical-representation). Specifically, the preceding
optional "+" sign is prohibited. The decimal point is required. Leading and
trailing zeroes are prohibited subject to the following: there must be at least
one digit to the right and to the left of the decimal point which may be a zero.

<a id="decimal-facets"></a>

##### 3.2.3.3 Constraining facets

**decimal** has the
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

<a id="decimal-derived-types"></a>

##### 3.2.3.4 Derived datatypes

The following [·built-in·](006-2-5-2-3-derived-by-union.md#dt-built-in)
datatypes are [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from
**decimal**:

- [integer](015-3-3-10-1-constraining-facets.md#integer)

<a id="float"></a>

#### 3.2.4 float

<a id="dt-float"></a>

[Definition:]**float**
is patterned after the IEEE single-precision 32-bit floating point type
[[IEEE 754-1985]](044-h-references.md#ieee754). The basic [·value space·](003-1-4-terminology.md#dt-value-space) of
**float** consists of the values
*m × 2^e*, where *m*
is an integer whose absolute value is less than
*2^24*, and *e* is an integer
between -149 and 104, inclusive. In addition to the basic
[·value space·](003-1-4-terminology.md#dt-value-space) described above, the
[·value space·](003-1-4-terminology.md#dt-value-space) of **float** also contains the
following
three
*special values*:
positive and negative infinity and not-a-number
(NaN).
The [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) on **float**
is: *x < y iff y - x* is positive
for x and y in the value space.
Positive infinity is greater than all other non-NaN values.
NaN equals itself but is [·incomparable·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-incomparable) with (neither greater than nor less than)
any other value in the [·value space·](003-1-4-terminology.md#dt-value-space).

**Note:**
"Equality" in this Recommendation is defined to be "identity" (i.e., values that
are identical in the [·value space·](003-1-4-terminology.md#dt-value-space) are equal and vice versa).
Identity must be used for the few operations that are defined in this Recommendation.
Applications using any of the datatypes defined in this Recommendation may use different
definitions of equality for computational purposes; [[IEEE 754-1985]](044-h-references.md#ieee754)-based computation systems
are examples. Nothing in this Recommendation should be construed as requiring that
such applications use identity as their equality relationship when computing.Any value [·incomparable·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-incomparable) with the value used for the four bounding facets
([·minInclusive·](031-4-3-10-mininclusive.md#dt-minInclusive), [·maxInclusive·](028-4-3-6-1-the-whitespace-schema-component.md#dt-maxInclusive),
[·minExclusive·](030-4-3-9-minexclusive.md#dt-minExclusive), and [·maxExclusive·](029-4-3-7-3-maxinclusive-validation-rules.md#dt-maxExclusive)) will be
excluded from the resulting restricted [·value space·](003-1-4-terminology.md#dt-value-space). In particular,
when "NaN" is used as a facet value for a bounding facet, since no other
**float** values are [·comparable·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-comparable) with it, the result is a [·value space·](003-1-4-terminology.md#dt-value-space)
either having NaN as its only member (the inclusive cases) or that is empty
(the exclusive cases). If any other value is used for a bounding facet,
NaN will be excluded from the resulting restricted [·value space·](003-1-4-terminology.md#dt-value-space);
to add NaN back in requires union with the NaN-only space.This datatype differs from that of [[IEEE 754-1985]](044-h-references.md#ieee754) in that there is only one
NaN and only one zero. This makes the equality and ordering of values in the data
space differ from that of [[IEEE 754-1985]](044-h-references.md#ieee754) only in that for schema purposes NaN = NaN.

A literal in the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) representing a
decimal number *d* maps to the normalized value
in the [·value space·](003-1-4-terminology.md#dt-value-space) of **float** that is
closest to *d* in the sense defined by
[[Clinger, WD (1990)]](044-h-references.md#clinger1990); if *d* is
exactly halfway between two such values then the even value is chosen.

<a id="float-lexical-representation"></a>

##### 3.2.4.1 Lexical representation

**float** values have a lexical representation
consisting of a mantissa followed, optionally, by the character
"E" or "e", followed by an exponent. The exponent [·must·](003-1-4-terminology.md#dt-must)
be an [integer](015-3-3-10-1-constraining-facets.md#integer). The mantissa must be a [decimal](006-2-5-2-3-derived-by-union.md#decimal) number. The representations
for exponent and mantissa must follow the lexical rules for
[integer](015-3-3-10-1-constraining-facets.md#integer) and [decimal](006-2-5-2-3-derived-by-union.md#decimal). If the "E" or "e" and
the following exponent are omitted, an exponent value of 0 is assumed.

The *special values*
positive
and negative infinity and not-a-number have lexical representations
`INF`, `-INF` and
`NaN`, respectively.
Lexical representations for zero may take a positive or negative sign.

For example, `-1E4, 1267.43233E12, 12.78e-2, 12`
`, -0, 0`
and `INF` are all legal literals for **float**.

<a id="float-canonical-representation"></a>

##### 3.2.4.2 Canonical representation

The canonical representation for **float** is defined by
prohibiting certain options from the
[Lexical representation (§3.2.4.1)](007-3-2-3-1-lexical-representation.md#float-lexical-representation). Specifically, the exponent
must be indicated by "E". Leading zeroes and the preceding optional "+" sign
are prohibited in the exponent.
If the exponent is zero, it must be indicated by "E0".
For the mantissa, the preceding optional "+" sign is prohibited
and the decimal point is required.
Leading and trailing zeroes are prohibited subject to the following:
number representations must
be normalized such that there is a single digit
which is non-zero
to the left of the decimal point and at least a single digit to the
right of the decimal point
unless the value being represented is zero. The canonical
representation for zero is 0.0E0.

<a id="float-facets"></a>

##### 3.2.4.3 Constraining facets

**float** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="double"></a>

#### 3.2.5 double

<a id="dt-double"></a>

[Definition:]The **double**
datatype
is patterned after the
IEEE double-precision 64-bit floating point
type [[IEEE 754-1985]](044-h-references.md#ieee754). The basic [·value space·](003-1-4-terminology.md#dt-value-space)
of **double** consists of the values
*m × 2^e*, where *m*
is an integer whose absolute value is less than
*2^53*, and *e* is an
integer between -1075 and 970, inclusive. In addition to the basic
[·value space·](003-1-4-terminology.md#dt-value-space) described above, the
[·value space·](003-1-4-terminology.md#dt-value-space) of **double** also contains
the following
three
*special values*:
positive and negative infinity and not-a-number
(NaN).
The [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) on **double**
is: *x < y iff y - x* is positive
for x and y in the value space.
Positive infinity is greater than all other non-NaN values.
NaN equals itself but is [·incomparable·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-incomparable) with (neither greater than nor less than)
any other value in the [·value space·](003-1-4-terminology.md#dt-value-space).

