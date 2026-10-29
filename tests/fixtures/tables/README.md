# Table output captures

The sources separate basic flow, property mapping, merged grids, Markdown,
recursive cell content and table-style inheritance. `captured/<case>/` retains
the full original package, evaluated model, source/package hashes, and an explicit
unreviewed Word status. Media in the content fixture is shared with the image
fixtures through a source-relative path.

Regression tests compare every package part and relationship. XML namespace
prefixes, attribute order, formatting whitespace and generation timestamps are
normalized; binary media compares exactly. The original package parts are kept
unchanged. Native table-layout evidence is added separately as that path develops.
