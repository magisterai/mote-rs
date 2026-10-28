# 3.1 Default Grapheme Cluster Boundary Specification

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-3-grapheme-cluster-boundaries.md) · [Next](005-4-word-boundaries.md)

However, aksaras may also include one or more additional
consonants, typically with a *virama* (halant) character between
each pair of consonants in the sequence. Some consonant cluster
aksaras are not incorporated into the default rules for extended
grapheme clusters, in part because not all such sequences are
considered to be single “characters” by users.
Another reason is that additional changes to the
rules are made when new information becomes available. Indic scripts vary
considerably in how they handle the rendering of such aksaras—in some
cases stacking them up into combined forms known as consonant
conjuncts, and in other cases stringing them out horizontally, with
visible renditions of the halant on each consonant in the sequence.
There is even greater variability in how the typical liquid
consonants (or “medials”), *ya, ra, la,* and *wa*, are
handled for display in combinations in aksaras. So tailorings for
aksaras may need to be script-, language-, font-, or context-specific
to be useful.

> **Note:** Font-based information may be required to determine the
> appropriate unit to use for UI purposes, such as identification of
> boundaries for first-letter paragraph styling. For example, such a
> unit could be a ligature formed of two grapheme clusters, such as
> لا (Arabic lam + alef).

The Unicode specification of grapheme clusters >allows for more sophisticated profiles where appropriate. Such definitions may more
precisely match the user expectations within individual languages for
given processes. For example, “ch” may be considered a grapheme
cluster in Slovak, for processes such as collation. The default
definitions are, however, designed to provide a much more accurate
match to overall user expectations for what the user perceives of as *characters* than is provided by individual Unicode code points.

> **Note:** The term cluster is
> used to emphasize that the term grapheme is used differently in
> linguistics.

***Display of Grapheme Clusters.*** Grapheme clusters are not
the same as ligatures. For example, the grapheme cluster “ch” in
Slovak is not normally a ligature and, conversely, the ligature “fi”
is not a grapheme cluster. Default grapheme clusters do not
necessarily reflect text display. For example, the sequence <f,
i> may be displayed as a single glyph on the screen, but would
still be two grapheme clusters.

For information on the matching of grapheme clusters with regular
expressions, see Unicode Technical Standard #18, “Unicode Regular
Expressions” [[UTS18](https://www.unicode.org/reports/tr41/tr41-36.html#UTS18)].

***Degenerate Cases.*** The default specifications are
designed to be simple to implement, and provide an algorithmic
determination of grapheme clusters. However, they do  *not* have
to cover edge cases that will not occur in practice. For the purpose
of segmentation, they may also include degenerate cases that are not
thought of as grapheme clusters, such as an isolated control
character or combining mark. In this, they differ from the combining
character sequences and extended combining character sequences
defined in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]. In
addition, Unassigned (Cn) code points and Private\_Use (Co) characters
are given property values that anticipate potential usage.

**Combining Character Sequences and
Grapheme Clusters.** For comparison, *[Table
1b](004-3-1-default-grapheme-cluster-boundary-specification.md#Table_Combining_Char_Sequences_and_Grapheme_Clusters)* shows the relationship between combining character sequences and
grapheme clusters, using regex notation. Note that given alternates
(X|Y), the first match is taken. The
simple identifiers starting with lowercase are variables that are
defined in [*Table 1c*](004-3-1-default-grapheme-cluster-boundary-specification.md#Regex_Definitions); those
starting with uppercase letters are **Grapheme\_Cluster\_Break
Property Values** defined in [*Table 2*](004-3-1-default-grapheme-cluster-boundary-specification.md#Grapheme_Cluster_Break_Property_Values).

<a id="Table_Combining_Char_Sequences_and_Grapheme_Clusters"></a>

Table 1b. [Combining
Character Sequences and Grapheme Clusters](004-3-1-default-grapheme-cluster-boundary-specification.md#Table_Combining_Char_Sequences_and_Grapheme_Clusters)

| Term | Regex | Notes |
| --- | --- | --- |
| combining character sequence | `ccs-base? ccs-extend+` | A single base character is not a combining character sequence. However, a single combining mark *is* a (degenerate) combining character sequence. |
| extended combining character sequence | `extended_base? ccs-extend+` | extended\_base includes Hangul Syllables |
| legacy grapheme cluster | `crlf  | Control   | legacy-core legacy-postcore*` | A single base character is a grapheme cluster. Degenerate cases include any isolated non-base characters, and non-base characters like controls. |
| extended grapheme cluster | `crlf   | Control   | precore* core postcore*` | Extended grapheme clusters add prepending and spacing marks. |

[*Table
1b*](004-3-1-default-grapheme-cluster-boundary-specification.md#Table_Combining_Char_Sequences_and_Grapheme_Clusters) uses several symbols defined in [*Table
1c*](004-3-1-default-grapheme-cluster-boundary-specification.md#Regex_Definitions). Square brackets and \p{...} are
used to indicate sets of characters, using the normal UnicodeSet
notion.

<a id="Regex_Definitions"></a>

Table 1c. [Regex
Definitions](004-3-1-default-grapheme-cluster-boundary-specification.md#Regex_Definitions)

|  |  |
| --- | --- |
| `ccs-base :=` | `[\p{L}\p{N}\p{P}\p{S}\p{Zs}]` |
| `ccs-extend :=` | `[\p{M}\p{Join_Control}]` |
| `extended_base :=` | `ccs-base   | hangul-syllable` |
| `crlf :=` | `CR LF | CR | LF` |
| `legacy-core :=` | `hangul-syllable   | RI-Sequence  | xpicto-sequence   | [^Control CR LF]` |
| `legacy-postcore :=` | `[Extend ZWJ]` |
| `core :=` | `hangul-syllable  | RI-Sequence  | xpicto-sequence   | conjunctCluster  | [^Control CR LF]` |
| `postcore :=` | `[Extend ZWJ SpacingMark]` |
| `precore :=` | `Prepend` |
| `RI-Sequence :=` | `RI RI` |
| `hangul-syllable :=` | `L* (V+ | LV V* | LVT) T*   | L+   | T+` |
| `xpicto-sequence :=` | `\p{Extended_Pictographic} (Extend* ZWJ \p{Extended_Pictographic})*` |
| `conjunctCluster :=` | `\p{InCB=Consonant} ([\p{InCB=Extend} \p{InCB=Linker}]* \p{InCB=Linker} [\p{InCB=Extend} \p{InCB=Linker}]* \p{InCB=Consonant})+` |

<a id="Default_Grapheme_Cluster_Table"></a>

### 3.1  [Default Grapheme Cluster Boundary Specification](004-3-1-default-grapheme-cluster-boundary-specification.md#Default_Grapheme_Cluster_Table)

The following is a general specification for grapheme cluster boundaries—language-specific rules in [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)] should be used where available.

The Grapheme\_Cluster\_Break property value assignments are explicitly
listed in the corresponding data file in [[Props](https://www.unicode.org/reports/tr41/tr41-36.html#Props0)]. The values in that
file are the normative property values.

For illustration, property values are summarized in [*Table 2*](004-3-1-default-grapheme-cluster-boundary-specification.md#Grapheme_Cluster_Break_Property_Values)*,*
but the lists of characters are illustrative.

<a id="Grapheme_Cluster_Break_Property_Values"></a>

Table 2. [Grapheme\_Cluster\_Break
Property Values](004-3-1-default-grapheme-cluster-boundary-specification.md#Grapheme_Cluster_Break_Property_Values)

<a id="CR"></a>

<a id="LF"></a>

<a id="Control"></a>

<a id="Extend"></a>

<a id="ZWJ"></a>

<a id="GB_After_Joiner"></a>

<a id="Prepend"></a>

<a id="SpacingMark"></a>

<a id="L"></a>

<a id="V"></a>

<a id="T"></a>

<a id="LV"></a>

<a id="LVT"></a>

<a id="E_Base"></a>

<a id="E_Modifier"></a>

<a id="Glue_After_Zwj"></a>

<a id="EBG"></a>

<a id="AnyGC"></a>

| Value | Summary List of Characters |
| --- | --- |
| **[CR](004-3-1-default-grapheme-cluster-boundary-specification.md#CR)** | U+000D CARRIAGE RETURN (CR) |
| **[LF](004-3-1-default-grapheme-cluster-boundary-specification.md#LF)** | U+000A LINE FEED (LF) |
| **[Control](004-3-1-default-grapheme-cluster-boundary-specification.md#Control)** | General\_Category = Line\_Separator, *or*  General\_Category = Paragraph\_Separator, *or*  General\_Category = Control, *or*  General\_Category = Unassigned *and* Default\_Ignorable\_Code\_Point, *or*  General\_Category = Format  *and not* U+000D CARRIAGE RETURN  *and not* U+000A LINE FEED  *and not* U+200C ZERO WIDTH NON-JOINER (ZWNJ)  *and not* U+200D ZERO WIDTH JOINER (ZWJ)  *and not* Prepended\_Concatenation\_Mark = Yes |
| **[Extend](004-3-1-default-grapheme-cluster-boundary-specification.md#Extend)** | Grapheme\_Extend = Yes, *or*  *Emoji\_Modifier=Yes*  *This includes:*  General\_Category = Nonspacing\_Mark  General\_Category = Enclosing\_Mark  U+200C ZERO WIDTH NON-JOINER  *plus a few* General\_Category = Spacing\_Mark *needed for canonical equivalence.* |
| **[ZWJ](004-3-1-default-grapheme-cluster-boundary-specification.md#ZWJ)** | U+200D ZERO WIDTH JOINER |
| [**Regional\_Indicator**](004-3-1-default-grapheme-cluster-boundary-specification.md#GB_After_Joiner) (RI) | Regional\_Indicator = Yes    *This consists of the range:*  U+1F1E6 REGIONAL INDICATOR SYMBOL LETTER A  ..U+1F1FF REGIONAL INDICATOR SYMBOL LETTER Z |
| **[Prepend](004-3-1-default-grapheme-cluster-boundary-specification.md#Prepend)** | Indic\_Syllabic\_Category = Consonant\_Preceding\_Repha*, or*  Indic\_Syllabic\_Category = Consonant\_Prefixed*, or*  Prepended\_Concatenation\_Mark = Yes |
| **[SpacingMark](004-3-1-default-grapheme-cluster-boundary-specification.md#SpacingMark)** | Grapheme\_Cluster\_Break ≠ Extend, *and*  General\_Category = Spacing\_Mark*, or*  *any of the following (which have* General\_Category = Other\_Letter*):*  U+0E33 ( ำ ) THAI CHARACTER SARA AM  U+0EB3 ( ຳ ) LAO VOWEL SIGN AM    *Exceptions: The following (which have* General\_Category = Spacing\_Mark *and would otherwise be included) are specifically excluded:*  U+102B ( ါ ) MYANMAR VOWEL SIGN TALL AA  U+102C ( ာ ) MYANMAR VOWEL SIGN AA  U+1038 ( း ) MYANMAR SIGN VISARGA  U+1062 ( ၢ ) MYANMAR VOWEL SIGN SGAW KAREN EU  ..U+1064 ( ၤ ) MYANMAR TONE MARK SGAW KAREN KE PHO  U+1067 ( ၧ ) MYANMAR VOWEL SIGN WESTERN PWO KAREN EU  ..U+106D ( ၭ ) MYANMAR SIGN WESTERN PWO KAREN TONE-5  U+1083 ( ႃ ) MYANMAR VOWEL SIGN SHAN AA  U+1087 ( ႇ ) MYANMAR SIGN SHAN TONE-2  ..U+108C ( ႌ ) MYANMAR SIGN SHAN COUNCIL TONE-3  U+108F ( ႏ ) MYANMAR SIGN RUMAI PALAUNG TONE-5  U+109A ( ႚ ) MYANMAR SIGN KHAMTI TONE-1  ..U+109C ( ႜ ) MYANMAR VOWEL SIGN AITON A  U+1A61 ( ᩡ ) TAI THAM VOWEL SIGN A  U+1A63 ( ᩣ ) TAI THAM VOWEL SIGN AA  U+1A64 ( ᩤ ) TAI THAM VOWEL SIGN TALL AA  U+AA7B ( ꩻ ) MYANMAR SIGN PAO KAREN TONE  U+AA7D ( ꩽ ) MYANMAR SIGN TAI LAING TONE-5  U+11720 ( 𑜠 ) AHOM VOWEL SIGN A  U+11721 ( 𑜡 ) AHOM VOWEL SIGN AA |
| **[L](004-3-1-default-grapheme-cluster-boundary-specification.md#L)** | Hangul\_Syllable\_Type=L, *such as:*  U+1100 ( ᄀ ) HANGUL CHOSEONG KIYEOK  U+115F ( **ᅟ** ) HANGUL CHOSEONG FILLER  U+A960 ( ꥠ ) HANGUL CHOSEONG TIKEUT-MIEUM  U+A97C ( ꥼ ) HANGUL CHOSEONG SSANGYEORINHIEUH |
| **[V](004-3-1-default-grapheme-cluster-boundary-specification.md#V)** | Hangul\_Syllable\_Type=V, *such as:*  U+1160 ( **ᅠ** ) HANGUL JUNGSEONG FILLER  U+11A2 ( ᆢ ) HANGUL JUNGSEONG SSANGARAEA  U+D7B0 ( ힰ ) HANGUL JUNGSEONG O-YEO  U+D7C6 ( ퟆ ) HANGUL JUNGSEONG ARAEA-E*, and:*  U+16D63 (𖵣) KIRAT RAI VOWEL SIGN AA  U+16D67 (𖵧) KIRAT RAI VOWEL SIGN E  ..U+16D6A (𖵪) KIRAT RAI VOWEL SIGN AU |
| **[T](004-3-1-default-grapheme-cluster-boundary-specification.md#T)** | Hangul\_Syllable\_Type=T, *such as:*  U+11A8 ( ᆨ ) HANGUL JONGSEONG KIYEOK  U+11F9 ( ᇹ ) HANGUL JONGSEONG YEORINHIEUH  U+D7CB ( ퟋ ) HANGUL JONGSEONG NIEUN-RIEUL  U+D7FB ( ퟻ ) HANGUL JONGSEONG PHIEUPH-THIEUTH |
| **[LV](004-3-1-default-grapheme-cluster-boundary-specification.md#LV)** | Hangul\_Syllable\_Type=LV, *that is:*  U+AC00 ( 가 ) HANGUL SYLLABLE GA  U+AC1C ( 개 ) HANGUL SYLLABLE GAE  U+AC38 ( 갸 ) HANGUL SYLLABLE GYA  ... |
| **[LVT](004-3-1-default-grapheme-cluster-boundary-specification.md#LVT)** | Hangul\_Syllable\_Type=LVT, *that is:*  U+AC01 ( 각 ) HANGUL SYLLABLE GAG  U+AC02 ( 갂 ) HANGUL SYLLABLE GAGG  U+AC03 ( 갃 ) HANGUL SYLLABLE GAGS  U+AC04 ( 간 ) HANGUL SYLLABLE GAN  ... |
| **[E\_Base](004-3-1-default-grapheme-cluster-boundary-specification.md#E_Base)** | *This value is obsolete and unused.* |
| **[E\_Modifier](004-3-1-default-grapheme-cluster-boundary-specification.md#E_Modifier)** | *This value is obsolete and unused.* |
| **[Glue\_After\_Zwj](004-3-1-default-grapheme-cluster-boundary-specification.md#Glue_After_Zwj)** | *This value is obsolete and unused.* |
| **[E\_Base\_GAZ](004-3-1-default-grapheme-cluster-boundary-specification.md#EBG)** (EBG) | *This value is obsolete and unused.* |
| **[Any](004-3-1-default-grapheme-cluster-boundary-specification.md#AnyGC)** | *This is not a property value; it is used in the rules to represent any code point.* |

  

<a id="Grapheme_Cluster_Boundary_Rules"></a>

#### 3.1.1  [Grapheme Cluster Boundary Rules](004-3-1-default-grapheme-cluster-boundary-specification.md#Grapheme_Cluster_Boundary_Rules)

The same rules are used for the two variants of grapheme clusters,
except the rules [GB9a](005-4-word-boundaries.md#GB9a), [GB9b](005-4-word-boundaries.md#GB9b), and [GB9c](005-4-word-boundaries.md#GB9c). The following table shows the
differences, which are also marked on the rules themselves. The extended rules are recommended, except where the legacy
variant is required for a specific environment.

| Grapheme Cluster Variant | Includes | Excludes |
| --- | --- | --- |
| LG: legacy grapheme clusters |  | GB9a, GB9b, GB9c |
| EG: extended grapheme clusters | GB9a, GB9b, GB9c |  |

When citing the Unicode definition of grapheme clusters, it
must be clear which of the two alternatives are being specified:
extended versus legacy.

