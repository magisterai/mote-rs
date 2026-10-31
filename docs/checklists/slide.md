# Slide

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md)

- [x] Block body required; an empty body creates an empty slide and a warning.
- [x] Positioned boxes/shapes/lines cannot mix with loose content in one slide.
- [x] `name`: text/number normalization; `notes`: speaker-note text.
- [x] `background`: color or `color`, `src`, `fit` property table; replaces the
  complete deck background rather than merging image and color fields.
- [x] `footer`, `date`, `slide_number`: slide overrides, including false to suppress.
- [x] `layout`: trimmed name, built-in/custom resolution and typed slots; see
  the [layout checklist](layout.md).
- [x] `properties` overrides, retained metadata and Lua construction.
- [x] Automatic loose-content layout: a leading heading, grouped text, separate
  image/table frames, shared fitting and page-size-aware geometry.
- [x] Implicit top-level Markdown slide separators and heading boundaries in
  presentation mode; explicit slides mixed with implicit segments.

Output supports explicit slides with positioned text, pictures, tables, shapes,
connectors and charts, plus automatic text, image, table and chart layout. Built-in and custom named layouts are supported.
Speaker notes are plain PPTX metadata. Footer text, dates and slide numbers render
in editable and native output, with independent suppression on each slide.
Master/layout inheritance, hidden slides, transitions, timing, animation and
extension properties remain inventoried and unsupported unless mapped above.
PowerPoint review remains pending.

The [language definition](../specification/presentations.md) and
[model tests](../../tests/presentations.rs) cover checked contracts.

Background-image output inventory: source-relative loading, inherited deck
backgrounds, whole-value slide overrides, color fallback, contain/cover/stretch
placement, clipping and foreground layering. Missing sources must retain the
background color and warn. These effects have
[editable and native captures](../../tests/fixtures/presentation-backgrounds/README.md);
PowerPoint review remains pending.

Footer and notes output inventory: inherited footer text, `footer=true` using
that text, fixed and automatic dates, one-based slide-number fields, independent
false overrides, field geometry/alignment, editable field identifiers and cached
text, plain speaker-note text, slide/notes/master relationships and notes counts.
Automatic dates use the local calendar at generation time. Notes remain package
metadata and do not paint onto slides.
[Editable and native captures](../../tests/fixtures/presentation-fields/README.md)
cover these effects, including independent overrides and combined slide content.
PowerPoint review remains pending.

Automatic layout has [editable and native captures](../../tests/fixtures/presentation-automatic/README.md) for standard, wide and custom canvases. PowerPoint review remains pending.

[Markdown deck captures](../../tests/fixtures/presentation-markdown/README.md) cover implicit boundaries, explicit/implicit mixtures and document-mode isolation.
