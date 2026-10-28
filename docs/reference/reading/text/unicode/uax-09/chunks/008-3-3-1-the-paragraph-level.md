# 3.3.1 The Paragraph Level

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-3-3-resolving-embedding-levels.md) · [Next](009-non-formatting-characters.md)

<a id="The_Paragraph_Level"></a>

### 3.3.1  [The Paragraph Level](008-3-3-1-the-paragraph-level.md#The_Paragraph_Level)

<a id="P1"></a>

***[P1](008-3-3-1-the-paragraph-level.md#P1).** Split the text into separate paragraphs. A paragraph separator (type B) is kept
with the previous paragraph. Within each paragraph, apply all the other rules of this algorithm.*

<a id="P2"></a>

***[P2](008-3-3-1-the-paragraph-level.md#P2).** In each paragraph, find the first character of type L, AL, or R
while skipping over any characters
between an isolate initiator and its matching PDI or,
if it has no matching PDI, the end of the paragraph.*

Note that:

- Because paragraph separators delimit text in this algorithm,
  the character found by this rule will generally be the first
  strong character after a paragraph separator or at the very beginning of the text.
- The characters between an isolate initiator and its matching PDI are ignored by this rule
  because a directional isolate is supposed to have the same effect on the ordering of the surrounding text as a neutral character,
  and the rule ignores neutral characters.
- The characters between an isolate initiator and its matching PDI are ignored by this rule
  even if the depth limit (as defined in rules [X5a](009-non-formatting-characters.md#X5a) through [X5c](009-non-formatting-characters.md#X5c) below)
  prevents the isolate initiator from raising the embedding level.
  This is meant to make the rule easier to implement.
- Embedding initiators (but not the characters within the embedding)
  are ignored in this rule.

<a id="P3"></a>

***[P3](008-3-3-1-the-paragraph-level.md#P3).** If a character is found in [P2](008-3-3-1-the-paragraph-level.md#P2) and it is of type AL or R, then set the
paragraph embedding level to one; otherwise, set it to zero.*

Whenever a higher-level protocol specifies the paragraph level, rules [P2](008-3-3-1-the-paragraph-level.md#P2) and [P3](008-3-3-1-the-paragraph-level.md#P3) may be overridden: see [HL1](014-4-1-boundary-neutrals.md#HL1).

<a id="Explicit_Levels_and_Directions"></a>

### 3.3.2  [Explicit Levels and Directions](008-3-3-1-the-paragraph-level.md#Explicit_Levels_and_Directions)

This phase of the algorithm determines *explicit* [embedding levels](004-3-basic-display-algorithm.md#BD2): levels introduced by explicit
[directional formatting characters](002-1-introduction.md#Directional_Formatting_Characters) (embedding, override, and isolate). This phase tracks how they
nest, eventually producing a tagging of different ranges of text with their embedding levels, the levels increasing with deeper nesting.

This is done by applying the explicit level rule [X1](008-3-3-1-the-paragraph-level.md#X1), which performs a logical pass over the
paragraph, applying rules [X2](008-3-3-1-the-paragraph-level.md#X2)–[X8](009-non-formatting-characters.md#X8) to each characters in turn.
The following variables are used during this pass:

- A *directional status stack* of max\_depth+2 entries where each entry consists of:
  - An embedding level, which is at least zero and at most max\_depth.
  - A directional override status.
  - A directional isolate status.In addition to supporting the usual destructive “pop” operation,
  the stack also allows read access to its last (i.e. top) entry without popping it.
  For efficiency, that last entry can be kept in a separate variable instead of on the directional status stack,
  but it is easier to explain the algorithm without that optimization.
  At the start of the pass, the directional status stack is initialized to an entry reflecting the paragraph embedding level,
  with the directional override status neutral and the directional isolate status false;
  this entry is not popped off until the end of the paragraph.
  During the pass, the directional status stack always contains entries for all the directional embeddings, overrides,
  and isolates within which the current position lies
  – except those that would overflow the depth limit –
  in addition to the paragraph level entry at the start of the stack.
  The last entry reflects the innermost valid scope within which the pass's current position lies.
  Implementers may find it useful to include more information in each stack entry.
  For example, in an isolate entry, the location of the isolate initiator could be used to create
  a mapping from the location of each valid isolate initiator to the location of the matching PDI, or vice versa.
  However, such optimizations are beyond the scope of this specification.
- A counter called the *overflow isolate count*.  
  This reflects the number of isolate initiators
  that were encountered in the pass so far without encountering their matching PDIs,
  but were invalidated by the depth limit and thus are not reflected in the directional status stack.
  They are nested one within the other and the stack's last scope.
  This count is used to determine whether a newly encountered PDI matches and terminates the scope of an overflow isolate initiator, thus decrementing the count,
  as opposed to possibly matching and terminating the scope of a valid isolate initiator, which should result in popping its entry off the directional status stack.
  It is also used to determine whether a newly encountered PDF falls within the scope of an overflow isolate initiator
  and can thus be completely ignored
  (regardless of whether it matches an embedding initiator within the same overflow isolate or nothing at all).
- A counter called the *overflow embedding count*.  
  This reflects the number of embedding initiators
  that were encountered in the pass so far without encountering their matching PDF, or encountering the PDI of an isolate within which they are nested,
  but were invalidated by the depth limit, and thus are not reflected in the directional status stack.
  They are nested one within the other and the stack's last scope.
  This count is used to determine whether a newly encountered PDF matches and terminates the scope of an overflow embedding initiator, thus decrementing the count,
  as opposed to possibly matching and terminating the scope of a valid embedding initiator, which should result in popping its entry off the directional status stack.
  However, this count does *not* include embedding initiators encountered within the scope of an overflow isolate
  (i.e. encountered when the overflow isolate count above is greater than zero).
  The scopes of those overflow embedding initiators fall within the scope of an overflow isolate and are terminated when the overflow isolate count turns zero.
  Thus, they do not need to be counted.
  In fact, if they were counted in the overflow embedding count,
  there would be no way to properly update that count when a PDI matching an overflow isolate initiator is encountered:
  without a stack of the overflow scopes, there would be no way to know how many (if any) overflow embedding initiators fall within the scope of that overflow isolate.
- A counter called the *valid isolate count*.  
  This reflects the number of isolate initiators
  that were encountered in the pass so far without encountering their matching PDIs,
  and have been judged valid by the depth limit, i.e. all the entries on the stack with a true directional isolate status.
  It ignores all embeddings and overrides,
  and is used to determine without having to look through the directional status stack
  whether a PDI encountered by the pass when the overflow isolate count is zero matches some valid isolate initiator or nothing at all.
  A PDI encountered when this counter is above zero terminates the scope of the isolate initiator it matches,
  as well as the embeddings and overrides nested within it – which appear above it on the stack, or are reflected in the overflow embedding count.

Note that there is no need for a valid embedding count in order to tell
whether a PDF encountered by the pass matches a valid embedding initiator or nothing at all.
That can be decided by checking the directional isolate status of the last entry on the directional status stack and the number of entries on the stack.
If the last entry has a true directional isolate status, it is for a directional isolate within whose scope the PDF lies.
Since the PDF cannot match an embedding initiator outside that isolate,
and there are no embedding entries within the isolate, it matches nothing at all.
And if the last entry has a false directional isolate status, but is also the
only entry on the stack, it belongs to paragraph level, and thus once again
the PDF matches nothing at all.

As each character is processed,
these variables’ values are modified and the character’s explicit embedding level is set
as defined by rules [X2](008-3-3-1-the-paragraph-level.md#X2) through [X8](009-non-formatting-characters.md#X8)
on the basis of the character’s bidirectional type and the variables’ current values.

<a id="X1"></a>

***[X1](008-3-3-1-the-paragraph-level.md#X1).** At the beginning of a paragraph, perform the following steps:*

- *Set the stack to empty.*
- *Push onto the stack an entry consisting of the paragraph embedding level, a **neutral** directional override status, and a **false** directional isolate status.*
- *Set the overflow isolate count to zero.*
- *Set the overflow embedding count to zero.*
- *Set the valid isolate count to zero.*
- *Process each character iteratively, applying rules [X2](008-3-3-1-the-paragraph-level.md#X2) through
  [X8](009-non-formatting-characters.md#X8).
  Only embedding levels from 0 through max\_depth are valid in this phase.*
  (Note that in the resolution of levels in rules [I1](012-3-3-6-resolving-implicit-levels.md#I1) and [I2](012-3-3-6-resolving-implicit-levels.md#I2),
  the maximum embedding level of max\_depth+1 can be reached.)

#### Explicit Embeddings

<a id="X2"></a>

***[X2](008-3-3-1-the-paragraph-level.md#X2).** With each RLE, perform the following steps:*

- *Compute the least **odd** embedding level greater than the embedding level of the last entry on the directional status stack.*
- *If this new level would be valid,
  and the overflow isolate count and overflow embedding count are both zero,
  then this RLE is valid.
  Push an entry consisting of the new embedding level, **neutral** directional override status, and **false** directional isolate status onto the directional status stack.*
- *Otherwise, this is an overflow RLE.
  If the overflow isolate count is zero, increment the overflow embedding count by one. Leave all other variables unchanged.*

For example, assuming the overflow counts are both zero,
level 0 → 1; levels 1, 2 → 3; levels 3, 4 → 5; and so on.
At max\_depth or if either overflow count is non-zero, the level remains the same (overflow RLE).

<a id="X3"></a>

***[X3](008-3-3-1-the-paragraph-level.md#X3).** With each LRE, perform the following steps:*

- *Compute the least **even** embedding level greater than the embedding level of the last entry on the directional status stack.*
- *If this new level would be valid,
  and the overflow isolate count and overflow embedding count are both zero,
  then this LRE is valid.
  Push an entry consisting of the new embedding level, **neutral** directional override status, and **false** directional isolate status onto the directional status stack.*
- *Otherwise, this is an overflow LRE.
  If the overflow isolate count is zero, increment the overflow embedding count by one. Leave all other variables unchanged.*

For example, assuming the overflow counts are both zero,
levels 0, 1 → 2; levels 2, 3 → 4; levels 4, 5 → 6; and so on.
At max\_depth or max\_depth-1 (which, being even, would have to go to max\_depth+1) or if either overflow count is non-zero, the level remains the same (overflow LRE).

#### Explicit Overrides

An explicit directional override sets the embedding level in the same way the explicit
embedding formatting characters do, but also changes the bidirectional character type of affected characters to the
override direction.

<a id="X4"></a>

***[X4](008-3-3-1-the-paragraph-level.md#X4).** With each RLO, perform the following steps:*

- *Compute the least **odd** embedding level greater than the embedding level of the last entry on the directional status stack.*
- *If this new level would be valid,
  and the overflow isolate count and overflow embedding count are both zero,
  then this RLO is valid.
  Push an entry consisting of the new embedding level, **right-to-left** directional override status, and **false** directional isolate status onto the directional status stack.*
- *Otherwise, this is an overflow RLO.
  If the overflow isolate count is zero, increment the overflow embedding count by one. Leave all other variables unchanged.*

<a id="X5"></a>

***[X5](008-3-3-1-the-paragraph-level.md#X5).** With each LRO, perform the following steps:*

- *Compute the least **even** embedding level greater than the embedding level of the last entry on the directional status stack.*
- *If this new level would be valid,
  and the overflow isolate count and overflow embedding count are both zero,
  then this LRO is valid.
  Push an entry consisting of the new embedding level, **left-to-right** directional override status, and **false** directional isolate status onto the directional status stack.*
- *Otherwise, this is an overflow LRO.
  If the overflow isolate count is zero, increment the overflow embedding count by one. Leave all other variables unchanged.*

#### Isolates

