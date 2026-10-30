# Slide

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md)

- [x] Block body required; an empty body creates an empty slide and a warning.
- [x] Positioned boxes/shapes/lines cannot mix with loose content in one slide.
- [x] `name`: text/number normalization; `notes`: speaker-note text.
- [x] `background`: color or `color`, `src`, `fit` property table; replaces the
  complete deck background rather than merging image and color fields.
- [x] `footer`, `date`, `slide_number`: slide overrides, including false to suppress.
- [ ] `layout`: trimmed name; built-in/custom resolution and typed slots.
- [x] `properties` overrides, retained metadata and Lua construction.
- [ ] Implicit top-level Markdown slide separators and automatic loose layout.

Initial scope is explicit slides with positioned text. Automatic layout, named
layouts, notes and footer fields need their own focused implementation and
captures. Master/layout inheritance, hidden slides, transitions, timing,
animation and extension properties remain inventoried and unsupported unless
mapped above. DOCX treatment and native/PPTX output require separate evidence;
Editable positioned text and solid slide backgrounds are covered by
[package captures](../../tests/fixtures/presentation-text/README.md). Positioned
image boxes also have [package and native captures](../../tests/fixtures/presentation-images/README.md).
Native PDF/PNG uses the same text resolution, wrapping and fitting. PowerPoint
review remains pending.

The [language definition](../specification/presentations.md) and
[model tests](../../tests/presentations.rs) cover checked contracts.
