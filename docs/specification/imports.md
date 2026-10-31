# Imported declarations

`#import("shared/preset.mote")` evaluates a source file and makes its local
bindings, functions and element defaults available in the importing scope.
It accepts one string path, with no named arguments or content body, and inserts
no content. Ordinary text in the imported file is discarded. Document-wide
font, style, layout and configuration declarations remain effective.

The path is relative to the source containing the import. The selected asset
resolver supplies both bytes and a stable logical name. A resolved source is
imported once per compilation, even when reached through different aliases.
An active import cycle is an error. A repeated import does not re-export bindings
into another scope; import a shared preset in a scope that encloses its uses.

Imported functions retain their definition's lexical values, defaults and source
location. Their JSON and CSV paths resolve from that source. Fonts retain
the location of their declaration. Imported diagnostics identify the file where
the failure occurred. Resource snapshots use the same resolver and cache as
other source-relative resources.

Lua element values currently retain attributes and children but lose their source
metadata when converted through Lua tables. Consequently an image returned by an
imported Lua function resolves relative to the output's calling source. Use
an absolute reference when that image must come from another
directory. Imported font and background declarations do retain their source.

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
