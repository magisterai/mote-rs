# Non-formatting characters

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-3-3-1-the-paragraph-level.md) · [Next](010-3-3-3-preparations-for-implicit-processing.md)

<a id="X5a"></a>

***[X5a](009-non-formatting-characters.md#X5a).**
With each RLI, perform the following steps:*

- *Set the RLI’s embedding level to the embedding level of the last entry on the directional status stack.*
- *If the directional override status of the last entry on the directional status stack is not neutral,
  reset the current character type from RLI to L if the override status is left-to-right,
  and to R if the override status is right-to-left.*
- *Compute the least **odd** embedding level greater than the embedding level of the last entry on the directional status stack.*
- *If this new level would be valid
  and the overflow isolate count and the overflow embedding count are both zero,
  then this RLI is valid.
  Increment the valid isolate count by one, and push an entry consisting of the new embedding level, **neutral** directional override status, and **true** directional isolate status onto the directional status stack.*
- *Otherwise, this is an overflow RLI.
  Increment the overflow isolate count by one, and leave all other variables unchanged.*

<a id="X5b"></a>

***[X5b](009-non-formatting-characters.md#X5b).**
With each LRI, perform the following steps:*

- *Set the LRI’s embedding level to the embedding level of the last entry on the directional status stack.*
- *If the directional override status of the last entry on the directional status stack is not neutral,
  reset the current character type from LRI to L if the override status is left-to-right,
  and to R if the override status is right-to-left.*
- *Compute the least **even** embedding level greater than the embedding level of the last entry on the directional status stack.*
- *If this new level would be valid
  and the overflow isolate count and the overflow embedding count are both zero,
  then this LRI is valid.
  Increment the valid isolate count by one, and push an entry consisting of the new embedding level, **neutral** directional override status, and **true** directional isolate status onto the directional status stack.*
- *Otherwise, this is an overflow LRI.
  Increment the overflow isolate count by one, and leave all other variables unchanged.*

<a id="X5c"></a>

***[X5c](009-non-formatting-characters.md#X5c).**
With each FSI, apply rules [P2](008-3-3-1-the-paragraph-level.md#P2) and [P3](008-3-3-1-the-paragraph-level.md#P3)
to the sequence of characters between the FSI and its matching PDI,
or if there is no matching PDI, the end of the paragraph,
as if this sequence of characters were a paragraph.
If these rules decide on paragraph embedding level 1,
treat the FSI as an RLI in rule [X5a](009-non-formatting-characters.md#X5a).
Otherwise, treat it as an LRI in rule [X5b](009-non-formatting-characters.md#X5b).*

Note that the new embedding level is not set
to the paragraph embedding level determined by P2 and P3.
It goes up by one or two levels, as it would for an LRI or RLI.

#### Non-formatting characters

<a id="X6"></a>

***[X6](009-non-formatting-characters.md#X6).** For all types besides B, BN, RLE, LRE, RLO, LRO, PDF,
RLI, LRI, FSI, and PDI:*

- *Set the current character’s embedding level to the embedding level of the last entry on the directional status stack.*
- *Whenever the directional override status of the last entry on the directional status stack is not neutral, reset the current
  character type according to the directional override status of the last entry on the directional status stack.*

In other words, if the directional override status of the last entry on the directional status stack is neutral,
then characters retain their normal types:
Arabic characters stay AL, Latin characters stay L, spaces stay WS, and so on. If the directional
override status is right-to-left, then characters become R. If the directional override status is left-to-right, then
characters become L.

Note that the current embedding level is not changed by this rule.

#### Terminating Isolates

A PDI terminates the scope of the isolate initiator it matches.
It also terminates the scopes of all embedding initiators
within the scope of the matched isolate initiator for which a matching PDF has not been encountered.
If it does not match any isolate initiator, it is ignored.

<a id="X6a"></a>

***[X6a](009-non-formatting-characters.md#X6a).** With each PDI, perform the following steps:*

- *If the overflow isolate count is greater than zero, this PDI matches an overflow isolate initiator.
  Decrement the overflow isolate count by one.*
- *Otherwise, if the valid isolate count is zero, this PDI does not match any isolate initiator, valid or overflow.
  Do nothing.*
- *Otherwise, this PDI matches a valid isolate initiator. Perform the following steps:*
  - *Reset the overflow embedding count to zero.*
    (This terminates the scope of those overflow embedding initiators
    within the scope of the matched isolate initiator
    whose scopes have not been terminated by a matching PDF, and which thus lack a matching PDF.)
  - *While the directional isolate status of the last entry on the stack is false,
    pop the last entry from the directional status stack.*
    (This terminates the scope of those valid embedding initiators
    within the scope of the matched isolate initiator
    whose scopes have not been terminated by a matching PDF, and which thus lack a matching PDF.
    Given that the valid isolate count is non-zero,
    the directional status stack before this step is executed
    must contain an entry with directional isolate status true,
    and thus after this step is executed the last entry on the stack will indeed have a true directional isolate status,
    i.e. represent the scope of the matched isolate initiator.
    This cannot be the stack's first entry, which always belongs to the paragraph level and has a false directional status,
    so there is at least one more entry below it on the stack.)
  - *Pop the last entry from the directional status stack
    and decrement the valid isolate count by one.*
    (This terminates the scope of the matched isolate initiator.
    Since the preceding step left the stack with at least two entries, this pop does not leave the stack empty.)
- *In all cases, look up
  the last entry on the directional status stack left after the steps above and:*
  - *Set the PDI’s level to the entry's embedding level.*
  - *If the entry's directional override status is not neutral,
    reset the current character type from PDI to L if the override status is left-to-right,
    and to R if the override status is right-to-left.*

Note that the level assigned to an isolate initiator
is always the same as that assigned to the matching PDI.

#### Terminating Embeddings and Overrides

A PDF terminates the scope of the embedding initiator it matches.
If it does not match any embedding initiator, it is ignored.

<a id="X7"></a>

***[X7](009-non-formatting-characters.md#X7).** With each PDF,
perform the following steps:*

- *If the overflow isolate count is greater than zero, do nothing.*
  (This PDF is within the scope of an overflow isolate initiator.
  It either matches and terminates the scope of an overflow embedding initiator within that overflow isolate,
  or does not match any embedding initiator.)
- *Otherwise, if the overflow embedding count is greater than zero, decrement it by one.*
  (This PDF matches and terminates the scope of an overflow embedding initiator
  that is not within the scope of an overflow isolate initiator.)
- *Otherwise, if the directional isolate status of the last entry on the directional status stack is false,
  and the directional status stack contains at least two entries,
  pop the last entry from the directional status stack.*
  (This PDF matches and terminates the scope of a valid embedding initiator.
  Since the stack has at least two entries, this pop does not leave the stack empty.)
- *Otherwise, do nothing.*
  (This PDF does not match any embedding initiator.)

#### End of Paragraph

<a id="X8"></a>

***[X8](009-non-formatting-characters.md#X8).** All explicit directional embeddings, overrides
and isolates are completely
terminated at the end of each paragraph.*

- Explicit paragraph separators (bidirectional character type B) indicate the end of a paragraph. As such, they
  are **not** included in any embedding, override or isolate. They are simply assigned
  the paragraph embedding level.

