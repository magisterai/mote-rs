# Normalization and coverage

[Reference guide](README.md)

The collection contains the full content of the 58 collected specification
publications, with progressive indexes and local visual companions. Original
PDF, HTML, and plain-text document containers are omitted. Source URLs, editions,
retrieval dates, and hashes identify the publisher material used.

## Representations

| Source | Local representation |
| --- | --- |
| 34 HTML documents | Markdown prose, headings, lists, examples, and tables; complex tables and mathematical/directional markup retain HTML |
| 15 PDF documents | Publisher bookmark hierarchy, complete page text in fenced blocks, and an image of every page |
| 9 plain-text documents | Complete consecutive text slices in fenced blocks, including whitespace and diagrams |
| Native schemas and resources | XSD, RELAX NG, geometry XML, spreadsheet styles, and Word art-border archive retained unchanged |

The usual chunk target is 14,000 content characters. Complete tables can exceed
that target. Large indexes are paginated. Follow continuations until the relevant
clause ends, and inspect the local page image when text extraction leaves a table,
equation, symbol, or diagram ambiguous.

PDF text is preserved without reflow, including front matter and publisher
notices. Every one of the 10,893 PDF pages has a 150-DPI RGB image, encoded with
lossless WebP compression. This retains visual context without requiring a PDF
reader. These are finite-resolution page images, not vector originals; lossless
refers to the image compression. Page positions are one-based and may differ
from printed folios. Images are linked, not embedded into every reading chunk.

HTML conversion excludes scripts, stylesheets, forms, and site navigation.
Microsoft Learn documents use the main article body. Code examples preserve
whitespace and literal Markdown fences. Complex tables, merged cells, directional
examples, and subscript/superscript markup retain HTML. All 56 technical figures
are stored locally, together with the SVG figures' font dependency. Nine decorative
image files were omitted: publisher logos, subsection arrows, and a licence badge.
Their links and licence notices remain as text. Subsection arrows are removed from headings, and
source anchors are kept separately from readable titles. Native SVG and raster
illustrations retain their original bytes.
Known links between collected documents resolve into local chunks and anchors.
Other cited works remain publisher links and are outside this collection.

## Verification

[reading-checks.json](reading-checks.json) records the completed checks:

- Input hashes verified before removing document containers; native resource
  hashes remain unchanged.
- Exact comparison of every PDF/plain-text slice with its source extraction,
  with gap-free, non-overlapping coverage of every page.
- Every PDF page rendered; each saved WebP decoded and compared pixel-for-pixel
  with the renderer's output before accepting it.
- HTML preformatted examples compared with the converted content, including
  examples inside retained HTML tables.
- HTML article text nodes checked for presence after normalization of markup,
  punctuation, entities, and whitespace; image occurrences checked against local
  assets, with SVG dependencies resolved locally.
- Generated-file hashes, local navigation targets, fragment anchors, image links,
  source URLs, and code-fence balance checked.

Word table layout, Office math, and JPEG diagram page images were inspected, as
were OpenType tables, PNG figure references, and Markdown syntax examples.
Automated coverage checks are exhaustive; visual inspection is representative.
These checks do not establish that every conversion is semantically perfect or
that any Mote implementation conforms to the specifications.

## Provenance and maintenance

[sources.json](sources.json) records source URLs, retrieved versions, original
hashes, archive membership, and the native resources still present.
[reading/manifest.json](reading/manifest.json) records extraction hashes, source
page/character ranges, conversion tools, and reading-file hashes.
[Page assets](assets/pages/manifest.json) and [web assets](assets/web/manifest.json)
have separate manifests so ordinary reading never requires loading them.

The temporary Python collection and normalization scripts are outside the
repository. Reading requires no Python runtime or conversion tools. To update a
reference, fetch the recorded source URL into a temporary directory, verify its
edition, regenerate the affected text, indexes, and assets, and repeat the checks
before discarding the downloaded container. Retain publisher notices and record
new hashes and retrieval metadata.
