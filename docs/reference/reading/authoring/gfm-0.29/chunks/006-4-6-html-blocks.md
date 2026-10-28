# 4.6 HTML blocks

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-4-4-indented-code-blocks.md) · [Next](007-4-7-link-reference-definitions.md)

```
foo
---
~~~
bar
~~~
# baz
```






```
<h2>foo</h2>
<pre><code>bar
</code></pre>
<h1>baz</h1>
```




An [info string](005-4-4-indented-code-blocks.md#info-string) can be provided after the opening code fence.
Although this spec doesn’t mandate any particular treatment of
the info string, the first word is typically used to specify
the language of the code block. In HTML output, the language is
normally indicated by adding a class to the `code` element consisting
of `language-` followed by the language name.

<a id="example-112"></a>

[Example 112](006-4-6-html-blocks.md#example-112)



````
```ruby
def foo(x)
  return 3
end
```
````






```
<pre><code class="language-ruby">def foo(x)
  return 3
end
</code></pre>
```




<a id="example-113"></a>

[Example 113](006-4-6-html-blocks.md#example-113)



```
~~~~    ruby startline=3 $%@#$
def foo(x)
  return 3
end
~~~~~~~
```






```
<pre><code class="language-ruby">def foo(x)
  return 3
end
</code></pre>
```




<a id="example-114"></a>

[Example 114](006-4-6-html-blocks.md#example-114)



`````
````;
````
`````






```
<pre><code class="language-;"></code></pre>
```




[Info strings](005-4-4-indented-code-blocks.md#info-string) for backtick code blocks cannot contain backticks:

<a id="example-115"></a>

[Example 115](006-4-6-html-blocks.md#example-115)



````
``` aa ```
foo
````






```
<p><code>aa</code>
foo</p>
```




[Info strings](005-4-4-indented-code-blocks.md#info-string) for tilde code blocks can contain backticks and tildes:

<a id="example-116"></a>

[Example 116](006-4-6-html-blocks.md#example-116)



````
~~~ aa ``` ~~~
foo
~~~
````






```
<pre><code class="language-aa">foo
</code></pre>
```




Closing code fences cannot have [info strings](005-4-4-indented-code-blocks.md#info-string):

<a id="example-117"></a>

[Example 117](006-4-6-html-blocks.md#example-117)



````
```
``` aaa
```
````






````
<pre><code>``` aaa
</code></pre>
````




<a id="html-blocks"></a>

## 4.6 HTML blocks

<a id="html-block"></a>

An [HTML block](006-4-6-html-blocks.md#html-block) is a group of lines that is treated
as raw HTML (and will not be escaped in HTML output).

<a id="start-condition"></a>

<a id="end-condition"></a>

There are seven kinds of [HTML block](006-4-6-html-blocks.md#html-block), which can be defined by their
start and end conditions. The block begins with a line that meets a
[start condition](006-4-6-html-blocks.md#start-condition) (after up to three spaces optional indentation).
It ends with the first subsequent line that meets a matching [end
condition](006-4-6-html-blocks.md#end-condition), or the last line of the document, or the last line of
the [container block](003-2-2-tabs.md#container-blocks) containing the current HTML
block, if no line is encountered that meets the [end condition](006-4-6-html-blocks.md#end-condition). If
the first line meets both the [start condition](006-4-6-html-blocks.md#start-condition) and the [end
condition](006-4-6-html-blocks.md#end-condition), the block will contain just that line.

1. **Start condition:** line begins with the string `<script`,
   `<pre`, or `<style` (case-insensitive), followed by whitespace,
   the string `>`, or the end of the line.  
   **End condition:** line contains an end tag
   `</script>`, `</pre>`, or `</style>` (case-insensitive; it
   need not match the start tag).
2. **Start condition:** line begins with the string `<!--`.  
   **End condition:** line contains the string `-->`.
3. **Start condition:** line begins with the string `<?`.  
   **End condition:** line contains the string `?>`.
4. **Start condition:** line begins with the string `<!`
   followed by an uppercase ASCII letter.  
   **End condition:** line contains the character `>`.
5. **Start condition:** line begins with the string
   `<![CDATA[`.  
   **End condition:** line contains the string `]]>`.
6. **Start condition:** line begins the string `<` or `</`
   followed by one of the strings (case-insensitive) `address`,
   `article`, `aside`, `base`, `basefont`, `blockquote`, `body`,
   `caption`, `center`, `col`, `colgroup`, `dd`, `details`, `dialog`,
   `dir`, `div`, `dl`, `dt`, `fieldset`, `figcaption`, `figure`,
   `footer`, `form`, `frame`, `frameset`,
   `h1`, `h2`, `h3`, `h4`, `h5`, `h6`, `head`, `header`, `hr`,
   `html`, `iframe`, `legend`, `li`, `link`, `main`, `menu`, `menuitem`,
   `nav`, `noframes`, `ol`, `optgroup`, `option`, `p`, `param`,
   `section`, `source`, `summary`, `table`, `tbody`, `td`,
   `tfoot`, `th`, `thead`, `title`, `tr`, `track`, `ul`, followed
   by [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), the end of the line, the string `>`, or
   the string `/>`.  
   **End condition:** line is followed by a [blank line](002-1-3-why-is-a-spec-needed.md#blank-line).
7. **Start condition:** line begins with a complete [open tag](021-6-10-raw-html.md#open-tag)
   (with any [tag name](021-6-10-raw-html.md#tag-name) other than `script`,
   `style`, or `pre`) or a complete [closing tag](021-6-10-raw-html.md#closing-tag),
   followed only by [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) or the end of the line.  
   **End condition:** line is followed by a [blank line](002-1-3-why-is-a-spec-needed.md#blank-line).

HTML blocks continue until they are closed by their appropriate
[end condition](006-4-6-html-blocks.md#end-condition), or the last line of the document or other [container
block](003-2-2-tabs.md#container-blocks). This means any HTML **within an HTML
block** that might otherwise be recognised as a start condition will
be ignored by the parser and passed through as-is, without changing
the parser’s state.

For instance, `<pre>` within a HTML block started by `<table>` will not affect
the parser state; as the HTML block was started in by start condition 6, it
will end at any blank line. This can be surprising:

<a id="example-118"></a>

[Example 118](006-4-6-html-blocks.md#example-118)



```
<table><tr><td>
<pre>
**Hello**,

_world_.
</pre>
</td></tr></table>
```






```
<table><tr><td>
<pre>
**Hello**,
<p><em>world</em>.
</pre></p>
</td></tr></table>
```




In this case, the HTML block is terminated by the newline — the `**Hello**`
text remains verbatim — and regular parsing resumes, with a paragraph,
emphasised `world` and inline and block HTML following.

All types of [HTML blocks](006-4-6-html-blocks.md#html-blocks) except type 7 may interrupt
a paragraph. Blocks of type 7 may not interrupt a paragraph.
(This restriction is intended to prevent unwanted interpretation
of long tags inside a wrapped paragraph as starting HTML blocks.)

Some simple examples follow. Here are some basic HTML blocks
of type 6:

<a id="example-119"></a>

[Example 119](006-4-6-html-blocks.md#example-119)



```
<table>
  <tr>
    <td>
           hi
    </td>
  </tr>
</table>

okay.
```






```
<table>
  <tr>
    <td>
           hi
    </td>
  </tr>
</table>
<p>okay.</p>
```




<a id="example-120"></a>

[Example 120](006-4-6-html-blocks.md#example-120)



```
 <div>
  *hello*
         <foo><a>
```






```
 <div>
  *hello*
         <foo><a>
```




A block can also start with a closing tag:

<a id="example-121"></a>

[Example 121](006-4-6-html-blocks.md#example-121)



```
</div>
*foo*
```






```
</div>
*foo*
```




Here we have two HTML blocks with a Markdown paragraph between them:

<a id="example-122"></a>

[Example 122](006-4-6-html-blocks.md#example-122)



```
<DIV CLASS="foo">

*Markdown*

</DIV>
```






```
<DIV CLASS="foo">
<p><em>Markdown</em></p>
</DIV>
```




The tag on the first line can be partial, as long
as it is split where there would be whitespace:

<a id="example-123"></a>

[Example 123](006-4-6-html-blocks.md#example-123)



```
<div id="foo"
  class="bar">
</div>
```






```
<div id="foo"
  class="bar">
</div>
```




<a id="example-124"></a>

[Example 124](006-4-6-html-blocks.md#example-124)



```
<div id="foo" class="bar
  baz">
</div>
```






```
<div id="foo" class="bar
  baz">
</div>
```




An open tag need not be closed:

<a id="example-125"></a>

[Example 125](006-4-6-html-blocks.md#example-125)



```
<div>
*foo*

*bar*
```






```
<div>
*foo*
<p><em>bar</em></p>
```




A partial tag need not even be completed (garbage
in, garbage out):

<a id="example-126"></a>

[Example 126](006-4-6-html-blocks.md#example-126)



```
<div id="foo"
*hi*
```






```
<div id="foo"
*hi*
```




<a id="example-127"></a>

[Example 127](006-4-6-html-blocks.md#example-127)



```
<div class
foo
```






```
<div class
foo
```




The initial tag doesn’t even need to be a valid
tag, as long as it starts like one:

<a id="example-128"></a>

[Example 128](006-4-6-html-blocks.md#example-128)



```
<div *???-&&&-<---
*foo*
```






```
<div *???-&&&-<---
*foo*
```




In type 6 blocks, the initial tag need not be on a line by
itself:

<a id="example-129"></a>

[Example 129](006-4-6-html-blocks.md#example-129)



```
<div><a href="bar">*foo*</a></div>
```






```
<div><a href="bar">*foo*</a></div>
```




<a id="example-130"></a>

[Example 130](006-4-6-html-blocks.md#example-130)



```
<table><tr><td>
foo
</td></tr></table>
```






```
<table><tr><td>
foo
</td></tr></table>
```




Everything until the next blank line or end of document
gets included in the HTML block. So, in the following
example, what looks like a Markdown code block
is actually part of the HTML block, which continues until a blank
line or the end of the document is reached:

<a id="example-131"></a>

[Example 131](006-4-6-html-blocks.md#example-131)



````
<div></div>
``` c
int x = 33;
```
````






````
<div></div>
``` c
int x = 33;
```
````




To start an [HTML block](006-4-6-html-blocks.md#html-block) with a tag that is *not* in the
list of block-level tags in (6), you must put the tag by
itself on the first line (and it must be complete):

<a id="example-132"></a>

[Example 132](006-4-6-html-blocks.md#example-132)



```
<a href="foo">
*bar*
</a>
```






```
<a href="foo">
*bar*
</a>
```




In type 7 blocks, the [tag name](021-6-10-raw-html.md#tag-name) can be anything:

<a id="example-133"></a>

[Example 133](006-4-6-html-blocks.md#example-133)



```
<Warning>
*bar*
</Warning>
```






```
<Warning>
*bar*
</Warning>
```




<a id="example-134"></a>

[Example 134](006-4-6-html-blocks.md#example-134)



```
<i class="foo">
*bar*
</i>
```






```
<i class="foo">
*bar*
</i>
```




<a id="example-135"></a>

[Example 135](006-4-6-html-blocks.md#example-135)



```
</ins>
*bar*
```






```
</ins>
*bar*
```




These rules are designed to allow us to work with tags that
can function as either block-level or inline-level tags.
The `<del>` tag is a nice example. We can surround content with
`<del>` tags in three different ways. In this case, we get a raw
HTML block, because the `<del>` tag is on a line by itself:

<a id="example-136"></a>

[Example 136](006-4-6-html-blocks.md#example-136)



```
<del>
*foo*
</del>
```






```
<del>
*foo*
</del>
```




In this case, we get a raw HTML block that just includes
the `<del>` tag (because it ends with the following blank
line). So the contents get interpreted as CommonMark:

<a id="example-137"></a>

[Example 137](006-4-6-html-blocks.md#example-137)



```
<del>

*foo*

</del>
```






```
<del>
<p><em>foo</em></p>
</del>
```




Finally, in this case, the `<del>` tags are interpreted
as [raw HTML](021-6-10-raw-html.md#raw-html) *inside* the CommonMark paragraph. (Because
the tag is not on a line by itself, we get inline HTML
rather than an [HTML block](006-4-6-html-blocks.md#html-block).)

<a id="example-138"></a>

[Example 138](006-4-6-html-blocks.md#example-138)



```
<del>*foo*</del>
```






```
<p><del><em>foo</em></del></p>
```




HTML tags designed to contain literal content
(`script`, `style`, `pre`), comments, processing instructions,
and declarations are treated somewhat differently.
Instead of ending at the first blank line, these blocks
end at the first line containing a corresponding end tag.
As a result, these blocks can contain blank lines:

A pre tag (type 1):

<a id="example-139"></a>

[Example 139](006-4-6-html-blocks.md#example-139)



```
<pre language="haskell"><code>
import Text.HTML.TagSoup

main :: IO ()
main = print $ parseTags tags
</code></pre>
okay
```






```
<pre language="haskell"><code>
import Text.HTML.TagSoup

main :: IO ()
main = print $ parseTags tags
</code></pre>
<p>okay</p>
```




A script tag (type 1):

<a id="example-140"></a>

[Example 140](006-4-6-html-blocks.md#example-140)



```
<script type="text/javascript">
// JavaScript example

document.getElementById("demo").innerHTML = "Hello JavaScript!";
</script>
okay
```






```
<script type="text/javascript">
// JavaScript example

document.getElementById("demo").innerHTML = "Hello JavaScript!";
</script>
<p>okay</p>
```




A style tag (type 1):

<a id="example-141"></a>

[Example 141](006-4-6-html-blocks.md#example-141)



```
<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
okay
```






```
<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
<p>okay</p>
```




If there is no matching end tag, the block will end at the
end of the document (or the enclosing [block quote](009-5-container-blocks.md#block-quotes)
or [list item](010-5-2-list-items.md#list-items)):

<a id="example-142"></a>

[Example 142](006-4-6-html-blocks.md#example-142)



```
<style
  type="text/css">

foo
```






```
<style
  type="text/css">

foo
```




