# Presentation

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md)

The complete deck, slide, master, layout, notes, timing, transition and theme
families are inventoried, including unsupported properties and value domains.
Output covers explicit decks with positioned text and pictures. Broader features
remain separate steps within the established compatibility baseline.

- [x] Named declaration arguments, no body, no visible element; repeated
  declarations merge normalized properties and size fields.
- [x] `size`: wide/widescreen/16:9, standard/4:3, or a length property table.
- [x] `width`, `height`: non-negative lengths; direct dimensions override size
  dimensions while retained declaration values remain inspectable.
- [x] `background`: color or property table with `color`, `src` and
  `fit` (stretch, cover, contain); retained extra fields.
- [x] `footer`, `date`: text or boolean; `slide_number`: boolean normalization.
- [x] `theme.name`; `theme.colors`: background/text aliases, dark1/light1,
  dark2/light2, accent1–accent6, hyperlink/followed_hyperlink.
- [x] `theme.fonts`: heading/body and major/minor; declared font aliases.
- [x] `properties` overrides; unknown metadata; Lua declaration constructor.

Slide/master ownership, printing, embedded fonts, custom shows, slide ordering,
notes masters, accessibility metadata, media, transitions, animation, zoom and
collaboration extensions remain visible in the inventories. No additional Mote
names are assigned to unmapped standard fields. The [language definition](../specification/presentations.md) and
[model captures](../../tests/fixtures/presentations/README.md) cover these contracts.
Editable packages support explicit slides, page dimensions, theme colors/fonts
and solid backgrounds. Background images are supported; footer fields still fail explicitly.
See [package captures](../../tests/fixtures/presentation-text/README.md). Native PDF/PNG supports the same initial subset and uses explicit slide geometry.
Native output needs at least one explicit slide. PowerPoint review remains
pending; model acceptance alone does not establish output effects.

Background-image output inventory: source-relative loading, inherited deck
backgrounds, whole-value slide overrides, color fallback, contain/cover/stretch
placement, clipping and foreground layering. Missing sources must retain the
background color and warn. These effects have [editable and native captures](../../tests/fixtures/presentation-backgrounds/README.md); PowerPoint review remains pending.

Footer and notes output inventory: inherited footer text, `footer=true` using
that text, fixed and automatic dates, one-based slide-number fields, independent
false overrides, field geometry/alignment, editable field identifiers and cached
text, plain speaker-note text, slide/notes/master relationships and notes counts.
Automatic dates use the local calendar at generation time. Notes remain package
metadata and do not paint onto slides. Focused output captures are pending.
