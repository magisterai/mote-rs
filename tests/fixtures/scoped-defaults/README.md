# Scoped defaults captures

Progressive paragraph and list defaults, nested table/cell overrides, lexical
branches and loops, captured Lua constructor defaults, and geometry-free drawing
defaults followed by explicit geometry.

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
