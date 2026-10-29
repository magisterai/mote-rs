# Lua constructor captures

Seven sources cover every currently exposed element constructor, declarations,
helper calls, retained aliases, typed properties, prebuilt body structure and
empty leaf values. `media` includes an image asset and a second page.

Each capture freezes the complete JSON model and DOCX package, source and asset
hashes, and native PNG pages. All eight native PDF pages were pixel-exact at
144 DPI and were visually inspected. PNG comparison tolerates rasterizer
antialiasing; observed bounds are recorded in the manifests and enforced in tests.
Microsoft Word review remains unreviewed.

The `empty` case retains converted child data in the model while leaf controls
keep their defined output semantics. `text` deliberately retains direct heading
color, which native output paints and editable output diagnoses as unsupported.
