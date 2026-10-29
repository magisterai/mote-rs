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
   nesting, whitespace, and explicit line breaks.
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
    margins, equal-width columns, repeated declarations, and model inspection.

Define a concept before using it in an example. Examples may use concepts defined
earlier in the reading order or immediately before the example. Every successful
Mote example has an expected document model; deliberate invalid examples identify
the rule they violate. Implementation deferrals do not replace definitions.

Later chapters will introduce bindings, data loading, additional formatting,
sections, scripting, and layout. Their syntax is not used in the
introductory examples.

The [directive checklists](../checklists/README.md) record planned attributes and
current completion separately from this reading order. Create or update the
relevant checklist before extending a directive, then define each new concept
here before demonstrating it.

The [reference library](../reference/README.md) contains the external file-format
and language standards used alongside this specification.
