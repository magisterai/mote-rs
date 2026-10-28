# Vertical header table example

Source: [Vertical header](https://learn.microsoft.com/en-us/typography/opentype/spec/vhea) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/vhea).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-overview.md)

<a id="vertical-header-table-example"></a>

## Vertical header table example

| Offset/length | Value | Name | Comment |
| --- | --- | --- | --- |
| 0/4 | 0x00011000 | version | Version number of the vertical header table is 1.1. |
| 4/2 | 1024 | vertTypoAscender | Half the em-square height. |
| 6/2 | -1024 | vertTypoDescender | Minus half the em-square height. |
| 8/2 | 0 | vertTypoLineGap | Typographic line gap is 0 font design units. |
| 10/2 | 2079 | advanceHeightMax | The maximum advance height measurement found in the font is 2079 font design units. |
| 12/2 | -342 | minTopSideBearing | The minimum top sidebearing measurement found in the font is -342 font design units. |
| 14/2 | -333 | minBottomSideBearing | The minimum bottom sidebearing measurement found in the font is -333 font design units. |
| 16/2 | 2036 | yMaxExtent | max(tsb + (yMax - yMin)) = 2036. |
| 18/2 | 0 | caretSlopeRise | The caret slope rise of 0 and a caret slope run of 1 indicate a horizontal caret for a vertical font. |
| 20/2 | 1 | caretSlopeRun | The caret slope rise of 0 and a caret slope run of 1 indicate a horizontal caret for a vertical font. |
| 22/2 | 0 | caretOffset | Value set to 0 for nonslanted fonts. |
| 24/4 | 0 | reserved | Set to 0. |
| 26/2 | 0 | reserved | Set to 0. |
| 28/2 | 0 | reserved | Set to 0. |
| 30/2 | 0 | reserved | Set to 0. |
| 32/2 | 0 | metricDataFormat | Set to 0. |
| 34/2 | 258 | numOfLongVerMetrics | Number of advance heights in the vertical metrics table is 258. |
