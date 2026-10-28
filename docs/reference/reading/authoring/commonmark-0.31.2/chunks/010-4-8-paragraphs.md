# 4.8 Paragraphs

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-4-7-link-reference-definitions.md) · [Next](011-5-2-list-items.md)

```
# [Foo]
[foo]: /url
> bar
```






```
<h1><a href="/url">Foo</a></h1>
<blockquote>
<p>bar</p>
</blockquote>
```




<a id="example-215"></a>

[Example 215](010-4-8-paragraphs.md#example-215)Try It



```
[foo]: /url
bar
===
[foo]
```






```
<h1>bar</h1>
<p><a href="/url">foo</a></p>
```




<a id="example-216"></a>

[Example 216](010-4-8-paragraphs.md#example-216)Try It



```
[foo]: /url
===
[foo]
```






```
<p>===
<a href="/url">foo</a></p>
```




Several [link reference definitions](009-4-7-link-reference-definitions.md#link-reference-definition)
can occur one after another, without intervening blank lines.

<a id="example-217"></a>

[Example 217](010-4-8-paragraphs.md#example-217)Try It



```
[foo]: /foo-url "foo"
[bar]: /bar-url
  "bar"
[baz]: /baz-url

[foo],
[bar],
[baz]
```






```
<p><a href="/foo-url" title="foo">foo</a>,
<a href="/bar-url" title="bar">bar</a>,
<a href="/baz-url">baz</a></p>
```




[Link reference definitions](009-4-7-link-reference-definitions.md#link-reference-definition) can occur
inside block containers, like lists and block quotations. They
affect the entire document, not just the container in which they
are defined:

<a id="example-218"></a>

[Example 218](010-4-8-paragraphs.md#example-218)Try It



```
[foo]

> [foo]: /url
```






```
<p><a href="/url">foo</a></p>
<blockquote>
</blockquote>
```




<a id="paragraphs"></a>

## 4.8 Paragraphs

<a id="paragraph"></a>

A sequence of non-blank lines that cannot be interpreted as other
kinds of blocks forms a [paragraph](010-4-8-paragraphs.md#paragraph).
The contents of the paragraph are the result of parsing the
paragraph’s raw content as inlines. The paragraph’s raw content
is formed by concatenating the lines and removing initial and final
spaces or tabs.

A simple example with two paragraphs:

<a id="example-219"></a>

[Example 219](010-4-8-paragraphs.md#example-219)Try It



```
aaa

bbb
```






```
<p>aaa</p>
<p>bbb</p>
```




Paragraphs can contain multiple lines, but no blank lines:

<a id="example-220"></a>

[Example 220](010-4-8-paragraphs.md#example-220)Try It



```
aaa
bbb

ccc
ddd
```






```
<p>aaa
bbb</p>
<p>ccc
ddd</p>
```




Multiple blank lines between paragraphs have no effect:

<a id="example-221"></a>

[Example 221](010-4-8-paragraphs.md#example-221)Try It



```
aaa


bbb
```






```
<p>aaa</p>
<p>bbb</p>
```




Leading spaces or tabs are skipped:

<a id="example-222"></a>

[Example 222](010-4-8-paragraphs.md#example-222)Try It



```
  aaa
 bbb
```






```
<p>aaa
bbb</p>
```




Lines after the first may be indented any amount, since indented
code blocks cannot interrupt paragraphs.

<a id="example-223"></a>

[Example 223](010-4-8-paragraphs.md#example-223)Try It



```
aaa
             bbb
                                       ccc
```






```
<p>aaa
bbb
ccc</p>
```




However, the first line may be preceded by up to three spaces of indentation.
Four spaces of indentation is too many:

<a id="example-224"></a>

[Example 224](010-4-8-paragraphs.md#example-224)Try It



```
   aaa
bbb
```






```
<p>aaa
bbb</p>
```




<a id="example-225"></a>

[Example 225](010-4-8-paragraphs.md#example-225)Try It



```
    aaa
bbb
```






```
<pre><code>aaa
</code></pre>
<p>bbb</p>
```




Final spaces or tabs are stripped before inline parsing, so a paragraph
that ends with two or more spaces will not end with a [hard line
break](022-6-7-hard-line-breaks.md#hard-line-break):

<a id="example-226"></a>

[Example 226](010-4-8-paragraphs.md#example-226)Try It



```
aaa     
bbb     
```






```
<p>aaa<br />
bbb</p>
```




<a id="blank-lines"></a>

## 4.9 Blank lines

[Blank lines](010-4-8-paragraphs.md#blank-lines) between block-level elements are ignored,
except for the role they play in determining whether a [list](013-5-2-1-motivation.md#list)
is [tight](013-5-2-1-motivation.md#tight) or [loose](013-5-2-1-motivation.md#loose).

Blank lines at the beginning and end of the document are also ignored.

<a id="example-227"></a>

[Example 227](010-4-8-paragraphs.md#example-227)Try It



```
  

aaa
  

# aaa

  
```






```
<p>aaa</p>
<h1>aaa</h1>
```




<a id="container-blocks"></a>

# 5 Container blocks

A [container block](010-4-8-paragraphs.md#container-blocks) is a block that has other
blocks as its contents. There are two basic kinds of container blocks:
[block quotes](010-4-8-paragraphs.md#block-quotes) and [list items](011-5-2-list-items.md#list-items).
[Lists](013-5-2-1-motivation.md#lists) are meta-containers for [list items](011-5-2-list-items.md#list-items).

We define the syntax for container blocks recursively. The general
form of the definition is:

> If X is a sequence of blocks, then the result of
> transforming X in such-and-such a way is a container of type Y
> with these blocks as its content.

So, we explain what counts as a block quote or list item by explaining
how these can be *generated* from their contents. This should suffice
to define the syntax, although it does not give a recipe for *parsing*
these constructions. (A recipe is provided below in the section entitled
[A parsing strategy](022-6-7-hard-line-breaks.md#appendix-a-parsing-strategy).)

<a id="block-quotes"></a>

## 5.1 Block quotes

<a id="block-quote-marker"></a>

A [block quote marker](010-4-8-paragraphs.md#block-quote-marker),
optionally preceded by up to three spaces of indentation,
consists of (a) the character `>` together with a following space of
indentation, or (b) a single character `>` not followed by a space of
indentation.

The following rules define [block quotes](010-4-8-paragraphs.md#block-quotes):

1. **Basic case.** If a string of lines *Ls* constitute a sequence
   of blocks *Bs*, then the result of prepending a [block quote
   marker](010-4-8-paragraphs.md#block-quote-marker) to the beginning of each line in *Ls*
   is a [block quote](010-4-8-paragraphs.md#block-quotes) containing *Bs*.
2. <a id="paragraph-continuation-text"></a>

   **Laziness.** If a string of lines *Ls* constitute a [block
   quote](010-4-8-paragraphs.md#block-quotes) with contents *Bs*, then the result of deleting
   the initial [block quote marker](010-4-8-paragraphs.md#block-quote-marker) from one or
   more lines in which the next character other than a space or tab after the
   [block quote marker](010-4-8-paragraphs.md#block-quote-marker) is [paragraph continuation
   text](010-4-8-paragraphs.md#paragraph-continuation-text) is a block quote with *Bs* as its content.
   [Paragraph continuation text](010-4-8-paragraphs.md#paragraph-continuation-text) is text
   that will be parsed as part of the content of a paragraph, but does
   not occur at the beginning of the paragraph.
3. **Consecutiveness.** A document cannot contain two [block
   quotes](010-4-8-paragraphs.md#block-quotes) in a row unless there is a [blank line](002-1-3-about-this-document.md#blank-line) between them.

Nothing else counts as a [block quote](010-4-8-paragraphs.md#block-quotes).

Here is a simple example:

<a id="example-228"></a>

[Example 228](010-4-8-paragraphs.md#example-228)Try It



```
> # Foo
> bar
> baz
```






```
<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>
```




The space or tab after the `>` characters can be omitted:

<a id="example-229"></a>

[Example 229](010-4-8-paragraphs.md#example-229)Try It



```
># Foo
>bar
> baz
```






```
<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>
```




The `>` characters can be preceded by up to three spaces of indentation:

<a id="example-230"></a>

[Example 230](010-4-8-paragraphs.md#example-230)Try It



```
   > # Foo
   > bar
 > baz
```






```
<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>
```




Four spaces of indentation is too many:

<a id="example-231"></a>

[Example 231](010-4-8-paragraphs.md#example-231)Try It



```
    > # Foo
    > bar
    > baz
```






```
<pre><code>&gt; # Foo
&gt; bar
&gt; baz
</code></pre>
```




The Laziness clause allows us to omit the `>` before
[paragraph continuation text](010-4-8-paragraphs.md#paragraph-continuation-text):

<a id="example-232"></a>

[Example 232](010-4-8-paragraphs.md#example-232)Try It



```
> # Foo
> bar
baz
```






```
<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>
```




A block quote can contain some lazy and some non-lazy
continuation lines:

<a id="example-233"></a>

[Example 233](010-4-8-paragraphs.md#example-233)Try It



```
> bar
baz
> foo
```






```
<blockquote>
<p>bar
baz
foo</p>
</blockquote>
```




Laziness only applies to lines that would have been continuations of
paragraphs had they been prepended with [block quote markers](010-4-8-paragraphs.md#block-quote-marker).
For example, the `>`  cannot be omitted in the second line of



```
> foo
> ---
```




without changing the meaning:

<a id="example-234"></a>

[Example 234](010-4-8-paragraphs.md#example-234)Try It



```
> foo
---
```






```
<blockquote>
<p>foo</p>
</blockquote>
<hr />
```




Similarly, if we omit the `>`  in the second line of



```
> - foo
> - bar
```




then the block quote ends after the first line:

<a id="example-235"></a>

[Example 235](010-4-8-paragraphs.md#example-235)Try It



```
> - foo
- bar
```






```
<blockquote>
<ul>
<li>foo</li>
</ul>
</blockquote>
<ul>
<li>bar</li>
</ul>
```




For the same reason, we can’t omit the `>`  in front of
subsequent lines of an indented or fenced code block:

<a id="example-236"></a>

[Example 236](010-4-8-paragraphs.md#example-236)Try It



```
>     foo
    bar
```






```
<blockquote>
<pre><code>foo
</code></pre>
</blockquote>
<pre><code>bar
</code></pre>
```




<a id="example-237"></a>

[Example 237](010-4-8-paragraphs.md#example-237)Try It



````
> ```
foo
```
````






```
<blockquote>
<pre><code></code></pre>
</blockquote>
<p>foo</p>
<pre><code></code></pre>
```




Note that in the following case, we have a [lazy
continuation line](012-continuation-12.md#lazy-continuation-line):

<a id="example-238"></a>

[Example 238](010-4-8-paragraphs.md#example-238)Try It



```
> foo
    - bar
```






```
<blockquote>
<p>foo
- bar</p>
</blockquote>
```




To see why, note that in



```
> foo
>     - bar
```




the `- bar` is indented too far to start a list, and can’t
be an indented code block because indented code blocks cannot
interrupt paragraphs, so it is [paragraph continuation text](010-4-8-paragraphs.md#paragraph-continuation-text).

A block quote can be empty:

<a id="example-239"></a>

[Example 239](010-4-8-paragraphs.md#example-239)Try It



```
>
```






```
<blockquote>
</blockquote>
```




<a id="example-240"></a>

[Example 240](010-4-8-paragraphs.md#example-240)Try It



```
>
>  
> 
```






```
<blockquote>
</blockquote>
```




A block quote can have initial or final blank lines:

<a id="example-241"></a>

[Example 241](010-4-8-paragraphs.md#example-241)Try It



```
>
> foo
>  
```






```
<blockquote>
<p>foo</p>
</blockquote>
```




A blank line always separates block quotes:

<a id="example-242"></a>

[Example 242](010-4-8-paragraphs.md#example-242)Try It



```
> foo

> bar
```






```
<blockquote>
<p>foo</p>
</blockquote>
<blockquote>
<p>bar</p>
</blockquote>
```




(Most current Markdown implementations, including John Gruber’s
original `Markdown.pl`, will parse this example as a single block quote
with two paragraphs. But it seems better to allow the author to decide
whether two block quotes or one are wanted.)

Consecutiveness means that if we put these block quotes together,
we get a single block quote:

<a id="example-243"></a>

[Example 243](010-4-8-paragraphs.md#example-243)Try It



```
> foo
> bar
```






```
<blockquote>
<p>foo
bar</p>
</blockquote>
```




To get a block quote with two paragraphs, use:

<a id="example-244"></a>

[Example 244](010-4-8-paragraphs.md#example-244)Try It



```
> foo
>
> bar
```






```
<blockquote>
<p>foo</p>
<p>bar</p>
</blockquote>
```




Block quotes can interrupt paragraphs:

<a id="example-245"></a>

[Example 245](010-4-8-paragraphs.md#example-245)Try It



```
foo
> bar
```






```
<p>foo</p>
<blockquote>
<p>bar</p>
</blockquote>
```




In general, blank lines are not needed before or after block
quotes:

<a id="example-246"></a>

[Example 246](010-4-8-paragraphs.md#example-246)Try It



```
> aaa
***
> bbb
```






```
<blockquote>
<p>aaa</p>
</blockquote>
<hr />
<blockquote>
<p>bbb</p>
</blockquote>
```




However, because of laziness, a blank line is needed between
a block quote and a following paragraph:

<a id="example-247"></a>

[Example 247](010-4-8-paragraphs.md#example-247)Try It



```
> bar
baz
```






```
<blockquote>
<p>bar
baz</p>
</blockquote>
```




<a id="example-248"></a>

[Example 248](010-4-8-paragraphs.md#example-248)Try It



```
> bar

baz
```






```
<blockquote>
<p>bar</p>
</blockquote>
<p>baz</p>
```




<a id="example-249"></a>

[Example 249](010-4-8-paragraphs.md#example-249)Try It



```
> bar
>
baz
```






```
<blockquote>
<p>bar</p>
</blockquote>
<p>baz</p>
```




