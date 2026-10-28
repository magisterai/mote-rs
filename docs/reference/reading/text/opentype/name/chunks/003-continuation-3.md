# Continuation 3

Source: [Naming table](https://learn.microsoft.com/en-us/typography/opentype/spec/name) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/name).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-name-ids.md) · [Next](004-name-id-examples.md)

<a id="nid0"></a>

<a id="nid1"></a>

<a id="nid2"></a>

<a id="nid3"></a>

<a id="nid4"></a>

<a id="nid5"></a>

<a id="nid6"></a>

<a id="nid7"></a>

<a id="nid8"></a>

<a id="nid9"></a>

<a id="nid10"></a>

<a id="nid11"></a>

<a id="nid12"></a>

<a id="nid13"></a>

<a id="nid14"></a>

<a id="nid15"></a>

<a id="nid16"></a>

<a id="nid17"></a>

<a id="nid18"></a>

<a id="nid19"></a>

<a id="nid20"></a>

<a id="nid21"></a>

<a id="nid22"></a>

<a id="nid23"></a>

<a id="nid24"></a>

<a id="nid25"></a>

| ID | Meaning |
| --- | --- |
| 0 | Copyright notice. |
| 1 | Font Family name. The Font Family name is used in combination with Font Subfamily name (name ID 2), and should be shared among at most four fonts that differ only in weight or style (italic/oblique), as described below.    This four-way distinction should also be reflected in the [OS/2.fsSelection](../../os2/chunks/008-achvendid.md#fss) field, using bits 0 and 5.    While some platforms or applications do not have this constraint, many existing applications that use this pair of names assume that a Font Family name is shared by at most four fonts that form a font style-linking group: regular, italic (or oblique), bold, and bold italic (or bold oblique). To be compatible with the broadest range of platforms and applications, it is strongly recommended that fonts limit use of Font Family name in this manner.    For extended typographic families that includes fonts other than the four basic styles (regular, italic, bold, bold italic), it is strongly recommended that name IDs [16](003-continuation-3.md#nid16) and [17](003-continuation-3.md#nid17) be used in fonts to create an extended, typographic grouping. (See examples provided below.)    It is also strongly recommended that applications support extended typographic-family groupings using name IDs 16 and 17. Note that variable fonts can include a large number of named instances, each of which will use a shared typographic family name (name ID 16) and will have a typographic subfamily name (equivalent to name ID 17). Applications that assume a four-style family grouping based on name IDs 1 and 2 are likely to provide a poor user experience with variable fonts.    For fonts within an extended typographic family that fall outside the basic four-way distinction, the distinguishing attributes should be reflected in the Font Family name so that those fonts appear as a separate font family in applications that support only four-member families. For example, the Font Family name for the Arial Narrow font is “Arial Narrow”; the Font Family name for the Arial Black font is “Arial Black”. Note that, in such cases, name ID 16 should also be included with a shared name (e.g., "Arial") that reflects the full, typographic family. |
| 2 | Font Subfamily name. The Font Subfamily is used in combination with Font Family name (name ID 1), and distinguishes the fonts in a group with the same Font Family name. This should be used for weight and style (italic/oblique) variants only, as described below.    This four-way distinction should also be reflected in the [OS/2.fsSelection](../../os2/chunks/008-achvendid.md#fss) field, using bits 0 and 5.    While some platforms or applications do not have this constraint, many existing applications that use name IDs 1 and 2 assume that a Font Family name is shared by at most four fonts that form a font style-linking group, and that Font Subfamily names would reflect one of the four basic styles, regular, italic (or oblique), bold, and bold italic (or bold oblique). To be compatible with the broadest range of platforms and applications, it is strongly recommended that fonts should limit use of Font Subfamily in this manner.    For extended typographic families that includes fonts other than the four basic styles (regular, italic, bold, bold italic), it is strongly recommended that name IDs [16](003-continuation-3.md#nid16) and [17](003-continuation-3.md#nid17) be used in fonts to create an extended, typographic grouping.    Within an extended typographic family that includes fonts beyond regular, bold, italic, or bold italic, distinctions for these other fonts are made in the Font Family name so that fonts appear to be in separate families. In some cases, this can lead to specifying a Subfamily name of “Regular” for a font that might not otherwise be considered a *regular* font. For example, the Arial Black font has a Font Family name of “Arial Black” and a Subfamily name of “Regular”. Note that, in such cases, name IDs 16 and 17 should also be included, using a shared value for name ID 16 (e.g., "Arial") that reflects the full typographic family, and values for name ID 17 that appropriately reflect the actual design variant of each font.    Fonts that are not part of an extended typographic family and with no distinctive weight or style (e.g., medium weight, not italic) should use "Regular" as the Font Subfamily name (for English). |
| 3 | Unique font identifier. |
| 4 | Full font name that reflects all family and relevant subfamily descriptors. The full font name is generally a combination of name IDs 1 and 2, or of name IDs 16 and 17, or a similar human-readable variant.    For fonts in extended typographic families (that is, families that include more than regular, italic, bold, and bold italic variants), values for name IDs 1 and 2 are normally chosen to provide compatibility with certain applications that assume a family has at most four style-linked fonts. In that case, some fonts may end up with a Subfamily name (name ID 2) of “Regular” even though the font would not be considered, typographically, a *regular* font. For such non-regular fonts in which name ID 2 is specified as “Regular”, the “Regular” descriptor would generally be omitted from name ID 4. For example, the Arial Black font has a Font Family name (name ID 1) of “Arial Black” and a Subfamily name (name ID 2) of “Regular”, but has a full font name (name ID 4) of “Arial Black”. Note that name IDs 16 and 17 should also be included in these fonts, and that name ID 4 would typically be a combination of name IDs 16 and 17, without needing any additional qualifications regarding “Regular”. |
| 5 | Version string. Should begin with the pattern “Version <number>.<number>” (upper case, lower case, or mixed, with a space between “Version” and the number).    The string must contain a version number of the following form: one or more digits (0-9) of value less than 65,535, followed by a period, followed by one or more digits of value less than 65,535. Any character other than a digit will terminate the minor number. A character such as “;” is helpful to separate different pieces of version information.    The first such match in the string can be used by installation software to compare font versions. Some installers could require the string to start with “Version ”, followed by a version number as above. |
| 6 | PostScript name for the font. Name ID 6 specifies a string which is used to invoke a PostScript language font that corresponds to this OpenType font. When translated to ASCII, the name string must be no longer than 63 characters and restricted to the printable ASCII subset, codes 33 to 126, except for the 10 characters '[', ']', '(', ')', '{', '}', '<', '>', '/', '%'.    In a CFF OpenType font, there is no requirement that this name be the same as the font name in the CFF’s Name INDEX. Thus, the same CFF may be shared among multiple font components in a Font Collection. See the ['name' table section](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#name) of “Recommendations for OpenType fonts” for additional information. |
| 7 | Trademark. This is used to save any trademark notice/information for this font. Such information should be based on legal advice. This is *distinctly* separate from the copyright. |
| 8 | Manufacturer Name. |
| 9 | Designer. Name of the designer of the typeface. |
| 10 | Description. Description of the typeface. Can contain revision information, usage recommendations, history, features, etc. |
| 11 | URL of Vendor. URL of font vendor (with protocol, e.g., http://, ftp://). If a unique serial number is embedded in the URL, it can be used to register the font. |
| 12 | URL of Designer. URL of typeface designer (with protocol, e.g., http://, ftp://). |
| 13 | License Description. Description of the license or licenses under which the font is provided. This could be a reference to a named license agreement (e.g., a common open source licenses), identification of a software-use license under which a font is bundled, information about where to locate an external license (see also name ID 14), a summary of permitted uses, or the full legal text of a license agreement. It is prudent to seek legal advice on the content of this name ID to avoid possible conflict of interpretation between it and the license(s). |
| 14 | License Info URL. URL where additional licensing information can be found. |
| 15 | Reserved. |
| 16 | Typographic Family name. The typographic family grouping doesn’t impose any constraints on the number of faces within it, in contrast with the 4-style family grouping (ID 1), which is present both for historical reasons and to express style linking groups. If name ID 16 is absent, then name ID 1 is considered to be the typographic family name. (In earlier versions of the specification, name ID 16 was known as “Preferred Family”.) |
| 17 | Typographic Subfamily name. This allows font designers to specify a subfamily name within the typographic family grouping. This string must be unique within a particular typographic family. If it is absent, then name ID 2 is considered to be the typographic subfamily name. (In earlier versions of the specification, name ID 17 was known as “Preferred Subfamily”.) |
| 18 | Compatible Full (Macintosh only). On the Macintosh, the menu name is constructed using the FOND resource. This usually matches the Full Name. If you want the name of the font to appear differently than the Full Name, you can insert the Compatible Full Name in ID 18. |
| 19 | Sample text. This can be the font name, or any other text that the designer thinks is the best sample to display the font in. |
| 20 | PostScript CID findfont name. Its presence in a font means that the nameID 6 holds a PostScript font name that is meant to be used with the “composefont” invocation in order to invoke the font in a PostScript interpreter. See the definition of name ID 6.    The value held in the name ID 20 string is interpreted as a PostScript font name that is meant to be used with the “findfont” invocation, in order to invoke the font in a PostScript interpreter.    When translated to ASCII, this name string must be restricted to the printable ASCII subset, codes 33 through 126, except for the 10 characters: '[', ']', '(', ')', '{', '}', '<', '>', '/', '%'.    See [“Recommendations for OTF fonts”](https://learn.microsoft.com/en-us/typography/opentype/spec/recom) for additional information |
| 21 | WWS Family Name. Used to provide a WWS-conformant family name in case the entries for IDs 16 and 17 do not conform to the WWS model. (That is, in case the entry for ID 17 includes qualifiers for some attribute other than weight, width or slope.) If bit 8 of the OS/2 fsSelection field is set, a WWS Family Name entry should not be needed and should not be included. Conversely, if an entry for this ID is included, bit 8 should not be set. (See [OS/2.fsSelection](../../os2/chunks/008-achvendid.md#fsselection) field for details.) Examples of name ID 21: “Minion Pro Caption” and “Minion Pro Display”. (Name ID 16 would be “Minion Pro” for these examples.)  See additional remarks regarding IDs 21 and 22 below. |
| 22 | WWS Subfamily Name. Used in conjunction with ID 21, this ID provides a WWS-conformant subfamily name (reflecting only weight, width and slope attributes) in case the entries for IDs 16 and 17 do not conform to the WWS model. As in the case of ID 21, use of this ID should correlate inversely with bit 8 of the OS/2 fsSelection field being set. Examples of name ID 22: “Semibold Italic”, “Bold Condensed”. (Name ID 17 could be “Semibold Italic Caption”, or “Bold Condensed Display”, for example.)  See additional remarks regarding IDs 21 and 22 below. |
| 23 | Light Background Palette. This ID, if used in the CPAL table’s Palette Labels Array, specifies that the corresponding color palette in the CPAL table is appropriate to use with the font when displaying it on a light background such as white. Strings for this ID are for use as user interface strings associated with this palette. |
| 24 | Dark Background Palette. This ID, if used in the CPAL table’s Palette Labels Array, specifies that the corresponding color palette in the CPAL table is appropriate to use with the font when displaying it on a dark background such as black. Strings for this ID are for use as user interface strings associated with this palette. |
| 25 | Variations PostScript Name Prefix. If present in a variable font, it may be used as the family prefix in the PostScript Name Generation for Variation Fonts algorithm. The character set is restricted to ASCII-range uppercase Latin letters, lowercase Latin letters, and digits. All name strings for name ID 25 within a font, when converted to ASCII, must be identical. See [Adobe Technical Note #5902: “PostScript Name Generation for Variation Fonts”](https://adobe-type-tools.github.io/font-tech-notes/pdfs/5902.AdobePSNameGeneration.pdf) for reasons to include name ID 25 in a font, and for examples. For general information on OpenType Font Variations, see the chapter, [OpenType Font Variations Overview](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview). |

