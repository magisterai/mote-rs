# D.2 Truncated and Reduced Formats

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](037-d-iso-8601-date-and-time-formats.md) · [Next](039-e-1-algorithm.md)

<a id="truncatedformats"></a>

### D.2 Truncated and Reduced Formats

[[ISO 8601]](044-h-references.md#ISO8601) supports a variety of "truncated" formats in
which some of the characters on the left of specific formats, for example,
the
century, can be omitted.
Truncated formats are, in
general, not permitted for the datatypes defined in this specification
with three exceptions. The [time](010-3-2-7-5-totally-ordered-datetimes.md#time) datatype uses
a truncated format for [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime)
which represents an instant of time that recurs every day.
Similarly, the [gMonthDay](011-3-2-10-1-lexical-representation.md#gMonthDay) and [gDay](011-3-2-10-1-lexical-representation.md#gDay)
datatypes use left-truncated formats for [date](010-3-2-7-5-totally-ordered-datetimes.md#date).
The datatype [gMonth](011-3-2-10-1-lexical-representation.md#gMonth) uses a right and left truncated format for
[date](010-3-2-7-5-totally-ordered-datetimes.md#date).

[[ISO 8601]](044-h-references.md#ISO8601) also supports a variety of "reduced" or right-truncated
formats in which some of the characters to the right of specific formats,
such as the
time specification, can be omitted. Right truncated formats are also, in
general,
not permitted for the datatypes defined in this specification
with the following exceptions:
right-truncated representations of [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) are used as
lexical representations for [date](010-3-2-7-5-totally-ordered-datetimes.md#date), [gMonth](011-3-2-10-1-lexical-representation.md#gMonth),
[gYear](011-3-2-10-1-lexical-representation.md#gYear).

<a id="deviantformats"></a>

### D.3 Deviations from ISO 8601 Formats

D.3.1 [Sign Allowed](038-d-2-truncated-and-reduced-formats.md#signallowed)  
        D.3.2 [No Year Zero](038-d-2-truncated-and-reduced-formats.md#noYearZero)  
        D.3.3 [More Than 9999 Years](038-d-2-truncated-and-reduced-formats.md#morethan9999years)  
        D.3.4 [Time zone permitted](038-d-2-truncated-and-reduced-formats.md#timeZonePermited)

<a id="signallowed"></a>

#### D.3.1 Sign Allowed

An optional minus sign is allowed immediately preceding, without a space,
the lexical representations for [duration](008-3-2-5-1-lexical-representation.md#duration), [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime),
[date](010-3-2-7-5-totally-ordered-datetimes.md#date), [gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), [gYear](011-3-2-10-1-lexical-representation.md#gYear).

<a id="noYearZero"></a>

#### D.3.2 No Year Zero

The year "0000" is an illegal year value.

<a id="morethan9999years"></a>

#### D.3.3 More Than 9999 Years

To accommodate year values greater than 9999, more than four digits are
allowed in the year representations of [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime),
[date](010-3-2-7-5-totally-ordered-datetimes.md#date), [gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), and [gYear](011-3-2-10-1-lexical-representation.md#gYear).
This follows
[[ISO 8601:2000 Second Edition]](044-h-references.md#ISO8601-2000).

<a id="timeZonePermited"></a>

#### D.3.4 Time zone permitted

The lexical representations for the datatypes [date](010-3-2-7-5-totally-ordered-datetimes.md#date),
[gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), [gMonthDay](011-3-2-10-1-lexical-representation.md#gMonthDay), [gDay](011-3-2-10-1-lexical-representation.md#gDay),
[gMonth](011-3-2-10-1-lexical-representation.md#gMonth) and [gYear](011-3-2-10-1-lexical-representation.md#gYear) permit an optional
trailing time zone specificiation.

<a id="adding-durations-to-dateTimes"></a>

## E Adding durations to dateTimes

Given a [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) S and a [duration](008-3-2-5-1-lexical-representation.md#duration) D, this
appendix specifies how to compute a [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) E where E is the
end of the time period with start S and duration D i.e. E = S + D. Such
computations are used, for example, to determine whether a [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime)
is within a specific time period. This appendix also addresses the addition of
[duration](008-3-2-5-1-lexical-representation.md#duration)s to the datatypes [date](010-3-2-7-5-totally-ordered-datetimes.md#date),
[gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), [gYear](011-3-2-10-1-lexical-representation.md#gYear), [gDay](011-3-2-10-1-lexical-representation.md#gDay) and
[gMonth](011-3-2-10-1-lexical-representation.md#gMonth), which can be viewed as a set of [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime)s.
In such cases, the addition is made to the first or starting
[dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime) in the set.

*This is a logical explanation of the process.
Actual implementations are free to optimize as long as they produce the same
results.*  The calculation uses the notation S[year] to represent the year
field of S, S[month] to represent the month field, and so on. It also depends on
the following functions:

- fQuotient(a, b) = the greatest integer less than or equal to a/b
  - fQuotient(-1,3) = -1
  - fQuotient(0,3)...fQuotient(2,3) = 0
  - fQuotient(3,3) = 1
  - fQuotient(3.123,3) = 1
- modulo(a, b) = a - fQuotient(a,b)\*b
  - modulo(-1,3) = 2
  - modulo(0,3)...modulo(2,3) = 0...2
  - modulo(3,3) = 0
  - modulo(3.123,3) = 0.123
- fQuotient(a, low, high) = fQuotient(a - low, high - low)
  - fQuotient(0, 1, 13) = -1
  - fQuotient(1, 1, 13) ... fQuotient(12, 1, 13) = 0
  - fQuotient(13, 1, 13) = 1
  - fQuotient(13.123, 1, 13) = 1
- modulo(a, low, high) = modulo(a - low, high - low) + low
  - modulo(0, 1, 13) = 12
  - modulo(1, 1, 13) ... modulo(12, 1, 13) = 1...12
  - modulo(13, 1, 13) = 1
  - modulo(13.123, 1, 13) = 1.123
- maximumDayInMonthFor(yearValue, monthValue) =
  - M := modulo(monthValue, 1, 13)
  - Y := yearValue + fQuotient(monthValue, 1, 13)
  - Return a value based on M and Y:

<table border="1"><tbody><tr><td style="background-color:#FFFF99"><b>31</b></td><td colspan="2">M = January, March, May, July, August, October, or
          December</td></tr><tr><td style="background-color:#FFFF99"><b>30</b></td><td colspan="2">M = April, June, September, or November</td></tr><tr><td style="background-color:#FFFF99"><b>29</b></td><td>M = February AND (modulo(Y, 400) = 0 OR
          (modulo(Y, 100) != 0) AND modulo(Y, 4) = 0)</td></tr><tr><td style="background-color:#FFFF99"><b>28</b></td><td>Otherwise</td></tr></tbody></table>

