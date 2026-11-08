# Mathematics

`math` contains an equation source, not a content body. `source` is a nonempty
string or number supplied positionally or by name; surrounding whitespace is
removed. `format` is `latex` by default, or `omml` for Office Math XML.

`display` defaults to `inline`. Boolean `true`, `"true"`, `"block"`, and
`"display"` select block content. Boolean `false`, `"false"`, `"inline"`, and
an empty string select inline content. `size` accepts a length. The shared
`properties` object overrides named attributes; extra properties are retained
without inventing mathematical layout semantics. No content body is accepted.

Inline equations also use `$...$` or `\(...\)`. Display equations use `\[...\]`.
A single-dollar opener cannot precede whitespace, a digit, or another dollar;
its closer must be on the same line. Thus `$25` is ordinary text. An unmatched
opener remains ordinary markup. Delimited equations have the same normalized
attributes as `math`. They are not a full TeX document language.

```mote
The identity $x^2 + y^2$ is inline.
#math("\\frac{a+b}{c}", display=true, size=pt(14))
```

The source grammar supports groups `{...}`, subscripts `_`, superscripts `^`,
and these expression families:

- Fractions `frac`, `dfrac`, `tfrac`, `cfrac`; no-bar `binom`; `sqrt` with an
  optional bracketed root degree.
- N-ary `sum`, `prod`, `bigcap`, `bigcup`, `int`, `iint`, `iiint`, `oint`, with
  optional scripts; `lim` with a limit. Ordinary operators include trigonometric
  functions, `log`, `ln`, `exp`, `max`, `min`, and `operatorname`.
- Greek letters and named relation, arrow, set and arithmetic symbols;
  `text` for normal text, `mathrm` for roman text, and `mathbf`, `mathcal`,
  `mathbb`, `mathfrak`, `mathit`, `mathsf`, `mathtt` for Unicode math alphabets.
- `left` and `right` delimiters; `hat`, `vec`, `dot`, `ddot`, `tilde` accents;
  `bar`, `overline`, `underline`, `overbrace`, and `underbrace`.
- `begin`/`end` environments: `matrix`, `smallmatrix`, `pmatrix`, `bmatrix`,
  `vmatrix`, `Vmatrix`, `array`, `cases`, `aligned`, `align`, `split`, and `gather`.
  Ampersands separate cells or alignment positions; doubled backslashes separate
  rows. An `array` column declaration is accepted without column-style effects.

Unknown commands become visible command names and warn; unknown environments
flatten their contents and warn. Missing delimiter symbols, an absent closing
`right`, or an unmatched `right` preserve the entire equation as literal source
text and warn. Whitespace outside text groups is ignored,
with explicit spacing commands retained. This deliberately bounded grammar
must not be treated as arbitrary LaTeX execution.

Lua `document.math` accepts the same attributes and returns an element. Its
normalized `display` determines whether it contributes inline or block content.

Office output retains editable math structure. Native PDF/PNG typesets each
equation with the OpenType MATH table of Cambria Math, the font Word and
PowerPoint use, or else STIX Two Math. A declared font named `Cambria Math`
takes precedence over both. A display equation is set in display style, with
enlarged operators and limits above and below. An inline equation that stands
alone in its paragraph is displayed, as Word displays it. Display style extends
to the bases of scripts and the cells of matrices, and every fraction of a
display equation keeps the full size, however deeply it is nested. The
outermost fraction outside delimiters takes the display shifts; a nested or
enclosed fraction takes the text shifts with the display gaps, unless the
equation is displayed only for standing alone. The ink of successive rows of a
matrix or an alignment is 0.38 em apart, and their baselines at least a line of
the math font apart. A matrix cell of plain characters takes only that line. A
large operator with limits is followed in Word by a space, which Office output
gives it as its argument. An overbrace rises 0.44 em above its body. A
stretched delimiter, brace or radical takes one fewer extender when that comes
nearer its target. An equation is an unbreakable object within its line. The
equation is measured against the single-spaced line: once it comes within 0.06
em of either edge, as in Word, or 0.13 em of the top in PowerPoint, the line
grows by the excess whatever the line spacing. A display equation in Word keeps
0.137 em above and 0.037 em below, or 0.289 em and 0.119 em if it opens with
ordinary text. A minimum line height absorbs that growth until the line
outgrows it. An exact height in Word is kept unchanged; PowerPoint keeps the
height but raises the line by the growth below less half the growth above. Text
in `text` is set upright in the paragraph font, as in Word and PowerPoint,
unless that font is a core PDF font, when it falls back to the math font. If no
math font is available, or the equation is raw OMML, native output shows italic
source text with a warning. Raw OMML must be a single `m:oMath` root, or
`m:oMathPara` for display math, in the Office Math namespace. Invalid raw XML
is emitted as visible text with a warning.
