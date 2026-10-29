# Link

[Directive index](README.md) · [Standard properties](attributes/links.md)

## Content and attributes

- [x] `url`: required string, positional or named; supplying both is invalid; positional value overrides `properties.url`.
- [x] One required inline body; nested text formatting remains available.
- [x] `title`, `tooltip`, `target`: string metadata, retained without output behavior.
- [x] `properties`: standard flattened property overrides.
- [x] Markdown inline links with optional title, angle URL/email links, and bare HTTP(S) links.
- [x] External destinations in DOCX and PDF; link text in native PNG.
- [ ] Internal bookmarks, visited-state customization, document locations and target frames.

The complete Word and DrawingML property inventory is linked above; unspecified
standard fields do not introduce new Mote attributes. See also [run properties](attributes/word-runs.md).

## Output limits and evidence

Internal anchors need their own language definition. Link metadata does not imply
screen-tip or target-frame output. Office rendering remains unreviewed.

[Language definition](../specification/links.md), [evaluation and output tests](../../tests/links.rs), and three frozen model/package captures cover these entries.
