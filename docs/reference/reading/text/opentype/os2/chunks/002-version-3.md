# Version 3

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-os-2-table-formats.md) · [Next](003-os-2-field-details.md)

<a id="version-3"></a>

### Version 3

Version 3 was defined in OpenType 1.4. Version 3 has the same fields as in version 4, and as in version 2. Although new fields were not added beyond those in version 2, the specification of certain fields was revised to reflect changes in Unicode 3.2. The format of version 3 is identical to the format for version 4, given above.

<a id="version-2"></a>

### Version 2

Version 2 was defined in OpenType 1.1. Version 2 has the same fields as in version 3, and five additional fields beyond those in version 1. The format of version 2 is identical to the format for version 4, given above.

<a id="version-1"></a>

### Version 1

Version 1 was defined in TrueType revision 1.66. Version 1 has five fewer fields than version 2, and two additional fields beyond those in version 0. The format of version 1 is as follows:

| Type | Name |
| --- | --- |
| uint16 | [version](003-os-2-field-details.md#ver) |
| FWORD | [xAvgCharWidth](003-os-2-field-details.md#acw) |
| uint16 | [usWeightClass](003-os-2-field-details.md#wtc) |
| uint16 | [usWidthClass](003-os-2-field-details.md#wdc) |
| uint16 | [fsType](004-fstype.md#fst) |
| FWORD | [ySubscriptXSize](004-fstype.md#subxs) |
| FWORD | [ySubscriptYSize](005-ysubscriptysize.md#subys) |
| FWORD | [ySubscriptXOffset](005-ysubscriptysize.md#subxo) |
| FWORD | [ySubscriptYOffset](005-ysubscriptysize.md#subyo) |
| FWORD | [ySuperscriptXSize](005-ysubscriptysize.md#supxs) |
| FWORD | [ySuperscriptYSize](005-ysubscriptysize.md#supys) |
| FWORD | [ySuperscriptXOffset](005-ysubscriptysize.md#supxo) |
| FWORD | [ySuperscriptYOffset](005-ysubscriptysize.md#supyo) |
| FWORD | [yStrikeoutSize](006-ystrikeoutsize.md#ss) |
| FWORD | [yStrikeoutPosition](006-ystrikeoutsize.md#sp) |
| int16 | [sFamilyClass](006-ystrikeoutsize.md#fc) |
| uint8 | [panose[10]](006-ystrikeoutsize.md#pan) |
| uint32 | [ulUnicodeRange1](006-ystrikeoutsize.md#ur) |
| uint32 | [ulUnicodeRange2](006-ystrikeoutsize.md#ur) |
| uint32 | [ulUnicodeRange3](006-ystrikeoutsize.md#ur) |
| uint32 | [ulUnicodeRange4](006-ystrikeoutsize.md#ur) |
| Tag | [achVendID](008-achvendid.md#vendid) |
| uint16 | [fsSelection](008-achvendid.md#fss) |
| uint16 | [usFirstCharIndex](009-usfirstcharindex.md#fci) |
| uint16 | [usLastCharIndex](009-usfirstcharindex.md#lci) |
| FWORD | [sTypoAscender](009-usfirstcharindex.md#sta) |
| FWORD | [sTypoDescender](009-usfirstcharindex.md#std) |
| FWORD | [sTypoLineGap](009-usfirstcharindex.md#tlg) |
| UFWORD | [usWinAscent](009-usfirstcharindex.md#wa) |
| UFWORD | [usWinDescent](010-uswindescent.md#wd) |
| uint32 | [ulCodePageRange1](010-uswindescent.md#cpr) |
| uint32 | [ulCodePageRange2](010-uswindescent.md#cpr) |

<a id="version-0"></a>

### Version 0

Version 0 was defined in TrueType revision 1.5. The format of version 0 is as follows:

| Type | Name |
| --- | --- |
| uint16 | [version](003-os-2-field-details.md#ver) |
| FWORD | [xAvgCharWidth](003-os-2-field-details.md#acw) |
| uint16 | [usWeightClass](003-os-2-field-details.md#wtc) |
| uint16 | [usWidthClass](003-os-2-field-details.md#wdc) |
| uint16 | [fsType](004-fstype.md#fst) |
| FWORD | [ySubscriptXSize](004-fstype.md#subxs) |
| FWORD | [ySubscriptYSize](005-ysubscriptysize.md#subys) |
| FWORD | [ySubscriptXOffset](005-ysubscriptysize.md#subxo) |
| FWORD | [ySubscriptYOffset](005-ysubscriptysize.md#subyo) |
| FWORD | [ySuperscriptXSize](005-ysubscriptysize.md#supxs) |
| FWORD | [ySuperscriptYSize](005-ysubscriptysize.md#supys) |
| FWORD | [ySuperscriptXOffset](005-ysubscriptysize.md#supxo) |
| FWORD | [ySuperscriptYOffset](005-ysubscriptysize.md#supyo) |
| FWORD | [yStrikeoutSize](006-ystrikeoutsize.md#ss) |
| FWORD | [yStrikeoutPosition](006-ystrikeoutsize.md#sp) |
| int16 | [sFamilyClass](006-ystrikeoutsize.md#fc) |
| uint8 | [panose[10]](006-ystrikeoutsize.md#pan) |
| uint32 | [ulUnicodeRange1](006-ystrikeoutsize.md#ur) |
| uint32 | [ulUnicodeRange2](006-ystrikeoutsize.md#ur) |
| uint32 | [ulUnicodeRange3](006-ystrikeoutsize.md#ur) |
| uint32 | [ulUnicodeRange4](006-ystrikeoutsize.md#ur) |
| Tag | [achVendID](008-achvendid.md#vendid) |
| uint16 | [fsSelection](008-achvendid.md#fss) |
| uint16 | [usFirstCharIndex](009-usfirstcharindex.md#fci) |
| uint16 | [usLastCharIndex](009-usfirstcharindex.md#lci) |
| FWORD | [sTypoAscender](009-usfirstcharindex.md#sta) |
| FWORD | [sTypoDescender](009-usfirstcharindex.md#std) |
| FWORD | [sTypoLineGap](009-usfirstcharindex.md#tlg) |
| UFWORD | [usWinAscent](009-usfirstcharindex.md#wa) |
| UFWORD | [usWinDescent](010-uswindescent.md#wd) |

> *Note:* Documentation for OS/2 version 0 in [Apple’s TrueType Reference Manual](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6OS2.html) stops at the usLastCharIndex field and does not include the last five fields of the table as it was defined by Microsoft. Some legacy TrueType fonts could have been built with a shortened version 0 OS/2 table. Applications should check the table length for a version 0 OS/2 table before reading these fields.

