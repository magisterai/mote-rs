# Anchor and reference captures

These frozen cases cover label validation, metadata, forward references, editable
bookmark ranges and fields, internal Markdown links, wrapping, section page-number
restarts, and destinations that cannot be placed in native output.

Each capture retains the evaluated model, complete editable package parts, native
text geometry, and native PNG pages. Capture manifests record source hashes and
comparison tolerances. The seven cases contain 13 pages; their PDF rasters matched
at 144 DPI with zero changed pixels. Native PNG comparisons allow bounded font-edge
rasterization differences while requiring matching page counts and geometry.

PDF links are checked for internal actions and logical target pages. Running-story
labels, mid-paragraph labels, labels before lists or breaks, and trailing labels
retain visible text but have no native destination. They warn instead of producing
a fabricated link. Office visual review remains unreviewed.
