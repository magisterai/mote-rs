# Footnote and endnote properties

[Inventory guide](README.md)

Sources: [Strict WML](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd)
and [Transitional WML](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd).
Schema names below are standard properties, not proposed Mote attributes.

- [ ] `CT_Footnotes.footnote` / `CT_Endnotes.endnote`: ordered note stories.
- [ ] `CT_FtnEdn.@id`: required signed decimal package identity. Authored `id` is metadata; output allocates identities.
- [ ] `CT_FtnEdn.@type`: normal, separator, continuationSeparator, continuationNotice. Only generated default separator stories are planned.
- [ ] `CT_FtnEdn.EG_BlockLevelElts`: block story content, including paragraphs and tables. See [paragraphs](word-paragraphs.md) and [tables](tables.md) for complete nested families.
- [ ] `CT_FtnEdnRef.@id`: required reference to a note story.
- [ ] `CT_FtnEdnRef.@customMarkFollows`: on/off custom reference mark.
- [ ] `CT_FtnEdnSepRef.@id`: document separator selection.
- [ ] `footnoteRef` / `endnoteRef`: automatic story marker.
- [ ] Note/run styling: [run properties](word-runs.md), with no independent note-style argument.
- [ ] Footnote `pos`: pageBottom or beneathText.
- [ ] Endnote `pos`: sectEnd or docEnd.
- [ ] `numFmt`, `numStart`, `numRestart`: [complete note property families](sections-and-stories.md#w-ct_ftnprops) and [endnote properties](sections-and-stories.md#w-ct_ednprops).
- [ ] `w15:footnoteColumns`: [extension inventory](word-extensions.md#w15-footnotecolumns).

These inventories include unsupported values. No custom note controls are introduced
merely because the package format can represent them.
