# 3.3.3 Preparations for Implicit Processing

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-non-formatting-characters.md) · [Next](011-3-3-4-resolving-weak-types.md)

<a id="Preparations_for_Implicit_Processing"></a>

### 3.3.3  [Preparations for Implicit Processing](010-3-3-3-preparations-for-implicit-processing.md#Preparations_for_Implicit_Processing)

The explicit embedding levels that have been assigned to the characters by the preceding rules
will be further adjusted (in rules [I1](012-3-3-6-resolving-implicit-levels.md#I1)–[I2](012-3-3-6-resolving-implicit-levels.md#I2))
on the basis of the characters' implicit bidirectional types (computed in rules [W1](011-3-3-4-resolving-weak-types.md#W1)–[W7](011-3-3-4-resolving-weak-types.md#W7), [N0](011-3-3-4-resolving-weak-types.md#N0)–[N2](011-3-3-4-resolving-weak-types.md#N2)).
The adjustment made for a given character will then depend on the characters around it.
However, this dependency is limited by logically dividing the paragraph into sub-units,
and doing the subsequent implicit processing on each unit independently, as shown in the next two steps.

<a id="X9"></a>

***[X9](010-3-3-3-preparations-for-implicit-processing.md#X9).** Remove all RLE, LRE, RLO, LRO, PDF, and BN characters.*

- Note that an implementation does not have to actually remove the characters; it just has to
  behave as though the characters were not present for the remainder of the algorithm. Conformance does
  not require any particular placement of these characters as long as all other characters are ordered
  correctly.

  See Section 5, *[Implementation Notes](016-4-4-bidirectional-conformance-testing.md#Implementation_Notes)*, for information on
  implementing the algorithm without removing the formatting characters.
- The *zero width joiner* and *non-joiner* affect the shaping of the adjacent characters—those
  that are adjacent in the original backing-store order, even though those characters may end up
  being rearranged to be non-adjacent by the Bidirectional Algorithm. For more information, see
  Section 6.1,
  *[Joiners](016-4-4-bidirectional-conformance-testing.md#Joiners)*.
- Note that FSI, LRI, RLI, and PDI characters are **not** removed.
  As indicated by the rules below, they are used, in part, to determine the paragraph’s isolating run sequences,
  within which they are then treated as neutral characters.
  Nevertheless, they are of course zero-width characters and, like LRM and RLM, should not be visible in the final output.

<a id="X10"></a>

***[X10](010-3-3-3-preparations-for-implicit-processing.md#X10).** Perform the following steps:*

- *Compute the set of isolating run sequences as specified by [BD13](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD13),
  based on the bidirectional types of the characters
  and the embedding levels assigned by the rules above
  ([X1](008-3-3-1-the-paragraph-level.md#X1)–[X9](010-3-3-3-preparations-for-implicit-processing.md#X9)).*
- *Determine the *start-of-sequence* (**sos**)
  and end-of-sequence (**eos**) types,
  either L or R, for each isolating run sequence.
  These depend on the higher of the two levels on either side of the sequence boundary:*
  - *For sos, compare the level of the first character in the sequence
    with the level of the character preceding it in the paragraph (not counting characters removed by [X9](010-3-3-3-preparations-for-implicit-processing.md#X9)),
    and if there is none, with the paragraph embedding level.*
  - *For eos, compare the level of the last character in the sequence
    with the level of the character following it in the paragraph (not counting characters removed by [X9](010-3-3-3-preparations-for-implicit-processing.md#X9)),
    and if there is none or the last character of the sequence is an isolate initiator (lacking a matching PDI),
    with the paragraph embedding level.*
  - *If the higher level is odd, the sos or eos is R; otherwise, it is L.*
  - Note that these computations must use the embedding levels assigned by the rules above,
    before any changes are made to them in the steps below.
- *Apply rules [W1](011-3-3-4-resolving-weak-types.md#W1)–[W7](011-3-3-4-resolving-weak-types.md#W7),
  [N0](011-3-3-4-resolving-weak-types.md#N0)–[N2](011-3-3-4-resolving-weak-types.md#N2),
  and [I1](012-3-3-6-resolving-implicit-levels.md#I1)–[I2](012-3-3-6-resolving-implicit-levels.md#I2),
  in the order in which they appear below,
  to each of the isolating run sequences,
  applying one rule to all the characters in the sequence in the order in which they occur in the sequence
  before applying another rule to any part of the sequence.
  The order that one isolating run sequence is treated relative to another does not matter.
  When applying a rule to an isolating run sequence,
  the last character of each level run in the isolating run sequence
  is treated as if it were immediately followed by the first character in the next level run in the sequence, if any.*

Here are some examples, each of which is assumed to be a paragraph
with base level 0 where no character sequence *text<sub>i</sub>* contains explicit directional formatting characters or paragraph separators.
The dots in the examples are intended to separate elements for visual clarity; they are not part of the text.

Example 1: *text<sub>1</sub>*·RLE·*text<sub>2</sub>*·LRE·*text<sub>3</sub>*·PDF·*text<sub>4</sub>*·PDF·RLE·*text<sub>5</sub>*·PDF·*text<sub>6</sub>*

<table class="subtle">
<tr><th>Isolating Run Sequence</th><th>Embedding Level</th><th>sos</th><th>eos</th></tr>
<tr><td><i>text<sub>1</sub></i></td><td>0</td><td>L</td><td>R</td></tr>
<tr><td><i>text<sub>2</sub></i></td><td>1</td><td>R</td><td>L</td></tr>
<tr><td><i>text<sub>3</sub></i></td><td>2</td><td>L</td><td>L</td></tr>
<tr><td><i>text<sub>4</sub></i>·<i>text<sub>5</sub></i></td><td>1</td><td>L</td><td>R</td></tr>
<tr><td><i>text<sub>6</sub></i></td><td>0</td><td>R</td><td>L</td></tr>
</table>

Example 2: *text<sub>1</sub>*·RLI·*text<sub>2</sub>*·LRI·*text<sub>3</sub>*·PDI·*text<sub>4</sub>*·PDI·RLI·*text<sub>5</sub>*·PDI·*text<sub>6</sub>*

<table class="subtle">
<tr><th>Isolating Run Sequence</th><th>Embedding Level</th><th>sos</th><th>eos</th></tr>
<tr><td><i>text<sub>1</sub></i>·<span class="control">RLI</span>·<span class="control">PDI</span>·<span class="control">RLI</span>·<span class="control">PDI</span>·<i>text<sub>6</sub></i></td><td>0</td><td>L</td><td>L</td></tr>
<tr><td><i>text<sub>2</sub></i>·<span class="control">LRI</span>·<span class="control">PDI</span>·<i>text<sub>4</sub></i></td><td>1</td><td>R</td><td>R</td></tr>
<tr><td><i>text<sub>3</sub></i></td><td>2</td><td>L</td><td>L</td></tr>
<tr><td><i>text<sub>5</sub></i></td><td>1</td><td>R</td><td>R</td></tr>
</table>

Example 3: *text<sub>1</sub>*·RLE·*text<sub>2</sub>*·LRI·*text<sub>3</sub>*·RLE·*text<sub>4</sub>*·PDI·*text<sub>5</sub>*·PDF·*text<sub>6</sub>*

<table class="subtle">
<tr><th>Isolating Run Sequence</th><th>Embedding Level</th><th>sos</th><th>eos</th></tr>
<tr><td><i>text<sub>1</sub></i></td><td>0</td><td>L</td><td>R</td></tr>
<tr><td><i>text<sub>2</sub></i>·<span class="control">LRI</span>·<span class="control">PDI</span>·<i>text<sub>5</sub></i></td><td>1</td><td>R</td><td>R</td></tr>
<tr><td><i>text<sub>3</sub></i></td><td>2</td><td>L</td><td>R</td></tr>
<tr><td><i>text<sub>4</sub></i></td><td>3</td><td>R</td><td>R</td></tr>
<tr><td><i>text<sub>6</sub></i></td><td>0</td><td>R</td><td>L</td></tr>
</table>

