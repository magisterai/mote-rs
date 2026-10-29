# Text fidelity reference

This capture covers Unicode, XML escaping, internal tabs, hard line breaks,
consecutive and boundary breaks, empty paragraphs, and ordinary line-edge
whitespace. `source.mote` and `model.json` preserve the authoring/evaluation
boundary; `package/` contains the captured XML. `capture.json` records fingerprints.

[DOCX tests](../../../docx.rs) compare the evaluated model and every package part.
Text values, run boundaries, and the order of `w:t`, `w:tab`, and `w:br` remain
significant. Ordinary spaces around explicit breaks are trimmed; Unicode text
and interior whitespace remain intact. XML 1.0 forbidden characters cause an
output error rather than corrupting the package.

This is structural evidence. Real Word layout review remains **unreviewed**.
