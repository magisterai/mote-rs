# Mathematics

[Directive index](README.md) · [Math reference](../reference/topics/math.md)

## Arguments and content

- [x] `source`: one positional or named string/number, trimmed and nonempty.
- [x] `format`: `latex` (default) or `omml`.
- [x] `display`: boolean or `inline`/`block`/`display`/`true`/`false` text;
  selects inline content or a block.
- [x] `size`: a length; `properties` overrides and retained metadata.
- [x] No content body. Lua `document.math` shares the attribute contract.
- [x] Inline `$...$` and `\(...\)`; display `\[...\]`.
  Dollar-prefixed numbers remain currency text.

## Office Math inventory

The [complete property inventory](attributes/math.md) and [math schema](../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-math.xsd)
record all nested properties and value domains. Raw OMML is retained XML, not evidence of a
Mote attribute for each XML field.

- [x] Runs, normal/roman text, mathematical alphabets, symbols and operators.
- [x] Fractions, no-bar binomials, radicals, sub/superscripts and n-ary limits.
- [x] Delimiters, accents, bars, group characters and upper/lower limits.
- [x] Matrices, equation arrays and alignment markers.
- [ ] Direct authoring mappings for every OMML control property, phantom,
  border box, function, pre-script and math paragraph setting remain unsupported.

## Output limits and evidence

Editable DOCX/PPTX math and native typesetting are separate capabilities.
Native output typesets equations with Cambria Math or STIX Two Math and falls
back to italic source text, with a warning, when neither is available. Text in
`text` uses the upright paragraph font, except a core PDF font. Unsupported
commands and environments must warn while preserving a visible degradation.
Raw OMML must have a valid math root before inclusion in an Office package.
Malformed XML becomes visible source text with a warning. Incomplete delimiter
pairs preserve the entire equation source; complete pairs remain editable math.

Evidence: [language definition](../specification/mathematics.md),
[contract and output tests](../../tests/mathematics.rs), and
[frozen captures](../../tests/fixtures/mathematics/README.md).
