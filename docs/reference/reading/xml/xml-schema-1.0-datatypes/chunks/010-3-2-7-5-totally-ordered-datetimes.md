# 3.2.7.5 Totally ordered dateTimes

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-3-2-6-3-facet-comparison-for-durations.md) · [Next](011-3-2-10-1-lexical-representation.md)

A.Normalize P and Q. That is, if there is a timezone present, but
it is not Z, convert it to Z using the addition operation defined in
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes)

- Thus 2000-03-04T23:00:00+03:00 normalizes to 2000-03-04T20:00:00Z

B. If P and Q either both have a time zone or both do not have a time
zone, compare P and Q field by field from the year field down to the
second field, and return a result as soon as it can be determined. That is:

1. For each i in {year, month, day, hour, minute, second}
   1. If P[i] and Q[i] are both not specified, continue to the next i
   2. If P[i] is not specified and Q[i] is, or vice versa, stop and return
      P <> Q
   3. If P[i] < Q[i], stop and return P < Q
   4. If P[i] > Q[i], stop and return P > Q
2. Stop and return P = Q

C.Otherwise, if P contains a time zone and Q does not, compare
as follows:

1. P < Q if P < (Q with time zone +14:00)
2. P > Q if P > (Q with time zone -14:00)
3. P <> Q otherwise, that is, if (Q with time zone +14:00) < P < (Q with time zone -14:00)

D. Otherwise, if P does not contain a time zone and Q does, compare
as follows:

1. P < Q if (P with time zone -14:00) < Q.
2. P > Q if (P with time zone +14:00) > Q.
3. P <> Q otherwise, that is, if (P with time zone +14:00) < Q < (P with time zone -14:00)

Examples:

<table border="1" cellpadding="4" cellspacing="0"><tbody><tr><th align="center" style="background-color: #FFFF99">Determinate</th><th align="center" style="background-color: #FFFF99">Indeterminate</th></tr><tr><td align="center">2000-01-15T00:00:00 <b>&lt;</b> 2000-02-15T00:00:00</td><td align="center">2000-01-01T12:00:00 <b>&lt;&gt;</b>
          1999-12-31T23:00:00Z</td></tr><tr><td align="center">2000-01-15T12:00:00 <b>&lt;</b> 2000-01-16T12:00:00Z</td><td align="center">2000-01-16T12:00:00 <b>&lt;&gt;</b>
          2000-01-16T12:00:00Z</td></tr><tr><td align="center"> </td><td align="center">2000-01-16T00:00:00 <b>&lt;&gt;</b> 2000-01-16T12:00:00Z</td></tr></tbody></table>

<a id="totally-ordered-instants"></a>

##### 3.2.7.5 Totally ordered dateTimes

Certain derived types from **dateTime**
can be guaranteed have a total order. To
do so, they must require that a specific set of fields are always specified, and
that remaining fields (if any) are always unspecified. For example, the date
datatype without time zone is defined to contain exactly year, month, and day.
Thus dates without time zone have a total order among themselves.

<a id="dateTime-facets"></a>

##### 3.2.7.6 Constraining facets

**dateTime** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="time"></a>

#### 3.2.8 time

<a id="dt-time"></a>

[Definition:]**time**
represents an instant of time that recurs every day. The
[·value space·](003-1-4-terminology.md#dt-value-space) of **time** is the space
of *time of day* values as defined in § 5.3 of
[[ISO 8601]](044-h-references.md#ISO8601). Specifically, it is a set of zero-duration daily
time instances.

Since the lexical representation allows an optional time zone
indicator, **time** values are partially ordered because it may
not be able to determine the order of two values one of which has a
time zone and the other does not. The order relation on
**time** values is the
[Order relation on dateTime (§3.2.7.4)](009-3-2-6-3-facet-comparison-for-durations.md#dateTime-order) using an arbitrary date. See also
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes). Pairs of **time** values with or without time zone indicators are totally ordered.

**Note:**See the conformance note in  [(§3.2.6)](008-3-2-5-1-lexical-representation.md#year-sec-conformance) which
applies to the seconds part of this datatype as well.

<a id="time-lexical-repr"></a>

##### 3.2.8.1 Lexical representation

The lexical representation for **time** is the left
truncated lexical representation for [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime):
hh:mm:ss.sss with optional following time zone indicator. For example,
to indicate 1:20 pm for Eastern Standard Time which is 5 hours behind
Coordinated Universal Time (UTC), one would write: 13:20:00-05:00. See also
[ISO 8601 Date and Time Formats (§D)](037-d-iso-8601-date-and-time-formats.md#isoformats).

<a id="time-canonical-repr"></a>

##### 3.2.8.2 Canonical representation

The canonical representation for **time** is defined
by prohibiting certain options from the
[Lexical representation (§3.2.8.1)](010-3-2-7-5-totally-ordered-datetimes.md#time-lexical-repr). Specifically, either the time zone must
be omitted or, if present, the time zone must be Coordinated Universal
Time (UTC) indicated by a "Z".
Additionally, the canonical representation for midnight is 00:00:00.

<a id="time-facets"></a>

##### 3.2.8.3 Constraining facets

**time** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="date"></a>

#### 3.2.9 date

<a id="dt-date"></a>

[Definition:]
The [·value space·](003-1-4-terminology.md#dt-value-space) of **date**
consists of top-open intervals of exactly one day in length on the timelines of
[dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime), beginning on the beginning moment of each day (in
each timezone), i.e. '00:00:00', up to but not including '24:00:00' (which is
identical with '00:00:00' of the next day). For nontimezoned values, the top-open
intervals disjointly cover the nontimezoned timeline, one per day. For timezoned
values, the intervals begin at every minute and therefore overlap.

A "date object" is an object with year, month, and day properties just like those
of [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) objects, plus an optional *timezone-valued*
timezone property. (As with values of [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) timezones are a
special case of durations.)
Just as a [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) object corresponds to a point on one of the
timelines, a **date** object corresponds to an interval on one
of the two timelines as just described.

<a id="recoverable-timezone"></a>

Timezoned **date** values track the starting moment of their day, as
determined by their timezone; said timezone is generally recoverable for
canonical representations.
[Definition:]
The **recoverable timezone** is that duration which
is the result of subtracting the first moment (or any moment) of the timezoned
**date** from the first moment (or the corresponding moment) UTC on the
same **date**. [·recoverable timezone·](010-3-2-7-5-totally-ordered-datetimes.md#recoverable-timezone)s are
always durations between '+12:00' and '-11:59'. This "timezone normalization"
(which follows automatically from the definition of the **date**
[·value space·](003-1-4-terminology.md#dt-value-space)) is explained more in
[Lexical representation (§3.2.9.1)](010-3-2-7-5-totally-ordered-datetimes.md#date-lexical-representation).

For example: the first moment of 2002-10-10+13:00 is 2002-10-10T00:00:00+13,
which is 2002-10-09T11:00:00Z, which is also the first moment of 2002-10-09-11:00.
Therefore 2002-10-10+13:00 is 2002-10-09-11:00; *they are the same interval*.

**Note:**
For most timezones, either the first moment or last moment of the day (a
[dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) value, always UTC) will have a **date** portion
different from that of the **date** itself!
However, noon of that **date** (the midpoint of the interval) in that
(normalized) timezone will always have the same **date** portion as the
**date** itself, even when that noon point in time is normalized to
UTC. For example, 2002-10-10-05:00 begins during 2002-10-09Z and 2002-10-10+05:00
ends during 2002-10-11Z, but noon of both 2002-10-10-05:00 and 2002-10-10+05:00
falls in the interval which is 2002-10-10Z.

**Note:**See the conformance note in  [(§3.2.6)](008-3-2-5-1-lexical-representation.md#year-sec-conformance) which
applies to the year part of this datatype as well.

<a id="date-lexical-representation"></a>

##### 3.2.9.1 Lexical representation

For the following discussion, let the "date portion" of a [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime)
or **date** object be an object similar to a [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) or
**date** object, with similar year, month, and day properties, but no
others, having the same value for these properties as the original
[dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) or **date** object.

The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **date** consists of finite-length
sequences of characters of the form:
`'-'? yyyy '-' mm '-' dd zzzzzz?`
where the **date** and optional timezone are represented exactly the
same way as they are for [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime). The first moment of the
interval is that represented by:
`'-' yyyy '-' mm '-' dd 'T00:00:00' zzzzzz?`
and the least upper bound of the interval is the timeline point represented
(noncanonically) by:
`'-' yyyy '-' mm '-' dd 'T24:00:00' zzzzzz?`.

**Note:**
The [·recoverable timezone·](010-3-2-7-5-totally-ordered-datetimes.md#recoverable-timezone) of a **date** will always be
a duration between '+12:00' and '11:59'. Timezone lexical representations, as
explained for [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime), can range from '+14:00' to '-14:00'.
The result is that literals of **date**s with very large or very
negative timezones will map to a "normalized" **date** value with a
[·recoverable timezone·](010-3-2-7-5-totally-ordered-datetimes.md#recoverable-timezone) different from that represented in the original
representation, and a matching difference of +/- 1 day in the **date** itself.

<a id="date-canonical-representation"></a>

##### 3.2.9.2 Canonical representation

Given a member of the **date** [·value space·](003-1-4-terminology.md#dt-value-space), the
**date** portion of the canonical representation (the entire representation
for nontimezoned values, and all but the timezone representation for timezoned values)
is always the **date** portion of the [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) canonical
representation of the interval midpoint (the [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) representation,
truncated on the right to eliminate 'T' and all following characters).
For timezoned values, append the canonical representation of the [·recoverable timezone·](010-3-2-7-5-totally-ordered-datetimes.md#recoverable-timezone).

<a id="gYearMonth"></a>

#### 3.2.10 gYearMonth

<a id="dt-gYearMonth"></a>

[Definition:]
**gYearMonth** represents a
specific gregorian month in a specific gregorian year. The
[·value space·](003-1-4-terminology.md#dt-value-space) of **gYearMonth**
is the set of Gregorian calendar months as defined in § 5.2.1 of
[[ISO 8601]](044-h-references.md#ISO8601). Specifically, it is a set of one-month long,
non-periodic instances
e.g. 1999-10 to represent the whole month of 1999-10, independent of
how many days this month has.

Since the lexical representation allows an optional time zone
indicator, **gYearMonth** values are partially ordered because it may
not be possible to unequivocally determine the order of two values one of
which has a time zone and the other does not. If **gYearMonth**
values are considered as periods of time, the order relation on
**gYearMonth** values is the order relation on their starting instants.
This is discussed in [Order relation on dateTime (§3.2.7.4)](009-3-2-6-3-facet-comparison-for-durations.md#dateTime-order). See also
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes). Pairs of **gYearMonth**
values with or without time zone indicators are totally ordered.

