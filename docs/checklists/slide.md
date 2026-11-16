# Slide

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Presentation extensions](attributes/presentation-extensions.md)

- [x] Block body required; an empty body creates an empty slide and a warning.
- [x] Positioned boxes/shapes/lines/groups cannot mix with loose content in one slide.
- [x] `name`: text/number normalization; `notes`: speaker-note text.
- [x] `background`: color or `color`, `src`, `fit` property table; replaces the
  complete deck background rather than merging image and color fields.
- [x] `footer`, `date`, `slide_number`: slide overrides, including false to suppress.
- [x] `layout`: trimmed name, built-in/custom resolution and typed slots; see
  the [layout checklist](layout.md).
- [x] `properties` overrides, retained metadata and Lua construction.
- [x] `field_text`: replaces the deck `field_text` member by member.
- [x] `duration`: a number of seconds, not negative, which `--frames` takes as
  an end. It writes nothing to PPTX.
- [x] `advance_after`: a number of seconds, not negative. PPTX writes it as the
  time after which the slide advances by itself.
- [x] Automatic loose-content layout: a leading heading, grouped text, separate
  image/table frames in which an image keeps its proportion, shared fitting and page-size-aware geometry.
- [x] Implicit top-level Markdown slide separators and heading boundaries in
  presentation mode; explicit slides mixed with implicit segments.

Output supports explicit slides with positioned text, pictures, tables, shapes,
connectors, [groups](group.md) and charts, plus automatic text, image, table and chart layout. Built-in and custom named layouts are supported.
Speaker notes are plain PPTX metadata. Footer text, dates and slide numbers render
in editable and native output, with independent suppression on each slide.
Master/layout inheritance, hidden slides, transition effects and extension
properties remain inventoried and unsupported unless mapped above.
PowerPoint review remains pending.

The [language definition](../specification/presentations.md) and
[model tests](../../tests/presentations.rs) cover checked contracts.

## Animation

The objects of a slide are animated on one clock, which starts when the slide
is first shown. The [language definition](../specification/presentations.md#animation)
is covered by [model tests](../../tests/presentations.rs), and the output by
[slide rendering tests](../../tests/presentation_rendering.rs) and
[command-line tests](../../tests/cli.rs).

- [x] `animate` on a [box](box.md), a [shape](shape.md), a [line](line.md) and
  a [group](group.md): one effect, or a list of effects.
- [x] `effect`: fade_in, fade_out, appear, disappear, move, scale, rotate.
- [x] `start`, `duration`: seconds, not negative; a start of zero and a
  duration of half a second where omitted. `appear` and `disappear` take no
  duration.
- [x] `easing`: linear, ease_in, ease_out, ease_in_out.
- [x] `by` of `move`: a displacement of signed `x` and `y` lengths. `by` of
  `rotate`: clockwise degrees. `to` of `scale`: a positive factor.
- [x] The binding `time`: the time of rendering in seconds, in Mote and in Lua.
- [x] `render_at` of the [presentation](presentation.md), and the `--time`
  option, which takes precedence over it.
- [x] `--frames RATE`, with `--from` and `--to`: numbered PNG frames.
- [ ] Effects started by a click, repetition, reversal, triggers, curved paths,
  colour changes, builds by paragraph and sound have no Mote argument.
- [ ] Transition effects between slides have no Mote argument.

A slide rendered without a time is the slide as authored, with every effect
disregarded. At a time, native PDF and PNG draw each object as it stands then:
an object is out of view before its first `fade_in` or `appear`, a `move` is
exact, and a faint, enlarged or turned object is drawn as one layer, so that
the parts of a group do not show through one another. PNG resamples an
enlarged or turned layer; PDF keeps its vectors.

PPTX writes the effects as a timing tree in the form PowerPoint itself writes:
every effect starts with the slide after a delay equal to its `start`. Easing
is the PowerPoint model of speed gathered and lost over half the duration, and
native output follows the same curve. PowerPoint opens the tree and reports
each effect with its kind, target, duration and path. A value computed from
`time` is fixed when the package is written; PowerPoint does not play it.

PowerPoint opens the [animation case](../../tests/cases/powerpoint-34-animation.mote),
which holds effects and a displaced group, and its export is recorded among the
[Office references](../../tests/office/README.md).

DOCX has no animation; a document shape is written as authored.

Background-image output inventory: source-relative loading, inherited deck
backgrounds, whole-value slide overrides, color fallback, contain/cover/stretch
placement, clipping and foreground layering. Missing sources must retain the
background color and warn. These effects have
[editable and native captures](../../tests/fixtures/presentation-backgrounds/README.md);
PowerPoint review remains pending.

Footer and notes output inventory: inherited footer text, `footer=true` using
that text, fixed and automatic dates, one-based slide-number fields, independent
false overrides, field geometry/alignment, editable field identifiers and cached
text, plain speaker-note text, slide/notes/master relationships and notes counts.
Automatic dates use the local calendar at generation time. Notes remain package
metadata and do not paint onto slides.
[Editable and native captures](../../tests/fixtures/presentation-fields/README.md)
cover these effects, including independent overrides and combined slide content.
PowerPoint review remains pending.

Automatic layout has [editable and native captures](../../tests/fixtures/presentation-automatic/README.md) for standard, wide and custom canvases. PowerPoint review remains pending.

[Markdown deck captures](../../tests/fixtures/presentation-markdown/README.md) cover implicit boundaries, explicit/implicit mixtures and document-mode isolation.
