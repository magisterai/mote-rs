# 4.2 Name Validation

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-continuation-8.md) · [Next](010-continuation-10.md)

<a id="Name_Validation"></a>

### 4.2  [Name Validation](009-4-2-name-validation.md#Name_Validation)

Related to word determination is the issue of *personal name
validation*. Implementations sometimes need to validate fields in
which personal names are entered. The goal is to distinguish between
characters like those in “James Smith-Faley, Jr.” and those in
“!#@♥≠”. It is important to be reasonably lenient, because users need
to be able to add legitimate names, like “di Silva”, even if the
names contain characters such as *space*. Typically, these
personal name validations should not be language-specific; someone
might be using a Web site in one language while his name is in a
different language, for example. A basic set of name validation
characters consists the characters allowed in words according to the
above definition, plus a number of exceptional characters:

*Basic Name Validation Characters*

- [[\p{name=/COMMA/}\p{name=/FULL
  STOP/}&\p{p}  
   \p{whitespace}-\p{c}  
   \p{alpha}  
  \p{wb=Katakana}\p{wb=Extend}\p{wb=ALetter}\p{wb=MidLetter}\p{wb=MidNumLet}  
  [\u002D\u055A\u058A\u0F0B\u1806\u2010\u2011\u201B\u2E17\u30A0\u30FB\uFE63\uFF0D]](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=[\p{name%3D%2FCOMMA%2F}\p{name%3D%2FFULL+STOP%2F}%26\p{p}%0D%0A\p{whitespace}-\p{c}%0D%0A\p{alpha}%0D%0A\p{wb%3DKatakana}\p{wb%3DExtend}\p{wb%3DALetter}\p{wb%3DMidLetter}\p{wb%3DMidNumLet}%0D%0A[\u002D\u055A\u058A\u0F0B\u1806\u2010\u2011\u201B\u2E17\u30A0\u30FB\uFE63\uFF0D]])]

This is only a basic set of validation characters; in
particular, the following points should be kept in mind:

- It is a lenient, non-language-specific set, and could be
  tailored where only a limited set of languages are permitted, or for
  other environments. For example, the set can be narrowed if name
  fields are separated: “,” and “.” may not be necessary if titles are
  not allowed.
- It includes characters that may not be appropriate for
  identifiers, and some that would not be parts of words. It also
  permits some characters that may be part of words in a broad sense,
  but not part of names, such as in “AIK:are” and “c:a” in Swedish, or hyphenation
  points used in dictionary words.
- Additional tests may be needed in cases where security is at
  issue. In particular, names may be validated by transforming them to
  NFC format, and then testing to ensure that no characters in the
  result of the transformation change under NFKC. A second test is to
  use the information in *Table 5. Recommended Scripts* in *Unicode
  Identifier and Pattern Syntax* [[UAX31](https://www.unicode.org/reports/tr41/tr41-36.html#UAX31)].
  If the name has one or more characters with explicit script values
  that are not in *Table 5*, then reject the name.

<a id="Sentence_Boundaries"></a>

## 5  [Sentence Boundaries](009-4-2-name-validation.md#Sentence_Boundaries)

Sentence boundaries are often used for triple-click or some
other method of selecting or iterating through blocks of text that
are larger than single words. They are also used to determine whether
words occur within the same sentence in database queries.

Plain text provides inadequate information for determining good
sentence boundaries. Periods can signal the end of a sentence,
indicate abbreviations, or be used for decimal points, for example.
Without much more sophisticated analysis, one cannot distinguish
between the two following examples of the sequence <?, ”, space,
uppercase-letter>. In the first example, they mark the end of a
sentence, while in the second they do not.

> |  |  |
> | --- | --- |
> | He said, “Are you going?” | John shook his head. |
>
>   
>
> |  |
> | --- |
> | “Are you going?” John asked. |

Without analyzing the text semantically, it is impossible to be
certain which of these usages is intended (and sometimes ambiguities
still remain). However, in most cases a straightforward mechanism
works well.

> **Note:** As with the other default specifications,
> implementations are free to override (tailor) the results to meet
> the requirements of different environments or particular languages.
> For example, locale-sensitive boundary suppression specifications
> can be expressed in LDML [[UTS35](https://www.unicode.org/reports/tr41/tr41-36.html#UTS35)].
> Specific sentence boundary suppressions are available in the Common
> Locale Data Repository [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)]
> and may be used to improve the quality of boundary analysis.

<a id="Default_Sentence_Boundaries"></a>

### 5.1  [Default Sentence Boundary Specification](009-4-2-name-validation.md#Default_Sentence_Boundaries)

The following is a general specification for sentence boundaries—language-specific rules in [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)] should be used where available.

The Sentence\_Break property value assignments are explicitly listed
in the corresponding data file in [[Props](https://www.unicode.org/reports/tr41/tr41-36.html#Props0)]. The values in that
file are the normative property values.

For illustration, property values are summarized in [*Table 4*](009-4-2-name-validation.md#Table_Sentence_Break_Property_Values),
but the lists of characters are illustrative.

<a id="Table_Sentence_Break_Property_Values"></a>

Table 4. [Sentence\_Break
Property Values](009-4-2-name-validation.md#Table_Sentence_Break_Property_Values)

<a id="CR1"></a>

<a id="LF1"></a>

<a id="Extend1"></a>

<a id="Sep"></a>

<a id="SB_Format"></a>

<a id="Sp"></a>

<a id="Lower"></a>

<a id="Upper"></a>

<a id="OLetter"></a>

<a id="SB_Numeric"></a>

<a id="ATerm"></a>

<a id="SContinue"></a>

<a id="STerm"></a>

<a id="Close"></a>

<a id="AnySB"></a>

| Value | Summary List of Characters |
| --- | --- |
| **[CR](009-4-2-name-validation.md#CR1)** | U+000D CARRIAGE RETURN (CR) |
| **[LF](009-4-2-name-validation.md#LF1)** | U+000A LINE FEED (LF) |
| **[Extend](009-4-2-name-validation.md#Extend1)** | Grapheme\_Extend = Yes, *or*  U+200D ZERO WIDTH JOINER (ZWJ), *or*  General\_Category = Spacing\_Mark |
| **[Sep](009-4-2-name-validation.md#Sep)** | U+0085 NEXT LINE (NEL)  U+2028 LINE SEPARATOR  U+2029 PARAGRAPH SEPARATOR |
| [**Format**](009-4-2-name-validation.md#SB_Format) | General\_Category = Format  *and not* U+200C ZERO WIDTH NON-JOINER (ZWNJ)  *and not* U+200D ZERO WIDTH JOINER (ZWJ) |
| **[Sp](009-4-2-name-validation.md#Sp)** | White\_Space = Yes  *and* Sentence\_Break ≠ Sep  *and* Sentence\_Break ≠ CR  *and* Sentence\_Break ≠ LF |
| **[Lower](009-4-2-name-validation.md#Lower)** | Lowercase = Yes  *and* Grapheme\_Extend = No *and* not in the ranges (for Mkhedruli Georgian)  U+10D0 (ა) GEORGIAN LETTER AN  ..U+10FA (ჺ) GEORGIAN LETTER AIN *and*  U+10FD (ჽ) GEORGIAN LETTER AEN  ..U+10FF (ჿ) GEORGIAN LETTER LABIAL SIGN |
| **[Upper](009-4-2-name-validation.md#Upper)** | General\_Category = Titlecase\_Letter, *or*  Uppercase = Yes *and* not in the ranges (for Mtavruli Georgian)  U+1C90 (Ა) GEORGIAN MTAVRULI CAPITAL LETTER AN  ..U+1CBA (Ჺ) GEORGIAN MTAVRULI CAPITAL LETTER AIN *and*  U+1CBD (Ჽ) GEORGIAN MTAVRULI CAPITAL LETTER AEN  ..U+1CBF (Ჿ) GEORGIAN LETTER MTAVRULI CAPITAL LABIAL SIGN |
| **[OLetter](009-4-2-name-validation.md#OLetter)** | Alphabetic = Yes, *or*  U+00A0 NO-BREAK SPACE (NBSP), *or*  U+05F3 ( ׳ ) HEBREW PUNCTUATION GERESH  *and* Lower = No  *and* Upper = No  *and* Sentence\_Break ≠ Extend |
| [**Numeric**](009-4-2-name-validation.md#SB_Numeric) | Line\_Break = Numeric |
| **[ATerm](009-4-2-name-validation.md#ATerm)** | U+002E ( . ) FULL STOP  U+2024 ( ․ ) ONE DOT LEADER  U+FE52 ( ﹒ ) SMALL FULL STOP  U+FF0E ( ． ) FULLWIDTH FULL STOP |
| **[SContinue](009-4-2-name-validation.md#SContinue)** | U+002C ( , ) COMMA  U+002D ( - ) HYPHEN-MINUS  U+003A ( : ) COLON  U+003B ( ; ) SEMICOLON  U+037E ( ; ) GREEK QUESTION MARK  U+055D ( ՝ ) ARMENIAN COMMA  U+060C ( ، ) ARABIC COMMA  U+060D ( ‎؍‎ ) ARABIC DATE SEPARATOR  U+07F8 ( ߸ ) NKO COMMA  U+1802 ( ᠂ ) MONGOLIAN COMMA  U+1808 ( ᠈ ) MONGOLIAN MANCHU COMMA  U+2013 ( – ) EN DASH  U+2014 ( — ) EM DASH  U+3001 ( 、 ) IDEOGRAPHIC COMMA  U+FE10 ( ︐ ) PRESENTATION FORM FOR VERTICAL COMMA  U+FE11 ( ︑ ) PRESENTATION FORM FOR VERTICAL IDEOGRAPHIC COMMA  U+FE13 ( ︓ ) PRESENTATION FORM FOR VERTICAL COLON  U+FE14 ( ︔ ) PRESENTATION FORM FOR VERTICAL SEMICOLON  U+FE31 ( ︱ ) PRESENTATION FORM FOR VERTICAL EM DASH  U+FE32 ( ︲ ) PRESENTATION FORM FOR VERTICAL EN DASH  U+FE50 ( ﹐ ) SMALL COMMA  U+FE51 ( ﹑ ) SMALL IDEOGRAPHIC COMMA  U+FE54 ( ﹔ ) SMALL SEMICOLON  U+FE55 ( ﹕ ) SMALL COLON  U+FE58 ( ﹘ ) SMALL EM DASH  U+FE63 ( ﹣ ) SMALL HYPHEN-MINUS  U+FF0C ( ， ) FULLWIDTH COMMA  U+FF0D ( － ) FULLWIDTH HYPHEN-MINUS  U+FF1A ( ： ) FULLWIDTH COLON  U+FF1B ( ； ) FULLWIDTH SEMICOLON  U+FF64 ( ､ ) HALFWIDTH IDEOGRAPHIC COMMA |
| **[STerm](009-4-2-name-validation.md#STerm)** | Sentence\_Terminal = Yes  *and not* ATerm |
| **[Close](009-4-2-name-validation.md#Close)** | General\_Category = Open\_Punctuation, *or*  General\_Category = Close\_Punctuation, *or*  Line\_Break = Quotation  *and not* U+05F3 ( ׳ ) HEBREW PUNCTUATION GERESH  *and* ATerm = No  *and* STerm = No |
| **[Any](009-4-2-name-validation.md#AnySB)** | *This is not a property value; it is used in the rules to represent any code point.* |

<a id="Sentence_Boundary_Rules"></a>

#### 5.1.1  [Sentence Boundary Rules](009-4-2-name-validation.md#Sentence_Boundary_Rules)

The table of sentence boundary rules uses the macro values
listed in Table 4a. Each macro represents a repeated union of the
basic Sentence\_Break property values and is shown in boldface to
distinguish it from the basic property values.

<a id="SB_Rule_Macros"></a>

Table 4a. [Sentence\_Break
Rule Macros](009-4-2-name-validation.md#SB_Rule_Macros)

| Macro | Represents |
| --- | --- |
| **ParaSep** | (Sep | CR | LF) |
| **SATerm** | (STerm | ATerm) |

