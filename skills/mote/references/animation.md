# Animation

The objects of a slide are animated on one clock. A **time** is a number of
seconds from the moment the slide is first shown. An **effect** changes one
object over an interval of that clock. The **binding `time`** holds the time at
which the source is rendered, so that any value may be computed from it.

## Effects

```mote
#presentation(size = "wide")
#slide(duration = 5)[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(1) },
       animate = { effect = "fade_in", duration = 1 })[
    #heading(level = 1)[Results]
  ]
  #shape(position = { x = inch(0.7), y = inch(2), width = inch(2), height = inch(1) }, fill = "#2563eb",
         animate = {
           { effect = "move", by = { x = inch(8) }, start = 1, duration = 2, easing = "ease_in_out" },
           { effect = "move", by = { y = inch(2) }, start = 3, duration = 1 },
         })
  #group(name = "Card", animate = { { effect = "fade_in", start = 2 }, { effect = "fade_out", start = 4 } })[
    #shape(position = { x = inch(0.7), y = inch(4.5), width = inch(4), height = inch(2) }, kind = "round_rect", fill = "#dcfce7")
    #box(position = { x = inch(0.9), y = inch(4.7), width = inch(3.6), height = inch(1) })[Retention improved.]
  ]
]
```

- The `animate` argument belongs to a box, a shape, a line and a group. Its
  value is one effect or a list of effects. A box is animated whatever it
  holds: text, an image, a table or a chart.
- The `effect` is `fade_in`, `fade_out`, `appear`, `disappear`, `move`, `scale`
  or `rotate`.
- The `start` and `duration` are numbers of seconds. A start is 0 and a duration
  0.5 where omitted; `appear` and `disappear` take no duration.
- The `easing` is `linear`, `ease_in`, `ease_out` or `ease_in_out`.
- A `move` requires `by`, a table of signed `x` and `y` lengths. A `rotate`
  requires `by`, a number of clockwise degrees. A `scale` requires `to`, a
  positive factor.
- An object whose first effect is `fade_in` or `appear` is out of view until
  that effect begins. A second `move` begins where the first ended.
- A `scale` begins at the size as authored and grows or shrinks about the
  centre of the object; `to = 1.2` ends a fifth larger and `to = 0.5` at half
  size. No effect grows an object from one edge: a bar that fills from the left
  is a shape whose width is computed from `time`, as below, and plays in frames
  but not in PowerPoint.
- Effects of one object whose intervals overlap play together, so a `move` and
  a `scale` with the same `start` make one motion.
- An effect on a group acts on the group as one object, so that a faint group
  does not show one part through another.
- The `duration` of a slide is the time for which the slide is shown.

## Values computed from the time

```mote lua
#presentation(size = "wide", render_at = 2)

#function eased(from, to, start, duration)
  local progress = math.min(math.max((time - start) / duration, 0), 1)
  progress = progress * progress * (3 - 2 * progress)
  return from + (to - from) * progress
end

#let(revenue = math.floor(eased(0, 1280, 0, 2.5)), height = 0.05 + eased(0, 3, 0.5, 1.5))

#slide(duration = 3)[
  #box(position = { x = inch(0.7), y = inch(0.6), width = inch(8), height = inch(1.6) },
       text = { size = pt(72), bold = true })[\$#revenue m]
  #shape(position = { x = inch(1), y = inch(6.5 - height), width = inch(1.6), height = inch(height) }, fill = "#38bdf8")
]
```

- The binding `time` is a number in Mote and in Lua. A text, a size, a position
  or a color may depend on it.
- The value of `time` is that of the `--time` option, otherwise the `render_at`
  of the deck, otherwise 0.
- A function such as `eased` above carries a value from one number to another
  over an interval; the effects need no such function.

## Rendering

```sh
mote deck.mote --format pptx -o deck.pptx                 # effects play in PowerPoint
mote deck.mote --format png --time 2.5 -o still           # still-1.png, the slide at 2.5 seconds
mote deck.mote --format pdf --time 2.5 -o still.pdf
mote deck.mote --format png --frames 30 -o frame          # frame-0001.png, frame-0002.png, ...
mote deck.mote --format png --frames 30 --from 1 --to 2 --dpi 144 -o frame
```

- A render without `--time` and without `render_at` shows every object as
  authored, with the effects disregarded. Such a render is the proper check of
  the layout of a slide; a render with `--time` checks one moment of its
  animation.
- The `--frames` option takes a number of frames a second. The frames end at
  the latest end of an effect or of a slide `duration`, unless `--to` is given.
  Both ends are rendered, so 4 seconds at 12 frames a second gives 49 frames.
- A source that computes values needs `--lua` on every compile, whatever the
  format.
- A deck of several slides writes `frame-0001-1.png`, `frame-0001-2.png` and so
  on, one file for each slide of each frame.
- A sequence of frames becomes a video with a tool such as `ffmpeg`:
  `ffmpeg -framerate 30 -i frame-%04d.png -pix_fmt yuv420p out.mp4`.

## Limits

- PowerPoint plays the effects of `animate` only. A value computed from `time`
  is fixed in the PPTX at the time of the render; a video of such values is
  made from frames.
- Every effect starts with the slide after its `start`. An effect started by a
  click, a repeated effect and a curved path are not available.
- All slides share one value of `time`, each on its own clock.
- A group has no opacity of its own; `fade_in` and `fade_out` make it faint.
- A chart rescales its axis to its data, so a chart whose values depend on
  `time` changes its scale; bars drawn as shapes keep a fixed scale.
- A PNG of an enlarged or turned object is resampled and slightly soft. A PDF
  keeps its vectors.
