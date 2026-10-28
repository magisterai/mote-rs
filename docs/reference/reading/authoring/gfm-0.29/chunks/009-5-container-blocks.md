# 5 Container blocks

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-4-8-paragraphs.md) · [Next](010-5-2-list-items.md)

# 5 Container blocks

A [container block](003-2-2-tabs.md#container-blocks) is a block that has other
blocks as its contents. There are two basic kinds of container blocks:
[block quotes](009-5-container-blocks.md#block-quotes) and [list items](010-5-2-list-items.md#list-items).
[Lists](012-5-3-task-list-items-extension.md#list) are meta-containers for [list items](010-5-2-list-items.md#list-items).

We define the syntax for container blocks recursively. The general
form of the definition is:

> If X is a sequence of blocks, then the result of
> transforming X in such-and-such a way is a container of type Y
> with these blocks as its content.

So, we explain what counts as a block quote or list item by explaining
how these can be *generated* from their contents. This should suffice
to define the syntax, although it does not give a recipe for *parsing*
these constructions. (A recipe is provided below in the section entitled
[A parsing strategy](022-6-11-disallowed-raw-html-extension.md#appendix-a-parsing-strategy).)

<a id="block-quotes"></a>

## 5.1 Block quotes

<a id="block-quote-marker"></a>

A [block quote marker](009-5-container-blocks.md#block-quote-marker)
consists of 0-3 spaces of initial indent, plus (a) the character `>` together
with a following space, or (b) a single character `>` not followed by a space.

The following rules define [block quotes](009-5-container-blocks.md#block-quotes):

1. **Basic case.** If a string of lines *Ls* constitute a sequence
   of blocks *Bs*, then the result of prepending a [block quote
   marker](009-5-container-blocks.md#block-quote-marker) to the beginning of each line in *Ls*
   is a [block quote](009-5-container-blocks.md#block-quotes) containing *Bs*.
2. <a id="paragraph-continuation-text"></a>

   **Laziness.** If a string of lines *Ls* constitute a [block
   quote](009-5-container-blocks.md#block-quotes) with contents *Bs*, then the result of deleting
   the initial [block quote marker](009-5-container-blocks.md#block-quote-marker) from one or
   more lines in which the next [non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character) after the [block
   quote marker](009-5-container-blocks.md#block-quote-marker) is [paragraph continuation
   text](009-5-container-blocks.md#paragraph-continuation-text) is a block quote with *Bs* as its content.
   [Paragraph continuation text](009-5-container-blocks.md#paragraph-continuation-text) is text
   that will be parsed as part of the content of a paragraph, but does
   not occur at the beginning of the paragraph.
3. **Consecutiveness.** A document cannot contain two [block
   quotes](009-5-container-blocks.md#block-quotes) in a row unless there is a [blank line](002-1-3-why-is-a-spec-needed.md#blank-line) between them.

Nothing else counts as a [block quote](009-5-container-blocks.md#block-quotes).

Here is a simple example:

<a id="example-206"></a>

[Example 206](009-5-container-blocks.md#example-206)



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




The spaces after the `>` characters can be omitted:

<a id="example-207"></a>

[Example 207](009-5-container-blocks.md#example-207)



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




The `>` characters can be indented 1-3 spaces:

<a id="example-208"></a>

[Example 208](009-5-container-blocks.md#example-208)



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




Four spaces gives us a code block:

<a id="example-209"></a>

[Example 209](009-5-container-blocks.md#example-209)



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
[paragraph continuation text](009-5-container-blocks.md#paragraph-continuation-text):

<a id="example-210"></a>

[Example 210](009-5-container-blocks.md#example-210)



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

<a id="example-211"></a>

[Example 211](009-5-container-blocks.md#example-211)



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
paragraphs had they been prepended with [block quote markers](009-5-container-blocks.md#block-quote-marker).
For example, the `>`  cannot be omitted in the second line of



```
> foo
> ---
```




without changing the meaning:

<a id="example-212"></a>

[Example 212](009-5-container-blocks.md#example-212)



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

<a id="example-213"></a>

[Example 213](009-5-container-blocks.md#example-213)



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

<a id="example-214"></a>

[Example 214](009-5-container-blocks.md#example-214)



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




<a id="example-215"></a>

[Example 215](009-5-container-blocks.md#example-215)



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
continuation line](011-5-2-1-motivation.md#lazy-continuation-line):

<a id="example-216"></a>

[Example 216](009-5-container-blocks.md#example-216)



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
interrupt paragraphs, so it is [paragraph continuation text](009-5-container-blocks.md#paragraph-continuation-text).

A block quote can be empty:

<a id="example-217"></a>

[Example 217](009-5-container-blocks.md#example-217)



```
>
```






```
<blockquote>
</blockquote>
```




<a id="example-218"></a>

[Example 218](009-5-container-blocks.md#example-218)



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

<a id="example-219"></a>

[Example 219](009-5-container-blocks.md#example-219)



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

<a id="example-220"></a>

[Example 220](009-5-container-blocks.md#example-220)



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

<a id="example-221"></a>

[Example 221](009-5-container-blocks.md#example-221)



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

<a id="example-222"></a>

[Example 222](009-5-container-blocks.md#example-222)



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

<a id="example-223"></a>

[Example 223](009-5-container-blocks.md#example-223)



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

<a id="example-224"></a>

[Example 224](009-5-container-blocks.md#example-224)



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

<a id="example-225"></a>

[Example 225](009-5-container-blocks.md#example-225)



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




<a id="example-226"></a>

[Example 226](009-5-container-blocks.md#example-226)



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




<a id="example-227"></a>

[Example 227](009-5-container-blocks.md#example-227)



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




It is a consequence of the Laziness rule that any number
of initial `>`s may be omitted on a continuation line of a
nested block quote:

<a id="example-228"></a>

[Example 228](009-5-container-blocks.md#example-228)



```
> > > foo
bar
```






```
<blockquote>
<blockquote>
<blockquote>
<p>foo
bar</p>
</blockquote>
</blockquote>
</blockquote>
```




<a id="example-229"></a>

[Example 229](009-5-container-blocks.md#example-229)



```
>>> foo
> bar
>>baz
```






```
<blockquote>
<blockquote>
<blockquote>
<p>foo
bar
baz</p>
</blockquote>
</blockquote>
</blockquote>
```




When including an indented code block in a block quote,
remember that the [block quote marker](009-5-container-blocks.md#block-quote-marker) includes
both the `>` and a following space. So *five spaces* are needed after
the `>`:

<a id="example-230"></a>

[Example 230](009-5-container-blocks.md#example-230)



```
>     code

>    not code
```






```
<blockquote>
<pre><code>code
</code></pre>
</blockquote>
<blockquote>
<p>not code</p>
</blockquote>
```




