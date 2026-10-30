# Slide backgrounds

Twelve fixed slides cover deck inheritance, whole-value slide overrides,
color-only overrides that suppress an image, contain/cover/stretch fitting,
matching and different source/canvas aspect ratios, translucent foreground
objects, JPEG resources and missing or inaccessible image sources.

Each source has a model, source/font/image hashes, canonical PPTX parts, ordered
native geometry and independent PNG captures. Geometry matches within 1e-8;
all twelve native PDFs match their 144-DPI captures exactly. PNG captures use
per-case channel and mean bounds, with no channel difference above 40. All
native pages were inspected. PowerPoint visual review remains unreviewed.

Contain uses an editable picture above the background color; cover and stretch
use a slide background image fill. Native images paint beneath foreground
objects and clip at the canvas. Missing images retain the resolved color and
warn. An asset-cache test checks source-relative loading, shared background and
foreground resources, unused overridden sources and painting after resources
become unavailable.
