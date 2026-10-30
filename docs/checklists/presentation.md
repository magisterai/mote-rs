# Presentation

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md)

The complete deck, slide, master, layout, notes, timing, transition and theme
families are inventoried, including unsupported properties and value domains.
The first implementation covers explicit decks and positioned text; broader
features remain separate steps within the established compatibility baseline.

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
Output and Office review are pending; model acceptance alone does not establish
their effects.
