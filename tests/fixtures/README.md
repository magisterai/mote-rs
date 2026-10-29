# Conformance fixtures

## Language examples

`specification.json` declares an outcome for each fenced `mote` example in
`docs/specification/`. Example identifiers use the chapter filename and the
one-based position of its Mote code block.

The expected models record the documented content, structure, and attribute
values. They are reviewable expectations, not automatically regenerated output.
Models retain text fragment boundaries and source markers; Markdown and directive
forms can have the same visible meaning with different inspection metadata.
The document envelope includes implicit section content and empty property groups.

The conformance test reads the source directly from the chapters. Adding or
removing an example requires updating its outcome here. Successful examples must
evaluate to their declared models. Deliberate invalid examples must parse and
fail evaluation for the rule explained in both the chapter and the fixture's
`invalid` field. Their `diagnostic` field identifies the expected problem.

Every concept used in an example must be defined earlier in the reading order or
immediately before the example. A missing implementation is not an intentional
invalid use: do not add deferrals to keep introductory examples passing. The test
rejects deferral entries. Definitions and explanations still require prose review;
successful execution alone cannot establish that an example has been introduced
properly.

These are language and model checks. They provide no rendered-layout or Office
conformance evidence.

## Output references

The DOCX fixtures retain source, evaluated models, complete package parts,
fingerprints and comparison notes:

- [Plain paragraph](docx/plain-paragraph/README.md): package structure and defaults.
- [Text fidelity](docx/text-fidelity/README.md): Unicode, whitespace, tabs and breaks.
- [Paragraph formatting](docx/paragraph-formatting/README.md): geometry and pagination requests.
- [Inline formatting](docx/inline-formatting/README.md): spans, decorations and code.
- [Styles and fonts](docx/styles-and-fonts/README.md): inheritance, references and script mappings.
- [Declared properties](docx/style-properties/README.md): property ordering and style identities.

- [Headings](docx/headings/README.md): levels, direct properties and fallback styles.
- [Heading styles](docx/heading-styles/README.md): declarations and opening spacing.
- [Bullet lists](docx/bullet-lists/README.md): nesting and item geometry.
- [Bullet instances](docx/bullet-instances/README.md): shared definitions and identities.
- [Ordered lists](docx/ordered-lists/README.md): formats, starts and nesting.
- [List continuation](docx/list-continuation/README.md): restarts and sequence identity.
- [Heading numbering](docx/heading-numbering/README.md): hierarchy and sequence controls.
- [Block quotes](docx/block-quotes/README.md): nested content and direct formatting.
- [Quote styles](docx/quote-styles/README.md): declared replacements and scope.
- [Task items](docx/task-items/README.md): editable checkbox controls and numbered-list interaction.
- [Derived built-in styles](docx/derived-builtin-styles/README.md): inherited direct-formatting precedence.

- [Page geometry](docx/page-geometry/README.md): named paper, landscape and all margin fields.
- [Page overrides](docx/page-overrides/README.md): repeated configuration and explicit dimensions.
- [Custom page](docx/page-custom/README.md): metric sizes, zero margins and fractional twips.

- [Document columns](docx/document-columns/README.md): equal widths, gap and separator.
- [Column overrides](docx/column-overrides/README.md): repeated fields and explicit false.
- [Column metadata](docx/column-metadata/README.md): omitted count and retained geometry.

- [Document defaults](docx/document-defaults/README.md): fonts, size, language and direct precedence.
- [Default overrides](docx/default-overrides/README.md): repeated declarations and Normal overrides.
- [Default script fonts](docx/default-script-fonts/README.md): script-only aliases and Latin fallback.

- [Inline tabs](docx/inline-tabs/README.md): stop alignment, edge trimming and preserved code.

[DOCX package tests](../docx_packages.rs) generate packages and compare each fixture's evaluated
model and all XML parts. They also verify that style, numbering and package
relationships resolve to existing targets. Numeric JSON values compare by value rather than decimal
spelling. XML comparisons retain text, units, attribute values and element order;
only generation timestamps and formatting whitespace outside text are ignored.
Any captured advisory pagination markers are inventoried explicitly and omitted
from comparison because the writer does not compute page layout. Authored break
settings remain part of the comparison.

All captures remain **unreviewed in Microsoft Word**. Structural agreement does
not establish layout, font substitution, wrapping or pagination conformance.
