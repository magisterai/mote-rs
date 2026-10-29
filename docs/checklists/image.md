# Image

[Directive index](README.md) · [Standard properties](attributes/images.md)

[Language definition](../specification/images.md) · [Tests](../../tests/images.rs) ·
[Captured outputs](../../tests/fixtures/images/README.md)

## Content and attributes

- [x] Required nonempty string `src`, positional or named; no body.
- [x] `name`, `alt`, `description`, `title`, `style`, `link`: strings; fallback metadata precedence is specified separately.
- [x] `width`, `height`: normalized lengths; natural 96-DPI dimensions and aspect-ratio inference.
- [x] `alignment`: left, center, right, justify; `spacing.before` and `spacing.after`.
- [x] `placement`: width/height, align; horizontal alignment and relative-to column fallback.
- [x] Retained placement fields: x/y, margin, type, float, wrap, anchor, allow_overlap, behind_text, layout_in_cell, locked, relative_height; axis offset, relative_to, alignment.
- [x] `crop`: left/top/right/bottom percentage values.
- [x] `rotation`: degrees; `flip_horizontal`, `flip_vertical`: booleans.
- [x] `border`: width, color, opacity, style (single, solid, dashed, dotted, none).
- [x] `effects`: brightness, contrast, transparency percentages; grayscale boolean.
- [x] `properties`: flattened overrides and retained extra metadata.
- [x] Markdown image syntax; source-relative and custom resolver assets.
- [x] PNG, JPEG and first-frame GIF assets; explicit missing/unsupported asset diagnostics.

## Limits

The linked inventory contains every reachable picture/placement property and
value domain. Floating anchors, wrapping exclusion, additional blip effects,
geometry presets and three-dimensional transforms are not enabled merely by
retaining their metadata. Native borders currently paint a solid opaque stroke;
DOCX retains authored dash and opacity. Images inside flattened quote/list text
use a visible native placeholder. Office visual review remains unreviewed.
