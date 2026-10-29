# Sections, headers and footers

[Specification](README.md) · [Section checklist](../checklists/section.md) ·
[Header](../checklists/header.md) · [Footer](../checklists/footer.md) ·
[Page numbers](../checklists/page-number.md)

A section is a sequence of block content with its own page geometry and running
header and footer. Content outside an explicit section belongs to an implicit
section. Document text defaults, font declarations and styles remain shared.

`#section(page = {...}, margin = {...}, columns = {...})[content]` requires one
block body and named arguments. The geometry objects use the fields defined in
[Document configuration](document-configuration.md). A section inherits document
geometry and overrides the fields it supplies. Its authored arguments remain in
`properties`; its resolved geometry is stored separately.

`properties.start` selects `next_page`, `continuous`, `even_page`, `odd_page`, or
`next_column`. `properties.vertical_alignment` selects `top`, `center`, `bottom`,
or `justified`; `both` normalizes to `justified`. These properties must use the
nested object. Unknown nested entries remain metadata and do not imply output
support. A continuous section's following implicit section also starts continuously.

`page_numbering` accepts `restart` (boolean), `start` (positive integer), and
`format` (a retained format string). Restarting without a start value selects 1.
A start value supplies the section's first displayed page number. A page's
physical position and displayed number are distinct.

`#header[content]` and `#footer[content]` each require one block body. Their named
`height` and `distance` arguments are nonnegative lengths. `height` reserves space;
`distance` supplies the distance from the page edge. These values can also be
supplied inside `properties`; direct arguments take precedence. Other nested
entries remain metadata. A document or section `margin.header`/`margin.footer`
value takes precedence over the corresponding story distance.

At document level, a declaration supplies the default story for subsequent
content. Changing a story after content begins starts a new implicit section;
earlier sections keep their stories. Inside an explicit section, declarations
replace that section's inherited story. An empty body clears it. These declarations
produce no body content. Separate first-page or odd/even stories are not defined.

`#page_number()` is an inline field with no body or positional arguments. Its
`kind` is `current` (the default) or `total` (the document's physical page count).
`format` retains a field format string; output support is recorded separately in
the checklist. The evaluated model preserves the field, rather than replacing it
with a page number before pagination.

```mote
#header(height = pt(24), distance = pt(18))[Quarterly report]
#footer(distance = pt(18))[Page #page_number() of #page_number(kind = "total")]

#paragraph[The report begins here.]
```

The body model contains one paragraph. Its implicit section has a header paragraph
and a footer paragraph containing current/total inline fields. Header and footer
properties are separate from body attributes. Multiple bodies, positional story
arguments, negative lengths, unknown outer story arguments and invalid `kind`
values are errors.
