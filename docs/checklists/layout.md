# Reusable presentation layouts

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md) ·
[Drawing text](attributes/drawing-text.md) · [Boxes](box.md)

## Arguments and declarations

- [ ] `name`: required nonempty trimmed text or number; built-in names reserved.
- [ ] `based_on`: optional trimmed layout name, including forward references;
  reject unknown parents and inheritance cycles.
- [ ] Optional block body containing only empty `box` slot declarations.
- [ ] Repeated names replace the earlier declaration in its original order.
- [ ] `properties`: overrides property arguments after outer type validation.
- [ ] `background`, `footer`, `date`, `slide_number`: slide-level defaults.
- [ ] Lua `document.layout` and typed JSON values use the same declarations.

## Slots and resolution

- [ ] Eight built-ins: blank, title_slide, title_and_content, section_header,
  two_column, comparison, image_and_caption, chart_and_caption.
- [ ] `box.slot`: nonempty name; direct slide assignments use layout order.
- [ ] `box.kind`: content, text, image, table or chart; content is the default.
- [ ] `box.required`: explicit false overrides inherited true; required slots
  must be supplied, and optional absent slots emit no content.
- [ ] `box.remove`: remove an inherited slot without other slot properties.
- [ ] New slots require positions; inherited slots retain geometry unless overridden.
- [ ] Recursive property overrides; retain inherited slot order and append new slots.
- [ ] Reject duplicate slots, unknown assignments, type mismatches, loose slide
  content and slot assignments outside a layout-backed slide.
- [ ] Ordinary positioned objects follow supplied slots in authored order.
- [ ] Layout defaults merge recursively; an explicit slide background replaces
  the complete inherited background. Source-relative assets remain resolvable.

## Format boundaries

Mote layout inheritance is resolved before output. It is not a PresentationML
layout-to-layout inheritance property. PPTX materializes editable slide objects;
native PDF and PNG use the same resolved geometry. PresentationML slide/layout/master
relationships, placeholder identities, inherited color maps, transitions, timing,
animation, and other unimplemented standard properties remain in the linked
inventories. These checkboxes do not imply support for them or Office conformance.
