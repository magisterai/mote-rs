# 4.3.2 HL4 Example 2 for Program Text

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-4-1-boundary-neutrals.md) · [Next](016-4-4-bidirectional-conformance-testing.md)

<a id="HL4Example2"></a>

### 4.3.2  [HL4 Example 2 for Program Text](015-4-3-2-hl4-example-2-for-program-text.md#HL4Example2)

Consider the following two lines:

> (1) `x + tav == 1`

> (2) `x + תו == 1`

Internally, they are the same except that the ASCII identifier `tav` in line (1) is replaced by the Hebrew
identifier `תו` in line (2). However, with a plain text display (with left-to-right paragraph direction) the user
will be misled, thinking that line (2) is a comparison between `(x + 1)` and `תו`, whereas it is actually a
comparison between `(x + תו)` and `1`.
The misleading rendering of (2) occurs because the directionality of the identifier תו influences subsequent
weakly-directional tokens, so that the entire sequence “`תו == 1`” is at a higher resolved level.

This is
illustrated in the first row of the following table, wherein characters at a resolved level higher than the
embedding level are highlighted.
Note that while the RTL display of that expression (second row) is not misleading, as the left-to-right
directionality of `x` does not influence the subsequent text, a similar issue would arise if the terms were
swapped (third row).

<table class="subtle">
<tr>
<th>Paragraph direction</th>
<th colspan="11">Underlying Representation</th>
<th>Display</th>
</tr>
<tr>
<td>LTR</td>
<td><code>x</code></td>
<td><code> </code></td>
<td><code>+</code></td>
<td><code> </code></td>
<td class="higher-resolved-level"><code>ת</code></td>
<td class="higher-resolved-level"><code>ו</code></td>
<td class="higher-resolved-level"><code> </code></td>
<td class="higher-resolved-level"><code>=</code></td>
<td class="higher-resolved-level"><code>=</code></td>
<td class="higher-resolved-level"><code> </code></td>
<td class="higher-resolved-level"><code>1</code></td>
<td dir="ltr"><code>x + <span class="higher-resolved-level">תו == 1</span></code></td>
</tr>
<tr>
<td>RTL</td>
<td class="higher-resolved-level"><code>x</code></td>
<td><code> </code></td>
<td><code>+</code></td>
<td><code> </code></td>
<td><code>ת</code></td>
<td><code>ו</code></td>
<td><code> </code></td>
<td><code>=</code></td>
<td><code>=</code></td>
<td><code> </code></td>
<td class="higher-resolved-level"><code>1</code></td>
<td dir="rtl" style="text-align: right"><code><span class="higher-resolved-level">x</span> + תו == <span class="higher-resolved-level">1</span></code></td>
</tr>
<tr>
<td>RTL</td>
<td><code>ת</code></td>
<td><code>ו</code></td>
<td><code> </code></td>
<td><code>+</code></td>
<td><code> </code></td>
<td class="higher-resolved-level"><code>x</code></td>
<td class="higher-resolved-level"><code> </code></td>
<td class="higher-resolved-level"><code>=</code></td>
<td class="higher-resolved-level"><code>=</code></td>
<td class="higher-resolved-level"><code> </code></td>
<td class="higher-resolved-level"><code>1</code></td>
<td dir="rtl" style="text-align: right"><code>תו + <span class="higher-resolved-level">x == 1</span></code></td>
</tr>
</table>

It would be better to apply protocol HL4 when displaying these expressions, treating each identifier as a separate
segment, thus isolating it from the rest of the source text, and then ordering the segments in a consistent direction,
as shown in the following table.

<table class="subtle">
<tr>
<th>Segment order</th>
<th colspan="9">Segments</th>
<th>Display</th>
</tr>
<tr>
<td>LTR</td>
<td><code>x</code></td>
<td><code> </code></td>
<td><code>+</code></td>
<td><code> </code></td>
<td><code>תו</code></td>
<td><code> </code></td>
<td><code>==</code></td>
<td><code> </code></td>
<td><code>1</code></td>
<td dir="ltr"><code>
<span dir="ltr">x</span>
<span dir="ltr"> </span>
<span dir="ltr">+</span>
<span dir="ltr"> </span>
<span dir="ltr">תו</span>
<span dir="ltr"> </span>
<span dir="ltr">==</span>
<span dir="ltr"> </span>
<span dir="ltr">1</span>
</code></td>
</tr>
<tr>
<td>RTL</td>
<td><code>x</code></td>
<td><code> </code></td>
<td><code>+</code></td>
<td><code> </code></td>
<td><code>תו</code></td>
<td><code> </code></td>
<td><code>==</code></td>
<td><code> </code></td>
<td><code>1</code></td>
<td dir="rtl" style="text-align: right"><code>
<span dir="rtl">x</span>
<span dir="rtl"> </span>
<span dir="rtl">+</span>
<span dir="rtl"> </span>
<span dir="rtl">תו</span>
<span dir="rtl"> </span>
<span dir="rtl">==</span>
<span dir="rtl"> </span>
<span dir="rtl">1</span>
</code></td>
</tr>
<tr>
<td>RTL</td>
<td><code>תו</code></td>
<td><code> </code></td>
<td><code>+</code></td>
<td><code> </code></td>
<td><code>x</code></td>
<td><code> </code></td>
<td><code>==</code></td>
<td><code> </code></td>
<td><code>1</code></td>
<td dir="rtl" style="text-align: right"><code>
<span dir="rtl">תו</span>
<span dir="rtl"> </span>
<span dir="rtl">+</span>
<span dir="rtl"> </span>
<span dir="rtl">x</span>
<span dir="rtl"> </span>
<span dir="rtl">==</span>
<span dir="rtl"> </span>
<span dir="rtl">1</span>
</code></td>
</tr>
</table>

A specification for the application of protocol HL4 to program text is given in
*Section 4.1, Bidirectional Ordering*, in
Unicode Technical Standard #55, “Unicode Source Code Handling” [[UTS55](https://www.unicode.org/reports/tr41/tr41-36.html#UTS55)].

<a id="HL4Example3"></a>

### 4.3.3  [HL4 Example 3 for URLs](015-4-3-2-hl4-example-2-for-program-text.md#HL4Example3)

When a URL is displayed simply using the BIDI algorithm, the following results are produced. As per convention, uppercase represents RTL letters:

| Environment | Display |
| --- | --- |
| LTR | http://ab.cd.com/mn/op  http://ab.cd.HG.FE.com/LK/JI/mn/op  http://LK/JI/HG.FE |
| RTL | http://ab.cd.com/mn/op  mn/op/LK/JI/com.HG.FE.http://ab.cd  LK/JI/HG.FE//:http |

Note that the various fields of the URL can appear to the user in a jumbled order.
Moreover, if any of the fields contain mixed bidi text (including digits), part of the contents of a field may flip around a delimiter, as in the following:

**Memory positions**

|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Memory pos. | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
| Character | / | 0 | א | ב | 1 | a | b | 2 | / | 3 | c | d | 4 | ו | ד | 5 | / |

**Display positions**

|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Display pos. | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
| Memory pos. | 16 | 15 | 14 | 13 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 3 | 2 | 1 | 0 |
| Character | / | 5 | ד | ו | 1 | a | b | 2 | / | 3 | c | d | 4 | ב | א | 0 | / |

The BIDI display process described in *Section 4.1, Bidirectional Ordering*, in Unicode Technical Standard #55, “Unicode Source Code Handling” [[UTS55](https://www.unicode.org/reports/tr41/tr41-36.html#UTS55)],
can be applied to URLs to remedy this situation.

In applying the rules of that section, the atoms are both the delimiters and the text runs that they delimit.
The latter are referred to as literals in the following.
Delimiters include both the characters that separate the scheme, host, path, query, and fragment,
plus any delimiters within each of those parts, such as query operators.
For example:

|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| http | : | // | foo | . | com | / | dir1 | / | dir2 | ? | hl | = | fr | & | rl | = | CA | # | fii |

The atoms are then displayed in monotonic order (RTL or LTR), and each literal is displayed with a paragraph direction equal to that monotonic order.
This results in the following orderings:

| Environment | Display |
| --- | --- |
| LTR | http://ab.cd.com/mn/op  http://ab.cd.FE.HG.com/JI/LK/mn/op  http://FE.HG/JI/LK |
| RTL | op/mn/com.cd.ab//:http  op/mn/LK/JI/com.HG.FE.cd.ab//:http  LK/JI/HG.FE//:http |

> **Note:**
> This display process is useful even when the labels follow the rule defined by RFC 5893:
>
> 1. It applies to the entire URL, including the path, query, and fragment parts, which are out of scope for that RFC.
> 2. Even within a domain name,
>    the rule specified by RFC 5893 does not suffice to make display order of the sequence of labels consistent with network order,
>    as documented in Section 2 of that RFC.
>
> Being a *display process*, as opposed to a requirement on the text being displayed, it does not conflict with that RFC.

