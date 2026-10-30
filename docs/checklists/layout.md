# Reusable presentation layouts

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md) ·
[Drawing text](attributes/drawing-text.md) · [Boxes](box.md)

## Arguments and declarations

- [x] `name`: required nonempty trimmed text or number; built-in names reserved.
- [x] `based_on`: optional trimmed layout name, including forward references;
  reject unknown parents and inheritance cycles.
- [x] Optional block body containing only empty `box` slot declarations.
- [x] Repeated names replace the earlier declaration in its original order.
- [x] `properties`: overrides property arguments after outer type validation.
- [x] `background`, `footer`, `date`, `slide_number`: slide-level defaults.
- [ ] Lua `document.layout` supports derived layouts and metadata; typed JSON
  values support slot declarations. Empty Lua box round-trips prevent slot
  declarations through the Lua constructor; see the language boundary.

## Slots and resolution

- [x] Eight built-ins: blank, title_slide, title_and_content, section_header,
  two_column, comparison, image_and_caption, chart_and_caption.
- [x] `box.slot`: nonempty name; direct slide assignments use layout order.
- [x] `box.kind`: content, text, image, table or chart; content is the default.
- [x] `box.required`: explicit false overrides inherited true; required slots
  must be supplied, and optional absent slots emit no content.
- [x] `box.remove`: remove an inherited slot without other slot properties.
- [x] New slots require positions; inherited slots retain geometry unless overridden.
- [x] Recursive property overrides; retain inherited slot order and append new slots.
- [x] Reject duplicate slots, unknown assignments, type mismatches, loose slide
  content and slot assignments outside a layout-backed slide.
- [x] Ordinary positioned objects follow supplied slots in authored order.
- [x] Layout defaults merge recursively; an explicit slide background replaces
  the complete inherited background. Source-relative assets remain resolvable.

## Format boundaries

Mote layout inheritance is resolved before output. It is not a PresentationML
layout-to-layout inheritance property. PPTX materializes editable slide objects;
native PDF and PNG use the same resolved geometry. PresentationML slide/layout/master
relationships, placeholder identities, inherited color maps, transitions, timing,
animation, and other unimplemented standard properties remain in the linked
inventories. These checkboxes do not imply support for them or Office conformance.

The [language definition](../specification/presentation-layouts.md), [validation tests](../../tests/presentation_layouts.rs) and [built-in captures](../../tests/fixtures/presentation-layouts/README.md) cover built-in slots and resolution. The [custom captures](../../tests/fixtures/presentation-custom/README.md) cover inheritance, metadata and source-relative assets. JSON inspection omits declarations; direct model tests cover them. PowerPoint review remains pending.
