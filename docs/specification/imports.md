# Imported declarations

`#import("shared/preset.mote")` evaluates a source file and makes its local
bindings, functions and element defaults available in the importing scope.
It accepts one string path, with no named arguments or content body, and inserts
no content. Ordinary text in the imported file is discarded. Document-wide
font, style, layout and configuration declarations remain effective.

The path is first taken relative to the source containing the import. The selected asset
resolver supplies both bytes and a stable logical name. A resolved source is
imported once per compilation, even when reached through different aliases.
A path that does not end in `.mote` is sought as written and then with that
extension, so that `#import("components/card")` finds `components/card.mote`.

A source that is not found beside the importing source is sought in each
directory of the **import path**, in order, and then in the **library**, the
sources that Mote itself carries. The caller states the import path, with
`--import-path` on the command line; the import path is empty unless stated, so
that the library alone is searched. A directory of the import path is resolved
as a root reference, not relative to the importing source. A source beside the
importing source therefore takes precedence over one of the same name in the
library. A failure other than absence, such as a refusal of access, ends the
search and is reported. A source of the library imports only other sources of
the library, sought beside itself and then from the root of the library.

The library holds `components/gantt` and `components/timeline`. Both define Lua
functions, and so require Lua. `components/timeline` is at present a
placeholder: its function `timeline()` returns a paragraph that says so.

`components/gantt` defines `gantt(rows, ...)`, which returns a
[group](presentations.md#groups) of shapes that draws a Gantt chart. `rows` is
an array of objects, of which at least one is required. A row has a `start`
date, a label in `task` or `name`, an optional last day in `end` or `finish`,
which must not precede the start, and an optional `color`. A row with a last
day is drawn as a bar from its start to the end of that day, and a row without
one as a diamond on its start. The named arguments are `width` and `height`,
the frame of the group, by default 468 points and 28 points with 24 for each
row; `x` and `y`, the place of the group, which is stated only if one of them
is given, so that the group of a document is otherwise set in the text;
`alignment`, of the group; `today`, a date at which a rule is drawn if the axis
holds it; `color`, of a row that states none; `label_width`, of the column of
labels, by default 24 parts in 100 of the width; and `text_size`. The axis runs
over whole days where the rows span at most 16 days, whole weeks from Monday
at most 120, months at most 800, quarters at most 2400, and years beyond.

An active import cycle is an error. A repeated import does not re-export bindings
into another scope; import a shared preset in a scope that encloses its uses.

Imported functions retain their definition's lexical values, defaults and source
location. Their JSON and CSV paths resolve from that source. Fonts retain
the location of their declaration. Imported diagnostics identify the file where
the failure occurred. Resource snapshots use the same resolver and cache as
other source-relative resources.

Lua element values retain their source location through conversion, including
nested elements. Images returned by imported functions therefore resolve relative
to their defining source. Font and background declarations retain their own
source locations as well.

Imports do not enable Lua implicitly. An imported file containing Lua functions
still requires Lua execution to be enabled by the caller.

See the [support checklist](../checklists/import.md).

For example, `preset.mote` contains:

```mote
#let(label = "Shared report")
#set paragraph(alignment = "center")
```

The importing source uses those declarations:

```mote
#import("preset.mote")
#paragraph[#label]
```
