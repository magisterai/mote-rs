# 3.3 Resolving Embedding Levels

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-3-1-3-paired-brackets.md) · [Next](008-3-3-1-the-paragraph-level.md)

<a id="L"></a>

<a id="R"></a>

<a id="AL"></a>

<a id="EN"></a>

<a id="ES"></a>

<a id="ET"></a>

<a id="AN"></a>

<a id="CS"></a>

<a id="NSM"></a>

<a id="BN"></a>

<a id="B"></a>

<a id="S"></a>

<a id="WS"></a>

<a id="ON"></a>

<a id="LRE"></a>

<a id="LRO"></a>

<a id="RLE"></a>

<a id="RLO"></a>

<a id="PDF"></a>

<a id="LRI"></a>

<a id="RLI"></a>

<a id="FSI"></a>

<a id="PDI"></a>

<table class="subtle">
<tr>
<th>Category</th>
<th>Type</th>
<th width="25%">Description</th>
<th>General Scope</th>
</tr>
<tr>
<td rowspan="3"><b>Strong</b></td>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#L" name="L">L</a></b></td>
<td valign="TOP">Left-to-Right</td>
<td valign="TOP">LRM, most alphabetic, syllabic, Han ideographs,
			non-European or non-Arabic digits, ...</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#R" name="R">R</a></b></td>
<td valign="TOP">Right-to-Left</td>
<td valign="TOP">RLM, Hebrew alphabet, and related punctuation</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#AL" name="AL">AL</a></b></td>
<td valign="TOP">Right-to-Left Arabic</td>
<td valign="TOP">ALM, Arabic, Thaana, and Syriac alphabets, most punctuation specific to
            those scripts, ...</td>
</tr>
<tr>
<td rowspan="7"><b>Weak</b></td>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#EN" name="EN">EN</a></b></td>
<td valign="TOP">European Number</td>
<td valign="TOP">European digits, Eastern Arabic-Indic digits, ...</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#ES" name="ES">ES</a></b></td>
<td valign="TOP">European Number Separator</td>
<td valign="TOP">PLUS SIGN, MINUS SIGN</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#ET" name="ET">ET</a></b></td>
<td valign="TOP">European Number Terminator</td>
<td valign="TOP">DEGREE SIGN, currency symbols, ...</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#AN" name="AN">AN</a></b></td>
<td valign="TOP">Arabic Number</td>
<td valign="TOP">Arabic-Indic digits, Arabic decimal and thousands separators, ...</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#CS" name="CS">CS</a></b></td>
<td valign="TOP">Common Number Separator</td>
<td valign="TOP">COLON, COMMA, FULL STOP, NO-BREAK SPACE, ...</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#NSM" name="NSM">NSM</a></b></td>
<td valign="TOP">Nonspacing Mark</td>
<td valign="TOP">Characters
            with the General_Category values: Mn (Nonspacing_Mark) and Me (Enclosing_Mark)</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#BN" name="BN">BN</a></b></td>
<td valign="TOP">Boundary Neutral</td>
<td valign="TOP">Default ignorables, non-characters, and control characters, other than
			those explicitly given other types.</td>
</tr>
<tr>
<td rowspan="4"><b>Neutral</b></td>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#B" name="B">B</a></b></td>
<td valign="TOP">Paragraph Separator</td>
<td valign="TOP">PARAGRAPH SEPARATOR,
			appropriate Newline Functions, higher-level protocol paragraph
			determination</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#S" name="S">S</a></b></td>
<td valign="TOP">Segment Separator</td>
<td valign="TOP"><i>Tab</i></td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#WS" name="WS">WS</a></b></td>
<td valign="TOP">Whitespace</td>
<td valign="TOP">SPACE, FIGURE SPACE, LINE SEPARATOR, FORM FEED,
			General Punctuation spaces, ...</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#ON" name="ON">ON</a></b></td>
<td valign="TOP">Other Neutrals</td>
<td valign="TOP">All other characters, including OBJECT REPLACEMENT CHARACTER</td>
</tr>
<tr>
<td rowspan="9"><b>Explicit Formatting</b></td>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#LRE" name="LRE">LRE</a></b></td>
<td valign="TOP">Left-to-Right Embedding</td>
<td valign="TOP">LRE</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#LRO" name="LRO">LRO</a></b></td>
<td valign="TOP">Left-to-Right Override</td>
<td valign="TOP">LRO</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#RLE" name="RLE">RLE</a></b></td>
<td valign="TOP">Right-to-Left Embedding</td>
<td valign="TOP">RLE</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#RLO" name="RLO">RLO</a></b></td>
<td valign="TOP">Right-to-Left Override</td>
<td valign="TOP">RLO</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#PDF" name="PDF">PDF</a></b></td>
<td valign="TOP">Pop Directional Format</td>
<td valign="TOP">PDF</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#LRI" name="LRI">LRI</a></b></td>
<td valign="TOP">Left-to-Right Isolate</td>
<td valign="TOP">LRI</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#RLI" name="RLI">RLI</a></b></td>
<td valign="TOP">Right-to-Left Isolate</td>
<td valign="TOP">RLI</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#FSI" name="FSI">FSI</a></b></td>
<td valign="TOP">First Strong Isolate</td>
<td valign="TOP">FSI</td>
</tr>
<tr>
<td valign="TOP"><b><a href="007-3-3-resolving-embedding-levels.md#PDI" name="PDI">PDI</a></b></td>
<td valign="TOP">Pop Directional Isolate</td>
<td valign="TOP">PDI</td>
</tr>
</table>

<a id="Resolving_Embedding_Levels"></a>

### 3.3  [Resolving Embedding Levels](007-3-3-resolving-embedding-levels.md#Resolving_Embedding_Levels)

The body of the Bidirectional Algorithm uses bidirectional character types,
explicit formatting characters, and bracket pairs to produce a
list of resolved levels. This resolution process consists of the following steps:

- Applying rule [P1](008-3-3-1-the-paragraph-level.md#P1) to split the text into paragraphs, and for each of these:
  - Applying rules [P2](008-3-3-1-the-paragraph-level.md#P2) and [P3](008-3-3-1-the-paragraph-level.md#P3)
    to determine the paragraph level.
  - Applying rule [X1](008-3-3-1-the-paragraph-level.md#X1) (which employs rules [X2](008-3-3-1-the-paragraph-level.md#X2)–[X8](009-non-formatting-characters.md#X8))
    to determine explicit embedding levels and directions.
  - Applying rule [X9](010-3-3-3-preparations-for-implicit-processing.md#X9) to remove many control characters from further consideration.
  - Applying rule [X10](010-3-3-3-preparations-for-implicit-processing.md#X10) to split the paragraph into isolating run sequences
    and for each of these:
    - Applying rules [W1](011-3-3-4-resolving-weak-types.md#W1)–[W7](011-3-3-4-resolving-weak-types.md#W7)
      to resolve weak types.
    - Applying rules [N0](011-3-3-4-resolving-weak-types.md#N0)–[N2](011-3-3-4-resolving-weak-types.md#N2)
      to resolve neutral types.
    - Applying rules [I1](012-3-3-6-resolving-implicit-levels.md#I1)–[I2](012-3-3-6-resolving-implicit-levels.md#I2)
      to resolve implicit embedding levels.

