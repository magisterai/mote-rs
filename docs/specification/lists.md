# Lists

[Specification index](README.md) · [Inline text](inline-text.md)

Status: initial draft. This chapter defines basic lists, their items, and nested
list structure. It builds on blocks, inline content, and paragraph normalization.
Numbering appearance, indentation distances, task lists, and list styling will
be specified separately.

## 1. Lists and items

A **list** is a block containing a sequence of **items**. An unordered list
requests bullet-like markers; an ordered list requests numbering. An item is a
block belonging to a list and can contain text and other blocks.

The `list` directive creates an unordered list. Its body contains `item`
directives, with optional whitespace between them. Each `item` body is evaluated
as block content: ordinary text becomes paragraphs, blank lines separate those
paragraphs, and explicit blocks remain blocks.

Both directives require exactly one body, which may be empty. They accept no
positional arguments. No named attributes are needed for the first example:

```mote
#list[
  #item[Read the report.]
  #item[Check the totals.]
]
```

The result is one list with two items. Each item contains one paragraph.
Indentation between the item directives separates their source and does not
become list content.

The list body accepts only items and separating whitespace. To include a
paragraph, heading, or another list, put it inside an item. Text directly inside
the list body is invalid.

## 2. Ordered lists

`ordered_list` has the same body rules as `list` and requests numbered items.
Its optional `start` attribute is the requested starting number: a positive
integer, supplied as a number or an integer string. Omission leaves it
unspecified in the model. The following list requests numbering starting at 3:

```mote
#ordered_list(start = 3)[
  #item[Read the report.]
  #item[Check the totals.]
]
```

`start` is an attribute of the list, not text inserted into its item bodies.
This chapter does not prescribe the font, punctuation, or physical position of
the rendered numbers.

## 3. Item content and nesting

An item can contain multiple paragraphs. Inline formatting works within each
paragraph using the forms already defined:

```mote
#list[
  #item[#paragraph[Read #emphasis[carefully].]
        #paragraph[Check #strong[every] total.]]
]
```

A **nested list** belongs to an item of its enclosing list. Put the nested list
inside that item's body, after any introductory text or blocks:

```mote
#list[
  #item[Prepare the report.
    #ordered_list[
      #item[Check the totals.]
      #item[Write the summary.]
    ]
  ]
  #item[Share the report.]
]
```

The outer list has two items. Its first item contains a paragraph followed by
an ordered list with two items. Source indentation makes the nesting readable;
the bracketed bodies determine the structure.

## 4. Markdown list items

At the beginning of a line, `-` or `+` followed by a space or tab starts an
unordered item. A sequence of ASCII digits followed by `.` and a space or tab
starts an ordered item. The marker is syntax and is not part of the item's text.
This chapter uses hyphens for unordered Markdown items.

```mote
- Read the report.
- Check the totals.
```

These items are grouped into one unordered list. Markdown item content retains
its inline children directly in the model; an explicit `item` body wraps bare
text in paragraphs. Both forms describe the same item text and hierarchy.
Explicit paragraph directives inside a Markdown item remain paragraph blocks.

An item's source marker is retained in its `marker` attribute. For an ordered
item, the digits and dot are retained exactly. Markdown does not synthesize a
list-level `start` attribute from the first marker:

```mote
3. Read the report.
4. Check the totals.
```

The model records one ordered list with item markers `"3."` and `"4."`.
Use the explicit `start` attribute when expressing a list-level starting number.

Consecutive items at document level are grouped by unordered or ordered kind.
Changing kind starts another list. Blank lines alone do not end the group;
intervening document text or another block does.

### Continuation and indentation

A following nonblank line belongs to the current item when its first content
character is farther right than that item's marker. Leading spaces and tabs
establish this source position; they are not physical indentation values.
Examples use spaces. A line at the marker's position or farther left ends the
item. This rule also applies when blank lines intervene.

An indented text continuation contributes a space between the text lines. A
blank line in a continuation contributes a paragraph break in the item's text.
An indented item marker introduces a nested item:

```mote
- Prepare the report
  carefully.
  - Check the totals.
- Share the report.
```

“carefully.” continues the first item. “Check the totals.” belongs to its nested
list. “Share the report.” starts the next outer item. Explicit directives can
express the same hierarchy without relying on indentation.

Markdown items can also appear directly in an explicit list body:

```mote
#list[
- Read the report.
- Check the totals.
]
```

Here `list` supplies the container; its body contributes the two items directly.
This direct Markdown form is limited to flat items in this chapter. For nested
Markdown content inside an explicit list, place it inside an `item` body, where
the block-content rules group the nested items. A nested explicit list container
is also supported.

## 5. Standalone items and markers

An `item` directive outside an explicit list is grouped with adjacent items by
the surrounding block-content rules. Its optional `marker` attribute is a
string. For grouping, an omitted marker or a marker beginning with a non-digit
selects an unordered list; a marker beginning with an ASCII digit selects an
ordered list.

```mote
#item(marker = "-")[Read the report.]
#item(marker = "-")[Check the totals.]
```

These become two items in one unordered list. Within an explicit `list` or
`ordered_list`, the enclosing directive supplies the container kind.
Marker values record authoring intent; marker layout remains a later concern.

## 6. Empty and invalid uses

An empty item is permitted, and remains an item with no content:

```mote
#list[#item[]]
```

An empty list body is also permitted. A missing body is an error.
The following examples intentionally fail evaluation.

A list cannot contain bare text directly:

```mote
#list[Read the report.]
```

A nested list must belong to an item, rather than occur directly in another
list's item-only body:

```mote
#list[#list[#item[Nested]]]
```

An explicit starting number must be positive:

```mote
#ordered_list(start = 0)[#item[Text]]
```

An inline paragraph body cannot contain a list block:

```mote
#paragraph[#list[#item[Text]]]
```

Continue with [Block quotes](block-quotes.md) for a container whose body follows
ordinary block-content rules.
