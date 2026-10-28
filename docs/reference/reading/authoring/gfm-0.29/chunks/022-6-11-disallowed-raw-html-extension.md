# 6.11 Disallowed Raw HTML (extension)

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](021-6-10-raw-html.md) · [Next](023-phase-1-block-structure.md)

<a id="disallowed-raw-html-extension-"></a>

## 6.11 Disallowed Raw HTML (extension)

GFM enables the `tagfilter` extension, where the following HTML tags will be
filtered when rendering HTML output:

- `<title>`
- `<textarea>`
- `<style>`
- `<xmp>`
- `<iframe>`
- `<noembed>`
- `<noframes>`
- `<script>`
- `<plaintext>`

Filtering is done by replacing the leading `<` with the entity `&lt;`. These
tags are chosen in particular as they change how HTML is interpreted in a way
unique to them (i.e. nested HTML is interpreted differently), and this is
usually undesireable in the context of other rendered Markdown content.

All other HTML tags are left untouched.

<a id="example-657"></a>

[Example 657](022-6-11-disallowed-raw-html-extension.md#example-657)



```
<strong> <title> <style> <em>

<blockquote>
  <xmp> is disallowed.  <XMP> is also disallowed.
</blockquote>
```






```
<p><strong> &lt;title> &lt;style> <em></p>
<blockquote>
  &lt;xmp> is disallowed.  &lt;XMP> is also disallowed.
</blockquote>
```




<a id="hard-line-breaks"></a>

## 6.12 Hard line breaks

<a id="hard-line-break"></a>

A line break (not in a code span or HTML tag) that is preceded
by two or more spaces and does not occur at the end of a block
is parsed as a [hard line break](022-6-11-disallowed-raw-html-extension.md#hard-line-break) (rendered
in HTML as a `<br />` tag):

<a id="example-658"></a>

[Example 658](022-6-11-disallowed-raw-html-extension.md#example-658)



```
foo  
baz
```






```
<p>foo<br />
baz</p>
```




For a more visible alternative, a backslash before the
[line ending](002-1-3-why-is-a-spec-needed.md#line-ending) may be used instead of two spaces:

<a id="example-659"></a>

[Example 659](022-6-11-disallowed-raw-html-extension.md#example-659)



```
foo\
baz
```






```
<p>foo<br />
baz</p>
```




More than two spaces can be used:

<a id="example-660"></a>

[Example 660](022-6-11-disallowed-raw-html-extension.md#example-660)



```
foo       
baz
```






```
<p>foo<br />
baz</p>
```




Leading spaces at the beginning of the next line are ignored:

<a id="example-661"></a>

[Example 661](022-6-11-disallowed-raw-html-extension.md#example-661)



```
foo  
     bar
```






```
<p>foo<br />
bar</p>
```




<a id="example-662"></a>

[Example 662](022-6-11-disallowed-raw-html-extension.md#example-662)



```
foo\
     bar
```






```
<p>foo<br />
bar</p>
```




Line breaks can occur inside emphasis, links, and other constructs
that allow inline content:

<a id="example-663"></a>

[Example 663](022-6-11-disallowed-raw-html-extension.md#example-663)



```
*foo  
bar*
```






```
<p><em>foo<br />
bar</em></p>
```




<a id="example-664"></a>

[Example 664](022-6-11-disallowed-raw-html-extension.md#example-664)



```
*foo\
bar*
```






```
<p><em>foo<br />
bar</em></p>
```




Line breaks do not occur inside code spans

<a id="example-665"></a>

[Example 665](022-6-11-disallowed-raw-html-extension.md#example-665)



```
`code  
span`
```






```
<p><code>code   span</code></p>
```




<a id="example-666"></a>

[Example 666](022-6-11-disallowed-raw-html-extension.md#example-666)



```
`code\
span`
```






```
<p><code>code\ span</code></p>
```




or HTML tags:

<a id="example-667"></a>

[Example 667](022-6-11-disallowed-raw-html-extension.md#example-667)



```
<a href="foo  
bar">
```






```
<p><a href="foo  
bar"></p>
```




<a id="example-668"></a>

[Example 668](022-6-11-disallowed-raw-html-extension.md#example-668)



```
<a href="foo\
bar">
```






```
<p><a href="foo\
bar"></p>
```




Hard line breaks are for separating inline content within a block.
Neither syntax for hard line breaks works at the end of a paragraph or
other block element:

<a id="example-669"></a>

[Example 669](022-6-11-disallowed-raw-html-extension.md#example-669)



```
foo\
```






```
<p>foo\</p>
```




<a id="example-670"></a>

[Example 670](022-6-11-disallowed-raw-html-extension.md#example-670)



```
foo  
```






```
<p>foo</p>
```




<a id="example-671"></a>

[Example 671](022-6-11-disallowed-raw-html-extension.md#example-671)



```
### foo\
```






```
<h3>foo\</h3>
```




<a id="example-672"></a>

[Example 672](022-6-11-disallowed-raw-html-extension.md#example-672)



```
### foo  
```






```
<h3>foo</h3>
```




<a id="soft-line-breaks"></a>

## 6.13 Soft line breaks

<a id="softbreak"></a>

A regular line break (not in a code span or HTML tag) that is not
preceded by two or more spaces or a backslash is parsed as a
[softbreak](022-6-11-disallowed-raw-html-extension.md#softbreak). (A softbreak may be rendered in HTML either as a
[line ending](002-1-3-why-is-a-spec-needed.md#line-ending) or as a space. The result will be the same in
browsers. In the examples here, a [line ending](002-1-3-why-is-a-spec-needed.md#line-ending) will be used.)

<a id="example-673"></a>

[Example 673](022-6-11-disallowed-raw-html-extension.md#example-673)



```
foo
baz
```






```
<p>foo
baz</p>
```




Spaces at the end of the line and beginning of the next line are
removed:

<a id="example-674"></a>

[Example 674](022-6-11-disallowed-raw-html-extension.md#example-674)



```
foo 
 baz
```






```
<p>foo
baz</p>
```




A conforming parser may render a soft line break in HTML either as a
line break or as a space.

A renderer may also provide an option to render soft line breaks
as hard line breaks.

<a id="textual-content"></a>

## 6.14 Textual content

Any characters not given an interpretation by the above rules will
be parsed as plain textual content.

<a id="example-675"></a>

[Example 675](022-6-11-disallowed-raw-html-extension.md#example-675)



```
hello $.;'there
```






```
<p>hello $.;'there</p>
```




<a id="example-676"></a>

[Example 676](022-6-11-disallowed-raw-html-extension.md#example-676)



```
Foo χρῆν
```






```
<p>Foo χρῆν</p>
```




Internal spaces are preserved verbatim:

<a id="example-677"></a>

[Example 677](022-6-11-disallowed-raw-html-extension.md#example-677)



```
Multiple     spaces
```






```
<p>Multiple     spaces</p>
```




<a id="appendix-a-parsing-strategy"></a>

# Appendix: A parsing strategy

In this appendix we describe some features of the parsing strategy
used in the CommonMark reference implementations.

<a id="overview"></a>

## Overview

Parsing has two phases:

1. In the first phase, lines of input are consumed and the block
   structure of the document—its division into paragraphs, block quotes,
   list items, and so on—is constructed. Text is assigned to these
   blocks but not parsed. Link reference definitions are parsed and a
   map of links is constructed.
2. In the second phase, the raw text contents of paragraphs and headings
   are parsed into sequences of Markdown inline elements (strings,
   code spans, links, emphasis, and so on), using the map of link
   references constructed in phase 1.

At each point in processing, the document is represented as a tree of
**blocks**. The root of the tree is a `document` block. The `document`
may have any number of other blocks as **children**. These children
may, in turn, have other blocks as children. The last child of a block
is normally considered **open**, meaning that subsequent lines of input
can alter its contents. (Blocks that are not open are **closed**.)
Here, for example, is a possible document tree, with the open blocks
marked by arrows:



```
-> document
  -> block_quote
       paragraph
         "Lorem ipsum dolor\nsit amet."
    -> list (type=bullet tight=true bullet_char=-)
         list_item
           paragraph
             "Qui *quodsi iracundia*"
      -> list_item
        -> paragraph
             "aliquando id"
```




