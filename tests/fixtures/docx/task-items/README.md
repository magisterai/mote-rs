# Task items reference

Covers checked/unchecked content controls, XML-sensitive names and titles,
legacy form fields with explicit and generated names, nested tasks, empty tasks,
geometry overrides and tasks interleaved with numbered items. Item checked state
wins over nested checkbox state. Nested metadata alone leaves ordinary numbering.

Modern controls use matching Word 2010 state and fallback glyphs; legacy fields
retain enabled/default/checked state without an open-time recalculation marker.
Both kinds share deterministic control identifiers beginning at 1000. Task items
use direct list geometry and allocate no numbering instance. The first ordinary
item in an ordered container allocates its instance using the container's start.

The model and every package part are compared. Tests also check state/glyph
agreement, unique IDs and numbering isolation. Word rendering remains unreviewed.
