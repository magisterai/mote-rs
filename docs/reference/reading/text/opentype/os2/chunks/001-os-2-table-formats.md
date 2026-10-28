# OS/2 Table Formats

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-version-3.md)

The OS/2 table consists of a set of metrics and other data that are required in OpenType fonts.

<a id="os2-table-formats"></a>

## OS/2 Table Formats

Six versions of the OS/2 table have been defined: versions 0 to 5. All versions are supported, but use of version 4 or later is strongly recommended.

<a id="version-5"></a>

### Version 5

The format of version 5 is as follows:

| Type | Name | Comments |
| --- | --- | --- |
| uint16 | [version](003-os-2-field-details.md#version) | 0x0005 |
| FWORD | [xAvgCharWidth](003-os-2-field-details.md#xavgcharwidth) |  |
| uint16 | [usWeightClass](003-os-2-field-details.md#usweightclass) |  |
| uint16 | [usWidthClass](003-os-2-field-details.md#uswidthclass) |  |
| uint16 | [fsType](004-fstype.md#fstype) |  |
| FWORD | [ySubscriptXSize](004-fstype.md#ysubscriptxsize) |  |
| FWORD | [ySubscriptYSize](005-ysubscriptysize.md#ysubscriptysize) |  |
| FWORD | [ySubscriptXOffset](005-ysubscriptysize.md#ysubscriptxoffset) |  |
| FWORD | [ySubscriptYOffset](005-ysubscriptysize.md#ysubscriptyoffset) |  |
| FWORD | [ySuperscriptXSize](005-ysubscriptysize.md#ysuperscriptxsize) |  |
| FWORD | [ySuperscriptYSize](005-ysubscriptysize.md#ysuperscriptysize) |  |
| FWORD | [ySuperscriptXOffset](005-ysubscriptysize.md#ysuperscriptxoffset) |  |
| FWORD | [ySuperscriptYOffset](005-ysubscriptysize.md#ysuperscriptyoffset) |  |
| FWORD | [yStrikeoutSize](006-ystrikeoutsize.md#ystrikeoutsize) |  |
| FWORD | [yStrikeoutPosition](006-ystrikeoutsize.md#ystrikeoutposition) |  |
| int16 | [sFamilyClass](006-ystrikeoutsize.md#sfamilyclass) |  |
| uint8 | [panose[10]](006-ystrikeoutsize.md#panose) |  |
| uint32 | [ulUnicodeRange1](006-ystrikeoutsize.md#ur) | Bits 0 – 31 |
| uint32 | [ulUnicodeRange2](006-ystrikeoutsize.md#ur) | Bits 32 – 63 |
| uint32 | [ulUnicodeRange3](006-ystrikeoutsize.md#ur) | Bits 64 – 95 |
| uint32 | [ulUnicodeRange4](006-ystrikeoutsize.md#ur) | Bits 96 – 127 |
| Tag | [achVendID](008-achvendid.md#achvendid) |  |
| uint16 | [fsSelection](008-achvendid.md#fsselection) |  |
| uint16 | [usFirstCharIndex](009-usfirstcharindex.md#usfirstcharindex) |  |
| uint16 | [usLastCharIndex](009-usfirstcharindex.md#uslastcharindex) |  |
| FWORD | [sTypoAscender](009-usfirstcharindex.md#stypoascender) |  |
| FWORD | [sTypoDescender](009-usfirstcharindex.md#stypodescender) |  |
| FWORD | [sTypoLineGap](009-usfirstcharindex.md#stypolinegap) |  |
| UFWORD | [usWinAscent](009-usfirstcharindex.md#uswinascent) |  |
| UFWORD | [usWinDescent](010-uswindescent.md#uswindescent) |  |
| uint32 | [ulCodePageRange1](010-uswindescent.md#cpr) | Bits 0 – 31 |
| uint32 | [ulCodePageRange2](010-uswindescent.md#cpr) | Bits 32 – 63 |
| FWORD | [sxHeight](011-sxheight.md#sxheight) |  |
| FWORD | [sCapHeight](011-sxheight.md#scapheight) |  |
| uint16 | [usDefaultChar](011-sxheight.md#usdefaultchar) |  |
| uint16 | [usBreakChar](011-sxheight.md#usbreakchar) |  |
| uint16 | [usMaxContext](011-sxheight.md#usmaxcontext) |  |
| uint16 | [usLowerOpticalPointSize](011-sxheight.md#lps) |  |
| uint16 | [usUpperOpticalPointSize](012-usupperopticalpointsize.md#ups) |  |

<a id="version-4"></a>

### Version 4

Version 4 was defined in OpenType 1.5. Version 4 has two fewer fields than version 5, and the same fields as in version 3. Although new fields were not added beyond those in version 3, the specification of certain fields was revised. The format of version 4 is as follows:

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
| FWORD | [sxHeight](011-sxheight.md#sxheight) |
| FWORD | [sCapHeight](011-sxheight.md#scapheight) |
| uint16 | [usDefaultChar](011-sxheight.md#usdefaultchar) |
| uint16 | [usBreakChar](011-sxheight.md#usbreakchar) |
| uint16 | [usMaxContext](011-sxheight.md#usmaxcontext) |

