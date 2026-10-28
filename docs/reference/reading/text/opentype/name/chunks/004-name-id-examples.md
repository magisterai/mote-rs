# Name ID examples

Source: [Naming table](https://learn.microsoft.com/en-us/typography/opentype/spec/name) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/name).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-continuation-3.md) · [Next](005-platform-encoding-and-language-ids.md)

For a typographic family that includes member faces that differ from Regular in relation to attributes other than weight, width or slope, there may also be some member faces that differ only in relation to these three attributes. For example, the Minion Pro family includes Minion Pro Display, but also includes Minion Pro Bold and Minion Pro Italic. IDs 21 and 22 should be used only in those fonts that differ from the Regular face in terms of an attribute other than weight, width or slope. For example, IDs 21 and 22 should be used in Minion Pro Display, but not in Minion Pro Bold or Minion Pro Italic.

> *Note:* While both Apple and Microsoft support the same set of name strings, the interpretations may be somewhat different. But since name strings are stored by platform, encoding and language (placing separate strings for both Apple and MS platforms), this should not present a problem.

The key information for this table for Microsoft platforms relates to the use of name IDs 1, 2, 4, 16 and 17. Note that some newer applications will use name IDs 16 and 17, while some legacy applications require name IDs 1 and 2 and also assume certain limitations on these values (see descriptions of name IDs 1 and 2 above). Fonts should include all of these strings for the broadest application compatibility. To better understand how to set values for these name IDs, some [examples](https://learn.microsoft.com/en-us/typography/opentype/spec/namesmp) of name usage, weight class and style flags have been created.

<a id="examples"></a>

<a id="name-id-examples"></a>

### Name ID examples

The following are examples of how these strings might be defined, based on Times New Roman Bold:

:   0. The copyright string from the font vendor. *© Copyright the Monotype Corporation plc, 1990*
      
      
    1. The name the user sees. *Times New Roman*
      
      
    2. The name of the style. *Bold*
      
      
    3. A unique identifier that applications can store to identify the font being used. *Monotype: Times New Roman Bold: 1990*
      
      
    4. The complete, unique, human readable name of the font. This name is used by Windows. *Times New Roman Bold*
      
      
    5. Release and version information from the font vendor. *Version 1.00 June 1, 1990, initial release*
      
      
    6. The name the font will be known by on a PostScript printer. *TimesNewRoman-Bold*
      
      
    7. Trademark string. *Times New Roman is a registered trademark of the Monotype Corporation.*
      
      
    8. Manufacturer. *Monotype Corporation*
      
      
    9. Designer. *Stanley Morison*
      
      
    10. Description. *Designed in 1932 for the Times of London newspaper. Excellent readability and a narrow overall width, allowing more words per line than most fonts.*
      
      
    11. URL of Vendor. *http://www.monotype.com*
      
      
    12. URL of Designer. *http://www.monotype.com*
      
      
    13. License Description. *This font may be installed on all of your machines and printers, but you may not sell or give these fonts to anyone else.*
      
      
    14. License Info URL. *http://www.monotype.com/license/*
      
      
    15. Reserved.
      
      
    16. Preferred Family. No name string present, since it is the same as name ID 1 (Font Family name).
      
      
    17. Preferred Subfamily. No name string present, since it is the same as name ID 2 (Font Subfamily name).
      
      
    18. Compatible Full (Macintosh only). No name string present, since it is the same as name ID 4 (Full name).
      
      
    19. Sample text. *The quick brown fox jumps over the lazy dog.*
      
      
    20. PostScript CID findfont name. No name string present. Thus, the PostScript Name defined by name ID 6 should be used with the “findfont” invocation for locating the font in the context of a PostScript interpreter.
      
      
    21. WWS family name: Since Times New Roman is a WWS font, this field does not need to be specified. If the font contained styles such as “caption”, “display”, “handwriting”, etc., that would be noted here.
      
      
    22. WWS subfamily name: Since Times New Roman is a WWS font, this field does not need to be specified.
      
      
    23. Light background palette name. No name string present, since this is not a color font.
      
      
    24. Dark background palette name. No name string present, since this is not a color font.
      
      
    25. Variations PostScript name prefix. No name string present, since this is not a variable font.

The following is an example of only name IDs 6 and 20 in the CFF OpenType Japanese font Kozuka Mincho Std Regular (other name IDs are also present in this font):
  
  
6. PostScript name: *KozMinStd-Regular*. Since a name ID 20 is present in the font (see below), then the PostScript name defined by name ID 6 should be used with the “composefont” invocation for locating the font in the context of a PostScript interpreter.
  
  
20. PostScript CID findfont name: *KozMinStd-Regular-83pv-RKSJ-H*, in a name record of Platform 1 [Macintosh], Platform-specific script 1 [Japanese], Language: 0xFFFF [English]. This name string is a PostScript name that should be used with the “findfont” invocation for locating the font in the context of a PostScript interpreter, and is associated with the encoding specified by the following 'cmap' subtable, which must be present in the font: Platform: 1 [Macintosh]; Platform-specific encoding: 1 [Japanese]; Language: 0 [not language-specific].

The following is an example of family/subfamily naming for an extended, WWS-only family. Consider Adobe Caslon Pro, with six members: upright and italic versions of regular, semibold and bold weights. (Bit 8 of the fsSelection field of the OS/2 table, version 4, should be set for all six fonts, and none should include 'name' entries for IDs 21 or 22.)

- Adobe Caslon Pro Regular:  
  Name ID 1: Adobe Caslon Pro  
  Name ID 2: Regular
- Adobe Caslon Pro Italic:  
  Name ID 1: Adobe Caslon Pro  
  Name ID 2: Italic
- Adobe Caslon Pro Semibold:  
  Name ID 1: Adobe Caslon Pro  
  Name ID 2: Bold  
  Name ID 16: Adobe Caslon Pro  
  Name ID 17: Semibold
- Adobe Caslon Pro Semibold Italic:  
  Name ID 1: Adobe Caslon Pro  
  Name ID 2: Bold Italic  
  Name ID 16: Adobe Caslon Pro  
  Name ID 17: Semibold Italic
- Adobe Caslon Pro Bold:  
  Name ID 1: Adobe Caslon Pro Bold  
  Name ID 2: Regular  
  Name ID 16: Adobe Caslon Pro  
  Name ID 17: Bold
- Adobe Caslon Pro Bold Italic:  
  Name ID 1: Adobe Caslon Pro Bold  
  Name ID 2: Italic  
  Name ID 16: Adobe Caslon Pro  
  Name ID 17: Bold Italic

The following is an example of family/subfamily naming for an extended, non-WWS family. Consider Minion Pro Opticals, with 32 member fonts: upright and italic versions of regular, medium, semibold and bold weights in each of four optical sizes: regular, caption, display and subhead. The following show names for a sampling of the fonts in this family. (Bit 8 of the fsSelection field in the OS/2 table, version 4, should be set in those fonts that do not include 'name' entries for IDs 21 or 22, and only in those fonts.)

- Minion Pro Regular:  
  Name ID 1: Minion Pro  
  Name ID 2: Regular
- Minion Pro Italic:  
  Name ID 1: Minion Pro  
  Name ID 2: Italic
- Minion Pro Semibold:  
  Name ID 1: Minion Pro SmBd  
  Name ID 2: Regular  
  Name ID 16: Minion Pro  
  Name ID 17: Semibold
- Minion Pro Semibold Italic:  
  Name ID 1: Minion Pro SmBd  
  Name ID 2: Italic  
  Name ID 16: Minion Pro  
  Name ID 17: Semibold Italic
- Minion Pro Caption:  
  Name ID 1: Minion Pro Capt  
  Name ID 2: Regular  
  Name ID 16: Minion Pro  
  Name ID 17: Caption  
  Name ID 21: Minion Pro Caption  
  Name ID 22: Regular
- Minion Pro Semibold Italic Caption:  
  Name ID 1: Minion Pro SmBd Capt  
  Name ID 2: Italic  
  Name ID 16: Minion Pro  
  Name ID 17: Semibold Italic Caption  
  Name ID 21: Minion Pro Caption  
  Name ID 22: Semibold Italic

