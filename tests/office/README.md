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
Each probe states its rule in its first paragraph. The present results are as
follows.

| Probe | Rule | Present result |
| --- | --- | --- |
| [exact-line-baseline](probes/exact-line-baseline.mote) | An exact line's baseline lies at four fifths of its height. | Matches Word exactly. |
| [paragraph-spacing](probes/paragraph-spacing.mote) | The gap between paragraphs follows from the first one's spacing after and the second one's spacing before. | Within 0.3 point of Word. |
| [east-asian-line-height](probes/east-asian-line-height.mote) | A line holding an MS Gothic glyph takes the height Word gives that font. | Within 0.9 point of Word. |
| [chart-value-axis](probes/chart-value-axis.mote) | A vertical value axis pads the data by a twentieth of its range and labels every major tick at a step chosen from the data and the plot height. | Every label of all forty charts matches Word. |
| [automatic-line-height](probes/automatic-line-height.mote) | A single line follows from the font metrics, scaled by the multiple. | Drifts by up to 0.1 point a line. Georgia and Times New Roman are substituted natively although installed, and four lines move to another page. |
| [footnote-spacing](probes/footnote-spacing.mote) | Footnotes stack at the foot of the page with Word's separator and spacing. | The notes lie 19 points from Word's position. |
| [table-row-height](probes/table-row-height.mote) | A row is at least its at-least height, or exactly its exact height. | Native layout ignores row heights; following rows lie up to 27 points away. |
