# GitHub Flavored Markdown Specification

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Topic guide](../../../README.md) · [Publisher](https://github.github.com/gfm/).

23 reading chunks. Headings, examples, tables, mathematical markup, figures, and source anchors are retained. Publisher logos and navigation images are replaced by text; subsection arrows are omitted from headings.

- [GitHub Flavored Markdown Spec](chunks/001-github-flavored-markdown-spec.md)
  - 1 Introduction
  - 1.1 What is GitHub Flavored Markdown?
  - 1.2 What is Markdown?
- [1.3 Why is a spec needed?](chunks/002-1-3-why-is-a-spec-needed.md)
  - 1.4 About this document
  - 2 Preliminaries
  - 2.1 Characters and lines
- [2.2 Tabs](chunks/003-2-2-tabs.md)
  - 2.3 Insecure characters
  - 3 Blocks and inlines
  - 3.1 Precedence
  - 3.2 Container blocks and leaf blocks
  - 4 Leaf blocks
  - 4.1 Thematic breaks
- [4.2 ATX headings](chunks/004-4-2-atx-headings.md)
  - 4.3 Setext headings
- [4.4 Indented code blocks](chunks/005-4-4-indented-code-blocks.md)
  - 4.5 Fenced code blocks
- [4.6 HTML blocks](chunks/006-4-6-html-blocks.md)
- [4.7 Link reference definitions](chunks/007-4-7-link-reference-definitions.md)
- [4.8 Paragraphs](chunks/008-4-8-paragraphs.md)
  - 4.9 Blank lines
  - 4.10 Tables (extension)
- [5 Container blocks](chunks/009-5-container-blocks.md)
  - 5.1 Block quotes
- [5.2 List items](chunks/010-5-2-list-items.md)
- [5.2.1 Motivation](chunks/011-5-2-1-motivation.md)
- [5.3 Task list items (extension)](chunks/012-5-3-task-list-items-extension.md)
  - 5.4 Lists
- [6 Inlines](chunks/013-6-inlines.md)
  - 6.1 Backslash escapes
  - 6.2 Entity and numeric character references
- [6.3 Code spans](chunks/014-6-3-code-spans.md)
  - 6.4 Emphasis and strong emphasis
- [Continuation 15](chunks/015-continuation-15.md)
- [Continuation 16](chunks/016-continuation-16.md)
- [6.5 Strikethrough (extension)](chunks/017-6-5-strikethrough-extension.md)
  - 6.6 Links
- [Continuation 18](chunks/018-continuation-18.md)
- [6.7 Images](chunks/019-6-7-images.md)
- [6.8 Autolinks](chunks/020-6-8-autolinks.md)
  - 6.9 Autolinks (extension)
- [6.10 Raw HTML](chunks/021-6-10-raw-html.md)
- [6.11 Disallowed Raw HTML (extension)](chunks/022-6-11-disallowed-raw-html-extension.md)
  - 6.12 Hard line breaks
  - 6.13 Soft line breaks
  - 6.14 Textual content
  - Appendix: A parsing strategy
  - Overview
- [Phase 1: block structure](chunks/023-phase-1-block-structure.md)
  - Phase 2: inline structure
  - An algorithm for parsing nested emphasis and links
  - *look for link or image*
  - *process emphasis*
