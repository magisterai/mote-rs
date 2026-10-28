# 4.1 Default Word Boundary Specification

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-4-word-boundaries.md) · [Next](007-continuation-7.md)

<a id="Default_Word_Boundaries"></a>

### 4.1  [Default Word Boundary Specification](006-4-1-default-word-boundary-specification.md#Default_Word_Boundaries)

The following is a general specification for word boundaries—language-specific rules in [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)] should be used where available.

The Word\_Break property value assignments are explicitly listed in
the corresponding data file in [[Props](https://www.unicode.org/reports/tr41/tr41-36.html#Props0)].
The values in that file are the normative property values.

For illustration, property values are summarized in [*Table 3*](006-4-1-default-word-boundary-specification.md#Table_Word_Break_Property_Values), but
the lists of characters are illustrative.

<a id="Table_Word_Break_Property_Values"></a>

Table 3. [Word\_Break Property
Values](006-4-1-default-word-boundary-specification.md#Table_Word_Break_Property_Values)

<a id="CR0"></a>

<a id="LF0"></a>

<a id="Newline"></a>

<a id="Extend0"></a>

<a id="ZWJ_WB"></a>

<a id="WB_After_Joiner"></a>

<a id="Format"></a>

<a id="Katakana"></a>

<a id="Hebrew_Letter"></a>

<a id="ALetter"></a>

<a id="Single_Quote"></a>

<a id="Double_Quote"></a>

<a id="MidNumLet"></a>

<a id="MidLetter"></a>

<a id="MidNum"></a>

<a id="Numeric"></a>

<a id="ExtendNumLetWB"></a>

<a id="E_Base_WB"></a>

<a id="E_Modifier_WB"></a>

<a id="Glue_After_Zwj_WB"></a>

<a id="EBG_WB"></a>

<a id="WSegSpace"></a>

<a id="AnyWB"></a>

| Value | Summary List of Characters |
| --- | --- |
| **[CR](006-4-1-default-word-boundary-specification.md#CR0)** | U+000D CARRIAGE RETURN (CR) |
| **[LF](006-4-1-default-word-boundary-specification.md#LF0)** | U+000A LINE FEED (LF) |
| **[Newline](006-4-1-default-word-boundary-specification.md#Newline)** | U+000B LINE TABULATION  U+000C FORM FEED (FF)  U+0085 NEXT LINE (NEL)  U+2028 LINE SEPARATOR  U+2029 PARAGRAPH SEPARATOR |
| **[Extend](006-4-1-default-word-boundary-specification.md#Extend0)** | Grapheme\_Extend = Yes, *or*  General\_Category = Spacing\_Mark, *or* Emoji\_Modifier=Yes  *and not* U+200D ZERO WIDTH JOINER (ZWJ) |
| **[ZWJ](006-4-1-default-word-boundary-specification.md#ZWJ_WB)** | U+200D ZERO WIDTH JOINER |
| [**Regional\_Indicator**](006-4-1-default-word-boundary-specification.md#WB_After_Joiner) (RI) | Regional\_Indicator = Yes    *This consists of the range:*  U+1F1E6 REGIONAL INDICATOR SYMBOL LETTER A  ..U+1F1FF REGIONAL INDICATOR SYMBOL LETTER Z |
| **[Format](006-4-1-default-word-boundary-specification.md#Format)** | General\_Category = Format  *and not* U+200B ZERO WIDTH SPACE (ZWSP)  *and not* U+200C ZERO WIDTH NON-JOINER (ZWNJ)  *and not* U+200D ZERO WIDTH JOINER (ZWJ)  *and not* Grapheme\_Cluster\_Break = Prepend |
| **[Katakana](006-4-1-default-word-boundary-specification.md#Katakana)** | Script = KATAKANA, *or  any of the following:*  U+3031 ( 〱 ) VERTICAL KANA REPEAT MARK  U+3032 ( 〲 ) VERTICAL KANA REPEAT WITH VOICED SOUND MARK  U+3033 ( 〳 ) VERTICAL KANA REPEAT MARK UPPER HALF  U+3034 ( 〴 ) VERTICAL KANA REPEAT WITH VOICED SOUND MARK UPPER HALF  U+3035 ( 〵 ) VERTICAL KANA REPEAT MARK LOWER HALF  U+309B ( ゛ ) KATAKANA-HIRAGANA VOICED SOUND MARK  U+309C ( ゜ ) KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK  U+30A0 ( ゠ ) KATAKANA-HIRAGANA DOUBLE HYPHEN  U+30FC ( ー ) KATAKANA-HIRAGANA PROLONGED SOUND MARK  U+FF70 ( ｰ ) HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK |
| **[Hebrew\_Letter](006-4-1-default-word-boundary-specification.md#Hebrew_Letter)** | Script = Hebrew  *and* General\_Category = Other\_Letter |
| **[ALetter](006-4-1-default-word-boundary-specification.md#ALetter)** | Alphabetic = Yes, *or*  *any of the following characters:*  U+00B8 ( ¸ ) CEDILLA  U+02C2 ( ˂ ) MODIFIER LETTER LEFT ARROWHEAD  ..U+02C5 ( ˅ ) MODIFIER LETTER DOWN ARROWHEAD  U+02D2 ( ˒ ) MODIFIER LETTER CENTRED RIGHT HALF RING  ..U+02D7 ( ˗ ) MODIFIER LETTER MINUS SIGN  U+02DE ( ˞ ) MODIFIER LETTER RHOTIC HOOK  U+02DF ( ˟ ) MODIFIER LETTER CROSS ACCENT  U+02E5 ( ˥ ) MODIFIER LETTER EXTRA-HIGH TONE BAR  ..U+02EB ( ˫ ) MODIFIER LETTER YANG DEPARTING TONE MARK  U+02ED ( ˭ ) MODIFIER LETTER UNASPIRATED  U+02EF ( ˯ ) MODIFIER LETTER LOW DOWN ARROWHEAD  ..U+02FF ( ˿ ) MODIFIER LETTER LOW LEFT ARROW  U+055A ( ՚ ) ARMENIAN APOSTROPHE  U+055B ( ՛ ) ARMENIAN EMPHASIS MARK  U+055C ( ՜ ) ARMENIAN EXCLAMATION MARK  U+055E ( ՞ ) ARMENIAN QUESTION MARK  U+058A ( ֊ ) ARMENIAN HYPHEN  U+05F3 ( ׳ ) HEBREW PUNCTUATION GERESH  U+070F ( ܏ ) SYRIAC ABBREVIATION MARK  U+A708 ( ꜈ ) MODIFIER LETTER EXTRA-HIGH DOTTED TONE BAR  ..U+A716 ( ꜖ ) MODIFIER LETTER EXTRA-LOW LEFT-STEM TONE BAR  U+A720 (꜠ ) MODIFIER LETTER STRESS AND HIGH TONE  U+A721 (꜡ ) MODIFIER LETTER STRESS AND LOW TONE  U+A789 (꞉ ) MODIFIER LETTER COLON  U+A78A ( ꞊ ) MODIFIER LETTER SHORT EQUALS SIGN  U+AB5B ( ꭛ ) MODIFIER BREVE WITH INVERTED BREVE  *and* Ideographic = No  *and* Word\_Break ≠ Katakana  *and* Line\_Break ≠ Complex\_Context (SA)  *and* Script ≠ Hiragana  *and* Word\_Break ≠ Extend  *and* Word\_Break ≠ Hebrew\_Letter |
| [**Single\_Quote**](006-4-1-default-word-boundary-specification.md#Single_Quote) | U+0027 ( ' ) APOSTROPHE |
| [**Double\_Quote**](006-4-1-default-word-boundary-specification.md#Double_Quote) | U+0022 ( " ) QUOTATION MARK |
| **[MidNumLet](006-4-1-default-word-boundary-specification.md#MidNumLet)** | U+002E ( . ) FULL STOP  U+2018 ( ‘ ) LEFT SINGLE QUOTATION MARK  U+2019 ( ’ ) RIGHT SINGLE QUOTATION MARK  U+2024 ( ․ ) ONE DOT LEADER  U+FE52 ( ﹒ ) SMALL FULL STOP  U+FF07 ( ＇ ) FULLWIDTH APOSTROPHE  U+FF0E ( ． ) FULLWIDTH FULL STOP |
| **[MidLetter](006-4-1-default-word-boundary-specification.md#MidLetter)** | U+003A ( : ) COLON *(used in Swedish)*  U+00B7 ( · ) MIDDLE DOT  U+0387 ( · ) GREEK ANO TELEIA  U+055F ( ՟ ) ARMENIAN ABBREVIATION MARK  U+05F4 ( ״ ) HEBREW PUNCTUATION GERSHAYIM  U+2027 ( ‧ ) HYPHENATION POINT  U+FE13 ( ︓ ) PRESENTATION FORM FOR VERTICAL COLON  U+FE55 ( ﹕ ) SMALL COLON  U+FF1A ( ： ) FULLWIDTH COLON |
| **[MidNum](006-4-1-default-word-boundary-specification.md#MidNum)** | Line\_Break = Infix\_Numeric, *or*  *any of the following:*  U+066C ( ٬ ) ARABIC THOUSANDS SEPARATOR  U+FE50 ( ﹐ ) SMALL COMMA  U+FE54 ( ﹔ ) SMALL SEMICOLON  U+FF0C ( ， ) FULLWIDTH COMMA  U+FF1B ( ； ) FULLWIDTH SEMICOLON  *and not* U+003A ( : ) COLON  *and not* U+FE13 ( ︓ ) PRESENTATION FORM FOR VERTICAL COLON  *and not* U+002E ( . ) FULL STOP |
| **[Numeric](006-4-1-default-word-boundary-specification.md#Numeric)** | Line\_Break = Numeric  *or* General\_Category = Decimal\_Number  *and not* U+066C ( ٬ ) ARABIC THOUSANDS SEPARATOR |
| **[ExtendNumLet](006-4-1-default-word-boundary-specification.md#ExtendNumLetWB)** | General\_Category = Connector\_Punctuation, *or*  U+202F NARROW NO-BREAK SPACE (NNBSP) |
| **[E\_Base](006-4-1-default-word-boundary-specification.md#E_Base_WB)** | *This value is obsolete and unused.* |
| **[E\_Modifier](006-4-1-default-word-boundary-specification.md#E_Modifier_WB)** | *This value is obsolete and unused.* |
| **[Glue\_After\_Zwj](006-4-1-default-word-boundary-specification.md#Glue_After_Zwj_WB)** | *This value is obsolete and unused.* |
| **[E\_Base\_GAZ](006-4-1-default-word-boundary-specification.md#EBG_WB)** (EBG) | *This value is obsolete and unused.* |
| **[WSegSpace](006-4-1-default-word-boundary-specification.md#WSegSpace)** | General\_Category = Zs  *and not* Linebreak = Glue |
| **[Any](006-4-1-default-word-boundary-specification.md#AnyWB)** | *This is not a property value; it is used in the rules to represent any code point.* |

<a id="Word_Boundary_Rules"></a>

#### 4.1.1  [Word Boundary Rules](006-4-1-default-word-boundary-specification.md#Word_Boundary_Rules)

The table of word boundary rules uses the macro values listed
in Table 3a. Each macro represents a repeated union of the basic
Word\_Break property values and is shown in boldface to distinguish it
from the basic property values.

<a id="WB_Rule_Macros"></a>

Table 3a. [Word\_Break
Rule Macros](006-4-1-default-word-boundary-specification.md#WB_Rule_Macros)

| Macro | Represents |
| --- | --- |
| **AHLetter** | (ALetter | Hebrew\_Letter) |
| **MidNumLetQ** | (MidNumLet | Single\_Quote) |

