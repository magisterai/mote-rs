# ** CB**: Contingent Break Opportunity

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-bb-break-before.md) · [Next](010-cp-closing-parenthesis.md)

<a id="CB"></a>

### ** [CB](009-cb-contingent-break-opportunity.md#CB)**: Contingent Break Opportunity

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1), [LB20](019-6-2-tailorable-line-breaking-rules.md#LB20), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

By default, there is a break opportunity both *before* and *after*
any inline object. Object-specific line breaking behavior is implemented in
the associated object itself, and where available can override the default
to prevent either or both of the default break opportunities. Using U+FFFC
OBJECT REPLACEMENT CHARACTER allows the object
anchor to take a character position in the string.

|  |  |
| --- | --- |
| FFFC | OBJECT REPLACEMENT CHARACTER |

Object-specific line break behavior is best implemented by
querying the object itself, not by replacing the  [CB](009-cb-contingent-break-opportunity.md#CB) line breaking class by
another class.

<a id="CJ"></a>

### ** [CJ](009-cb-contingent-break-opportunity.md#CJ)**: Conditional Japanese Starter

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1)*

This character class contains Japanese small hiragana and katakana. Characters of this class may be treated
as either [NS](013-ns-nonstarters.md#NS) or [ID](011-h2-hangul-lv-syllable.md#ID).

CSS Text Level 3 (which supports Japanese line layout) defines three distinct values
for its line-break behavior:

- strict, typically used for long lines
- normal, the behavior typically used for books and documents
- loose, typically used for short lines such as in newspapers

These have different sets of “kinsoku” characters which cannot be at the beginning or end of
a line; strict has the largest set, while loose has the smallest. The motivation for the smaller
number of kinsoku characters is to avoid triggering justification that puts characters off the grid
position.

Treating characters of class [CJ](009-cb-contingent-break-opportunity.md#CJ)
as class [NS](013-ns-nonstarters.md#NS) will give CSS strict line breaking;
treating them as class [ID](011-h2-hangul-lv-syllable.md#ID) will give CSS normal breaking.

The [CJ](009-cb-contingent-break-opportunity.md#CJ) line break class includes

|  |  |
| --- | --- |
| 3041, 3043, 3045, etc. | Small hiragana |
| 30A1, 30A3, 30A5, etc. | Small katakana |
| 30FC | KATAKANA-HIRAGANA PROLONGED SOUND MARK |
| FF67..FF70 | Halfwidth variants |

<a id="CL"></a>

### ** [CL](009-cb-contingent-break-opportunity.md#CL)**: Close Punctuation

*[LB13](019-6-2-tailorable-line-breaking-rules.md#LB13), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB16](019-6-2-tailorable-line-breaking-rules.md#LB16), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25)*

The closing character of any set of paired punctuation should be kept with
the preceding character, and the same applies to all forms of wide comma and
full stop. This is desirable, even when there are
intervening space characters, to prevent the appearance of a bare
closing punctuation mark at the head of a line.

The class [CL](009-cb-contingent-break-opportunity.md#CL) is closely related to the class
[CP](010-cp-closing-parenthesis.md#CP) (Close Parenthesis). They differ only in that
[CP](010-cp-closing-parenthesis.md#CP) will not introduce a break when followed
by a letter or number, which prevents breaks within constructs like “(s)he”.

The [CL](009-cb-contingent-break-opportunity.md#CL) line break class contains characters
of General\_Category Pe in the Unicode Character Database, but
excludes any characters included in the class [CP](010-cp-closing-parenthesis.md#CP).
It also contains certain non-paired punctuation characters, including:

|  |  |
| --- | --- |
| 3001..3002 | IDEOGRAPHIC COMMA..IDEOGRAPHIC FULL STOP |
| FE10 | PRESENTATION FORM FOR VERTICAL COMMA |
| FE11 | PRESENTATION FORM FOR VERTICAL IDEOGRAPHIC COMMA |
| FE12 | PRESENTATION FORM FOR VERTICAL IDEOGRAPHIC FULL STOP |
| FE50 | SMALL COMMA |
| FE52 | SMALL FULL STOP |
| FF0C | FULLWIDTH COMMA |
| FF0E | FULLWIDTH FULL STOP |
| FF61 | HALFWIDTH IDEOGRAPHIC FULL STOP |
| FF64 | HALFWIDTH IDEOGRAPHIC COMMA |

<a id="CM"></a>

### ** [CM](009-cb-contingent-break-opportunity.md#CM)**: Combining Mark (Non-tailorable)

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB10](018-6-1-non-tailorable-line-breaking-rules.md#LB10)*

#### Combining Characters

Combining character sequences are treated as units for the purpose of line
breaking. The line breaking behavior of the sequence is that of the base
character.

The preferred base character for showing combining
marks in isolation is U+00A0 NO-BREAK SPACE.
If a line break before or after the combining sequence is desired, U+200B
ZERO WIDTH SPACE
can be used. The use of U+0020 SPACE as a base character
is deprecated.

For most purposes, combining characters take on
the properties of their base characters, and that is how the [CM](009-cb-contingent-break-opportunity.md#CM) class is
treated in rule [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9) of this specification.
As a result, if the sequence <0021, 20E4> is
used to represent a triangle enclosing an exclamation point, it
is effectively treated as [EX](010-cp-closing-parenthesis.md#EX), the line
break class of the exclamation mark. If U+26A0 WARNING SIGN
had been used, which also looks like an exclamation point inside a triangle,
it would have the line break class of [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).
Only the latter corresponds to the line breaking behavior expected by
users for this symbol. To
avoid surprising behavior, always use a base character that is a symbol
or letter (Line Break [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL)) when
using enclosing combining marks (General\_Category Me).

The [CM](009-cb-contingent-break-opportunity.md#CM) line break class includes all
combining characters with General\_Category Mc, Me, and Mn, unless listed
explicitly elsewhere. This includes *viramas* that don’t have line break class [VI](015-sy-symbols-allowing-break-after.md#VI) or [VF](015-sy-symbols-allowing-break-after.md#VF).

In particular, line breaking class [CM](009-cb-contingent-break-opportunity.md#CM)
includes the character U+034F COMBINING GRAPHEME JOINER.
This character is used for specialized collation or display; see
Unicode Technical Standard #10, “Unicode Collation Algorithm” [[UTS10](https://www.unicode.org/reports/tr41/tr41-36.html#UTS10)], and
Unicode Standard Annex #53, “Unicode Arabic Mark Rendering” [[UAX53](https://www.unicode.org/reports/tr41/tr41-36.html#UAX53)].
It functions as an invisible combining mark; it should be ignored outside of the few
processes that ascribe meaning to it.
Assigning it class [CM](009-cb-contingent-break-opportunity.md#CM) means the line
breaking algorithm ignores it.

#### Control and Formatting Characters

Most control and formatting characters are ignored in line breaking and do
not contribute to the line width. By giving them class [CM](009-cb-contingent-break-opportunity.md#CM), the line breaking
behavior of the last preceding character that is not of class [CM](009-cb-contingent-break-opportunity.md#CM) affects the
line breaking behavior.

> Note: When control codes and format characters
> are rendered visibly during editing, more graceful layout might be achieved
> by treating them as if they had the line break
> class of the visible symbols instead, that is
> [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) or [ID](011-h2-hangul-lv-syllable.md#ID).
> Such visible modes do not violate
> the constraint on tailorability, because they are logically equivalent to
> having temporarily substituted symbol *characters*, such as the
> characters from the Control Pictures block, or in some cases, character
> sequences, for the actual control characters.

The [CM](009-cb-contingent-break-opportunity.md#CM) line break class includes all characters
of General\_Category Cc and Cf, unless listed explicitly elsewhere.

The [CM](009-cb-contingent-break-opportunity.md#CM) class also includes
U+3035 VERTICAL KANA REPEAT MARK LOWER HALF. This character is
normally preceded by either U+3033 VERTICAL KANA REPEAT MARK UPPER HALF
or U+3034 VERTICAL KANA REPEAT WITH VOICED SOUND MARK UPPER HALF,
and should not be separated from them.

