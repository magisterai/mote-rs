# 9 Implementation Notes

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](020-7-deleted.md) · [Next](022-11-history.md)

<a id="ImplementationNotes"></a>

## 9  [Implementation Notes](021-9-implementation-notes.md#ImplementationNotes)

This section provides additional notes on implementation issues.

<a id="RegExCombining"></a>

### 9.1  [Combining Marks in Regular Expression-Based Implementations](021-9-implementation-notes.md#RegExCombining)

Implementations that use regular expressions cannot directly
express rules [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9) and [LB10](018-6-1-non-tailorable-line-breaking-rules.md#LB10).
However, it is possible to make these rules unnecessary by rewriting *all*
the rules from [LB11](018-6-1-non-tailorable-line-breaking-rules.md#LB11) on down so that the
overall result of the algorithm is unchanged. This restatement of the rules is
therefore not a tailoring, but rather an equivalent statement of the algorithm
that can be directly expressed as regular expressions.

To replace rule [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), terms of
the form

**B** # **A**

**B** SP\* # **A**

**B** #

**B** SP\* #

are replaced by terms of the form

**B** CM\* # **A**

**B** CM\* SP\* # **A**

**B** CM\* #

**B** CM\* SP\* #

where **B** and **A** are any line break class or set of
alternate line break classes, such as (X |Y), and where # is any of the three
operators !, ÷, or ×.

Note that because **sot**, [BK](008-bb-break-before.md#BK), [CR](010-cp-closing-parenthesis.md#CR), [LF](012-hl-hebrew-letter.md#LF), [NL](012-hl-hebrew-letter.md#NL), and [ZW](016-zw-zero-width-space-non-tailorable.md#ZW) are all handled by rules
above [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), these classes cannot occur in
position **B** in any rule that is rewritten as shown here.

Replace [LB10](018-6-1-non-tailorable-line-breaking-rules.md#LB10)
by the following rule:

× CM

For each rule containing AL on its left side,
add a rule that is identical except for the replacement of AL by CM, but taking
care of correctly handling sets of alternate line break classes. For example,
for rule

(AL | NU) × OP

add another rule

CM × OP.

These prescriptions for rewriting the rules are, in
principle, valid
even where the rules have been tailored as permitted in *Section 4,
[Conformance](004-3-introduction.md#Conformance)*. However, for extended context rules
such as in [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25), additional
considerations apply. These are described in *Section 6.2, Replacing Ignore Rules*, of
Unicode Standard Annex #29, “Unicode Text Segmentation” [[UAX29](https://www.unicode.org/reports/tr41/tr41-36.html#UAX29)].

<a id="LegacySpace"></a>

### 9.2  [Legacy Support for Space Character as Base for Combining Marks](021-9-implementation-notes.md#LegacySpace)

As stated in *Section 7.9, Combining Marks* of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)],
combining characters are shown in isolation by applying them to U+00A0 NO-BREAK SPACE (NBSP).
In earlier versions, this recommendation included the use of U+0020 SPACE.
The use of SPACE for this purpose has been deprecated
because it leads to many complications in text processing. The visual appearance is the same with
both NO-BREAK SPACE and SPACE,
but the line breaking behavior is different.
Under the current rules, [SP](014-qu-quotation.md#SP) [CM\*](009-cb-contingent-break-opportunity.md#CM)
will allow a break between [SP](014-qu-quotation.md#SP) and
[CM\*](009-cb-contingent-break-opportunity.md#CM), which could result in a new line starting with a combining mark. Previously,
whenever the base character was [SP](014-qu-quotation.md#SP), the sequences
[CM\*](009-cb-contingent-break-opportunity.md#CM)
and [SP](014-qu-quotation.md#SP) [CM\*](009-cb-contingent-break-opportunity.md#CM)
were defined to act like indivisible clusters,
allowing breaks on either side like [ID](011-h2-hangul-lv-syllable.md#ID).

Where backward compatibility with documents created under the prior
practice is desired, the following tailoring should be applied to those
[CM](009-cb-contingent-break-opportunity.md#CM) characters that have a General\_Category value of Combining\_Mark (M):

*Legacy-CM: In all of the rules following rule [LB8](018-6-1-non-tailorable-line-breaking-rules.md#LB8),
if a space is the base character for a
combining mark, the space is changed to type  [ID](011-h2-hangul-lv-syllable.md#ID).
In other words, break before [SP](014-qu-quotation.md#SP)
in the same cases as one would break before an  [ID](011-h2-hangul-lv-syllable.md#ID).*

Treat [SP](014-qu-quotation.md#SP) [CM\*](009-cb-contingent-break-opportunity.md#CM)
as if it were [ID](011-h2-hangul-lv-syllable.md#ID).

While this tailoring changes the location of the line break
opportunities in the string, it is ordinarily not expected to affect the display of
the text. That is because spaces at the end of the line are normally
invisible and the recommended display for isolated combining marks is the
same as if they were applied to a preceding SPACE
or NBSP.

<a id="Testing"></a>

## 10  [Testing](021-9-implementation-notes.md#Testing)

As with the other default specifications,
implementations are free to override (tailor) the results to meet the
requirements of different environments or particular languages as described in
*Section 4, [Conformance](004-3-introduction.md#Conformance)*. For those
who do implement the default breaks as specified in this annex and wish
to check that their implementation matches that specification, a
test file has been made available in [[Tests14](https://www.unicode.org/reports/tr41/tr41-36.html#Tests14)].

These tests cannot be exhaustive, because of the large number of possible
combinations; but they do provide samples that test all pairs of property
values, using a representative character for each value, plus certain other
sequences.

A sample HTML file is also available for each that shows various
combinations in chart form, in [[Charts14](https://www.unicode.org/reports/tr41/tr41-36.html#Charts14)].
The header cells of the chart
consist of a property value, followed by a representative code point number.
The body cells in the chart show the break status: whether a break occurs
between the row property value and the column property value. If the browser
supports tool-tips, then hovering the mouse over the code point number will
show the character name, General\_Category and Script property
values. Hovering over the break status will display the number of the rule
responsible for that status.

> Note: To determine a break it is generally not sufficient to
> just test the two adjacent characters.

The chart is followed by some test cases. These test cases consist of
various strings with the break status between each pair of characters shown
by blue lines for breaks and by whitespace for non-breaks. Hovering over
each character (with tool-tips enabled) shows the character name and
property value; hovering over the break status shows the number of the rule
responsible for that status.

Due to the way they have been mechanically processed for generation, the
test rules do not match the rules in this annex precisely. In particular:

1. The rules are cast into a more regex-style.
2. The rules “sot”, “eot”, and “Any” are added
   mechanically and have artificial numbers.
3. The rules are given decimal numbers without prefixes,
   so rules such as LB14 are given a number using tenths, such as 14.0.
4. Where a rule has multiple parts (lines), each one is numbered using
   hundredths, such as
   - 13.01) [^NU] × CL
   - 13.02) × EX
   - ...

The mapping from the rule numbering in this annex to the numbering for
the test rules is summarized in *Table 4*.

<a id="Table4"></a>

Table 4. [Numbering of Test Rules](021-9-implementation-notes.md#Table4)

| Rule in This Annex | Test Rule | Comment |
| --- | --- | --- |
| LB2 | 0.2 | start of text |
| LB3 | 0.3 | end of text |
| LB12a | 12.0 | GL × |
| LB12b | 12.1 | [^SP, BA, HY] × GL |
| LB31 | 999 | ÷ any |

