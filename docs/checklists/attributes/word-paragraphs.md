# Word paragraph properties

[Inventory guide](README.md)

Used by paragraphs, headings, quotes, and paragraphs inside list items. Section-boundary properties are included because they are children of paragraph properties.

Checkmarks concern the stated Mote language/model behaviour only. Partial
mappings stay unchecked. Unannotated fields have no supported Mote mapping;
shared schema carriers do not independently define authoring attributes.

Nested types and complete value definitions are linked below. XML element
names and `@attributes` identify specification properties, not new Mote syntax.

## Types

- [w:CT_PPr](#w-ct_ppr)
- [w:CT_P](#w-ct_p)
- [w:CT_String](#w-ct_string)
- [w:CT_OnOff](#w-ct_onoff)
- [w:CT_FramePr](#w-ct_framepr)
- [w:CT_NumPr](#w-ct_numpr)
- [w:CT_PBdr](#w-ct_pbdr)
- [w:CT_Shd](#w-ct_shd)
- [w:CT_Tabs](#w-ct_tabs)
- [w:CT_Spacing](#w-ct_spacing)
- [w:CT_Ind](#w-ct_ind)
- [w:CT_Jc](#w-ct_jc)
- [w:CT_TextDirection](#w-ct_textdirection)
- [w:CT_TextAlignment](#w-ct_textalignment)
- [w:CT_TextboxTightWrap](#w-ct_textboxtightwrap)
- [w:CT_DecimalNumber](#w-ct_decimalnumber)
- [w:CT_Cnf](#w-ct_cnf)
- [w:CT_PPrChange](#w-ct_pprchange)
- [w:CT_TabStop](#w-ct_tabstop)
- [w:CT_PPrBase](#w-ct_pprbase)

<a id="w-ct_ppr"></a>

## w:CT_PPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `pStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Model support: `style` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `keepNext` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Keep Paragraph With Next Paragraph; [§17.3.1.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0233-0237-0049.md#source-page-236-part-1). Model support: `keep_with_next` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `keepLines` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Keep All Lines On One Page; [§17.3.1.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0233-0237-0049.md#source-page-235-part-1). Model support: `keep_together` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `pageBreakBefore` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Start Paragraph on Next Page; [§17.3.1.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-247-part-1). Model support: `page_break_before` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `framePr` — [w:CT_FramePr](word-paragraphs.md#w-ct_framepr); Strict, Transitional; minOccurs=0. Text Frame Properties; [§17.3.1.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0216-0220-0045.md#source-page-218-part-1).
- [x] `widowControl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow First/Last Line to Display on a Separate Page; [§17.3.1.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-271-part-1). Model support: `widow_control` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `numPr` — [w:CT_NumPr](word-paragraphs.md#w-ct_numpr); Strict, Transitional; minOccurs=0. Numbering Definition Instance Reference; [§17.3.1.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-243-part-1). Partial: List and heading numbering emits level/instance references and start overrides; numbering revision metadata is not represented. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs), [docx_packages.rs](../../../tests/docx_packages.rs).
- [ ] `suppressLineNumbers` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Suppress Line Numbers for Paragraph; [§17.3.1.35](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-262-part-1).
- [ ] `pBdr` — [w:CT_PBdr](word-paragraphs.md#w-ct_pbdr); Strict, Transitional; minOccurs=0. Paragraph Borders; [§17.3.1.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0249-0254-0052.md#source-page-249-part-1). Partial: `border` retains side objects as metadata; complete border values and style semantics are not implemented. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0. Partial: `fill` covers hexadecimal background color; patterns, automatic colors, and theme-based shading are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `tabs` — [w:CT_Tabs](word-paragraphs.md#w-ct_tabs); Strict, Transitional; minOccurs=0. Set of Custom Tab Stops; [§17.3.1.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-265-part-1). Partial: `tabs` retains position/alignment/leader metadata; complete stop semantics and schema value validation are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `suppressAutoHyphens` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Suppress Hyphenation for Paragraph; [§17.3.1.34](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-261-part-1).
- [ ] `kinsoku` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Use East Asian Typography Rules for First and Last Character per Line; [§17.3.1.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0238-0242-0050.md#source-page-238-part-1).
- [ ] `wordWrap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow Line Breaking At Character Level; [§17.3.1.45](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-272-part-1).
- [ ] `overflowPunct` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow Punctuation to Extend Past Text Extents; [§17.3.1.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-244-part-1).
- [ ] `topLinePunct` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Compress Punctuation at Start of a Line; [§17.3.1.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-271-part-1).
- [ ] `autoSpaceDE` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Spacing of Latin and East Asian Text; [§17.3.1.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-205-part-1).
- [ ] `autoSpaceDN` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Spacing of East Asian Text and Numbers; [§17.3.1.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-206-part-1).
- [ ] `bidi` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `adjustRightInd` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Right Indent When Using Document Grid; [§17.3.1.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-205-part-1).
- [ ] `snapToGrid` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `spacing` — [w:CT_Spacing](word-paragraphs.md#w-ct_spacing); Strict, Transitional; minOccurs=0. Partial: `spacing` and flat aliases cover physical before/after spacing and three line rules; line-count and automatic before/after controls are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `ind` — [w:CT_Ind](word-paragraphs.md#w-ct_ind); Strict, Transitional; minOccurs=0. Paragraph Indentation; [§17.3.1.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0229-0232-0048.md#source-page-229-part-1). Partial: `indent` covers non-negative physical left/right/first-line/hanging values; signed, character-count, and logical start/end controls are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `contextualSpacing` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Ignore Spacing Above and Below When Using Identical Styles; [§17.3.1.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0216-0220-0045.md#source-page-216-part-1).
- [ ] `mirrorIndents` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Use Left/Right Indents as Inside/Outside Indents; [§17.3.1.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0238-0242-0050.md#source-page-242-part-1).
- [ ] `suppressOverlap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Prevent Text Frames From Overlapping; [§17.3.1.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-263-part-1).
- [ ] `jc` — [w:CT_Jc](word-paragraphs.md#w-ct_jc); Strict, Transitional; minOccurs=0. Partial: `alignment` supports left, center, right, and justify; other Word alignment values have no defined Mote mapping. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0.
- [ ] `textAlignment` — [w:CT_TextAlignment](word-paragraphs.md#w-ct_textalignment); Strict, Transitional; minOccurs=0. Vertical Character Alignment on Line; [§17.3.1.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-265-part-1).
- [ ] `textboxTightWrap` — [w:CT_TextboxTightWrap](word-paragraphs.md#w-ct_textboxtightwrap); Strict, Transitional; minOccurs=0. Allow Surrounding Paragraphs to Tight Wrap to Text Box Contents; [§17.3.1.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-267-part-1).
- [x] `outlineLvl` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Associated Outline Level; [§17.3.1.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-243-part-1). Model support: `outline_level` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `divId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
- [ ] `rPr` — [w:CT_ParaRPr](word-runs.md#w-ct_pararpr); Strict, Transitional; minOccurs=0.
- [ ] `sectPr` — [w:CT_SectPr](sections-and-stories.md#w-ct_sectpr); Strict, Transitional; minOccurs=0.
- [ ] `pPrChange` — [w:CT_PPrChange](word-paragraphs.md#w-ct_pprchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_p"></a>

## w:CT_P

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

Container attributes only; child content has its own directive inventories.

- [ ] `@rsidRPr` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidR` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidDel` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidP` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.
- [ ] `@rsidRDefault` — [w:ST_LongHexNumber](values-word.md#w-st_longhexnumber); Strict, Transitional.

<a id="w-ct_string"></a>

## w:CT_String

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.

<a id="w-ct_onoff"></a>

## w:CT_OnOff

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.

<a id="w-ct_framepr"></a>

## w:CT_FramePr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@dropCap` — [w:ST_DropCap](values-word.md#w-st_dropcap); Strict, Transitional; use=optional.
- [ ] `@lines` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@w` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional.
- [ ] `@h` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional.
- [ ] `@vSpace` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional.
- [ ] `@hSpace` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional.
- [ ] `@wrap` — [w:ST_Wrap](values-word.md#w-st_wrap); Strict, Transitional; use=optional.
- [ ] `@hAnchor` — [w:ST_HAnchor](values-word.md#w-st_hanchor); Strict, Transitional; use=optional.
- [ ] `@vAnchor` — [w:ST_VAnchor](values-word.md#w-st_vanchor); Strict, Transitional; use=optional.
- [ ] `@x` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=optional.
- [ ] `@xAlign` — [s:ST_XAlign](values-shared.md#s-st_xalign); Strict, Transitional; use=optional.
- [ ] `@y` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=optional.
- [ ] `@yAlign` — [s:ST_YAlign](values-shared.md#s-st_yalign); Strict, Transitional; use=optional.
- [ ] `@hRule` — [w:ST_HeightRule](values-word.md#w-st_heightrule); Strict, Transitional; use=optional.
- [ ] `@anchorLock` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional.

<a id="w-ct_numpr"></a>

## w:CT_NumPr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `ilvl` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Numbering Level Reference; [§17.9.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0700-0706-0132.md#source-page-706-part-1). Mote list/heading levels map to Word numbering levels; list levels are bounded to nine and heading levels map to at most nine. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs).
- [x] `numId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Numbering Definition Instance Reference; [§17.9.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0725-0730-0136.md#source-page-729-part-1). List and heading paragraphs reference allocated numbering instances; task items do not consume those instances. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs), [docx_controls.rs](../../../tests/docx_controls.rs), [docx_packages.rs](../../../tests/docx_packages.rs).
- [ ] `numberingChange` — [w:CT_TrackChangeNumbering](anchors-and-fields.md#w-ct_trackchangenumbering); Transitional; minOccurs=0.
- [ ] `ins` — [w:CT_TrackChange](word-runs.md#w-ct_trackchange); Strict, Transitional; minOccurs=0.

<a id="w-ct_pbdr"></a>

## w:CT_PBdr

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `top` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `left` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `bottom` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `right` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0.
- [ ] `between` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0. Paragraph Border Between Identical Paragraphs; [§17.3.1.5](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-207-part-1).
- [ ] `bar` — [w:CT_Border](word-runs.md#w-ct_border); Strict, Transitional; minOccurs=0. Paragraph Border Between Facing Pages; [§17.3.1.4](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-206-part-1).

<a id="w-ct_shd"></a>

## w:CT_Shd

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Shd](values-word.md#w-st_shd); Strict, Transitional; use=required.
- [ ] `@color` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=optional.
- [ ] `@themeColor` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@fill` — [w:ST_HexColor](values-word.md#w-st_hexcolor); Strict, Transitional; use=optional. Partial: `fill` accepts hexadecimal colors; automatic and theme shading are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@themeFill` — [w:ST_ThemeColor](values-word.md#w-st_themecolor); Strict, Transitional; use=optional.
- [ ] `@themeFillTint` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.
- [ ] `@themeFillShade` — [w:ST_UcharHexNumber](values-word.md#w-st_ucharhexnumber); Strict, Transitional; use=optional.

<a id="w-ct_tabs"></a>

## w:CT_Tabs

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `tab` — [w:CT_TabStop](word-paragraphs.md#w-ct_tabstop); Strict, Transitional; minOccurs=1; maxOccurs=unbounded.

<a id="w-ct_spacing"></a>

## w:CT_Spacing

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `@before` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional; default=0. Model support: `spacing.before` and its flat alias store a non-negative physical length. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@beforeLines` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional; default=0.
- [ ] `@beforeAutospacing` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional; default=off.
- [x] `@after` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional; default=0. Model support: `spacing.after` and its flat alias store a non-negative physical length. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@afterLines` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional; default=0.
- [ ] `@afterAutospacing` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional; use=optional; default=off.
- [ ] `@line` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=optional; default=0. Partial: `spacing.line` supports multiple, exact and at-least rules with verified DOCX unit conversion; signed raw Word values remain outside the defined language range. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx.rs](../../../tests/docx.rs).
- [ ] `@lineRule` — [w:ST_LineSpacingRule](values-word.md#w-st_linespacingrule); Strict, Transitional; use=optional; default=auto. Partial: `spacing.line` supports multiple, exact and at-least rules with verified DOCX unit conversion; signed raw Word values remain outside the defined language range. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx.rs](../../../tests/docx.rs).

<a id="w-ct_ind"></a>

## w:CT_Ind

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@start` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=optional.
- [ ] `@startChars` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@end` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=optional.
- [ ] `@endChars` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [ ] `@left` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Transitional; use=optional. Partial: `indent.left` accepts non-negative lengths; negative Word indentation is outside the current value contract. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@leftChars` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Transitional; use=optional.
- [ ] `@right` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Transitional; use=optional. Partial: `indent.right` accepts non-negative lengths; negative Word indentation is outside the current value contract. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@rightChars` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Transitional; use=optional.
- [x] `@hanging` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional. Model support: `indent.hanging` stores a non-negative physical length. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@hangingChars` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.
- [x] `@firstLine` — [s:ST_TwipsMeasure](values-shared.md#s-st_twipsmeasure); Strict, Transitional; use=optional. Model support: `indent.first_line` stores a non-negative physical length. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@firstLineChars` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=optional.

<a id="w-ct_jc"></a>

## w:CT_Jc

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Jc](values-word.md#w-st_jc); Strict, Transitional; use=required. Partial: `alignment` covers left/center/right/justify, not the complete enumeration. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).

<a id="w-ct_textdirection"></a>

## w:CT_TextDirection

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TextDirection](values-word.md#w-st_textdirection); Strict, Transitional; use=required.

<a id="w-ct_textalignment"></a>

## w:CT_TextAlignment

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TextAlignment](values-word.md#w-st_textalignment); Strict, Transitional; use=required.

<a id="w-ct_textboxtightwrap"></a>

## w:CT_TextboxTightWrap

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TextboxTightWrap](values-word.md#w-st_textboxtightwrap); Strict, Transitional; use=required.

<a id="w-ct_decimalnumber"></a>

## w:CT_DecimalNumber

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.

<a id="w-ct_cnf"></a>

## w:CT_Cnf

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_Cnf](values-word.md#w-st_cnf); Transitional.
- [ ] `@firstRow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastRow` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@firstColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@oddVBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@evenVBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@oddHBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@evenHBand` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@firstRowFirstColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@firstRowLastColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastRowFirstColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.
- [ ] `@lastRowLastColumn` — [s:ST_OnOff](values-shared.md#s-st_onoff); Strict, Transitional.

<a id="w-ct_pprchange"></a>

## w:CT_PPrChange

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@id` — [w:ST_DecimalNumber](values-word.md#w-st_decimalnumber); Strict, Transitional; use=required.
- [ ] `@author` — [s:ST_String](values-shared.md#s-st_string); Strict, Transitional; use=required.
- [ ] `@date` — [w:ST_DateTime](values-word.md#w-st_datetime); Strict, Transitional; use=optional.
- [ ] `pPr` — [w:CT_PPrBase](word-paragraphs.md#w-ct_pprbase); Strict, Transitional; minOccurs=1.

<a id="w-ct_tabstop"></a>

## w:CT_TabStop

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [ ] `@val` — [w:ST_TabJc](values-word.md#w-st_tabjc); Strict, Transitional; use=required. Partial: `tabs` retains alignment/leader/position fields; permissive retention does not implement all tab-stop values. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@leader` — [w:ST_TabTlc](values-word.md#w-st_tabtlc); Strict, Transitional; use=optional. Partial: `tabs` retains alignment/leader/position fields; permissive retention does not implement all tab-stop values. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `@pos` — [w:ST_SignedTwipsMeasure](values-word.md#w-st_signedtwipsmeasure); Strict, Transitional; use=required. Partial: `tabs` retains alignment/leader/position fields; permissive retention does not implement all tab-stop values. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).

<a id="w-ct_pprbase"></a>

## w:CT_PPrBase

Sources: [Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd).

- [x] `pStyle` — [w:CT_String](word-paragraphs.md#w-ct_string); Strict, Transitional; minOccurs=0. Model support: `style` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `keepNext` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Keep Paragraph With Next Paragraph; [§17.3.1.15](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0233-0237-0049.md#source-page-236-part-1). Model support: `keep_with_next` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `keepLines` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Keep All Lines On One Page; [§17.3.1.14](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0233-0237-0049.md#source-page-235-part-1). Model support: `keep_together` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [x] `pageBreakBefore` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Start Paragraph on Next Page; [§17.3.1.23](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-247-part-1). Model support: `page_break_before` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `framePr` — [w:CT_FramePr](word-paragraphs.md#w-ct_framepr); Strict, Transitional; minOccurs=0. Text Frame Properties; [§17.3.1.11](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0216-0220-0045.md#source-page-218-part-1).
- [x] `widowControl` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow First/Last Line to Display on a Separate Page; [§17.3.1.44](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-271-part-1). Model support: `widow_control` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `numPr` — [w:CT_NumPr](word-paragraphs.md#w-ct_numpr); Strict, Transitional; minOccurs=0. Numbering Definition Instance Reference; [§17.3.1.19](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-243-part-1). Partial: List and heading numbering emits level/instance references and start overrides; numbering revision metadata is not represented. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs), [docx_lists.rs](../../../tests/docx_lists.rs), [docx_packages.rs](../../../tests/docx_packages.rs).
- [ ] `suppressLineNumbers` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Suppress Line Numbers for Paragraph; [§17.3.1.35](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-262-part-1).
- [ ] `pBdr` — [w:CT_PBdr](word-paragraphs.md#w-ct_pbdr); Strict, Transitional; minOccurs=0. Paragraph Borders; [§17.3.1.24](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0249-0254-0052.md#source-page-249-part-1). Partial: `border` retains side objects as metadata; complete border values and style semantics are not implemented. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `shd` — [w:CT_Shd](word-paragraphs.md#w-ct_shd); Strict, Transitional; minOccurs=0. Partial: `fill` covers hexadecimal background color; patterns, automatic colors, and theme-based shading are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `tabs` — [w:CT_Tabs](word-paragraphs.md#w-ct_tabs); Strict, Transitional; minOccurs=0. Set of Custom Tab Stops; [§17.3.1.38](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-265-part-1). Partial: `tabs` retains position/alignment/leader metadata; complete stop semantics and schema value validation are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `suppressAutoHyphens` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Suppress Hyphenation for Paragraph; [§17.3.1.34](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-261-part-1).
- [ ] `kinsoku` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Use East Asian Typography Rules for First and Last Character per Line; [§17.3.1.16](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0238-0242-0050.md#source-page-238-part-1).
- [ ] `wordWrap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow Line Breaking At Character Level; [§17.3.1.45](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-272-part-1).
- [ ] `overflowPunct` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Allow Punctuation to Extend Past Text Extents; [§17.3.1.21](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-244-part-1).
- [ ] `topLinePunct` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Compress Punctuation at Start of a Line; [§17.3.1.43](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0271-0276-0056.md#source-page-271-part-1).
- [ ] `autoSpaceDE` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Spacing of Latin and East Asian Text; [§17.3.1.2](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-205-part-1).
- [ ] `autoSpaceDN` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Spacing of East Asian Text and Numbers; [§17.3.1.3](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-206-part-1).
- [ ] `bidi` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `adjustRightInd` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Automatically Adjust Right Indent When Using Document Grid; [§17.3.1.1](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0205-0210-0043.md#source-page-205-part-1).
- [ ] `snapToGrid` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0.
- [ ] `spacing` — [w:CT_Spacing](word-paragraphs.md#w-ct_spacing); Strict, Transitional; minOccurs=0. Partial: `spacing` and flat aliases cover physical before/after spacing and three line rules; line-count and automatic before/after controls are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `ind` — [w:CT_Ind](word-paragraphs.md#w-ct_ind); Strict, Transitional; minOccurs=0. Paragraph Indentation; [§17.3.1.12](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0229-0232-0048.md#source-page-229-part-1). Partial: `indent` covers non-negative physical left/right/first-line/hanging values; signed, character-count, and logical start/end controls are missing. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `contextualSpacing` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Ignore Spacing Above and Below When Using Identical Styles; [§17.3.1.9](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0216-0220-0045.md#source-page-216-part-1).
- [ ] `mirrorIndents` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Use Left/Right Indents as Inside/Outside Indents; [§17.3.1.18](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0238-0242-0050.md#source-page-242-part-1).
- [ ] `suppressOverlap` — [w:CT_OnOff](word-paragraphs.md#w-ct_onoff); Strict, Transitional; minOccurs=0. Prevent Text Frames From Overlapping; [§17.3.1.36](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0260-0264-0054.md#source-page-263-part-1).
- [ ] `jc` — [w:CT_Jc](word-paragraphs.md#w-ct_jc); Strict, Transitional; minOccurs=0. Partial: `alignment` supports left, center, right, and justify; other Word alignment values have no defined Mote mapping. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `textDirection` — [w:CT_TextDirection](word-paragraphs.md#w-ct_textdirection); Strict, Transitional; minOccurs=0.
- [ ] `textAlignment` — [w:CT_TextAlignment](word-paragraphs.md#w-ct_textalignment); Strict, Transitional; minOccurs=0. Vertical Character Alignment on Line; [§17.3.1.39](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-265-part-1).
- [ ] `textboxTightWrap` — [w:CT_TextboxTightWrap](word-paragraphs.md#w-ct_textboxtightwrap); Strict, Transitional; minOccurs=0. Allow Surrounding Paragraphs to Tight Wrap to Text Box Contents; [§17.3.1.40](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0265-0270-0055.md#source-page-267-part-1).
- [x] `outlineLvl` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0. Associated Outline Level; [§17.3.1.20](../../reference/reading/ooxml/ecma-376/part-1/chunks/pages-0243-0248-0051.md#source-page-243-part-1). Model support: `outline_level` on paragraphs/headings and applicable style declarations; formatting remains authored intent. Evidence: [formatting-attributes.md](../../../docs/specification/formatting-attributes.md), [attributes.rs](../../../tests/attributes.rs).
- [ ] `divId` — [w:CT_DecimalNumber](word-paragraphs.md#w-ct_decimalnumber); Strict, Transitional; minOccurs=0.
- [ ] `cnfStyle` — [w:CT_Cnf](word-paragraphs.md#w-ct_cnf); Strict, Transitional; minOccurs=0; maxOccurs=1.
