# 4.4 Bidirectional Conformance Testing

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](015-4-3-2-hl4-example-2-for-program-text.md) · [Next](017-6-2-vertical-text.md)

<a id="Bidi_Conformance_Testing"></a>

### 4.4  [Bidirectional Conformance Testing](016-4-4-bidirectional-conformance-testing.md#Bidi_Conformance_Testing)

The *Unicode Character Database* [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]
includes two files that provide conformance tests for implementations of
the Bidirectional Algorithm [[Tests9](https://www.unicode.org/reports/tr41/tr41-36.html#Tests9)].
One of the test files, `BidiTest.txt`, comprises exhaustive test sequences
of bidirectional types up to a given length, currently 4. The other test file, `BidiCharacterTest.txt`,
contains test sequences of explicit code points, including, for example, bracket pairs.
The format of each test file is described in the header of that file.

<a id="Implementation_Notes"></a>

## 5  [Implementation Notes](016-4-4-bidirectional-conformance-testing.md#Implementation_Notes)

<a id="Reference_Code"></a>

### 5.1  [Reference Code](016-4-4-bidirectional-conformance-testing.md#Reference_Code)

Reference implementations of the Bidirectional Algorithm written in C and in Java are available.
The source code can be downloaded from [[Code9](https://www.unicode.org/reports/tr41/tr41-36.html#Code9)].
Implementers are encouraged to use these resources to test their implementations.

The reference code is designed to follow the steps of the algorithm without applying
any optimizations. An example of an effective optimization is to first test for right-to-left characters and
invoke the Bidirectional Algorithm only if they are present. Another example of optimization is in matching bracket pairs.
The bidirectional bracket pairs (the characters with Bidi\_Paired\_Bracket\_Type property values Open and Close)
constitute a subset of the characters with bidirectional type ON. Conversely, the characters with a bidirectional type
distinct from ON have the Bidi\_Paired\_Bracket\_Type property value None. Therefore, lookup of Bidi\_Paired\_Bracket\_Type
property values for the identification of bracket pairs can be optimized by restricting the processing to characters
whose bidirectional type is ON.

An online demo is also available at [[Demo9](https://www.unicode.org/reports/tr41/tr41-36.html#Demo9)],
which shows the results of the Bidirectional Algorithm, as well as the embedding levels and
the rules invoked for each character. Implementers are cautioned when using that online demo that it implements
the rules for UBA as of Version 6.2, and has not been updated for the major changes to UBA in Version 6.3 and
subsequent versions. The online demo also does not handle supplemental characters gracefully.

<a id="Retaining_Explicit_Formatting_Characters"></a>
<a id="Retaining_Format_Codes"></a>

### 5.2  [Retaining BNs and Explicit Formatting Characters](016-4-4-bidirectional-conformance-testing.md#Retaining_Explicit_Formatting_Characters)

Some implementations may wish to retain the explicit directional embedding and override formatting characters
and BNs when running the algorithm.
In fact, retention of these formatting characters and BNs is important to
users who need to display a graphic representation of hidden characters, and who thus need to obtain
their visual positions for display.

The following describes
how this may be done by implementations that do retain these characters
through the steps of the algorithm. Note that this description is an informative
implementation guideline; it should provide the same results as the explicit algorithm above, but
in case of any deviation the explicit algorithm is the normative statement for conformance.

- In rules [X2](008-3-3-1-the-paragraph-level.md#X2) through [X5](008-3-3-1-the-paragraph-level.md#X5),
  insert an initial step setting the explicit embedding or override character's
  embedding level to the embedding level of the last entry on the directional status stack. This
  applies to RLE, LRE, RLO, and LRO.
- In rule [X6](009-non-formatting-characters.md#X6), remove the exclusion of
  BN characters for the purposes of setting embedding levels. Continue
  not updating the character types of these characters.
  In other words, apply the first bullet point of the rule to all types except
  B, RLE, LRE, RLO, LRO, PDF, RLI, LRI, FSI, and PDI.
- In rule [X7](009-non-formatting-characters.md#X7), add a final step setting
  the embedding level of the PDF to the embedding level of the last entry on the
  directional status stack, in all cases.
- In rule [X9](010-3-3-3-preparations-for-implicit-processing.md#X9), do not remove any characters, but turn
  all RLE, LRE, RLO, LRO, and PDF characters into BN.
- In rule [X10](010-3-3-3-preparations-for-implicit-processing.md#X10),
  when determining the sos and eos for an isolating run sequence,
  skip over any BNs when looking for the character preceding the isolating run sequence's first character and following its last character.
  Do the same when determining if the last character of the sequence is an isolate initiator.
- In rule [W1](011-3-3-4-resolving-weak-types.md#W1), search backward from each NSM to the first character in the isolating run sequence whose
  bidirectional type is not BN, and set the NSM to ON if it is an isolate initiator or PDI, and to its type otherwise.
  If the NSM is the first non-BN character, change the NSM to the type of *sos*.
- In rule [W4](011-3-3-4-resolving-weak-types.md#W4), scan past BN types that are adjacent to ES or CS.
- In rule [W5](011-3-3-4-resolving-weak-types.md#W5), change all appropriate sequences of ET and BN, not just ET.
- In rule [W6](011-3-3-4-resolving-weak-types.md#W6), change all BN types adjacent to ET, ES, or CS to ON as well.
- In rule [W7](011-3-3-4-resolving-weak-types.md#W7), scan past BN.
- In rules [N0](011-3-3-4-resolving-weak-types.md#N0)–[N2](011-3-3-4-resolving-weak-types.md#N2), treat BNs that adjoin
  neutrals the same as those neutrals.
- In rules [I1](012-3-3-6-resolving-implicit-levels.md#I1) and [I2](012-3-3-6-resolving-implicit-levels.md#I2), ignore BN.
- In rule [L1](012-3-3-6-resolving-implicit-levels.md#L1), include the embedding and override formatting characters and BNs
  together with whitespace characters and isolate formatting characters in the sequences
  whose level gets reset before a separator or line break. Resolve any LRE, RLE, LRO, RLO, PDF, or BN to the level of the preceding character if there is one,
  and otherwise to the base level.

<a id="Usage"></a>

## 6  [Usage](016-4-4-bidirectional-conformance-testing.md#Usage)

<a id="Joiners"></a>

### 6.1  [Joiners](016-4-4-bidirectional-conformance-testing.md#Joiners)

As described under [X9](010-3-3-3-preparations-for-implicit-processing.md#X9), the *zero width joiner* and
*non-joiner* affect the
shaping of the adjacent characters—those that are adjacent in the original backing-store
order—even though those characters may end up being rearranged to be non-adjacent by the
Bidirectional Algorithm. To determine the joining behavior of a particular
character after applying the Bidirectional Algorithm, there are two main
strategies:

- When shaping, an implementation can refer back to the original backing store to see if there
  were adjacent ZWNJ or ZWJ characters.
- Alternatively, the implementation can replace ZWJ and ZWNJ by an out-of-band character
  property associated with those adjacent characters, so that the information does not interfere
  with the Bidirectional Algorithm and the information is preserved across rearrangement of those
  characters. Once the Bidirectional Algorithm has been applied, that out-of-band information can then be
  used for proper shaping.

