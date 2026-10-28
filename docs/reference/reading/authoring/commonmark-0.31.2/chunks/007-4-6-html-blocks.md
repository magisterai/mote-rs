# 4.6 HTML blocks

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-4-4-indented-code-blocks.md) · [Next](008-continuation-8.md)

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




An [info string](006-4-4-indented-code-blocks.md#info-string) can be provided after the opening code fence.
Although this spec doesn’t mandate any particular treatment of
the info string, the first word is typically used to specify
the language of the code block. In HTML output, the language is
normally indicated by adding a class to the `code` element consisting
of `language-` followed by the language name.

<a id="example-142"></a>

[Example 142](007-4-6-html-blocks.md#example-142)Try It



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




<a id="example-143"></a>

[Example 143](007-4-6-html-blocks.md#example-143)Try It



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




<a id="example-144"></a>

[Example 144](007-4-6-html-blocks.md#example-144)Try It



`````
````;
````
`````






```
<pre><code class="language-;"></code></pre>
```




[Info strings](006-4-4-indented-code-blocks.md#info-string) for backtick code blocks cannot contain backticks:

<a id="example-145"></a>

[Example 145](007-4-6-html-blocks.md#example-145)Try It



````
``` aa ```
foo
````






```
<p><code>aa</code>
foo</p>
```




[Info strings](006-4-4-indented-code-blocks.md#info-string) for tilde code blocks can contain backticks and tildes:

<a id="example-146"></a>

[Example 146](007-4-6-html-blocks.md#example-146)Try It



````
~~~ aa ``` ~~~
foo
~~~
````






```
<pre><code class="language-aa">foo
</code></pre>
```




Closing code fences cannot have [info strings](006-4-4-indented-code-blocks.md#info-string):

<a id="example-147"></a>

[Example 147](007-4-6-html-blocks.md#example-147)Try It



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

An [HTML block](007-4-6-html-blocks.md#html-block) is a group of lines that is treated
as raw HTML (and will not be escaped in HTML output).

<a id="start-condition"></a>

<a id="end-condition"></a>

There are seven kinds of [HTML block](007-4-6-html-blocks.md#html-block), which can be defined by their
start and end conditions. The block begins with a line that meets a
[start condition](007-4-6-html-blocks.md#start-condition) (after up to three optional spaces of indentation).
It ends with the first subsequent line that meets a matching
[end condition](007-4-6-html-blocks.md#end-condition), or the last line of the document, or the last line of
the [container block](010-4-8-paragraphs.md#container-blocks) containing the current HTML
block, if no line is encountered that meets the [end condition](007-4-6-html-blocks.md#end-condition). If
the first line meets both the [start condition](007-4-6-html-blocks.md#start-condition) and the [end
condition](007-4-6-html-blocks.md#end-condition), the block will contain just that line.

1. **Start condition:** line begins with the string `<pre`,
   `<script`, `<style`, or `<textarea` (case-insensitive), followed by a space,
   a tab, the string `>`, or the end of the line.  
   **End condition:** line contains an end tag
   `</pre>`, `</script>`, `</style>`, or `</textarea>` (case-insensitive; it
   need not match the start tag).
2. **Start condition:** line begins with the string `<!--`.  
   **End condition:** line contains the string `-->`.
3. **Start condition:** line begins with the string `<?`.  
   **End condition:** line contains the string `?>`.
4. **Start condition:** line begins with the string `<!`
   followed by an ASCII letter.  
   **End condition:** line contains the character `>`.
5. **Start condition:** line begins with the string
   `<![CDATA[`.  
   **End condition:** line contains the string `]]>`.
6. **Start condition:** line begins with the string `<` or `</`
   followed by one of the strings (case-insensitive) `address`,
   `article`, `aside`, `base`, `basefont`, `blockquote`, `body`,
   `caption`, `center`, `col`, `colgroup`, `dd`, `details`, `dialog`,
   `dir`, `div`, `dl`, `dt`, `fieldset`, `figcaption`, `figure`,
   `footer`, `form`, `frame`, `frameset`,
   `h1`, `h2`, `h3`, `h4`, `h5`, `h6`, `head`, `header`, `hr`,
   `html`, `iframe`, `legend`, `li`, `link`, `main`, `menu`, `menuitem`,
   `nav`, `noframes`, `ol`, `optgroup`, `option`, `p`, `param`,
   `search`, `section`, `summary`, `table`, `tbody`, `td`,
   `tfoot`, `th`, `thead`, `title`, `tr`, `track`, `ul`, followed
   by a space, a tab, the end of the line, the string `>`, or
   the string `/>`.  
   **End condition:** line is followed by a [blank line](002-1-3-about-this-document.md#blank-line).
7. **Start condition:** line begins with a complete [open tag](021-6-5-autolinks.md#open-tag)
   (with any [tag name](021-6-5-autolinks.md#tag-name) other than `pre`, `script`,
   `style`, or `textarea`) or a complete [closing tag](021-6-5-autolinks.md#closing-tag),
   followed by zero or more spaces and tabs, followed by the end of the line.  
   **End condition:** line is followed by a [blank line](002-1-3-about-this-document.md#blank-line).

HTML blocks continue until they are closed by their appropriate
[end condition](007-4-6-html-blocks.md#end-condition), or the last line of the document or other [container
block](010-4-8-paragraphs.md#container-blocks). This means any HTML **within an HTML
block** that might otherwise be recognised as a start condition will
be ignored by the parser and passed through as-is, without changing
the parser’s state.

For instance, `<pre>` within an HTML block started by `<table>` will not affect
the parser state; as the HTML block was started in by start condition 6, it
will end at any blank line. This can be surprising:

<a id="example-148"></a>

[Example 148](007-4-6-html-blocks.md#example-148)Try It



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




In this case, the HTML block is terminated by the blank line — the `**Hello**`
text remains verbatim — and regular parsing resumes, with a paragraph,
emphasised `world` and inline and block HTML following.

All types of [HTML blocks](007-4-6-html-blocks.md#html-blocks) except type 7 may interrupt
a paragraph. Blocks of type 7 may not interrupt a paragraph.
(This restriction is intended to prevent unwanted interpretation
of long tags inside a wrapped paragraph as starting HTML blocks.)

Some simple examples follow. Here are some basic HTML blocks
of type 6:

<a id="example-149"></a>

[Example 149](007-4-6-html-blocks.md#example-149)Try It



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




<a id="example-150"></a>

[Example 150](007-4-6-html-blocks.md#example-150)Try It



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

<a id="example-151"></a>

[Example 151](007-4-6-html-blocks.md#example-151)Try It



```
</div>
*foo*
```






```
</div>
*foo*
```




Here we have two HTML blocks with a Markdown paragraph between them:

<a id="example-152"></a>

[Example 152](007-4-6-html-blocks.md#example-152)Try It



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

<a id="example-153"></a>

[Example 153](007-4-6-html-blocks.md#example-153)Try It



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




<a id="example-154"></a>

[Example 154](007-4-6-html-blocks.md#example-154)Try It



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

<a id="example-155"></a>

[Example 155](007-4-6-html-blocks.md#example-155)Try It



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

<a id="example-156"></a>

[Example 156](007-4-6-html-blocks.md#example-156)Try It



```
<div id="foo"
*hi*
```






```
<div id="foo"
*hi*
```




<a id="example-157"></a>

[Example 157](007-4-6-html-blocks.md#example-157)Try It



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

<a id="example-158"></a>

[Example 158](007-4-6-html-blocks.md#example-158)Try It



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

<a id="example-159"></a>

[Example 159](007-4-6-html-blocks.md#example-159)Try It



```
<div><a href="bar">*foo*</a></div>
```






```
<div><a href="bar">*foo*</a></div>
```




<a id="example-160"></a>

[Example 160](007-4-6-html-blocks.md#example-160)Try It



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

<a id="example-161"></a>

[Example 161](007-4-6-html-blocks.md#example-161)Try It



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




To start an [HTML block](007-4-6-html-blocks.md#html-block) with a tag that is *not* in the
list of block-level tags in (6), you must put the tag by
itself on the first line (and it must be complete):

<a id="example-162"></a>

[Example 162](007-4-6-html-blocks.md#example-162)Try It



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




In type 7 blocks, the [tag name](021-6-5-autolinks.md#tag-name) can be anything:

<a id="example-163"></a>

[Example 163](007-4-6-html-blocks.md#example-163)Try It



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




<a id="example-164"></a>

[Example 164](007-4-6-html-blocks.md#example-164)Try It



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




<a id="example-165"></a>

[Example 165](007-4-6-html-blocks.md#example-165)Try It



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

<a id="example-166"></a>

[Example 166](007-4-6-html-blocks.md#example-166)Try It



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

<a id="example-167"></a>

[Example 167](007-4-6-html-blocks.md#example-167)Try It



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
as [raw HTML](021-6-5-autolinks.md#raw-html) *inside* the CommonMark paragraph. (Because
the tag is not on a line by itself, we get inline HTML
rather than an [HTML block](007-4-6-html-blocks.md#html-block).)

<a id="example-168"></a>

[Example 168](007-4-6-html-blocks.md#example-168)Try It



```
<del>*foo*</del>
```






```
<p><del><em>foo</em></del></p>
```




HTML tags designed to contain literal content
(`pre`, `script`, `style`, `textarea`), comments, processing instructions,
and declarations are treated somewhat differently.
Instead of ending at the first blank line, these blocks
end at the first line containing a corresponding end tag.
As a result, these blocks can contain blank lines:

A pre tag (type 1):

<a id="example-169"></a>

[Example 169](007-4-6-html-blocks.md#example-169)Try It



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

<a id="example-170"></a>

[Example 170](007-4-6-html-blocks.md#example-170)Try It



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




A textarea tag (type 1):

<a id="example-171"></a>

[Example 171](007-4-6-html-blocks.md#example-171)Try It



```
<textarea>

*foo*

_bar_

</textarea>
```






```
<textarea>

*foo*

_bar_

</textarea>
```




A style tag (type 1):

