# 5.3 Task list items (extension)

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-5-2-1-motivation.md) · [Next](013-6-inlines.md)

<a id="task-list-items-extension-"></a>

## 5.3 Task list items (extension)

GFM enables the `tasklist` extension, where an additional processing step is
performed on [list items](010-5-2-list-items.md#list-items).

<a id="task-list-item"></a>

A [task list item](012-5-3-task-list-items-extension.md#task-list-item) is a [list item](010-5-2-list-items.md#list-items) where the first block in it
is a paragraph which begins with a [task list item marker](012-5-3-task-list-items-extension.md#task-list-item-marker) and at least one
whitespace character before any other content.

<a id="task-list-item-marker"></a>

A [task list item marker](012-5-3-task-list-items-extension.md#task-list-item-marker) consists of an optional number of spaces, a left
bracket (`[`), either a whitespace character or the letter `x` in either
lowercase or uppercase, and then a right bracket (`]`).

When rendered, the [task list item marker](012-5-3-task-list-items-extension.md#task-list-item-marker) is replaced with a semantic checkbox element;
in an HTML output, this would be an `<input type="checkbox">` element.

If the character between the brackets is a whitespace character, the checkbox
is unchecked. Otherwise, the checkbox is checked.

This spec does not define how the checkbox elements are interacted with: in practice,
implementors are free to render the checkboxes as disabled or inmutable elements,
or they may dynamically handle dynamic interactions (i.e. checking, unchecking) in
the final rendered document.

<a id="example-279"></a>

[Example 279](012-5-3-task-list-items-extension.md#example-279)



```
- [ ] foo
- [x] bar
```






```
<ul>
<li><input disabled="" type="checkbox"> foo</li>
<li><input checked="" disabled="" type="checkbox"> bar</li>
</ul>
```




Task lists can be arbitrarily nested:

<a id="example-280"></a>

[Example 280](012-5-3-task-list-items-extension.md#example-280)



```
- [x] foo
  - [ ] bar
  - [x] baz
- [ ] bim
```






```
<ul>
<li><input checked="" disabled="" type="checkbox"> foo
<ul>
<li><input disabled="" type="checkbox"> bar</li>
<li><input checked="" disabled="" type="checkbox"> baz</li>
</ul>
</li>
<li><input disabled="" type="checkbox"> bim</li>
</ul>
```




<a id="lists"></a>

## 5.4 Lists

<a id="list"></a>

A [list](012-5-3-task-list-items-extension.md#list) is a sequence of one or more
list items [of the same type](012-5-3-task-list-items-extension.md#of-the-same-type). The list items
may be separated by any number of blank lines.

<a id="of-the-same-type"></a>

Two list items are [of the same type](012-5-3-task-list-items-extension.md#of-the-same-type)
if they begin with a [list marker](010-5-2-list-items.md#list-marker) of the same type.
Two list markers are of the
same type if (a) they are bullet list markers using the same character
(`-`, `+`, or `*`) or (b) they are ordered list numbers with the same
delimiter (either `.` or `)`).

<a id="ordered-list"></a>

<a id="bullet-list"></a>

A list is an [ordered list](012-5-3-task-list-items-extension.md#ordered-list)
if its constituent list items begin with
[ordered list markers](010-5-2-list-items.md#ordered-list-marker), and a
[bullet list](012-5-3-task-list-items-extension.md#bullet-list) if its constituent list
items begin with [bullet list markers](010-5-2-list-items.md#bullet-list-marker).

<a id="start-number"></a>

The [start number](012-5-3-task-list-items-extension.md#start-number)
of an [ordered list](012-5-3-task-list-items-extension.md#ordered-list) is determined by the list number of
its initial list item. The numbers of subsequent list items are
disregarded.

<a id="loose"></a>

<a id="tight"></a>

A list is [loose](012-5-3-task-list-items-extension.md#loose) if any of its constituent
list items are separated by blank lines, or if any of its constituent
list items directly contain two block-level elements with a blank line
between them. Otherwise a list is [tight](012-5-3-task-list-items-extension.md#tight).
(The difference in HTML output is that paragraphs in a loose list are
wrapped in `<p>` tags, while paragraphs in a tight list are not.)

Changing the bullet or ordered list delimiter starts a new list:

<a id="example-281"></a>

[Example 281](012-5-3-task-list-items-extension.md#example-281)



```
- foo
- bar
+ baz
```






```
<ul>
<li>foo</li>
<li>bar</li>
</ul>
<ul>
<li>baz</li>
</ul>
```




<a id="example-282"></a>

[Example 282](012-5-3-task-list-items-extension.md#example-282)



```
1. foo
2. bar
3) baz
```






```
<ol>
<li>foo</li>
<li>bar</li>
</ol>
<ol start="3">
<li>baz</li>
</ol>
```




In CommonMark, a list can interrupt a paragraph. That is,
no blank line is needed to separate a paragraph from a following
list:

<a id="example-283"></a>

[Example 283](012-5-3-task-list-items-extension.md#example-283)



```
Foo
- bar
- baz
```






```
<p>Foo</p>
<ul>
<li>bar</li>
<li>baz</li>
</ul>
```




`Markdown.pl` does not allow this, through fear of triggering a list
via a numeral in a hard-wrapped line:



```
The number of windows in my house is
14.  The number of doors is 6.
```




Oddly, though, `Markdown.pl` *does* allow a blockquote to
interrupt a paragraph, even though the same considerations might
apply.

In CommonMark, we do allow lists to interrupt paragraphs, for
two reasons. First, it is natural and not uncommon for people
to start lists without blank lines:



```
I need to buy
- new shoes
- a coat
- a plane ticket
```




Second, we are attracted to a

> <a id="principle-of-uniformity"></a>
>
> [principle of uniformity](012-5-3-task-list-items-extension.md#principle-of-uniformity):
> if a chunk of text has a certain
> meaning, it will continue to have the same meaning when put into a
> container block (such as a list item or blockquote).

(Indeed, the spec for [list items](010-5-2-list-items.md#list-items) and [block quotes](009-5-container-blocks.md#block-quotes) presupposes
this principle.) This principle implies that if



```
  * I need to buy
    - new shoes
    - a coat
    - a plane ticket
```




is a list item containing a paragraph followed by a nested sublist,
as all Markdown implementations agree it is (though the paragraph
may be rendered without `<p>` tags, since the list is “tight”),
then



```
I need to buy
- new shoes
- a coat
- a plane ticket
```




by itself should be a paragraph followed by a nested sublist.

Since it is well established Markdown practice to allow lists to
interrupt paragraphs inside list items, the [principle of
uniformity](012-5-3-task-list-items-extension.md#principle-of-uniformity) requires us to allow this outside list items as
well. ([reStructuredText](http://docutils.sourceforge.net/rst.html)
takes a different approach, requiring blank lines before lists
even inside other list items.)

In order to solve of unwanted lists in paragraphs with
hard-wrapped numerals, we allow only lists starting with `1` to
interrupt paragraphs. Thus,

<a id="example-284"></a>

[Example 284](012-5-3-task-list-items-extension.md#example-284)



```
The number of windows in my house is
14.  The number of doors is 6.
```






```
<p>The number of windows in my house is
14.  The number of doors is 6.</p>
```




We may still get an unintended result in cases like

<a id="example-285"></a>

[Example 285](012-5-3-task-list-items-extension.md#example-285)



```
The number of windows in my house is
1.  The number of doors is 6.
```






```
<p>The number of windows in my house is</p>
<ol>
<li>The number of doors is 6.</li>
</ol>
```




but this rule should prevent most spurious list captures.

There can be any number of blank lines between items:

<a id="example-286"></a>

[Example 286](012-5-3-task-list-items-extension.md#example-286)



```
- foo

- bar


- baz
```






```
<ul>
<li>
<p>foo</p>
</li>
<li>
<p>bar</p>
</li>
<li>
<p>baz</p>
</li>
</ul>
```




<a id="example-287"></a>

[Example 287](012-5-3-task-list-items-extension.md#example-287)



```
- foo
  - bar
    - baz


      bim
```






```
<ul>
<li>foo
<ul>
<li>bar
<ul>
<li>
<p>baz</p>
<p>bim</p>
</li>
</ul>
</li>
</ul>
</li>
</ul>
```




To separate consecutive lists of the same type, or to separate a
list from an indented code block that would otherwise be parsed
as a subparagraph of the final list item, you can insert a blank HTML
comment:

<a id="example-288"></a>

[Example 288](012-5-3-task-list-items-extension.md#example-288)



```
- foo
- bar

<!-- -->

- baz
- bim
```






```
<ul>
<li>foo</li>
<li>bar</li>
</ul>
<!-- -->
<ul>
<li>baz</li>
<li>bim</li>
</ul>
```




<a id="example-289"></a>

[Example 289](012-5-3-task-list-items-extension.md#example-289)



```
-   foo

    notcode

-   foo

<!-- -->

    code
```






```
<ul>
<li>
<p>foo</p>
<p>notcode</p>
</li>
<li>
<p>foo</p>
</li>
</ul>
<!-- -->
<pre><code>code
</code></pre>
```




List items need not be indented to the same level. The following
list items will be treated as items at the same list level,
since none is indented enough to belong to the previous list
item:

<a id="example-290"></a>

[Example 290](012-5-3-task-list-items-extension.md#example-290)



```
- a
 - b
  - c
   - d
  - e
 - f
- g
```






```
<ul>
<li>a</li>
<li>b</li>
<li>c</li>
<li>d</li>
<li>e</li>
<li>f</li>
<li>g</li>
</ul>
```




<a id="example-291"></a>

[Example 291](012-5-3-task-list-items-extension.md#example-291)



```
1. a

  2. b

   3. c
```






```
<ol>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>c</p>
</li>
</ol>
```




Note, however, that list items may not be indented more than
three spaces. Here `- e` is treated as a paragraph continuation
line, because it is indented more than three spaces:

<a id="example-292"></a>

[Example 292](012-5-3-task-list-items-extension.md#example-292)



```
- a
 - b
  - c
   - d
    - e
```






```
<ul>
<li>a</li>
<li>b</li>
<li>c</li>
<li>d
- e</li>
</ul>
```




And here, `3. c` is treated as in indented code block,
because it is indented four spaces and preceded by a
blank line.

<a id="example-293"></a>

[Example 293](012-5-3-task-list-items-extension.md#example-293)



```
1. a

  2. b

    3. c
```






```
<ol>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
</ol>
<pre><code>3. c
</code></pre>
```




This is a loose list, because there is a blank line between
two of the list items:

<a id="example-294"></a>

[Example 294](012-5-3-task-list-items-extension.md#example-294)



```
- a
- b

- c
```






```
<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>c</p>
</li>
</ul>
```




So is this, with a empty second item:

<a id="example-295"></a>

[Example 295](012-5-3-task-list-items-extension.md#example-295)



```
* a
*

* c
```






```
<ul>
<li>
<p>a</p>
</li>
<li></li>
<li>
<p>c</p>
</li>
</ul>
```




These are loose lists, even though there is no space between the items,
because one of the items directly contains two block-level elements
with a blank line between them:

<a id="example-296"></a>

[Example 296](012-5-3-task-list-items-extension.md#example-296)



```
- a
- b

  c
- d
```






```
<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
<p>c</p>
</li>
<li>
<p>d</p>
</li>
</ul>
```




<a id="example-297"></a>

[Example 297](012-5-3-task-list-items-extension.md#example-297)



```
- a
- b

  [ref]: /url
- d
```






```
<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>d</p>
</li>
</ul>
```




This is a tight list, because the blank lines are in a code block:

<a id="example-298"></a>

[Example 298](012-5-3-task-list-items-extension.md#example-298)



````
- a
- ```
  b


  ```
- c
````






```
<ul>
<li>a</li>
<li>
<pre><code>b


</code></pre>
</li>
<li>c</li>
</ul>
```




This is a tight list, because the blank line is between two
paragraphs of a sublist. So the sublist is loose while
the outer list is tight:

<a id="example-299"></a>

[Example 299](012-5-3-task-list-items-extension.md#example-299)



```
- a
  - b

    c
- d
```






```
<ul>
<li>a
<ul>
<li>
<p>b</p>
<p>c</p>
</li>
</ul>
</li>
<li>d</li>
</ul>
```




This is a tight list, because the blank line is inside the
block quote:

<a id="example-300"></a>

[Example 300](012-5-3-task-list-items-extension.md#example-300)



```
* a
  > b
  >
* c
```






```
<ul>
<li>a
<blockquote>
<p>b</p>
</blockquote>
</li>
<li>c</li>
</ul>
```




This list is tight, because the consecutive block elements
are not separated by blank lines:

<a id="example-301"></a>

[Example 301](012-5-3-task-list-items-extension.md#example-301)



````
- a
  > b
  ```
  c
  ```
- d
````






```
<ul>
<li>a
<blockquote>
<p>b</p>
</blockquote>
<pre><code>c
</code></pre>
</li>
<li>d</li>
</ul>
```




A single-paragraph list is tight:

<a id="example-302"></a>

[Example 302](012-5-3-task-list-items-extension.md#example-302)



```
- a
```






```
<ul>
<li>a</li>
</ul>
```




<a id="example-303"></a>

[Example 303](012-5-3-task-list-items-extension.md#example-303)



```
- a
  - b
```






```
<ul>
<li>a
<ul>
<li>b</li>
</ul>
</li>
</ul>
```




This list is loose, because of the blank line between the
two block elements in the list item:

<a id="example-304"></a>

[Example 304](012-5-3-task-list-items-extension.md#example-304)



````
1. ```
   foo
   ```

   bar
````






```
<ol>
<li>
<pre><code>foo
</code></pre>
<p>bar</p>
</li>
</ol>
```




Here the outer list is loose, the inner list tight:

<a id="example-305"></a>

[Example 305](012-5-3-task-list-items-extension.md#example-305)



```
* foo
  * bar

  baz
```






```
<ul>
<li>
<p>foo</p>
<ul>
<li>bar</li>
</ul>
<p>baz</p>
</li>
</ul>
```




<a id="example-306"></a>

[Example 306](012-5-3-task-list-items-extension.md#example-306)



```
- a
  - b
  - c

- d
  - e
  - f
```






```
<ul>
<li>
<p>a</p>
<ul>
<li>b</li>
<li>c</li>
</ul>
</li>
<li>
<p>d</p>
<ul>
<li>e</li>
<li>f</li>
</ul>
</li>
</ul>
```




