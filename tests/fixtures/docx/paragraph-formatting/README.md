# Paragraph formatting reference

The capture checks all four alignments, physical units and unitless twips,
zero values, flat/nested spacing precedence, exact/minimum/multiple line spacing,
indentation, explicit true/false pagination flags, outline levels, shading,
borders and tab stops. Border widths use eighths of a point; line multiples use
240ths of a line. Source, evaluated model and complete package parts are retained.

[DOCX tests](../../../docx.rs) compare numeric JSON values independent of integer
or decimal spelling and compare all package parts structurally. The one declared
exception is the captured `w:lastRenderedPageBreak` advisory marker. Its presence
and count are asserted separately; the writer omits pagination estimates because
it does not compute layout. The authored `w:pageBreakBefore` is compared normally.
No content, formatting, geometry, or actual break settings are excluded.

The capture warns that the unitless 240-twip exact line height may clip its text.
The current writer preserves that requested spacing without estimating clipping.
Output outline levels are limited to Word's 0–9 range even though model storage
accepts larger non-negative integers. Extra nested properties fail explicitly.

Real Word rendering and layout review remain **unreviewed**.
