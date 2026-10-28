# 4.1 Conformance Requirements

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-3-introduction.md) · [Next](006-5-1-description-of-line-breaking-properties.md)

<a id="ConfRequirements"></a>

### 4.1  [Conformance Requirements](005-4-1-conformance-requirements.md#ConfRequirements)

<a id="UAX14-C1"></a>

***[UAX14-C1](005-4-1-conformance-requirements.md#UAX14-C1)**. A process that determines line breaks in
Unicode text, and that purports to implement the Unicode Line Breaking
Algorithm, shall do so in accordance with the specifications in this annex.
In particular, the following three subconditions shall be met:*

1. *The sets of mandatory break positions and of break opportunities which
   the implementation produces include all of those specified by the rules in
   Section 6.1,
   [Non-tailorable Line Breaking Rules](018-6-1-non-tailorable-line-breaking-rules.md#BreakingRules).*
2. *There exist no break opportunities or mandatory breaks produced by the
   implementation that fall on a "non-break" position specified by the rules in
   Section 6.1,
   [Non-tailorable Line Breaking Rules](018-6-1-non-tailorable-line-breaking-rules.md#BreakingRules).*
3. *If the implementation tailors the behavior
   of Section 6.2,
   [Tailorable Line Breaking Rules](019-6-2-tailorable-line-breaking-rules.md#TailorableBreakingRules),
   that fact must be disclosed.*

<a id="UAX14-C2"></a>

***[UAX14-C2](005-4-1-conformance-requirements.md#UAX14-C2)**. If an implementation
has a default line breaking operation which conforms to [UAX14-C1](005-4-1-conformance-requirements.md#UAX14-C1), but also
has overrides based on a higher-level protocol, that fact must be disclosed
and any behavior that differs from that specified by the rules of Section 6.1,
[Non-tailorable Line Breaking Rules](018-6-1-non-tailorable-line-breaking-rules.md#BreakingRules),
must be documented.*

> Example: An XML format provides markup which disables all line breaking over
> some span of text. When the markup is not in place, the default behavior is
> in conformance according to [UAX14-C1](005-4-1-conformance-requirements.md#UAX14-C1). As long as the existence of the option
> is disclosed, that format can be said to conform to the Unicode
> Line Breaking Algorithm according to [UAX14-C2](005-4-1-conformance-requirements.md#UAX14-C2).

As is the case for all other Unicode
algorithms, this specification is a logical description—particular
implementations can have more efficient mechanisms as long as they produce
the same results. See C18 in *Chapter 3, Conformance*, of
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].
While only disclosure of tailorings is required in the conformance clauses,
documentation of the differences in behaviors is strongly encouraged.

<a id="Properties"></a>

## 5  [Line Breaking Properties](005-4-1-conformance-requirements.md#Properties)

This section provides detailed narrative descriptions of
the line breaking behavior of many Unicode characters.
Many descriptions in this section provide additional
informative detail about handling a given character at the end of a line, or during
line layout, which goes beyond the simple
determination of line breaks. In some cases, the
text also gives guidance as to preferred characters for achieving a particular
effect in line breaking.

This section also summarizes the membership of character
classes corresponding to each value of the line breaking property. Note
that the mnemonic names for the line break classes are intended neither as
exhaustive descriptions of their membership nor as indicators of their
entire range of behaviors in the line breaking process. Instead, their main
purpose is to serve as unique, yet broadly mnemonic labels. In other words,
as long as their line breaking behavior is identical, otherwise unrelated
characters will be grouped together in the same line break class.

The classification
by property values defined in this section and in the data file is used as input
into the algorithm defined in
*Section 6, [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)*.
That section describes
a workable default line breaking method.
*Section 8, [Customization](020-7-deleted.md#Customization)*,
discusses how the default line breaking behavior can be tailored to the
needs of specific languages or for particular document styles and user preferences.
Permitted customizations can include changing the classification
of characters for certain classes.

In addition to the line breaking properties defined in this section,
the algorithm defined in *Section 6, [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)* also makes use of
East\_Asian\_Width property values, defined in
Unicode Standard Annex #11, *East Asian Width* [[UAX11](https://www.unicode.org/reports/tr41/tr41-36.html#UAX11)],
as well as the General\_Category and Extended\_Pictographic properties.
Note that for purposes of the line breaking algorithm, those property
values are tailorable, as are the rules of the line breaking algorithm which use them.
(See rules
[LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a),
[LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b),
[LB19](019-6-2-tailorable-line-breaking-rules.md#LB19),
[LB19a](019-6-2-tailorable-line-breaking-rules.md#LB19a),
[LB21a](019-6-2-tailorable-line-breaking-rules.md#LB21a),
[LB30](019-6-2-tailorable-line-breaking-rules.md#LB30),
and [LB30b](019-6-2-tailorable-line-breaking-rules.md#LB30b).)

#### Data File

The full classification of all Unicode characters by their line breaking
properties is available in
the file LineBreak.txt [[Data14](https://www.unicode.org/reports/tr41/tr41-36.html#Data14)] in the Unicode
Character Database [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)].
This is a semicolon-delimited,
two-column, plain text file, with code position and line breaking class. A
comment at the end of each line indicates the character name.

The same data, but with a more explicit listing of
code point ranges with complex default values, is available in the file
DerivedLineBreak.txt [[Data14Derived](https://www.unicode.org/reports/tr41/tr41-36.html#Data14Derived)].

The line break property assignments from the data file are normative. The descriptions
of the line break classes in this UAX include examples of representative or interesting characters
for each class, but for the complete list always refer to the data file.

#### Future Updates

As scripts are added to the Unicode Standard and become more widely implemented,
line breaking classes may be added or the assignment of line breaking class may be changed for some characters.
Implementers must not make any assumptions to the contrary.
Any future updates will be reflected in the
[latest version](https://www.unicode.org/Public/UCD/latest/ucd/LineBreak.txt) of the data file.
(See the [Unicode Character Database](https://www.unicode.org/ucd/)
[[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)] for any specific version of the data file.)

