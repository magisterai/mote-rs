# 3.3.4 Resolving Weak Types

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-3-3-3-preparations-for-implicit-processing.md) · [Next](012-3-3-6-resolving-implicit-levels.md)

<a id="Resolving_Weak_Types"></a>

### 3.3.4  [Resolving Weak Types](011-3-3-4-resolving-weak-types.md#Resolving_Weak_Types)

Weak types are now resolved one isolating run sequence at a time.
At isolating run sequence boundaries where the type of
the character on the other side of the boundary is required, the type assigned to
*sos* or *eos* is used.

First, each nonspacing mark is resolved based on the character it follows.

<a id="W1"></a>

***[W1](011-3-3-4-resolving-weak-types.md#W1).** Examine each nonspacing mark (NSM) in the isolating run sequence, and change the
type of the NSM to Other Neutral if the previous character is an isolate initiator or PDI, and to
the type of the previous character otherwise.
If the NSM is at the start of the
isolating run sequence, it will get the type of **sos**.*
(Note that in an isolating run sequence, an isolate initiator followed by an NSM or any type other than PDI must be an overflow isolate initiator.)

Assume in this example that *sos* is R:



```

AL  NSM NSM → AL  AL  AL

sos NSM     → sos R

LRI NSM     → LRI ON

PDI NSM     → PDI ON
```




The text is next parsed for numbers. This pass will change the directional
types European Number Separator, European Number Terminator, and Common
Number Separator to be European Number text, Arabic Number text, or Other
Neutral text. The text to be scanned may have already had its type altered
by directional overrides. If so, then it will not parse as numeric.

<a id="W2"></a>

***[W2](011-3-3-4-resolving-weak-types.md#W2).** Search backward from each instance of a European number until the
first strong type (R, L, AL, or **sos**) is found. If an AL is found, change the type of
the European number to Arabic number.*



```

AL EN     → AL AN

AL NI EN  → AL NI AN

sos NI EN → sos NI EN

L NI EN   → L NI EN

R NI EN   → R NI EN
    
```




<a id="W3"></a>

***[W3](011-3-3-4-resolving-weak-types.md#W3).** Change all ALs to R.*

<a id="W4"></a>

***[W4](011-3-3-4-resolving-weak-types.md#W4).** A single European separator between two European numbers changes to a
European number. A single common separator between two numbers of the same type changes to that
type.*



```
EN ES EN → EN EN EN

EN CS EN → EN EN EN

AN CS AN → AN AN AN
```




<a id="W5"></a>

***[W5](011-3-3-4-resolving-weak-types.md#W5).** A sequence of European terminators adjacent to European numbers changes
to all European numbers.*



```
ET ET EN → EN EN EN

EN ET ET → EN EN EN

AN ET EN → AN EN EN
```




<a id="W6"></a>

***[W6](011-3-3-4-resolving-weak-types.md#W6).** All remaining separators and terminators (after the application of [W4](011-3-3-4-resolving-weak-types.md#W4) and [W5](011-3-3-4-resolving-weak-types.md#W5)) change to Other Neutral.*



```
AN ET    → AN ON

L  ES EN → L  ON EN

EN CS AN → EN ON AN

ET AN    → ON AN
```




<a id="W7"></a>

***[W7](011-3-3-4-resolving-weak-types.md#W7).** Search backward from each instance of a European number until the
first strong type (R, L, or **sos**) is found. If an L is found, then change the type of
the European number to L.*



```
L  NI EN → L  NI  L

R  NI EN → R  NI  EN
```




<a id="Resolving_Neutral_Types"></a>

### 3.3.5  [Resolving Neutral and Isolate Formatting Types](011-3-3-4-resolving-weak-types.md#Resolving_Neutral_Types)

In the next phase, neutral and isolate formatting (i.e. [NI](006-3-1-3-paired-brackets.md#NI)) characters are resolved
one isolating run sequence at a time.
Its results are that all
[NIs](006-3-1-3-paired-brackets.md#NI) become either **R** or **L**.
Generally, [NIs](006-3-1-3-paired-brackets.md#NI) take on the direction of the
surrounding text. In case of a conflict, they take on the embedding direction.
At isolating run sequence boundaries where the type
of the character on the other side of the boundary is required, the type assigned
to *sos* or *eos*
is used.

Bracket pairs within an isolating run sequence are processed as units so that both the opening
and the closing paired bracket in a pair resolve to the same direction.

<a id="N0"></a>

***[N0](011-3-3-4-resolving-weak-types.md#N0).** Process bracket pairs in an isolating run sequence sequentially in the logical
order of the text positions of the opening paired brackets using the logic given below.
Within this scope, bidirectional types EN and AN are treated as R.*

- *Identify the bracket pairs in the current isolating run sequence according to [BD16](006-3-1-3-paired-brackets.md#BD16).*

  Note that [BD14](006-3-1-3-paired-brackets.md#BD14) and [BD15](006-3-1-3-paired-brackets.md#BD15) identify bracket characters based on the current bidirectional character type of each paired bracket
  and not the original type, as this could have changed under [X6](009-non-formatting-characters.md#X6).
- *For each bracket-pair element in the list of pairs of text positions*
  1. *Inspect the bidirectional types of the characters enclosed
     within the bracket pair.*
  2. *If any strong type (either L or R) matching
     the embedding direction is found, set the type for both brackets in the pair to match
     the embedding direction.*
     > Note that EN and AN should be treated as a strong R type when searching within the brackets.
     >
     > Note that the isolating run sequence may not be contiguous.
     > Implementations should take care to ignore characters not contained in the isolating run sequence when processing neutral or weak characters.
     >


```

o [ e ] o → o e e e o

o [ o e ] → o e o e e

o [ NI e ] → o e NI e e
    
```



  3. *Otherwise, if there is a strong type it must be opposite the embedding direction. Therefore, test for an established context with a preceding strong type by checking backwards before the opening paired bracket until the first strong type (L or R) is found, using the value of sos if there is none.*

     Note that EN and AN should be treated as a strong R type when searching for established context.

     1. *If the preceding strong type is also opposite the embedding direction, context is established, so set the type for both brackets in the pair to that direction.*


```

o [ o ] e → o o o o e

o [ o NI ] o → o o o NI o o
		
```



     2. *Otherwise set the type for both brackets in the pair to the embedding direction.*


```

e [ o ] o → e e o e o

e [ o ] e → e e o e e
		
```




        Note that taken together the two steps in item 2 are guaranteed to set the type for both brackets to the preceding strong type, as there are only two possible values (L and R).
  4. *Otherwise, there are no strong types within the bracket pair. Therefore, do not set the type for that bracket pair.*


```

e ( NI ) o → e ( NI ) o
    
```




     Note that if the enclosed text contains no strong types the bracket pairs will both resolve
     to the same level when resolved individually using rules [N1](011-3-3-4-resolving-weak-types.md#N1) and [N2](011-3-3-4-resolving-weak-types.md#N2).
- *Any number of characters that had original bidirectional character type NSM prior
  to the application of [W1](011-3-3-4-resolving-weak-types.md#W1) that immediately follow a paired bracket
  which changed to L or R under [N0](011-3-3-4-resolving-weak-types.md#N0) should change
  to match the type of their preceding bracket.*

**Example 1**. Bracket pairs are resolved sequentially in logical order of the opening paired brackets.

(RTL paragraph direction)

<table class="simple">
<tr>
<td>Storage</td>
<td><code>AB</code></td>
<td><code>(</code></td>
<td><code>CD</code></td>
<td><code>[</code></td>
<td><code>&amp;</code></td>
<td><code>ef</code></td>
<td><code>]</code></td>
<td><code>!</code></td>
<td><code>)</code></td>
<td><code>gh</code></td>
</tr>
<tr>
<td>Bidi_Class</td>
<td><code>R</code></td>
<td><code>ON</code></td>
<td><code>R</code></td>
<td><code>ON</code></td>
<td><code>ON</code></td>
<td><code>L</code></td>
<td><code>ON</code></td>
<td><code>ON</code></td>
<td><code>ON</code></td>
<td><code>L</code></td>
</tr>
<tr>
<td>N0 applied (first pair)</td>
<td><code></code></td>
<td><code>N0b: ON→R</code></td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
<td><code>N0b: ON→R</code></td>
<td><code></code></td>
</tr>
<tr>
<td>N0 applied (second pair)</td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
<td><code>N0c2: ON→R</code></td>
<td><code></code></td>
<td><code></code></td>
<td><code>N0c2: ON→R</code></td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
</tr>
<tr>
<td>Display</td>
<td colspan="10"><code>gh(![ef&amp;]DC)BA</code></td>
</tr>
</table>

**Example 2**. Bracket pairs enclosing mixed strong types take the paragraph direction.

(RTL paragraph direction)

<table class="simple">
<tr>
<td>Storage</td>
<td><code>smith</code></td>
<td><code> </code></td>
<td><code>(</code></td>
<td><code>fabrikam</code></td>
<td><code> </code></td>
<td><code>ARABIC</code></td>
<td><code>)</code></td>
<td><code> </code></td>
<td><code>HEBREW</code></td>
</tr>
<tr>
<td>Bidi_Class</td>
<td><code>L</code></td>
<td><code>WS</code></td>
<td><code>ON</code></td>
<td><code>L</code></td>
<td><code>WS</code></td>
<td><code>R</code></td>
<td><code>ON</code></td>
<td><code>WS</code></td>
<td><code>R</code></td>
</tr>
<tr>
<td>N0 applied</td>
<td><code></code></td>
<td><code></code></td>
<td><code>N0b: ON→R</code></td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
<td><code>N0b: ON→R</code></td>
<td><code></code></td>
<td><code></code></td>
</tr>
<tr>
<td>Display</td>
<td colspan="9"><code>WERBEH (CIBARA fabrikam) smith</code></td>
</tr>
</table>

Note that in the above example, the resolution of the bracket pairs is
stable if the order of smith and HEBREW, or fabrikam and ARABIC, is reversed.

**Example 3**. Bracket pairs enclosing strong types opposite the
embedding direction with additional strong-type context take the direction
opposite the embedding direction.

(RTL paragraph direction)

<table class="simple">
<tr>
<td>Storage</td>
<td><code>ARABIC</code></td>
<td><code> </code></td>
<td><code>book</code></td>
<td><code>(</code></td>
<td><code>s</code></td>
<td><code>)</code></td>
</tr>
<tr>
<td>Bidi_Class</td>
<td><code>R</code></td>
<td><code>WS</code></td>
<td><code>L</code></td>
<td><code>ON</code></td>
<td><code>L</code></td>
<td><code>ON</code></td>
</tr>
<tr>
<td>N0 applied</td>
<td><code></code></td>
<td><code></code></td>
<td><code></code></td>
<td><code>N0c1: ON→L</code></td>
<td><code></code></td>
<td><code>N0c1: ON→L</code></td>
</tr>
<tr>
<td>Display</td>
<td colspan="6"><code>book(s) CIBARA</code></td>
</tr>
</table>

<a id="N1"></a>

***[N1](011-3-3-4-resolving-weak-types.md#N1).** A sequence of [NIs](006-3-1-3-paired-brackets.md#NI) takes the direction of the surrounding strong
text if the text on both sides has the same direction.
European and Arabic numbers act as if they were R in terms of their influence on
[NIs](006-3-1-3-paired-brackets.md#NI).
The start-of-sequence (**sos**)
and end-of-sequence (**eos**) types
are used at isolating run sequence boundaries.*



```

 L  NI   L  →   L  L   L

 R  NI   R  →   R  R   R

 R  NI  AN  →   R  R  AN

 R  NI  EN  →   R  R  EN

AN  NI   R  →  AN  R   R

AN  NI  AN  →  AN  R  AN

AN  NI  EN  →  AN  R  EN

EN  NI   R  →  EN  R   R

EN  NI  AN  →  EN  R  AN

EN  NI  EN  →  EN  R  EN
```




<a id="N2"></a>

***[N2](011-3-3-4-resolving-weak-types.md#N2).** Any remaining [NIs](006-3-1-3-paired-brackets.md#NI) take the embedding direction.*



```
NI → e
```




The embedding direction for the given [NI](006-3-1-3-paired-brackets.md#NI)
character is derived from its embedding level: L if the character is set to
an even level, and R if the level is odd. (See [BD3](004-3-basic-display-algorithm.md#BD3).)

Assume in the following example that *eos* is L and *sos* is R. Then an application of
[N1](011-3-3-4-resolving-weak-types.md#N1) and [N2](011-3-3-4-resolving-weak-types.md#N2) yields the following:



```
L   NI eos → L   L eos

R   NI eos → R   e eos

sos NI L   → sos e L

sos NI R   → sos R R
```




*Examples.* A list of numbers separated by neutrals and embedded in a directional run will
come out in the run’s order.



```
Storage:	he said "THE VALUES ARE 123, 456, 789, OK".

Display:	he said "KO ,789 ,456 ,123 ERA SEULAV EHT".
```




In this case, both the comma and the space between the numbers take on the direction of the
surrounding text (uppercase = right-to-left), ignoring the numbers. The commas are not considered
part of the number because they are not surrounded on both sides
by digits (see
Section 3.3.4, *Resolving Weak Types*). However, if
there is a preceding left-to-right sequence, then European numbers will adopt that direction:



```
Storage:	IT IS A bmw 500, OK.

Display:	.KO ,bmw 500 A SI TI
```




