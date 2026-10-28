# Continuation 8

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-continuation-7.md) · [Next](009-4-2-name-validation.md)

> **Notes:**
>
> - It is not possible to provide a uniform set of rules that
>   resolves all issues across languages or that handles all ambiguous
>   situations within a given language. The goal for the specification
>   presented in this annex is to provide a workable default; tailored
>   implementations can be more sophisticated.
> - The correct interpretation of hyphens in the context of word
>   boundaries is challenging. It is quite common for separate words to
>   be connected with a hyphen: “out-of-the-box,” “under-the-table,”
>   “Italian-American,” and so on. A significant number are hyphenated
>   names, such as “Smith-Hawkins.” When doing a Whole Word Search or
>   query, users expect to find the word within those hyphens. While
>   there are some cases where they are separate words (usually to
>   resolve some ambiguity such as “re-sort” as opposed to “resort”),
>   it is better overall to keep the hyphen out of the default
>   definition. Hyphens include U+002D HYPHEN-MINUS, U+2010 HYPHEN,
>   possibly also U+058A ARMENIAN HYPHEN, and U+30A0 KATAKANA-HIRAGANA
>   DOUBLE HYPHEN.
> - Implementations may build on the information supplied by word
>   boundaries. For example, a spell-checker would first check that
>   each word was valid according to the above definition, checking the
>   four words in “out-of-the-box.” If any of the words failed, it
>   could build the compound word and check if it as a whole sequence
>   was in the dictionary (even if all the components were not in the
>   dictionary), such as with “re-iterate.” Of course, spell-checkers
>   for highly inflected or agglutinative languages will need much more
>   sophisticated algorithms.
> - The use of the apostrophe is ambiguous. It is usually
>   considered part of one word (“can’t” or “aujourd’hui”) but it may
>   also be considered as part of two words (“l’objectif”). A further
>   complication is the use of the same character as an apostrophe and
>   as a quotation mark. Therefore leading or trailing apostrophes are
>   best excluded from the default definition of a word. In some
>   languages, such as French and Italian, tailoring to break words
>   when the character after the apostrophe is a vowel may yield better
>   results in more cases. This can be done by adding a rule WB5a.
>
>   <table class="subtle-nb loose rules">
>   <tr>
>   <td class="rule" colspan="4">Break between apostrophe and
>   							vowels (French, Italian).</td>
>   </tr>
>   <tr>
>   <td>WB5a</td>
>   <td style="text-align: right"><i>apostrophe</i></td>
>   <td style="text-align: center">÷</td>
>   <td>vowels</td>
>   </tr>
>   </table>
>
>   and defining appropriate property values for apostrophe and vowels.
>   Apostrophe includes U+0027 ( ' ) APOSTROPHE and U+2019 ( ’ )
>   RIGHT SINGLE QUOTATION MARK (curly apostrophe). Finally, in some
>   transliteration schemes, apostrophe is used at the beginning of
>   words, requiring special tailoring.
> - Certain cases such as colons in words (for example, “AIK:are” and “c:a”) are included in
>   the default even though they may be specific to relatively small
>   user communities (Swedish) because they do not occur otherwise, in
>   normal text, and so do not cause a problem for other languages.
> - For Hebrew, a tailoring may include a double quotation mark
>   between letters, because legacy data may contain that in place of
>   U+05F4 ( ״ ) HEBREW PUNCTUATION GERSHAYIM. This can be done
>   by adding double quotation mark to MidLetter. U+05F3 ( ׳ )
>   HEBREW PUNCTUATION GERESH may also be included in a tailoring.
> - Format characters are included if they are not initial. Thus
>   <LRM><ALetter> will break before the <letter>,
>   but there is no break in <ALetter><LRM><ALetter>
>   or <ALetter><LRM>.
> - Characters such as hyphens, apostrophes, quotation marks, and colon
>   should be taken into account when using identifiers that are
>   intended to represent words of one or more natural languages. See
>   Section 2.4, *Specific Character Adjustments*, of [[UAX31](https://www.unicode.org/reports/tr41/tr41-36.html#UAX31)]. Treatment of
>   hyphens, in particular, may be different in the case of processing
>   identifiers than when using word break analysis for a Whole Word
>   Search or query, because when handling identifiers the goal will be
>   to parse maximal units corresponding to natural language “words,”
>   rather than to find smaller word units within longer lexical units
>   connected by hyphens.
> - Normally word breaking does not require breaking between
>   different scripts. However, adding that capability may be useful in
>   combination with other extensions of word segmentation. For
>   example, in Korean the sentence “I live in Chicago.” is written as
>   three segments delimited by spaces:
>
>   - 나는  Chicago에  산다.
>
>   According to Korean standards, the grammatical suffixes, such
>   as “에” meaning “in”, are considered separate words. Thus the above
>   sentence would be broken into the following five words:
>
>   - 나,  는,  Chicago,  에, and  산다.
>
>   Separating the first two words requires a dictionary lookup,
>   but for Latin text (“Chicago”) the separation is trivial based on
>   the script boundary.
> - Modifier letters (General\_Category = Lm) are almost
>   all included in the ALetter class, by virtue of their Alphabetic
>   property value. Thus, by default, modifier letters do not cause
>   word breaks and should be included in word selections. Modifier
>   symbols (General\_Category = Sk) are not in the ALetter class and so
>   do cause word breaks by default.
> - Some or all of the following characters may be tailored to
>   be in MidLetter, depending on the environment:
>   - U+002D ( - ) HYPHEN-MINUS  
>      U+055A ( ՚ ) ARMENIAN
>     APOSTROPHE  
>      U+058A ( ֊ ) ARMENIAN HYPHEN  
>      U+0F0B (
>     ་ ) TIBETAN MARK INTERSYLLABIC TSHEG  
>      U+1806 ( ᠆ )
>     MONGOLIAN TODO SOFT HYPHEN  
>      U+2010 ( ‐ ) HYPHEN  
>     U+2011 ( ‑ ) NON-BREAKING HYPHEN  
>      U+201B ( ‛ ) SINGLE
>     HIGH-REVERSED-9 QUOTATION MARK  
>      U+30A0 ( ゠ )
>     KATAKANA-HIRAGANA DOUBLE HYPHEN  
>      U+30FB ( ・ ) KATAKANA
>     MIDDLE DOT  
>      U+FE63 ( ﹣ ) SMALL HYPHEN-MINUS  
>     U+FF0D ( － ) FULLWIDTH HYPHEN-MINUS
>   - In UnicodeSet notation, this is: [[\u002D\uFF0D\uFE63\u058A\u1806\u2010\u2011\u30A0\u30FB\u201B\u055A\u0F0B](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=[\u002D\uFF0D\uFE63\u058A\u1806\u2010\u2011\u30A0\u30FB\u201B\u055A\u0F0B])]
>   - For example, some writing systems use a hyphen character
>     between syllables within a word. An example is the Iu Mien
>     language written with the Thai script. Such words should behave as
>     single words for the purpose of selection (“double-click”),
>     indexing, and so forth, meaning that they should not word-break on
>     the hyphen.
> - Some or all of the following characters may be tailored to
>   be in MidNum, depending on the environment, to allow for languages
>   that use spaces as thousands separators, such as €1 234,56.
>   - U+0020 SPACE  
>      U+00A0 NO-BREAK SPACE   
>     U+2007 FIGURE SPACE  
>      U+2008 PUNCTUATION SPACE  
>     U+2009 THIN SPACE  
>      U+202F NARROW NO-BREAK SPACE
>   - In UnicodeSet notation, this is: [[\u0020\u00A0\u2007\u2008\u2009\u202F](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=[\u0020\u00A0\u2007\u2008\u2009\u202F])]

