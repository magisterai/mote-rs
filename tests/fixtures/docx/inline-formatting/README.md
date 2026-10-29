# Inline formatting reference

This capture covers strong/emphasis nesting, span fonts and colors, fractional
font sizes, highlights, underline, small caps, strikes, and explicit false values.
Sizes are mirrored in `w:sz` and `w:szCs`; bold and italic are mirrored for complex
scripts. Quarter-point rounding and the twip intermediate used by non-point
lengths are intentional. A direct size of zero omits explicit run size.

The default `Code` character style specifies Courier New at 10pt. It clears
inherited direct font/size while retaining unrelated formatting. Preserved code
tabs expand to four spaces, ordinary tabs become `w:tab`, and breaks inside inline
formatting retain their distinct run placement. Child changes never affect siblings.

[Tests](../../../docx.rs) compare the model and complete captured package. Direct
strong/emphasis attributes and direct code font/size/color have no DOCX mapping
in this scope and produce an error. Named styles and font declarations follow
in a separate increment. Real Word review remains **unreviewed**.
