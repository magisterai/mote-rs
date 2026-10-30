# Mote specification

This specification defines Mote's source language, document model, and behaviour.
It is developed progressively: each chapter introduces a small set of concepts
and demonstrates their meaning with examples.

Status: initial draft.

## Reading order

1. [Fundamentals](fundamentals.md): content, directives, arguments, attributes,
   nesting, Markdown equivalents, and a first length helper.
2. [Syntax and values](syntax-and-values.md): core call syntax, identifiers,
   literals, property objects, helper expressions, escapes, and errors.
3. [Basic content](basic-content.md): paragraphs, headings, strong text, body
   rules, omitted attributes, length helpers, alignment, spacing, and invalid uses.
4. [Inline text](inline-text.md): emphasis, literal and evaluated code content,
   nesting, whitespace, explicit line breaks, and tabs.
5. [Lists](lists.md): list and item bodies, ordered lists, markers, continuation,
   nesting, and Markdown forms.
6. [Block quotes](block-quotes.md): quoted block content, Markdown quote lines,
   and lists within quotes.
7. [Formatting attributes](formatting-attributes.md): shared value rules, property
   overrides, style/font references, paragraph geometry, borders, tabs, heading
   numbering, list/task metadata, and preserved code whitespace.
8. [Spans](spans.md): general inline formatting, normalization, and nested intent.
9. [Styles and fonts](styles-and-fonts.md): declarations, registration, inheritance,
   direct precedence, font aliases, and the resolution API.

10. [Document configuration](document-configuration.md): document-wide page geometry,
    margins, equal-width columns, text defaults, repeated declarations, and model inspection.

11. [Flow breaks](flow-breaks.md): explicit page, column and block line breaks.

12. [Links](links.md): external destinations, Markdown forms, metadata, and inline bodies.

13. [Images](images.md): block media, dimensions, transforms, and asset resolution.

14. [Tables](tables.md): row/cell structure, sizing, merges, borders and Markdown tables.

15. [Sections, headers and footers](sections-and-stories.md): section geometry,
    running stories and page-number fields.

16. [Anchors and references](anchors-and-references.md): named ranges, invisible labels,
    internal links, forward references and displayed page references.

17. [Figures and captions](figures-and-captions.md): grouped content, captions,
    shared numbering sequences and caption references.

18. [Footnotes and endnotes](notes.md): inline note references, block note bodies,
    identifiers and Markdown definitions.

19. [Bindings and property access](bindings.md): typed values, lexical scopes,
    content references and object/array paths.

20. [JSON data](json-data.md): typed resources, source-relative paths and snapshots.

21. [Conditions and iteration](control-flow.md): truthiness, selected branches,
    collection order, local scopes and generated content.

22. [Lua expressions](lua-expressions.md): opt-in expression evaluation, typed
    conversions, indexing, available helpers and the shared execution budget.

23. [Functions](functions.md): lexical definitions, arguments, content bodies,
    return conversions and current scope limits.

24. [Document constructors](document-constructors.md): Lua element values, property
    tables, content arguments, declarations and output semantics.

25. [Shapes and lines](drawings.md): positioned geometry, appearance, shape text,
    and connector markers.

26. [Charts](charts.md): chart data, kinds, dimensions, appearance, legends and labels.

Define a concept before using it in an example. Examples may use concepts defined
earlier in the reading order or immediately before the example. Every successful
Mote example has an expected document model; deliberate invalid examples identify
the rule they violate. Implementation deferrals do not replace definitions.

Later chapters will introduce additional data helpers, formatting,
and layout. Their syntax is not used in the
introductory examples.

The [directive checklists](../checklists/README.md) record planned attributes and
current completion separately from this reading order. Create or update the
relevant checklist before extending a directive, then define each new concept
here before demonstrating it.

The [reference library](../reference/README.md) contains the external file-format
and language standards used alongside this specification.
