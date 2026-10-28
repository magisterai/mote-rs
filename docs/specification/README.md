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

Define a concept before using it in an example. Examples may use concepts defined
earlier in the reading order or immediately before the example. Every successful
Mote example has an expected document model; deliberate invalid examples identify
the rule they violate. Implementation deferrals do not replace definitions.

Later chapters will introduce bindings, data loading, additional formatting,
document configuration, scripting, and layout. Their syntax is not used in the
introductory examples.

The [reference library](../reference/README.md) contains the external file-format
and language standards used alongside this specification.
