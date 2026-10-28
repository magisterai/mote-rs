# 3.2.6.3 Facet Comparison for durations

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-3-2-5-1-lexical-representation.md) · [Next](010-3-2-7-5-totally-ordered-datetimes.md)

<table border="1" cellpadding="2" cellspacing="0"><tbody><tr><th align="center"> </th><th align="center" style="background-color: #FFFF99">Months</th><th align="center" style="background-color: #FFFF99">1</th><th align="center" style="background-color: #FFFF99">2</th><th align="center" style="background-color: #FFFF99">3</th><th align="center" style="background-color: #FFFF99">4</th><th align="center" style="background-color: #FFFF99">5</th><th align="center" style="background-color: #FFFF99">6</th><th align="center" style="background-color: #FFFF99">7</th><th align="center" style="background-color: #FFFF99">8</th><th align="center" style="background-color: #FFFF99">9</th><th align="center" style="background-color: #FFFF99">10</th><th align="center" style="background-color: #FFFF99">11</th><th align="center" style="background-color: #FFFF99">12</th><th align="center" style="background-color: #FFFF99">13</th><th align="center" style="background-color: #FFFF99">...</th></tr><tr><th align="center" rowspan="2" style="background-color: #FFFF99">Days</th><th align="center" style="background-color: #FFFF99">Minimum</th><td align="center">28</td><td align="center">59</td><td align="center">89</td><td align="center">120</td><td align="center">150</td><td align="center">181</td><td align="center">212</td><td align="center">242</td><td align="center">273</td><td align="center">303</td><td align="center">334</td><td align="center">365</td><td align="center">393</td><td align="center">...</td></tr><tr><th align="center" style="background-color: #FFFF99">Maximum</th><td align="center">31</td><td align="center">62</td><td align="center">92</td><td align="center">123</td><td align="center">153</td><td align="center">184</td><td align="center">215</td><td align="center">245</td><td align="center">276</td><td align="center">306</td><td align="center">337</td><td align="center">366</td><td align="center">397</td><td align="center">...</td></tr></tbody></table>

<a id="facet-comparison-for-durations"></a>

##### 3.2.6.3 Facet Comparison for durations

In comparing **duration**
values with [minInclusive](031-4-3-10-mininclusive.md#dc-minInclusive), [minExclusive](030-4-3-9-minexclusive.md#dc-minExclusive),
[maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#dc-maxInclusive) and [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#dc-maxExclusive) facet values
indeterminate comparisons should be considered as "false".

<a id="total-order-durations"></a>

##### 3.2.6.4 Totally ordered durations

Certain derived datatypes of durations can be guaranteed have a total order. For
this, they must have fields from only one row in the list below and the time zone
must either be required or prohibited.

- year, month
- day, hour, minute, second

For example, a datatype could be defined to correspond to the
[[SQL]](044-h-references.md#SQL) datatype Year-Month interval that required a four digit
year field and a two digit month field but required all other fields to be unspecified. This datatype could be defined as below and would have a total order.



```
<simpleType name='SQL-Year-Month-Interval'>
    <restriction base='duration'>
      <pattern value='P\p{Nd}{4}Y\p{Nd}{2}M'/>
    </restriction>
</simpleType>
```




<a id="duration-facets"></a>

##### 3.2.6.5 Constraining facets

**duration** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="dateTime"></a>

#### 3.2.7 dateTime

<a id="dt-dateTime"></a>

[Definition:]
**dateTime** values may be viewed as objects with integer-valued
year, month, day, hour and minute properties, a decimal-valued second property,
and a boolean timezoned property.
Each such object also has one decimal-valued
method or computed property, timeOnTimeline, whose value is always a decimal
number; the values are dimensioned in seconds, the integer 0 is
0001-01-01T00:00:00 and the value of timeOnTimeline for other **dateTime**
values is computed using the Gregorian algorithm as modified for leap-seconds.
The timeOnTimeline values form two related "timelines", one for timezoned
values and one for non-timezoned values.
Each timeline is a copy of the [·value space·](003-1-4-terminology.md#dt-value-space) of [decimal](006-2-5-2-3-derived-by-union.md#decimal),
with integers given units of seconds.

The [·value space·](003-1-4-terminology.md#dt-value-space) of
**dateTime** is closely related to the dates and times described in ISO 8601.
For clarity, the text above specifies a particular origin point for the
timeline.
It should be noted, however, that schema processors need not expose the
timeOnTimeline value to schema users, and there is no requirement that a
timeline-based implementation use the particular origin described here in
its internal representation.
Other interpretations of the [·value space·](003-1-4-terminology.md#dt-value-space) which lead to the
same results (i.e., are isomorphic) are of course acceptable.

All timezoned times are Coordinated Universal Time (UTC, sometimes called
"Greenwich Mean Time"). Other timezones indicated in lexical representations
are converted to UTC during conversion of literals to values.
"Local" or untimezoned times are presumed to be the time in the timezone of some
unspecified locality as prescribed by the appropriate legal authority;
currently there are no legally prescribed timezones which are durations
whose magnitude is greater than 14 hours. The value of each numeric-valued property
(other than timeOnTimeline) is limited to the maximum value within the interval
determined by the next-higher property. For example, the day value can never be 32,
and cannot even be 29 for month 02 and year 2002 (February 2002).

<a id="year-zero"></a>

**Note:**

The date and time datatypes described in this recommendation were inspired
by [[ISO 8601]](044-h-references.md#ISO8601). '0001' is the lexical representation of the year 1 of the Common Era
(1
CE, sometimes written "AD 1" or "1 AD"). There is no year 0, and '0000' is not a valid lexical representation. '-0001' is the lexical representation of the year 1 Before
Common Era (1 BCE, sometimes written "1 BC").Those using this (1.0) version of this Recommendation to
represent negative years should be aware that the interpretation of lexical
representations beginning with a `'-'` is likely to change in
subsequent versions.[[ISO 8601]](044-h-references.md#ISO8601)
makes no mention of the year 0; in [[ISO 8601:1998 Draft Revision]](044-h-references.md#ISO8601-1998)
the form '0000' was disallowed and this recommendation disallows it as well.
However, [[ISO 8601:2000 Second Edition]](044-h-references.md#ISO8601-2000), which became available just as we were completing version
1.0, allows the form '0000', representing the year 1 BCE. A number of external commentators
have also suggested that '0000' be
allowed, as the lexical representation for 1 BCE, which is the normal usage in
astronomical contexts.
It is the intention of the XML Schema
Working Group to allow '0000' as a lexical representation in the
**dateTime**, **date**, **gYear**, and
**gYearMonth** datatypes in a subsequent version
of this Recommendation. '0000' will be the lexical representation of 1
BCE (which is a leap year), '-0001' will become the lexical representation of 2
BCE (not 1 BCE as in this (1.0) version), '-0002' of 3 BCE, etc.

**Note:**See the conformance note in  [(§3.2.6)](008-3-2-5-1-lexical-representation.md#year-sec-conformance) which
applies to this datatype as well.

<a id="dateTime-lexical-representation"></a>

##### 3.2.7.1 Lexical representation

The [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) of **dateTime** consists of
finite-length sequences of characters of the form:
`'-'? yyyy '-' mm '-' dd 'T' hh ':' mm ':' ss ('.' s+)? (zzzzzz)?`,
where

- '-'? *yyyy* is a four-or-more digit optionally negative-signed
  numeral that represents the year; if more than four digits, leading zeros
  are prohibited, and '0000' is prohibited (see the Note above  [(§3.2.7)](009-3-2-6-3-facet-comparison-for-durations.md#year-zero); also note that a plus sign is **not** permitted);
- the remaining '-'s are separators between parts of the date portion;
- the first *mm* is a two-digit numeral that represents the month;
- *dd* is a two-digit numeral that represents the day;
- 'T' is a separator indicating that time-of-day follows;
- *hh* is a two-digit numeral that represents the hour; '24' is permitted if the
  minutes and seconds represented are zero, and the **dateTime** value so
  represented is the first instant of the following day (the hour property of a
  **dateTime** object in the [·value space·](003-1-4-terminology.md#dt-value-space) cannot have
  a value greater than 23);
- ':' is a separator between parts of the time-of-day portion;
- the second *mm* is a two-digit numeral that represents the minute;
- *ss* is a two-integer-digit numeral that represents the
  whole seconds;
- '.' *s+* (if present) represents the
  fractional seconds;
- *zzzzzz* (if present) represents the timezone (as described below).

For example, 2002-10-10T12:00:00-05:00 (noon on 10 October 2002, Central Daylight
Savings Time as well as Eastern Standard Time in the U.S.) is 2002-10-10T17:00:00Z,
five hours later than 2002-10-10T12:00:00Z.

For further guidance on arithmetic with **dateTime**s and durations,
see [Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes).

<a id="dateTime-canonical-representation"></a>

##### 3.2.7.2 Canonical representation

Except for trailing fractional zero digits in the seconds representation,
'24:00:00' time representations, and timezone (for timezoned values), the mapping
from literals to values is one-to-one. Where there is more than
one possible representation, the canonical representation is as follows:

- The 2-digit numeral representing the hour must not be '`24`';
- The fractional second string, if present, must not end in '`0`';
- for timezoned values, the timezone must be
  represented with '`Z`'
  (All timezoned **dateTime** values are UTC.).

<a id="dateTime-timezones"></a>

##### 3.2.7.3 Timezones

Timezones are durations with (integer-valued) hour and minute properties
(with the hour magnitude limited to at most 14, and the minute magnitude
limited to at most 59, except that if the hour magnitude is 14, the minute
value must be 0); they may be both positive or both negative.

The lexical representation of a timezone is a string of the form:
`(('+' | '-') hh ':' mm) | 'Z'`,
where

- *hh* is a two-digit numeral (with leading zeros as required) that
  represents the hours,
- *mm* is a two-digit numeral that represents the minutes,
- '+' indicates a nonnegative duration,
- '-' indicates a nonpositive duration.

The mapping so defined is one-to-one, except that '+00:00', '-00:00', and 'Z'
all represent the same zero-length duration timezone, UTC; 'Z' is its canonical
representation.

When a timezone is added to a UTC **dateTime**, the result is the date
and time "in that timezone". For example, 2002-10-10T12:00:00+05:00 is
2002-10-10T07:00:00Z and 2002-10-10T00:00:00+05:00 is 2002-10-09T19:00:00Z.

<a id="dateTime-order"></a>

##### 3.2.7.4 Order relation on dateTime

**dateTime** value objects on either timeline are totally ordered by their timeOnTimeline
values; between the two timelines, **dateTime** value objects are ordered by their
timeOnTimeline values when their timeOnTimeline values differ by more than
fourteen hours, with those whose difference is a duration of 14 hours or less
being [·incomparable·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-incomparable).

In general, the [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) on **dateTime**
is a partial order since there is no determinate relationship between certain
instants. For example, there is no determinate
ordering between
(a)
2000-01-20T12:00:00 and (b) 2000-01-20T12:00:00**Z**. Based on
timezones currently in use, (c) could vary from 2000-01-20T12:00:00+12:00 to
2000-01-20T12:00:00-13:00. It is, however, possible for this range to expand or
contract in the future, based on local laws. Because of this, the following
definition uses a somewhat broader range of indeterminate values: +14:00..-14:00.

The following definition uses the notation S[year] to represent the year
field of S, S[month] to represent the month field, and so on. The notation (Q
& "-14:00") means adding the timezone -14:00 to Q, where Q did not
already have a timezone. *This is a logical explanation of the process. Actual
implementations are free to optimize as long as they produce the same results.*

The ordering between two **dateTime**s P and Q is defined by the following
algorithm:

