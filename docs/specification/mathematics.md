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
flatten their contents and warn. An absent closing `right` delimiter retains a
replacement character and warns. Whitespace outside text groups is ignored,
with explicit spacing commands retained. This deliberately bounded grammar
must not be treated as arbitrary LaTeX execution.

Lua `document.math` accepts the same attributes and returns an element. Its
normalized `display` determines whether it contributes inline or block content.

Office output retains editable math structure. Native PDF/PNG uses visible
italic source text and warns that mathematical typesetting is deferred. Raw
OMML must be a single `m:oMath` root, or `m:oMathPara` for display math, in the
Office Math namespace. Invalid raw XML is emitted as visible text with a warning.
