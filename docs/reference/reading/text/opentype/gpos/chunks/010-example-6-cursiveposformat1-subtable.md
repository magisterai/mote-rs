# Example 6: CursivePosFormat1 subtable

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-example-3-singleposformat2-subtable.md) · [Next](011-example-9-markmarkposformat1-subtable.md)

<a id="example6"></a>

<a id="example-6-cursiveposformat1-subtable"></a>

### Example 6: CursivePosFormat1 subtable

In Example 6, the Urdu language system uses a CursivePosFormat1 subtable to attach glyphs along a diagonal baseline that descends from right to left. Two glyphs are defined with attachment data and listed in the Coverage table-the Kaf and Ha glyphs. For each glyph, the subtable contains an EntryExit record that defines offsets to two Anchor tables, an entry attachment point, and an exit attachment point. Each Anchor table defines X and Y coordinate values. To render Urdu down and diagonally, the entry point’s Y coordinate is above the baseline and the exit point’s Y coordinate is located below the baseline.

*Example 6*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **CursivePosFormat1**  DiagonalWritingSubTable | CursivePos subtable definition |
| 0001 | 1 | format |
| 000E | DiagonalWritingCoverage | offset to Coverage table |
| 0002 | 2 | entryExitCount |
|  | entryExitRecords[0] | EntryExit record for Kaf glyph |
| 0016 | KafEntryAnchor | offset to EntryAnchor table |
| 001C | KafExitAnchor | offset to ExitAnchor table |
|  | entryExitRecords[1] | EntryExit record for Ha glyph |
| 0022 | HaEntryAnchor | offset to EntryAnchor table |
| 0028 | HaExitAnchor | offset to ExitAnchor table |
|  | **CoverageFormat1**  DiagonalWritingCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0002 | 2 | glyphCount |
| 0203 | KafGlyphID | glyphArray[0] |
| 027E | HaGlyphID | glyphArray[1] |
|  | **AnchorFormat1**  KafEntryAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 05DC | 1500 | xCoordinate |
| 002C | 44 | yCoordinate |
|  | **AnchorFormat1**  KafExitAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 0000 | 0 | xCoordinate |
| FFEC | -20 | yCoordinate |
|  | **AnchorFormat1**  HaEntryAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 05DC | 1500 | xCoordinate |
| 002C | 44 | yCoordinate |
|  | **AnchorFormat1**  HaExitAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 0000 | 0 | xCoordinate |
| FFEC | -20 | yCoordinate |

<a id="example7"></a>

<a id="example-7-markbaseposformat1-subtable"></a>

### Example 7: MarkBasePosFormat1 subtable

The MarkBasePosFormat1 subtable in Example 7 defines one Arabic base glyph, Tah, and two Arabic mark glyphs: a fathatan mark above the base glyph, and a kasra mark below the base glyph. The BaseGlyphsCoverage table lists the base glyph, and the MarkGlyphsCoverage table lists the mark glyphs.

Each mark is also listed in the MarkArray, along with its attachment point data and a mark Class value. The MarkArray defines two mark classes: class 0 consists of marks located above base glyphs, and class 1 consists of marks located below base glyphs.

The BaseArray defines attachment data for base glyphs. In this array, one BaseRecord is defined for the Tah glyph with offsets to two BaseAnchor tables, one for each class of marks. AboveBaseAnchor defines an attachment point for marks placed above the Tah base glyph, and BelowBaseAnchor defines an attachment point for marks placed below it.

*Example 7*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **MarkBasePosFormat1**  MarkBaseAttachSubTable | MarkBasePos subtable definition |
| 0001 | 1 | format |
| 000C | MarkGlyphsCoverage | offset to markCoverage table |
| 0014 | BaseGlyphsCoverage | offset to baseCoverage table |
| 0002 | 2 | markClassCount |
| 001A | MarkGlyphsArray | offset to MarkArray table |
| 0030 | BaseGlyphsArray | offset to BaseArray table |
|  | **CoverageFormat1**  MarkGlyphsCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0002 | 2 | glyphCount |
| 0333 | fathatanMarkGlyphID | glyphArray[0] |
| 033F | kasraMarkGlyphID | glyphArray[1] |
|  | **CoverageFormat1**  BaseGlyphsCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0001 | 1 | glyphCount |
| 0190 | tahBaseGlyphID | glyphArray[0] |
|  | **MarkArray**  MarkGlyphsArray | MarkArray table definition |
| 0002 | 2 | markCount |
|  | markRecords[0] | MarkRecords in Coverage index order |
| 0000 | 0 | markClass, for marks over base |
| 000A | fathatanMarkAnchor | markAnchorOffset |
|  | markRecords[1] |  |
| 0001 | 1 | markClass, for marks under |
| 0010 | kasraMarkAnchor | markAnchorOffset |
|  | **AnchorFormat1**  fathatanMarkAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 015A | 346 | xCoordinate |
| FF9E | -98 | yCoordinate |
|  | **AnchorFormat1**  kasraMarkAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 0105 | 261 | xCoordinate |
| 0058 | 88 | yCoordinate |
|  | **BaseArray**  BaseGlyphsArray | BaseArray table definition |
| 0001 | 1 | baseCount |
|  | baseRecords[0] |  |
| 0006 | AboveBaseAnchor | baseAnchorOffsets[0] |
| 000C | BelowBaseAnchor | baseAnchorOffsets[1] |
|  | **AnchorFormat1**  AboveBaseAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 033E | 830 | xCoordinate |
| 0640 | 1600 | yCoordinate |
|  | **AnchorFormat1**  BelowBaseAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 033E | 830 | xCoordinate |
| FFAD | -83 | yCoordinate |

<a id="example8"></a>

<a id="example-8-markligposformat1-subtable"></a>

### Example 8: MarkLigPosFormat1 subtable

Example 8 uses the MarkLigPosFormat1 subtable to attach marks to a ligature glyph in the Arabic script. The hypothetical ligature is composed of three glyph components: a Lam (initial form), a meem (medial form), and a jeem (medial form). Accent marks are defined for the first two components: the sukun accent is positioned above lam, and the kasratan accent is placed below meem.

The LigGlyphsCoverage table lists the ligature glyph and the MarkGlyphsCoverage table lists the two accent marks. Each mark is also listed in the MarkArray, along with its attachment point data and a mark Class value. The MarkArray defines two mark classes: class 0 consists of marks located above base glyphs, and class 1 consists of marks located below base glyphs.

The LigGlyphsArray has an offset to one LigatureAttach table for the covered ligature glyph. This table, called LamWithMeemWithJeemLigAttach, defines a count and array of the component glyphs in the ligature. Each ComponentRecord defines offsets to two Anchor tables, one for each mark class.

In the example, the first glyph component, lam, specifies a high attachment point for positioning accents above, but does not specify a low attachment point for placing accents below. The second glyph component, meem, defines a low attachment point for placing accents below, but not above. The third component, jeem, has no attachment points since the example defines no accents for it.

*Example 8*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **MarkLigPosFormat1**  MarkLigAttachSubTable | MarkLigPos subtable definition |
| 0001 | 1 | format |
| 000C | MarkGlyphsCoverage | offset to markCoverage table |
| 0014 | LigGlyphsCoverage | offset to ligatureCoverage table |
| 0002 | 2 | markClassCount |
| 001A | MarkGlyphsArray | offset to MarkArray table |
| 0030 | LigGlyphsArray | offset to LigatureArray table |
|  | **CoverageFormat1**  MarkGlyphsCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0002 | 2 | glyphCount |
| 033C | sukunMarkGlyphID | glyphArray[0] |
| 033F | kasratanMarkGlyphID | glyphArray[1] |
|  | **CoverageFormat1**  LigGlyphsCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0001 | 1 | glyphCount |
| 0234 | LamWithMeemWithJeem  LigatureGlyphID | glyphArray[0] |
|  | **MarkArray**  MarkGlyphsArray | MarkArray table definition |
| 0002 | 2 | markCount |
|  | markRecords[0] | MarkRecords in Coverage index order |
| 0000 | 0 | markClass, for marks above components |
| 000A | sukunMarkAnchor | markAnchorOffset |
|  | markRecords[1] |  |
| 0001 | 1 | markClass, for marks below components |
| 0010 | kasratanMarkAnchor | markAnchorOffset |
|  | **AnchorFormat1**  sukunMarkAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 015A | 346 | xCoordinate |
| FF9E | -98 | yCoordinate |
|  | **AnchorFormat1**  kasratanMarkAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 0105 | 261 | xCoordinate |
| 01E8 | 488 | yCoordinate |
|  | **LigatureArray**  LigGlyphsArray | LigatureArray table definition |
| 0001 | 1 | ligatureCount |
| 0004 | LamWithMeemWithJeemLigAttach | ligatureAttachOffsets[0] |
|  | **LigatureAttach**  LamWithMeemWithJeemLigAttach | LigatureAttach table definition |
| 0003 | 3 | componentCount |
|  | componentRecords[0] | Right-to-left text; ComponentRecords in writing-direction (logical) order: right-most glyph first |
| 000E | AboveLamAnchor | ligatureAnchorOffsets[0] — offsets ordered by mark class |
| 0000 | NULL | ligatureAnchorOffsets[1] — no attachment points for class 1 marks |
|  | componentRecords[1] |  |
| 0000 | NULL | ligatureAnchorOffsets[0] — no attachment points for class 0 marks |
| 0014 | BelowMeemAnchor | ligatureAnchorOffsets — for class 1 marks (below) |
|  | componentRecords[2] |  |
| 0000 | NULL | ligatureAnchorOffsets — no attachment points for class 0 marks |
| 0000 | NULL | ligatureAnchorOffsets[1] — no attachment points for class 1 marks |
|  | **AnchorFormat1**  AboveLamAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 0271 | 625 | xCoordinate |
| 0708 | 1800 | yCoordinate |
|  | **AnchorFormat1**  BelowMeemAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 0178 | 376 | xCoordinate |
| FE90 | -368 | yCoordinate |

