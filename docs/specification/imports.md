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

The library holds `components/gantt` and `components/timeline`. Each is at
present a placeholder: its function, `gantt()` or `timeline()`, returns a
paragraph that says so. Both define Lua functions, and so require Lua.

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
