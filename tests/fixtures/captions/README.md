# Figure and caption captures

Ten frozen cases cover standalone and grouped captions, images and tables,
numbering formats and shared identities, restarts, caption/figure overrides,
explicit label metadata, forward number/title/page references, natural pagination,
nested figures, empty captions, formatting, and retained placement properties.

Each capture contains the evaluated model, every DOCX package part, native geometry,
and native PNG pages. Manifests record source and artifact hashes. Geometry is
compared within 1e-8 points. All 12 native PDF pages matched their captured rasters
at 144 DPI with zero changed pixels and were visually inspected. Native PNG permits
bounded font-edge rasterization differences, with maximum channel difference 56
and mean foreground difference 11. Office visual review remains unreviewed.

The limit cases deliberately remain unchanged: `position` exposes authored DOCX
order versus native caption order and the placement-alignment fallback difference.
`nested` and `stories` expose caption numbers omitted from native cell/story text,
while their metadata still advances number references. These limits warn and remain
visible; changing fixture content to hide them would defeat the comparison.
