# 3.2.5.1 Lexical representation

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-3-2-3-1-lexical-representation.md) · [Next](009-3-2-6-3-facet-comparison-for-durations.md)

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
**double** values are [·comparable·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-comparable) with it, the result is a [·value space·](003-1-4-terminology.md#dt-value-space)
either having NaN as its only member (the inclusive cases) or that is empty
(the exclusive cases). If any other value is used for a bounding facet,
NaN will be excluded from the resulting restricted [·value space·](003-1-4-terminology.md#dt-value-space);
to add NaN back in requires union with the NaN-only space.This datatype differs from that of [[IEEE 754-1985]](044-h-references.md#ieee754) in that there is only one
NaN and only one zero. This makes the equality and ordering of values in the data
space differ from that of [[IEEE 754-1985]](044-h-references.md#ieee754) only in that for schema purposes NaN = NaN.

A literal in the [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) representing a
decimal number *d* maps to the normalized value
in the [·value space·](003-1-4-terminology.md#dt-value-space) of **double** that is
closest to *d*; if *d* is
exactly halfway between two such values then the even value is chosen.
This is the *best approximation* of *d*
([[Clinger, WD (1990)]](044-h-references.md#clinger1990), [[Gay, DM (1990)]](044-h-references.md#gay1990)), which is more
accurate than the mapping required by [[IEEE 754-1985]](044-h-references.md#ieee754).

<a id="double-lexical-representation"></a>

##### 3.2.5.1 Lexical representation

**double** values have a lexical representation
consisting of a mantissa followed, optionally, by the character "E" or
"e", followed by an exponent. The exponent [·must·](003-1-4-terminology.md#dt-must) be
an integer. The mantissa must be a decimal number. The representations
for exponent and mantissa must follow the lexical rules for
[integer](015-3-3-10-1-constraining-facets.md#integer) and [decimal](006-2-5-2-3-derived-by-union.md#decimal). If the "E" or "e"
and the following exponent are omitted, an exponent value of 0 is assumed.

The *special values*
positive
and negative infinity and not-a-number have lexical representations
`INF`, `-INF` and
`NaN`, respectively.
Lexical representations for zero may take a positive or negative sign.

For example, `-1E4, 1267.43233E12, 12.78e-2, 12`
`, -0, 0`
and `INF`
are all legal literals for **double**.

<a id="double-canonical-representation"></a>

##### 3.2.5.2 Canonical representation

The canonical representation for **double** is defined by
prohibiting certain options from the
[Lexical representation (§3.2.5.1)](008-3-2-5-1-lexical-representation.md#double-lexical-representation). Specifically, the exponent
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

<a id="double-facets"></a>

##### 3.2.5.3 Constraining facets

**double** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="duration"></a>

#### 3.2.6 duration

<a id="dt-duration"></a>

[Definition:]
**duration** represents a duration of time.
The [·value space·](003-1-4-terminology.md#dt-value-space) of **duration** is
a six-dimensional space where the coordinates
designate the Gregorian year, month, day, hour, minute, and second components defined in
§ 5.5.3.2 of [[ISO 8601]](044-h-references.md#ISO8601),
respectively. These components are ordered
in their significance by their order of appearance i.e. as year, month, day,
hour, minute, and second.

<a id="year-sec-conformance"></a>

**Note:**

All [·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors [·must·](003-1-4-terminology.md#dt-must)
support year values with a minimum of 4 digits (i.e., `YYYY`) and a minimum fractional second precision of milliseconds or three decimal digits (i.e. `s.sss`). However,
[·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors [·may·](003-1-4-terminology.md#dt-may)
set an application-defined limit on the maximum number of digits
they are prepared to support in these two cases, in which case that application-defined
maximum number [·must·](003-1-4-terminology.md#dt-must) be clearly documented.

<a id="duration-lexical-repr"></a>

##### 3.2.6.1 Lexical representation

The lexical representation for **duration** is the
[[ISO 8601]](044-h-references.md#ISO8601) extended format P*n*Y*n*
M*n*DT*n*H *n*M*n*S, where
*n*Y represents the number of years, *n*M the
number of months, *n*D the number of days, 'T' is the
date/time separator, *n*H the number of hours,
*n*M the number of minutes and *n*S the
number of seconds. The number of seconds can include decimal digits
to arbitrary precision.

The values of the
Year, Month, Day, Hour and Minutes components are not restricted but
allow an arbitrary
unsigned integer, i.e., an integer that
conforms to the pattern `[0-9]+`..
Similarly, the value of the Seconds component
allows an arbitrary unsigned decimal.
Following [[ISO 8601]](044-h-references.md#ISO8601), at least one digit must
follow the decimal point if it appears. That is, the value of the Seconds component
must conform to the pattern `[0-9]+(\.[0-9]+)?`.
Thus, the lexical representation of
**duration** does not follow the alternative
format of § 5.5.3.2.1 of [[ISO 8601]](044-h-references.md#ISO8601).

An optional preceding minus sign ('-') is
allowed, to indicate a negative duration. If the sign is omitted a
positive duration is indicated. See also [ISO 8601 Date and Time Formats (§D)](037-d-iso-8601-date-and-time-formats.md#isoformats).

For example, to indicate a duration of 1 year, 2 months, 3 days, 10
hours, and 30 minutes, one would write: `P1Y2M3DT10H30M`.
One could also indicate a duration of minus 120 days as:
`-P120D`.

Reduced precision and truncated representations of this format are allowed
provided they conform to the following:

- If the number of years, months, days, hours, minutes, or seconds in any
  expression equals zero, the number and its corresponding designator [·may·](003-1-4-terminology.md#dt-may)
  be omitted. However, at least one number and its designator [·must·](003-1-4-terminology.md#dt-must)
  be present.
- The seconds part [·may·](003-1-4-terminology.md#dt-may) have a decimal fraction.
- The designator 'T' must
  be absent if and only if all of the time items are absent.
  The designator 'P' must always be present.

For example, P1347Y, P1347M and P1Y2MT2H are all allowed;
P0Y1347M and P0Y1347M0D are allowed. P-1347M is not allowed although
-P1347M is allowed. P1Y2MT is not allowed.

<a id="duration-order"></a>

##### 3.2.6.2 Order relation on duration

In general, the [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) on **duration**
is a partial order since there is no determinate relationship between certain
durations such as one month (P1M) and 30 days (P30D).
The [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation)
of two **duration** values *x* and
*y* is *x < y iff s+x < s+y*
for each qualified [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime)  *s*
in the list below. These values for *s* cause the greatest deviations in the addition of
dateTimes and durations. Addition of durations to time instants is defined
in [Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes).

- 1696-09-01T00:00:00Z
- 1697-02-01T00:00:00Z
- 1903-03-01T00:00:00Z
- 1903-07-01T00:00:00Z

The following table shows the strongest relationship that can be determined
between example durations. The symbol <> means that the order relation is
indeterminate. Note that because of leap-seconds, a seconds field can vary
from 59 to 60. However, because of the way that addition is defined in
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes), they are still totally ordered.

<table border="1" cellpadding="4" cellspacing="0"><tbody><tr><th> </th><th colspan="7" style="background-color:#FFFF99">Relation</th></tr><tr><td style="background-color:#FFFF99">P<b>1Y</b></td><td>&gt; P<b>364D</b></td><td>&lt;&gt; P<b>365D</b></td><td colspan="3"> </td><td>&lt;&gt; P<b>366D</b></td><td>&lt; P<b>367D</b></td></tr><tr><td style="background-color:#FFFF99">P<b>1M</b></td><td>&gt; P<b>27D</b></td><td>&lt;&gt; P<b>28D</b></td><td colspan="2">&lt;&gt; P<b>29D</b></td><td>&lt;&gt; P<b>30D</b></td><td>&lt;&gt; P<b>31D</b></td><td>&lt; P<b>32D</b></td></tr><tr><td style="background-color:#FFFF99">P<b>5M</b></td><td>&gt; P<b>149D</b></td><td>&lt;&gt; P<b>150D</b></td><td>&lt;&gt; P<b>151D</b></td><td colspan="2">&lt;&gt; P<b>152D</b></td><td>&lt;&gt; P<b>153D</b></td><td>&lt; P<b>154D</b></td></tr></tbody></table>

Implementations are free to optimize the computation of the ordering relationship. For example, the following table can be used to
compare durations of a small number of months against days.

