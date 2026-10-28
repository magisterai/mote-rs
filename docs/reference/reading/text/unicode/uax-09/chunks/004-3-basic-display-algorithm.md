# 3 Basic Display Algorithm

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-2-2-explicit-directional-overrides.md) · [Next](005-3-1-2-matching-explicit-directional-formatting-characters.md)

<a id="Basic_Display_Algorithm"></a>

## 3  [Basic Display Algorithm](004-3-basic-display-algorithm.md#Basic_Display_Algorithm)

The Unicode Bidirectional Algorithm
(UBA) takes a stream of text as input and proceeds in
four main phases:

- **Separation into paragraphs.** The
  rest of the algorithm is applied separately to the text within each
  paragraph.
- **Initialization.** A list of
  bidirectional character types is initialized, with one entry for each
  character in the original text. The value of each entry is the
  Bidi\_Class property value of the respective character.
  A list of embedding levels, with one level per character, is then initialized.
  Note that the original characters are referenced in *Section 3.3.5,
  [Resolving Neutral and Isolate Formatting Types](011-3-3-4-resolving-weak-types.md#Resolving_Neutral_Types)*.
- **Resolution of the embedding levels.** 
  A series of rules is applied to the lists of embedding levels and
  bidirectional character types. Each rule
  operates on the current values of those lists, and can modify those values.
  The original characters and their Bidi\_Paired\_Bracket and Bidi\_Paired\_Bracket\_Type
  property values are referenced in the application of certain rules.
  The result of this phase is a modified list of embedding levels;
  the list of bidirectional character types is no longer needed.
- **Reordering.**  The text within each
  paragraph is reordered for display: first, the text in the paragraph is
  broken into lines, then the resolved embedding levels are used to reorder the
  text of each line for display.

The algorithm reorders text only within a paragraph; characters in one paragraph have no effect
on characters in a different paragraph. Paragraphs are divided by the Paragraph Separator or
appropriate Newline Function (for guidelines on the handling of CR, LF, and CRLF, see *Section 4.4, Directionality*, and *Section 5.8, Newline Guidelines* of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]).
Paragraphs may also be determined by higher-level protocols: for example, the text in two
different cells of a table will be in different paragraphs.

Combining characters always attach to the preceding base character in the memory
representation. Even after reordering for display and performing character shaping, the glyph
representing a combining character will attach to the glyph representing its base character in
memory. Depending on the line orientation and the placement direction of base letterform glyphs,
it may, for example, attach to the glyph on the left, or on the right, or above.

This annex uses the numbering conventions for normative
definitions and rules in *Table 1*.

<a id="Table_Normative_Defs_Rules"></a>

Table 1. [Normative Definitions and Rules](004-3-basic-display-algorithm.md#Table_Normative_Defs_Rules)

| Numbering | Section |
| --- | --- |
| BDn | Definitions |
| Pn | Paragraph levels |
| Xn | Explicit levels and directions |
| Wn | Weak types |
| Nn | Neutral types |
| In | Implicit levels |
| Ln | Resolved levels |

<a id="Definitions"></a>

### 3.1  [Definitions](004-3-basic-display-algorithm.md#Definitions)

<a id="Basics"></a>

### 3.1.1  [Basics](004-3-basic-display-algorithm.md#Basics)

<a id="BD1"></a>

***[BD1](004-3-basic-display-algorithm.md#BD1).*** The *bidirectional character types* are values assigned to each
Unicode character, including unassigned characters.
The formal property name in the
*Unicode Character Database* [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]
is Bidi\_Class.

<a id="BD2"></a>

***[BD2](004-3-basic-display-algorithm.md#BD2).*** *Embedding levels* are numbers that indicate how deeply the text is
nested, and the default direction of text on that level. The minimum embedding level of text is zero, and the maximum explicit depth is
125, a value referred to as *max\_depth* in the rest of this document.

As rules [X1](008-3-3-1-the-paragraph-level.md#X1) through [X8](009-non-formatting-characters.md#X8) will specify,
embedding levels are set by
explicit formatting characters (embedding, isolate, and override);
higher numbers mean the text is more deeply nested. The reason for having a limitation is
to provide a precise stack limit for implementations to guarantee the same results. A
maximum explicit level of 125
is far more than sufficient for ordering, even with mechanically generated formatting;
the display becomes rather muddied with more than a small number of embeddings.

For implementation stability, this specification now guarantees that the
value of 125 for max\_depth will not be increased (or decreased) in future versions.
Thus, it is safe for implementations to treat the max\_depth value as a constant. The max\_depth
value has been 125 since UBA Version 6.3.0.

<a id="BD3"></a>

***[BD3](004-3-basic-display-algorithm.md#BD3).*** The default direction of the current embedding level
(for the character in question) is called the *embedding direction*. It is **L** if the embedding level is even,
and **R** if the embedding level is odd.

For example, in a particular piece of text, level 0 is plain English text. Level 1 is plain
Arabic text, possibly embedded within English level 0 text. Level 2 is English text, possibly
embedded within Arabic level 1 text, and so on. Unless their direction is overridden, English
text and numbers will always be an even level; Arabic text (excluding numbers) will always be an
odd level. The exact meaning of the embedding level will become clear when the reordering
algorithm is discussed, but the following provides an example of how the algorithm works.

<a id="BD4"></a>

***[BD4](004-3-basic-display-algorithm.md#BD4).*** The *paragraph embedding level* is the embedding level that
determines the default bidirectional orientation of the text in that paragraph.

<a id="BD5"></a>

***[BD5](004-3-basic-display-algorithm.md#BD5).*** The direction of the paragraph embedding level is called the *paragraph direction*.

- In some contexts the paragraph direction is also known as the *base direction*.

<a id="BD6"></a>

***[BD6](004-3-basic-display-algorithm.md#BD6).*** The *directional override status* determines whether the
bidirectional type of characters is to be reset. The directional override status
is set by using explicit directional formatting characters. This status
has three states, as shown in *Table 2*.

<a id="Table_Directional_Override_Status"></a>

Table 2. [Directional Override Status](004-3-basic-display-algorithm.md#Table_Directional_Override_Status)

| Status | Interpretation |
| --- | --- |
| **Neutral** | No override is currently active |
| **Right-to-left** | Characters are to be reset to **R** |
| **Left-to-right** | Characters are to be reset to **L** |

<a id="BD7"></a>

***[BD7](004-3-basic-display-algorithm.md#BD7).*** A *level run* is a maximal substring of characters that have the
same embedding level. It is maximal in that no character immediately before or after the substring
has the same level (a level run is also known as a *directional run)*.

As specified below, level runs are important at two different stages of the Bidirectional Algorithm.
The first stage occurs after rules [X1](008-3-3-1-the-paragraph-level.md#X1) through [X9](010-3-3-3-preparations-for-implicit-processing.md#X9) have assigned an explicit embedding level
to each character on the basis of the paragraph direction and the explicit directional formatting characters.
At this stage, in rule [X10](010-3-3-3-preparations-for-implicit-processing.md#X10), level runs are used to build up the units to which subsequent rules are applied.
Those rules further adjust each character’s embedding level on the basis of its implicit bidirectional type and those
of other characters in the unit – but not outside it.
The level runs resulting from these resolved embedding levels are then used in the actual reordering of the text by rule [L2](012-3-3-6-resolving-implicit-levels.md#L2).
The following example illustrates level runs at this later stage of the algorithm.

#### Example

In this and the following examples, case is used to indicate different
implicit character types for those unfamiliar with right-to-left letters.
Uppercase letters stand for right-to-left characters (such as Arabic or
Hebrew), and lowercase letters stand for left-to-right characters (such as
English or Russian).



```

Memory:            car is THE CAR in arabic

Character types:   LLL-LL-RRR-RRR-LL-LLLLLL

Paragraph level:   0

Resolved levels:   000000011111110000000000
```




Notice that the neutral character (space) between THE and CAR gets the level of the surrounding
characters. The level of the neutral characters could be changed by
inserting appropriate directional marks around neutral characters, or using explicit directional formatting characters.

