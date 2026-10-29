# Anchors and references

[Specification](README.md) · [Bookmarks](../checklists/bookmark.md) ·
[Labels](../checklists/label.md) · [References](../checklists/ref.md)

An anchor names a document location. `#bookmark("name")[inline content]` preserves
its content and names its range; the inline body is optional. `name` may instead
be named. Optional `kind` and `title` strings provide metadata. An empty bookmark
name is invalid.

`#label("name")` declares an invisible anchor with no body. Its name must be a
nonempty string, supplied positionally or by name. Optional `kind` and `title`
are strings; `number` is a string or number. Label names must be unique across the document, including running stories.
References require an exact declared label name; bookmarks do not satisfy this
requirement. Inherited running stories count as one declaration. Output metadata
uses trimmed names, so avoid names that differ only in surrounding whitespace.
A standalone label attaches to the following block without adding a visible
paragraph. Its title is inferred from that block's text if no title was supplied.
A bookmark's default title is its enclosed text.

`#ref("target")` refers to a label. It has no body. The named `target` form also
accepts an object with `label` or its alias `name`; `label` takes precedence.
Targets may appear later in the document. `kind` selects `title`, `number`, `page`,
or `text`. `display` accepts those values plus `auto`; a non-auto display overrides
kind. The default is auto, which selects the target title when available.

A numbered reference uses number metadata first. Otherwise an explicit `fallback`
string takes precedence over title metadata. If no applicable text is available,
the normalized target name remains visible. Page references resolve the displayed
page number after layout, using the fallback or `?` while the page is unknown.

Markdown `[visible text](#target)` produces an internal reference with its authored
inline text and formatting. That text takes precedence over inferred title metadata.
The explicit `#link` directive retains its external-URL meaning.

All three directives accept a `properties` object using the ordinary flattened
override rules. Positional names/targets override the corresponding property entry;
a primary argument cannot be supplied both positionally and directly by name.
Unknown retained metadata does not imply a rendering effect.

```mote
#paragraph[Continue to #ref("details") on page #ref("details", kind = "page").]
#label("details", title = "Detailed results")
#heading(1)[Results]
#paragraph[A #bookmark("term")[named term] remains ordinary visible text.]
```

The model contains a paragraph with two inline references, a paragraph containing
an invisible label, a heading, and a paragraph with an inline bookmark. References
and labels stay explicit in the model; page numbers are not evaluated there.
Editable reference targets use labels; authored Word bookmarks occupy a separate
namespace. Output behavior and remaining limits are recorded in the checklists.
