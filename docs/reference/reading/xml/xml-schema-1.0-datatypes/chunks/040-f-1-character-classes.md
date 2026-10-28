# F.1 Character Classes

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](039-e-1-algorithm.md) · [Next](041-f-1-1-character-class-escapes.md)

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Quanitifer</font></strong></td></tr><tr><td><a id="quant"></a><a id="nt-quantifier"></a><a id="quantity"></a><a id="nt-quantity"></a><a id="quantRange"></a><a id="nt-quantRange"></a><a id="quantMin"></a><a id="nt-quantMin"></a><a id="quantExact"></a><a id="nt-QuantExact"></a><table border="0"><tbody><tr valign="baseline"><td><a id="quant" name="quant"></a><a id="nt-quantifier" name="nt-quantifier"></a>[4]   </td><td><code>quantifier</code></td><td>   ::=   </td><td><code>[?*+] | ( '{' <a href="040-f-1-character-classes.md#nt-quantity">quantity</a> '}' )</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="quantity" name="quantity"></a><a id="nt-quantity" name="nt-quantity"></a>[5]   </td><td><code>quantity</code></td><td>   ::=   </td><td><code><a href="040-f-1-character-classes.md#nt-quantRange">quantRange</a> |
			<a href="040-f-1-character-classes.md#nt-quantMin">quantMin</a> | <a href="040-f-1-character-classes.md#nt-QuantExact">QuantExact</a></code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="quantRange" name="quantRange"></a><a id="nt-quantRange" name="nt-quantRange"></a>[6]   </td><td><code>quantRange</code></td><td>   ::=   </td><td><code><a href="040-f-1-character-classes.md#nt-QuantExact">QuantExact</a> ',' <a href="040-f-1-character-classes.md#nt-QuantExact">QuantExact</a></code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="quantMin" name="quantMin"></a><a id="nt-quantMin" name="nt-quantMin"></a>[7]   </td><td><code>quantMin</code></td><td>   ::=   </td><td><code><a href="040-f-1-character-classes.md#nt-QuantExact">QuantExact</a> ','</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="quantExact" name="quantExact"></a><a id="nt-QuantExact" name="nt-QuantExact"></a>[8]   </td><td><code>QuantExact</code></td><td>   ::=   </td><td><code>[0-9]+</code></td></tr></tbody></table></td></tr></tbody></table>

<a id="dt-atom"></a>

[Definition:]
An **atom** is either a
[·normal character·](040-f-1-character-classes.md#dt-normalc), a [·character class·](040-f-1-character-classes.md#dt-charclass), or
a parenthesized [·regular expression·](039-e-1-algorithm.md#dt-regex).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Atom</font></strong></td></tr><tr><td><a id="atom"></a><a id="nt-atom"></a><table border="0"><tbody><tr valign="baseline"><td><a id="atom" name="atom"></a><a id="nt-atom" name="nt-atom"></a>[9]   </td><td><code>atom</code></td><td>   ::=   </td><td><code><a href="040-f-1-character-classes.md#nt-Char">Char</a> |
			<a href="040-f-1-character-classes.md#nt-charClass">charClass</a> | ( '('
			<a href="039-e-1-algorithm.md#nt-regExp">regExp</a> ')' )</code></td></tr></tbody></table></td></tr></tbody></table>

| For all [·normal character·](040-f-1-character-classes.md#dt-normalc)s *c*, [·character class·](040-f-1-character-classes.md#dt-charclass)es *C*, and [·regular expression·](039-e-1-algorithm.md#dt-regex)s *S*, valid [·atom·](040-f-1-character-classes.md#dt-atom)s *R* are: | Denoting the set of strings *L(R)* containing: |
| --- | --- |
| *c* | the single string consisting only of *c* |
| *C* | all strings in *L(C)* |
| (*S*) | all strings in *L(S)* |

<a id="dt-metac"></a>

[Definition:]
A **metacharacter**
is either `.`, `\`, `?`,
`*`, `+`, `{`, `}`
`(`, `)`, `[` or `]`.
These characters have special meanings in [·regular expression·](039-e-1-algorithm.md#dt-regex)s,
but can be escaped to form [·atom·](040-f-1-character-classes.md#dt-atom)s that denote the
sets of strings containing only themselves, i.e., an escaped
[·metacharacter·](040-f-1-character-classes.md#dt-metac) behaves like a [·normal character·](040-f-1-character-classes.md#dt-normalc).

<a id="dt-normalc"></a>

[Definition:]
A
**normal character** is any XML character that is not a
metacharacter. In [·regular expression·](039-e-1-algorithm.md#dt-regex)s, a normal character is an
atom that denotes the singleton set of strings containing only itself.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Normal Character</font></strong></td></tr><tr><td><a id="char"></a><a id="nt-Char"></a><table border="0"><tbody><tr valign="baseline"><td><a id="char" name="char"></a><a id="nt-Char" name="nt-Char"></a>[10]   </td><td><code>Char</code></td><td>   ::=   </td><td><code>[^.\?*+()|#x5B#x5D]</code></td></tr></tbody></table></td></tr></tbody></table>

Note that a [·normal character·](040-f-1-character-classes.md#dt-normalc) can be represented either as
itself, or with a [character
reference](https://www.w3.org/TR/2000/WD-xml-2e-20000814#dt-charref).

<a id="charcter-classes"></a>

### F.1 Character Classes

<a id="dt-charclass"></a>

[Definition:]
A
**character class** is an [·atom·](040-f-1-character-classes.md#dt-atom)
 *R* that identifies a **set of characters**
 *C(R)*. The set of strings *L(R)* denoted by a
character class *R* contains one single-character string
"*c*" for each character *c* in *C(R)*.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Character Class</font></strong></td></tr><tr><td><a id="charClass"></a><a id="nt-charClass"></a><table border="0"><tbody><tr valign="baseline"><td><a id="charClass" name="charClass"></a><a id="nt-charClass" name="nt-charClass"></a>[11]   </td><td><code>charClass</code></td><td>   ::=   </td><td><code>
<a href="041-f-1-1-character-class-escapes.md#nt-charClassEsc">charClassEsc</a> |
			<a href="040-f-1-character-classes.md#nt-charClassExpr">charClassExpr</a> |
			<a href="042-continuation-42.md#nt-WildcardEsc">WildcardEsc</a>
</code></td></tr></tbody></table></td></tr></tbody></table>

A character class is either a [·character class escape·](041-f-1-1-character-class-escapes.md#dt-cces) or a
[·character class expression·](040-f-1-character-classes.md#dt-charexpr).

<a id="dt-charexpr"></a>

[Definition:]
A
**character class expression** is a [·character group·](040-f-1-character-classes.md#dt-chargroup) surrounded
by `[` and `]` characters. For all character
groups *G*, [*G*] is a valid **character class
expression**, identifying the set of characters
*C*([*G*]) = *C*(*G*).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Character Class Expression</font></strong></td></tr><tr><td><a id="charClassExpr"></a><a id="nt-charClassExpr"></a><table border="0"><tbody><tr valign="baseline"><td><a id="charClassExpr" name="charClassExpr"></a><a id="nt-charClassExpr" name="nt-charClassExpr"></a>[12]   </td><td><code>charClassExpr</code></td><td>   ::=   </td><td><code>'[' <a href="040-f-1-character-classes.md#nt-charGroup">charGroup</a> ']'</code></td></tr></tbody></table></td></tr></tbody></table>

<a id="dt-chargroup"></a>

[Definition:]
A
**character group** is either a [·positive character group·](040-f-1-character-classes.md#dt-poschargroup),
a [·negative character group·](040-f-1-character-classes.md#dt-negchargroup), or a [·character class subtraction·](040-f-1-character-classes.md#dt-subchargroup).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Character Group</font></strong></td></tr><tr><td><a id="chargroup"></a><a id="nt-charGroup"></a><table border="0"><tbody><tr valign="baseline"><td><a id="chargroup" name="chargroup"></a><a id="nt-charGroup" name="nt-charGroup"></a>[13]   </td><td><code>charGroup</code></td><td>   ::=   </td><td><code>
<a href="040-f-1-character-classes.md#nt-posCharGroup">posCharGroup</a> |
			<a href="040-f-1-character-classes.md#nt-negCharGroup">negCharGroup</a> |
			<a href="040-f-1-character-classes.md#nt-charClassSub">charClassSub</a>
</code></td></tr></tbody></table></td></tr></tbody></table>

<a id="dt-poschargroup"></a>

[Definition:]
A **positive character group** consists of one or more
[·character range·](041-f-1-1-character-class-escapes.md#dt-charrange)s or [·character class escape·](041-f-1-1-character-class-escapes.md#dt-cces)s, concatenated
together. A **positive character group** identifies the set of
characters containing all of the characters in all of the sets identified
by its constituent ranges or escapes.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Positive Character Group</font></strong></td></tr><tr><td><a id="poschargroup"></a><a id="nt-posCharGroup"></a><table border="0"><tbody><tr valign="baseline"><td><a id="poschargroup" name="poschargroup"></a><a id="nt-posCharGroup" name="nt-posCharGroup"></a>[14]   </td><td><code>posCharGroup</code></td><td>   ::=   </td><td><code>
			(
			<a href="041-f-1-1-character-class-escapes.md#nt-charRange">charRange</a> | <a href="041-f-1-1-character-class-escapes.md#nt-charClassEsc">charClassEsc</a>
			)+
		</code></td></tr></tbody></table></td></tr></tbody></table>

| For all [·character range·](041-f-1-1-character-class-escapes.md#dt-charrange)s *R*, all [·character class escape·](041-f-1-1-character-class-escapes.md#dt-cces)s *E*, and all [·positive character group·](040-f-1-character-classes.md#dt-poschargroup)s *P*, valid [·positive character group·](040-f-1-character-classes.md#dt-poschargroup)s *G* are: | Identifying the set of characters *C(G)* containing: |
| --- | --- |
| *R* | all characters in *C(R)*. |
| *E* | all characters in *C(E)*. |
| *RP* | all characters in *C(R)* and all characters in *C(P)*. |
| *EP* | all characters in *C(E)* and all characters in *C(P)*. |

<a id="dt-negchargroup"></a>

[Definition:]
A **negative character group** is a
[·positive character group·](040-f-1-character-classes.md#dt-poschargroup) preceded by the `^` character.
For all [·positive character group·](040-f-1-character-classes.md#dt-poschargroup)s *P*, ^*P*
is a valid **negative character group**, and *C(^P)*
contains all XML characters that are *not* in *C(P)*.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Negative Character Group</font></strong></td></tr><tr><td><a id="negchargroup"></a><a id="nt-negCharGroup"></a><table border="0"><tbody><tr valign="baseline"><td><a id="negchargroup" name="negchargroup"></a><a id="nt-negCharGroup" name="nt-negCharGroup"></a>[15]   </td><td><code>negCharGroup</code></td><td>   ::=   </td><td><code>'^' <a href="040-f-1-character-classes.md#nt-posCharGroup">posCharGroup</a></code></td></tr></tbody></table></td></tr></tbody></table>

<a id="dt-subchargroup"></a>

[Definition:]
A
**character class subtraction** is a [·character class expression·](040-f-1-character-classes.md#dt-charexpr)
subtracted from a [·positive character group·](040-f-1-character-classes.md#dt-poschargroup) or
[·negative character group·](040-f-1-character-classes.md#dt-negchargroup), using the `-` character.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Character Class Subtraction</font></strong></td></tr><tr><td><a id="charclasssub"></a><a id="nt-charClassSub"></a><table border="0"><tbody><tr valign="baseline"><td><a id="charclasssub" name="charclasssub"></a><a id="nt-charClassSub" name="nt-charClassSub"></a>[16]   </td><td><code>charClassSub</code></td><td>   ::=   </td><td><code>
			( <a href="040-f-1-character-classes.md#nt-posCharGroup">posCharGroup</a> |
				<a href="040-f-1-character-classes.md#nt-negCharGroup">negCharGroup</a> ) '-'
			<a href="040-f-1-character-classes.md#nt-charClassExpr">charClassExpr</a>
</code></td></tr></tbody></table></td></tr></tbody></table>

