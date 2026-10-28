# 3 Grapheme Cluster Boundaries

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-1-1-notation.md) · [Next](004-3-1-default-grapheme-cluster-boundary-specification.md)

<a id="Grapheme_Cluster_Boundaries"></a>

## 3  [Grapheme Cluster Boundaries](003-3-grapheme-cluster-boundaries.md#Grapheme_Cluster_Boundaries)

A single Unicode code point is often, but not always the same as a basic unit of a writing
system for a language, or what a typical user might think of as a “character”. There are many
cases where such a basic unit is made up of multiple Unicode code points. To avoid ambiguity
with the term character as defined for encoding purposes, it can be useful to speak of a
*user-perceived character*. For example, “G” + grave-accent is a user-perceived character: users
think of it as a single character, yet is actually represented by two Unicode code points.

The notion of user-perceived character is not always an unambiguous concept for a given writing
system: it may differ based on language, script style, or even based on context, for the same
user. Drop-caps and initialisms, text selection, or "character" counting for text size limits
are all contexts in which the basic unit may be defined differently.

In implementations, the notion of user-perceived characters corresponds to the concept of
grapheme clusters. They are a best-effort approximation that can be determined
programmatically and unambiguously. The definition of grapheme clusters attempts to achieve
uniformity across all human text without requiring language or font metadata about that text.
As an approximation, it may not cover all potential types of user-perceived characters, and it
may have suboptimal behavior in some scripts where further metadata is needed, or where a
different notion of user-perceived character is preferred. Such special cases may require a
customization of the algorithm, while the generic case continues to be supported by the standard
algorithm.

As far as a user is concerned, the underlying representation of text is not important, but
it is important that an editing interface present a uniform implementation of what the user
thinks of as characters. Grapheme clusters can be treated as units, by default, for processes
such as the formatting of drop caps, as well as the implementation of text selection, arrow
key movement, forward deletion, and so forth. For example, when a grapheme cluster
is represented internally by a character sequence consisting of base character + accents, then
using the right arrow key would skip from the start of the base character to the end of the
last accent.

Grapheme cluster boundaries are also important for collation, regular expressions, UI
interactions, segmentation for vertical text, identification of boundaries for first-letter
styling, and counting “character” positions within text. Word boundaries, line boundaries, and
sentence boundaries should not occur within a grapheme cluster: in other words, a grapheme
cluster should be an atomic unit with respect to the process of determining these other
boundaries.

This document defines a default specification for grapheme clusters. It may be customized for particular languages, operations, or other situations. For example, arrow key movement could be tailored by language, or could use knowledge specific to particular fonts to move in a more granular manner, in circumstances where it would be useful to edit individual components. This could apply, for example, to the complex editorial requirements for the Northern Thai script Tai Tham (Lanna). Similarly,
editing a grapheme cluster element by element
may be preferable in some circumstances. For example, on a given system the *backspace
key* might delete by code point, while the *delete key* may
delete an entire cluster.

Moreover, there is not a one-to-one
relationship between grapheme clusters and keys on a keyboard. A
single key on a keyboard may correspond to a whole grapheme cluster,
a part of a grapheme cluster, or a sequence of more than one grapheme
cluster.

Grapheme clusters can only provide an
approximation of where to put cursors. Detailed cursor placement
depends on the text editing framework. The text editing framework
determines where the edges of glyphs are, and how they correspond to
the underlying characters, based on information supplied by the
lower-level text rendering engine and font. For example, the text
editing framework must know if a digraph is represented as a single
glyph in the font, and therefore may not be able to position a cursor
at the proper position separating its two components. That framework
must also be able to determine display representation in cases where
two glyphs overlap—this is true generally when a character is
displayed together with a subsequent nonspacing mark, but must also
be determined in detail for complex script rendering. For cursor
placement, grapheme clusters boundaries can only supply an
approximate guide for cursor placement using least-common-denominator
fonts for the script.

In those relatively rare circumstances where programmers need to
supply end users with user-perceived character counts, the counts
should correspond to the number of segments delimited by grapheme
cluster boundaries. Grapheme clusters *may also be* used in
searching and matching; for more information, see Unicode Technical
Standard #10, “Unicode Collation Algorithm” [[UTS10](https://www.unicode.org/reports/tr41/tr41-36.html#UTS10)], and Unicode Technical
Standard #18, “Unicode Regular Expressions” [[UTS18](https://www.unicode.org/reports/tr41/tr41-36.html#UTS18)].

The Unicode Standard provides a default algorithm for determining grapheme cluster boundaries; the default grapheme clusters are also known as **extended grapheme clusters**. For backwards compatibility with earlier versions of this specification, the Standard also defines and maintains a profile for **legacy grapheme clusters**.

These algorithms can be adapted to produce **tailored
grapheme clusters** for specific locales or other customizations,
such as the contractions used in collation tailoring tables. In [*Table 1a*](003-3-grapheme-cluster-boundaries.md#Table_Sample_Grapheme_Clusters) are
some examples of the differences between these concepts. The tailored
examples are only for illustration: what constitutes a grapheme
cluster will depend on the customizations used by the particular
tailoring in question.

<a id="Table_Sample_Grapheme_Clusters"></a>

Table 1a. [Sample Grapheme Clusters](003-3-grapheme-cluster-boundaries.md#Table_Sample_Grapheme_Clusters)

<table class="subtle">
<tr>
<th>Ex</th>
<th width="40%">Characters</th>
<th>Comments</th>
</tr>
<tr>
<td class="lightgray" colspan="3"><i>Grapheme clusters
							(both legacy and extended)</i></td>
</tr>
<tr>
<td>g̈</td>
<td>0067 ( g ) LATIN SMALL LETTER G<br/> 0308
						( ◌̈ ) COMBINING DIAERESIS
					</td>
<td valign="top">combining character sequences</td>
</tr>
<tr>
<td rowspan="2">각</td>
<td>AC01 ( 각 ) HANGUL SYLLABLE GAG</td>
<td rowspan="2" valign="top">Hangul syllables such as<i>
							gag</i> (which may be a single character, or a sequence of conjoining
						jamos)
					</td>
</tr>
<tr>
<td>1100 ( ᄀ ) HANGUL CHOSEONG KIYEOK<br/> 1161
						( ᅡ ) HANGUL JUNGSEONG A<br/> 11A8 ( ᆨ )
						HANGUL JONGSEONG KIYEOK
					</td>
</tr>
<tr>
<td>ก</td>
<td>0E01 ( ก ) THAI CHARACTER KO KAI</td>
<td valign="top">Thai <i>ko</i></td>
</tr>
<tr>
<td class="lightgray" colspan="3"><i>Extended grapheme
							clusters</i></td>
</tr>
<tr>
<td>நி</td>
<td>0BA8 ( ந ) TAMIL LETTER NA<br/> 0BBF ( ி ) TAMIL VOWEL
						SIGN I
					</td>
<td valign="top">Tamil <i>ni</i></td>
</tr>
<tr>
<td>เ</td>
<td>0E40 ( เ ) THAI CHARACTER SARA E</td>
<td valign="top">Thai <i>e</i></td>
</tr>
<tr>
<td>กำ</td>
<td>0E01 ( ก ) THAI CHARACTER KO KAI<br/> 0E33
						( ำ ) THAI CHARACTER SARA AM
					</td>
<td valign="top">Thai <i>kam</i></td>
</tr>
<tr>
<td>षि</td>
<td>0937 ( ष ) DEVANAGARI LETTER SSA<br/> 093F ( ि )
						DEVANAGARI VOWEL SIGN I
					</td>
<td valign="top">Devanagari <em>ssi</em></td>
</tr>
<tr>
<td>क्षि</td>
<td>0915 ( क ) DEVANAGARI LETTER KA<br/> 094D ( ् )
						DEVANAGARI SIGN VIRAMA<br/> 0937 ( ष ) DEVANAGARI LETTER SSA<br/>
						093F ( ि ) DEVANAGARI VOWEL SIGN I
					</td>
<td valign="top">Devanagari <i>kshi</i></td>
</tr>
<tr>
<td class="lightgray" colspan="3"><i>Legacy grapheme
							clusters</i></td>
</tr>
<tr>
<td>ำ</td>
<td>0E33 ( ำ ) THAI CHARACTER SARA AM</td>
<td valign="top">Thai <i>am</i></td>
</tr>
<tr>
<td>ष</td>
<td>0937 ( ष ) DEVANAGARI LETTER SSA</td>
<td valign="top">Devanagari <i>ssa</i></td>
</tr>
<tr>
<td>ि</td>
<td>093F ( ि ) DEVANAGARI VOWEL SIGN I</td>
<td valign="top">Devanagari <i>i</i></td>
</tr>
<tr>
<td class="lightgray" colspan="3"><i>Possible tailored grapheme
							clusters in a profile</i></td>
</tr>
<tr>
<td>ch</td>
<td>0063 ( c ) LATIN SMALL LETTER C<br/> 0068
						( h ) LATIN SMALL LETTER H
					</td>
<td valign="top">Slovak <i>ch</i> digraph
					</td>
</tr>
<tr>
<td>kʷ</td>
<td>006B ( k ) LATIN SMALL LETTER K<br/> 02B7
						( ʷ ) MODIFIER LETTER SMALL W
					</td>
<td valign="top">sequence with modifier letter</td>
</tr>
</table>

*See also: [Where
is my Character?](https://www.unicode.org/standard/where/), and the UCD file **NamedSequences.txt**
[[Data34](https://www.unicode.org/reports/tr41/tr41-36.html#Data34)].*

A ***legacy grapheme cluster*** is defined as a base (such as
A or カ) followed by zero or more continuing characters. One way to
think of this is as a sequence of characters that form a “stack”.

The base can be single characters, or be any sequence of Hangul Jamo
characters that form a Hangul Syllable, as defined by D133 in The
Unicode Standard, or be a pair of Regional\_Indicator (RI) characters.
For more information about RI characters, see [[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)].

The continuing characters include nonspacing marks, the Join\_Controls
(U+200C ZERO WIDTH NON-JOINER and U+200D ZERO WIDTH JOINER) used in
Indic languages, and a few spacing combining marks to ensure
canonical equivalence.
There are cases in Bangla, Khmer, Malayalam, and Odiya in which a ZWNJ occurs after a consonant and before a *virama* or other combining mark. These cases should not provide an opportunity for a grapheme cluster break. Therefore, ZWNJ has been included in the Extend class.
Additional cases need to be added for
completeness, so that any string of text can be divided up into a
sequence of grapheme clusters. Some of these may be *degenerate*
cases, such as a control code, or an isolated combining mark.

An ***extended grapheme cluster*** is the same as a legacy
grapheme cluster, with the addition of some other characters. The
continuing characters are extended to include all spacing combining
marks, such as the spacing (but dependent) vowel signs in Indic
scripts. For example, this includes U+093F ( ि ) DEVANAGARI
VOWEL SIGN I. The extended grapheme clusters should be used in
implementations in preference to legacy grapheme clusters, because
they provide better results for Indic scripts such as Tamil or
Devanagari in which editing by orthographic syllable is typically
preferred. For scripts such as Thai, Lao, and certain other Southeast
Asian scripts, editing by visual unit is typically preferred, so for
those scripts the behavior of extended grapheme clusters is similar
to (but not identical to) the behavior of legacy grapheme clusters.

For the rules defining the boundaries for grapheme clusters, see *[Section 3.1](004-3-1-default-grapheme-cluster-boundary-specification.md#Default_Grapheme_Cluster_Table)*. For more
information on the composition of Hangul syllables, see *Chapter
3, Conformance*, of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

A key feature of Unicode grapheme clusters (both legacy
and extended) is that they remain unchanged across all canonically
equivalent forms of the underlying text. Thus the boundaries remain
unchanged whether the text is in NFC or NFD. Using a grapheme cluster
as the fundamental unit of matching thus provides a very clear and
easily explained basis for canonically equivalent matching. This is
important for applications from searching to regular expressions.

Another key feature is that default Unicode grapheme clusters are
atomic units with respect to the process of determining the Unicode
default word, and sentence boundaries. They are usually—but not
always—atomic units with respect to line boundaries: there are
exceptions due to the special handling of spaces. For more
information, see *Section 9.2 Legacy Support for Space
Character as Base for Combining Marks* in [[UAX14](https://www.unicode.org/reports/tr41/tr41-36.html#UAX14)].

Grapheme clusters can be tailored to meet further requirements. Such
tailoring is permitted, but the possible rules are outside of the
scope of this document. One example of such a tailoring would be for
the *aksaras*, or *orthographic syllables*, used in many
Indic scripts. Aksaras usually consist of a consonant, sometimes with
an inherent vowel and sometimes followed by an explicit, dependent
vowel whose rendering may end up on any side of the consonant letter
base. Extended grapheme clusters include such simple combinations.

