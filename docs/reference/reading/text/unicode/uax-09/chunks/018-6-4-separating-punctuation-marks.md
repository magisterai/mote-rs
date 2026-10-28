# 6.4 Separating Punctuation Marks

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](017-6-2-vertical-text.md) · [Next](019-section-reorganization.md)

<a id="Separators"></a>

### 6.4  [Separating Punctuation Marks](018-6-4-separating-punctuation-marks.md#Separators)

A common problem case is where the text really represents a
sequence of items with separating punctuation marks,
often programmatically concatenated. These separators are often strings of neutral characters. For example, a web page might have the following at the
bottom:

advertising programs - business solutions - privacy policy
- help - about

This might be built up on the server by concatenating a variable number of strings with " - " as a
separator, for example. If all of the text is translated into Arabic or Hebrew and the overall
page direction is set to be RTL, then the right result occurs, such as the following:

TUOBA - PLEH - YCILOP YCAVIRP - SNOITULOS SSENISUB
- SMARGORP GNISITREVDA

However, suppose that in the translation, there remain some LTR characters. This is not
uncommon for company names, product names, technical terms, and so on. If one of the separators is bounded on
both sides by LTR characters, then the result will be badly jumbled. For example, suppose that
"programs" in the first term and "business" in the second were left in English. Then the result
would be

TUOBA - PLEH - YCILOP YCAVIRP - SNOITULOS **programs****-** **business** GNISITREVDA

The result is a jumble, with
the apparent first term being "advertising business" and the second being "programs solutions".
The simplest solution for this problem is to include an RLM character in each separator string. That will
cause each separator to adopt a right-to-left direction, and produce the correct output:

TUOBA - PLEH - YCILOP YCAVIRP - SNOITULOS **business****-** **programs** GNISITREVDA

The explicit formatting characters
(LRE, RLE, and PDF or LRI, RLI, FSI, and PDI) can be used to achieve the
same effect; web pages would use spans with the attributes *dir="ltr"* or *dir="rtl"*.
Each separate field would be embedded, excluding the separators. In general, LRM and RLM are
preferred to the explicit formatting characters because their effects are more local in scope, and are more
robust than the dir attributes when text is copied. (Ideally programs would convert *dir*
attributes to the corresponding explicit formatting characters when converting to plain text, but that is not
generally supported.)

<a id="Conversion_to_Plain_Text"></a>

### 6.5  [Conversion to Plain Text](018-6-4-separating-punctuation-marks.md#Conversion_to_Plain_Text)

For consistent appearance, when bidirectional text subject to a higher-level
protocol is to be converted to Unicode plain text, formatting
characters should be inserted to ensure that the display order resulting from
the application of the Unicode Bidirectional Algorithm matches that specified by the
higher-level protocol. The same principle should be followed whenever text using a
higher-level protocol is converted to marked-up text that is unaware of the higher-level
protocol. For example, if a higher-level protocol sets the paragraph direction to 1
(R) based on the number of L versus R/AL characters, when converted to plain text the
paragraph would be embedded in a bracketing pair of RLE..PDF formatting
characters. If the same text were converted to HTML4.0 the attribute dir =
"rtl" would be added to the paragraph element.

The display of program text is subject to higher-lever protocols; see
*Section 4.3.2, [HL4 Example 2 for Program Text](015-4-3-2-hl4-example-2-for-program-text.md#HL4Example2)*,
and *Section 4.1, Bidirectional Ordering*, in Unicode Technical Standard #55, “Unicode Source Code Handling”
[[UTS55](https://www.unicode.org/reports/tr41/tr41-36.html#UTS55)].
However, in addition to preserving the appearance resulting from higher-level protocols,
program text must be converted to plain text in a semantics-preserving way,
by inserting characters that are ignored by the compiler.
It is recommended that computer languages allow for the insertion of some
formatting characters in appropriate locations without changing the meaning of a program;
for computer languages that allow this insertion, a procedure is specified for conversion to
plain text. See *Section 4.1, Whitespace*, in Unicode Standard Annex #31, “Unicode Identifiers and Syntax”
[[UAX31](https://www.unicode.org/reports/tr41/tr41-36.html#UAX31)],
and *Section 5.2, Conversion to Plain Text*, in Unicode Technical Standard #55, “Unicode Source Code Handling”
[[UTS55](https://www.unicode.org/reports/tr41/tr41-36.html#UTS55)].

<a id="Mirroring"></a>

## 7  [Mirroring](018-6-4-separating-punctuation-marks.md#Mirroring)

The mirrored property is important to ensure that the correct characters are used for the
desired semantic. This is of particular importance where the name of a character does not indicate
the intended semantic, such as with U+0028 “(” LEFT PARENTHESIS. While the name
indicates that it is a left parenthesis, the character really expresses an *open parenthesis*—the *leading* character in a parenthetical phrase, not the trailing one.

Some of the characters that do not have the Bidi\_Mirrored property
may be rendered with mirrored glyphs, according to a higher level
protocol that adds
mirroring: see Section 4.3,
*[Higher-Level Protocols](014-4-1-boundary-neutrals.md#Higher-Level_Protocols)*, especially
[HL6](014-4-1-boundary-neutrals.md#HL6). Except in such cases, mirroring must be done
according to rule [L4](012-3-3-6-resolving-implicit-levels.md#L4), to ensure that the correct character
is used to express the intended semantic,
and to avoid interoperability and security problems.

Implementing rule [L4](012-3-3-6-resolving-implicit-levels.md#L4) calls for mirrored glyphs. These glyphs may not be exact
*graphical* mirror images. For example, clearly an italic parenthesis is not
an exact mirror image of another— “*(*” is not the mirror image of “*)*”.
Instead, mirror glyphs are those acceptable as mirrors within the normal parameters of the font in
which they are represented.

In implementation, sometimes pairs of characters are acceptable mirrors for one
another—for example, U+0028 “(” LEFT PARENTHESIS and U+0029
“)”
RIGHT PARENTHESIS or U+22E0 “⋠” DOES NOT PRECEDE OR EQUAL and
U+22E1 “⋡” DOES NOT SUCCEED OR EQUAL. Other characters such as
U+2231 “∱” CLOCKWISE INTEGRAL do not have corresponding characters that can be
used for acceptable mirrors. The informative BidiMirroring.txt data file [[Data9]](https://www.unicode.org/reports/tr41/tr41-36.html#Data9),
lists the paired characters with acceptable mirror glyphs.
The formal property name for this data in the
*Unicode Character Database* [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]
is
Bidi\_Mirroring\_Glyph. A comment in the file indicates where
the pairs are “best fit”: they should be acceptable in rendering, although ideally the mirrored
glyphs may have somewhat different shapes.

<a id="Migration"></a>
<a id="Migrating_from_2.0_to_3.0"></a>

## [Migration Issues](018-6-4-separating-punctuation-marks.md#Migration)

There are two major enhancements in the Unicode 6.3 version of the UBA:

- Directional isolates
- Bracket Pairs

Implementations of the new directional isolates should see very few compatibility issues; the UBA has been carefully modified to minimize differences for older text written without them. There are a few edge cases near the limit of the number of levels where there are some differences, but those are not likely to be encountered in practice.

With bracket pairs, there may be more changes. The problem is that without knowing (or having good UI access to) the directional marks or embeddings, people have constructed text with the correct visual appearance but incorrect underlying structure (eg …[…[…, appearing as …[…]…). The new algorithm catches cases like these, because such malformed sequences of brackets are not matched.

However, there are some cases where older implementations without rule [N0](011-3-3-4-resolving-weak-types.md#N0) produced the desired appearance, and newer implementations will not. The user feedback on implementations was sufficiently positive that the decision was made to add [N0](011-3-3-4-resolving-weak-types.md#N0).

There are also incompatibilities from some implementation's failing to updating correctly to previous versions of Unicode, notably in the mishandling solidus such that "T 1/2" (T is an Arabic character) appears incorrectly as "2/1 T".

To mitigate compatibility problems, it is strongly recommended that implementations take the following steps:

- Add appropriate directional formatting characters on both any parentheses that are resolved with rule [N0](011-3-3-4-resolving-weak-types.md#N0) so that they appear properly on older systems. This can be done with directional marks
  (RLM or LRM) on both sides of each parenthesis. For forward compatibility, text authored on older systems should use semantically correct brackets (with directional formatting characters as necessary) to ensure correct display on systems with implementations after Unicode 6.3.
- Add the appropriate explicit embedding around any sequence of numbers + solidus + numbers.

