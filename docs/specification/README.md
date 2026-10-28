# Mote specification

This specification defines Mote's source language, document model, and behaviour.
It is developed progressively: each chapter introduces a small set of concepts
and demonstrates their meaning with examples.

Status: initial draft.

## Reading order

1. [Fundamentals](fundamentals.md): Markdown, directives, arguments, attributes,
   content bodies, helpers, document values, data, evaluation, and output.
2. [Syntax and values](syntax-and-values.md): core call syntax, identifiers,
   literals, property objects, helper expressions, escapes, and errors.

Subsequent chapters will extend the syntax and values coverage and define scope,
defaults, individual directives, layout, and diagnostics. These first chapters
introduce the language without specifying every detail.

The [reference library](../reference/README.md) contains the external file-format
and language standards used alongside this specification.
