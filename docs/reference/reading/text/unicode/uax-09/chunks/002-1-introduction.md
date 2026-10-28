# 1 Introduction

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-unicode-standard-annex-9.md) · [Next](003-2-2-explicit-directional-overrides.md)

<a id="Introduction"></a>
<a id="3.1_Bidirectional_Behavior"></a>

## 1  [Introduction](002-1-introduction.md#Introduction)

The Unicode Standard prescribes a *memory* representation order
known as logical order. When text is presented in horizontal lines, most
scripts display characters from left to right. However, there are several
scripts (such as Arabic or Hebrew) where the natural ordering of horizontal
text in display is from right to left. If all of the text has
a uniform horizontal
direction, then the ordering of the display text is unambiguous.

However, because these right-to-left scripts
use digits that are written from left to right, the text is actually *bidirectional*:
a mixture of right-to-left *and* left-to-right text. In addition to
digits, embedded words from English and other scripts are also written from
left to right, also producing bidirectional text. Without a clear
specification, ambiguities can
arise in determining the ordering of the displayed characters when the
horizontal direction of the text is not uniform.

This annex describes the algorithm used to determine the directionality for bidirectional
Unicode text. The algorithm extends the implicit model currently employed by a number of existing
implementations and adds explicit formatting characters for special circumstances.
In most cases, there is no need to include additional information with the text to obtain correct display
ordering.

However, in the case of bidirectional text, there are circumstances where
an implicit bidirectional ordering is not sufficient to produce
comprehensible text. To deal with these cases, a minimal set of directional
formatting characters is defined to control the ordering of characters when
rendered. This allows exact control of the display ordering for legible
interchange and
ensures that plain text used for simple items like filenames or labels can always be correctly
ordered for display.

The directional formatting characters are used *only* to influence the display ordering of
text. In all other respects they should be ignored—they have no effect on the comparison of text or on word breaks, parsing, or numeric analysis.

Each character has an implicit *bidirectional type*.
The bidirectional types left-to-right and right-to-left are called *strong types*, and characters
of those types are called strong directional characters. The bidirectional types associated with numbers
are called *weak types*, and characters of those types are called weak directional characters.
With the exception of the directional formatting characters, the remaining bidirectional types and characters are called neutral.
The algorithm uses the implicit bidirectional types of the characters in a text to arrive at a
reasonable display ordering for text.

When working with bidirectional text, the characters are still interpreted in logical
order—only the display is affected. The display ordering of bidirectional text depends on the
directional properties of the characters in the text. Note that there
are important security issues connected with bidirectional text: for more information, see [[UTR36](https://www.unicode.org/reports/tr41/tr41-36.html#UTR36)].

<a id="Directional_Formatting_Characters"></a>
<a id="Directional_Formatting_Codes"></a>

## 2  [Directional Formatting Characters](002-1-introduction.md#Directional_Formatting_Characters)

Three types of explicit directional formatting characters are used to modify the standard implicit Unicode
Bidirectional Algorithm (UBA). In addition, there are implicit directional formatting characters, the *right-to-left* and *left-to-right* marks. The effects of all of these formatting characters are limited to the current paragraph;
thus, they are terminated by a *paragraph separator*.

These formatting characters all have the property *Bidi\_Control*,
and are divided into three groups:

|  |  |
| --- | --- |
| Implicit Directional Formatting Characters | LRM, RLM, ALM |
| Explicit Directional Embedding and Override Formatting Characters | LRE, RLE, LRO, RLO, PDF |
| Explicit Directional Isolate Formatting Characters | LRI, RLI, FSI, PDI |

Although the term *embedding* is used for some explicit formatting characters, the text within the scope
of the embedding formatting characters is not independent of the surrounding text. Characters within an embedding can affect
the ordering of characters outside, and vice versa.
This is not the case with the isolate formatting characters, however.
Characters within an isolate cannot affect the ordering of characters outside it, or vice versa.
The effect that an isolate as a whole has on the ordering of the surrounding characters
is the same as that of a neutral character,
whereas an embedding or override roughly has the effect of a strong character.

Directional isolate characters were introduced in Unicode 6.3 after it became apparent that
directional embeddings usually have too strong an effect on their surroundings and are thus unnecessarily difficult to use.
The new characters were introduced instead of changing the behavior of the existing ones because doing so might have
had an undesirable effect on those existing documents that do rely on the old behavior.
Nevertheless, the use of the directional isolates instead of embeddings is encouraged in new documents –
once target platforms are known to support them.

On web pages, the *explicit* directional formatting characters (of all types – embedding, override, and isolate)
should be replaced by other mechanisms suitable for HTML and CSS. For information on the correspondence between
explicit directional formatting characters and equivalent HTML5 markup and CSS properties, see
*Section 2.7, [Markup and Formatting Characters](003-2-2-explicit-directional-overrides.md#Markup_And_Formatting)*.

<a id="Explicit_Directional_Embeddings"></a>
<a id="Explicit_Directional_Embedding"></a>

### 2.1  [Explicit Directional Embeddings](002-1-introduction.md#Explicit_Directional_Embeddings)

The following characters signal that a piece of text is to be treated as embedded. For example, an
English quotation in the middle of an Arabic sentence could be marked as being embedded
left-to-right text. If there were a Hebrew phrase in the middle of the English quotation,
that phrase could be marked as being embedded right-to-left text.
Embeddings can be nested one inside another, and in isolates and overrides.

| Abbr. | Code Point | Name | Description |
| --- | --- | --- | --- |
| **LRE** | U+202A | LEFT-TO-RIGHT EMBEDDING | Treat the following text as embedded left-to-right. |
| **RLE** | U+202B | RIGHT-TO-LEFT EMBEDDING | Treat the following text as embedded right-to-left. |

The
effect of right-left line direction, for example, can be accomplished by embedding the text
with RLE...PDF. (PDF will be described in *Section 2.3, [Terminating Explicit Directional Embeddings and Overrides](003-2-2-explicit-directional-overrides.md#Terminating_Explicit_Directional_Embeddings_and_Overrides)*.)

