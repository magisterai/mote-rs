# 3.2.10.1 Lexical representation

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-3-2-7-5-totally-ordered-datetimes.md) · [Next](012-3-2-15-hexbinary.md)

**Note:**
Because month/year combinations in one calendar only rarely correspond
to month/year combinations in other calendars, values of this type
are not, in general, convertible to simple values corresponding to month/year
combinations in other calendars. This type should therefore be used with caution
in contexts where conversion to other calendars is desired.

**Note:**See the conformance note in  [(§3.2.6)](008-3-2-5-1-lexical-representation.md#year-sec-conformance) which
applies to the year part of this datatype as well.

<a id="gYearMonth-lexical-repr"></a>

##### 3.2.10.1 Lexical representation

The lexical representation for **gYearMonth** is the reduced
(right truncated) lexical representation for [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime):
CCYY-MM. No left truncation is allowed. An optional following time
zone qualifier is allowed. To accommodate year values outside the range from 0001 to 9999, additional digits
can be added to the left of this representation and a preceding "-" sign is allowed.

For example, to indicate the month of May 1999, one would write: 1999-05.
See also [ISO 8601 Date and Time Formats (§D)](037-d-iso-8601-date-and-time-formats.md#isoformats).

<a id="gYearMonth-facets"></a>

##### 3.2.10.2 Constraining facets

**gYearMonth** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="gYear"></a>

#### 3.2.11 gYear

<a id="dt-gYear"></a>

[Definition:]
**gYear** represents a
gregorian calendar year. The [·value space·](003-1-4-terminology.md#dt-value-space) of
**gYear** is the set of Gregorian calendar years as defined in
§ 5.2.1 of [[ISO 8601]](044-h-references.md#ISO8601). Specifically, it is a set of one-year
long, non-periodic instances
e.g. lexical 1999 to represent the whole year 1999, independent of
how many months and days this year has.

Since the lexical representation allows an optional time zone
indicator, **gYear** values are partially ordered because it may
not be possible to unequivocally determine the order of two values one of which has a
time zone and the other does not. If
**gYear** values are considered as periods of time, the order relation
on **gYear** values is the order relation on their starting instants.
This is discussed in [Order relation on dateTime (§3.2.7.4)](009-3-2-6-3-facet-comparison-for-durations.md#dateTime-order). See also
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes). Pairs of **gYear** values with or without time zone indicators are totally ordered.

**Note:**
Because years in one calendar only rarely correspond to years
in other calendars, values of this type
are not, in general, convertible to simple values corresponding to years
in other calendars. This type should therefore be used with caution
in contexts where conversion to other calendars is desired.

**Note:**See the conformance note in  [(§3.2.6)](008-3-2-5-1-lexical-representation.md#year-sec-conformance) which
applies to the year part of this datatype as well.

<a id="gYear-lexical-repr"></a>

##### 3.2.11.1 Lexical representation

The lexical representation for **gYear** is the reduced (right
truncated) lexical representation for [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime): CCYY.
No left truncation is allowed. An optional following time
zone qualifier is allowed as for [dateTime](009-3-2-6-3-facet-comparison-for-durations.md#dateTime). To
accommodate year values outside the range from 0001 to 9999, additional
digits can be added to the left of this representation and a preceding
"-" sign is allowed.

For example, to indicate 1999, one would write: 1999.
See also [ISO 8601 Date and Time Formats (§D)](037-d-iso-8601-date-and-time-formats.md#isoformats).

<a id="gYear-facets"></a>

##### 3.2.11.2 Constraining facets

**gYear** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="gMonthDay"></a>

#### 3.2.12 gMonthDay

<a id="dt-gMonthDay"></a>

[Definition:]
**gMonthDay** is a gregorian date that recurs, specifically a day of
the year such as the third of May. Arbitrary recurring dates are not
supported by this datatype. The [·value space·](003-1-4-terminology.md#dt-value-space) of
**gMonthDay** is the set of *calendar
dates*, as defined in § 3 of [[ISO 8601]](044-h-references.md#ISO8601). Specifically,
it is a set of one-day long, annually periodic instances.

Since the lexical representation allows an optional time zone
indicator, **gMonthDay** values are partially ordered because it may
not be possible to unequivocally determine the order of two values one of which has a
time zone and the other does not. If
**gMonthDay** values are considered as periods of time,
in an arbitrary leap year, the order relation
on **gMonthDay** values is the order relation on their starting instants.
This is discussed in [Order relation on dateTime (§3.2.7.4)](009-3-2-6-3-facet-comparison-for-durations.md#dateTime-order). See also
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes). Pairs of **gMonthDay** values with or without time zone indicators are totally ordered.

**Note:**
Because day/month combinations in one calendar only rarely correspond
to day/month combinations in other calendars, values of this type do not,
in general, have any straightforward or intuitive representation
in terms of most other calendars. This type should therefore be
used with caution in contexts where conversion to other calendars
is desired.

<a id="gMonthDay-lexical-repr"></a>

##### 3.2.12.1 Lexical representation

The lexical representation for **gMonthDay** is the left
truncated lexical representation for [date](010-3-2-7-5-totally-ordered-datetimes.md#date): --MM-DD.
An optional following time
zone qualifier is allowed as for [date](010-3-2-7-5-totally-ordered-datetimes.md#date).
No preceding sign is allowed. No other formats are allowed. See also [ISO 8601 Date and Time Formats (§D)](037-d-iso-8601-date-and-time-formats.md#isoformats).

This datatype can be used to represent a specific day in a month.
To say, for example, that my birthday occurs on the 14th of September ever year.

<a id="gMonthDay-facets"></a>

##### 3.2.12.2 Constraining facets

**gMonthDay** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="gDay"></a>

#### 3.2.13 gDay

<a id="dt-gDay"></a>

[Definition:]
**gDay** is a gregorian day that recurs, specifically a day
of the month such as the 5th of the month. Arbitrary recurring days
are not supported by this datatype. The [·value space·](003-1-4-terminology.md#dt-value-space)
of **gDay** is the space of a set of *calendar
dates* as defined in § 3 of [[ISO 8601]](044-h-references.md#ISO8601). Specifically,
it is a set of one-day long, monthly periodic instances.

This datatype can be used to represent a specific day of the month.
To say, for example, that I get my paycheck on the 15th of each month.

Since the lexical representation allows an optional time zone
indicator, **gDay** values are partially ordered because it may
not be possible to unequivocally determine the order of two values one of
which has a time zone and the other does not. If
**gDay** values are considered as periods of time,
in an arbitrary month that has 31 days,
the order relation
on **gDay** values is the order relation on their starting instants.
This is discussed in [Order relation on dateTime (§3.2.7.4)](009-3-2-6-3-facet-comparison-for-durations.md#dateTime-order). See also
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes). Pairs of **gDay**
values with or without time zone indicators are totally ordered.

**Note:**
Because days in one calendar only rarely correspond
to days in other calendars, values of this type do not,
in general, have any straightforward or intuitive representation
in terms of most other calendars. This type should therefore be
used with caution in contexts where conversion to other calendars
is desired.

<a id="gDay-lexical-repr"></a>

##### 3.2.13.1 Lexical representation

The lexical representation for **gDay** is the left
truncated lexical representation for [date](010-3-2-7-5-totally-ordered-datetimes.md#date): ---DD .
An optional following time
zone qualifier is allowed as for [date](010-3-2-7-5-totally-ordered-datetimes.md#date). No preceding sign is
allowed. No other formats are allowed. See also [ISO 8601 Date and Time Formats (§D)](037-d-iso-8601-date-and-time-formats.md#isoformats).

<a id="gDay-facets"></a>

##### 3.2.13.2 Constraining facets

**gDay** has the
following [·constraining facets·](004-2-3-lexical-space.md#dt-constraining-facet):

- [pattern](026-4-3-3-maxlength.md#rf-pattern)
- [enumeration](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-enumeration)
- [whiteSpace](027-4-3-4-3-constraints-on-xml-representation-of-pattern.md#rf-whiteSpace)
- [maxInclusive](028-4-3-6-1-the-whitespace-schema-component.md#rf-maxInclusive)
- [maxExclusive](029-4-3-7-3-maxinclusive-validation-rules.md#rf-maxExclusive)
- [minInclusive](031-4-3-10-mininclusive.md#rf-minInclusive)
- [minExclusive](030-4-3-9-minexclusive.md#rf-minExclusive)

<a id="gMonth"></a>

#### 3.2.14 gMonth

<a id="dt-gMonth"></a>

[Definition:]
**gMonth** is a gregorian month that recurs every year.
The [·value space·](003-1-4-terminology.md#dt-value-space)
of **gMonth** is the space of a set of *calendar
months* as defined in § 3 of [[ISO 8601]](044-h-references.md#ISO8601). Specifically,
it is a set of one-month long, yearly periodic instances.

This datatype can be used to represent a specific month.
To say, for example, that Thanksgiving falls in the month of November.

Since the lexical representation allows an optional time zone
indicator, **gMonth** values are partially ordered because it may
not be possible to unequivocally determine the order of two values one of which has a
time zone and the other does not. If
**gMonth** values are considered as periods of time, the order relation
on **gMonth** is the order relation on their starting instants.
This is discussed in [Order relation on dateTime (§3.2.7.4)](009-3-2-6-3-facet-comparison-for-durations.md#dateTime-order). See also
[Adding durations to dateTimes (§E)](038-d-2-truncated-and-reduced-formats.md#adding-durations-to-dateTimes). Pairs of **gMonth**
values with or without time zone indicators are totally ordered.

**Note:**
Because months in one calendar only rarely correspond
to months in other calendars, values of this type do not,
in general, have any straightforward or intuitive representation
in terms of most other calendars. This type should therefore be
used with caution in contexts where conversion to other calendars
is desired.

<a id="gMonth-lexical-repr"></a>

##### 3.2.14.1 Lexical representation

The lexical representation for **gMonth** is the left
and right truncated lexical representation for [date](010-3-2-7-5-totally-ordered-datetimes.md#date): --MM.
An optional following time
zone qualifier is allowed as for [date](010-3-2-7-5-totally-ordered-datetimes.md#date). No preceding sign is
allowed. No other formats are allowed. See also [ISO 8601 Date and Time Formats (§D)](037-d-iso-8601-date-and-time-formats.md#isoformats).

<a id="gMonth-facets"></a>

##### 3.2.14.2 Constraining facets

