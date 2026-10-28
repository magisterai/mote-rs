# CommonMark Specification

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Topic guide](../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

23 reading chunks. Headings, examples, tables, mathematical markup, figures, and source anchors are retained. Publisher logos and navigation images are replaced by text; subsection arrows are omitted from headings.

- [CommonMark Spec](chunks/001-commonmark-spec.md)
  - 1 Introduction
  - 1.1 What is Markdown?
  - 1.2 Why is a spec needed?
- [1.3 About this document](chunks/002-1-3-about-this-document.md)
  - 2 Preliminaries
  - 2.1 Characters and lines
  - 2.2 Tabs
- [2.3 Insecure characters](chunks/003-2-3-insecure-characters.md)
  - 2.4 Backslash escapes
  - 2.5 Entity and numeric character references
- [3 Blocks and inlines](chunks/004-3-blocks-and-inlines.md)
  - 3.1 Precedence
  - 3.2 Container blocks and leaf blocks
  - 4 Leaf blocks
  - 4.1 Thematic breaks
  - 4.2 ATX headings
- [4.3 Setext headings](chunks/005-4-3-setext-headings.md)
- [4.4 Indented code blocks](chunks/006-4-4-indented-code-blocks.md)
  - 4.5 Fenced code blocks
- [4.6 HTML blocks](chunks/007-4-6-html-blocks.md)
- [Continuation 8](chunks/008-continuation-8.md)
- [4.7 Link reference definitions](chunks/009-4-7-link-reference-definitions.md)
- [4.8 Paragraphs](chunks/010-4-8-paragraphs.md)
  - 4.9 Blank lines
  - 5 Container blocks
  - 5.1 Block quotes
- [5.2 List items](chunks/011-5-2-list-items.md)
- [Continuation 12](chunks/012-continuation-12.md)
- [5.2.1 Motivation](chunks/013-5-2-1-motivation.md)
  - 5.3 Lists
- [6 Inlines](chunks/014-6-inlines.md)
  - 6.1 Code spans
- [6.2 Emphasis and strong emphasis](chunks/015-6-2-emphasis-and-strong-emphasis.md)
- [Continuation 16](chunks/016-continuation-16.md)
- [Continuation 17](chunks/017-continuation-17.md)
- [6.3 Links](chunks/018-6-3-links.md)
- [Continuation 19](chunks/019-continuation-19.md)
- [6.4 Images](chunks/020-6-4-images.md)
- [6.5 Autolinks](chunks/021-6-5-autolinks.md)
  - 6.6 Raw HTML
- [6.7 Hard line breaks](chunks/022-6-7-hard-line-breaks.md)
  - 6.8 Soft line breaks
  - 6.9 Textual content
  - Appendix: A parsing strategy
  - Overview
- [Phase 1: block structure](chunks/023-phase-1-block-structure.md)
  - Phase 2: inline structure
  - An algorithm for parsing nested emphasis and links
  - *look for link or image*
  - *process emphasis*
