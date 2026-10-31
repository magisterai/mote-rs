# Block content captures

Literal fenced and raw text, tabs and blank lines, definition paragraphs, and
document rules or the slide text-rule fallback. Word preserves separate definition
paragraphs inside table cells; other supported definition projections join their
paragraph children without inserting separators.

The frozen sources retain their evaluated models, every editable package part,
and independent native PNG pages. Manifests pin source, dependency and artifact
hashes. Imported logical names use `${fixture}` in the stored model so the tests
remain portable without discarding source identity.

PDF comparisons at 144 DPI are pixel-identical for every page in this group.
Native PNG differences stay within the captured per-page text-antialiasing limits;
no pixel differs by more than 64 channel levels. Models and complete normalized
editable packages match. All native PDF pages were visually inspected.

Real Microsoft Word and PowerPoint review remains pending. Native comparisons
and editable package checks do not establish Office review.

The established PNG divider profile places rules eight points above their PDF
position. This cross-format difference is retained explicitly, without changing
fixture spacing or the shared PDF geometry.
