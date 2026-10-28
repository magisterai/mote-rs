# 3.1.2 Matching Explicit Directional Formatting Characters

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-3-basic-display-algorithm.md) · [Next](006-3-1-3-paired-brackets.md)

<a id="Matching_Explicit_Directional_Formatting_Characters"></a>

### 3.1.2  [Matching Explicit Directional Formatting Characters](005-3-1-2-matching-explicit-directional-formatting-characters.md#Matching_Explicit_Directional_Formatting_Characters)

<a id="BD8"></a>

***[BD8](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD8).*** An *isolate initiator* is a character of type LRI, RLI, or FSI.

As rules [X5a](009-non-formatting-characters.md#X5a) through [X5c](009-non-formatting-characters.md#X5c) will specify,
an isolate initiator raises the embedding level for the characters following it
when the rules enforcing the depth limit allow it.

<a id="BD9"></a>

***[BD9](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD9).*** The *matching PDI* for a given isolate initiator
is the one determined by the following algorithm:

- Initialize a counter to one.
- Scan the text following the isolate initiator to the end of the paragraph
  while incrementing the counter at every isolate initiator,
  and decrementing it at every PDI.
- Stop at the first PDI, if any, for which the counter is decremented to zero.
- If such a PDI was found, it is the matching PDI for the given isolate initiator.
  Otherwise, there is no matching PDI for it.

Note that all formatting characters except for isolate initiators and PDIs are ignored when finding the matching PDI.

Note that this algorithm assigns a matching PDI (or lack of one) to an isolate initiator
whether the isolate initiator raises the embedding level or is prevented from doing so by the depth limit rules.

As rule [X6a](009-non-formatting-characters.md#X6a) will specify,
a matching PDI returns the embedding level to the value it had before the isolate initiator that the PDI matches.
The PDI itself is assigned the new embedding level.
If it does not match any isolate initiator, or if the isolate initiator did not raise the embedding level,
it leaves the embedding level unchanged.
Thus, an isolate initiator and its matching PDI are always assigned the same explicit embedding level, which is the one outside the isolate.
In the later stages of the Bidirectional Algorithm, an isolate initiator and its matching PDI function as invisible neutral characters,
and their embedding level then helps ensure that
the isolate has the effect of a neutral character on the display order of the text outside it,
and is assigned the corresponding display position in the surrounding text.

<a id="BD10"></a>

***[BD10](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD10).*** An *embedding initiator* is a character of type LRE, RLE, LRO, or RLO.

Note that an embedding initiator initiates either a directional embedding or a directional override;
its name omits overrides only for conciseness.

As rules [X2](008-3-3-1-the-paragraph-level.md#X2) through [X5](008-3-3-1-the-paragraph-level.md#X5) will specify,
an embedding initiator raises the embedding level for the characters following it
when the rules enforcing the depth limit allow it.

<a id="BD11"></a>

***[BD11](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD11).*** The *matching PDF* for a given embedding initiator
is the one determined by the following algorithm:

- Initialize a counter to one.
- Scan the text following the embedding initiator:
  - At an isolate initiator, skip past the matching PDI,
    or if there is no matching PDI, to the end of the paragraph.
  - At the end of a paragraph, or at a PDI that matches an isolate initiator
    whose text location is before the embedding initiator's location,
    stop: the embedding initiator has no matching PDF.
  - At an embedding initiator, increment the counter.
  - At a PDF, decrement the counter. If its new value is zero, stop: this is the matching PDF.

Note that this algorithm assigns a matching PDF (or lack of one)
to an embedding initiator whether it raises the embedding level or is prevented from doing so by the depth limit rules.

Although the algorithm above serves to give a precise meaning to the term “matching PDF”,
note that the overall Bidirectional Algorithm never actually calls for its use to find the PDF matching an embedding initiator.
Instead, rules [X1](008-3-3-1-the-paragraph-level.md#X1) through [X7](009-non-formatting-characters.md#X7) specify a mechanism
that determines what embedding initiator scope, if any, is terminated by a PDF, i.e. which valid embedding initiator a PDF matches.

As rule [X7](009-non-formatting-characters.md#X7) will specify,
a matching PDF returns the embedding level to the value it had before the embedding initiator that the PDF matches.
If it does not match any embedding initiator, or if the embedding initiator did not raise the embedding level,
a PDF leaves the embedding level unchanged.

As rule [X9](010-3-3-3-preparations-for-implicit-processing.md#X9) will specify,
once explicit directional formatting characters have been used to assign embedding levels to the characters in a paragraph,
embedding initiators and PDFs are removed (or virtually removed) from the paragraph.
Thus, the embedding levels assigned to the embedding initiators and PDFs themselves are irrelevant.
In this, embedding initiators and PDFs differ from isolate initiators and PDIs,
which continue to play a part in determining the paragraph’s display order as mentioned above.

<a id="BD12"></a>

***[BD12](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD12).*** The *directional isolate status* is a Boolean value set by using isolate formatting characters:
it is true when the current embedding level was started by an isolate initiator.

<a id="BD13"></a>

***[BD13](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD13).*** An *isolating run sequence* is a maximal sequence of level runs such that
for all level runs except the last one in the sequence,
the last character of the run is an isolate initiator whose matching PDI is the first character of the next level run in the sequence.
It is maximal in the sense that if the first character of the first level run in the sequence is a PDI, it must not match any isolate initiator,
and if the last character of the last level run in the sequence is an isolate initiator, it must not have a matching PDI.

The set of isolating run sequences in a paragraph can be computed by the following algorithm:

- Start with an empty set of isolating run sequences.
- For each level run in the paragraph whose first character is not a PDI, or is a PDI that does not match any isolate initiator:
  - Create a new level run sequence, and initialize it to contain just that level run.
  - While the level run currently last in the sequence ends with an isolate initiator that has a matching PDI, append the level run containing the matching PDI to the sequence.
    (Note that this matching PDI must be the first character of its level run.)
  - Add the resulting sequence of level runs to the set of isolating run sequences.

Note that:

- Each level run in a paragraph belongs to exactly one isolating run sequence.
- In the absence of isolate initiators, each isolating run sequence in a paragraph consists of exactly one level run,
  and each level run constitutes a separate isolating run sequence.
- For any two adjacent level runs in an isolating run sequence,
  since one ends with an isolate initiator whose matching PDI starts the other,
  the two must have the same embedding level.
  Thus, all the level runs in an isolating run sequence have the same embedding level.
- When an isolate initiator raises the embedding level,
  both the isolate initiator and its matching PDI, if any, get the original embedding level, not the raised one.
  Thus, if the matching PDI does not immediately follow the isolate initiator in the paragraph,
  the isolate initiator is the last character in its level run, but the matching PDI, if any,
  is the first character of its level run and immediately follows the isolate initiator in the same isolating run sequence.
  On the other hand, the level run following the isolate initiator in the paragraph starts a new isolating run sequence,
  and the level run preceding the matching PDI (if any) in the paragraph ends its isolating run sequence.

In the following examples, assume that:

- The paragraph embedding level is 0.
- No character sequence *text<sub>i</sub>* contains explicit formatting characters or paragraph separators.
- The dots are used only to improve the example's visual clarity; they are not part of the text.
- The characters in the paragraph text are assigned embedding levels as loosely described above such that they form the set of level runs given in each example.

#### Example 1

Paragraph text: *text<sub>1</sub>*·RLE·*text<sub>2</sub>*·PDF·RLE·*text<sub>3</sub>*·PDF·*text<sub>4</sub>*

Level runs:

- *text<sub>1</sub>* – level 0
- *text<sub>2</sub>*·*text<sub>3</sub>* – level 1
- *text<sub>4</sub>* – level 0

Resulting isolating run sequences:

- *text<sub>1</sub>* – level 0
- *text<sub>2</sub>*·*text<sub>3</sub>* – level 1
- *text<sub>4</sub>* – level 0

#### Example 2

Paragraph text: *text<sub>1</sub>*·RLI·*text<sub>2</sub>*·PDI·RLI·*text<sub>3</sub>*·PDI·*text<sub>4</sub>*

Level runs:

- *text<sub>1</sub>*·RLI – level 0
- *text<sub>2</sub>* – level 1
- PDI·RLI – level 0
- *text<sub>3</sub>* – level 1
- PDI·*text<sub>4</sub>* – level 0

Resulting isolating run sequences:

- *text<sub>1</sub>*·RLI
  PDI·RLI
  PDI·*text<sub>4</sub>* – level 0
- *text<sub>2</sub>* – level 1
- *text<sub>3</sub>* – level 1

#### Example 3

Paragraph text: *text<sub>1</sub>*·RLI·*text<sub>2</sub>*·LRI·*text<sub>3</sub>*·RLE·*text<sub>4</sub>*·PDF·*text<sub>5</sub>*·PDI·*text<sub>6</sub>*·PDI·*text<sub>7</sub>*

Level runs:

- *text<sub>1</sub>*·RLI – level 0
- *text<sub>2</sub>*·LRI – level 1
- *text<sub>3</sub>* – level 2
- *text<sub>4</sub>* – level 3
- *text<sub>5</sub>* – level 2
- PDI·*text<sub>6</sub>* – level 1
- PDI·*text<sub>7</sub>* – level 0

Resulting isolating run sequences:

- *text<sub>1</sub>*·RLI
  PDI·*text<sub>7</sub>* – level 0
- *text<sub>2</sub>*·LRI
  PDI·*text<sub>6</sub>* – level 1
- *text<sub>3</sub>* – level 2
- *text<sub>4</sub>* – level 3
- *text<sub>5</sub>* – level 2

As rule [X10](010-3-3-3-preparations-for-implicit-processing.md#X10) will specify, an isolating run sequence is the unit
to which the rules following it are applied,
and the last character of one level run in the sequence is considered to be
immediately followed by the first character of the next level run in the sequence
during this phase of the algorithm.
Since those rules are based on the characters' implicit bidirectional types,
an isolate really does have the same effect on the ordering of the text surrounding it
as a neutral character –
or, to be more precise, a pair of neutral characters, the isolate initiator and the PDI,
which behave in those rules just like neutral characters.

