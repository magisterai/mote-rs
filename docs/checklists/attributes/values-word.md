# Word value definitions

[Inventory guide](README.md)

These are complete schema value domains for the selected properties.
Property rows record Mote support and supported subsets separately.
Identical Strict and Transitional definitions are combined.
See also [prose-defined extensions](prose-extensions.md).

## Types

- [w:ST_AnnotationVMerge](#w-st_annotationvmerge)
- [w:ST_Border](#w-st_border)
- [w:ST_BrClear](#w-st_brclear)
- [w:ST_BrType](#w-st_brtype)
- [w:ST_CaptionPos](#w-st_captionpos)
- [w:ST_ChapterSep](#w-st_chaptersep)
- [w:ST_Cnf](#w-st_cnf)
- [w:ST_CombineBrackets](#w-st_combinebrackets)
- [w:ST_DateTime](#w-st_datetime)
- [w:ST_DecimalNumber](#w-st_decimalnumber)
- [w:ST_DecimalNumberOrPercent](#w-st_decimalnumberorpercent)
- [w:ST_DisplacedByCustomXml](#w-st_displacedbycustomxml)
- [w:ST_DocGrid](#w-st_docgrid)
- [w:ST_DropCap](#w-st_dropcap)
- [w:ST_EdnPos](#w-st_ednpos)
- [w:ST_EighthPointMeasure](#w-st_eighthpointmeasure)
- [w:ST_Em](#w-st_em)
- [w:ST_FFHelpTextVal](#w-st_ffhelptextval)
- [w:ST_FFName](#w-st_ffname)
- [w:ST_FFStatusTextVal](#w-st_ffstatustextval)
- [w:ST_FFTextType](#w-st_fftexttype)
- [w:ST_FldCharType](#w-st_fldchartype)
- [w:ST_FontFamily](#w-st_fontfamily)
- [w:ST_FtnPos](#w-st_ftnpos)
- [w:ST_HAnchor](#w-st_hanchor)
- [w:ST_HdrFtr](#w-st_hdrftr)
- [w:ST_HeightRule](#w-st_heightrule)
- [w:ST_HexColor](#w-st_hexcolor)
- [w:ST_HexColorAuto](#w-st_hexcolorauto)
- [w:ST_HexColorRGB](#w-st_hexcolorrgb)
- [w:ST_HighlightColor](#w-st_highlightcolor)
- [w:ST_Hint](#w-st_hint)
- [w:ST_HpsMeasure](#w-st_hpsmeasure)
- [w:ST_InfoTextType](#w-st_infotexttype)
- [w:ST_Jc](#w-st_jc)
- [w:ST_JcTable](#w-st_jctable)
- [w:ST_LevelSuffix](#w-st_levelsuffix)
- [w:ST_LineNumberRestart](#w-st_linenumberrestart)
- [w:ST_LineSpacingRule](#w-st_linespacingrule)
- [w:ST_Lock](#w-st_lock)
- [w:ST_LongHexNumber](#w-st_longhexnumber)
- [w:ST_MacroName](#w-st_macroname)
- [w:ST_MeasurementOrPercent](#w-st_measurementorpercent)
- [w:ST_Merge](#w-st_merge)
- [w:ST_MultiLevelType](#w-st_multileveltype)
- [w:ST_NumberFormat](#w-st_numberformat)
- [w:ST_OnOff](#w-st_onoff)
- [w:ST_PageBorderDisplay](#w-st_pageborderdisplay)
- [w:ST_PageBorderOffset](#w-st_pageborderoffset)
- [w:ST_PageBorderZOrder](#w-st_pageborderzorder)
- [w:ST_PageOrientation](#w-st_pageorientation)
- [w:ST_Pitch](#w-st_pitch)
- [w:ST_PointMeasure](#w-st_pointmeasure)
- [w:ST_RestartNumber](#w-st_restartnumber)
- [w:ST_SdtDateMappingType](#w-st_sdtdatemappingtype)
- [w:ST_SectionMark](#w-st_sectionmark)
- [w:ST_Shd](#w-st_shd)
- [w:ST_ShortHexNumber](#w-st_shorthexnumber)
- [w:ST_SignedHpsMeasure](#w-st_signedhpsmeasure)
- [w:ST_SignedTwipsMeasure](#w-st_signedtwipsmeasure)
- [w:ST_String](#w-st_string)
- [w:ST_StyleType](#w-st_styletype)
- [w:ST_TabJc](#w-st_tabjc)
- [w:ST_TabTlc](#w-st_tabtlc)
- [w:ST_TblLayoutType](#w-st_tbllayouttype)
- [w:ST_TblOverlap](#w-st_tbloverlap)
- [w:ST_TblStyleOverrideType](#w-st_tblstyleoverridetype)
- [w:ST_TblWidth](#w-st_tblwidth)
- [w:ST_TextAlignment](#w-st_textalignment)
- [w:ST_TextDirection](#w-st_textdirection)
- [w:ST_TextEffect](#w-st_texteffect)
- [w:ST_TextScale](#w-st_textscale)
- [w:ST_TextScaleDecimal](#w-st_textscaledecimal)
- [w:ST_TextScalePercent](#w-st_textscalepercent)
- [w:ST_TextboxTightWrap](#w-st_textboxtightwrap)
- [w:ST_Theme](#w-st_theme)
- [w:ST_ThemeColor](#w-st_themecolor)
- [w:ST_UcharHexNumber](#w-st_ucharhexnumber)
- [w:ST_Underline](#w-st_underline)
- [w:ST_UnqualifiedPercentage](#w-st_unqualifiedpercentage)
- [w:ST_UnsignedDecimalNumber](#w-st_unsigneddecimalnumber)
- [w:ST_VAnchor](#w-st_vanchor)
- [w:ST_VerticalJc](#w-st_verticaljc)
- [w:ST_Wrap](#w-st_wrap)

<a id="w-st_annotationvmerge"></a>

## w:ST_AnnotationVMerge

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `cont`
- enumeration: value: `rest`

<a id="w-st_border"></a>

## w:ST_Border

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `nil`
- enumeration: value: `none`
- enumeration: value: `single`
- enumeration: value: `thick`
- enumeration: value: `double`
- enumeration: value: `dotted`
- enumeration: value: `dashed`
- enumeration: value: `dotDash`
- enumeration: value: `dotDotDash`
- enumeration: value: `triple`
- enumeration: value: `thinThickSmallGap`
- enumeration: value: `thickThinSmallGap`
- enumeration: value: `thinThickThinSmallGap`
- enumeration: value: `thinThickMediumGap`
- enumeration: value: `thickThinMediumGap`
- enumeration: value: `thinThickThinMediumGap`
- enumeration: value: `thinThickLargeGap`
- enumeration: value: `thickThinLargeGap`
- enumeration: value: `thinThickThinLargeGap`
- enumeration: value: `wave`
- enumeration: value: `doubleWave`
- enumeration: value: `dashSmallGap`
- enumeration: value: `dashDotStroked`
- enumeration: value: `threeDEmboss`
- enumeration: value: `threeDEngrave`
- enumeration: value: `outset`
- enumeration: value: `inset`
- enumeration: value: `apples`
- enumeration: value: `archedScallops`
- enumeration: value: `babyPacifier`
- enumeration: value: `babyRattle`
- enumeration: value: `balloons3Colors`
- enumeration: value: `balloonsHotAir`
- enumeration: value: `basicBlackDashes`
- enumeration: value: `basicBlackDots`
- enumeration: value: `basicBlackSquares`
- enumeration: value: `basicThinLines`
- enumeration: value: `basicWhiteDashes`
- enumeration: value: `basicWhiteDots`
- enumeration: value: `basicWhiteSquares`
- enumeration: value: `basicWideInline`
- enumeration: value: `basicWideMidline`
- enumeration: value: `basicWideOutline`
- enumeration: value: `bats`
- enumeration: value: `birds`
- enumeration: value: `birdsFlight`
- enumeration: value: `cabins`
- enumeration: value: `cakeSlice`
- enumeration: value: `candyCorn`
- enumeration: value: `celticKnotwork`
- enumeration: value: `certificateBanner`
- enumeration: value: `chainLink`
- enumeration: value: `champagneBottle`
- enumeration: value: `checkedBarBlack`
- enumeration: value: `checkedBarColor`
- enumeration: value: `checkered`
- enumeration: value: `christmasTree`
- enumeration: value: `circlesLines`
- enumeration: value: `circlesRectangles`
- enumeration: value: `classicalWave`
- enumeration: value: `clocks`
- enumeration: value: `compass`
- enumeration: value: `confetti`
- enumeration: value: `confettiGrays`
- enumeration: value: `confettiOutline`
- enumeration: value: `confettiStreamers`
- enumeration: value: `confettiWhite`
- enumeration: value: `cornerTriangles`
- enumeration: value: `couponCutoutDashes`
- enumeration: value: `couponCutoutDots`
- enumeration: value: `crazyMaze`
- enumeration: value: `creaturesButterfly`
- enumeration: value: `creaturesFish`
- enumeration: value: `creaturesInsects`
- enumeration: value: `creaturesLadyBug`
- enumeration: value: `crossStitch`
- enumeration: value: `cup`
- enumeration: value: `decoArch`
- enumeration: value: `decoArchColor`
- enumeration: value: `decoBlocks`
- enumeration: value: `diamondsGray`
- enumeration: value: `doubleD`
- enumeration: value: `doubleDiamonds`
- enumeration: value: `earth1`
- enumeration: value: `earth2`
- enumeration: value: `earth3`
- enumeration: value: `eclipsingSquares1`
- enumeration: value: `eclipsingSquares2`
- enumeration: value: `eggsBlack`
- enumeration: value: `fans`
- enumeration: value: `film`
- enumeration: value: `firecrackers`
- enumeration: value: `flowersBlockPrint`
- enumeration: value: `flowersDaisies`
- enumeration: value: `flowersModern1`
- enumeration: value: `flowersModern2`
- enumeration: value: `flowersPansy`
- enumeration: value: `flowersRedRose`
- enumeration: value: `flowersRoses`
- enumeration: value: `flowersTeacup`
- enumeration: value: `flowersTiny`
- enumeration: value: `gems`
- enumeration: value: `gingerbreadMan`
- enumeration: value: `gradient`
- enumeration: value: `handmade1`
- enumeration: value: `handmade2`
- enumeration: value: `heartBalloon`
- enumeration: value: `heartGray`
- enumeration: value: `hearts`
- enumeration: value: `heebieJeebies`
- enumeration: value: `holly`
- enumeration: value: `houseFunky`
- enumeration: value: `hypnotic`
- enumeration: value: `iceCreamCones`
- enumeration: value: `lightBulb`
- enumeration: value: `lightning1`
- enumeration: value: `lightning2`
- enumeration: value: `mapPins`
- enumeration: value: `mapleLeaf`
- enumeration: value: `mapleMuffins`
- enumeration: value: `marquee`
- enumeration: value: `marqueeToothed`
- enumeration: value: `moons`
- enumeration: value: `mosaic`
- enumeration: value: `musicNotes`
- enumeration: value: `northwest`
- enumeration: value: `ovals`
- enumeration: value: `packages`
- enumeration: value: `palmsBlack`
- enumeration: value: `palmsColor`
- enumeration: value: `paperClips`
- enumeration: value: `papyrus`
- enumeration: value: `partyFavor`
- enumeration: value: `partyGlass`
- enumeration: value: `pencils`
- enumeration: value: `people`
- enumeration: value: `peopleWaving`
- enumeration: value: `peopleHats`
- enumeration: value: `poinsettias`
- enumeration: value: `postageStamp`
- enumeration: value: `pumpkin1`
- enumeration: value: `pushPinNote2`
- enumeration: value: `pushPinNote1`
- enumeration: value: `pyramids`
- enumeration: value: `pyramidsAbove`
- enumeration: value: `quadrants`
- enumeration: value: `rings`
- enumeration: value: `safari`
- enumeration: value: `sawtooth`
- enumeration: value: `sawtoothGray`
- enumeration: value: `scaredCat`
- enumeration: value: `seattle`
- enumeration: value: `shadowedSquares`
- enumeration: value: `sharksTeeth`
- enumeration: value: `shorebirdTracks`
- enumeration: value: `skyrocket`
- enumeration: value: `snowflakeFancy`
- enumeration: value: `snowflakes`
- enumeration: value: `sombrero`
- enumeration: value: `southwest`
- enumeration: value: `stars`
- enumeration: value: `starsTop`
- enumeration: value: `stars3d`
- enumeration: value: `starsBlack`
- enumeration: value: `starsShadowed`
- enumeration: value: `sun`
- enumeration: value: `swirligig`
- enumeration: value: `tornPaper`
- enumeration: value: `tornPaperBlack`
- enumeration: value: `trees`
- enumeration: value: `triangleParty`
- enumeration: value: `triangles`
- enumeration: value: `triangle1`
- enumeration: value: `triangle2`
- enumeration: value: `triangleCircle1`
- enumeration: value: `triangleCircle2`
- enumeration: value: `shapes1`
- enumeration: value: `shapes2`
- enumeration: value: `twistedLines1`
- enumeration: value: `twistedLines2`
- enumeration: value: `vine`
- enumeration: value: `waveline`
- enumeration: value: `weavingAngles`
- enumeration: value: `weavingBraid`
- enumeration: value: `weavingRibbon`
- enumeration: value: `weavingStrips`
- enumeration: value: `whiteFlowers`
- enumeration: value: `woodwork`
- enumeration: value: `xIllusions`
- enumeration: value: `zanyTriangles`
- enumeration: value: `zigZag`
- enumeration: value: `zigZagStitch`
- enumeration: value: `custom`

<a id="w-st_brclear"></a>

## w:ST_BrClear

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `left`
- enumeration: value: `right`
- enumeration: value: `all`

<a id="w-st_brtype"></a>

## w:ST_BrType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `page`
- enumeration: value: `column`
- enumeration: value: `textWrapping`

<a id="w-st_captionpos"></a>

## w:ST_CaptionPos

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `above`
- enumeration: value: `below`
- enumeration: value: `left`
- enumeration: value: `right`

<a id="w-st_chaptersep"></a>

## w:ST_ChapterSep

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `hyphen`
- enumeration: value: `period`
- enumeration: value: `colon`
- enumeration: value: `emDash`
- enumeration: value: `enDash`

<a id="w-st_cnf"></a>

## w:ST_Cnf

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- restriction: base: `xsd:string`
- length: value: `12`
- pattern: value: `[01]*`

<a id="w-st_combinebrackets"></a>

## w:ST_CombineBrackets

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `round`
- enumeration: value: `square`
- enumeration: value: `angle`
- enumeration: value: `curly`

<a id="w-st_datetime"></a>

## w:ST_DateTime

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:dateTime`

<a id="w-st_decimalnumber"></a>

## w:ST_DecimalNumber

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:integer`

<a id="w-st_decimalnumberorpercent"></a>

## w:ST_DecimalNumberOrPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- union: memberTypes: [w:ST_UnqualifiedPercentage](values-word.md#w-st_unqualifiedpercentage), [s:ST_Percentage](values-shared.md#s-st_percentage)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- union: memberTypes: [s:ST_Percentage](values-shared.md#s-st_percentage)

<a id="w-st_displacedbycustomxml"></a>

## w:ST_DisplacedByCustomXml

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `next`
- enumeration: value: `prev`

<a id="w-st_docgrid"></a>

## w:ST_DocGrid

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `default`
- enumeration: value: `lines`
- enumeration: value: `linesAndChars`
- enumeration: value: `snapToChars`

<a id="w-st_dropcap"></a>

## w:ST_DropCap

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `drop`
- enumeration: value: `margin`

<a id="w-st_ednpos"></a>

## w:ST_EdnPos

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `sectEnd`
- enumeration: value: `docEnd`

<a id="w-st_eighthpointmeasure"></a>

## w:ST_EighthPointMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: [s:ST_UnsignedDecimalNumber](values-shared.md#s-st_unsigneddecimalnumber)

<a id="w-st_em"></a>

## w:ST_Em

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `dot`
- enumeration: value: `comma`
- enumeration: value: `circle`
- enumeration: value: `underDot`

<a id="w-st_ffhelptextval"></a>

## w:ST_FFHelpTextVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- maxLength: value: `256`

<a id="w-st_ffname"></a>

## w:ST_FFName

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- maxLength: value: `65`

<a id="w-st_ffstatustextval"></a>

## w:ST_FFStatusTextVal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- maxLength: value: `140`

<a id="w-st_fftexttype"></a>

## w:ST_FFTextType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `regular`
- enumeration: value: `number`
- enumeration: value: `date`
- enumeration: value: `currentTime`
- enumeration: value: `currentDate`
- enumeration: value: `calculated`

<a id="w-st_fldchartype"></a>

## w:ST_FldCharType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `begin`
- enumeration: value: `separate`
- enumeration: value: `end`

<a id="w-st_fontfamily"></a>

## w:ST_FontFamily

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `decorative`
- enumeration: value: `modern`
- enumeration: value: `roman`
- enumeration: value: `script`
- enumeration: value: `swiss`
- enumeration: value: `auto`

<a id="w-st_ftnpos"></a>

## w:ST_FtnPos

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `pageBottom`
- enumeration: value: `beneathText`
- enumeration: value: `sectEnd`
- enumeration: value: `docEnd`

<a id="w-st_hanchor"></a>

## w:ST_HAnchor

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `text`
- enumeration: value: `margin`
- enumeration: value: `page`

<a id="w-st_hdrftr"></a>

## w:ST_HdrFtr

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `even`
- enumeration: value: `default`
- enumeration: value: `first`

<a id="w-st_heightrule"></a>

## w:ST_HeightRule

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `auto`
- enumeration: value: `exact`
- enumeration: value: `atLeast`

<a id="w-st_hexcolor"></a>

## w:ST_HexColor

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- union: memberTypes: [w:ST_HexColorAuto](values-word.md#w-st_hexcolorauto), [s:ST_HexColorRGB](values-shared.md#s-st_hexcolorrgb)

<a id="w-st_hexcolorauto"></a>

## w:ST_HexColorAuto

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `auto`

<a id="w-st_hexcolorrgb"></a>

## w:ST_HexColorRGB

Publisher imports use this legacy name; the corresponding shared
value definition is `s:ST_HexColorRGB` in the retained ECMA schemas.

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:hexBinary`
- length: value: `3`; fixed: `true`

<a id="w-st_highlightcolor"></a>

## w:ST_HighlightColor

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `black`
- enumeration: value: `blue`
- enumeration: value: `cyan`
- enumeration: value: `green`
- enumeration: value: `magenta`
- enumeration: value: `red`
- enumeration: value: `yellow`
- enumeration: value: `white`
- enumeration: value: `darkBlue`
- enumeration: value: `darkCyan`
- enumeration: value: `darkGreen`
- enumeration: value: `darkMagenta`
- enumeration: value: `darkRed`
- enumeration: value: `darkYellow`
- enumeration: value: `darkGray`
- enumeration: value: `lightGray`
- enumeration: value: `none`

<a id="w-st_hint"></a>

## w:ST_Hint

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `default`
- enumeration: value: `eastAsia`

<a id="w-st_hpsmeasure"></a>

## w:ST_HpsMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- union: memberTypes: [s:ST_UnsignedDecimalNumber](values-shared.md#s-st_unsigneddecimalnumber), [s:ST_PositiveUniversalMeasure](values-shared.md#s-st_positiveuniversalmeasure)

<a id="w-st_infotexttype"></a>

## w:ST_InfoTextType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `text`
- enumeration: value: `autoText`

<a id="w-st_jc"></a>

## w:ST_Jc

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `start`
- enumeration: value: `center`
- enumeration: value: `end`
- enumeration: value: `both`
- enumeration: value: `mediumKashida`
- enumeration: value: `distribute`
- enumeration: value: `numTab`
- enumeration: value: `highKashida`
- enumeration: value: `lowKashida`
- enumeration: value: `thaiDistribute`
- enumeration: value: `left`
- enumeration: value: `right`

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `start`
- enumeration: value: `center`
- enumeration: value: `end`
- enumeration: value: `both`
- enumeration: value: `mediumKashida`
- enumeration: value: `distribute`
- enumeration: value: `numTab`
- enumeration: value: `highKashida`
- enumeration: value: `lowKashida`
- enumeration: value: `thaiDistribute`

<a id="w-st_jctable"></a>

## w:ST_JcTable

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `center`
- enumeration: value: `end`
- enumeration: value: `left`
- enumeration: value: `right`
- enumeration: value: `start`

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `center`
- enumeration: value: `end`
- enumeration: value: `start`

<a id="w-st_levelsuffix"></a>

## w:ST_LevelSuffix

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `tab`
- enumeration: value: `space`
- enumeration: value: `nothing`

<a id="w-st_linenumberrestart"></a>

## w:ST_LineNumberRestart

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `newPage`
- enumeration: value: `newSection`
- enumeration: value: `continuous`

<a id="w-st_linespacingrule"></a>

## w:ST_LineSpacingRule

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `auto`
- enumeration: value: `exact`
- enumeration: value: `atLeast`

<a id="w-st_lock"></a>

## w:ST_Lock

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `sdtLocked`
- enumeration: value: `contentLocked`
- enumeration: value: `unlocked`
- enumeration: value: `sdtContentLocked`

<a id="w-st_longhexnumber"></a>

## w:ST_LongHexNumber

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:hexBinary`
- length: value: `4`

<a id="w-st_macroname"></a>

## w:ST_MacroName

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- maxLength: value: `33`

<a id="w-st_measurementorpercent"></a>

## w:ST_MeasurementOrPercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- union: memberTypes: [w:ST_DecimalNumberOrPercent](values-word.md#w-st_decimalnumberorpercent), [s:ST_UniversalMeasure](values-shared.md#s-st_universalmeasure)

<a id="w-st_merge"></a>

## w:ST_Merge

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `continue`
- enumeration: value: `restart`

<a id="w-st_multileveltype"></a>

## w:ST_MultiLevelType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `singleLevel`
- enumeration: value: `multilevel`
- enumeration: value: `hybridMultilevel`

<a id="w-st_numberformat"></a>

## w:ST_NumberFormat

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `decimal`
- enumeration: value: `upperRoman`
- enumeration: value: `lowerRoman`
- enumeration: value: `upperLetter`
- enumeration: value: `lowerLetter`
- enumeration: value: `ordinal`
- enumeration: value: `cardinalText`
- enumeration: value: `ordinalText`
- enumeration: value: `hex`
- enumeration: value: `chicago`
- enumeration: value: `ideographDigital`
- enumeration: value: `japaneseCounting`
- enumeration: value: `aiueo`
- enumeration: value: `iroha`
- enumeration: value: `decimalFullWidth`
- enumeration: value: `decimalHalfWidth`
- enumeration: value: `japaneseLegal`
- enumeration: value: `japaneseDigitalTenThousand`
- enumeration: value: `decimalEnclosedCircle`
- enumeration: value: `decimalFullWidth2`
- enumeration: value: `aiueoFullWidth`
- enumeration: value: `irohaFullWidth`
- enumeration: value: `decimalZero`
- enumeration: value: `bullet`
- enumeration: value: `ganada`
- enumeration: value: `chosung`
- enumeration: value: `decimalEnclosedFullstop`
- enumeration: value: `decimalEnclosedParen`
- enumeration: value: `decimalEnclosedCircleChinese`
- enumeration: value: `ideographEnclosedCircle`
- enumeration: value: `ideographTraditional`
- enumeration: value: `ideographZodiac`
- enumeration: value: `ideographZodiacTraditional`
- enumeration: value: `taiwaneseCounting`
- enumeration: value: `ideographLegalTraditional`
- enumeration: value: `taiwaneseCountingThousand`
- enumeration: value: `taiwaneseDigital`
- enumeration: value: `chineseCounting`
- enumeration: value: `chineseLegalSimplified`
- enumeration: value: `chineseCountingThousand`
- enumeration: value: `koreanDigital`
- enumeration: value: `koreanCounting`
- enumeration: value: `koreanLegal`
- enumeration: value: `koreanDigital2`
- enumeration: value: `vietnameseCounting`
- enumeration: value: `russianLower`
- enumeration: value: `russianUpper`
- enumeration: value: `none`
- enumeration: value: `numberInDash`
- enumeration: value: `hebrew1`
- enumeration: value: `hebrew2`
- enumeration: value: `arabicAlpha`
- enumeration: value: `arabicAbjad`
- enumeration: value: `hindiVowels`
- enumeration: value: `hindiConsonants`
- enumeration: value: `hindiNumbers`
- enumeration: value: `hindiCounting`
- enumeration: value: `thaiLetters`
- enumeration: value: `thaiNumbers`
- enumeration: value: `thaiCounting`
- enumeration: value: `bahtText`
- enumeration: value: `dollarText`
- enumeration: value: `custom`

<a id="w-st_onoff"></a>

## w:ST_OnOff

Publisher imports use this legacy name; the corresponding shared
value definition is `s:ST_OnOff` in the retained ECMA schemas.

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd):

- union: memberTypes: `xsd:boolean`, [s:ST_OnOff1](values-shared.md#s-st_onoff1)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- union: memberTypes: `xsd:boolean`

<a id="w-st_pageborderdisplay"></a>

## w:ST_PageBorderDisplay

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `allPages`
- enumeration: value: `firstPage`
- enumeration: value: `notFirstPage`

<a id="w-st_pageborderoffset"></a>

## w:ST_PageBorderOffset

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `page`
- enumeration: value: `text`

<a id="w-st_pageborderzorder"></a>

## w:ST_PageBorderZOrder

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `front`
- enumeration: value: `back`

<a id="w-st_pageorientation"></a>

## w:ST_PageOrientation

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `portrait`
- enumeration: value: `landscape`

<a id="w-st_pitch"></a>

## w:ST_Pitch

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `fixed`
- enumeration: value: `variable`
- enumeration: value: `default`

<a id="w-st_pointmeasure"></a>

## w:ST_PointMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: [s:ST_UnsignedDecimalNumber](values-shared.md#s-st_unsigneddecimalnumber)

<a id="w-st_restartnumber"></a>

## w:ST_RestartNumber

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `continuous`
- enumeration: value: `eachSect`
- enumeration: value: `eachPage`

<a id="w-st_sdtdatemappingtype"></a>

## w:ST_SdtDateMappingType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `text`
- enumeration: value: `date`
- enumeration: value: `dateTime`

<a id="w-st_sectionmark"></a>

## w:ST_SectionMark

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `nextPage`
- enumeration: value: `nextColumn`
- enumeration: value: `continuous`
- enumeration: value: `evenPage`
- enumeration: value: `oddPage`

<a id="w-st_shd"></a>

## w:ST_Shd

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `nil`
- enumeration: value: `clear`
- enumeration: value: `solid`
- enumeration: value: `horzStripe`
- enumeration: value: `vertStripe`
- enumeration: value: `reverseDiagStripe`
- enumeration: value: `diagStripe`
- enumeration: value: `horzCross`
- enumeration: value: `diagCross`
- enumeration: value: `thinHorzStripe`
- enumeration: value: `thinVertStripe`
- enumeration: value: `thinReverseDiagStripe`
- enumeration: value: `thinDiagStripe`
- enumeration: value: `thinHorzCross`
- enumeration: value: `thinDiagCross`
- enumeration: value: `pct5`
- enumeration: value: `pct10`
- enumeration: value: `pct12`
- enumeration: value: `pct15`
- enumeration: value: `pct20`
- enumeration: value: `pct25`
- enumeration: value: `pct30`
- enumeration: value: `pct35`
- enumeration: value: `pct37`
- enumeration: value: `pct40`
- enumeration: value: `pct45`
- enumeration: value: `pct50`
- enumeration: value: `pct55`
- enumeration: value: `pct60`
- enumeration: value: `pct62`
- enumeration: value: `pct65`
- enumeration: value: `pct70`
- enumeration: value: `pct75`
- enumeration: value: `pct80`
- enumeration: value: `pct85`
- enumeration: value: `pct87`
- enumeration: value: `pct90`
- enumeration: value: `pct95`

<a id="w-st_shorthexnumber"></a>

## w:ST_ShortHexNumber

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:hexBinary`
- length: value: `2`

<a id="w-st_signedhpsmeasure"></a>

## w:ST_SignedHpsMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- union: memberTypes: `xsd:integer`, [s:ST_UniversalMeasure](values-shared.md#s-st_universalmeasure)

<a id="w-st_signedtwipsmeasure"></a>

## w:ST_SignedTwipsMeasure

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- union: memberTypes: `xsd:integer`, [s:ST_UniversalMeasure](values-shared.md#s-st_universalmeasure)

<a id="w-st_string"></a>

## w:ST_String

Publisher imports use this legacy name; the corresponding shared
value definition is `s:ST_String` in the retained ECMA schemas.

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:string`

<a id="w-st_styletype"></a>

## w:ST_StyleType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `paragraph`
- enumeration: value: `character`
- enumeration: value: `table`
- enumeration: value: `numbering`

<a id="w-st_tabjc"></a>

## w:ST_TabJc

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `clear`
- enumeration: value: `start`
- enumeration: value: `center`
- enumeration: value: `end`
- enumeration: value: `decimal`
- enumeration: value: `bar`
- enumeration: value: `num`
- enumeration: value: `left`
- enumeration: value: `right`

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `clear`
- enumeration: value: `start`
- enumeration: value: `center`
- enumeration: value: `end`
- enumeration: value: `decimal`
- enumeration: value: `bar`
- enumeration: value: `num`

<a id="w-st_tabtlc"></a>

## w:ST_TabTlc

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `dot`
- enumeration: value: `hyphen`
- enumeration: value: `underscore`
- enumeration: value: `heavy`
- enumeration: value: `middleDot`

<a id="w-st_tbllayouttype"></a>

## w:ST_TblLayoutType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `fixed`
- enumeration: value: `autofit`

<a id="w-st_tbloverlap"></a>

## w:ST_TblOverlap

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `never`
- enumeration: value: `overlap`

<a id="w-st_tblstyleoverridetype"></a>

## w:ST_TblStyleOverrideType

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `wholeTable`
- enumeration: value: `firstRow`
- enumeration: value: `lastRow`
- enumeration: value: `firstCol`
- enumeration: value: `lastCol`
- enumeration: value: `band1Vert`
- enumeration: value: `band2Vert`
- enumeration: value: `band1Horz`
- enumeration: value: `band2Horz`
- enumeration: value: `neCell`
- enumeration: value: `nwCell`
- enumeration: value: `seCell`
- enumeration: value: `swCell`

<a id="w-st_tblwidth"></a>

## w:ST_TblWidth

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `nil`
- enumeration: value: `pct`
- enumeration: value: `dxa`
- enumeration: value: `auto`

<a id="w-st_textalignment"></a>

## w:ST_TextAlignment

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `top`
- enumeration: value: `center`
- enumeration: value: `baseline`
- enumeration: value: `bottom`
- enumeration: value: `auto`

<a id="w-st_textdirection"></a>

## w:ST_TextDirection

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `tb`
- enumeration: value: `rl`
- enumeration: value: `lr`
- enumeration: value: `tbV`
- enumeration: value: `rlV`
- enumeration: value: `lrV`
- enumeration: value: `btLr`
- enumeration: value: `lrTb`
- enumeration: value: `lrTbV`
- enumeration: value: `tbLrV`
- enumeration: value: `tbRl`
- enumeration: value: `tbRlV`

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `tb`
- enumeration: value: `rl`
- enumeration: value: `lr`
- enumeration: value: `tbV`
- enumeration: value: `rlV`
- enumeration: value: `lrV`

<a id="w-st_texteffect"></a>

## w:ST_TextEffect

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `blinkBackground`
- enumeration: value: `lights`
- enumeration: value: `antsBlack`
- enumeration: value: `antsRed`
- enumeration: value: `shimmer`
- enumeration: value: `sparkle`
- enumeration: value: `none`

<a id="w-st_textscale"></a>

## w:ST_TextScale

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- union: memberTypes: [w:ST_TextScalePercent](values-word.md#w-st_textscalepercent), [w:ST_TextScaleDecimal](values-word.md#w-st_textscaledecimal)

[Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- union: memberTypes: [w:ST_TextScalePercent](values-word.md#w-st_textscalepercent)

<a id="w-st_textscaledecimal"></a>

## w:ST_TextScaleDecimal

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- restriction: base: `xsd:integer`
- minInclusive: value: `0`
- maxInclusive: value: `600`

<a id="w-st_textscalepercent"></a>

## w:ST_TextScalePercent

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- pattern: value: `0*(600|([0-5]?[0-9]?[0-9]))%`

<a id="w-st_textboxtightwrap"></a>

## w:ST_TextboxTightWrap

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `none`
- enumeration: value: `allLines`
- enumeration: value: `firstAndLastLine`
- enumeration: value: `firstLineOnly`
- enumeration: value: `lastLineOnly`

<a id="w-st_theme"></a>

## w:ST_Theme

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `majorEastAsia`
- enumeration: value: `majorBidi`
- enumeration: value: `majorAscii`
- enumeration: value: `majorHAnsi`
- enumeration: value: `minorEastAsia`
- enumeration: value: `minorBidi`
- enumeration: value: `minorAscii`
- enumeration: value: `minorHAnsi`

<a id="w-st_themecolor"></a>

## w:ST_ThemeColor

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `dark1`
- enumeration: value: `light1`
- enumeration: value: `dark2`
- enumeration: value: `light2`
- enumeration: value: `accent1`
- enumeration: value: `accent2`
- enumeration: value: `accent3`
- enumeration: value: `accent4`
- enumeration: value: `accent5`
- enumeration: value: `accent6`
- enumeration: value: `hyperlink`
- enumeration: value: `followedHyperlink`
- enumeration: value: `none`
- enumeration: value: `background1`
- enumeration: value: `text1`
- enumeration: value: `background2`
- enumeration: value: `text2`

<a id="w-st_ucharhexnumber"></a>

## w:ST_UcharHexNumber

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:hexBinary`
- length: value: `1`

<a id="w-st_underline"></a>

## w:ST_Underline

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `single`
- enumeration: value: `words`
- enumeration: value: `double`
- enumeration: value: `thick`
- enumeration: value: `dotted`
- enumeration: value: `dottedHeavy`
- enumeration: value: `dash`
- enumeration: value: `dashedHeavy`
- enumeration: value: `dashLong`
- enumeration: value: `dashLongHeavy`
- enumeration: value: `dotDash`
- enumeration: value: `dashDotHeavy`
- enumeration: value: `dotDotDash`
- enumeration: value: `dashDotDotHeavy`
- enumeration: value: `wave`
- enumeration: value: `wavyHeavy`
- enumeration: value: `wavyDouble`
- enumeration: value: `none`

<a id="w-st_unqualifiedpercentage"></a>

## w:ST_UnqualifiedPercentage

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd):

- restriction: base: `xsd:integer`

<a id="w-st_unsigneddecimalnumber"></a>

## w:ST_UnsignedDecimalNumber

Publisher imports use this legacy name; the corresponding shared
value definition is `s:ST_UnsignedDecimalNumber` in the retained ECMA schemas.

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/shared-commonSimpleTypes.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/shared-commonSimpleTypes.xsd):

- restriction: base: `xsd:unsignedLong`

<a id="w-st_vanchor"></a>

## w:ST_VAnchor

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `text`
- enumeration: value: `margin`
- enumeration: value: `page`

<a id="w-st_verticaljc"></a>

## w:ST_VerticalJc

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `top`
- enumeration: value: `center`
- enumeration: value: `both`
- enumeration: value: `bottom`

<a id="w-st_wrap"></a>

## w:ST_Wrap

[Transitional](../../reference/ooxml/ecma-376/part-4/xsd-transitional/wml.xsd), [Strict](../../reference/ooxml/ecma-376/part-1/xsd-strict/wml.xsd):

- restriction: base: `xsd:string`
- enumeration: value: `auto`
- enumeration: value: `notBeside`
- enumeration: value: `around`
- enumeration: value: `tight`
- enumeration: value: `through`
- enumeration: value: `none`
