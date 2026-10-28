# 4.1 Boundary Neutrals

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-3-5-shaping.md) · [Next](015-4-3-2-hl4-example-2-for-program-text.md)

<a id="Boundary_Neutrals"></a>

### 4.1  [Boundary Neutrals](014-4-1-boundary-neutrals.md#Boundary_Neutrals)

The goal in marking a formatting or control character as BN is that it have no effect on the rest
of the algorithm. (ZWJ and ZWNJ are exceptions; see [X9](010-3-3-3-preparations-for-implicit-processing.md#X9)). Because
conformance does not require the
precise ordering of formatting characters with respect to others,
implementations can handle them in different ways as long as they preserve the ordering
of the other characters.

<a id="Explicit_Formatting_Characters"></a>
<a id="Explicit_Formatting_Codes"></a>

### 4.2  [Explicit Formatting Characters](014-4-1-boundary-neutrals.md#Explicit_Formatting_Characters)

As with any Unicode characters, systems do not have to support any particular explicit
directional formatting character (although it is not generally useful to include a terminating character
without including the initiator). Generally, conforming systems will fall into four classes:

- *No bidirectional formatting.* This implies that the system does not visually interpret
  characters from right-to-left scripts.
- *Implicit bidirectionality.* The implicit Bidirectional Algorithm and the directional
  marks ALM, RLM and LRM are supported.
- *Non-isolate bidirectionality.* The implicit Bidirectional Algorithm, the implicit directional
  marks, and the explicit non-isolate directional formatting characters are supported:
  ALM, RLM, LRM, LRE, RLE, LRO, RLO,
  PDF.
- *Full bidirectionality.* The implicit Bidirectional Algorithm, the implicit directional
  marks, and all the explicit directional formatting characters are supported:
  ALM, RLM, LRM, LRE, RLE, LRO, RLO,
  PDF, FSI, LRI, RLI, PDI.

<a id="Higher-Level_Protocols"></a>

### 4.3  [Higher-Level Protocols](014-4-1-boundary-neutrals.md#Higher-Level_Protocols)

The following clauses are the only permissible ways for systems to apply higher-level protocols
to the ordering of bidirectional text. Some of the clauses apply to *segments* of structured
text. This refers to the situation where text is interpreted as being structured, whether with
explicit markup such as XML or HTML, or internally structured such as in a word processor or
spreadsheet. In such a case, a segment is span of text that is distinguished in some way by the
structure.

<a id="HL1"></a>

***[HL1](014-4-1-boundary-neutrals.md#HL1).**
Override [P3](008-3-3-1-the-paragraph-level.md#P3), and set the paragraph embedding level explicitly.
This does **not** apply when deciding how to treat FSI in rule [X5c](009-non-formatting-characters.md#X5c).*

- A higher-level protocol may set any paragraph level. This can be done on the basis of the context, such as on a table cell, paragraph, document, or system level. ([P2](008-3-3-1-the-paragraph-level.md#P2) may be skipped if [P3](008-3-3-1-the-paragraph-level.md#P3) is overridden). Note that this does not allow a higher-level protocol to override the limit specified in [BD2](004-3-basic-display-algorithm.md#BD2).
- A higher-level protocol may apply rules equivalent to [P2](008-3-3-1-the-paragraph-level.md#P2) and [P3](008-3-3-1-the-paragraph-level.md#P3) but default to level 1 (RTL) rather than 0 (LTR) to match overall RTL context.
- A higher-level protocol may use an entirely different algorithm that heuristically auto-detects the paragraph embedding level based on the paragraph text and its context. For example, it could base it on whether there are more RTL characters in the text than LTR. As another example, when the paragraph contains no strong characters, its direction could be determined by the levels of the paragraphs before and after.

<a id="HL2"></a>

***[HL2](014-4-1-boundary-neutrals.md#HL2).** Override [W2](011-3-3-4-resolving-weak-types.md#W2), and set EN or AN explicitly.*

- A higher-level protocol may reset characters of type EN to AN, or vice versa, and ignore
  [W2](011-3-3-4-resolving-weak-types.md#W2). For example, style sheet or markup information can be used within a span of text to
  override the setting of EN text to be always be AN, or vice versa.

<a id="HL3"></a>

***[HL3](014-4-1-boundary-neutrals.md#HL3).** Emulate explicit directional formatting characters.*

- A higher-level protocol can impose a directional embedding, isolate or override on a segment of
  structured text. The behavior must always be defined by reference to what would happen if
  the equivalent explicit directional formatting characters as defined in the algorithm were inserted into the text. For
  example, a style sheet or markup can modify the embedding level on a span of text.

<a id="HL4"></a>

***[HL4](014-4-1-boundary-neutrals.md#HL4).** Apply the Bidirectional Algorithm to segments*.

- The Bidirectional Algorithm can be applied independently to one or more segments of structured
  text. For example, when displaying a document consisting of textual data and visible markup
  in an editor, a higher-level process can handle syntactic elements in the markup separately
  from the textual data.

<a id="HL5"></a>

***[HL5](014-4-1-boundary-neutrals.md#HL5).** Provide artificial context.*

- Text can be processed by the Bidirectional Algorithm as if it were preceded by a character of a
  given type and/or followed by a character of a given type. This allows a piece of text that
  is extracted from a longer sequence of text to behave as it did in the larger context.

<a id="HL6"></a>

***[HL6](014-4-1-boundary-neutrals.md#HL6).** Additional mirroring*.

- Certain characters that do not
  have the Bidi\_Mirrored property can also be depicted by
  a mirrored glyph in specialized contexts. Such contexts include, but are
  not limited to, historic scripts and associated punctuation, private-use
  characters, and characters in mathematical expressions. (See Section 7, *[Mirroring](018-6-4-separating-punctuation-marks.md#Mirroring)*.)
  These characters are those that fit at least one of the following
  conditions:
  1. Characters with a resolved directionality of R
  2. Characters with a resolved
     directionality of L and whose bidirectional type is R or AL

Clauses [HL1](014-4-1-boundary-neutrals.md#HL1) and [HL3](014-4-1-boundary-neutrals.md#HL3) are specialized applications of the more general clauses [HL4](014-4-1-boundary-neutrals.md#HL4) and [HL5](014-4-1-boundary-neutrals.md#HL5). They are provided here explicitly because they directly correspond to common operations.

<a id="HL4Example1"></a>

### 4.3.1  [HL4 Example 1 for XML](014-4-1-boundary-neutrals.md#HL4Example1)

As an example of the application of [HL4](014-4-1-boundary-neutrals.md#HL4), suppose an XML document contains the following
fragment. (Note: This is a simplified example for illustration: element names, attribute names,
and attribute values could all be involved.)

> `ARABICenglishARABIC<e1 type='ab'>ARABICenglish<e2 type='cd'>english`

This can be analyzed as being five different segments:

1. `ARABICenglishARABIC`
2. `<e1 type='ab'>`
3. `ARABICenglish`
4. `<e2 type='cd'>`
5. `english`

To make the XML file readable as source text, the display in an editor
could order these elements all in a uniform direction (for example, all left-to-right) and apply the
Bidirectional Algorithm to each
field separately. It could also choose to order the element names, attribute names, and attribute
values uniformly in the same direction (for example, all left-to-right). For final display, the markup
could be ignored, allowing all of the text (segments a, c, and e) to be reordered together.

