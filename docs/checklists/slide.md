# Slide

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md)

- [ ] Block body required; an empty body creates an empty slide and a warning.
- [ ] Positioned boxes/shapes/lines cannot mix with loose content in one slide.
- [ ] `name`: text/number normalization; `notes`: speaker-note text.
- [ ] `background`: color or `color`, `src`, `fit` property table; replaces the
  complete deck background rather than merging image and color fields.
- [ ] `footer`, `date`, `slide_number`: slide overrides, including false to suppress.
- [ ] `layout`: trimmed name; built-in/custom resolution and typed slots.
- [ ] `properties` overrides, retained metadata and Lua construction.
- [ ] Implicit top-level Markdown slide separators and automatic loose layout.

Initial scope is explicit slides with positioned text. Automatic layout, named
layouts, notes and footer fields need their own focused implementation and
captures. Master/layout inheritance, hidden slides, transitions, timing,
animation and extension properties remain inventoried and unsupported unless
mapped above. DOCX treatment and native/PPTX output require separate evidence;
PowerPoint review is pending.
