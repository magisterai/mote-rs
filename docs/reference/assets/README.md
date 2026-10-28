# Reference images

[Reference guide](../README.md)

| Directory | Contents | How to read |
| --- | --- | --- |
| `pages/` | All 10,893 source PDF pages as 150-DPI lossless WebP images, grouped by document | Open the page-image link beside the relevant Markdown text |
| `web/` | 56 technical figures from HTML specifications, plus one SVG font dependency | Figures are embedded at their corresponding positions in Markdown |

The Markdown lives in `../reading/`. Images are supplementary visual references;
start with the topic and clause indexes before opening individual assets.

[Page provenance](pages/manifest.json) records source page positions, dimensions,
and hashes. [Figure provenance](web/manifest.json) records source URLs, hashes,
document associations, and the decorative assets omitted during normalization.
