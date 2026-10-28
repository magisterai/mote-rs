# GitHub Flavored Markdown Spec

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-1-3-why-is-a-spec-needed.md)

# GitHub Flavored Markdown Spec

Version 0.29-gfm (2019-04-06)

This formal specification is based on the
[CommonMark Spec](http://spec.commonmark.org) by
[John MacFarlane](http://github.com/jgm) and licensed under
[Creative Commons BY-SA](http://creativecommons.org/licenses/by-sa/4.0/)

<a id="watermark"></a>

<a id="TOC"></a>

- [1Introduction](001-github-flavored-markdown-spec.md#introduction)
  - [1.1What is GitHub Flavored Markdown?](001-github-flavored-markdown-spec.md#what-is-github-flavored-markdown-)
  - [1.2What is Markdown?](001-github-flavored-markdown-spec.md#what-is-markdown-)
  - [1.3Why is a spec needed?](002-1-3-why-is-a-spec-needed.md#why-is-a-spec-needed-)
  - [1.4About this document](002-1-3-why-is-a-spec-needed.md#about-this-document)
- [2Preliminaries](002-1-3-why-is-a-spec-needed.md#preliminaries)
  - [2.1Characters and lines](002-1-3-why-is-a-spec-needed.md#characters-and-lines)
  - [2.2Tabs](003-2-2-tabs.md#tabs)
  - [2.3Insecure characters](003-2-2-tabs.md#insecure-characters)
- [3Blocks and inlines](003-2-2-tabs.md#blocks-and-inlines)
  - [3.1Precedence](003-2-2-tabs.md#precedence)
  - [3.2Container blocks and leaf blocks](003-2-2-tabs.md#container-blocks-and-leaf-blocks)
- [4Leaf blocks](003-2-2-tabs.md#leaf-blocks)
  - [4.1Thematic breaks](003-2-2-tabs.md#thematic-breaks)
  - [4.2ATX headings](004-4-2-atx-headings.md#atx-headings)
  - [4.3Setext headings](004-4-2-atx-headings.md#setext-headings)
  - [4.4Indented code blocks](005-4-4-indented-code-blocks.md#indented-code-blocks)
  - [4.5Fenced code blocks](005-4-4-indented-code-blocks.md#fenced-code-blocks)
  - [4.6HTML blocks](006-4-6-html-blocks.md#html-blocks)
  - [4.7Link reference definitions](007-4-7-link-reference-definitions.md#link-reference-definitions)
  - [4.8Paragraphs](008-4-8-paragraphs.md#paragraphs)
  - [4.9Blank lines](008-4-8-paragraphs.md#blank-lines)
  - [4.10Tables (extension)](008-4-8-paragraphs.md#tables-extension-)
- [5Container blocks](003-2-2-tabs.md#container-blocks)
  - [5.1Block quotes](009-5-container-blocks.md#block-quotes)
  - [5.2List items](010-5-2-list-items.md#list-items)
  - [5.3Task list items (extension)](012-5-3-task-list-items-extension.md#task-list-items-extension-)
  - [5.4Lists](012-5-3-task-list-items-extension.md#lists)
- [6Inlines](013-6-inlines.md#inlines)
  - [6.1Backslash escapes](013-6-inlines.md#backslash-escapes)
  - [6.2Entity and numeric character references](013-6-inlines.md#entity-and-numeric-character-references)
  - [6.3Code spans](014-6-3-code-spans.md#code-spans)
  - [6.4Emphasis and strong emphasis](014-6-3-code-spans.md#emphasis-and-strong-emphasis)
  - [6.5Strikethrough (extension)](017-6-5-strikethrough-extension.md#strikethrough-extension-)
  - [6.6Links](017-6-5-strikethrough-extension.md#links)
  - [6.7Images](019-6-7-images.md#images)
  - [6.8Autolinks](020-6-8-autolinks.md#autolinks)
  - [6.9Autolinks (extension)](020-6-8-autolinks.md#autolinks-extension-)
  - [6.10Raw HTML](021-6-10-raw-html.md#raw-html)
  - [6.11Disallowed Raw HTML (extension)](022-6-11-disallowed-raw-html-extension.md#disallowed-raw-html-extension-)
  - [6.12Hard line breaks](022-6-11-disallowed-raw-html-extension.md#hard-line-breaks)
  - [6.13Soft line breaks](022-6-11-disallowed-raw-html-extension.md#soft-line-breaks)
  - [6.14Textual content](022-6-11-disallowed-raw-html-extension.md#textual-content)
- [Appendix: A parsing strategy](022-6-11-disallowed-raw-html-extension.md#appendix-a-parsing-strategy)
  - [Overview](022-6-11-disallowed-raw-html-extension.md#overview)
  - [Phase 1: block structure](023-phase-1-block-structure.md#phase-1-block-structure)
  - [Phase 2: inline structure](023-phase-1-block-structure.md#phase-2-inline-structure)

<a id="introduction"></a>

# 1 Introduction

<a id="what-is-github-flavored-markdown-"></a>

## 1.1 What is GitHub Flavored Markdown?

GitHub Flavored Markdown, often shortened as GFM, is the dialect of Markdown
that is currently supported for user content on GitHub.com and GitHub
Enterprise.

This formal specification, based on the CommonMark Spec, defines the syntax and
semantics of this dialect.

GFM is a strict superset of CommonMark. All the features which are supported in
GitHub user content and that are not specified on the original CommonMark Spec
are hence known as **extensions**, and highlighted as such.

While GFM supports a wide range of inputs, it’s worth noting that GitHub.com
and GitHub Enterprise perform additional post-processing and sanitization after
GFM is converted to HTML to ensure security and consistency of the website.

<a id="what-is-markdown-"></a>

## 1.2 What is Markdown?

Markdown is a plain text format for writing structured documents,
based on conventions for indicating formatting in email
and usenet posts. It was developed by John Gruber (with
help from Aaron Swartz) and released in 2004 in the form of a
[syntax description](http://daringfireball.net/projects/markdown/syntax)
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
> ([http://daringfireball.net/projects/markdown/](http://daringfireball.net/projects/markdown/))

The point can be illustrated by comparing a sample of
[AsciiDoc](http://www.methods.co.nz/asciidoc/) with
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

