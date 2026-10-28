# 6.7 Hard line breaks

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](021-6-5-autolinks.md) · [Next](023-phase-1-block-structure.md)

Processing instructions:

<a id="example-627"></a>

[Example 627](022-6-7-hard-line-breaks.md#example-627)Try It



```
foo <?php echo $a; ?>
```






```
<p>foo <?php echo $a; ?></p>
```




Declarations:

<a id="example-628"></a>

[Example 628](022-6-7-hard-line-breaks.md#example-628)Try It



```
foo <!ELEMENT br EMPTY>
```






```
<p>foo <!ELEMENT br EMPTY></p>
```




CDATA sections:

<a id="example-629"></a>

[Example 629](022-6-7-hard-line-breaks.md#example-629)Try It



```
foo <![CDATA[>&<]]>
```






```
<p>foo <![CDATA[>&<]]></p>
```




Entity and numeric character references are preserved in HTML
attributes:

<a id="example-630"></a>

[Example 630](022-6-7-hard-line-breaks.md#example-630)Try It



```
foo <a href="&ouml;">
```






```
<p>foo <a href="&ouml;"></p>
```




Backslash escapes do not work in HTML attributes:

<a id="example-631"></a>

[Example 631](022-6-7-hard-line-breaks.md#example-631)Try It



```
foo <a href="\*">
```






```
<p>foo <a href="\*"></p>
```




<a id="example-632"></a>

[Example 632](022-6-7-hard-line-breaks.md#example-632)Try It



```
<a href="\"">
```






```
<p>&lt;a href=&quot;&quot;&quot;&gt;</p>
```




<a id="hard-line-breaks"></a>

## 6.7 Hard line breaks

<a id="hard-line-break"></a>

A line ending (not in a code span or HTML tag) that is preceded
by two or more spaces and does not occur at the end of a block
is parsed as a [hard line break](022-6-7-hard-line-breaks.md#hard-line-break) (rendered
in HTML as a `<br />` tag):

<a id="example-633"></a>

[Example 633](022-6-7-hard-line-breaks.md#example-633)Try It



```
foo  
baz
```






```
<p>foo<br />
baz</p>
```




For a more visible alternative, a backslash before the
[line ending](002-1-3-about-this-document.md#line-ending) may be used instead of two or more spaces:

<a id="example-634"></a>

[Example 634](022-6-7-hard-line-breaks.md#example-634)Try It



```
foo\
baz
```






```
<p>foo<br />
baz</p>
```




More than two spaces can be used:

<a id="example-635"></a>

[Example 635](022-6-7-hard-line-breaks.md#example-635)Try It



```
foo       
baz
```






```
<p>foo<br />
baz</p>
```




Leading spaces at the beginning of the next line are ignored:

<a id="example-636"></a>

[Example 636](022-6-7-hard-line-breaks.md#example-636)Try It



```
foo  
     bar
```






```
<p>foo<br />
bar</p>
```




<a id="example-637"></a>

[Example 637](022-6-7-hard-line-breaks.md#example-637)Try It



```
foo\
     bar
```






```
<p>foo<br />
bar</p>
```




Hard line breaks can occur inside emphasis, links, and other constructs
that allow inline content:

<a id="example-638"></a>

[Example 638](022-6-7-hard-line-breaks.md#example-638)Try It



```
*foo  
bar*
```






```
<p><em>foo<br />
bar</em></p>
```




<a id="example-639"></a>

[Example 639](022-6-7-hard-line-breaks.md#example-639)Try It



```
*foo\
bar*
```






```
<p><em>foo<br />
bar</em></p>
```




Hard line breaks do not occur inside code spans

<a id="example-640"></a>

[Example 640](022-6-7-hard-line-breaks.md#example-640)Try It



```
`code  
span`
```






```
<p><code>code   span</code></p>
```




<a id="example-641"></a>

[Example 641](022-6-7-hard-line-breaks.md#example-641)Try It



```
`code\
span`
```






```
<p><code>code\ span</code></p>
```




or HTML tags:

<a id="example-642"></a>

[Example 642](022-6-7-hard-line-breaks.md#example-642)Try It



```
<a href="foo  
bar">
```






```
<p><a href="foo  
bar"></p>
```




<a id="example-643"></a>

[Example 643](022-6-7-hard-line-breaks.md#example-643)Try It



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

<a id="example-644"></a>

[Example 644](022-6-7-hard-line-breaks.md#example-644)Try It



```
foo\
```






```
<p>foo\</p>
```




<a id="example-645"></a>

[Example 645](022-6-7-hard-line-breaks.md#example-645)Try It



```
foo  
```






```
<p>foo</p>
```




<a id="example-646"></a>

[Example 646](022-6-7-hard-line-breaks.md#example-646)Try It



```
### foo\
```






```
<h3>foo\</h3>
```




<a id="example-647"></a>

[Example 647](022-6-7-hard-line-breaks.md#example-647)Try It



```
### foo  
```






```
<h3>foo</h3>
```




<a id="soft-line-breaks"></a>

## 6.8 Soft line breaks

<a id="softbreak"></a>

A regular line ending (not in a code span or HTML tag) that is not
preceded by two or more spaces or a backslash is parsed as a
[softbreak](022-6-7-hard-line-breaks.md#softbreak). (A soft line break may be rendered in HTML either as a
[line ending](002-1-3-about-this-document.md#line-ending) or as a space. The result will be the same in
browsers. In the examples here, a [line ending](002-1-3-about-this-document.md#line-ending) will be used.)

<a id="example-648"></a>

[Example 648](022-6-7-hard-line-breaks.md#example-648)Try It



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

<a id="example-649"></a>

[Example 649](022-6-7-hard-line-breaks.md#example-649)Try It



```
foo 
 baz
```






```
<p>foo
baz</p>
```




A conforming parser may render a soft line break in HTML either as a
line ending or as a space.

A renderer may also provide an option to render soft line breaks
as hard line breaks.

<a id="textual-content"></a>

## 6.9 Textual content

Any characters not given an interpretation by the above rules will
be parsed as plain textual content.

<a id="example-650"></a>

[Example 650](022-6-7-hard-line-breaks.md#example-650)Try It



```
hello $.;'there
```






```
<p>hello $.;'there</p>
```




<a id="example-651"></a>

[Example 651](022-6-7-hard-line-breaks.md#example-651)Try It



```
Foo χρῆν
```






```
<p>Foo χρῆν</p>
```




Internal spaces are preserved verbatim:

<a id="example-652"></a>

[Example 652](022-6-7-hard-line-breaks.md#example-652)Try It



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




