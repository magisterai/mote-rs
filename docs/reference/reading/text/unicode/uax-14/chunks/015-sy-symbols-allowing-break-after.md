# ** SY**: Symbols Allowing Break After

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-qu-quotation.md) · [Next](016-zw-zero-width-space-non-tailorable.md)

<a id="SY"></a>

### ** [SY](015-sy-symbols-allowing-break-after.md#SY)**: Symbols Allowing Break After

*[LB13](019-6-2-tailorable-line-breaking-rules.md#LB13), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB21b](019-6-2-tailorable-line-breaking-rules.md#LB21b), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25)*

The [SY](015-sy-symbols-allowing-break-after.md#SY)
line breaking property is intended to provide a break opportunity after, except in front of
digits, so as to not break “1/2” or “06/07/99”.

|  |  |
| --- | --- |
| 002F | SOLIDUS |

URLs are now so common in regular plain text that they need to be taken
into account when assigning general-purpose line breaking properties. Slash (*solidus*)
is allowed as an additional, limited break opportunity to improve layout of Web addresses.
As a side effect, some common abbreviations
such as “w/o” or “A/S”, which normally would not be broken,
acquire a line
break opportunity. The recommendation in this case is for the layout system
not to utilize a line break opportunity allowed by [SY](015-sy-symbols-allowing-break-after.md#SY) unless the distance
between it and the next line break opportunity exceeds an implementation-defined minimal distance.

> Note: Normally, symbols are treated as [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).
> However, symbols can be added to this line breaking
> class or classes [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA), [BB](008-bb-break-before.md#BB),
> and [B2](008-bb-break-before.md#B2) by tailoring.
> This can be used to allow additional line breaks—for example,
> after “=”. Mathematics requires additional specifications for line
> breaking, which are outside the scope of this annex.

<a id="VF"></a>

### ** [VF](015-sy-symbols-allowing-break-after.md#VF)**: Virama Final

*[LB28a](019-6-2-tailorable-line-breaking-rules.md#LB28a)*

The [VF](015-sy-symbols-allowing-break-after.md#VF) line break class is only used for scripts that use the Brahmic style of context analysis.
It contains the viramas of Indic syllabic category Pure\_Killer in scripts where the final consonant of a phonological syllable is expressed as a sequence of a consonant and such a virama, and the final consonant needs to be kept together with the preceding orthographic syllable.
This includes:

|  |  |
| --- | --- |
| 1BF2..1BF3 | BATAK PANGOLAT..BATAK PANONGONAN |

Viramas of Indic syllabic category Pure\_Killer that don’t meet the conditions for line break class [VF](015-sy-symbols-allowing-break-after.md#VF) use the line break class [CM](009-cb-contingent-break-opportunity.md#CM).

<a id="VI"></a>

### ** [VI](015-sy-symbols-allowing-break-after.md#VI)**: Virama

*[LB28a](019-6-2-tailorable-line-breaking-rules.md#LB28a)*

The [VI](015-sy-symbols-allowing-break-after.md#VI) line break class is only used for scripts that use the Brahmic style of context analysis.
It contains the viramas of Indic syllabic categories Virama and Invisible\_Stacker of such scripts.

|  |  |
| --- | --- |
| 1B44 | BALINESE ADEG ADEG |
| A9C0 | JAVANESE PANGKON |
| 11046 | BRAHMI VIRAMA |
| 1134D | GRANTHA SIGN VIRAMA |
| 11F42 | KAWI CONJOINER |

<a id="WJ"></a>

### [WJ](015-sy-symbols-allowing-break-after.md#WJ): Word Joiner (Non-tailorable)

*[LB11](018-6-1-non-tailorable-line-breaking-rules.md#LB11), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b)*

These characters glue together left and right neighbor characters such
that they are kept on the same line.

|  |  |
| --- | --- |
| 2060 | WORD JOINER (WJ) |
| FEFF | ZERO WIDTH NO-BREAK SPACE (ZWNBSP) |

The word joiner character is the preferred choice for an invisible
character to keep other characters together that would otherwise be split
across the line at a direct break. The character FEFF has the same effect, but
because it is also used in an unrelated way as a *byte order mark,* the use
of the WJ as the preferred interword glue simplifies the handling of FEFF.

By definition, WJ and ZWNBSP take precedence over the action
of [SP](014-qu-quotation.md#SP), but not
[ZW](016-zw-zero-width-space-non-tailorable.md#ZW).

<a id="XX"></a>

### ** [XX](015-sy-symbols-allowing-break-after.md#XX)**: Unknown

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1)*

The [XX](015-sy-symbols-allowing-break-after.md#XX) line break class consists of all characters with
General\_Category Co as well as those unassigned code points that are not within a CJK block.
Unassigned characters in blocks or ranges of the Unicode codespace
that have been reserved for CJK scripts default to the class [ID](011-h2-hangul-lv-syllable.md#ID),
and are listed in the description of that class.

Unassigned code positions, private-use characters, and characters for which
reliable line breaking information is not available are assigned this
line breaking property. The default behavior for this class is identical to
class [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).
Users can manually insert ZWSP or WORD JOINER around
characters of class [XX](015-sy-symbols-allowing-break-after.md#XX) to allow or prevent breaks as needed.

In addition, implementations can override or tailor this default behavior—for example,
by assigning characters the property [ID](011-h2-hangul-lv-syllable.md#ID) or another class.
Doing so may give better default behavior for their users. There are
other possible means of determining the desired behavior of private-use
characters. For example, one implementation
might treat any private-use character in ideographic context as [ID](011-h2-hangul-lv-syllable.md#ID),
while another implementation might support a method for assigning specific
properties to specific definitions of private-use characters. The details of
such use of private-use characters are outside the scope of this standard.

For supplementary characters, a useful default is to treat characters in the
range 10000..1FFFD as [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) and characters in the ranges
20000..2FFFD and 30000..3FFFD as [ID](011-h2-hangul-lv-syllable.md#ID),
until the implementation can be revised to take into
account the actual line breaking properties for these characters.

For more information on handling default property values for unassigned
characters, see the discussion on default property values in *Section
5.3, Unknown and Missing Characters*, of
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

The line breaking rules in *Section 6, [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)*
assume that all unknown characters have
been assigned one of the other line breaking classes,
such as [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL),
as part of assigning line breaking classes to the
input characters.

Implementations that do not support a given character should also treat it as unknown
([XX](015-sy-symbols-allowing-break-after.md#XX)).

