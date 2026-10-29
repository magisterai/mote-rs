# Line and connector

[Directive index](README.md) · [Drawing property inventory](attributes/shapes-and-lines.md)

Inventory precedes implementation. All DrawingML line properties, marker ends,
transform records, connection metadata and value domains remain in the shared
inventory, including properties without an authoring mapping.

- [ ] Named arguments only; no content body or positional arguments.
- [ ] `name`: string or number, normalized to a string.
- [ ] Required `from` and `to` point objects, each with required `x` and `y` lengths.
- [ ] `border`: width, color, style (single/solid/dashed/dotted/none), opacity.
- [ ] `cap`: flat, round or square.
- [ ] `opacity`: numeric or percentage text, normalized to text.
- [ ] `arrow` string selects the end marker; none, triangle/arrow, stealth,
  diamond and oval kinds.
- [ ] `arrow` object start/end markers; from/to aliases normalize to start/end.
- [ ] Shared size/length and start_size/start_length/end_size/end_length;
  from_size/from_length/to_size/to_length aliases normalize to start/end fields.
- [ ] Marker sizes small/sm, medium/med and large/lg normalize to sm/med/lg.
- [ ] Shared `properties` overrides and retained unknown nested metadata.
- [ ] Lua `document.line` uses the same validation and typed attributes.

Output boundary: editable connectors and native line painting belong to
presentation layout. The document output baseline does not emit editable lines;
its unsupported behavior must remain explicit. Shape borders are a separate
supported drawing property and must not be confused with line directives.
