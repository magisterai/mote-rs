# Positioned slide charts

Seventeen fixed slides cover all twelve supported chart kinds, multiple charts
per slide, explicit and inherited palettes, theme fonts/text colors, signed data,
axes, titles, legends, data labels, chart/plot fills, borders, gridlines, gap and
overlap. Label cases exercise value/category/series/percentage combinations.
Unknown `show_*` label keys in the theme case are deliberately retained without
an effect; the labels case uses the documented vocabulary.

Each source has a model, source/font hashes, all canonical PPTX parts and nested
workbook parts, ordered native geometry and independent PNG captures. Geometry
matches within 1e-8. Thirteen native PDFs match their 144-DPI capture exactly;
the other four differ at seven isolated pixels in total, at most 23 channel
levels. Native PNG differences remain below 48 levels and a per-case mean bound.
All native slides were inspected. PowerPoint visual review remains unreviewed.

Chart sizing follows the containing box. Document chart rendering continues to
fit its flow column. Both layouts share chart geometry and painters; slide
layout supplies the deck's palette, text and axis colors, font and doughnut
background. Editable charts retain embedded workbooks and distinct axis IDs.
The existing native chart limits recorded with document chart captures remain
in effect, including radial label placement and omission of area labels.
