# Specification attribute inventory

[Directive checklists](../README.md)

This inventory lists the specification properties relevant to the thirteen
currently documented directives, including unsupported properties and values.
Start with a directive's Mote attribute list, follow its links here, then open
the nested property or value definition you need.

The inventory covers every declared field reachable from the property types in
[scope.json](scope.json), in both retained ECMA-376 Strict and Transitional schema
sets. Inherited fields and schema groups are expanded. Related Word extensions
and DrawingML hyperlink-color extensions are included from the retained Microsoft
specifications. XML names identify source properties; an unchecked XML name does
not introduce a new Mote argument.

## Reading support status

- A checked entry has the specific Mote language/model behaviour stated beside
  it, backed by linked specifications and tests. It does not claim XML emission
  or rendered support unless the note explicitly identifies verified output.
  XML emission remains separate from real Office layout review.
- A partial entry stays unchecked and names the supported Mote subset. For
  example, four alignment values do not complete the full Word alignment domain,
  and boolean underline does not complete all underline styles.
- An unannotated entry has no supported Mote mapping. Some entries are schema
  carriers or format metadata rather than independently authorable attributes.
- A checked child does not complete its unchecked parent. A shared boolean or
  color representation does not implement every property that uses that type.

The directive pages remain the authority for where a Mote attribute can be used.
Shared inventory entries may describe `span` or style support without granting
the same direct attributes to `strong`, `emphasis`, or another directive.
Keeping an arbitrary value in `properties` is not support for its semantics.

Inventory breadth and implementation scope are separate. Implement only features
within the established Mote compatibility baseline. Listing an unsupported
standard property does not add it to that approved feature set. Keep undefined
Mote names and examples out of the language specification until their language
behaviour is deliberately defined and implemented.

## Property families

| Inventory | Used by | Specification starting point |
| --- | --- | --- |
| [Word paragraphs](word-paragraphs.md) | Paragraphs, headings, quotes, and list-item paragraphs; includes nested section-boundary properties. | [17.3.1 Paragraphs](../../reference/reading/ooxml/ecma-376/part-1/sections/00163-17-3-1-paragraphs.md) |
| [Word runs](word-runs.md) | Spans, strong/emphasis/code, paragraph marks, and line breaks. | [17.3.2 Runs](../../reference/reading/ooxml/ecma-376/part-1/sections/00209-17-3-2-run.md) |
| [Numbering](numbering.md) | Lists, ordered lists, items, and heading numbering, including marker definitions and overrides. | [17.9 Numbering](../../reference/reading/ooxml/ecma-376/part-1/sections/00529-17-9-numbering.md) |
| [Fonts](fonts.md) | Font declarations, script selections, themes, classification, and embedded variants. | [17.8.3 Font elements](../../reference/reading/ooxml/ecma-376/part-1/sections/00512-17-8-3-elements.md) |
| [Styles](styles.md) | Paragraph/character/numbering/table styles and conditional table, row, and cell properties. | [17.7 Styles](../../reference/reading/ooxml/ecma-376/part-1/sections/00460-17-7-styles.md) |
| [Item controls](controls.md) | Task-checkbox metadata, legacy fields, structured controls, and alternative control properties. | [Word topic guide](../../reference/topics/word.md) |
| [Drawing and presentation text](drawing-text.md) | Paragraph/run/body properties, list defaults, bullets, fonts, colors, fills, effects, hyperlinks, and text geometry. | [21.1.2 Text](../../reference/reading/ooxml/ecma-376/part-1/sections/02769-21-1-2-paragraphs-and-rich-formatting.md) |
| [Word extensions](word-extensions.md) | Typography, text effects, paragraph metadata, numbering, symbols, and controls. | [MS-DOCX extensions](../../reference/reading/ooxml/microsoft/MS-DOCX/sections/00017-2-2-extensions.md) |
| [Drawing extensions](drawing-extensions.md) | Text hyperlink-color selection. | [MS-ODRAWXML](../../reference/reading/ooxml/microsoft/MS-ODRAWXML/README.md) |
| [Prose-defined extensions](prose-extensions.md) | Custom numbering sequences, an additional calendar value, and document compatibility settings. | MS-DOCX §§2.2.7, 2.3, and 2.4. |
| [Schema value domains](values.md) | Enumerations, ranges, patterns, lists, unions, and their base types. | Follow the source links on each value definition. |

Properties shared between families are stored once and linked from their users.
For example, numbering-level paragraph and run properties link back to their
respective text inventories. Border styles, numbering formats, underline styles,
theme colors, and other enumerations retain their full source value lists.
Occurrence annotations are local schema declarations; surrounding choice/group
rules remain in the linked schema and must also be consulted when implementing.

## Extension attachment points

MS-DOCX adds typography and appearance children to run properties: glow, shadow,
reflection, outline/fill, scene/3D properties, ligatures, numeral form/spacing,
stylistic sets, and contextual alternates. It also adds paragraph identifiers and
proofing metadata, collapsed paragraph state, restart-after-break numbering,
footnote-column counts, extended symbols, checkbox symbols/state, repeated
sections, control appearance/color/data binding, and control locking/checksums.
Their full nested fields are in the extension inventory. Most remain unsupported.

The source declares the run-property extensions individually even though some
are omitted from its consolidated schema appendix. Their individual clauses are
included as supplemental declarations. Four legacy Word scalar imports are
explicitly mapped to the corresponding shared scalar definitions in the retained
ECMA schema set; the value pages identify those aliases.

## Coverage boundary

This is an attribute inventory for existing directive families, rather than an
inventory of every object in Office formats. It includes all nested property
records reached from those families, even when they describe unsupported table
styles, effects, or picture bullets. Paragraph/run container roots include their
attributes; their arbitrary child content is handled by the corresponding content
directives. Document-level font embedding switches are explicitly selected from
the settings record.

Open `any`/extension payloads are recorded as wildcards; they cannot enumerate
unpublished or arbitrary future namespaces. Known text-property extensions above
have explicit roots. Separate content such as equations, drawings, charts,
comments, revisions as document content, linked text boxes, diagram objects, and
presentation animations needs the appropriate directive inventory when introduced.
Their presence in a standard does not make them additional arguments to a
paragraph or span. Presentation text formatting uses DrawingML property types;
slide/master ownership is context, not a direct text attribute.

[OpenType and Unicode](../../reference/topics/fonts-text.md) define font data and
text-processing behaviour; [CommonMark and GFM](../../reference/topics/authoring-data.md)
define source forms. Those algorithms, binary tables, and grammar productions are
not extra formatting attributes. The language chapters and directive body rules
describe the currently supported authoring forms.

## Maintaining completeness

[scope.json](scope.json) selects source editions, property roots, and the explicit
container/setting projections above. [support.json](support.json) records reviewed
Mote mappings, supported subsets, and evidence. Property and scalar pages are
derived from those records and the retained schemas. The prose-defined extension
page is maintained separately against its linked clauses.

From the repository root:

```sh
python3 tools/attribute_inventory.py
python3 tools/attribute_inventory.py --write
```

The first command checks for missing/stale fields, nested types, values, and
support records; the second refreshes the generated pages after an
intentional source/scope/support change. Refreshing never infers support or checks
off a newly discovered field. Review the support evidence separately. Add roots
for a future directive before implementing it, then link its inventory from that
directive's checklist.
