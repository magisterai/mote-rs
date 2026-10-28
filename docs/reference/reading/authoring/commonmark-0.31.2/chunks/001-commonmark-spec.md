# CommonMark Spec

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-1-3-about-this-document.md)

# CommonMark Spec

Version 0.31.2 (2024-01-28)

John MacFarlane

[Creative Commons BY-SA](https://creativecommons.org/licenses/by-sa/4.0/)  
<span style="display: none;"><span href="https://purl.org/dc/dcmitype/Text" property="dct:title" rel="dct:type" xmlns:dct="https://purl.org/dc/terms/">CommonMark Spec</span> by
   <a href="https://spec.commonmark.org" property="cc:attributionName" rel="cc:attributionURL" xmlns:cc="https://creativecommons.org/ns#">John MacFarlane</a> is licensed under a
   <a href="https://creativecommons.org/licenses/by-sa/4.0/" rel="license">Creative
   Commons Attribution-ShareAlike 4.0 International License</a>.</span>

<a id="watermark"></a>

<a id="TOC"></a>

- [1Introduction](001-commonmark-spec.md#introduction)
  - [1.1What is Markdown?](001-commonmark-spec.md#what-is-markdown-)
  - [1.2Why is a spec needed?](001-commonmark-spec.md#why-is-a-spec-needed-)
  - [1.3About this document](002-1-3-about-this-document.md#about-this-document)
- [2Preliminaries](002-1-3-about-this-document.md#preliminaries)
  - [2.1Characters and lines](002-1-3-about-this-document.md#characters-and-lines)
  - [2.2Tabs](002-1-3-about-this-document.md#tabs)
  - [2.3Insecure characters](003-2-3-insecure-characters.md#insecure-characters)
  - [2.4Backslash escapes](003-2-3-insecure-characters.md#backslash-escapes)
  - [2.5Entity and numeric character references](003-2-3-insecure-characters.md#entity-and-numeric-character-references)
- [3Blocks and inlines](004-3-blocks-and-inlines.md#blocks-and-inlines)
  - [3.1Precedence](004-3-blocks-and-inlines.md#precedence)
  - [3.2Container blocks and leaf blocks](004-3-blocks-and-inlines.md#container-blocks-and-leaf-blocks)
- [4Leaf blocks](004-3-blocks-and-inlines.md#leaf-blocks)
  - [4.1Thematic breaks](004-3-blocks-and-inlines.md#thematic-breaks)
  - [4.2ATX headings](004-3-blocks-and-inlines.md#atx-headings)
  - [4.3Setext headings](005-4-3-setext-headings.md#setext-headings)
  - [4.4Indented code blocks](006-4-4-indented-code-blocks.md#indented-code-blocks)
  - [4.5Fenced code blocks](006-4-4-indented-code-blocks.md#fenced-code-blocks)
  - [4.6HTML blocks](007-4-6-html-blocks.md#html-blocks)
  - [4.7Link reference definitions](009-4-7-link-reference-definitions.md#link-reference-definitions)
  - [4.8Paragraphs](010-4-8-paragraphs.md#paragraphs)
  - [4.9Blank lines](010-4-8-paragraphs.md#blank-lines)
- [5Container blocks](010-4-8-paragraphs.md#container-blocks)
  - [5.1Block quotes](010-4-8-paragraphs.md#block-quotes)
  - [5.2List items](011-5-2-list-items.md#list-items)
  - [5.3Lists](013-5-2-1-motivation.md#lists)
- [6Inlines](014-6-inlines.md#inlines)
  - [6.1Code spans](014-6-inlines.md#code-spans)
  - [6.2Emphasis and strong emphasis](015-6-2-emphasis-and-strong-emphasis.md#emphasis-and-strong-emphasis)
  - [6.3Links](018-6-3-links.md#links)
  - [6.4Images](020-6-4-images.md#images)
  - [6.5Autolinks](021-6-5-autolinks.md#autolinks)
  - [6.6Raw HTML](021-6-5-autolinks.md#raw-html)
  - [6.7Hard line breaks](022-6-7-hard-line-breaks.md#hard-line-breaks)
  - [6.8Soft line breaks](022-6-7-hard-line-breaks.md#soft-line-breaks)
  - [6.9Textual content](022-6-7-hard-line-breaks.md#textual-content)
- [Appendix: A parsing strategy](022-6-7-hard-line-breaks.md#appendix-a-parsing-strategy)
  - [Overview](022-6-7-hard-line-breaks.md#overview)
  - [Phase 1: block structure](023-phase-1-block-structure.md#phase-1-block-structure)
  - [Phase 2: inline structure](023-phase-1-block-structure.md#phase-2-inline-structure)

<a id="introduction"></a>

# 1 Introduction

<a id="what-is-markdown-"></a>

## 1.1 What is Markdown?

Markdown is a plain text format for writing structured documents,
based on conventions for indicating formatting in email
and usenet posts. It was developed by John Gruber (with
help from Aaron Swartz) and released in 2004 in the form of a
[syntax description](https://daringfireball.net/projects/markdown/syntax)
and a Perl script (`Markdown.pl`) for converting Markdown to
HTML. In the next decade, dozens of implementations were
developed in many languages. Some extended the original
Markdown syntax with conventions for footnotes, tables, and
other document elements. Some allowed Markdown documents to be
rendered in formats other than HTML. Websites like Reddit,
StackOverflow, and GitHub had millions of people using Markdown.
And Markdown started to be used beyond the web, to author books,
articles, slide shows, letters, and lecture notes.

What distinguishes Markdown from many other lightweight markup
syntaxes, which are often easier to write, is its readability.
As Gruber writes:

> The overriding design goal for Markdown’s formatting syntax is
> to make it as readable as possible. The idea is that a
> Markdown-formatted document should be publishable as-is, as
> plain text, without looking like it’s been marked up with tags
> or formatting instructions.
> ([https://daringfireball.net/projects/markdown/](https://daringfireball.net/projects/markdown/))

The point can be illustrated by comparing a sample of
[AsciiDoc](https://asciidoc.org/) with
an equivalent sample of Markdown. Here is a sample of
AsciiDoc from the AsciiDoc manual:



```
1. List item one.
+
List item one continued with a second paragraph followed by an
Indented block.
+
.................
$ ls *.sh
$ mv *.sh ~/tmp
.................
+
List item continued with a third paragraph.

2. List item two continued with an open block.
+
--
This paragraph is part of the preceding list item.

a. This list is nested and does not require explicit item
continuation.
+
This paragraph is part of the preceding list item.

b. List item b.

This paragraph belongs to item two of the outer list.
--
```




And here is the equivalent in Markdown:



```
1.  List item one.

    List item one continued with a second paragraph followed by an
    Indented block.

        $ ls *.sh
        $ mv *.sh ~/tmp

    List item continued with a third paragraph.

2.  List item two continued with an open block.

    This paragraph is part of the preceding list item.

    1. This list is nested and does not require explicit item continuation.

       This paragraph is part of the preceding list item.

    2. List item b.

    This paragraph belongs to item two of the outer list.
```




The AsciiDoc version is, arguably, easier to write. You don’t need
to worry about indentation. But the Markdown version is much easier
to read. The nesting of list items is apparent to the eye in the
source, not just in the processed document.

<a id="why-is-a-spec-needed-"></a>

## 1.2 Why is a spec needed?

John Gruber’s [canonical description of Markdown’s
syntax](https://daringfireball.net/projects/markdown/syntax)
does not specify the syntax unambiguously. Here are some examples of
questions it does not answer:

1. How much indentation is needed for a sublist? The spec says that
   continuation paragraphs need to be indented four spaces, but is
   not fully explicit about sublists. It is natural to think that
   they, too, must be indented four spaces, but `Markdown.pl` does
   not require that. This is hardly a “corner case,” and divergences
   between implementations on this issue often lead to surprises for
   users in real documents. (See [this comment by John
   Gruber](https://web.archive.org/web/20170611172104/http://article.gmane.org/gmane.text.markdown.general/1997).)
2. Is a blank line needed before a block quote or heading?
   Most implementations do not require the blank line. However,
   this can lead to unexpected results in hard-wrapped text, and
   also to ambiguities in parsing (note that some implementations
   put the heading inside the blockquote, while others do not).
   (John Gruber has also spoken [in favor of requiring the blank
   lines](https://web.archive.org/web/20170611172104/http://article.gmane.org/gmane.text.markdown.general/2146).)
3. Is a blank line needed before an indented code block?
   (`Markdown.pl` requires it, but this is not mentioned in the
   documentation, and some implementations do not require it.)



```
paragraph
    code?
```



4. What is the exact rule for determining when list items get
   wrapped in `<p>` tags? Can a list be partially “loose” and partially
   “tight”? What should we do with a list like this?



```
1. one

2. two
3. three
```




   Or this?



```
1.  one
    - a

    - b
2.  two
```




   (There are some relevant comments by John Gruber
   [here](https://web.archive.org/web/20170611172104/http://article.gmane.org/gmane.text.markdown.general/2554).)
5. Can list markers be indented? Can ordered list markers be right-aligned?



```
 8. item 1
 9. item 2
10. item 2a
```



6. Is this one list with a thematic break in its second item,
   or two lists separated by a thematic break?



```
* a
* * * * *
* b
```



7. When list markers change from numbers to bullets, do we have
   two lists or one? (The Markdown syntax description suggests two,
   but the perl scripts and many other implementations produce one.)



```
1. fee
2. fie
-  foe
-  fum
```



8. What are the precedence rules for the markers of inline structure?
   For example, is the following a valid link, or does the code span
   take precedence ?



```
[a backtick (`)](/url) and [another backtick (`)](/url).
```



9. What are the precedence rules for markers of emphasis and strong
   emphasis? For example, how should the following be parsed?



```
*foo *bar* baz*
```



10. What are the precedence rules between block-level and inline-level
    structure? For example, how should the following be parsed?



```
- `a long code span can contain a hyphen like this
  - and it can screw things up`
```



11. Can list items include section headings? (`Markdown.pl` does not
    allow this, but does allow blockquotes to include headings.)



```
- # Heading
```



12. Can list items be empty?



```
* a
*
* b
```



13. Can link references be defined inside block quotes or list items?



```
> Blockquote [foo].
>
> [foo]: /url
```



14. If there are multiple definitions for the same reference, which takes
    precedence?



```
[foo]: /url1
[foo]: /url2

[foo][]
```




In the absence of a spec, early implementers consulted `Markdown.pl`
to resolve these ambiguities. But `Markdown.pl` was quite buggy, and
gave manifestly bad results in many cases, so it was not a
satisfactory replacement for a spec.

Because there is no unambiguous spec, implementations have diverged
considerably. As a result, users are often surprised to find that
a document that renders one way on one system (say, a GitHub wiki)
renders differently on another (say, converting to docbook using
pandoc). To make matters worse, because nothing in Markdown counts
as a “syntax error,” the divergence often isn’t discovered right away.

