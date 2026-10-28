# 3.1.3 Paired Brackets

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-3-1-2-matching-explicit-directional-formatting-characters.md) · [Next](007-3-3-resolving-embedding-levels.md)

<a id="Paired_Brackets"></a>

### 3.1.3  [Paired Brackets](006-3-1-3-paired-brackets.md#Paired_Brackets)

The following definitions utilize the normative properties
Bidi\_Paired\_Bracket and Bidi\_Paired\_Bracket\_Type defined in the `BidiBrackets.txt` file
[[Data9](https://www.unicode.org/reports/tr41/tr41-36.html#Data9)] of the *Unicode Character Database*
[[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)].

<a id="BD14"></a>

***[BD14](006-3-1-3-paired-brackets.md#BD14).*** An *opening paired bracket* is a character whose
Bidi\_Paired\_Bracket\_Type property value is Open and whose current bidirectional character type is ON.

<a id="BD15"></a>

***[BD15](006-3-1-3-paired-brackets.md#BD15).*** A *closing paired bracket* is a character whose Bidi\_Paired\_Bracket\_Type
property value is Close and whose current bidirectional character type is ON.

<a id="BD16"></a>

***[BD16](006-3-1-3-paired-brackets.md#BD16).*** A *bracket pair* is a pair of characters consisting of an opening
paired bracket and a closing paired bracket such that the Bidi\_Paired\_Bracket property
value of the former or its canonical equivalent equals the latter or its canonical equivalent
and which are algorithmically identified at specific text positions
within an isolating run sequence. The following algorithm identifies all of the bracket pairs
in a given isolating run sequence:

- Create a fixed-size stack for exactly 63 elements each consisting of a bracket character and a text
  position. Initialize it to empty.
- Create a list of resulting bracket pairs for elements each consisting of two text positions, one for an opening
  paired bracket and the other for a corresponding closing paired bracket. Initialize it to empty.
- Inspect each character in the isolating run sequence in logical order.
  - If an opening paired bracket is found and there is room in the stack, push its Bidi\_Paired\_Bracket property
    value and its text position onto the stack.
  - If an opening paired bracket is found and there is no room in the stack, stop processing BD16 for the remainder of the isolating run sequence and return an empty list.
  - If a closing paired bracket is found, do the following:
    1. Declare a variable that holds a reference to the current stack element and initialize
       it with the top element of the stack. If the stack is empty, skip to step 5.
    2. Compare the closing paired bracket being inspected
       to the bracket in the current stack element, where U+3009 and U+232A are treated as equivalent.

       Note that although *bracket pairs* are defined under canonical equivalence, canonical equivalents only exist between U+3008/U+3009, and U+2329/U+232A, and the Unicode Consortium will not add more such pairs.
    3. If the values match, meaning the two characters form a bracket pair, then
       - Append the text position in the current stack element together with the
         text position of the closing paired bracket to the list of resulting bracket pairs.
       - Pop the stack through the current stack element inclusively.
    4. Else, if the current stack element is not at the bottom of the stack, advance it to
       the next element deeper in the stack and go back to step 2.
    5. Else, continue with inspecting the next character without popping the stack.
- Sort the list of resulting bracket pairs in ascending order based on the text position of the opening paired bracket.

Note that bracket pairs can only occur in an isolating run sequence
because they are processed in rule [N0](011-3-3-4-resolving-weak-types.md#N0) after explicit level resolution. See
*Section 3.3.2, [Explicit Levels and Directions](008-3-3-1-the-paragraph-level.md#Explicit_Levels_and_Directions)*.

#### Examples of bracket pairs



```

	Text			Pairings
	1 2 3 4 5 6 7 8
	a ) b ( c		None
	a ( b ] c		None
	a ( b ) c		2-4
	a ( b [ c ) d ]		2-6
	a ( b ] c ) d		2-6
	a ( b ) c ) d		2-4
	a ( b ( c ) d		4-6
	a ( b ( c ) d )		2-8, 4-6
	a ( b { c } d )		2-8, 4-6
  
```




<a id="Additional_Abbreviations"></a>

### 3.1.4  [Additional Abbreviations](006-3-1-3-paired-brackets.md#Additional_Abbreviations)

*Table 3* lists additional abbreviations used in the examples and internal character types used
in the algorithm.

<a id="Table_Abbrevs_for_Examples_Internal_Types"></a>

Table 3. [Abbreviations for Examples and Internal Types](006-3-1-3-paired-brackets.md#Table_Abbrevs_for_Examples_Internal_Types)

<a id="NI"></a>

<a id="N"></a>

<a id="e"></a>

<a id="o"></a>

<a id="sos"></a>

<a id="sor"></a>

<a id="eos"></a>

<a id="eor"></a>

| **Symbol** | **Description** |
| --- | --- |
| [NI](006-3-1-3-paired-brackets.md#NI) | Neutral or Isolate formatting character ([B](007-3-3-resolving-embedding-levels.md#B), [S](007-3-3-resolving-embedding-levels.md#S), [WS](007-3-3-resolving-embedding-levels.md#WS), [ON](007-3-3-resolving-embedding-levels.md#ON), [FSI](007-3-3-resolving-embedding-levels.md#FSI), [LRI](007-3-3-resolving-embedding-levels.md#LRI), [RLI](007-3-3-resolving-embedding-levels.md#RLI), [PDI](007-3-3-resolving-embedding-levels.md#PDI)). |
| [e](006-3-1-3-paired-brackets.md#e) | The text ordering type ([L](007-3-3-resolving-embedding-levels.md#L) or [R](007-3-3-resolving-embedding-levels.md#R)) that matches the **embedding** level direction (even or odd). |
| [o](006-3-1-3-paired-brackets.md#o) | The text ordering type ([L](007-3-3-resolving-embedding-levels.md#L) or [R](007-3-3-resolving-embedding-levels.md#R)) that matches the direction **opposite** the embedding level direction (even or odd).  Note that o is the opposite of e. |
| [sos](006-3-1-3-paired-brackets.md#sos) | The text ordering type ([L](007-3-3-resolving-embedding-levels.md#L) or [R](007-3-3-resolving-embedding-levels.md#R)) assigned to the virtual position before an isolating run sequence. |
| [eos](006-3-1-3-paired-brackets.md#eos) | The text ordering type ([L](007-3-3-resolving-embedding-levels.md#L) or [R](007-3-3-resolving-embedding-levels.md#R)) assigned to the virtual position after an isolating run sequence. |

<a id="Bidirectional_Character_Types"></a>

### 3.2  [Bidirectional Character Types](006-3-1-3-paired-brackets.md#Bidirectional_Character_Types)

The normative bidirectional character types for each character are specified in the
[Unicode Character Database](https://www.unicode.org/Public/UNIDATA/) [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]
and are summarized in *[Table 4](006-3-1-3-paired-brackets.md#Table_Bidirectional_Character_Types)*. This is a summary only: there are exceptions to the general
scope. For example, certain characters such as U+0CBF KANNADA VOWEL SIGN I are given Type L
(instead of NSM) to preserve canonical equivalence.

- The term European digits is used to refer to decimal forms common in Europe and elsewhere,
  and Arabic-Indic digits to refer to the native Arabic forms. (See *Section 9.2, Arabic* of
  [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)], for more details on naming digits.)
- Unassigned characters are given strong types in the algorithm. This is an explicit exception
  to the general Unicode conformance requirements with respect to unassigned characters. As
  characters become assigned in the future, these bidirectional types may change. For
  assignments to character types, see DerivedBidiClass.txt
  [[DerivedBIDI](https://www.unicode.org/reports/tr41/tr41-36.html#Props)]
  in the [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)].
- Private-use characters can be assigned different values by a conformant implementation.
- For the purpose of the Bidirectional Algorithm, inline objects (such as graphics) are
  treated as if they are an U+FFFC OBJECT REPLACEMENT CHARACTER.

<a id="canonical_equivalence"></a>

- As of Unicode 4.0, the Bidirectional Character Types of a few Indic characters were altered
  so that the Bidirectional Algorithm preserves [canonical
  equivalence](006-3-1-3-paired-brackets.md#canonical_equivalence). That is, two canonically equivalent strings will result in equivalent ordering
  after applying the algorithm. This invariant will be maintained in the future.

  **Note:**
  The Bidirectional Algorithm does *not* preserve compatibility equivalence.

<a id="Table_Bidirectional_Character_Types"></a>

Table 4. [Bidirectional Character Types](006-3-1-3-paired-brackets.md#Table_Bidirectional_Character_Types)

