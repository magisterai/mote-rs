# E.1 Algorithm

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](038-d-2-truncated-and-reduced-formats.md) · [Next](040-f-1-character-classes.md)

<a id="d0e11648"></a>

### E.1 Algorithm

Essentially, this calculation is equivalent to separating D into <year,month>
and <day,hour,minute,second> fields. The <year,month> is added to S.
If the day is out of range, it is *pinned* to be within range. Thus April
31 turns into April 30. Then the <day,hour,minute,second> is added. This
latter addition can cause the year and month to change.

Leap seconds are handled by the computation by treating them as overflows.
Essentially, a value of 60
seconds in S is treated as if it were a duration of 60 seconds added to S
(with a zero seconds field). All calculations
thereafter use 60 seconds per minute.

Thus the addition of either PT1M or PT60S to any dateTime will always
produce the same result. This is a special definition of addition which
is designed to match common practice, and -- most importantly -- be stable
over time.

A definition that attempted to take leap-seconds into account would need to
be constantly updated, and could not predict the results of future
implementation's additions. The decision to introduce a leap second in UTC
is the responsibility of the [[International Earth Rotation Service (IERS)]](044-h-references.md#IERS). They make periodic
announcements as to when
leap seconds are to be added, but this is not known more than a year in
advance. For more information on leap seconds, see [[U.S. Naval Observatory Time Service Department]](044-h-references.md#USNavy).

The following is the precise specification. These steps must be followed in
the same order. If a field in D is not specified, it is treated as if it were
zero. If a field in S is not specified, it is treated in the calculation as if
it were the minimum allowed value in that field, however, after the calculation
is concluded, the corresponding field in E is removed (set to unspecified).

- *Months (may be modified additionally below)*
  - temp := S[month] + D[month]
  - E[month] := modulo(temp, 1, 13)
  - carry := fQuotient(temp, 1, 13)
- *Years (may be modified additionally below)*
  - E[year] := S[year] + D[year] + carry
- *Zone*
  - E[zone] := S[zone]
- *Seconds*
  - temp := S[second] + D[second]
  - E[second] := modulo(temp, 60)
  - carry := fQuotient(temp, 60)
- *Minutes*
  - temp := S[minute] + D[minute] + carry
  - E[minute] := modulo(temp, 60)
  - carry := fQuotient(temp, 60)
- *Hours*
  - temp := S[hour] + D[hour] + carry
  - E[hour] := modulo(temp, 24)
  - carry := fQuotient(temp, 24)
- *Days*
  - if S[day] > maximumDayInMonthFor(E[year], E[month])
    - tempDays := maximumDayInMonthFor(E[year], E[month])
  - else if S[day] < 1
    - tempDays := 1
  - else
    - tempDays := S[day]
  - E[day] := tempDays + D[day] + carry
  - **START LOOP**
    - **IF** E[day] < 1
      - E[day] := E[day] + maximumDayInMonthFor(E[year], E[month] - 1)
      - carry := -1
    - **ELSE IF** E[day] > maximumDayInMonthFor(E[year], E[month])
      - E[day] := E[day] - maximumDayInMonthFor(E[year], E[month])
      - carry := 1
    - **ELSE EXIT LOOP**
    - temp := E[month] + carry
    - E[month] := modulo(temp, 1, 13)
    - E[year] := E[year] + fQuotient(temp, 1, 13)
    - **GOTO START LOOP**

*Examples:*

<table border="1" cellpadding="4" cellspacing="0"><tbody><tr><th align="center" style="background-color: #FFFF99">dateTime</th><th align="center" style="background-color: #FFFF99">duration</th><th align="center" style="background-color: #FFFF99">result</th></tr><tr><td align="center">2000-01-12T12:13:14Z</td><td align="center">P1Y3M5DT7H10M3.3S</td><td align="center">2001-04-17T19:23:17.3Z</td></tr><tr><td align="center">2000-01</td><td align="center">-P3M</td><td align="center">1999-10</td></tr><tr><td align="center">2000-01-12</td><td align="center">PT33H</td><td align="center">2000-01-13</td></tr></tbody></table>

<a id="adding-durations-to-instants-commutativity-associativity"></a>

### E.2 Commutativity and Associativity

Time durations are added by simply adding each of their fields, respectively,
without overflow.

The order of addition of durations to instants *is* significant.
For example, there are cases where:

((dateTime + duration1) + duration2) != ((dateTime +
duration2) + duration1)

*Example:*

(2000-03-30 + P1D) + P1M = 2000-03-31 + P1M = 2000-**04-30**

(2000-03-30 + P1M) + P1D = 2000-04-30 + P1D = 2000-**05-01**

<a id="regexs"></a>

## F Regular Expressions

A [·regular expression·](039-e-1-algorithm.md#dt-regex) *R* is a sequence of
characters that denote a **set of strings**  *L(R)*.
When used to constrain a [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space), a
**regular expression**  *R* asserts that only strings
in *L(R)* are valid literals for values of that type.

**Note:**
Unlike some popular regular expression languages (including those
defined by Perl and standard Unix utilities), the regular
expression language defined here implicitly anchors all regular
expressions at the head and tail, as the most common use of
regular expressions in [·pattern·](026-4-3-3-maxlength.md#dt-pattern) is to match entire literals.
For example, a datatype [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from [string](006-2-5-2-3-derived-by-union.md#string) such
that all values must begin with the character `A` (#x41) and end with the character
`Z` (#x5a) would be defined as follows:



```
<simpleType name='myString'>
 <restriction base='string'>
  <pattern value='A.*Z'/>
 </restriction>
</simpleType>
```




In regular expression languages that are not implicitly anchored at the head and tail,
it is customary to write the equivalent regular expression as:`^A.*Z$`where "^" anchors the pattern at the head and "$" anchors at the tail.In those rare cases where an unanchored match is desired, including
`.*` at the beginning and ending of the regular expression will
achieve the desired results. For example, a datatype [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) from string such that all values must contain at least 3 consecutive `A` (`#x41`) characters somewhere within the value could be defined as follows:



```
<simpleType name='myString'>
 <restriction base='string'>
  <pattern value='.*AAA.*'/>
 </restriction>
</simpleType>
```




<a id="dt-regex"></a>

[Definition:]A
**regular expression** is composed from zero or more
[·branch·](039-e-1-algorithm.md#dt-branch)es, separated by `|` characters.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Regular Expression</font></strong></td></tr><tr><td><a id="regex"></a><a id="nt-regExp"></a><table border="0"><tbody><tr valign="baseline"><td><a id="regex" name="regex"></a><a id="nt-regExp" name="nt-regExp"></a>[1]   </td><td><code>regExp</code></td><td>   ::=   </td><td><code>
<a href="039-e-1-algorithm.md#nt-branch">branch</a>
			( '|' <a href="039-e-1-algorithm.md#nt-branch">branch</a> )*
		</code></td></tr></tbody></table></td></tr></tbody></table>

| For all [·branch·](039-e-1-algorithm.md#dt-branch)es *S*, and for all [·regular expression·](039-e-1-algorithm.md#dt-regex)s *T*, valid [·regular expression·](039-e-1-algorithm.md#dt-regex)s *R* are: | Denoting the set of strings *L(R)* containing: |
| --- | --- |
| (empty string) | the set containing just the empty string |
| *S* | all strings in *L(S)* |
| *S*|*T* | all strings in *L(S)* and all strings in *L(T)* |

<a id="dt-branch"></a>

[Definition:]
A **branch** consists
of zero or more [·piece·](039-e-1-algorithm.md#dt-piece)s, concatenated together.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Branch</font></strong></td></tr><tr><td><a id="branch"></a><a id="nt-branch"></a><table border="0"><tbody><tr valign="baseline"><td><a id="branch" name="branch"></a><a id="nt-branch" name="nt-branch"></a>[2]   </td><td><code>branch</code></td><td>   ::=   </td><td><code><a href="039-e-1-algorithm.md#nt-piece">piece</a>*</code></td></tr></tbody></table></td></tr></tbody></table>

| For all [·piece·](039-e-1-algorithm.md#dt-piece)s *S*, and for all [·branch·](039-e-1-algorithm.md#dt-branch)es *T*, valid [·branch·](039-e-1-algorithm.md#dt-branch)es *R* are: | Denoting the set of strings *L(R)* containing: |
| --- | --- |
| *S* | all strings in *L(S)* |
| *S**T* | all strings *st* with *s* in *L(S)* and *t* in *L(T)* |

<a id="dt-piece"></a>

[Definition:]
A **piece** is an
[·atom·](040-f-1-character-classes.md#dt-atom), possibly followed by a
[·quantifier·](039-e-1-algorithm.md#dt-quantifier).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Piece</font></strong></td></tr><tr><td><a id="piece"></a><a id="nt-piece"></a><table border="0"><tbody><tr valign="baseline"><td><a id="piece" name="piece"></a><a id="nt-piece" name="nt-piece"></a>[3]   </td><td><code>piece</code></td><td>   ::=   </td><td><code><a href="040-f-1-character-classes.md#nt-atom">atom</a> <a href="040-f-1-character-classes.md#nt-quantifier">quantifier</a>?</code></td></tr></tbody></table></td></tr></tbody></table>

| For all [·atom·](040-f-1-character-classes.md#dt-atom)s *S* and non-negative integers *n*, *m* such that *n <= m*, valid [·piece·](039-e-1-algorithm.md#dt-piece)s *R* are: | Denoting the set of strings *L(R)* containing: |
| --- | --- |
| *S* | all strings in *L(S)* |
| *S*? | the empty string, and all strings in *L(S)*. |
| *S*\* | All strings in *L(S?)* and all strings *st* with *s* in *L(S\*)* and *t* in *L(S)*. *( all concatenations of zero or more strings from L(S) )* |
| *S*+ | All strings *st* with *s* in *L(S)* and *t* in *L(S\*)*. *( all concatenations of one or more strings from L(S) )* |
| *S*{n,m} | All strings *st* with *s* in *L(S)* and *t* in *L(S{n-1,m-1})*. *( All sequences of at least n, and at most m, strings from L(S) )* |
| *S*{n} | All strings in *L(S{n,n})*. *( All sequences of exactly n strings from L(S) )* |
| *S*{n,} | All strings in L(S{n}S\*) *( All sequences of at least n, strings from L(S) )* |
| *S*{0,m} | All strings *st* with *s* in *L(S?)* and *t* in *L(S{0,m-1})*. *( All sequences of at most m, strings from L(S) )* |
| *S*{0,0} | The set containing only the empty string |

**Note:**
The regular expression language in the Perl Programming Language
[[Perl]](044-h-references.md#Perl) does not include a quantifier of the form
`S{,m}`, since it is logically equivalent to `S{0,m}`.
We have, therefore, left this logical possibility out of the regular
expression language defined by this specification.

<a id="dt-quantifier"></a>

[Definition:]
A **quantifier**
is one of `?`, `*`, `+`,
`{n,m}` or `{n,}`, which have the meanings
defined in the table above.

