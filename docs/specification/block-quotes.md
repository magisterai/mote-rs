# Block quotes

[Specification index](README.md) · [Lists](lists.md)

Status: initial draft. This chapter adds a basic container for quoted material.
It uses the block-content and inline-content rules already defined.

## 1. Quoted block content

A **block quote** groups content presented as a quotation. The `blockquote`
directive creates a block with exactly one required body. That body accepts
block content: ordinary text becomes paragraphs, and headings, lists, and other
explicit blocks retain their structure. The body may be empty.

`blockquote` takes no positional arguments. No named attributes are defined
here. The model records the quotation's structure; indentation, borders, and
other visual treatment belong to later formatting rules.

```mote
#blockquote[Read #emphasis[carefully].]
```

The quote contains one paragraph, including an emphasis span. A blank line
separates paragraphs inside the quote just as it does at document level:

```mote
#blockquote[First paragraph.

Second paragraph.]
```

## 2. Markdown quote lines

At the beginning of a line, `>` introduces a block quote. Spaces or tabs after
the marker are removed. The rest of that source line supplies its content,
including inline formatting and directive calls.

```mote
> Read _carefully_.
```

This has the same evaluated content as the first example. Each marked source
line creates its own quote block. Adjacent quote lines are not merged:

```mote
> First quote.
> Second quote.
```

The result has two quote blocks, each containing one paragraph. Use one
`blockquote` body to group multiple paragraphs in a single quote. Recursive
Markdown quote markers are outside this chapter; nested quotation can be
expressed with nested `blockquote` directives.

## 3. Lists inside quotes

The list container and item rules apply unchanged inside a quote. The following
quote contains an introductory paragraph followed by a list:

```mote
#blockquote[Review these points.
  #list[
    #item[Check the totals.]
    #item[Read the summary.]
  ]
]
```

## 4. Invalid uses

These examples intentionally fail evaluation.

A block quote requires a body:

```mote
#blockquote()
```

A paragraph accepts only inline content, so it cannot contain a block quote:

```mote
#paragraph[#blockquote[Quoted text.]]
```

Continue with [Formatting attributes](formatting-attributes.md) for the attribute
contracts of the directives introduced so far.
