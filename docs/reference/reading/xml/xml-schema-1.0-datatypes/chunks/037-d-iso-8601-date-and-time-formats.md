# D ISO 8601 Date and Time Formats

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](036-continuation-36.md) · [Next](038-d-2-truncated-and-reduced-formats.md)

<a id="isoformats"></a>

## D ISO 8601 Date and Time Formats

<a id="formatdetails"></a>

### D.1 ISO 8601 Conventions

The [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) datatypes
[duration](008-3-2-5-1-lexical-representation.md#duration), [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime), [time](010-3-2-7-5-totally-ordered-datetimes.md#time),
[date](010-3-2-7-5-totally-ordered-datetimes.md#date), [gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), [gMonthDay](011-3-2-10-1-lexical-representation.md#gMonthDay),
[gDay](011-3-2-10-1-lexical-representation.md#gDay), [gMonth](011-3-2-10-1-lexical-representation.md#gMonth) and [gYear](011-3-2-10-1-lexical-representation.md#gYear)
use lexical formats inspired by
[[ISO 8601]](044-h-references.md#ISO8601).
Following [[ISO 8601]](044-h-references.md#ISO8601), the lexical forms of
these datatypes can include only the characters #20 through #7F.
This appendix provides more detail on the ISO
formats and discusses some deviations from them for the datatypes
defined in this specification.

[[ISO 8601]](044-h-references.md#ISO8601) "specifies the representation of dates in the
proleptic Gregorian calendar and times and representations of periods of time".
The proleptic Gregorian calendar includes dates prior to 1582 (the year it came
into use as an ecclesiastical calendar).
It should be pointed out that the datatypes described in this
specification do not cover all the types of data covered by
[[ISO 8601]](044-h-references.md#ISO8601), nor do they support all the lexical
representations for those types of data.

[[ISO 8601]](044-h-references.md#ISO8601) lexical formats are described using "pictures"
in which characters are used in place of decimal digits.
The allowed decimal digits are (#x30-#x39).
For the primitive datatypes
[dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime), [time](010-3-2-7-5-totally-ordered-datetimes.md#time),
[date](010-3-2-7-5-totally-ordered-datetimes.md#date), [gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), [gMonthDay](011-3-2-10-1-lexical-representation.md#gMonthDay),
[gDay](011-3-2-10-1-lexical-representation.md#gDay), [gMonth](011-3-2-10-1-lexical-representation.md#gMonth) and [gYear](011-3-2-10-1-lexical-representation.md#gYear).
these characters have the following meanings:

- C -- represents a digit used in the thousands and hundreds components,
  the "century" component, of the time element "year". Legal values are
  from 0 to 9.
- Y -- represents a digit used in the tens and units components of the time
  element "year". Legal values are from 0 to 9.
- M -- represents a digit used in the time element "month". The two
  digits in a MM format can have values from 1 to 12.
- D -- represents a digit used in the time element "day". The two digits
  in a DD format can have values from 1 to 28 if the month value equals 2,
  1 to 29 if the month value equals 2 and the year is a leap year, 1 to 30
  if the month value equals 4, 6, 9 or 11, and 1 to 31 if the month value
  equals 1, 3, 5, 7, 8, 10 or 12.
- h -- represents a digit used in the time element "hour". The two digits
  in a hh format can have values from 0 to
  24.
  If the value of the hour element is 24 then the values of the minutes
  element and the seconds element must be 00 and 00.
- m -- represents a digit used in the time element "minute". The two digits
  in a mm format can have values from 0 to 59.
- s -- represents a digit used in the time element "second". The two
  digits in a ss format can have values from 0 to 60. In the formats
  described in this specification the whole number of seconds [·may·](003-1-4-terminology.md#dt-may)
  be followed by decimal seconds to an arbitrary level of precision.
  This is represented in the picture by "ss.sss". A value of 60 or more is
  allowed only in the case of leap seconds.Strictly speaking, a value of
  60 or more is not sensible unless the month and day could
  represent March 31, June 30, September 30, or December 31 *in UTC*.
  Because the leap second is added or subtracted as the last second of the day
  in UTC time, the long (or short) minute could occur at other times in local
  time. In cases where the leap second is used with an inappropriate month
  and day it, and any fractional seconds, should considered as added or
  subtracted from the following minute.

For all the information items indicated by the above characters, leading
zeros are required where indicated.

In addition to the above, certain characters are used as designators
and appear as themselves in lexical formats.

- T -- is used as time designator to indicate the start of the
  representation of the time of day in [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime).
- Z -- is used as time-zone designator, immediately (without a space)
  following a data element expressing the time of day in Coordinated
  Universal Time (UTC) in
  [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime), [time](010-3-2-7-5-totally-ordered-datetimes.md#time),
  [date](010-3-2-7-5-totally-ordered-datetimes.md#date), [gYearMonth](010-3-2-7-5-totally-ordered-datetimes.md#gYearMonth), [gMonthDay](011-3-2-10-1-lexical-representation.md#gMonthDay),
  [gDay](011-3-2-10-1-lexical-representation.md#gDay), [gMonth](011-3-2-10-1-lexical-representation.md#gMonth), and [gYear](011-3-2-10-1-lexical-representation.md#gYear).

In the lexical format for [duration](008-3-2-5-1-lexical-representation.md#duration) the following
characters are also used as designators and appear as themselves in
lexical formats:

- P -- is used as the time duration designator, preceding a data element
  representing a given duration of time.
- Y -- follows the number of years in a time duration.
- M -- follows the number of months or minutes in a time duration.
- D -- follows the number of days in a time duration.
- H -- follows the number of hours in a time duration.
- S -- follows the number of seconds in a time duration.

The values of the
Year, Month, Day, Hour and Minutes components are not restricted but
allow an arbitrary integer. Similarly, the value of the Seconds component
allows an arbitrary decimal. Thus, the lexical format for
[duration](008-3-2-5-1-lexical-representation.md#duration) and datatypes derived from it
does not follow the alternative
format of § 5.5.3.2.1 of [[ISO 8601]](044-h-references.md#ISO8601).

