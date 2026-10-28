# Sources and native resources

Specifications for editable Office documents, native rendering, text layout,
and authoring. Collected on 27 October 2026 from publishers and standards bodies.

Start with the topic map below. The [catalogue](catalogue.md) lists every source,
version, source URL, and local reading index. [sources.json](sources.json) records
download URLs, retrieval times, SHA-256 hashes, archive members, and retained
native resources.

## Office Open XML

[ECMA-376](https://ecma-international.org/publications-and-standards/standards/ecma-376/)
is the core reference for the Office Open XML vocabularies. The four parts have
different publication dates; the copies here are the fifth edition of each part.

| Reference | Local copy | Purpose |
| --- | --- | --- |
| Part 1, December 2016 | [Read](reading/ooxml/ecma-376/part-1/README.md) | WordprocessingML, SpreadsheetML, PresentationML, DrawingML, Office Math, and shared properties |
| Part 2, December 2021 | [Read](reading/ooxml/ecma-376/part-2/README.md) | Open Packaging Conventions: parts, relationships, content types, and package metadata |
| Part 3, December 2015 | [Read](reading/ooxml/ecma-376/part-3/README.md) | Markup compatibility, extensibility, and alternate content |
| Part 4, December 2016 | [Read](reading/ooxml/ecma-376/part-4/README.md) | Transitional markup, compatibility features, and VML |

Useful entry points in Part 1:

| Topic | Clauses |
| --- | --- |
| Word packages, paragraphs, sections, styles, numbering, tables, notes, comments, and fields | 11 and 17 |
| Embedded spreadsheet workbooks and worksheet data for editable charts | 12 and 18 |
| Presentations, slides, masters, layouts, and notes | 13 and 19 |
| Drawing geometry, pictures, text, themes, and drawing placement | 14, 20, and 21.1 |
| Charts and chart drawings | 21.2 and 21.3 |
| Office Math Markup Language (OMML) | 22.1 |
| Shared properties, relationships, and simple types | 15 and 22 |

Machine-readable attachments are included:

- [Strict XSD](ooxml/ecma-376/part-1/xsd-strict/) and
  [RELAX NG](ooxml/ecma-376/part-1/relaxng-strict/).
- [Transitional XSD](ooxml/ecma-376/part-4/xsd-transitional/) and
  [RELAX NG](ooxml/ecma-376/part-4/relaxng-transitional/).
- [Packaging XSD](ooxml/ecma-376/part-2/xsd/) and
  [RELAX NG](ooxml/ecma-376/part-2/relaxng/).
- [Preset shape geometry](ooxml/ecma-376/part-1/drawingml-geometries/presetShapeDefinitions.xml)
  and [text warp geometry](ooxml/ecma-376/part-1/drawingml-geometries/presetTextWarpDefinitions.xml).
- [Spreadsheet style resources](ooxml/ecma-376/part-1/spreadsheetml-styles/)
  and the original [Word art-border resource archive](ooxml/ecma-376/part-1/wordprocessingml-art-borders.zip).

Strict and Transitional use different namespaces and schemas. Match the schema
to the package being generated; namespace names alone do not establish conformance.

## Microsoft extensions and implementation notes

These supplement the core standards. Each document has a complete, indexed
Markdown reading copy with local page images.

| Reference | Purpose |
| --- | --- |
| [MS-DOCX](reading/ooxml/microsoft/MS-DOCX/README.md) | Word extensions |
| [MS-PPTX](reading/ooxml/microsoft/MS-PPTX/README.md) | PowerPoint extensions |
| [MS-ODRAWXML](reading/ooxml/microsoft/MS-ODRAWXML/README.md) | Drawing extensions shared by Office applications |
| [MS-XLSX](reading/ooxml/microsoft/MS-XLSX/README.md) | Spreadsheet extensions relevant to embedded chart workbooks |
| [MS-OI29500](reading/ooxml/microsoft/MS-OI29500/README.md) | Office behaviour and deviations from ISO/IEC 29500 |
| [MS-OE376](reading/ooxml/microsoft/MS-OE376/README.md) | Office behaviour against ECMA-376; historical compatibility reference |

Schema validity and visual fidelity are separate checks. Microsoft Word and
PowerPoint desktop are the layout references for their respective formats.
Native PDF and PNG rendering needs independent implementation and comparison.

## Supporting specifications

| Area | Local references |
| --- | --- |
| XML | [XML 1.0](reading/xml/xml-1.0/README.md), [namespaces](reading/xml/xml-namespaces-1.0/README.md), XML Schema [structures](reading/xml/xml-schema-1.0-structures/README.md) and [datatypes](reading/xml/xml-schema-1.0-datatypes/README.md) |
| Packaging and compression | [ZIP](reading/packaging/zip-appnote-6.3.10/README.md), [zlib](reading/packaging/rfc1950/README.md), [DEFLATE](reading/packaging/rfc1951/README.md), [URIs](reading/packaging/rfc3986/README.md), [IRIs](reading/packaging/rfc3987/README.md) |
| PDF | [ISO 32000-1:2008 / PDF 1.7](reading/rendering/pdf-1.7-iso-32000-1-2008/README.md), Adobe's publication |
| Raster images | [PNG third edition](reading/images/png-third-edition/README.md), [JPEG T.81](reading/images/jpeg-t81/README.md), [JFIF 1.02](reading/images/jpeg-jfif-1.02/README.md), [GIF89a](reading/images/gif89a/README.md) |
| Fonts | [OpenType 1.9.1](reading/text/opentype/index/README.md), focused table references in [OpenType reading copies](reading/text/opentype/), [CFF](reading/text/fonts/adobe-5176-cff/README.md), [Type 2 Charstrings](reading/text/fonts/adobe-5177-type2/README.md) |
| Text | [UTF-8](reading/text/unicode/rfc3629/README.md), Unicode [bidirectionality](reading/text/unicode/uax-09/README.md), [East Asian width](reading/text/unicode/uax-11/README.md), [line breaking](reading/text/unicode/uax-14/README.md), [normalization](reading/text/unicode/uax-15/README.md), and [segmentation](reading/text/unicode/uax-29/README.md) |
| Markdown | [CommonMark 0.31.2](reading/authoring/commonmark-0.31.2/README.md) and [GFM 0.29](reading/authoring/gfm-0.29/README.md) |
| Scripting and data | [Lua 5.1](reading/authoring/lua-5.1/README.md), [JSON / RFC 8259](reading/authoring/rfc8259/README.md), [CSV / RFC 4180](reading/authoring/rfc4180/README.md) |

The Unicode annexes are pinned together to Unicode 17.0.0. OpenType includes
font containers, metrics, glyph mapping, TrueType and CFF outlines, embedding
flags, shaping tables, vertical metrics, and mathematical typesetting data.

Markdown and Lua references inform the authoring language; they do not define
Mote's directives or promise full language compatibility. SpreadsheetML is
included for chart workbooks. Collecting a specification does not make every
feature in it an implementation goal or a supported feature.

## Source provenance

The [catalogue](catalogue.md) links every publisher and download URL. The source
manifest records the exact retrieved editions and their SHA-256 hashes. URLs
without a version may change; use the recorded edition and checksum when
refreshing a reference.

PDF, HTML, and plain-text document containers are replaced by their normalized
reading copies and local visual assets. XSD, RELAX NG, geometry XML, spreadsheet
style resources, and the art-border archive remain in their native formats.
Publisher notices are retained in the reading copies. [Conversion notes](reading-notes.md)
describe coverage and verification.
