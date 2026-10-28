# 4.4 Indented code blocks

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-4-3-setext-headings.md) · [Next](007-4-6-html-blocks.md)

<a id="indented-code-blocks"></a>

## 4.4 Indented code blocks

<a id="indented-code-block"></a>

<a id="indented-chunk"></a>

An [indented code block](006-4-4-indented-code-blocks.md#indented-code-block) is composed of one or more
[indented chunks](006-4-4-indented-code-blocks.md#indented-chunk) separated by blank lines.
An [indented chunk](006-4-4-indented-code-blocks.md#indented-chunk) is a sequence of non-blank lines,
each preceded by four or more spaces of indentation. The contents of the code
block are the literal contents of the lines, including trailing
[line endings](002-1-3-about-this-document.md#line-ending), minus four spaces of indentation.
An indented code block has no [info string](006-4-4-indented-code-blocks.md#info-string).

An indented code block cannot interrupt a paragraph, so there must be
a blank line between a paragraph and a following indented code block.
(A blank line is not needed, however, between a code block and a following
paragraph.)

<a id="example-107"></a>

[Example 107](006-4-4-indented-code-blocks.md#example-107)Try It



```
    a simple
      indented code block
```






```
<pre><code>a simple
  indented code block
</code></pre>
```




If there is any ambiguity between an interpretation of indentation
as a code block and as indicating that material belongs to a [list
item](011-5-2-list-items.md#list-items), the list item interpretation takes precedence:

<a id="example-108"></a>

[Example 108](006-4-4-indented-code-blocks.md#example-108)Try It



```
  - foo

    bar
```






```
<ul>
<li>
<p>foo</p>
<p>bar</p>
</li>
</ul>
```




<a id="example-109"></a>

[Example 109](006-4-4-indented-code-blocks.md#example-109)Try It



```
1.  foo

    - bar
```






```
<ol>
<li>
<p>foo</p>
<ul>
<li>bar</li>
</ul>
</li>
</ol>
```




The contents of a code block are literal text, and do not get parsed
as Markdown:

<a id="example-110"></a>

[Example 110](006-4-4-indented-code-blocks.md#example-110)Try It



```
    <a/>
    *hi*

    - one
```






```
<pre><code>&lt;a/&gt;
*hi*

- one
</code></pre>
```




Here we have three chunks separated by blank lines:

<a id="example-111"></a>

[Example 111](006-4-4-indented-code-blocks.md#example-111)Try It



```
    chunk1

    chunk2
  
 
 
    chunk3
```






```
<pre><code>chunk1

chunk2



chunk3
</code></pre>
```




Any initial spaces or tabs beyond four spaces of indentation will be included in
the content, even in interior blank lines:

<a id="example-112"></a>

[Example 112](006-4-4-indented-code-blocks.md#example-112)Try It



```
    chunk1
      
      chunk2
```






```
<pre><code>chunk1
  
  chunk2
</code></pre>
```




An indented code block cannot interrupt a paragraph. (This
allows hanging indents and the like.)

<a id="example-113"></a>

[Example 113](006-4-4-indented-code-blocks.md#example-113)Try It



```
Foo
    bar
```






```
<p>Foo
bar</p>
```




However, any non-blank line with fewer than four spaces of indentation ends
the code block immediately. So a paragraph may occur immediately
after indented code:

<a id="example-114"></a>

[Example 114](006-4-4-indented-code-blocks.md#example-114)Try It



```
    foo
bar
```






```
<pre><code>foo
</code></pre>
<p>bar</p>
```




And indented code can occur immediately before and after other kinds of
blocks:

<a id="example-115"></a>

[Example 115](006-4-4-indented-code-blocks.md#example-115)Try It



```
# Heading
    foo
Heading
------
    foo
----
```






```
<h1>Heading</h1>
<pre><code>foo
</code></pre>
<h2>Heading</h2>
<pre><code>foo
</code></pre>
<hr />
```




The first line can be preceded by more than four spaces of indentation:

<a id="example-116"></a>

[Example 116](006-4-4-indented-code-blocks.md#example-116)Try It



```
        foo
    bar
```






```
<pre><code>    foo
bar
</code></pre>
```




Blank lines preceding or following an indented code block
are not included in it:

<a id="example-117"></a>

[Example 117](006-4-4-indented-code-blocks.md#example-117)Try It



```

    
    foo
    
```






```
<pre><code>foo
</code></pre>
```




Trailing spaces or tabs are included in the code block’s content:

<a id="example-118"></a>

[Example 118](006-4-4-indented-code-blocks.md#example-118)Try It



```
    foo  
```






```
<pre><code>foo  
</code></pre>
```




<a id="fenced-code-blocks"></a>

## 4.5 Fenced code blocks

<a id="code-fence"></a>

<a id="fenced-code-block"></a>

A [code fence](006-4-4-indented-code-blocks.md#code-fence) is a sequence
of at least three consecutive backtick characters (`` ` ``) or
tildes (`~`). (Tildes and backticks cannot be mixed.)
A [fenced code block](006-4-4-indented-code-blocks.md#fenced-code-block)
begins with a code fence, preceded by up to three spaces of indentation.

<a id="info-string"></a>

The line with the opening code fence may optionally contain some text
following the code fence; this is trimmed of leading and trailing
spaces or tabs and called the [info string](006-4-4-indented-code-blocks.md#info-string). If the [info string](006-4-4-indented-code-blocks.md#info-string) comes
after a backtick fence, it may not contain any backtick
characters. (The reason for this restriction is that otherwise
some inline code would be incorrectly interpreted as the
beginning of a fenced code block.)

The content of the code block consists of all subsequent lines, until
a closing [code fence](006-4-4-indented-code-blocks.md#code-fence) of the same type as the code block
began with (backticks or tildes), and with at least as many backticks
or tildes as the opening code fence. If the leading code fence is
preceded by N spaces of indentation, then up to N spaces of indentation are
removed from each line of the content (if present). (If a content line is not
indented, it is preserved unchanged. If it is indented N spaces or less, all
of the indentation is removed.)

The closing code fence may be preceded by up to three spaces of indentation, and
may be followed only by spaces or tabs, which are ignored. If the end of the
containing block (or document) is reached and no closing code fence
has been found, the code block contains all of the lines after the
opening code fence until the end of the containing block (or
document). (An alternative spec would require backtracking in the
event that a closing code fence is not found. But this makes parsing
much less efficient, and there seems to be no real downside to the
behavior described here.)

A fenced code block may interrupt a paragraph, and does not require
a blank line either before or after.

The content of a code fence is treated as literal text, not parsed
as inlines. The first word of the [info string](006-4-4-indented-code-blocks.md#info-string) is typically used to
specify the language of the code sample, and rendered in the `class`
attribute of the `code` tag. However, this spec does not mandate any
particular treatment of the [info string](006-4-4-indented-code-blocks.md#info-string).

Here is a simple example with backticks:

<a id="example-119"></a>

[Example 119](006-4-4-indented-code-blocks.md#example-119)Try It



````
```
<
 >
```
````






```
<pre><code>&lt;
 &gt;
</code></pre>
```




With tildes:

<a id="example-120"></a>

[Example 120](006-4-4-indented-code-blocks.md#example-120)Try It



```
~~~
<
 >
~~~
```






```
<pre><code>&lt;
 &gt;
</code></pre>
```




Fewer than three backticks is not enough:

<a id="example-121"></a>

[Example 121](006-4-4-indented-code-blocks.md#example-121)Try It



```
``
foo
``
```






```
<p><code>foo</code></p>
```




The closing code fence must use the same character as the opening
fence:

<a id="example-122"></a>

[Example 122](006-4-4-indented-code-blocks.md#example-122)Try It



````
```
aaa
~~~
```
````






```
<pre><code>aaa
~~~
</code></pre>
```




<a id="example-123"></a>

[Example 123](006-4-4-indented-code-blocks.md#example-123)Try It



````
~~~
aaa
```
~~~
````






````
<pre><code>aaa
```
</code></pre>
````




The closing code fence must be at least as long as the opening fence:

<a id="example-124"></a>

[Example 124](006-4-4-indented-code-blocks.md#example-124)Try It



```````
````
aaa
```
``````
```````






````
<pre><code>aaa
```
</code></pre>
````




<a id="example-125"></a>

[Example 125](006-4-4-indented-code-blocks.md#example-125)Try It



```
~~~~
aaa
~~~
~~~~
```






```
<pre><code>aaa
~~~
</code></pre>
```




Unclosed code blocks are closed by the end of the document
(or the enclosing [block quote](010-4-8-paragraphs.md#block-quotes) or [list item](011-5-2-list-items.md#list-items)):

<a id="example-126"></a>

[Example 126](006-4-4-indented-code-blocks.md#example-126)Try It



````
```
````






```
<pre><code></code></pre>
```




<a id="example-127"></a>

[Example 127](006-4-4-indented-code-blocks.md#example-127)Try It



``````
`````

```
aaa
``````






````
<pre><code>
```
aaa
</code></pre>
````




<a id="example-128"></a>

[Example 128](006-4-4-indented-code-blocks.md#example-128)Try It



````
> ```
> aaa

bbb
````






```
<blockquote>
<pre><code>aaa
</code></pre>
</blockquote>
<p>bbb</p>
```




A code block can have all empty lines as its content:

<a id="example-129"></a>

[Example 129](006-4-4-indented-code-blocks.md#example-129)Try It



````
```

  
```
````






```
<pre><code>
  
</code></pre>
```




A code block can be empty:

<a id="example-130"></a>

[Example 130](006-4-4-indented-code-blocks.md#example-130)Try It



````
```
```
````






```
<pre><code></code></pre>
```




Fences can be indented. If the opening fence is indented,
content lines will have equivalent opening indentation removed,
if present:

<a id="example-131"></a>

[Example 131](006-4-4-indented-code-blocks.md#example-131)Try It



````
 ```
 aaa
aaa
```
````






```
<pre><code>aaa
aaa
</code></pre>
```




<a id="example-132"></a>

[Example 132](006-4-4-indented-code-blocks.md#example-132)Try It



````
  ```
aaa
  aaa
aaa
  ```
````






```
<pre><code>aaa
aaa
aaa
</code></pre>
```




<a id="example-133"></a>

[Example 133](006-4-4-indented-code-blocks.md#example-133)Try It



````
   ```
   aaa
    aaa
  aaa
   ```
````






```
<pre><code>aaa
 aaa
aaa
</code></pre>
```




Four spaces of indentation is too many:

<a id="example-134"></a>

[Example 134](006-4-4-indented-code-blocks.md#example-134)Try It



````
    ```
    aaa
    ```
````






````
<pre><code>```
aaa
```
</code></pre>
````




Closing fences may be preceded by up to three spaces of indentation, and their
indentation need not match that of the opening fence:

<a id="example-135"></a>

[Example 135](006-4-4-indented-code-blocks.md#example-135)Try It



````
```
aaa
  ```
````






```
<pre><code>aaa
</code></pre>
```




<a id="example-136"></a>

[Example 136](006-4-4-indented-code-blocks.md#example-136)Try It



````
   ```
aaa
  ```
````






```
<pre><code>aaa
</code></pre>
```




This is not a closing fence, because it is indented 4 spaces:

<a id="example-137"></a>

[Example 137](006-4-4-indented-code-blocks.md#example-137)Try It



````
```
aaa
    ```
````






````
<pre><code>aaa
    ```
</code></pre>
````




Code fences (opening and closing) cannot contain internal spaces or tabs:

<a id="example-138"></a>

[Example 138](006-4-4-indented-code-blocks.md#example-138)Try It



````
``` ```
aaa
````






```
<p><code> </code>
aaa</p>
```




<a id="example-139"></a>

[Example 139](006-4-4-indented-code-blocks.md#example-139)Try It



```
~~~~~~
aaa
~~~ ~~
```






```
<pre><code>aaa
~~~ ~~
</code></pre>
```




Fenced code blocks can interrupt paragraphs, and can be followed
directly by paragraphs, without a blank line between:

<a id="example-140"></a>

[Example 140](006-4-4-indented-code-blocks.md#example-140)Try It



````
foo
```
bar
```
baz
````






```
<p>foo</p>
<pre><code>bar
</code></pre>
<p>baz</p>
```




Other blocks can also occur before and after fenced code blocks
without an intervening blank line:

<a id="example-141"></a>

[Example 141](006-4-4-indented-code-blocks.md#example-141)Try It



