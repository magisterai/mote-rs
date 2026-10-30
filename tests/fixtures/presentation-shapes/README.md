# Positioned slide drawings

Five fixed sources cover all ten supported shape presets, preset text bounds,
transforms, fill and stroke opacity, shadows, text fitting and formatting, and
connector endpoints, caps and marker kinds/sizes. Models and complete PPTX
packages are captured, including shared content-addressed parts and font hashes.
Four cases also capture ordered native geometry and independent PNGs.

Native PDF rasters at 144 DPI are exact for presets and connectors; fitting
has one pixel differing by one channel level, and transforms has two pixels
within four levels. PNG comparisons bound text and shape-edge rasterization
separately by case. Every native slide was inspected; these captures remain
unreviewed in Microsoft PowerPoint.

The caps case retains editable coverage separately. Native flat caps are accepted
and painted. Native connector caps share the existing rounded raster treatment;
diamond markers use triangular geometry, and separate marker lengths affect
editable PPTX only. Subpixel horizontal strokes can disappear in the native
72-DPI raster. These limits have dedicated fixed inputs.

Text-height growth extends the native content clip while retaining the original
shape outline. Editable PPTX increases its shape height. Shapes keep text readable
through flips. Rotated hyperlink hit areas retain the existing native limitation.
