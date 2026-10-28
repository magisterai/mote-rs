# 4.4 Indented code blocks

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-4-2-atx-headings.md) · [Next](006-4-6-html-blocks.md)

<a id="indented-code-blocks"></a>

## 4.4 Indented code blocks

<a id="indented-code-block"></a>

<a id="indented-chunk"></a>

An [indented code block](005-4-4-indented-code-blocks.md#indented-code-block) is composed of one or more
[indented chunks](005-4-4-indented-code-blocks.md#indented-chunk) separated by blank lines.
An [indented chunk](005-4-4-indented-code-blocks.md#indented-chunk) is a sequence of non-blank lines,
each indented four or more spaces. The contents of the code block are
the literal contents of the lines, including trailing
[line endings](002-1-3-why-is-a-spec-needed.md#line-ending), minus four spaces of indentation.
An indented code block has no [info string](005-4-4-indented-code-blocks.md#info-string).

An indented code block cannot interrupt a paragraph, so there must be
a blank line between a paragraph and a following indented code block.
(A blank line is not needed, however, between a code block and a following
paragraph.)

<a id="example-77"></a>

[Example 77](005-4-4-indented-code-blocks.md#example-77)



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
item](010-5-2-list-items.md#list-items), the list item interpretation takes precedence:

<a id="example-78"></a>

[Example 78](005-4-4-indented-code-blocks.md#example-78)



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




<a id="example-79"></a>

[Example 79](005-4-4-indented-code-blocks.md#example-79)



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

<a id="example-80"></a>

[Example 80](005-4-4-indented-code-blocks.md#example-80)



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

<a id="example-81"></a>

[Example 81](005-4-4-indented-code-blocks.md#example-81)



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




Any initial spaces beyond four will be included in the content, even
in interior blank lines:

<a id="example-82"></a>

[Example 82](005-4-4-indented-code-blocks.md#example-82)



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

<a id="example-83"></a>

[Example 83](005-4-4-indented-code-blocks.md#example-83)



```
Foo
    bar
```






```
<p>Foo
bar</p>
```




However, any non-blank line with fewer than four leading spaces ends
the code block immediately. So a paragraph may occur immediately
after indented code:

<a id="example-84"></a>

[Example 84](005-4-4-indented-code-blocks.md#example-84)



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

<a id="example-85"></a>

[Example 85](005-4-4-indented-code-blocks.md#example-85)



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




The first line can be indented more than four spaces:

<a id="example-86"></a>

[Example 86](005-4-4-indented-code-blocks.md#example-86)



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

<a id="example-87"></a>

[Example 87](005-4-4-indented-code-blocks.md#example-87)



```

    
    foo
    
```






```
<pre><code>foo
</code></pre>
```




Trailing spaces are included in the code block’s content:

<a id="example-88"></a>

[Example 88](005-4-4-indented-code-blocks.md#example-88)



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

A [code fence](005-4-4-indented-code-blocks.md#code-fence) is a sequence
of at least three consecutive backtick characters (`` ` ``) or
tildes (`~`). (Tildes and backticks cannot be mixed.)
A [fenced code block](005-4-4-indented-code-blocks.md#fenced-code-block)
begins with a code fence, indented no more than three spaces.

<a id="info-string"></a>

The line with the opening code fence may optionally contain some text
following the code fence; this is trimmed of leading and trailing
whitespace and called the [info string](005-4-4-indented-code-blocks.md#info-string). If the [info string](005-4-4-indented-code-blocks.md#info-string) comes
after a backtick fence, it may not contain any backtick
characters. (The reason for this restriction is that otherwise
some inline code would be incorrectly interpreted as the
beginning of a fenced code block.)

The content of the code block consists of all subsequent lines, until
a closing [code fence](005-4-4-indented-code-blocks.md#code-fence) of the same type as the code block
began with (backticks or tildes), and with at least as many backticks
or tildes as the opening code fence. If the leading code fence is
indented N spaces, then up to N spaces of indentation are removed from
each line of the content (if present). (If a content line is not
indented, it is preserved unchanged. If it is indented less than N
spaces, all of the indentation is removed.)

The closing code fence may be indented up to three spaces, and may be
followed only by spaces, which are ignored. If the end of the
containing block (or document) is reached and no closing code fence
has been found, the code block contains all of the lines after the
opening code fence until the end of the containing block (or
document). (An alternative spec would require backtracking in the
event that a closing code fence is not found. But this makes parsing
much less efficient, and there seems to be no real down side to the
behavior described here.)

A fenced code block may interrupt a paragraph, and does not require
a blank line either before or after.

The content of a code fence is treated as literal text, not parsed
as inlines. The first word of the [info string](005-4-4-indented-code-blocks.md#info-string) is typically used to
specify the language of the code sample, and rendered in the `class`
attribute of the `code` tag. However, this spec does not mandate any
particular treatment of the [info string](005-4-4-indented-code-blocks.md#info-string).

Here is a simple example with backticks:

<a id="example-89"></a>

[Example 89](005-4-4-indented-code-blocks.md#example-89)



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

<a id="example-90"></a>

[Example 90](005-4-4-indented-code-blocks.md#example-90)



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

<a id="example-91"></a>

[Example 91](005-4-4-indented-code-blocks.md#example-91)



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

<a id="example-92"></a>

[Example 92](005-4-4-indented-code-blocks.md#example-92)



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




<a id="example-93"></a>

[Example 93](005-4-4-indented-code-blocks.md#example-93)



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

<a id="example-94"></a>

[Example 94](005-4-4-indented-code-blocks.md#example-94)



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




<a id="example-95"></a>

[Example 95](005-4-4-indented-code-blocks.md#example-95)



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
(or the enclosing [block quote](009-5-container-blocks.md#block-quotes) or [list item](010-5-2-list-items.md#list-items)):

<a id="example-96"></a>

[Example 96](005-4-4-indented-code-blocks.md#example-96)



````
```
````






```
<pre><code></code></pre>
```




<a id="example-97"></a>

[Example 97](005-4-4-indented-code-blocks.md#example-97)



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




<a id="example-98"></a>

[Example 98](005-4-4-indented-code-blocks.md#example-98)



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

<a id="example-99"></a>

[Example 99](005-4-4-indented-code-blocks.md#example-99)



````
```

  
```
````






```
<pre><code>
  
</code></pre>
```




A code block can be empty:

<a id="example-100"></a>

[Example 100](005-4-4-indented-code-blocks.md#example-100)



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

<a id="example-101"></a>

[Example 101](005-4-4-indented-code-blocks.md#example-101)



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




<a id="example-102"></a>

[Example 102](005-4-4-indented-code-blocks.md#example-102)



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




<a id="example-103"></a>

[Example 103](005-4-4-indented-code-blocks.md#example-103)



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




Four spaces indentation produces an indented code block:

<a id="example-104"></a>

[Example 104](005-4-4-indented-code-blocks.md#example-104)



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




Closing fences may be indented by 0-3 spaces, and their indentation
need not match that of the opening fence:

<a id="example-105"></a>

[Example 105](005-4-4-indented-code-blocks.md#example-105)



````
```
aaa
  ```
````






```
<pre><code>aaa
</code></pre>
```




<a id="example-106"></a>

[Example 106](005-4-4-indented-code-blocks.md#example-106)



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

<a id="example-107"></a>

[Example 107](005-4-4-indented-code-blocks.md#example-107)



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




Code fences (opening and closing) cannot contain internal spaces:

<a id="example-108"></a>

[Example 108](005-4-4-indented-code-blocks.md#example-108)



````
``` ```
aaa
````






```
<p><code> </code>
aaa</p>
```




<a id="example-109"></a>

[Example 109](005-4-4-indented-code-blocks.md#example-109)



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

<a id="example-110"></a>

[Example 110](005-4-4-indented-code-blocks.md#example-110)



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

<a id="example-111"></a>

[Example 111](005-4-4-indented-code-blocks.md#example-111)



