# Headings reference

Covers Markdown and directive headings, levels one through six, empty headings,
inline formatting, direct spacing and pagination overrides, and declared styles
for higher levels. DOCX clamps levels above nine to nine. Levels seven through
nine need a declared style; the writer rejects unresolved style references.

The opening heading suppresses implicit before-spacing. Authored before-spacing
is retained. Fallback styles carry bold text, paired script sizes, outline levels,
keep-next and single-line spacing. Direct properties remain on the paragraph.

The capture reported warnings for fill, keep-together and outline level, although
its XML emits all three properties. The comparison retains those emitted values.
Every package part and the evaluated model are compared by the DOCX tests. Real
Word rendering and layout review remain **unreviewed**.
