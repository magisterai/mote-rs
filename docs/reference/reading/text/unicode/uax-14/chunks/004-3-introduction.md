# 3 Introduction

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-continuation-3.md) · [Next](005-4-1-conformance-requirements.md)

<a id="Introduction"></a>

## 3  [Introduction](004-3-introduction.md#Introduction)

Lines are broken as the result of two conditions.
The first is the presence of a mandatory line breaking character. The second
condition results from a formatting algorithm having selected among available
line break opportunities; ideally the chosen line break results in the optimal
layout of the text.

Different formatting algorithms may use different methods to determine an
optimal line break. For example, simple implementations consider a single line at a
time, trying to find a *locally optimal* line break. A basic, yet widely
used approach is to
allow no compression or expansion of the intercharacter and interword spaces
and consider the longest line that fits. More complex formatting algorithms
often take into account the interaction of line
breaking decisions for the whole paragraph. The well-known text layout system
[[T<sub>E</sub>X](https://www.unicode.org/reports/tr41/tr41-36.html#TeX)] implements an
example of such a *globally
optimal* strategy that may make complex tradeoffs across an entire
paragraph to avoid unnecessary
hyphenation and other legal, but inferior breaks. For a description of this
strategy, see [[Knuth78](https://www.unicode.org/reports/tr41/tr41-36.html#Knuth78)].

When compression or expansion is allowed,
a locally optimal line break seeks to balance the relative merits
of the resulting amounts of compression and expansion for different line break
candidates. When expanding or compressing interword space according to common
typographical practice, only the spaces marked by
U+0020 SPACE and U+00A0 NO-BREAK SPACE are subject
to compression, and only spaces marked by U+0020 SPACE,
U+00A0 NO-BREAK SPACE,
and occasionally spaces marked by U+2009 THIN SPACE
are subject to expansion. All other space characters normally have
fixed width. When expanding or compressing intercharacter space, the presence
of U+200B ZERO WIDTH SPACE or U+2060 WORD JOINER is always ignored.

Local custom or document style determines whether and to what degree expansion of
intercharacter space
is allowed in justifying a line. In languages, such as German, where
intercharacter space is commonly used to mark e m p h a s i s
(like this), allowing variable intercharacter spacing would
have the unintended effect of adding random emphasis, and is therefore best
avoided. In table headings that use Han ideographs, even extreme
amounts of intercharacter space commonly occur as short texts are spread out
across the entire available space to distribute the characters evenly from end
to end.

In line breaking it is necessary to distinguish
between three related tasks.
The first is the determination of all legal line
break opportunities, given a string of text. This is the scope of the
Unicode Line Breaking Algorithm. The second task is the selection of the actual
location for breaking a given line of text. This selection not only takes
into account the width of the line compared to the width of the text, but
may also apply an additional prioritization of line breaks based on
aesthetic and other criteria. What defines an optimal choice for a given
line break is outside the scope of this annex, as are methods for its
selection. The third is the possible justification of lines,
once actual locations for line breaking have been determined, and is also
out of scope for the Unicode Line Breaking Algorithm.

Finally, text layout systems may support an emergency mode that
handles the case of an unusual line that contains no
otherwise permitted line break
opportunities. In such line layout emergencies, line breaks may be placed with
no regard to the ordinary line breaking behavior of the characters involved.
The details of such an emergency mode are outside
the scope of this annex, however, it is recommended that grapheme clusters
be kept together.

<a id="BreakOpportunities"></a>

### 3.1  [Determining Line Break Opportunities](004-3-introduction.md#BreakOpportunities)

Four principal styles of context analysis determine line break
opportunities.

1. *Western:* spaces and hyphens are used to determine breaks
2. *East Asian:* lines can break anywhere, unless prohibited
3. *South East Asian:* line breaks require morphological analysis
4. *Brahmic:* line breaks can occur at the boundaries of any orthographic syllable

The Western style is commonly used for scripts employing the space character.
Hyphenation is often used with space-based line breaking to provide additional
line break opportunities—however, it requires knowledge of the language and
it may need user interaction or overrides.

The second style of context analysis is used with East Asian ideographic and
syllabic scripts. In these scripts, lines can break anywhere, except
before or after certain characters. The precise set of prohibited line
breaks may depend on user preference or local custom and is commonly
tailorable.

Korean makes use of both styles of line break. When Korean text is justified, the second style is
commonly used, even for interspersed Latin letters. But when ragged margins
are used, the Western style (relying on spaces) is commonly used instead, even
for ideographs.

The third style is used for scripts such as Thai, which allow
line breaks only at word boundaries, but
do not mark word boundaries in any way, so that the determination of line
break opportunities requires language dependent text analysis. Algorithms
and data for such analysis are beyond the scope of the Unicode
Standard.

The fourth style is used in some Brahmic scripts, such as Brahmi, Balinese, or Javanese, which allow line breaks to occur at the boundaries of any orthographic syllable, without restricting them to word boundaries.
This style is only supported for scripts that encode orthographic syllables in primarily phonetic order.

For multilingual text, the Western, East Asian, and Brahmic styles can be unified into a single set
of specifications, based on the information in this annex. Unicode characters have explicit line breaking properties assigned to them.
These properties can be utilized to implement the effect of both of these two styles of context analysis for line break
opportunities. Customization for user preferences or document style can
then be achieved by tailoring that specification.

In bidirectional text, line breaks are determined before applying rule L1 of the Unicode Bidirectional
Algorithm [[UAX9](https://www.unicode.org/reports/tr41/tr41-36.html#UAX9)].
However, line breaking is strictly independent of directional properties of
the characters or of any auxiliary information determined by the application
of rules of that algorithm.

<a id="Conformance"></a>

## 4  [Conformance](004-3-introduction.md#Conformance)

There is no single method for determining line breaks; the rules may
differ based on user preference and document layout. The information
in this annex, including the specification of the line breaking algorithm,
allows for the necessary flexibility in determining line breaks according to
different conventions. However, some characters
have been encoded explicitly for their effect on line breaking.
Because users adding such characters to a text expect that they will have
the desired effect, these characters have been given required line breaking behavior.

To handle certain situations, some line
breaking implementations use techniques that cannot be expressed within the
framework of the Unicode Line Breaking Algorithm. Examples
include using dictionaries of words for languages that do not use
spaces, such as Thai; recognition of the language
of the text in order to choose among different punctuation conventions;
using dictionaries of common abbreviations or contractions to resolve
ambiguities with periods or apostrophes; or a deeper analysis of common
syntaxes for numbers or dates, and so on. The conformance requirements permit variations of this kind.

Processes which support multiple modes for
determining line breaks are also accommodated. This situation can arise with
marked-up text, rich text, style sheets, or other environments in which a
higher-level protocol can carry formatting instructions that prevent or
force line breaks in positions that differ from those specified by the
Unicode Line Breaking Algorithm. The approach taken here
requires that such processes have a conforming default line break behavior, and to
disclose that they also include
overrides or optional behaviors that are invoked via a higher-level protocol.

The methods by which a line layout process
chooses optimal line breaks from among the available break opportunities is
outside the scope of this specification. The behavior of a line layout
process in situations where there are no suitable break opportunities is
also outside of the scope of this specification.

Note:
Locale-sensitive line break specifications can be expressed in LDML [[UTS35](https://www.unicode.org/reports/tr41/tr41-36.html#UTS35)].
Tailorings are available in the Common Locale Data Repository [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)].

