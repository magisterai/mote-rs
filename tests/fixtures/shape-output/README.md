# Editable shape captures

Six sources freeze complete models and DOCX packages for preset geometry,
transforms, appearance, nested text formatting, margins, fit settings, Lua
constructors, labels, and mixed images/shapes. Source and asset hashes guard the
captured fixtures. Shapes and images share a unique drawing-ID sequence while
media part numbering remains independent.

Drawing position and shadow lengths use explicit units when authored with a
length helper; bare numeric drawing coordinates represent EMUs in editable output.
Image dimensions retain their own established conversion rules. The `defaults`
case exercises numeric drawing units independently of explicit point/inch values.

All six packages match their captures. Native rendering is pending, and Microsoft
Word review remains unreviewed.
