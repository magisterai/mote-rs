# ** ZW**: Zero Width Space (Non-tailorable)

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](015-sy-symbols-allowing-break-after.md) · [Next](017-5-5-use-of-double-hyphen.md)

<a id="ZW"></a>

### ** [ZW](016-zw-zero-width-space-non-tailorable.md#ZW)**: Zero Width Space (Non-tailorable)

*[LB7](018-6-1-non-tailorable-line-breaking-rules.md#LB7), [LB8](018-6-1-non-tailorable-line-breaking-rules.md#LB8), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

|  |  |
| --- | --- |
| 200B | ZERO WIDTH SPACE (ZWSP) |

This character is used to enable additional
(invisible) break opportunities wherever SPACE
cannot be used. As its name implies, it normally has no width. However,
its presence between two characters does not prevent increased letter
spacing in justification.

<a id="ZWJ"></a>

### ** [ZWJ](016-zw-zero-width-space-non-tailorable.md#ZWJ)**: Zero Width Joiner (Non-tailorable)

*[LB8a](018-6-1-non-tailorable-line-breaking-rules.md#LB8a), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB10](018-6-1-non-tailorable-line-breaking-rules.md#LB10)*

|  |  |
| --- | --- |
| 200D | ZERO WIDTH JOINER (ZWJ) |

A ZWJ prevents breaks between most pairs of characters that would otherwise break. It
has various uses, including as a connector in emoji zwj sequences
and as a joiner in complex scripts.

Emoji zwj sequences are defined by
*ED-16, emoji zwj sequence*, in [[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)]
and implemented for line breaking by rule [LB8a](018-6-1-non-tailorable-line-breaking-rules.md#LB8a).
In other respects, the line breaking behavior of ZWJ is that of a combining character of class
[CM](009-cb-contingent-break-opportunity.md#CM).

<a id="Dictionary"></a>

### 5.2  [Dictionary Usage](016-zw-zero-width-space-non-tailorable.md#Dictionary)

Dictionaries follow specific conventions that guide their use of special characters to
indicate features of the terms they list. Marks used for some of these
conventions may occur near line break opportunities and therefore interact
with line breaking. For example, in one dictionary a natural hyphen in a
word becomes a tilde dash when the word is split.
Section 6.2.8, *Hyphenation Point and Dictionary Syllabification*,
of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)] illustrates the use
of marks whose line breaking classes have been assigned to accomodate various
dictionary usages.

<a id="Hyphen"></a>

### 5.3  [Use of Hyphen](016-zw-zero-width-space-non-tailorable.md#Hyphen)

The rules for treating hyphens in line breaking
vary by language. In many instances, these rules are not supported as such in the
algorithm, but the correct appearance can be realized by using a*non-breaking hyphen.*

Some languages and some transliteration systems
use a hyphen at the first position in a word. For example, the Finnish
orthography uses a hyphen at the start of a word in certain types of
compounds of the form xxx yyy -zzz (where xxx yyy is a two-word expression
that acts as the first part of a compound noun, with zzz as the second
part). Line break after the hyphen is not allowed here by
rule [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a).

There are line breaking conventions that
modify the appearance of a line break when the line break opportunity is
based on an explicit hyphen. In
standard Polish orthography, explicit hyphens are always promoted to the
next line if a line break occurs at that location in the text. For example,
if, given the sentence "Tam wisi czerwono-niebieska flaga" ("There
hangs a red-blue flag"), the optimal line break occurs at the location of
the explicit hyphen, an additional hyphen
will be displayed at the beginning of the next line like this:

> Tam wisi czerwono-   
> -niebieska flaga.

The same convention is used in Portuguese, where the use
of hyphens is common, because they are mandatory for verb forms that include a
pronoun. Homographs or ambiguity may arise if hyphens are treated
incorrectly: for example, "disparate" means "folly" while "dispara-te" means "fire
yourself" (or "fires onto you"). Therefore the former needs to be line
broken as

> dispara-  
> te

and the latter as

> dispara-  
> -te.

A recommended practice is to type <SHY,
NON-BREAKING HYPHEN> instead of <HYPHEN> to achieve promotion of the hyphen to the next
line. This practice is reportedly already common and supported by major text
layout applications. See also *Section 5.4,
[Use of Soft Hyphen](016-zw-zero-width-space-non-tailorable.md#SoftHyphen)*.

<a id="SoftHyphen"></a>

### 5.4  [Use of Soft Hyphen](016-zw-zero-width-space-non-tailorable.md#SoftHyphen)

Unlike U+2010 HYPHEN, which always has a visible rendition, the character
U+00AD SOFT HYPHEN (SHY) is an invisible format character that merely indicates a
preferred intraword line break position. If the line is broken at that point,
then whatever mechanism is appropriate for intraword line breaks should be
invoked, just as if the line break had been triggered by another hyphenation mechanism,
such as a dictionary lookup. Depending on the language and the word, that
may produce different visible results, for example:

- Simply inserting a hyphen glyph
- Inserting a hyphen glyph and changing spelling in the divided word
  parts
- Not showing any visible change and simply breaking at that point
- Inserting a hyphen glyph at the beginning of the new line

The following are a few examples of spelling changes. Each example shows the line
break as “ / ” and any inserted hyphens. There are many other cases.

- In pre-reform German orthography, a “c” before the
  hyphenation point can change into a “k”: “Drucker”
  hyphenates into “Druk- / ker”.
- In modern Dutch, an *e-diaeresis* after the hyphenation point can
  change into a simple “e”: “geërfde” hyphenates
  into “ge- / erfde”, and “geëerd” into “ge-/ eerd”.
- In Swedish, a consonant is sometimes doubled: “tuggummi”;
  hyphenates into “tugg- / gummi”.
- In Dutch, a letter can disappear: “opaatje” hyphenates into
  “opa- / tje”.

The inserted hyphen glyph can take a wide variety of shapes, as appropriate for the situation. Examples
include shapes like U+2010 HYPHEN, U+058A ARMENIAN HYPHEN, U+180A MONGOLIAN
NIRUGU, or U+1806 MONGOLIAN TODO SOFT HYPHEN.

When a SHY is used to represent a possible hyphenation
location, the spelling is that of the word without hyphenation:
“tug<SHY>gummi”. It is up to the line breaking
implementation to make any necessary spelling changes when such a possible
hyphenation is actually used.

Sometimes it is desirable to encode text that includes line breaking
decisions and will not be further broken into lines. If
such text includes hyphenations, the spelling needs to reflect the changes due to
hyphenation: “tugg<U+2010>/ gummi”, including the appropriate
character for any inserted hyphen. For a list of dash-like characters in
Unicode, see *Section 6.2, General Punctuation*, in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

Hyphenation, and therefore the SHY, can be used
with the Arabic script. If the rendering
system breaks at that point, the display—including shaping—should be what
is appropriate for the given language. For
example, sometimes a hyphen-like mark is placed
on the end of the line. This mark looks like a *kashida*, but is not
connected to the letter preceding it. Instead, the
appearance of the mark is as if it had been placed—and the line
divided—after the contextual shapes for the line have been determined. For
more information on shaping, see [[UAX9](https://www.unicode.org/reports/tr41/tr41-36.html#UAX9)] and
*Section 9.2, Arabic*, of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

There are three types of hyphens: explicit hyphens, conditional hyphens,
and dictionary-inserted hyphens resulting from a hyphenation process. There
is no character code for the third kind of hyphen. If a
distinction is desired, the fact that a hyphen is dictionary-inserted and
not user-supplied can only be
represented out of band or by using another control code instead
of SHY.

The action of a hyphenation algorithm is equivalent to the insertion of a
SHY. However, when a word contains an explicit SHY, it is customarily treated
as overriding the action of the hyphenator for that word.

The sequence <SHY, NON-BREAKING HYPHEN>
is given a particular interpretation, see
*Section 5.3, [Use of Hyphen](016-zw-zero-width-space-non-tailorable.md#Hyphen)*.

