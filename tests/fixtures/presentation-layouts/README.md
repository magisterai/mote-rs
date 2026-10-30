# Built-in layout captures

`wide` and `standard` exercise every built-in layout, including a blank layout
with an ordinary positioned box, reverse slot assignment order, text, tables,
images, charts and ordinary shapes after slots. `custom` checks fractional page
sizes, position and text overrides, absent optional slots and Lua assignments.

The frozen models retain authored slot order and position-free assignments.
Canonical PPTX captures include every package part and embedded chart workbook.
Native captures record resolved geometry and independent 144-DPI PNG pages.
All 19 PDF pages were compared as raster images. Native PNG tests allow bounded
text-edge antialiasing differences without relaxing geometry comparisons.

Real Microsoft PowerPoint review remains pending. Package and native captures
do not establish Office conformance.
