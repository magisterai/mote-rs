# 1.1 Notation

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-unicode-standard-annex-29.md) · [Next](003-3-grapheme-cluster-boundaries.md)

<a id="Notation"></a>

### 1.1  [Notation](002-1-1-notation.md#Notation)

A boundary specification summarizes boundary property values
used in that specification, then lists the rules for boundary
determinations in terms of those property values. The summary is
provided as a list, where each element of the list is one of the
following:

- A literal character
- A range of literal characters
- All characters satisfying a given condition, using
  properties defined in the Unicode Character Database [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]:
  - Non-Boolean property values are given as *<property>
    = <property value>*, such as General\_Category =
    Titlecase\_Letter.
  - Boolean properties are given as *<property> =
    Yes*, such as Uppercase = Yes.
  - Other conditions are specified textually in terms of UCD
    properties.
- Boolean combinations of the above
- Two special identifiers, *sot* and *eot*, standing
  for *start of text* and *end of text*, respectively

For example, the following is such a list:

> General\_Category = Line\_Separator, *or*  
> General\_Category = Paragraph\_Separator, *or*  
> General\_Category = Control, *or*  
>  General\_Category =
> Format  
>  *and not* U+000D CARRIAGE RETURN (CR)  
> *and not* U+000A LINE FEED (LF)  
>  *and not* U+200C
> ZERO WIDTH NON-JOINER (ZWNJ)  
>  *and not* U+200D ZERO
> WIDTH JOINER (ZWJ)

In the table assigning the boundary property values, all of the
values are intended to be disjoint except for the special value **Any**.
In case of conflict, rows higher in the table have precedence in
terms of assigning property values to characters. Data files
containing explicit assignments of the property values are found in [[Props](https://www.unicode.org/reports/tr41/tr41-36.html#Props0)].

Boundary determination is specified in terms of an ordered list
of rules, indicating the status of a boundary position. The rules are
numbered for reference and are applied in sequence to determine
whether there is a boundary at any given offset. That is, there is an
implicit “otherwise” at the front of each rule following the first.
The rules are processed from top to bottom. As soon as a rule matches
and produces a boundary status (boundary or no boundary) for that
offset, the process is terminated.

Each rule consists of a left side, a boundary symbol (see [*Table 1*](002-1-1-notation.md#Table_Boundary_Symbols)), and a right
side. Either of the sides can be empty. The left and right sides use
the boundary property values in regular expressions. The regular
expression syntax used is a simplified version of the format supplied
in *Unicode Technical Standard #18, Unicode Regular
Expressions* [[UTS18](https://www.unicode.org/reports/tr41/tr41-36.html#UTS18)].

<a id="Table_Boundary_Symbols"></a>

Table 1. [Boundary Symbols](002-1-1-notation.md#Table_Boundary_Symbols)

|  |  |
| --- | --- |
| ÷ | Boundary (allow break here) |
| × | No boundary (do not allow break here) |
| → | Treat whatever on the left side as if it were what is on the right side |

An *open-box* symbol (“␣”) is used to indicate a space in
examples.

<a id="Rule_Constraints"></a>

### 1.2  [Rule Constraints](002-1-1-notation.md#Rule_Constraints)

These rules are constrained in three ways, to make
implementations significantly simpler and more efficient. These
constraints have not been found to be limitations for natural
language use. In particular, the rules are formulated so that they
can be efficiently implemented, such as with a deterministic
finite-state machine based on a small number of property values.

1. ***Single boundaries**.* Each rule has
   exactly one boundary position. This restriction is more a limitation
   on the specification methods, because a rule with multiple
   boundaries could be expressed instead as multiple rules. For
   example:
   - “a b ÷ c d ÷ e f” could be broken into two rules “a b ÷ c
     d e f” and “a b c d ÷ e f”
   - “a b × c d × e f” could be broken into two rules “a b × c
     d e f” and “a b c d × e f”
2. ***Limited negation**.* Negation of
   expressions is limited to instances that resolve to a match against
   single characters, such as “¬(OLetter | Upper | Lower | Sep)”.
3. ***Ignore degenerates**.* No special
   provisions are made to get marginally better behavior for degenerate
   cases that never occur in practice, such as an *A* followed by
   an Indic combining mark.
4. ***Script boundaries**.*
   Script boundaries are treated as degenerate cases in these rules, so
   the string “aquaφοβία” is treated as a single word, and the sequence
   ‘a’ + ‘ ि’ as a single grapheme cluster. However, implementations
   are free to customize boundary testing to break at script
   boundaries, which may be especially useful for grapheme clusters.
   When this is done, the Common/Inherited values need to be handled
   properly, and the Script\_Extensions property should be used instead
   of the Script property alone.

<a id="Conformance"></a>

## 2  [Conformance](002-1-1-notation.md#Conformance)

There are many different ways to divide text elements
corresponding to user-perceived characters, words, and sentences, and
the Unicode Standard does not restrict the ways in which
implementations can produce these divisions. However, it does provide conformance clauses to enable implementations to clearly describe their behavior in relation to the default behavior.

<a id="C1"></a>

**[UAX29-C1](002-1-1-notation.md#C1)**. **Extended Grapheme Cluster Boundaries:** *An implementation shall choose either UAX29-C1-1 or UAX29-C1-2 to determine whether an offset within a sequence of characters is an extended grapheme cluster boundary.*

<a id="C1-1"></a>

**[UAX29-C1-1](002-1-1-notation.md#C1-1)**. *Use the property values defined in the Unicode Character Database [[UCD](https://unicode.org/reports/tr41/tr41-36.html#UCD)] and the **extended** rules in Section 3.1 [Grapheme Cluster Boundary Rules](004-3-1-default-grapheme-cluster-boundary-specification.md#Grapheme_Cluster_Boundary_Rules) to determine the boundaries.*

> The default grapheme clusters are also known as **extended grapheme clusters**.

<a id="C1-2"></a>

**[UAX29-C1-2](002-1-1-notation.md#C1-2)**. *Declare the use of a profile of UAX29-C1-1, and define that profile with a precise specification of any changes in property values or rules and/or provide a description of programmatic overrides to the behavior of UAX29-C1-1.*

> Legacy grapheme clusters are such a profile.

<a id="C2"></a>

**[UAX29-C2](002-1-1-notation.md#C2)**. **Word Boundaries:** *An implementation shall choose either UAX29-C2-1 or UAX29-C2-2 to determine whether an offset within a sequence of characters is a word boundary.*

<a id="C2-1"></a>

**[UAX29-C2-1](002-1-1-notation.md#C2-1)**. *Use the property values defined in the Unicode Character Database [[UCD](https://unicode.org/reports/tr41/tr41-36.html#UCD)] and the rules in Section 4.1 [Default Word Boundary Specification](https://unicode.org/reports/tr29/#Default_Word_Boundaries) to determine the boundaries.*

<a id="C2-2"></a>

**[UAX29-C2-2](002-1-1-notation.md#C2-2)**. *Declare the use of a profile of UAX29-C2-1, and define that profile with a precise specification of any changes in property values or rules and/or provide a description of programmatic overrides to the behavior of UAX29-C2-1.*

<a id="C3"></a>

**[UAX29-C3](002-1-1-notation.md#C3)**. **Sentence Boundaries:** *An implementation shall choose either UAX29-C3-1 or UAX29-C3-2 to determine whether an offset within a sequence of characters is a sentence boundary.*

<a id="C3-1"></a>

**[UAX29-C3-1](002-1-1-notation.md#C3-1)**. *Use the property values defined in the Unicode Character Database [[UCD](https://unicode.org/reports/tr41/tr41-36.html#UCD)] and the rules in Section 5.1 [Default Sentence Boundary Specification](https://unicode.org/reports/tr29/#Default_Word_Boundaries) to determine the boundaries.*

<a id="C3-2"></a>

**[UAX29-C3-2](002-1-1-notation.md#C3-2)**. *Declare the use of a profile of UAX29-C3-1, and define that profile with a precise specification of any changes in property values or rules and/or provide a description of programmatic overrides to the behavior of UAX29-C3-1.*

This specification defines *default* mechanisms; more
sophisticated implementations can *and should* tailor them for
particular locales or environments and, for the purpose of claiming conformance, document the tailoring in the form of a profile. For example, reliable detection
of word boundaries in languages such as Thai, Lao, Chinese, or
Japanese requires the use of dictionary lookup or other mechanisms, analogous to English
hyphenation. An implementation therefore may need to provide means for a programmatic override of the default mechanisms described in this annex.
Note that a profile can both add and remove boundary positions, compared to the results specified by [UAX29-C1-1](002-1-1-notation.md#C1-1), [UAX29-C2-1](002-1-1-notation.md#C2-1), or [UAX29-C3-1](002-1-1-notation.md#C3-1).

> **Notes:**
>
> - Locale-sensitive boundary specifications, including
>   boundary suppressions, can be expressed in LDML [[UTS35](https://www.unicode.org/reports/tr41/tr41-36.html#UTS35)]. Some profiles are
>   available in the Common Locale Data Repository [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)].
> - Some changes to rules and data are needed for best
>   segmentation behavior of additional emoji zwj sequences [[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)]. Implementations are
>   strongly encouraged to use the extended text segmentation rules in
>   the latest version of CLDR.

To maintain canonical equivalence, all of the following
specifications are defined on text normalized in form NFD, as defined
in Unicode Standard Annex #15, “Unicode Normalization
Forms” [[UAX15](https://www.unicode.org/reports/tr41/tr41-36.html#UAX15)].
Boundaries never occur within a combining character sequence or conjoining sequence,
so the boundaries within non-NFD text can be derived from corresponding boundaries in the NFD form of that text.
For convenience, the default rules have been written so that they can be applied directly to non-NFD text and yield equivalent results.
(This may not be the case with tailored default rules.)
For more information, see Section 6, [*Implementation Notes*](011-6-implementation-notes.md#Implementation_Notes).

