# Continuation 3

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-2-definitions.md) · [Next](004-3-introduction.md)

<table class="gray">
<tr>
<td class="grayfirst" style="border-bottom-style: solid; ; border-bottom-width: 2px" valign="top" width="5%">
<p><b>Class</b></p></td>
<td class="grayfirst" style="border-bottom-style: solid; ; border-bottom-width: 2px" valign="top" width="23%">
<p><b>Descriptive Name</b></p></td>
<td class="grayfirst" style="border-bottom-style: solid; ; border-bottom-width: 2px" valign="top" width="22%">
<p><b>Examples</b></p></td>
<td class="grayfirst" style="border-bottom-style: solid; ; border-bottom-width: 2px" valign="top" width="44%">
<p><b>Behavior</b></p></td>
</tr>
<tr>
<td class="graymiddle" colspan="4" style="border-bottom-style: solid; border-bottom-width: 1px" valign="top">
<p style="text-align:center; margin-top: 1em"><b>Non-tailorable Line Breaking Classes</b></p></td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="008-bb-break-before.md#BK">BK</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Mandatory Break</i></td>
<td class="graymiddle" valign="top" width="22%">NL, PARAGRAPH SEPARATOR</td>
<td class="graymiddle" valign="top" width="44%">Cause a line break (after)</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="010-cp-closing-parenthesis.md#CR">CR</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Carriage Return</i></td>
<td class="graymiddle" valign="top" width="22%">CR</td><td class="graymiddle" valign="top" width="44%">
            Cause a line break (after), except between CR and LF</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="012-hl-hebrew-letter.md#LF">LF</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Line Feed</i></td>
<td class="graymiddle" valign="top" width="22%">LF</td>
<td class="graymiddle" valign="top" width="44%">Cause a line break (after)</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="009-cb-contingent-break-opportunity.md#CM">CM</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Combining Mark</i></td>
<td class="graymiddle" valign="top" width="22%">Combining marks, control codes</td>
<td class="graymiddle" valign="top" width="44%">Prohibit a line break between the character and the preceding character</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"> <a class="charclass" href="012-hl-hebrew-letter.md#NL">NL</a></td>
<td class="graymiddle" valign="top" width="23%"> <i>Next Line</i></td>
<td class="graymiddle" valign="top" width="22%"> NEL</td>
<td class="graymiddle" valign="top" width="44%"> Cause a line break (after)</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="014-qu-quotation.md#SG">SG</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Surrogate</i></td>
<td class="graymiddle" valign="top" width="22%">Surrogates</td>
<td class="graymiddle" valign="top" width="44%">Do not occur in well-formed text</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"> <a class="charclass" href="015-sy-symbols-allowing-break-after.md#WJ">WJ</a></td>
<td class="graymiddle" valign="top" width="23%"> <i>Word Joiner </i></td>
<td class="graymiddle" valign="top" width="22%"> WJ</td>
<td class="graymiddle" valign="top" width="44%"> Prohibit line breaks before and after</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="016-zw-zero-width-space-non-tailorable.md#ZW">ZW</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Zero Width Space</i></td>
<td class="graymiddle" valign="top" width="22%">ZWSP</td>
<td class="graymiddle" valign="top" width="44%">Provide a break opportunity</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"> <a class="charclass" href="010-cp-closing-parenthesis.md#GL">GL</a></td>
<td class="graymiddle" valign="top" width="23%"> <i>Non-breaking (“Glue”)</i></td>
<td class="graymiddle" valign="top" width="22%"> CGJ, NBSP, ZWNBSP</td>
<td class="graymiddle" valign="top" width="44%"> Prohibit line breaks before and after</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"> <a class="charclass" href="014-qu-quotation.md#SP">SP</a></td>
<td class="graymiddle" valign="top" width="23%"> <i>Space</i></td>
<td class="graymiddle" valign="top" width="22%"> <span class="graymiddle">SPACE</span></td>
<td class="graymiddle" valign="top" width="44%"> Enable indirect line breaks</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"> <a class="charclass" href="016-zw-zero-width-space-non-tailorable.md#ZWJ">ZWJ</a></td>
<td class="graymiddle" valign="top" width="23%"> <i>Zero Width Joiner</i></td>
<td class="graymiddle" valign="top" width="22%"> <span class="graymiddle">Zero Width Joiner</span></td>
<td class="graymiddle" valign="top" width="44%"> Prohibit line breaks within joiner sequences</td>
</tr>
<tr>
<td class="graymiddle" colspan="4" style="border-bottom-style: solid; border-bottom-width: 1px" valign="top">
<p style="text-align:center; margin-top: 1em"><b>Break Opportunities</b></p></td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"> <a class="charclass" href="008-bb-break-before.md#B2">B2</a></td>
<td class="graymiddle" valign="top" width="23%"> <i>Break Opportunity Before and After</i></td>
<td class="graymiddle" valign="top" width="22%">Em dash</td>
<td class="graymiddle" valign="top" width="44%">Provide a line break opportunity before and after the character</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="007-al-ordinary-alphabetic-and-symbol-characters.md#BA">BA</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Break After</i></td>
<td class="graymiddle" valign="top" width="22%">Spaces, most sentence-terminal punctuation</td>
<td class="graymiddle" valign="top" width="44%">Generally provide a line break opportunity after the character</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="008-bb-break-before.md#BB">BB</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Break Before</i></td>
<td class="graymiddle" valign="top" width="22%">Punctuation used in dictionaries</td>
<td class="graymiddle" valign="top" width="44%">Generally provide a line break opportunity before the character</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="011-h2-hangul-lv-syllable.md#HY">HY</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Hyphen</i></td>
<td class="graymiddle" valign="top" width="22%">HYPHEN-MINUS</td>
<td class="graymiddle" valign="top" width="44%">Provide a line break opportunity after the character, except in numeric context</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="011-h2-hangul-lv-syllable.md#HH">HH</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Unambiguous Hyphen</i></td>
<td class="graymiddle" valign="top" width="22%">HYPHEN</td>
<td class="graymiddle" valign="top" width="44%">Generally provide a line break opportunity after the character, except word-initially.</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"> <a class="charclass" href="009-cb-contingent-break-opportunity.md#CB">CB</a></td>
<td class="graymiddle" valign="top" width="23%"> <i>Contingent Break Opportunity</i></td>
<td class="graymiddle" valign="top" width="22%"> Inline objects</td>
<td class="graymiddle" valign="top" width="44%"> Provide a line break opportunity contingent on additional information</td>
</tr>
<tr>
<td class="graymiddle" colspan="4" style="border-bottom-style: solid; border-bottom-width: 1px" valign="top">
<p style="text-align:center; margin-top: 1em"><b>Characters Prohibiting Certain Breaks</b></p></td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="009-cb-contingent-break-opportunity.md#CL">CL</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Close Punctuation</i></td>
<td class="graymiddle" valign="top" width="22%">“}”, “❳”, “⟫” etc.</td>
<td class="graymiddle" valign="top" width="44%">Prohibit line breaks before</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="010-cp-closing-parenthesis.md#CP">CP</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Close Parenthesis</i></td>
<td class="graymiddle" valign="top" width="22%">“)”, “]”</td>
<td class="graymiddle" valign="top" width="44%">Prohibit line breaks before</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="010-cp-closing-parenthesis.md#EX">EX</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Exclamation/</i><br/> <i>Interrogation</i></td>
<td class="graymiddle" valign="top" width="22%">“!”, “?”, etc.</td>
<td class="graymiddle" valign="top" width="44%">Prohibit line breaks before</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="012-hl-hebrew-letter.md#IN">IN</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Inseparable</i></td>
<td class="graymiddle" valign="top" width="22%">Leaders</td>
<td class="graymiddle" valign="top" width="44%">Allow only indirect line breaks between pairs</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="013-ns-nonstarters.md#NS">NS</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Nonstarter</i></td>
<td class="graymiddle" valign="top" width="22%">“‼”, “‽”, “⁇”, “⁉”, etc.</td>
<td class="graymiddle" valign="top" width="44%">Allow only indirect line breaks before</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="013-ns-nonstarters.md#OP">OP</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Open Punctuation</i></td>
<td class="graymiddle" valign="top" width="22%">“(“, “[“, “{“, etc.</td>
<td class="graymiddle" valign="top" width="44%">Prohibit line breaks after</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="014-qu-quotation.md#QU">QU</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Quotation</i></td>
<td class="graymiddle" valign="top" width="22%">Quotation marks</td>
<td class="graymiddle" valign="top" width="44%">Act like they are opening, closing, or both</td>
</tr>
<tr>
<td class="graymiddle" colspan="4" style="border-bottom-style: solid; border-bottom-width: 1px" valign="top">
<p style="text-align:center; margin-top: 1em"><b>Numeric Context</b></p></td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="012-hl-hebrew-letter.md#IS">IS</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Infix Numeric Separator</i></td>
<td class="graymiddle" valign="top" width="22%">. ,</td>
<td class="graymiddle" valign="top" width="44%">Prevent breaks after any and before numeric</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="013-ns-nonstarters.md#NU">NU</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Numeric</i></td>
<td class="graymiddle" valign="top" width="22%">Digits</td>
<td class="graymiddle" valign="top" width="44%">Form numeric expressions for line breaking purposes</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="013-ns-nonstarters.md#PO">PO</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Postfix Numeric</i></td>
<td class="graymiddle" valign="top" width="22%">%, ¢</td>
<td class="graymiddle" valign="top" width="44%">Do not break following a numeric expression</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="013-ns-nonstarters.md#PR">PR</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Prefix Numeric</i></td>
<td class="graymiddle" valign="top" width="22%">$, £, ¥, etc.</td>
<td class="graymiddle" valign="top" width="44%">Do not break in front of a numeric expression</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="015-sy-symbols-allowing-break-after.md#SY">SY</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Symbols Allowing Break After</i></td>
<td class="graymiddle" valign="top" width="22%">/</td>
<td class="graymiddle" valign="top" width="44%">Prevent a break before, and allow a break after</td>
</tr>
<tr>
<td class="graymiddle" colspan="4" style="border-bottom-style: solid; border-bottom-width: 1px" valign="top">
<p style="text-align:center; margin-top: 1em"><b>Other Characters</b></p></td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="006-5-1-description-of-line-breaking-properties.md#AI">AI</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Ambiguous (Alphabetic or Ideographic)</i></td>
<td class="graymiddle" valign="top" width="22%">Characters with Ambiguous East Asian Width</td>
<td class="graymiddle" valign="top" width="44%">Act like <a class="charclass" href="007-al-ordinary-alphabetic-and-symbol-characters.md#AL">AL</a> when the resolved
            <abbr title="East Asian Width">EAW</abbr> is N; otherwise, act as <a class="charclass" href="011-h2-hangul-lv-syllable.md#ID">ID</a></td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="006-5-1-description-of-line-breaking-properties.md#AK">AK</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Aksara</i></td>
<td class="graymiddle" valign="top" width="22%">Consonants</td>
<td class="graymiddle" valign="top" width="44%">Form orthographic syllables in Brahmic scripts</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="007-al-ordinary-alphabetic-and-symbol-characters.md#AL">AL</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Alphabetic</i></td>
<td class="graymiddle" valign="top" width="22%">Alphabets and regular symbols</td>
<td class="graymiddle" valign="top" width="44%">Are alphabetic characters or symbols that are used with alphabetic characters</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="007-al-ordinary-alphabetic-and-symbol-characters.md#AP">AP</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Aksara Pre-Base</i></td>
<td class="graymiddle" valign="top" width="22%">Pre-base repha</td>
<td class="graymiddle" valign="top" width="44%">Form orthographic syllables in Brahmic scripts</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="007-al-ordinary-alphabetic-and-symbol-characters.md#AS">AS</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Aksara Start</i></td>
<td class="graymiddle" valign="top" width="22%">Independent vowels</td>
<td class="graymiddle" valign="top" width="44%">Form orthographic syllables in Brahmic scripts</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="009-cb-contingent-break-opportunity.md#CJ">CJ</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Conditional Japanese Starter</i></td>
<td class="graymiddle" valign="top" width="22%">Small kana</td>
<td class="graymiddle" valign="top" width="44%">Treat as <a class="charclass" href="013-ns-nonstarters.md#NS">NS</a> or
                                                       <a class="charclass" href="011-h2-hangul-lv-syllable.md#ID">ID</a> for strict or normal breaking.</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="010-cp-closing-parenthesis.md#EB">EB</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Emoji Base</i></td>
<td class="graymiddle" valign="top" width="22%">All emoji allowing modifiers</td>
<td class="graymiddle" valign="top" width="44%">Do not break from following Emoji Modifier</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="010-cp-closing-parenthesis.md#EM">EM</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Emoji Modifier</i></td>
<td class="graymiddle" valign="top" width="22%">Skin tone modifiers</td>
<td class="graymiddle" valign="top" width="44%">Do not break from preceding Emoji Base</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="011-h2-hangul-lv-syllable.md#H2">H2</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Hangul LV Syllable</i></td>
<td class="graymiddle" valign="top" width="22%">Hangul</td>
<td class="graymiddle" valign="top" width="44%">Form Korean syllable blocks</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="011-h2-hangul-lv-syllable.md#H3">H3</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Hangul LVT Syllable</i></td>
<td class="graymiddle" valign="top" width="22%">Hangul</td>
<td class="graymiddle" valign="top" width="44%">Form Korean syllable blocks</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="012-hl-hebrew-letter.md#HL">HL</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Hebrew Letter</i></td>
<td class="graymiddle" valign="top" width="22%">Hebrew</td>
<td class="graymiddle" valign="top" width="44%">Special rules around hyphens and SOLIDUS; otherwise act as Alphabetic</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="011-h2-hangul-lv-syllable.md#ID">ID</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Ideographic</i></td>
<td class="graymiddle" valign="top" width="22%">Ideographs</td>
<td class="graymiddle" valign="top" width="44%">Break before or after, except in some numeric context</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="012-hl-hebrew-letter.md#JL">JL</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Hangul L Jamo</i></td>
<td class="graymiddle" valign="top" width="22%">Conjoining jamo</td>
<td class="graymiddle" valign="top" width="44%">Form Korean syllable blocks</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="012-hl-hebrew-letter.md#JV">JV</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Hangul V Jamo</i></td>
<td class="graymiddle" valign="top" width="22%">Conjoining jamo</td>
<td class="graymiddle" valign="top" width="44%">Form Korean syllable blocks</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="012-hl-hebrew-letter.md#JT">JT</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Hangul T Jamo</i></td>
<td class="graymiddle" valign="top" width="22%">Conjoining jamo</td>
<td class="graymiddle" valign="top" width="44%">Form Korean syllable blocks</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="014-qu-quotation.md#RI">RI</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Regional Indicator</i></td>
<td class="graymiddle" valign="top" width="22%">REGIONAL INDICATOR SYMBOL LETTER A .. Z</td>
<td class="graymiddle" valign="top" width="44%">Keep pairs
                                                      together. For pairs, break before and after
                                                      other classes</td>
</tr>
<tr>
<td class="graymiddle" valign="top"><a class="charclass" href="014-qu-quotation.md#SA">SA</a></td>
<td class="graymiddle" valign="top"><i>Complex Context Dependent (South East Asian)</i></td>
<td class="graymiddle" valign="top">South East Asian: Thai, Lao, Khmer</td>
<td class="graymiddle" valign="top">Provide a line break opportunity contingent on additional, language-specific context analysis</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="015-sy-symbols-allowing-break-after.md#VF">VF</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Virama Final</i></td>
<td class="graymiddle" valign="top" width="22%">Viramas for final consonants</td>
<td class="graymiddle" valign="top" width="44%">Form orthographic syllables in Brahmic scripts</td>
</tr>
<tr>
<td class="graymiddle" valign="top" width="5%"><a class="charclass" href="015-sy-symbols-allowing-break-after.md#VI">VI</a></td>
<td class="graymiddle" valign="top" width="23%"><i>Virama</i></td>
<td class="graymiddle" valign="top" width="22%">Conjoining viramas</td>
<td class="graymiddle" valign="top" width="44%">Form orthographic syllables in Brahmic scripts</td>
</tr>
<tr>
<td class="graylast" valign="top" width="5%"><a class="charclass" href="015-sy-symbols-allowing-break-after.md#XX">XX</a></td>
<td class="graylast" valign="top" width="23%"><i>Unknown</i></td>
<td class="graylast" valign="top" width="22%">Most unassigned, private-use</td>
<td class="graylast" valign="top" width="44%">Have as yet unknown line breaking behavior or unassigned code positions</td>
</tr>
</table>

