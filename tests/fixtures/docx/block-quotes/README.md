# Block quotes reference

Covers Markdown quote lines, an explicit quote with nested paragraphs/lists/quotes,
inline formatting, empty quotes, named styles, borders, fill and direct geometry.
Outside table cells, each quote emits one Word paragraph. Nested block content
flattens to inline runs without inserted separators or list counters. Formatting
on nested blocks remains unsupported by this writer and produces explicit errors.

The capture reported alignment, indentation and spacing as unsupported, although
its XML contains those direct paragraph properties. The comparison retains them.
Omitted style selects BlockQuote; an empty style suppresses that reference.
Every package part and the model are compared. Word review remains unreviewed.
