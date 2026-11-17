# Office comparison

Microsoft Word and PowerPoint are the reference renderers for Mote's native
layout. This directory holds what they produced for every review case and probe,
so that the native layout can be compared with Office on any platform without
Office installed.

## References

A reference in [references/](references/) records the text lines of one Office PDF
export: each line's text, the position of its first glyph after any list marker,
and its baseline, in points from the top-left corner of the page. The reference
also records the SHA-256 digest of the source it was exported from and the
version of the application that exported it. A reference whose source has
changed fails the comparison until it is recorded again, since an edited source
has no Office evidence.

`cargo xtask office-compare --record` builds each subject's DOCX or PPTX, exports
it through Word or PowerPoint on macOS and replaces the reference. The command is
described in the [repository task guide](../../xtask/README.md#office-comparison).

The native side is read from Mote's own PDF with the same extractor, and the two
renderings are paired word by word, since two renderers may divide a row of text
into lines differently where a wide gap separates its parts. Chart and equation
text differs by design, because Office lays it out with its own engines.

## Allowances

[allowances.json](allowances.json) records, for each subject, how far the native
layout may at present lie from its reference: the lines on another page, the lines
without a partner, and the largest and median vertical and largest horizontal
offsets in points. A test fails when any subject exceeds its allowance.

An allowance records the present state, not an acceptable one. A layout fix
lowers the allowances it improves, through `--allow`, and an allowance is raised
only for a change that is itself supported by Office evidence.

## Probes

A probe in [probes/](probes/) isolates one layout rule and sweeps its parameters.
Each probe states its rule in its first paragraph. A probe whose name begins with
`powerpoint-` is exported through PowerPoint; every other probe is exported
through Word. The present results are as follows.

| Probe | Rule | Present result |
| --- | --- | --- |
| [exact-line-baseline](probes/exact-line-baseline.mote) | An exact line's baseline lies at four fifths of its height. | Matches Word exactly. |
| [line-baseline](probes/line-baseline.mote) | An automatic line's baseline lies below the line gap and ascender of its font. A multiple above one adds its space beneath the descender, a multiple below one shrinks the line in proportion, and a minimum adds its space above the line. | Within 0.4 point of Word over ten pages. |
| [paragraph-spacing](probes/paragraph-spacing.mote) | The gap between paragraphs follows from the first one's spacing after and the second one's spacing before. | Within 0.3 point of Word. |
| [east-asian-line-height](probes/east-asian-line-height.mote) | A line holding an MS Gothic glyph takes the height Word gives that font. | Within 0.2 point of Word. |
| [chart-value-axis](probes/chart-value-axis.mote) | A vertical value axis pads the data by a twentieth of its range and labels every major tick at a step chosen from the data and the plot height. | Every label of all forty charts matches Word. |
| [chart-axis-floor](probes/chart-axis-floor.mote) | A value axis starts at zero unless the values, or the running totals of a stacked chart, have one sign, differ, and span at most a sixth of the largest magnitude. The axis then starts half the span beyond the value nearest zero and ends a twentieth of it beyond the farthest, each moved outward to a whole number of steps. | Every label of the eighty-three charts whose axis is vertical or excludes zero matches Word. A horizontal axis that starts at zero follows an older rule, and lacks five of Word's labels in four charts. Labels of 14 points lie up to 3.2 points from Word's. |
| [automatic-line-height](probes/automatic-line-height.mote) | A single line follows from the font metrics, scaled by the multiple. | Within 0.6 point of Word over three pages. |
| [footnote-spacing](probes/footnote-spacing.mote) | Footnotes stack at the foot of the page with Word's separator and spacing. | Within 0.3 point of Word. |
| [table-row-height](probes/table-row-height.mote) | A table lies its top border below the preceding paragraph and half its left border within the margin. An automatic column takes the width of its content and margins. A row holds an at-least height within its cells' margins, and an exact height above its cells' bottom margin with its border drawn inside. | Every row lies within 0.2 point of Word, and every column within 0.1 point. |
| [script-position](probes/script-position.mote) | Word draws a superscript or subscript at a part of the text size that its font states, rounded to a half point, and moves its baseline by a part of the text size, whatever the line spacing. | Within 0.9 point of Word, and 1.5 points for a Courier New subscript. |
| [break-opportunities](probes/break-opportunities.mote) | Word breaks a line at a space and after a hyphen or dash, and divides a longer unbroken token between any two characters. | Every line but one breaks where Word's does. |
| [equation-height](probes/equation-height.mote) | A display equation stands in a paragraph of its own, whose height follows from the parts of the equation. An inline equation alone in its paragraph is displayed likewise. | Fifty-four of the sixty equations measured lie within 0.5 point of Word's height. A line topped by a superscript and an alignment of three rows differ by up to 0.8 point. |
| [radical-height](probes/radical-height.mote) | A radical sign is the smallest of its sizes that covers the radicand, so that the height of a radical moves in steps as the radicand grows. | Fifty-five of the fifty-eight radicals measured lie within 0.5 point of Word's height, and every sign is of Word's size. A sign assembled over six rows differs by 1.0 point. |
| [equation-line-multiple-108](probes/equation-line-multiple-108.mote), [equation-line-multiple-150](probes/equation-line-multiple-150.mote) | A line multiple adds to the height of an equation the space that it adds to a line of text, whatever the height of the equation. | The equations lie within 0.7 point of Word's height under either multiple. |
| [figure-keep](probes/figure-keep.mote) | The content of a figure and its caption stay on one page: a picture or a chart above its caption, a caption above its table, and a table above its caption each begin the next page when the caption or the first row does not fit. | Every figure lies on Word's page, and every line within 0.3 point of Word. |
| [keep-with-next-blocks](probes/keep-with-next-blocks.mote) | A paragraph or heading kept with the next block moves to the page of a chart, a picture or the first row of a table that does not fit below it. | Every kept paragraph lies on Word's page, and every line within 0.2 point of Word. |
| [page-foot](probes/page-foot.mote) | A line lies at the foot of a page that holds the line less the space a line multiple above one adds beneath its descender. The space before a paragraph must fit, the space after it need not, and a table row must fit whole. Widow control and a kept paragraph apply the same measure to the last line that they require on the page. | Every sample lies on Word's page on both sides of each threshold, over seven fonts and sizes. Word's line of Helvetica 11 is 0.05 point taller than the native line, so that sample is held 0.05 point beyond the threshold. |
| [section-footnotes](probes/section-footnotes.mote) | A footnote lies at the foot of the page that cites it, within the side margins and above the bottom margin of its section. A page that continuous sections share takes the bottom margin of the first of them. The numbering continues through every section. | Every note carries Word's number and lies within 0.3 point of Word. |
| [section-endnotes](probes/section-endnotes.mote) | The endnotes follow the content of the last section, in the page geometry of that section, numbered through every section. | Within 0.3 point of Word. |
| [paragraph-shading](probes/paragraph-shading.mote) | The box of a paragraph runs from its leftmost line to its right indent. The shading passes the box by 1.5 points on each side; a right rule draws the shading out to itself, and as far beyond the left edge where a left rule stands. Shading crosses the space between two paragraphs whose boxes and borders are identical. | Within 0.4 point of Word. The probe measures the lines, and each sample states the extent of its shading in Word's PDF; the [border tests](../paragraph_borders.rs) hold the native shading and rules to those extents within the 0.25 point of Word's grid. |
| [cell-paragraph-shading](probes/cell-paragraph-shading.mote) | A paragraph in a table cell is shaded and ruled by the rule of body text, measured from the text area of its cell. The shading ends at the borders of the cell and the rules at its edge. The paragraphs of one cell share a box and carry shading across the space between them, and a box is left open where its row is divided between pages. A border widens no automatic column. | Within half a point of Word. The probe measures the lines, and each sample states the extent of its shading in Word's PDF; the [cell paragraph tests](../cell_paragraphs.rs) hold the native shading to those extents within 0.3 point. |
| [nested-table-cell-end](probes/nested-table-cell-end.mote) | A cell whose content ends with a nested table is closed in the package by an empty paragraph, to which Word gives no height: the row after the cell lies where the nested table alone requires. A nested table lies the width of its top border below the content above it, and the row that begins a page lies the width of its top border below the top of the text area. | Within 0.2 point of Word over two pages, for top borders of none, a half, three and six points. |
| [paragraph-borders](probes/paragraph-borders.mote) | Consecutive paragraphs with identical borders and identical side indents share one box, which holds the space between them and no rule there. A top or bottom border adds its space and its width to the height of a paragraph. | Within 0.2 point of Word. The probe measures the lines; the rules and the shading were measured from Word's PDF and are held by the [border tests](../paragraph_borders.rs). |
| [shape-text](probes/shape-text.mote) | Word lays out the text of a shape as body text, aligns the block of lines in the text rectangle, and keeps the text clear of the inner half of the outline. | Within 0.3 point of Word. |
| [powerpoint-line-height](probes/powerpoint-line-height.mote) | PowerPoint sets a line at six fifths of its font size whatever the font, and every baseline a whole number of points below the top of the text. | The first baseline of every box matches. Later lines lie up to a point away, and Courier New up to two. |
