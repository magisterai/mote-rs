# Links

[Specification](README.md) · [Checklist](../checklists/link.md)

A link is inline content with an external destination. `#link("https://example.com")[Example]`
produces a `link` element with `url = "https://example.com"` and the text child
`Example`, inside an implicit paragraph. The body is required, may be empty, and
may contain the inline formatting defined in earlier chapters. Block content is
invalid in a link body.

The URL is a required string supplied positionally or as `url`. Supplying both forms is invalid. A positional value overrides `properties.url`. More than
one positional argument is invalid. An empty URL is invalid; other URL strings are retained as authored. Mote does not fetch a link destination during generation.

`title`, `tooltip`, and `target` are optional string metadata. They are retained
in the model; they do not currently change rendered appearance, screen tips, or
which window opens. `properties` follows the existing attribute override rules.
Use nested spans to format link text. Internal bookmarks and cross references
are not defined by this chapter; a fragment in a complete URL is part of that external URL. Markdown destinations
starting with `#` request an internal reference and are rejected until that
language feature is defined.

Markdown `[Example](https://example.com "Site")` produces the same element with
an additional `title = "Site"`. Destinations may be enclosed in angle brackets;
parentheses in an unquoted destination must balance. Titles may be enclosed in
single quotes, double quotes, or parentheses. A non-delimited trailing title is
also retained. A malformed Markdown destination remains literal text.

`<https://example.com>` produces linked URL text. `<reader@example.com>` produces
that label with `url = "mailto:reader@example.com"`. Bare `http://` and `https://`
URLs are also recognized, stopping at whitespace, angle brackets, or a double
quote; final `. , ; : ! ?` punctuation is excluded from the link. Literal code
content does not recognize links.

DOCX uses the Hyperlink character style; native output defaults to blue
`0563C1` and underlined text. Nested spans can override these defaults. Wrapped
links retain a destination for each visible fragment in PDF; PNG retains only
the visual text. Format-specific details are tracked in the output tests.
